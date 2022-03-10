mod api_service;
mod authentication;
mod config;
mod errors;
mod hybrid;
mod layout;
mod members;
mod models;
mod registration_handler;
mod secrets;
mod service_accounts;
mod team;
mod vaults;

use axum::extract::Extension;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "app=debug,tower_http=info")
    }
    tracing_subscriber::fmt::init();

    let config = config::Config::new();

    let db_pool = PgPool::connect(&config.app_database_url)
        .await
        .expect("Problem connecting to the database");
    let grpc_db_pool = db_pool.clone();

    let axum_make_service = axum::Router::new()
        .merge(vaults::routes())
        .merge(secrets::routes())
        .merge(team::routes())
        .merge(members::routes())
        .merge(service_accounts::routes())
        .merge(registration_handler::routes())
        .merge(statics::asset_pipeline_routes())
        .merge(statics::image_routes())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db_pool))
        .into_make_service();

    let grpc_service = tonic::transport::Server::builder()
        .add_service(app::vault::vault_server::VaultServer::new(
            api_service::VaultService { pool: grpc_db_pool },
        ))
        .into_service();

    let hybrid_make_service = hybrid::hybrid(axum_make_service, grpc_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::debug!("listening on {}", addr);
    let server = hyper::Server::bind(&addr).serve(hybrid_make_service);

    if let Err(e) = server.await {
        tracing::error!("server error: {}", e);
    }
}

// Error here disabled with "rust-analyzer.diagnostics.disabled": ["macro-error"]
// in .vscode/settings.json
pub mod statics {
    include!(concat!(env!("OUT_DIR"), "/statics.rs"));
}
