use crate::errors::CustomError;
use actix_web::{web, HttpResponse};
use horrorshow::owned_html;

use horrorshow::prelude::*;

pub static INDEX: &str = "/vaults";

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource(INDEX).route(web::get().to(index)));
}

pub async fn index() -> Result<HttpResponse, CustomError> {
    let page = owned_html! {
        h1 { :"Home Page" }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(crate::layout::layout("Home", page).into_string().unwrap()))
}
