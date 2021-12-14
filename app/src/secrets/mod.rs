mod index;
use crate::models;

use axum::{routing::get, Router};

pub static INDEX: &str = "/app/vault/:id/secrets";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index::index))
}

pub fn secret_route(vault: &models::Vault) -> String {
    format!("/app/vault/{}/secrets", vault.id)
}
