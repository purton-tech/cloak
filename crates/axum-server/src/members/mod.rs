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
