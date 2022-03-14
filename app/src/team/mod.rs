mod accept_invite;
mod create_invite;
mod delete_member;
mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/team";
pub static CREATE_INVITE: &str = "/app/team/create_invite";
pub static ACCEPT_INVITE: &str = "/app/team/accept_invite/";
pub static DELETE_MEMBER: &str = "/app/team/delete";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(ACCEPT_INVITE, get(accept_invite::invite))
        .route(CREATE_INVITE, post(create_invite::create_invite))
        .route(DELETE_MEMBER, post(delete_member::delete))
}
