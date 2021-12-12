use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    // The gRPC server
    pub app_database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let port: u16 = if env::var("PORT").is_ok() {
            env::var("PORT").unwrap().parse::<u16>().unwrap()
        } else {
            7103
        };

        let app_database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        Config {
            port,
            app_database_url,
        }
    }
}
