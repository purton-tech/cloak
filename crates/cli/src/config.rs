use crate::{keyring, Cli};
use p256::{
    pkcs8::{DecodePrivateKey, EncodePublicKey},
    SecretKey,
};
use std::fs;
use std::{collections::HashMap, error::Error};

pub struct Config {
    pub secret_key: Option<SecretKey>,
    pub keyring_password: Option<String>,
    pub api_host_url: String,
}

impl Config {
    pub fn configure(cli: &Cli) -> Result<Config, Box<dyn Error>> {
        let secret_key;

        if let Some(ecdh_private_key) = &cli.ecdh_private_key {
            // Did the user pass in a private key as an env car?
            secret_key = Some(
                SecretKey::from_pkcs8_pem(ecdh_private_key).map_err(|_| "Problem loading key")?,
            );
        } else {
            // Did the user supply a key in a file
            let pem_string = fs::read_to_string(&cli.ecdh_private_key_file);
            if let Ok(pem_string) = pem_string {
                secret_key = Some(
                    SecretKey::from_pkcs8_pem(&pem_string).map_err(|_| "Problem loading key")?,
                );
            } else {
                secret_key = None;
            }
        }

        let config = Config {
            secret_key,
            keyring_password: cli.keyring_password.clone(),
            api_host_url: cli.api_host_url.clone(),
        };
        Ok(config)
    }

    pub fn get_password(&self) -> String {
        if let Some(password) = self.keyring_password.clone() {
            password
        } else {
            println!("Enter your password to decrypt the service account.");
            println!("NOTE: You can set the env var CLOAK_KEYRING_PASSWORD to a password so you don't have to type this each time.");
            rpassword::prompt_password("Your password: ").unwrap()
        }
    }

    pub fn set_password(&self) -> String {
        if let Some(password) = self.keyring_password.clone() {
            password
        } else {
            println!("Please set a password to encrypt this key (Note this doesn't have to be your cloak password)");
            rpassword::prompt_password("Your password: ").unwrap()
        }
    }

    /***
     * We retrive secrets using the public key of the ECDH private key.
     * The question is which key to use? We have 3 options.
     *
     * 1) The user passed in an ECDH private key in PEM format as an env var.
     * 2) The user has a ECDH private key in a .pem file. (i.e. cloak.pem).
     * 3) The user has a private key stored in the cloak keyring
     *
     * We look for the key in that order.
     */
    pub async fn get_secrets(&self) -> Option<HashMap<String, String>> {
        // Did the user supply a key via and env var or a file?
        if let Some(secret_key) = &self.secret_key {
            let service_account_public_key = secret_key.public_key();

            let public_key_der = service_account_public_key.to_public_key_der().unwrap();
            let public_key_der_base64 = base64::encode(public_key_der);
            let secrets =
                grpc_api::get_secrets(secret_key, &self.api_host_url, &public_key_der_base64)
                    .await
                    .expect(
                        "Problem retreiving secrets, 
                        do you have the correct service account key?",
                    );
            Some(secrets)
        } else {
            // Try and get a key from the keyring
            let keyring = keyring::KeyRing::load();
            if keyring.accounts.is_empty() {
                println!(
                    "You didn't pass in a key as an env var or as a file, to top it all,
                    there's no keys in your cloak keyring. You need to import a key."
                );
                return None;
            }

            let key = keyring.get_selected_key();
            let password = self.get_password();
            let secret_key =
                SecretKey::from_pkcs8_encrypted_pem(&key, password).expect("Problem parsing key");
            let service_account_public_key = secret_key.public_key();

            let public_key_der = service_account_public_key.to_public_key_der().unwrap();
            let public_key_der_base64 = base64::encode(public_key_der);
            let secrets =
                grpc_api::get_secrets(&secret_key, &self.api_host_url, &public_key_der_base64)
                    .await
                    .expect(
                        "Problem retreiving secrets, 
                        do you have the correct service account key?",
                    );
            Some(secrets)
        }
    }

    pub async fn get_public_key(&self) -> Option<String> {
        if let Some(secret_key) = &self.secret_key {
            let service_account_public_key = secret_key.public_key();

            let public_key_der = service_account_public_key.to_public_key_der().unwrap();
            let public_key_der_base64 = base64::encode(public_key_der);
            return Some(public_key_der_base64);
        }

        None
    }
}
