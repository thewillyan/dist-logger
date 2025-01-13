use greet::{
    greeter_client::GreeterClient,
    greeter_server::{Greeter, GreeterServer},
    GreetRequest, GreetResponse,
};
use std::{collections::HashMap, env};
use tokio::fs::File;
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
        let name = req.into_inner().name;

        let reply = GreetResponse {
            text: format!("Hello ${name}!"),
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
    let topology: HashMap<String, Vec<String>> = serde_json::from_reader(topology_file)?;
    let neighbors = topology
        .get(&hostname)
        .expect("Topology should list the neighbors of every node.");

    logger
        .log(&format!("neighbors = {}.", neighbors.join(",")))
        .await?;

    let addr = format!("[::1]:{GRPC_PORT}").parse()?;
    let server = Server::builder().add_service(GreeterServer::new(GreeterService));

    tokio::spawn(async move {
        server
            .serve(addr)
            .await
            .expect("Failed to start gRPC server.");
    });

    let mut clients = Vec::with_capacity(neighbors.len());
    for n in neighbors {
        let url = format!("http://{n}:{GRPC_PORT}");
        // this way i can't create a clients concurrently, make it possible.
        clients.push(GreeterClient::connect(url).await?);
    }

    Ok(())
}
