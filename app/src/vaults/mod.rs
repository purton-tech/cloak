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

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/vaults", get(index::index))
        .route("/app/team/:organisation_id/new_vault", post(new_vault::new))
        .route(
            "/app/team/:organisation_id/vaults/delete",
            post(delete_vault::delete),
        )
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/vaults", organisation_id)
}

pub fn new_route(organisation_id: i32) -> String {
    format!("/app/team/{}/new_vault", organisation_id)
}

pub fn delete_route(organisation_id: i32) -> String {
    format!("/app/team/{}/vaults/delete", organisation_id)
}
