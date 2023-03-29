mod delete_vault;
mod index;
mod new_vault;
mod rename;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/vaults", get(index::index))
        .route("/app/team/:organisation_id/new_vault", post(new_vault::new))
        .route(
            "/app/team/:organisation_id/vaults/rename",
            post(rename::rename),
        )
        .route(
            "/app/team/:organisation_id/vaults/delete",
            post(delete_vault::delete),
        )
}
