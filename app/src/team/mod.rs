mod accept_invite;
mod create_invite;
mod delete_member;
mod index;
mod switch;

use axum::{
    routing::{get, post},
    Router,
};

pub static CREATE: &str = "/app/team/create_invite";
pub static ACCEPT_INVITE: &str = "/app/team/accept_invite/";
pub static DELETE: &str = "/app/team/delete";

pub fn routes() -> Router {
    Router::new()
        .route("/app/team/:organisation_id", get(index::index))
        .route("/app/team/:organisation_id/switch", get(switch::switch))
        .route(ACCEPT_INVITE, get(accept_invite::invite))
        .route(CREATE, post(create_invite::create_invite))
        .route(DELETE, post(delete_member::delete))
}

pub fn index_route(organisation_id: i32) -> String {
    format!("/app/team/{}", organisation_id)
}

pub fn switch_route(organisation_id: i32) -> String {
    format!("/app/team/{}/switch", organisation_id)
}
