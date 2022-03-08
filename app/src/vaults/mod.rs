mod delete_vault;
mod index;
mod new_vault;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/vaults";
pub static NEW: &str = "/app/new_vault";
pub static DELETE: &str = "/app/vaults/delete";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new_vault::new))
        .route(DELETE, post(delete_vault::delete))
}
