mod api_service;
mod authentication;
mod config;
mod email;
mod errors;
mod hybrid;
mod layout;
mod members;
mod registration_handler;
mod secrets;
mod service_accounts;
mod team;
mod vaults;

use axum::extract::{Extension, Path};
use axum::http::{Response, StatusCode, header, HeaderValue};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use crate::templates::statics::StaticFile;
use axum::body::{self, Empty, Body};
use axum::{response::IntoResponse, routing::get};

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
        .route("/static/*path", get(static_path))
        .merge(vaults::routes())
        .merge(secrets::routes())
        .merge(team::routes())
        .merge(members::routes())
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

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(body::boxed(Body::from(data.content)))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}

pub mod cornucopia {
    include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
}

include!(concat!(env!("OUT_DIR"), "/ructe/templates.rs"));
