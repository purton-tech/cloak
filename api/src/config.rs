use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    // The database
    pub database_url: String,
    // The gRPC server
    pub vault_server_listen_address: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            database_url: env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set"),
            vault_server_listen_address: env::var("VAULT_SERVER_LISTEN_ADDRESS")
                .expect("VAULT_SERVER_LISTEN_ADDRESS not set"),
        }
    }
}
