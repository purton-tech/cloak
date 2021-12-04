use crate::errors::CustomError;
use actix_web::{http, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new(form: web::Form<NewVault>) -> Result<HttpResponse, CustomError> {
    dbg!(&form);

    Ok(HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, super::INDEX))
        .finish())
}
