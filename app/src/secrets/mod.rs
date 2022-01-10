mod index;
mod new_secret;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/vault/:id/secrets";
pub static NEW: &str = "/app/vault/:id/new";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new_secret::new))
}

pub fn secret_route(vault_id: i32) -> String {
    format!("/app/vault/{}/secrets", vault_id)
}

pub fn new_route(vault_id: i32) -> String {
    format!("/app/vault/{}/new", vault_id)
}
