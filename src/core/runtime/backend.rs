use std::path::Path;
use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;

use super::config::RuntimeConfig;

pub type TokenStream = Pin<Box<dyn Stream<Item = Result<String>> + Send>>;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait RuntimeBackend: Send + Sync {
    async fn load_model(&mut self, path: &Path, config: Option<RuntimeConfig>) -> Result<()>;
    async fn generate(&self, prompt: &str, config: Option<RuntimeConfig>) -> Result<TokenStream>;
    async fn generate_chat(
        &self,
        messages: &[ChatMessage],
        config: Option<RuntimeConfig>,
    ) -> Result<TokenStream>;
    async fn unload(&mut self) -> Result<()>;
    fn is_loaded(&self) -> bool;
    fn name(&self) -> &str;
}
