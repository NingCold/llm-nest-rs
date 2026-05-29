use std::fs;
use std::path::Path;

use anyhow::Result;

use super::paths;
use crate::core::models::enums::QuantType;
use crate::core::models::gguf::GgufMetadata;
use crate::core::models::model_info::{ModelInfo, ModelMetadata};

const GGUF_EXTENSIONS: &[&str] = &[".gguf"];

pub fn scan_models(directory: Option<&Path>) -> Result<Vec<ModelInfo>> {
    let dir = match directory {
        Some(d) => d.to_path_buf(),
        None => paths::models_dir()?,
    };

    let mut models = Vec::new();

    if !dir.exists() {
        return Ok(models);
    }

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let is_gguf = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| GGUF_EXTENSIONS.iter().any(|ge| ge.ends_with(ext)))
            .unwrap_or(false);

        if !is_gguf {
            continue;
        }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let size_bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        let mut info = ModelInfo::new(name, path.clone(), size_bytes);

        match GgufMetadata::parse(&path) {
            Ok(gguf) => {
                info.quant_type = gguf.quant_type;
                info.metadata = ModelMetadata {
                    arch: gguf.arch,
                    context_length: gguf.context_length,
                    vocab_size: gguf.vocab_size,
                    chat_template: gguf.chat_template,
                    embedding_length: gguf.embedding_length,
                    block_count: gguf.block_count,
                };
            }
            Err(_) => {
                info.quant_type = QuantType::Unknown;
            }
        }

        models.push(info);
    }

    models.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(models)
}

pub fn find_model(name: &str, directory: Option<&Path>) -> Result<Option<ModelInfo>> {
    let models = scan_models(directory)?;
    Ok(models
        .into_iter()
        .find(|m| m.name == name || m.stem() == name))
}

pub fn delete_model(path: &Path) -> anyhow::Result<()> {
    super::file_ops::delete_model_file(path)
}
