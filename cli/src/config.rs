use std::env;

pub struct Config {
    pub api_host_url: String,
}

impl Config {
    pub fn new() -> Config {
        let api_host_url: String = if env::var("API_HOST_URL").is_ok() {
            env::var("API_HOST_URL").unwrap()
        } else {
            "https://keyvault.authn.tech".to_string()
        };

        Config { api_host_url }
    }
}
