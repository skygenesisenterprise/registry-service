use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub registry_url: String,
    pub auth_token: Option<String>,
    pub cache_dir: PathBuf,
    pub install_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            registry_url: "http://localhost:3000".to_string(),
            auth_token: None,
            cache_dir: home.join(".cpkgs/cache"),
            install_dir: home.join(".cpkgs/packages"),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cpkgs/config.toml");

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cpkgs");

        fs::create_dir_all(&config_path)?;
        
        let config_file = config_path.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(config_file, content)?;

        fs::create_dir_all(&self.cache_dir)?;
        fs::create_dir_all(&self.install_dir)?;

        Ok(())
    }

    pub fn set_auth_token(&mut self, token: String) {
        self.auth_token = Some(token);
    }

    pub fn clear_auth_token(&mut self) {
        self.auth_token = None;
    }
}