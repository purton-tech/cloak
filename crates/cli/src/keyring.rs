use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default)]
pub struct KeyRing {
    pub keys: Vec<String>,
}

impl KeyRing {
    pub fn load() -> Self {
        let config_dir = dirs::config_dir().expect("Cloak couldn't get a config directory.");

        let data = fs::read_to_string(format!("{}/cloak.json", config_dir.to_string_lossy()));

        if let Ok(data) = data {
            let keyring: KeyRing =
                serde_json::from_str(&data).expect("JSON does not have correct format.");
            keyring
        } else {
            KeyRing::default()
        }
    }
}
