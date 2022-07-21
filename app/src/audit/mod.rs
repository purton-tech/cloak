mod index; 

use axum::{
    routing::{get},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id/audit", get(index::index))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}/audit", organisation_id)
}
