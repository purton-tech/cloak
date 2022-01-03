use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::response::{IntoResponse, Redirect};

pub async fn new(_authentication: Authentication) -> Result<impl IntoResponse, CustomError> {
    Ok(Redirect::to(super::INDEX.parse().unwrap()))
}
