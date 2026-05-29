use std::path::PathBuf;

use anyhow::Result;

use crate::core::registry::manager::ModelRegistry;

pub struct CliContext {
    pub registry: ModelRegistry,
    pub models_dir: Option<PathBuf>,
}

impl CliContext {
    pub fn new(models_dir: Option<PathBuf>) -> Result<Self> {
        let mut registry = ModelRegistry::new(models_dir.clone());
        registry.scan()?;

        Ok(Self {
            registry,
            models_dir,
        })
    }

    pub fn rescan(&mut self) -> Result<()> {
        self.registry.scan()
    }
}
