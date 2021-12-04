use tonic::transport::Server;
use vault::vault_server::VaultServer;
mod server;

pub mod vault {
    // The string specified here must match the proto package name
    tonic::include_proto!("vault");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = server::VaultImplementation::default();

    Server::builder()
        .add_service(VaultServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
