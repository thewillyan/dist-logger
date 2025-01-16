use greet::{
    greeter_server::{Greeter, GreeterServer},
    GreetRequest, GreetResponse,
};
use tonic::{transport::Server, Request, Response, Status};

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

const GRPC_PORT: u32 = 50051;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup server
    let addr = format!("[::0]:{GRPC_PORT}").parse()?;

    Server::builder()
        .add_service(GreeterServer::new(GreeterService))
        .serve(addr)
        .await
        .expect("Failed to start gRPC server.");

    Ok(())
}
