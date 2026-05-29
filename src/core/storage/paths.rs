use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn models_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let dir = home.join(".llmn").join("models");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create models directory: {}", dir.display()))?;
    Ok(dir)
}

pub fn cache_dir() -> Result<PathBuf> {
    let dir = dirs::cache_dir()
        .unwrap_or_else(|| {
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            home.join(".cache").join("llm-nest")
        })
        .join("llm-nest");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create cache directory: {}", dir.display()))?;
    Ok(dir)
}

pub fn config_dir() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| {
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            home.join(".config").join("llm-nest")
        })
        .join("llm-nest");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create config directory: {}", dir.display()))?;
    Ok(dir)
}

pub fn model_path(name: &str) -> Result<PathBuf> {
    let dir = models_dir()?;
    Ok(dir.join(format!("{name}.gguf")))
}

pub fn aliases_path() -> Result<PathBuf> {
    let dir = models_dir()?;
    Ok(dir.join("aliases.json"))
}

pub fn config_file_path() -> Result<PathBuf> {
    let dir = config_dir()?;
    Ok(dir.join("config.json"))
}
