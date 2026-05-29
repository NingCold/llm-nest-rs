use anyhow::Result;

use crate::cli::context::CliContext;

#[cfg(feature = "runtime")]
use anyhow::Context;
#[cfg(feature = "runtime")]
use crate::config::i18n::{t, t_fmt};
#[cfg(feature = "runtime")]
use crate::core::models::model_info::ModelInfo;
#[cfg(feature = "runtime")]
use crate::core::runtime::config::RuntimeConfig;

#[cfg(not(feature = "runtime"))]
pub async fn serve(
    _ctx: &CliContext,
    _model_name: &str,
    _host: &str,
    _port: u16,
) -> Result<()> {
    anyhow::bail!("Runtime feature not enabled. Rebuild with `--features runtime` to use the API server.")
}

#[cfg(feature = "runtime")]
pub async fn serve(
    ctx: &CliContext,
    model_name: &str,
    host: &str,
    port: u16,
) -> Result<()> {
    use std::sync::Arc;

    use actix_web::{web, App, HttpServer};
    use tokio::sync::Mutex;

    use crate::core::runtime::backend::RuntimeBackend;
    use crate::core::runtime::llama_cpp::LlamaCppBackend;

    let model_info = ctx
        .registry
        .get_model(model_name)
        .cloned()
        .or_else(|| {
            let path = std::path::PathBuf::from(model_name);
            if path.exists() {
                let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(model_name).to_string();
                Some(ModelInfo::new(stem, path, size))
            } else {
                None
            }
        })
        .context("Model not found")?;

    let config = RuntimeConfig::auto_tune(&model_info.path);

    println!(
        "{}",
        t_fmt(
            "msg.loading_model",
            &[("name", &model_info.display_name())]
        )
    );

    let mut backend = LlamaCppBackend::new();
    backend
        .load_model(&model_info.path, Some(config))
        .await
        .context("Failed to load model")?;

    println!("{}", t("msg.model_loaded"));

    let model_name_str = model_info.name.clone();
    let app_state = web::Data::new(AppState {
        backend: Arc::new(Mutex::new(backend)),
        model_name: model_name_str,
    });

    println!("Starting server on {host}:{port}");
    println!();
    println!("Endpoints:");
    println!("  GET  /health");
    println!("  GET  /v1/models");
    println!("  POST /v1/chat/completions");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(health))
            .route("/v1/models", web::get().to(list_models))
            .route("/v1/chat/completions", web::post().to(chat_completions))
    })
    .bind((host, port))
    .context("Failed to bind server")?
    .run()
    .await
    .context("Server error")?;

    Ok(())
}

#[cfg(feature = "runtime")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "runtime")]
#[derive(Debug, Deserialize)]
struct ChatCompletionRequest {
    messages: Vec<ChatMessageDto>,
    #[serde(default)]
    stream: bool,
    #[serde(default = "default_max_tokens")]
    max_tokens: u32,
    #[serde(default = "default_temperature")]
    temperature: f32,
}

#[cfg(feature = "runtime")]
fn default_max_tokens() -> u32 { 512 }

#[cfg(feature = "runtime")]
fn default_temperature() -> f32 { 0.8 }

#[cfg(feature = "runtime")]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChatMessageDto {
    role: String,
    content: String,
}

#[cfg(feature = "runtime")]
#[derive(Debug, Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[cfg(feature = "runtime")]
#[derive(Debug, Serialize)]
struct Choice {
    index: u32,
    message: ChatMessageDto,
    finish_reason: String,
}

#[cfg(feature = "runtime")]
#[derive(Debug, Serialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[cfg(feature = "runtime")]
#[derive(Debug, Serialize)]
struct ModelListResponse {
    object: String,
    data: Vec<ModelData>,
}

#[cfg(feature = "runtime")]
#[derive(Debug, Serialize)]
struct ModelData {
    id: String,
    object: String,
    created: i64,
    owned_by: String,
}

