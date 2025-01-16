use greet::{
    greeter_server::{Greeter, GreeterServer},
    GreetRequest, GreetResponse,
};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::broadcast;
use tonic::{transport::Server, Request, Response, Status};

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

    let num_neighbors = {
        let topology_file = std::fs::File::open(format!("{PREFIX_PATH}/topology.json"))?;
        let mut topology: HashMap<String, Vec<String>> = serde_json::from_reader(topology_file)?;
        topology
            .remove(&hostname)
            .expect("Topology should list the neighbors of every node.")
            .len()
    };

    // setup server
    let (tx, mut rx) = broadcast::channel(1);
    let greeter_service = GreeterService {
        service_budget: Arc::new(num_neighbors.into()),
        shutdown_signal: tx,
    };
    let addr = format!("[::0]:{GRPC_PORT}").parse()?;

    Server::builder()
        .add_service(GreeterServer::new(greeter_service))
        .serve_with_shutdown(addr, async { rx.recv().await.unwrap() })
        .await
        .expect("Failed to start gRPC server.");

    Ok(())
}
