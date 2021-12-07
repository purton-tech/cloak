mod index;
mod new_vault;

use actix_web::web;

pub static INDEX: &str = "/app/vaults";
pub static NEW: &str = "/app/new_vault";

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource(INDEX).route(web::get().to(index::index)));
    cfg.service(web::resource(NEW).route(web::post().to(new_vault::new)));
}
