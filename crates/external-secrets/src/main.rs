mod config;
mod errors;
mod secrets_handler;

use axum::{extract::Extension, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt().init();

    let config = config::Config::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // build our application with a route
    let app = Router::new()
        .route("/:key", get(secrets_handler::get_secrets))
        .layer(Extension(config));

    // run it
    tracing::info!(message = "Listening on ", %addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
