use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub credentials: Credentials,
    pub network: Network,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub user_id: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub wlanacip: String,
    pub wlanacname: String,
    pub mac: String,
    pub vlan: String,
    pub base_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let paths = [
            PathBuf::from("config.toml"),
            dirs_config_path(),
        ];

        for path in &paths {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                let config: Config = toml::from_str(&content)?;
                println!("Loaded config from: {}", path.display());
                return Ok(config);
            }
        }

        let config = Self::prompt()?;
        if let Some(parent) = paths[1].parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let toml_str = toml::to_string_pretty(&config)?;
        std::fs::write(&paths[1], &toml_str)?;
        println!("Config saved to: {}", paths[1].display());
        Ok(config)
    }

    fn prompt() -> Result<Self, Box<dyn std::error::Error>> {
        println!("No config file found. Let's set up.\n");

        let user_id = prompt_input("User ID: ");
        let password = prompt_input("Password: ");
        let wlanacip = prompt_input("WLAN AC IP [10.240.192.33]: ");
        let wlanacip = if wlanacip.is_empty() { "10.240.192.33".into() } else { wlanacip };
        let wlanacname = prompt_input("WLAN AC Name [gxmzu]: ");
        let wlanacname = if wlanacname.is_empty() { "gxmzu".into() } else { wlanacname };
        let mac = prompt_input("MAC Address [00:00:00:00:00:00]: ");
        let mac = if mac.is_empty() { "00:00:00:00:00:00".into() } else { mac };
        let base_url = prompt_input("Base URL [https://aaa.gxmzu.edu.cn]: ");
        let base_url = if base_url.is_empty() { "https://aaa.gxmzu.edu.cn".into() } else { base_url };

        Ok(Config {
            credentials: Credentials { user_id, password },
            network: Network {
                wlanacip,
                wlanacname,
                mac,
                vlan: "0".into(),
                base_url,
            },
        })
    }
}

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn dirs_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config").join("pulse").join("config.toml")
}
