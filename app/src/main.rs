mod api_service;
mod errors;
mod hybrid;
mod layout;
mod vaults;

use app::vault::vault_server::VaultServer;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "app=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let axum_make_service = axum::Router::new()
        .merge(routes())
        .merge(vaults::routes())
        .merge(statics::asset_pipeline_routes())
        .merge(statics::image_routes())
        .layer(TraceLayer::new_for_http())
        .into_make_service();

    let grpc_service = tonic::transport::Server::builder()
        .add_service(VaultServer::new(api_service::VaultService {}))
        .into_service();

    let hybrid_make_service = hybrid::hybrid(axum_make_service, grpc_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 7101));
    tracing::debug!("listening on {}", addr);
    let server = hyper::Server::bind(&addr).serve(hybrid_make_service);

    if let Err(e) = server.await {
        tracing::error!("server error: {}", e);
    }
}

pub fn routes() -> Router {
    axum::Router::new().route("/", get(root))
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    Html("<a href='/app/vaults'>Vaults</a>")
}

// Error here disabled with "rust-analyzer.diagnostics.disabled": ["macro-error"]
// in .vscode/settings.json
pub mod statics {
    include!(concat!(env!("OUT_DIR"), "/statics.rs"));
}
