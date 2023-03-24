use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct KeyRing {
    pub keys: Vec<String>,
}

impl KeyRing {
    pub fn load() -> Self {
        let config_dir = Self::config_dir();

        dbg!(&config_dir);

        let data = fs::read_to_string(config_dir);

        if let Ok(data) = data {
            let keyring: KeyRing =
                serde_json::from_str(&data).expect("JSON does not have correct format.");
            keyring
        } else {
            KeyRing::default()
        }
    }

    pub fn save(&self) {
        let config_dir = Self::config_dir();
        fs::write(config_dir, serde_json::to_string_pretty(&self).unwrap())
            .expect("Problem writing to keyring");
    }

    pub fn add_key(&mut self, key: String) {
        self.keys.push(key);
    }

    fn config_dir() -> String {
        let config_dir = dirs::config_dir().expect("Cloak couldn't get a config directory.");
        fs::create_dir_all(&config_dir).expect("Problem creating the config folder");
        format!("{}/cloak.json", config_dir.to_string_lossy())
    }
}
