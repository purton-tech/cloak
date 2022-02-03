mod add_member;
mod index;

use axum::{routing::get, Router};

pub static INDEX: &str = "/app/vault/:id/members";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index::index))
}

pub fn member_route(vault_id: i32) -> String {
    format!("/app/vault/{}/members", vault_id)
}
