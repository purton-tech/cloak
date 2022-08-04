mod filter;
mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/audit", get(index::index))
        .route("/app/team/:organisation_id/audit", post(filter::filter))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/audit", organisation_id)
}
