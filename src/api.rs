use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::core::models::model_info::ModelInfo;
use crate::core::registry::manager::ModelRegistry;
use crate::hub::result::HubModelResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSummary {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub size_gb: f64,
    pub quant_type: String,
    pub status: String,
    pub arch: String,
    pub context_length: u64,
}

impl From<&ModelInfo> for ModelSummary {
    fn from(info: &ModelInfo) -> Self {
        Self {
            name: info.name.clone(),
            path: info.path.clone(),
            size_bytes: info.size_bytes,
            size_gb: info.size_gb(),
            quant_type: info.quant_type.to_string(),
            status: info.status.to_string(),
            arch: info.metadata.arch.clone(),
            context_length: info.metadata.context_length,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubModelSummary {
    pub repo_id: String,
    pub filename: String,
    pub size_bytes: u64,
    pub size_gb: f64,
    pub downloads: u64,
    pub likes: u64,
}

impl From<&HubModelResult> for HubModelSummary {
    fn from(result: &HubModelResult) -> Self {
        Self {
            repo_id: result.repo_id.clone(),
            filename: result.filename.clone(),
            size_bytes: result.size_bytes,
            size_gb: result.size_gb(),
            downloads: result.downloads,
            likes: result.likes,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOptions {
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub system_prompt: Option<String>,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            temperature: 0.8,
            top_p: 0.95,
            system_prompt: None,
        }
    }
}

pub struct LlmNest {
    registry: ModelRegistry,
}

impl LlmNest {
    pub fn new() -> Result<Self> {
        let mut registry = ModelRegistry::new(None);
        registry.scan()?;
        Ok(Self { registry })
    }

    pub fn with_directory(dir: PathBuf) -> Result<Self> {
        let mut registry = ModelRegistry::new(Some(dir));
        registry.scan()?;
        Ok(Self { registry })
    }

    pub fn rescan(&mut self) -> Result<()> {
        self.registry.scan()
    }

    pub fn list_models(&self) -> Vec<ModelSummary> {
        self.registry
            .list_models()
            .iter()
            .map(ModelSummary::from)
            .collect()
    }

    pub fn get_model(&self, name: &str) -> Option<ModelSummary> {
        self.registry.get_model(name).map(ModelSummary::from)
    }

    pub fn delete_model(&mut self, name: &str) -> Result<bool> {
        self.registry.delete_model(name)
    }

    pub fn set_alias(&self, alias: &str, model_name: &str) -> Result<()> {
        self.registry.set_alias(alias, model_name)
    }

    pub fn remove_alias(&self, alias: &str) -> Result<bool> {
        self.registry.remove_alias(alias)
    }

    pub fn get_aliases(&self) -> Result<std::collections::HashMap<String, String>> {
        self.registry.get_aliases()
    }

    pub async fn search_hub(&self, query: &str, limit: usize) -> Result<Vec<HubModelSummary>> {
        let results = crate::hub::search::search_gguf(query, limit).await?;
        Ok(results.iter().map(HubModelSummary::from).collect())
    }

    pub async fn search_hub_repo(&self, repo_id: &str) -> Result<Vec<HubModelSummary>> {
        let results = crate::hub::search::search_repo_files(repo_id).await?;
        Ok(results.iter().map(HubModelSummary::from).collect())
    }

    pub async fn download_model(
        &self,
        repo_id: &str,
        filename: &str,
        on_progress: Option<Box<dyn Fn(u64, u64) + Send>>,
    ) -> Result<PathBuf> {
        crate::hub::download::download_model(repo_id, filename, None, on_progress).await
    }
}

impl Default for LlmNest {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            registry: ModelRegistry::new(None),
        })
    }
}

// ============ Runtime API (需要 runtime feature) ============

#[cfg(feature = "runtime")]
pub struct LlmRuntime {
    backend: crate::core::runtime::llama_cpp::LlamaCppBackend,
    config: crate::core::runtime::config::RuntimeConfig,
}

#[cfg(feature = "runtime")]
impl LlmRuntime {
    pub fn new() -> Self {
        use crate::core::runtime::config::RuntimeConfig;
        use crate::core::runtime::llama_cpp::LlamaCppBackend;

        Self {
            backend: LlamaCppBackend::new(),
            config: RuntimeConfig::default(),
        }
    }

