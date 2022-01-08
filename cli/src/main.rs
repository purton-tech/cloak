pub mod vault {
    tonic::include_proto!("vault");
}

mod config;

use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::process::{Command, Stdio};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "git")]
#[clap(about = "A fictional versioning CLI")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs a program and passes secret environment variables to it
    #[clap(external_subcommand)]
    Run(Vec<OsString>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::new();

    let client = vault::vault_client::Vault::new(config.api_host_url);

    let response = client
        .get_service_account(vault::GetServiceAccountRequest {
            ecdh_public_key: "".to_string(),
        })
        .await?;

    dbg!(&response);

    let args = Cli::parse();

    match &args.command {
        Commands::Run(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
            let filtered_env: HashMap<String, String> = env::vars()
                .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH")
                .collect();

            Command::new("printenv")
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .env_clear()
                .envs(&filtered_env)
                .spawn()
                .expect("printenv failed to start");
        }
    }

    Ok(())
}
