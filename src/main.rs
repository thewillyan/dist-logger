use greet::{
    greeter_client::GreeterClient,
    greeter_server::{Greeter, GreeterServer},
    GreetRequest, GreetResponse,
};
use std::{collections::HashMap, env};
use tokio::{fs::File, task::JoinSet};
use tonic::{transport::Server, Request, Response, Status};

use crate::log::Logger;

mod log;

mod greet {
    tonic::include_proto!("greet");
}

struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn greet(&self, req: Request<GreetRequest>) -> Result<Response<GreetResponse>, Status> {
        let inner = req.into_inner();
        let text = format!("Hello, {}!", inner.src);

        let reply = GreetResponse {
            src: inner.dst,
            dst: inner.src,
            text,
        };

        Ok(Response::new(reply))
    }
}

const PREFIX_PATH: &str = "/root";
const GRPC_PORT: u32 = 50051;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = env::var("HOSTNAME")?;
    let log_file = File::create(format!("{PREFIX_PATH}/{hostname}.log")).await?;
    let mut logger = Logger::new(log_file);

    logger.log(&format!("{hostname} started running.")).await?;

    let topology_file = std::fs::File::open(format!("{PREFIX_PATH}/topology.json"))?;
    let mut topology: HashMap<String, Vec<String>> = serde_json::from_reader(topology_file)?;
    let neighbors = topology
        .remove(&hostname)
        .expect("Topology should list the neighbors of every node.");

    logger
        .log(&format!("neighbors = {}.", neighbors.join(",")))
        .await?;

    // setup server
    let addr = format!("[::1]:{GRPC_PORT}").parse()?;
    let server = Server::builder().add_service(GreeterServer::new(GreeterService));

    tokio::spawn(async move {
        server
            .serve(addr)
            .await
            .expect("Failed to start gRPC server.");
    });

    // setup clients
    let mut connections = JoinSet::new();
    for n in neighbors.clone() {
        connections.spawn(async move {
            let url = format!("http://{n}:{GRPC_PORT}");
            let client = GreeterClient::connect(url)
                .await
                .unwrap_or_else(|err| panic!("Failed to connect to client {n}: {err}."));
            (n, client)
        });
    }

    let mut client_map = HashMap::with_capacity(neighbors.len());
    for (n, client) in connections.join_all().await {
        client_map.insert(n, client);
    }

    // TODO: send greeting requests
    Ok(())
}
