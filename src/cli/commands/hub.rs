use anyhow::Result;
use std::io::{self, Write};

use crate::cli::ui::output;
use crate::config::i18n::t;

pub async fn search(query: &str, limit: usize) -> Result<()> {
    println!("{}", t("msg.searching"));
    let results = crate::hub::search::search_gguf(query, limit).await?;
    output::print_hub_results(&results);
    Ok(())
}

pub async fn get(repo_id: &str, filename: Option<&str>) -> Result<()> {
    let files = crate::hub::search::search_repo_files(repo_id).await?;

    let filename = match filename {
        Some(f) => f.to_string(),
        None => {
            if files.is_empty() {
                anyhow::bail!("No GGUF files found in {repo_id}");
            }
            if files.len() == 1 {
                println!("Found 1 GGUF file: {}", files[0].filename);
                files[0].filename.clone()
            } else {
                println!("Available GGUF files in {repo_id}:");
                println!();
                for (i, f) in files.iter().enumerate() {
                    println!("  [{}] {} ({:.2} GB)", i + 1, f.filename, f.size_gb());
                }
                println!();
                print!("Select file number (1-{}): ", files.len());
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

                if input.is_empty() {
                    anyhow::bail!("No file selected");
                }

                let idx: usize = input
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid number: {input}"))?;

                if idx == 0 || idx > files.len() {
                    anyhow::bail!(
                        "Invalid selection: {idx}. Must be between 1 and {}",
                        files.len()
                    );
                }

                files[idx - 1].filename.clone()
            }
        }
    };

    println!();
    println!("Downloading: {filename}");
    println!();

    let path = crate::hub::download::download_model(
        repo_id,
        &filename,
        None,
        Some(Box::new(|downloaded, total| {
            if total > 0 {
                let pct = (downloaded as f64 / total as f64 * 100.0) as u32;
                let downloaded_mb = downloaded as f64 / (1024.0 * 1024.0);
                let total_mb = total as f64 / (1024.0 * 1024.0);
                print!("\rDownloading: {pct}% ({downloaded_mb:.1}/{total_mb:.1} MB)");
                io::stdout().flush().ok();
            }
        })),
    )
    .await?;

    println!();
    println!();
    println!("Downloaded to: {}", path.display());
    Ok(())
}
