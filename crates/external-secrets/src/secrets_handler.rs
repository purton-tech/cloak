/***
 * Retrieve secrets fom cloak based on the ECDH key. Decrypt those
 * secrets and return as JSON
 */
use crate::errors::CustomError;
use axum::extract::Extension;

pub async fn get_secrets(
    Extension(_config): Extension<super::config::Config>,
) -> Result<String, CustomError> {
    //Ok(Json(users))
    Ok("Hello".to_string())
}
