use std::path::PathBuf;

use anyhow::Result;

use super::aliases;
use crate::core::models::model_info::ModelInfo;
use crate::core::storage::model_store;

pub struct ModelRegistry {
    directory: Option<PathBuf>,
    models: Vec<ModelInfo>,
}

impl ModelRegistry {
    pub fn new(directory: Option<PathBuf>) -> Self {
        Self {
            directory,
            models: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> Result<()> {
        let dir = self.directory.as_deref();
        self.models = model_store::scan_models(dir)?;
        Ok(())
    }

    pub fn list_models(&self) -> &[ModelInfo] {
        &self.models
    }

    pub fn get_model(&self, name: &str) -> Option<&ModelInfo> {
        let resolved = aliases::resolve_alias(name).ok()?;
        self.models
            .iter()
            .find(|m| m.name == resolved || m.stem() == resolved)
    }

    pub fn delete_model(&mut self, name: &str) -> Result<bool> {
        let resolved = aliases::resolve_alias(name)?;
        if let Some(model) = self
            .models
            .iter()
            .find(|m| m.name == resolved || m.stem() == resolved)
        {
            let path = model.path.clone();
            model_store::delete_model(&path)?;
            self.models.retain(|m| m.path != path);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn set_alias(&self, alias: &str, model_name: &str) -> Result<()> {
        aliases::set_alias(alias, model_name)
    }

    pub fn remove_alias(&self, alias: &str) -> Result<bool> {
        aliases::remove_alias(alias)
    }

    pub fn get_aliases(&self) -> Result<std::collections::HashMap<String, String>> {
        aliases::load_aliases()
    }
}
