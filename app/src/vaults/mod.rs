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

pub static NEW: &str = "/app/team/:organisation_id/new_vault";
pub static DELETE: &str = "/app/team/:organisation_id/vaults/delete";

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/vaults", get(index::index))
        .route(NEW, post(new_vault::new))
        .route(DELETE, post(delete_vault::delete))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/vaults", organisation_id)
}
