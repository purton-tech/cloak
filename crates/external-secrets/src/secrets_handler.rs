use std::collections::HashMap;

/***
 * Retrieve secrets fom cloak based on the ECDH key. Decrypt those
 * secrets and return as JSON
 */
use crate::errors::CustomError;
use axum::{extract::Extension, response::Html};

pub async fn get_secrets(
    Extension(config): Extension<super::config::Config>,
) -> Result<Html<String>, CustomError> {
    let secrets: HashMap<String, String> = grpc_api::get_secrets(
        &config.secret_key,
        &config.api_host_url,
        &config.public_key_der_base64,
    )
    .await?;

    dbg!(&secrets);

    Ok(Html("Hello".to_string()))
}
