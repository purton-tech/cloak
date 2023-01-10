mod delete_vault;
mod index;
mod new_vault;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/vaults", get(index::index))
        .route("/app/team/:organisation_id/new_vault", post(new_vault::new))
        .route(
            "/app/team/:organisation_id/vaults/delete",
            post(delete_vault::delete),
        )
}
