mod config;
mod import;
mod info;
mod keyring;
mod run;
mod secrets;
mod select;

use clap::{Parser, Subcommand};
use std::ffi::OsString;

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
        Commands::Run(args) => run::run(&config, args).await,
        Commands::Info => {
            info::info(&config).await;
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
