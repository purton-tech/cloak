mod index;
mod invite_user;

use axum::{routing::get, Router};

pub static INDEX: &str = "/app/team";

pub fn routes() -> Router {
    Router::new().route(INDEX, get(index::index))
}
