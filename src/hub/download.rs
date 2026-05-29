use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use futures::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::core::storage::paths;

pub async fn download_model(
    repo_id: &str,
    filename: &str,
    dest_dir: Option<&Path>,
    on_progress: Option<Box<dyn Fn(u64, u64) + Send>>,
) -> Result<PathBuf> {
    let dest_dir = match dest_dir {
        Some(d) => d.to_path_buf(),
        None => paths::models_dir()?,
    };

    let url = format!(
        "https://huggingface.co/{repo_id}/resolve/main/{filename}"
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to start download")?;

    if !response.status().is_success() {
        anyhow::bail!("Download failed: HTTP {}", response.status());
    }

    let total_size = response.content_length().unwrap_or(0);

    let dest_path = dest_dir.join(filename);
    let mut file = File::create(&dest_path)
        .await
        .with_context(|| format!("Failed to create file: {}", dest_path.display()))?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("Failed to read download chunk")?;
        file.write_all(&chunk)
            .await
            .context("Failed to write to file")?;
        downloaded += chunk.len() as u64;

        if let Some(ref callback) = on_progress {
            callback(downloaded, total_size);
        }
    }

    file.flush().await.context("Failed to flush file")?;

    Ok(dest_path)
}
