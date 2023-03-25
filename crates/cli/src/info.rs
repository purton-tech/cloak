use crate::{config::Config, keyring};

pub async fn info(config: &Config) {
    if let Some(public_key) = config.get_public_key().await {
        println!("A key was in via an env var or a file");
        println!("Public Key : {}", public_key);
    }
    let keyring = keyring::KeyRing::load();
    println!(
        "You cloak key ring contains keys for {} service account(s).",
        keyring.accounts.len()
    );
    for (index, account) in keyring.accounts.iter().enumerate() {
        println!("{}. {}", index + 1, account.name);
    }
}
