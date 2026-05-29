use anyhow::Result;

use crate::core::models::enums::QuantType;
use crate::core::models::model_info::ModelInfo;
use crate::core::storage::model_store;

pub fn search_models(query: &str, directory: Option<&std::path::Path>) -> Result<Vec<ModelInfo>> {
    let models = model_store::scan_models(directory)?;
    let query_lower = query.to_lowercase();

    let matched: Vec<ModelInfo> = models
        .into_iter()
        .filter(|m| m.name.to_lowercase().contains(&query_lower))
        .collect();

    Ok(matched)
}

pub fn filter_by_quant(models: Vec<ModelInfo>, quant_type: QuantType) -> Vec<ModelInfo> {
    models.into_iter().filter(|m| m.quant_type == quant_type).collect()
}
