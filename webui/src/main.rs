mod config;
mod layout;
mod vaults;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    // build our application with a route
    let app = Router::new().route("/", get(vaults::index));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
