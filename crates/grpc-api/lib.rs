pub mod vault {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("vault");
}

pub mod grpc_web_vault {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("grpc_web/vault");
}

use aes_gcm::aead::{generic_array::GenericArray, Aead, Payload};
use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use p256::ecdh::SharedSecret;
use p256::SecretKey;
use p256::{elliptic_curve::ecdh, pkcs8::DecodePublicKey, PublicKey};
use std::collections::HashMap;

/***
 * Helper function to call the API and get the secrets in a service account.
 * These are then decrypted and passed back as a HashMap
 */
pub async fn get_secrets(
    secret_key: &SecretKey,
    api_host_url: &String,
    public_key_der_base64: &String,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let client = grpc_web_vault::vault_client::Vault::new(api_host_url.into());
    let response = client
        .get_service_account(grpc_web_vault::GetServiceAccountRequest {
            ecdh_public_key: public_key_der_base64.into(),
        })
        .await?;

    let aad = transform_u32_to_array_of_u8(response.service_account_id);

    let mut env_vars_to_inject: HashMap<String, String> = Default::default();
    for secret in response.secrets {
        let key_der = base64::decode(secret.ecdh_public_key).unwrap();

        let public_key = PublicKey::from_public_key_der(&key_der).unwrap();

        let shared_secret =
            ecdh::diffie_hellman(secret_key.to_nonzero_scalar(), public_key.as_affine());

        let plaintext_name = decrypt_secret(secret.encrypted_name, &aad, &shared_secret)?;
        let plaintext_value = decrypt_secret(secret.encrypted_secret_value, &aad, &shared_secret)?;
        env_vars_to_inject.insert(plaintext_name, plaintext_value);
    }
    Ok(env_vars_to_inject)
}

fn decrypt_secret(
    nonce_and_cipher: String,
    aad: &[u8; 4],
    shared_secret: &SharedSecret,
) -> Result<String, Box<dyn std::error::Error>> {
    let nonce_and_cipher: Vec<&str> = nonce_and_cipher.split('|').collect();
    if let Some(nonce) = nonce_and_cipher.first() {
        if let Some(cipher) = nonce_and_cipher.get(1) {
            let nonce_bytes = base64::decode(nonce).unwrap();
            let cipher_bytes = base64::decode(cipher).unwrap();

            let payload = Payload {
                msg: &cipher_bytes,
                aad,
            };
            let key: &GenericArray<u8, _> = GenericArray::from_slice(shared_secret.as_bytes());
            let nonce = GenericArray::from_slice(&nonce_bytes as &[u8]);

            let cipher = <Aes256Gcm>::new(key);
            let plaintext = cipher.decrypt(nonce, payload).unwrap();
            let plaintext = std::str::from_utf8(&plaintext).unwrap();

            return Ok(plaintext.to_string());
        }
    }
    Err("Bad request".into())
}

fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b4: u8 = ((x >> 24) & 0xff) as u8;
    let b3: u8 = ((x >> 16) & 0xff) as u8;
    let b2: u8 = ((x >> 8) & 0xff) as u8;
    let b1: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}
