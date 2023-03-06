/***
 * Retrieve secrets fom cloak based on the ECDH key. Decrypt those
 * secrets and return as JSON
 */
use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};

pub async fn get_secrets(
    Extension(_config): Extension<super::config::Config>,
) -> Result<Html<String>, CustomError> {
    tracing::debug!("Here");
    //Ok(Json(users))
    Ok(Html("Hello".to_string()))
}
