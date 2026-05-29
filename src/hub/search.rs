use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

use super::result::HubModelResult;

#[derive(Debug, Deserialize)]
struct HfRepo {
    id: String,
    downloads: Option<u64>,
    likes: Option<u64>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct HfSibling {
    rfilename: String,
    size: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct HfRepoInfo {
    siblings: Option<Vec<HfSibling>>,
}

pub async fn search_gguf(query: &str, limit: usize) -> Result<Vec<HubModelResult>> {
    let client = Client::new();

    let url = format!(
        "https://huggingface.co/api/models?search={query}&filter=gguf&sort=downloads&direction=-1&limit={limit}"
    );

    let repos: Vec<HfRepo> = client
        .get(&url)
        .send()
        .await
        .context("Failed to search HuggingFace Hub")?
        .json()
        .await
        .context("Failed to parse Hub search results")?;

    let mut results = Vec::new();

    for repo in repos {
        let repo_info_url = format!("https://huggingface.co/api/models/{}", repo.id);

        let info_result = client.get(&repo_info_url).send().await;

        if let Ok(response) = info_result
            && let Ok(info) = response.json::<HfRepoInfo>().await
            && let Some(siblings) = info.siblings
        {
            for sibling in siblings {
                if sibling.rfilename.ends_with(".gguf") {
                    results.push(HubModelResult {
                        repo_id: repo.id.clone(),
                        filename: sibling.rfilename,
                        size_bytes: sibling.size.unwrap_or(0),
                        downloads: repo.downloads.unwrap_or(0),
                        likes: repo.likes.unwrap_or(0),
                        tags: repo.tags.clone().unwrap_or_default(),
                    });
                }
            }
        }

        if results.len() >= limit {
            break;
        }
    }

    results.truncate(limit);
    Ok(results)
}

pub async fn search_repo_files(repo_id: &str) -> Result<Vec<HubModelResult>> {
    let client = Client::new();
    let url = format!("https://huggingface.co/api/models/{repo_id}");

    let info: HfRepoInfo = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch repo info")?
        .json()
        .await
        .context("Failed to parse repo info")?;

    let mut results = Vec::new();

    if let Some(siblings) = info.siblings {
        for sibling in siblings {
            if sibling.rfilename.ends_with(".gguf") {
                results.push(HubModelResult {
                    repo_id: repo_id.to_string(),
                    filename: sibling.rfilename,
                    size_bytes: sibling.size.unwrap_or(0),
                    downloads: 0,
                    likes: 0,
                    tags: Vec::new(),
                });
            }
        }
    }

    Ok(results)
}
