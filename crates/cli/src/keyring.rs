use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ServiceAccount {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct KeyRing {
    pub selected_account: u32,
    pub accounts: Vec<ServiceAccount>,
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

    pub fn add_service_account(&mut self, name: String, key: String) {
        self.accounts.push(ServiceAccount { name, key });
    }

    pub fn list_service_accounts(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.name.clone())
            .collect()
    }

    pub fn select_service_account(&mut self, index: u32) {
        self.selected_account = index;
    }

    fn config_dir() -> String {
        let config_dir = dirs::config_dir().expect("Cloak couldn't get a config directory.");
        fs::create_dir_all(&config_dir).expect("Problem creating the config folder");
        format!("{}/cloak-keyring.json", config_dir.to_string_lossy())
    }
}
