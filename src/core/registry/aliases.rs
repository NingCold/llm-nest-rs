use std::collections::HashMap;
use std::fs;

use anyhow::{Context, Result};

use crate::core::storage::paths;

pub fn load_aliases() -> Result<HashMap<String, String>> {
    let path = paths::aliases_path()?;
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let data = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read aliases file: {}", path.display()))?;
    let aliases: HashMap<String, String> =
        serde_json::from_str(&data).context("Failed to parse aliases file")?;
    Ok(aliases)
}

pub fn save_aliases(aliases: &HashMap<String, String>) -> Result<()> {
    let path = paths::aliases_path()?;
    let data = serde_json::to_string_pretty(aliases)?;
    super::super::storage::file_ops::atomic_write(&path, data.as_bytes())?;
    Ok(())
}

pub fn set_alias(alias: &str, model_name: &str) -> Result<()> {
    let mut aliases = load_aliases()?;
    aliases.insert(alias.to_string(), model_name.to_string());
    save_aliases(&aliases)
}

pub fn remove_alias(alias: &str) -> Result<bool> {
    let mut aliases = load_aliases()?;
    let removed = aliases.remove(alias).is_some();
    if removed {
        save_aliases(&aliases)?;
    }
    Ok(removed)
}

pub fn resolve_alias(name: &str) -> Result<String> {
    let aliases = load_aliases()?;
    Ok(aliases.get(name).cloned().unwrap_or_else(|| name.to_string()))
}
