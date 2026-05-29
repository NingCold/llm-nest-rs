use std::fs;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::core::storage::paths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn default_lang() -> String {
    "en".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            lang: default_lang(),
        }
    }
}

pub fn load_config() -> Result<AppConfig> {
    let path = paths::config_file_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let data = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: AppConfig =
        serde_json::from_str(&data).context("Failed to parse config file")?;
    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = paths::config_file_path()?;
    let data = serde_json::to_string_pretty(config)?;
    crate::core::storage::file_ops::atomic_write(&path, data.as_bytes())?;
    Ok(())
}

pub fn get_lang() -> String {
    load_config().map(|c| c.lang).unwrap_or_else(|_| "en".to_string())
}

pub fn set_lang(lang: &str) -> Result<()> {
    let mut config = load_config()?;
    config.lang = lang.to_string();
    save_config(&config)
}
