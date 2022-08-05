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

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/service_accounts", organisation_id)
}

pub fn delete_route(organisation_id: i32) -> String {
    format!("/app/team/{}/service_accounts/delete", organisation_id)
}

pub fn connect_route(organisation_id: i32) -> String {
    format!("/app/team/{}/service_accounts/connect", organisation_id)
}

pub fn new_route(organisation_id: i32) -> String {
    format!("/app/team/{}/service_accounts/new", organisation_id)
}
