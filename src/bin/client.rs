use greet::{greeter_client::GreeterClient, GreetRequest};
use std::collections::HashMap;
use tokio::task::JoinSet;
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
        .log(&format!("neighbors = {}.", neighbors.join(", ")))
        .await?;

    // setup clients
    let mut connections = JoinSet::new();
    for n in neighbors.clone() {
        connections.spawn(async move {
            let url = format!("http://{n}.lxd:{GRPC_PORT}");
            let client = GreeterClient::connect(url)
                .await
                .unwrap_or_else(|err| panic!("Failed to connect to client {n}: {err:?}."));
            (n, client)
        });
    }

    let mut client_map = HashMap::with_capacity(neighbors.len());
    for (n, client) in connections.join_all().await {
        client_map.insert(n, client);
    }

    // Send greeting requests
    let mut requests = JoinSet::new();
    for (n, mut client) in client_map.clone().into_iter() {
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
