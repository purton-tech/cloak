use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/***
 * Retrieve secrets fom cloak based on the ECDH key. Decrypt those
 * secrets and return as JSON
 */
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Path},
    Json,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    pub value: String,
}
pub async fn get_secrets(
    Extension(config): Extension<super::config::Config>,
    Path(key): Path<String>,
) -> Result<Json<Value>, CustomError> {
    let secrets: HashMap<String, String> = grpc_api::get_secrets(
        &config.secret_key,
        &config.api_host_url,
        &config.public_key_der_base64,
    )
    .await?;

    if let Some(value) = secrets.get(&key) {
        tracing::info!(message = "Secrets Retrieved", %config.public_key_der_base64);

        let value = Value {
            value: value.into(),
        };

        // You can convert a HashMap to Json. Nice.
        return Ok(Json(value));
    }

    tracing::error!(message = "Key Not Retrieved", %key);

    Err(CustomError::FaultySetup("Key Not Found".to_string()))
}
