mod delete_secret;
mod index;
mod new_secret;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/vault/:id/secrets";
pub static NEW: &str = "/app/vault/:id/secrets/new";
pub static DELETE: &str = "/app/vault/:id/secrets/delete";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new_secret::new))
        .route(DELETE, post(delete_secret::delete))
}

pub fn secret_route(vault_id: i32) -> String {
    format!("/app/vault/{}/secrets", vault_id)
}

pub fn new_route(vault_id: i32) -> String {
    format!("/app/vault/{}/secrets/new", vault_id)
}

pub fn delete_route(vault_id: u32) -> String {
    format!("/app/vault/{}/secrets/delete", vault_id)
}