    pub fn with_options(max_tokens: u32, temperature: f32) -> Self {
        use crate::core::runtime::config::RuntimeConfig;
        use crate::core::runtime::llama_cpp::LlamaCppBackend;

        let mut config = RuntimeConfig::default();
        config.max_tokens = max_tokens;
        config.temperature = temperature;

        Self {
            backend: LlamaCppBackend::new(),
            config,
        }
    }

    pub async fn load_model(&mut self, model_path: &PathBuf) -> Result<()> {
        use crate::core::runtime::backend::RuntimeBackend;
        use crate::core::runtime::config::RuntimeConfig;

        self.config = RuntimeConfig::auto_tune(model_path);
        self.backend.load_model(model_path, Some(self.config.clone())).await
    }

    pub async fn load_model_with_config(
        &mut self,
        model_path: &PathBuf,
        options: GenerateOptions,
    ) -> Result<()> {
        use crate::core::runtime::backend::RuntimeBackend;
        use crate::core::runtime::config::RuntimeConfig;

        let mut config = RuntimeConfig::auto_tune(model_path);
        config.max_tokens = options.max_tokens;
        config.temperature = options.temperature;
        config.top_p = options.top_p;
        if let Some(system) = options.system_prompt {
            config.system_prompt = system;
        }

        self.config = config.clone();
        self.backend.load_model(model_path, Some(config)).await
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        use crate::core::runtime::backend::RuntimeBackend;
        use futures::StreamExt;

        let stream = self.backend.generate(prompt, Some(self.config.clone())).await?;
        tokio::pin!(stream);

        let mut output = String::new();
        while let Some(token) = stream.next().await {
            output.push_str(&token?);
        }
        Ok(output)
    }

    pub async fn generate_stream(
        &self,
        prompt: &str,
    ) -> Result<impl futures::Stream<Item = Result<String>>> {
        use crate::core::runtime::backend::RuntimeBackend;

        let stream = self.backend.generate(prompt, Some(self.config.clone())).await?;
        Ok(stream)
    }

    pub async fn chat(&self, messages: &[ChatMessage]) -> Result<String> {
        use crate::core::runtime::backend::{ChatMessage as BackendChatMessage, RuntimeBackend};
        use futures::StreamExt;

        let backend_messages: Vec<BackendChatMessage> = messages
            .iter()
            .map(|m| BackendChatMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect();

        let stream = self
            .backend
            .generate_chat(&backend_messages, Some(self.config.clone()))
            .await?;
        tokio::pin!(stream);

        let mut output = String::new();
        while let Some(token) = stream.next().await {
            output.push_str(&token?);
        }
        Ok(output)
    }

    pub async fn chat_stream(
        &self,
        messages: &[ChatMessage],
    ) -> Result<impl futures::Stream<Item = Result<String>>> {
        use crate::core::runtime::backend::{ChatMessage as BackendChatMessage, RuntimeBackend};

        let backend_messages: Vec<BackendChatMessage> = messages
            .iter()
            .map(|m| BackendChatMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect();

        let stream = self
            .backend
            .generate_chat(&backend_messages, Some(self.config.clone()))
            .await?;
        Ok(stream)
    }

    pub fn is_loaded(&self) -> bool {
        use crate::core::runtime::backend::RuntimeBackend;
        self.backend.is_loaded()
    }

    pub async fn unload(&mut self) -> Result<()> {
        use crate::core::runtime::backend::RuntimeBackend;
        self.backend.unload().await
    }
}

#[cfg(feature = "runtime")]
impl Default for LlmRuntime {
    fn default() -> Self {
        Self::new()
    }
}
