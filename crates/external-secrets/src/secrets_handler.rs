use std::collections::HashMap;

/***
 * Retrieve secrets fom cloak based on the ECDH key. Decrypt those
 * secrets and return as JSON
 */
use crate::errors::CustomError;
use axum::{extract::Extension, Json};

pub async fn get_secrets(
    Extension(config): Extension<super::config::Config>,
) -> Result<Json<HashMap<String, String>>, CustomError> {
    let secrets: HashMap<String, String> = grpc_api::get_secrets(
        &config.secret_key,
        &config.api_host_url,
        &config.public_key_der_base64,
    )
    .await?;

    tracing::info!(message = "Secrets Retrieved", %config.public_key_der_base64);

    // You can convert a HashMap to Json. Nice.
    Ok(Json(secrets))
}
