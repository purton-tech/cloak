mod add_member;
mod delete_member;
mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/app/team/:organisation_id/vault/:id/members",
            get(index::index),
        )
        .route(
            "/app/team/:organisation_id/vault/:id/members/add",
            post(add_member::add),
        )
        .route(
            "/app/team/:organisation_id/vault/:id/members/delete",
            post(delete_member::delete),
        )
}

pub fn member_route(vault_id: i32, organisation_id: i32) -> String {
    format!("/app/team/{}/vault/{}/members", organisation_id, vault_id)
}
