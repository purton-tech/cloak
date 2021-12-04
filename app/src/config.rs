use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    // The database
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let port: u16 = if env::var("PORT").is_ok() {
            env::var("PORT").unwrap().parse::<u16>().unwrap()
        } else {
            3000
        };

        Config {
            port,
            database_url: env::var("WEB_APP_DATABASE_URL").expect("WEB_APP_DATABASE_URL not set"),
        }
    }
}
