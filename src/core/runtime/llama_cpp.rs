use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use encoding_rs::UTF_8;
use futures::stream;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;

use super::backend::{ChatMessage, RuntimeBackend, TokenStream};
use super::config::RuntimeConfig;

pub struct LlamaCppBackend {
    backend: Option<LlamaBackend>,
    model: Option<Arc<LlamaModel>>,
    config: Option<RuntimeConfig>,
    model_path: Option<std::path::PathBuf>,
}

impl Default for LlamaCppBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LlamaCppBackend {
    pub fn new() -> Self {
        Self {
            backend: None,
            model: None,
            config: None,
            model_path: None,
        }
    }

    fn generate_tokens(&self, prompt: &str, config: &RuntimeConfig) -> Result<Vec<String>> {
        let model = self.model.as_ref().context("Model not loaded")?;
        let backend = self.backend.as_ref().context("Backend not initialized")?;

        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(Some(
                std::num::NonZeroU32::new(config.n_ctx)
                    .unwrap_or(std::num::NonZeroU32::new(2048).unwrap()),
            ))
            .with_n_threads(config.n_threads as i32)
            .with_n_threads_batch(config.n_threads as i32);

        let mut ctx = model
            .new_context(backend, ctx_params)
            .context("Failed to create context")?;

        let tokens_list = model
            .str_to_token(prompt, AddBos::Always)
            .context("Failed to tokenize prompt")?;

        let n_len = config.max_tokens as i32;
        let n_cxt = ctx.n_ctx() as i32;
        let n_kv_req = tokens_list.len() as i32 + n_len;

        if n_kv_req > n_cxt {
            anyhow::bail!("KV cache too small for prompt + max_tokens");
        }

        let mut batch = LlamaBatch::new(512, 1);
        let last_index = (tokens_list.len() - 1) as i32;
        for (i, token) in (0_i32..).zip(tokens_list.iter()) {
            let is_last = i == last_index;
            batch.add(*token, i, &[0], is_last)?;
        }

        ctx.decode(&mut batch).context("Failed to decode prompt")?;

        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::dist(config.seed as u32),
            LlamaSampler::temp(config.temperature),
            LlamaSampler::top_p(config.top_p, 1),
            LlamaSampler::greedy(),
        ]);

        let mut decoder = UTF_8.new_decoder();
        let mut output = Vec::new();
        let mut n_cur = batch.n_tokens();
        let mut n_decode = 0;

        while n_cur <= n_len {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);

            if model.is_eog_token(token) {
                break;
            }

            let piece = model.token_to_piece(token, &mut decoder, true, None)?;
            output.push(piece);

            batch.clear();
            batch.add(token, n_cur, &[0], true)?;

            ctx.decode(&mut batch).context("Failed to eval")?;

            n_cur += 1;
            n_decode += 1;

            if n_decode >= n_len {
                break;
            }
        }

        Ok(output)
    }
}

#[async_trait]
impl RuntimeBackend for LlamaCppBackend {
    async fn load_model(&mut self, path: &Path, config: Option<RuntimeConfig>) -> Result<()> {
        let config = config.unwrap_or_else(|| RuntimeConfig::auto_tune(path));

        let backend = LlamaBackend::init().context("Failed to initialize llama backend")?;

        let model_params = LlamaModelParams::default();

        let model = LlamaModel::load_from_file(&backend, path, &model_params)
            .with_context(|| format!("Failed to load model: {}", path.display()))?;

        self.backend = Some(backend);
        self.model = Some(Arc::new(model));
        self.config = Some(config);
        self.model_path = Some(path.to_path_buf());

        Ok(())
    }

    async fn generate(&self, prompt: &str, config: Option<RuntimeConfig>) -> Result<TokenStream> {
        let config = config.or_else(|| self.config.clone()).unwrap_or_default();
        let tokens = self.generate_tokens(prompt, &config)?;
        Ok(Box::pin(stream::iter(tokens.into_iter().map(Ok))))
    }

    async fn generate_chat(
        &self,
        messages: &[ChatMessage],
        config: Option<RuntimeConfig>,
    ) -> Result<TokenStream> {
        let model = self.model.as_ref().context("Model not loaded")?;

        let llama_messages: Vec<llama_cpp_2::model::LlamaChatMessage> = messages
            .iter()
            .map(|m| {
                llama_cpp_2::model::LlamaChatMessage::new(m.role.clone(), m.content.clone())
                    .map_err(|e| anyhow::anyhow!("Failed to create chat message: {}", e))
            })
            .collect::<Result<Vec<_>>>()?;

        let tmpl = model
            .chat_template(None)
            .map_err(|e| anyhow::anyhow!("Failed to get chat template: {}", e))?;

        let prompt = model
            .apply_chat_template(&tmpl, &llama_messages, true)
            .context("Failed to apply chat template")?;

        self.generate(&prompt, config).await
    }

    async fn unload(&mut self) -> Result<()> {
        self.model = None;
        self.backend = None;
        self.config = None;
        self.model_path = None;
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.model.is_some()
    }

    fn name(&self) -> &str {
        "llama-cpp"
    }
}
