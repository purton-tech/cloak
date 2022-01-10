pub mod vault {
    tonic::include_proto!("vault");
}

mod config;

use clap::{Parser, Subcommand};
use p256::{
    elliptic_curve::ecdh,
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePublicKey},
    PublicKey, SecretKey,
};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
//use std::process::{Command, Stdio};

use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead, Payload};
use aes_gcm::Aes256Gcm;

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "git")]
#[clap(about = "A fictional versioning CLI")]
struct Cli {
    #[clap(short, long)]
    ecdh_private_key: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs a program and passes secret environment variables to it
    #[clap(external_subcommand)]
    Run(Vec<OsString>),
}

const PKCS8_PRIVATE_KEY_PEM: &str = include_str!("../key.pem");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::new();

    let secret_key = SecretKey::from_pkcs8_pem(PKCS8_PRIVATE_KEY_PEM).unwrap();
    let service_account_public_key = secret_key.public_key();

    let client = vault::vault_client::Vault::new(config.api_host_url);

    let public_key_der = service_account_public_key.to_public_key_der().unwrap();
    let public_key_der_base64 = base64::encode(public_key_der);

    let response = client
        .get_service_account(vault::GetServiceAccountRequest {
            ecdh_public_key: public_key_der_base64,
        })
        .await?;

    let vault_key_der = base64::decode(response.vault_public_ecdh_key).unwrap();

    let vault_public_key = PublicKey::from_public_key_der(&vault_key_der).unwrap();

    let shared_secret =
        ecdh::diffie_hellman(secret_key.to_nonzero_scalar(), vault_public_key.as_affine());

    for secret in response.secrets {
        let nonce_and_cipher: Vec<&str> = secret.encrypted_name.split('|').collect();

        if let Some(nonce) = nonce_and_cipher.get(0) {
            if let Some(cipher) = nonce_and_cipher.get(1) {
                let nonce_bytes = base64::decode(nonce).unwrap();
                let cipher_bytes = base64::decode(cipher).unwrap();

                let payload = Payload {
                    msg: &cipher_bytes,
                    aad: Default::default(),
                };
                let key: &GenericArray<u8, _> = GenericArray::from_slice(shared_secret.as_bytes());
                let nonce = GenericArray::from_slice(&nonce_bytes as &[u8]);

                let cipher = <Aes256Gcm>::new(key);
                let plaintext = cipher.decrypt(nonce, payload).unwrap();
                let plaintext = std::str::from_utf8(&plaintext).unwrap();

                dbg!(plaintext);
            }
        }
    }

    // cargo run -- --ecdh-private-key $ECDH_PRIVATE_KEY run ls
    let args = Cli::parse();
    match &args.command {
        Commands::Run(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
            let _filtered_env: HashMap<String, String> = env::vars()
                .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
                .collect();

            /***Command::new("printenv")
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .env_clear()
            .envs(&filtered_env)
            .spawn()
            .expect("printenv failed to start");**/
        }
    }

    Ok(())
}
