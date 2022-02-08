pub mod vault {
    tonic::include_proto!("vault");
}

mod config;

use clap::{Parser, Subcommand};
use p256::ecdh::SharedSecret;
use p256::{elliptic_curve::ecdh, pkcs8::DecodePublicKey, PublicKey};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::process::{Command, Stdio};

use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead, Payload};
use aes_gcm::Aes256Gcm;
use dotenv::dotenv;

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "cloak")]
#[clap(about = "Secrets automation")]
pub struct Cli {
    #[clap(short, long, env = "ECDH_PRIVATE_KEY")]
    pub ecdh_private_key: Option<String>,

    #[clap(long, default_value_t=String::from("./cloak.pem"))]
    pub ecdh_private_key_file: String,

    #[clap(short, long, env="API_HOST_URL", default_value_t=String::from("https://cloak.software"))]
    pub api_host_url: String,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs a program and passes secret environment variables to it
    #[clap(external_subcommand)]
    Run(Vec<OsString>),
    Info,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Cli::parse();
    let config = config::Config::configure(&args)?;

    // cargo run -- printenv
    // cargo run -- echo $HOSTNAME
    match &args.command {
        Commands::Run(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);

            let env_vars_to_inject = get_secrets(&config).await?;

            let filtered_env: HashMap<String, String> = env::vars()
                .filter(|&(ref k, _)| k != "ECDH_PRIVATE_KEY")
                .collect();

            let filtered_env: HashMap<String, String> =
                filtered_env.into_iter().chain(env_vars_to_inject).collect();

            Command::new(&args[0])
                .args(&args[1..])
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .envs(&filtered_env)
                .spawn()
                .expect("Failed to run command");
        }
        Commands::Info => {
            println!("Public Key {:?}", config.public_key_der_base64);
        }
    }

    Ok(())
}

async fn get_secrets(
    config: &config::Config,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let response = config
        .client
        .get_service_account(vault::GetServiceAccountRequest {
            ecdh_public_key: config.public_key_der_base64.clone(),
        })
        .await?;

    let vault_key_der = base64::decode(response.vault_public_ecdh_key).unwrap();

    let vault_public_key = PublicKey::from_public_key_der(&vault_key_der).unwrap();

    let shared_secret = ecdh::diffie_hellman(
        config.secret_key.to_nonzero_scalar(),
        vault_public_key.as_affine(),
    );

    let aad = transform_u32_to_array_of_u8(response.service_account_id);

    let mut env_vars_to_inject: HashMap<String, String> = Default::default();
    for secret in response.secrets {
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
    if let Some(nonce) = nonce_and_cipher.get(0) {
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
