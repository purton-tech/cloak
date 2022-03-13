mod accept_invite;
mod create_invite;
mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub static INDEX: &str = "/app/team";
pub static CREATE_INVITE: &str = "/app/team/create_invite";
pub static INVITE: &str = "/app/team/invite/:org";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(INVITE, get(accept_invite::invite))
        .route(CREATE_INVITE, post(create_invite::create_invite))
}
