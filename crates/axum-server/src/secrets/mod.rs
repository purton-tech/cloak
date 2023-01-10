mod delete_secret;
mod index;
mod new_secret;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/app/team/:organisation_id/vault/:id/secrets",
            get(index::index),
        )
        .route(
            "/app/team/:organisation_id/vault/:id/secrets/new",
            post(new_secret::new),
        )
        .route(
            "/app/team/:organisation_id/vault/:id/secrets/delete",
            post(delete_secret::delete),
        )
}
