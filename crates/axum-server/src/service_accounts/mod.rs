mod connect_account;
mod delete;
mod index;
mod new_account;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/app/team/:organisation_id/service_accounts",
            get(index::index),
        )
        .route(
            "/app/team/:organisation_id/service_accounts/new",
            post(new_account::new),
        )
        .route(
            "/app/team/:organisation_id/service_accounts/connect",
            post(connect_account::connect),
        )
        .route(
            "/app/team/:organisation_id/service_accounts/delete",
            post(delete::delete),
        )
}
