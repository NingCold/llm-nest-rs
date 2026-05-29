use anyhow::Result;

use crate::cli::context::CliContext;
use crate::core::models::model_info::ModelInfo;
use crate::core::storage::paths;

#[cfg(feature = "runtime")]
use crate::cli::ui::output;
#[cfg(feature = "runtime")]
use crate::config::i18n::{t, t_fmt};
#[cfg(feature = "runtime")]
use crate::core::runtime::config::RuntimeConfig;

#[cfg(feature = "runtime")]
use crate::core::runtime::backend::{ChatMessage, RuntimeBackend};
#[cfg(feature = "runtime")]
use crate::core::runtime::llama_cpp::LlamaCppBackend;

#[allow(dead_code)]
fn resolve_model(ctx: &CliContext, name: &str) -> Result<ModelInfo> {
    if let Some(model) = ctx.registry.get_model(name) {
        return Ok(model.clone());
    }

    let path = std::path::PathBuf::from(name);
    if path.exists() {
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(name)
            .to_string();
        return Ok(ModelInfo::new(stem, path, size));
    }

    let model_path = paths::model_path(name)?;
    if model_path.exists() {
        let size = std::fs::metadata(&model_path).map(|m| m.len()).unwrap_or(0);
        return Ok(ModelInfo::new(name.to_string(), model_path, size));
    }

    anyhow::bail!("Model not found: {name}")
}

#[cfg(not(feature = "runtime"))]
pub async fn run(
    _ctx: &CliContext,
    _model_name: &str,
    _prompt: Option<&str>,
    _system: Option<&str>,
    _max_tokens: u32,
    _temperature: f32,
    _ctx_size: Option<u32>,
) -> Result<()> {
    anyhow::bail!("Runtime feature not enabled. Rebuild with `--features runtime` to use model inference.")
}

#[cfg(feature = "runtime")]
pub async fn run(
    ctx: &CliContext,
    model_name: &str,
    prompt: Option<&str>,
    system: Option<&str>,
    max_tokens: u32,
    temperature: f32,
    ctx_size: Option<u32>,
) -> Result<()> {
    use std::io::{self, Write};
    use anyhow::Context;

    let model_info = resolve_model(ctx, model_name)?;

    let mut config = RuntimeConfig::auto_tune(&model_info.path);
    config.max_tokens = max_tokens;
    config.temperature = temperature;
    if let Some(size) = ctx_size {
        config.n_ctx = size;
    }
    if let Some(sys) = system {
        config.system_prompt = sys.to_string();
    }

    println!(
        "{}",
        t_fmt(
            "msg.loading_model",
            &[("name", &model_info.display_name())]
        )
    );

    let mut backend = LlamaCppBackend::new();
    backend
        .load_model(&model_info.path, Some(config.clone()))
        .await
        .context("Failed to load model")?;

    println!("{}", t("msg.model_loaded"));

    if let Some(prompt_text) = prompt {
        let mut messages = Vec::new();
        if !config.system_prompt.is_empty() {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: config.system_prompt.clone(),
            });
        }
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: prompt_text.to_string(),
        });

        let stream = backend.generate_chat(&messages, Some(config)).await?;
        use futures::StreamExt;
        tokio::pin!(stream);

        while let Some(token) = stream.next().await {
            match token {
                Ok(text) => print!("{text}"),
                Err(e) => {
                    output::print_error(&e.to_string());
                    break;
                }
            }
            io::stdout().flush()?;
        }
        println!();
    } else {
        println!("{}", t("msg.exit_hint"));
        println!();

        let mut messages = Vec::new();
        if !config.system_prompt.is_empty() {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: config.system_prompt.clone(),
            });
        }

        loop {
            print!("You: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "exit" || input == "quit" {
                println!("{}", t("msg.goodbye"));
                break;
            }

            if input.is_empty() {
                continue;
            }

            messages.push(ChatMessage {
                role: "user".to_string(),
                content: input.to_string(),
            });

            print!("Assistant: ");
            io::stdout().flush()?;

            let stream = backend.generate_chat(&messages, Some(config.clone())).await?;
            use futures::StreamExt;
            tokio::pin!(stream);

            let mut response = String::new();
            while let Some(token) = stream.next().await {
                match token {
                    Ok(text) => {
                        print!("{text}");
                        response.push_str(&text);
                    }
                    Err(e) => {
                        output::print_error(&e.to_string());
                        break;
                    }
                }
                io::stdout().flush()?;
            }
            println!();
            println!();

            messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: response,
            });
        }
    }

    backend.unload().await?;
    Ok(())
}
