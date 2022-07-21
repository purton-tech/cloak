mod connect_account;
mod delete;
mod index;
mod new_account;

use axum::{
    routing::{get, post},
    Router,
};

pub static NEW: &str = "/app/service_accounts/new";
pub static DELETE: &str = "/app/service_accounts/delete";
pub static CONNECT: &str = "/app/service_accounts/connect";

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/service_accounts", get(index::index))
        .route(NEW, post(new_account::new))
        .route(CONNECT, post(connect_account::connect))
        .route(DELETE, post(delete::delete))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/service_accounts", organisation_id)
}
