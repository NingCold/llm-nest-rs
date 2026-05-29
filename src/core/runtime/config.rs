use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub n_ctx: u32,
    pub n_threads: u32,
    pub n_gpu_layers: u32,
    pub n_batch: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub max_tokens: u32,
    pub repeat_penalty: f32,
    pub seed: i32,
    pub system_prompt: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        let n_threads = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(4);

        Self {
            n_ctx: 4096,
            n_threads,
            n_gpu_layers: 0,
            n_batch: 512,
            temperature: 0.8,
            top_p: 0.95,
            top_k: 40,
            max_tokens: 512,
            repeat_penalty: 1.1,
            seed: -1,
            system_prompt: String::new(),
        }
    }
}

impl RuntimeConfig {
    pub fn auto_tune(model_path: &std::path::Path) -> Self {
        let mut config = Self::default();

        // Estimate RAM based on file size
        let size_bytes = std::fs::metadata(model_path)
            .map(|m| m.len())
            .unwrap_or(0);
        let est_ram_gb = size_bytes as f64 / (0.6 * 1024.0 * 1024.0 * 1024.0);

        config.n_ctx = if est_ram_gb > 16.0 {
            8192
        } else if est_ram_gb > 8.0 {
            4096
        } else {
            2048
        };

        config
    }
}
