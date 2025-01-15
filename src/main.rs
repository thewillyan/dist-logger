use greet::{
    greeter_client::GreeterClient,
    greeter_server::{Greeter, GreeterServer},
    GreetRequest, GreetResponse,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::{sync::broadcast, task::JoinSet};
use tonic::{transport::Server, Request, Response, Status};

use crate::log::Logger;

mod log;

mod greet {
    tonic::include_proto!("greet");
}

struct GreeterService {
    service_budget: Arc<AtomicUsize>,
    shutdown_signal: broadcast::Sender<()>,
}

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

        self.service_budget.fetch_sub(1, Ordering::AcqRel);

        if self.service_budget.load(Ordering::Relaxed) == 0 {
            self.shutdown_signal.send(()).unwrap();
        }

        Ok(Response::new(reply))
    }
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

    let topology_file = std::fs::File::open(format!("{PREFIX_PATH}/topology.json"))?;
    let mut topology: HashMap<String, Vec<String>> = serde_json::from_reader(topology_file)?;
    let neighbors = topology
        .remove(&hostname)
        .expect("Topology should list the neighbors of every node.");

    logger
        .log(&format!("neighbors = {}.", neighbors.join(",")))
        .await?;

    // setup server
    let (tx, mut rx) = broadcast::channel(1);
    let greeter_service = GreeterService {
        service_budget: Arc::new(neighbors.len().into()),
        shutdown_signal: tx,
    };

    let server = Server::builder().add_service(GreeterServer::new(greeter_service));
    let addr = format!("[::1]:{GRPC_PORT}").parse()?;

    let server_handler = tokio::spawn(async move {
        server
            .serve_with_shutdown(addr, async { rx.recv().await.unwrap() })
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

    server_handler.await?;
    Ok(())
}
