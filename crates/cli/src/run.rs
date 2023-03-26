use std::ffi::OsString;

use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio};

use crate::config::Config;

pub async fn run(config: &Config, args: &[OsString]) {
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
