mod index;
mod invite_user;

use axum::{routing::get, Router};

pub static INDEX: &str = "/app/team";
pub static INVITE: &str = "/app/team/invite/:org";

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(INVITE, get(invite_user::invite))
}
