/**
 * We derive all our configuration from environment variables. This struct
 * handles parsing those variables.
 */
#[derive(Clone, Debug)]
pub struct Config {
    // All secrets in Cloak are encrypted using ECDH, we need the private key
    // to decrypt them.
    pub ecdh_private_key: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Config {
        let ecdh_private_key = std::env::var("ECDH_PRIVATE_KEY").expect("ECDH_PRIVATE_KEY not set");

        let port: u16 = if std::env::var("PORT").is_ok() {
            std::env::var("PORT").unwrap().parse::<u16>().unwrap()
        } else {
            7105
        };

        Config {
            ecdh_private_key,
            port,
        }
    }
}
