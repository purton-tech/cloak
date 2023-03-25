use crate::config::Config;

use super::keyring;
use p256::{
    pkcs8::{DecodePrivateKey, EncodePrivateKey},
    SecretKey,
};
use rand_core::OsRng; // requires 'getrandom' feature

pub async fn import(name: String, key: String, config: &Config) {
    // Pem format is 64 columns wide
    let subs = key
        .as_bytes()
        .chunks(64)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let key = subs.join("\n");

    let key = if key.contains("BEGIN PRIVATE") {
        key
    } else {
        format!(
            "-----BEGIN PRIVATE KEY-----
{}
-----END PRIVATE KEY-----",
            key
        )
    };

    let secret_key = SecretKey::from_pkcs8_pem(&key).expect("Problem Loading Key");

    let password = config.set_password();

    let secret_key_serialized = secret_key
        .to_pkcs8_encrypted_pem(&mut OsRng, password, Default::default())
        .unwrap()
        .to_string();

    println!("{}", secret_key_serialized);

    let mut keyring = keyring::KeyRing::load();

    keyring.add_service_account(name, secret_key_serialized);

    keyring.save();
}
