mod delete_secret;
mod index;
mod new_secret;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/vault/:id/secrets", get(index::index))
        .route("/app/team/:organisation_id/vault/:id/secrets/new", post(new_secret::new))
        .route("/app/team/:organisation_id/vault/:id/secrets/delete", post(delete_secret::delete))
}

pub fn index_route(organisation_id: i32, vault_id: i32) -> String {
    format!("/app/team/{}/vault/{}/secrets", organisation_id, vault_id)
}

pub fn new_route(organisation_id: i32, vault_id: i32) -> String {
    format!("/app/team/{}/vault/{}/secrets/new", organisation_id, vault_id)
}

pub fn delete_route(organisation_id: i32, vault_id: i32) -> String {
    format!("/app/team/{}/vault/{}/secrets/delete", organisation_id, vault_id)
}
