use std::path::Path;

use super::config::RuntimeConfig;

pub fn estimate_ram_gb(model_path: &Path) -> f64 {
    let size_bytes = std::fs::metadata(model_path)
        .map(|m| m.len())
        .unwrap_or(0);
    size_bytes as f64 / (0.6 * 1024.0 * 1024.0 * 1024.0)
}

pub fn create_default_config(model_path: &Path) -> RuntimeConfig {
    RuntimeConfig::auto_tune(model_path)
}
