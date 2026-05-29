use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::enums::{ModelStatus, QuantType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelMetadata {
    pub arch: String,
    pub context_length: u64,
    pub vocab_size: u64,
    pub chat_template: String,
    pub embedding_length: u64,
    pub block_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub quant_type: QuantType,
    pub metadata: ModelMetadata,
    pub status: ModelStatus,
    pub source: String,
    pub downloaded_at: Option<DateTime<Utc>>,
}

impl ModelInfo {
    pub fn new(name: String, path: PathBuf, size_bytes: u64) -> Self {
        Self {
            name,
            path,
            size_bytes,
            quant_type: QuantType::Unknown,
            metadata: ModelMetadata::default(),
            status: ModelStatus::Available,
            source: String::new(),
            downloaded_at: None,
        }
    }

    pub fn size_gb(&self) -> f64 {
        self.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    pub fn display_name(&self) -> String {
        format!(
            "{} ({}, {:.1}GB)",
            self.name,
            self.quant_type,
            self.size_gb()
        )
    }

    pub fn stem(&self) -> &str {
        self.path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.name)
    }
}
