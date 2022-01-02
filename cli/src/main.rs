pub mod vault {
    tonic::include_proto!("vault");
}

mod config;

use clap::{AppSettings, Parser, Subcommand};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
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
    /// pushes things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Init {
        /// The remote to target
        remote: String,
    },
    /// adds things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Read {
        /// Stuff to add
        #[clap(required = true, parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    /// adds things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Write {
        /// Stuff to add
        #[clap(required = true, parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    /// pushes things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Generate {
        /// The remote to target
        remote: String,
    },
    /// pushes things
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Ls {
        /// The remote to target
        remote: String,
    },
    /// Runs a program and passes secret environment variables to it
    #[clap(external_subcommand)]
    Run(Vec<OsString>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::new();

    let client = vault::vault_client::Vault::new(config.api_host_url);

    let response = client.list_vaults(vault::ListVaultsRequest {}).await?;

    println!("RESPONSE={:?}", response);

    let args = Cli::parse();

    match &args.command {
        Commands::Init { remote } => {
            println!("Cloning {}", remote);
        }
        Commands::Read { path } => {
            println!("Adding {:?}", path);
        }
        Commands::Write { path } => {
            println!("Adding {:?}", path);
        }
        Commands::Generate { remote } => {
            println!("Adding {:?}", remote);
        }
        Commands::Ls { remote } => {
            println!("Adding {:?}", remote);
        }
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
