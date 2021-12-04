use tonic::transport::Server;
use vault::vault_server::VaultServer;
mod config;
mod server;

pub mod vault {
    // The string specified here must match the proto package name
    tonic::include_proto!("vault");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::new();
    let addr = config.vault_server_listen_address.parse()?;
    let vault = server::VaultImplementation::default();

    Server::builder()
        .add_service(VaultServer::new(vault))
        .serve(addr)
        .await?;

    Ok(())
}
