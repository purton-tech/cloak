mod accept_invite;
mod create_invite;
mod delete_member;
mod index;
mod switch;

use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id", get(index::index))
        .route("/app/team/:organisation_id/switch", get(switch::switch))
        .route("/app/team/accept_invite", get(accept_invite::invite))
        .route("/app/team/:organisation_id/create_invite", post(create_invite::create_invite))
        .route("/app/team/:organisation_id/delete", post(delete_member::delete))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}", organisation_id)
}

pub fn switch_route(organisation_id: i32) -> String {
    format!("/app/team/{}/switch", organisation_id)
}

pub fn create_route(organisation_id: i32) -> String {
    format!("/app/team/{}/create_invite", organisation_id)
}

pub fn delete_route(organisation_id: i32) -> String {
    format!("/app/team/{}/delete", organisation_id)
}
