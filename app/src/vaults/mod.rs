mod delete_vault;
mod index;
mod new_vault;
use time::OffsetDateTime;

use axum::{
    routing::{get, post},
    Router,
};


pub struct VaultSummary {
    pub id: i32,
    pub name: String,
    pub user_count: i32,
    pub secrets_count: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub static INDEX: &str = "/app/vaults";
pub static NEW: &str = "/app/new_vault";
pub static DELETE: &str = "/app/vaults/delete";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new_vault::new))
        .route(DELETE, post(delete_vault::delete))
}
