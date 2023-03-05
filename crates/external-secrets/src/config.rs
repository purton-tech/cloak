#[derive(Clone, Debug)]
pub struct Config {
    // All secrets in Cloak are encrypted using ECDH, we need the private
    // To decrypt them.
    pub ecdh_private_key: String,
}

impl Config {
    pub fn new() -> Config {
        let ecdh_private_key = std::env::var("ECDH_PRIVATE_KEY").expect("ECDH_PRIVATE_KEY not set");

        Config {
            ecdh_private_key,
        }
    }
}