#[cfg(feature = "runtime")]
struct AppState {
    backend: std::sync::Arc<tokio::sync::Mutex<crate::core::runtime::llama_cpp::LlamaCppBackend>>,
    model_name: String,
}

#[cfg(feature = "runtime")]
async fn health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[cfg(feature = "runtime")]
async fn list_models(data: actix_web::web::Data<AppState>) -> impl actix_web::Responder {
    let response = ModelListResponse {
        object: "list".to_string(),
        data: vec![ModelData {
            id: data.model_name.clone(),
            object: "model".to_string(),
            created: chrono::Utc::now().timestamp(),
            owned_by: "local".to_string(),
        }],
    };
    actix_web::HttpResponse::Ok().json(response)
}

#[cfg(feature = "runtime")]
async fn chat_completions(
    data: actix_web::web::Data<AppState>,
    body: actix_web::web::Json<ChatCompletionRequest>,
) -> impl actix_web::Responder {
    use crate::core::runtime::backend::{ChatMessage, RuntimeBackend};
    use actix_web::HttpResponse;
    use futures::StreamExt;

    let backend = data.backend.lock().await;

    let messages: Vec<ChatMessage> = body
        .messages
        .iter()
        .map(|m| ChatMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    let config = RuntimeConfig {
        max_tokens: body.max_tokens,
        temperature: body.temperature,
        ..Default::default()
    };

    let stream = match backend.generate_chat(&messages, Some(config)).await {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": e.to_string()}));
        }
    };

    if body.stream {
        let request_id = uuid::Uuid::new_v4().to_string();
        let model_name = data.model_name.clone();

        let stream = async_stream::stream! {
            tokio::pin!(stream);

            while let Some(token) = stream.next().await {
                match token {
                    Ok(text) => {
                        let chunk = serde_json::json!({
                            "id": format!("chatcmpl-{request_id}"),
                            "object": "chat.completion.chunk",
                            "created": chrono::Utc::now().timestamp(),
                            "model": model_name,
                            "choices": [{
                                "index": 0,
                                "delta": {
                                    "content": text
                                },
                                "finish_reason": null
                            }]
                        });
                        yield Ok::<_, actix_web::Error>(actix_web::web::Bytes::from(
                            format!("data: {}\n\n", chunk)
                        ));
                    }
                    Err(e) => {
                        let error_chunk = serde_json::json!({
                            "error": {
                                "message": e.to_string(),
                                "type": "server_error"
                            }
                        });
                        yield Ok(actix_web::web::Bytes::from(
                            format!("data: {}\n\n", error_chunk)
                        ));
                        break;
                    }
                }
            }

            let final_chunk = serde_json::json!({
                "id": format!("chatcmpl-{request_id}"),
                "object": "chat.completion.chunk",
                "created": chrono::Utc::now().timestamp(),
                "model": model_name,
                "choices": [{
                    "index": 0,
                    "delta": {},
                    "finish_reason": "stop"
                }]
            });
            yield Ok(actix_web::web::Bytes::from(
                format!("data: {}\n\n", final_chunk)
            ));
            yield Ok(actix_web::web::Bytes::from("data: [DONE]\n\n"));
        };

        HttpResponse::Ok()
            .insert_header(("Content-Type", "text/event-stream"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Connection", "keep-alive"))
            .streaming(stream)
    } else {
        tokio::pin!(stream);

        let mut response_text = String::new();
        while let Some(token) = stream.next().await {
            match token {
                Ok(text) => response_text.push_str(&text),
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": e.to_string()}));
                }
            }
        }

        let response = ChatCompletionResponse {
            id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            object: "chat.completion".to_string(),
            created: chrono::Utc::now().timestamp(),
            model: data.model_name.clone(),
            choices: vec![Choice {
                index: 0,
                message: ChatMessageDto {
                    role: "assistant".to_string(),
                    content: response_text,
                },
                finish_reason: "stop".to_string(),
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        };

        HttpResponse::Ok().json(response)
    }
}
