mod index;
mod set_details;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/profile", get(index::index))
        .route(
            "/app/team/:organisation_id/set_details",
            post(set_details::set_details),
        )
}

pub fn set_details_route(organisation_id: i32) -> String {
    format!("/app/team/{}/set_details", organisation_id)
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/profile", organisation_id)
}
