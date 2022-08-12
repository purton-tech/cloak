mod api_service;
mod audit;
mod authentication;
mod config;
mod email;
mod errors;
mod hybrid;
mod layout;
mod members;
mod profile;
mod registration_handler;
mod rls;
mod secrets;
mod service_accounts;
mod static_files;
mod team;
mod vaults;

use axum::extract::Extension;
use axum::{response::Html, routing::get};
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
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let config = config::Config::new();
    let pool = config.create_pool();

    let axum_make_service = axum::Router::new()
        .route("/static/*path", get(static_files::static_path))
        .merge(audit::routes())
        .merge(vaults::routes())
        .merge(secrets::routes())
        .merge(team::routes())
        .merge(members::routes())
        .merge(profile::routes())
        .merge(service_accounts::routes())
        .merge(registration_handler::routes())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config))
        .layer(Extension(pool.clone()))
        .into_make_service();

    let grpc_service = tonic::transport::Server::builder()
        .add_service(app::vault::vault_server::VaultServer::new(
            api_service::VaultService { pool },
        ))
        .into_service();

    let hybrid_make_service = hybrid::hybrid(axum_make_service, grpc_service);

    tracing::debug!("listening on {}", addr);
    let server = hyper::Server::bind(&addr).serve(hybrid_make_service);

    if let Err(e) = server.await {
        tracing::error!("server error: {}", e);
    }
}

pub fn render<F>(f: F) -> Html<&'static str>
where
    F: FnOnce(&mut Vec<u8>) -> Result<(), std::io::Error>,
{
    let mut buf = Vec::new();
    f(&mut buf).expect("Error rendering template");
    let html: String = String::from_utf8_lossy(&buf).into();

    Html(Box::leak(html.into_boxed_str()))
}

#[allow(clippy::all)]
pub mod cornucopia {
    include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
}

#[allow(clippy::all)]
pub mod ructe {
    // https://github.com/rust-lang/rust-analyzer/issues/5484
    include!(concat!(env!("OUT_DIR"), "/ructe/templates.rs"));
}
