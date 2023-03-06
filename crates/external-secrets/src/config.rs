use p256::{
    pkcs8::{DecodePrivateKey, EncodePublicKey},
    SecretKey,
};

/**
 * We derive all our configuration from environment variables. This struct
 * handles parsing those variables.
 */
#[derive(Clone, Debug)]
pub struct Config {
    // All secrets in Cloak are encrypted using ECDH, we need the private key
    // to decrypt them.
    pub secret_key: SecretKey,
    pub api_host_url: String,
    pub port: u16,
    pub public_key_der_base64: String,
}

impl Config {
    pub fn new() -> Config {
        let ecdh_private_key = std::env::var("ECDH_PRIVATE_KEY").expect("ECDH_PRIVATE_KEY not set");

        let api_host_url = if let Ok(api_host_url) = std::env::var("API_HOST_URL") {
            api_host_url
        } else {
            "https://app.cloak.software".to_string()
        };

        let port: u16 = if std::env::var("PORT").is_ok() {
            std::env::var("PORT").unwrap().parse::<u16>().unwrap()
        } else {
            7105
        };

        let secret_key = SecretKey::from_pkcs8_pem(&ecdh_private_key)
            .map_err(|_| "Problem loading key")
            .unwrap();

        let service_account_public_key = secret_key.public_key();

        let public_key_der = service_account_public_key.to_public_key_der().unwrap();
        let public_key_der_base64 = base64::encode(public_key_der);

        Config {
            secret_key,
            api_host_url,
            port,
            public_key_der_base64,
        }
    }
}
