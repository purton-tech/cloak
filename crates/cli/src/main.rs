mod config;
mod import;
mod keyring;
mod secrets;
mod select;

use clap::{Parser, Subcommand};
use std::ffi::OsString;

use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio};

/// A fictional versioning CLI
#[derive(Parser)]
#[clap(name = "cloak")]
#[clap(about = "Secrets automation")]
pub struct Cli {
    #[clap(short, long, env = "ECDH_PRIVATE_KEY")]
    pub ecdh_private_key: Option<String>,

    #[clap(long, default_value_t=String::from("./cloak.pem"))]
    pub ecdh_private_key_file: String,

    #[clap(short, long, env="API_HOST_URL", default_value_t=String::from("https://app.cloak.software"))]
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
    Import {
        name: String,
        key: String,
    },
    Secrets,
    Select,
    Env,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let config = config::Config::configure(&args)?;

    // cargo run -- --api-host-url=http://envoy:7100 run printenv
    // cargo run -- info
    // cargo run -- --api-host-url=http://envoy:7100 secrets
    match &args.command {
        Commands::Run(args) => {
            let secrets = config.get_secrets().await;

            if let Some(secrets) = secrets {
                let filtered_env: HashMap<String, String> = env::vars()
                    .filter(|(k, _)| k != "ECDH_PRIVATE_KEY")
                    .collect();

                let filtered_env: HashMap<String, String> =
                    filtered_env.into_iter().chain(secrets).collect();

                let cmd_args = insert_secrets(&args[1..], &filtered_env).await;

                println!("Calling out to {:?} with {:?}", &args[0], &cmd_args);

                let mut child = Command::new(&args[0])
                    .args(&cmd_args)
                    .stdin(Stdio::null())
                    .stdout(Stdio::inherit())
                    .envs(&filtered_env)
                    .spawn()
                    .expect("Failed to run command");

                child.wait().expect("failed to wait on child");
            }
        }
        Commands::Info => {
            //println!("Public Key {:?}", config.public_key_der_base64);
        }
        Commands::Secrets => {
            secrets::secrets(&config).await;
        }
        Commands::Import { name, key } => {
            import::import(name.into(), key.into(), &config).await;
        }
        Commands::Select => {
            select::select().await;
        }
        Commands::Env => {
            let secrets = config.get_secrets().await;

            if let Some(secrets) = secrets {
                for (name, value) in secrets.into_iter() {
                    println!("{}={}", name, value);
                }
            }
        }
    }

    Ok(())
}

// The user may wish to use an env var on the command line, so we process them here
async fn insert_secrets(cmd_args: &[OsString], secrets: &HashMap<String, String>) -> Vec<OsString> {
    let mut process_args: Vec<OsString> = Default::default();

    for arg in cmd_args.iter() {
        let mut arg = OsString::from(arg);
        for (name, value) in secrets.iter() {
            if let Ok(arg_to_check) = arg.clone().into_string() {
                let env_name = format!("${}", name);
                if arg_to_check == env_name {
                    arg = OsString::from(value);
                }
            }
        }
        process_args.push(arg);
    }

    process_args
}
