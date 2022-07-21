mod accept_invite;
mod create_invite;
mod delete_member;
mod index;
mod switch;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/team";
pub static SWITCH: &str = "/app/team/switch";
pub static CREATE: &str = "/app/team/create_invite";
pub static ACCEPT_INVITE: &str = "/app/team/accept_invite/";
pub static DELETE: &str = "/app/team/delete";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(SWITCH, get(switch::switch))
        .route(ACCEPT_INVITE, get(accept_invite::invite))
        .route(CREATE, post(create_invite::create_invite))
        .route(DELETE, post(delete_member::delete))
}
