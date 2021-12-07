use tonic::transport::Server;
use vault::vault_server::VaultServer;
mod auth_id;
mod config;
mod errors;
mod models;
mod server;
use sqlx::PgPool;

pub mod vault {
    // The string specified here must match the proto package name
    tonic::include_proto!("vault");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt().with_env_filter("info").init();

    let config = config::Config::new();
    let addr = config.vault_server_listen_address.parse()?;

    let db_pool = PgPool::connect(&config.database_url)
        .await
        .expect("Problem connecting to the database");

    let vault = server::VaultImplementation { db_pool };

    tracing::info!("Starting {:?}", config.vault_server_listen_address);

    Server::builder()
        .add_service(VaultServer::new(vault))
        .serve(addr)
        .await?;

    Ok(())
}
