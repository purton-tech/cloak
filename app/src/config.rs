use std::env;
use tonic::codegen::http::Uri;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    // The gRPC server
    pub vault_server_uri: Uri,
}

impl Config {
    pub fn new() -> Config {
        let port: u16 = if env::var("PORT").is_ok() {
            env::var("PORT").unwrap().parse::<u16>().unwrap()
        } else {
            7101
        };

        let server = env::var("VAULT_SERVER_URL").expect("VAULT_SERVER_URL not set");

        let vault_server_uri = server.parse::<Uri>().expect("Could not parse server URI");

        Config {
            port,
            vault_server_uri,
        }
    }
}
