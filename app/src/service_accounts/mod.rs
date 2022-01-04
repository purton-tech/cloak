mod index;
mod new_account;
mod view;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/service_accounts";
pub static NEW: &str = "/app/service_accounts/new";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new_account::new))
}
