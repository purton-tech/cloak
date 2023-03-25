use crate::Cli;
use p256::{
    pkcs8::{DecodePrivateKey, EncodePublicKey},
    SecretKey,
};
use std::error::Error;
use std::fs;

pub struct Config {
    pub secret_key: SecretKey,
    pub api_host_url: String,
    pub public_key_der_base64: String,
}

impl Config {
    pub fn configure(cli: &Cli) -> Result<Config, Box<dyn Error>> {
        let secret_key;

        if let Some(ecdh_private_key) = &cli.ecdh_private_key {
            secret_key =
                SecretKey::from_pkcs8_pem(ecdh_private_key).map_err(|_| "Problem loading key")?;
        } else {
            let pem_string = fs::read_to_string(&cli.ecdh_private_key_file)?;
            secret_key =
                SecretKey::from_pkcs8_pem(&pem_string).map_err(|_| "Problem loading key")?;
        }

        let service_account_public_key = secret_key.public_key();

        let public_key_der = service_account_public_key.to_public_key_der().unwrap();
        let public_key_der_base64 = base64::encode(public_key_der);

        let config = Config {
            secret_key,
            api_host_url: cli.api_host_url.clone(),
            public_key_der_base64,
        };
        Ok(config)
    }

    pub fn set_password(&self) -> String {
        println!("Please set a password to encrypt this key (Note this doesn't have to be your cloak password)");
        rpassword::prompt_password("Your password: ").unwrap()
    }
}
