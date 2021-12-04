use hello_world::greeter_server::GreeterServer;
use tonic::transport::Server;
mod server;

pub mod hello_world {
    // The string specified here must match the proto package name
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = server::MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
