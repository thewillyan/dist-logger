use greet::{greeter_client::GreeterClient, GreetRequest};
use std::collections::HashMap;
use tokio::{
    task::JoinSet,
    time::{sleep, Duration},
};
use tonic::Request;

use dist_logger::log::Logger;

mod greet {
    tonic::include_proto!("greet");
}

const PREFIX_PATH: &str = "/root";
const GRPC_PORT: u32 = 50051;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = hostname::get()?
        .into_string()
        .expect("Covert OsString to String.");
    let log_file = tokio::fs::File::create(format!("{PREFIX_PATH}/{hostname}.log")).await?;
    let mut logger = Logger::new(log_file);

    logger.log(&format!("{hostname} started running.")).await?;

    let neighbors = {
        let topology_file = std::fs::File::open(format!("{PREFIX_PATH}/topology.json"))?;
        let mut topology: HashMap<String, Vec<String>> = serde_json::from_reader(topology_file)?;
        topology
            .remove(&hostname)
            .expect("Topology should list the neighbors of every node.")
    };

    logger
        .log(&format!("{hostname} neighbors = {}.", neighbors.join(", ")))
        .await?;

    // setup clients
    let max_conn_attempts = 20;
    let retry_interval = Duration::from_millis(100);
    let mut connections = JoinSet::new();
    for n in neighbors.clone() {
        connections.spawn(async move {
            let url = format!("http://{n}.lxd:{GRPC_PORT}");
            let mut client = None;
            let mut conn_attempts: u32 = 0;
            while client.is_none() && conn_attempts < max_conn_attempts {
                conn_attempts += 1;
                match GreeterClient::connect(url.clone()).await {
                    Ok(grpc_client) => client = Some(grpc_client),
                    Err(err) => {
                        eprintln!("Connection attempt {conn_attempts} failed: {err}.");
                        sleep(retry_interval).await;
                    }
                }
            }
            let client = client.unwrap_or_else(|| {
                panic!("Max connection attempts reached. Failed to connect to server on node {n}.")
            });
            (n, client)
        });
    }

    // send greeting requests
    let mut requests = JoinSet::new();
    while let Some(connection) = connections.join_next().await {
        let (n, mut client) = connection?;
        let src = hostname.clone();
        requests.spawn(async move {
            let request = Request::new(GreetRequest {
                src,
                dst: n.to_owned(),
            });
            client.greet(request).await
        });
    }

    for reply in requests.join_all().await {
        let reply = reply?.into_inner();
        let text = format!("{} said: '{}'", reply.src, reply.text);
        logger.log(&text).await?;
    }
    Ok(())
}
