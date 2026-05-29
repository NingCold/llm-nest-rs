use std::path::PathBuf;

use llm_nest_rs::core::models::enums::QuantType;
use llm_nest_rs::core::models::model_info::ModelInfo;
use llm_nest_rs::core::registry::manager::ModelRegistry;
use llm_nest_rs::core::storage::paths;

#[test]
fn test_model_info_creation() {
    let path = PathBuf::from("/tmp/test.gguf");
    let info = ModelInfo::new("test".to_string(), path.clone(), 1024 * 1024 * 1024);

    assert_eq!(info.name, "test");
    assert_eq!(info.path, path);
    assert_eq!(info.size_bytes, 1024 * 1024 * 1024);
    assert_eq!(info.quant_type, QuantType::Unknown);
    assert!((info.size_gb() - 1.0).abs() < 0.01);
}

#[test]
fn test_model_info_display_name() {
    let path = PathBuf::from("/tmp/test.gguf");
    let mut info = ModelInfo::new("test".to_string(), path, 1024 * 1024 * 1024);
    info.quant_type = QuantType::Q4_K_M;

    let display = info.display_name();
    assert!(display.contains("test"));
    assert!(display.contains("Q4_K_M"));
    assert!(display.contains("1.0GB"));
}

#[test]
fn test_quant_type_display() {
    assert_eq!(format!("{}", QuantType::F32), "F32");
    assert_eq!(format!("{}", QuantType::Q4_K_M), "Q4_K_M");
    assert_eq!(format!("{}", QuantType::Unknown), "Unknown");
}

#[test]
fn test_quant_type_from_gguf_type() {
    assert_eq!(QuantType::from_gguf_type(0), QuantType::F32);
    assert_eq!(QuantType::from_gguf_type(2), QuantType::Q4_0);
    assert_eq!(QuantType::from_gguf_type(14), QuantType::Q4_K_M);
    assert_eq!(QuantType::from_gguf_type(999), QuantType::Unknown);
}

#[test]
fn test_model_status_display() {
    use llm_nest_rs::core::models::enums::ModelStatus;

    assert_eq!(format!("{}", ModelStatus::Available), "available");
    assert_eq!(format!("{}", ModelStatus::Loaded), "loaded");
}

#[test]
fn test_runtime_config_default() {
    use llm_nest_rs::core::runtime::config::RuntimeConfig;

    let config = RuntimeConfig::default();
    assert_eq!(config.n_ctx, 4096);
    assert_eq!(config.max_tokens, 512);
    assert!((config.temperature - 0.8).abs() < 0.01);
}

#[test]
fn test_paths_models_dir() {
    let dir = paths::models_dir().unwrap();
    assert!(dir.exists());
    assert!(dir.to_string_lossy().contains(".llmn"));
}

#[test]
fn test_paths_cache_dir() {
    let dir = paths::cache_dir().unwrap();
    assert!(dir.exists());
}

#[test]
fn test_paths_config_dir() {
    let dir = paths::config_dir().unwrap();
    assert!(dir.exists());
}

#[test]
fn test_registry_creation() {
    let registry = ModelRegistry::new(None);
    assert!(registry.list_models().is_empty());
}

#[test]
fn test_registry_scan_empty() {
    let mut registry = ModelRegistry::new(None);
    registry.scan().unwrap();
    // Should not panic even if no models exist
}

#[test]
fn test_config_load_save() {
    use llm_nest_rs::config::settings;

    // Save current config
    let original = settings::load_config().unwrap();

    // Test setting language
    settings::set_lang("zh").unwrap();

    // Verify immediately
    let current_lang = settings::get_lang();
    assert_eq!(current_lang, "zh", "Expected lang to be 'zh', got '{}'", current_lang);

    // Restore original
    settings::set_lang(&original.lang).unwrap();
}

#[test]
fn test_i18n() {
    use llm_nest_rs::config::{i18n, settings};

    // Save current lang
    let original = settings::get_lang();

    // Test English
    settings::set_lang("en").unwrap();
    let en_text = i18n::t("msg.goodbye");
    assert!(!en_text.is_empty());

    // Test Chinese
    settings::set_lang("zh").unwrap();
    let zh_text = i18n::t("msg.goodbye");
    assert!(!zh_text.is_empty());

    // Restore
    settings::set_lang(&original).unwrap();
}

#[test]
fn test_hub_model_result() {
    use llm_nest_rs::hub::result::HubModelResult;

    let result = HubModelResult {
        repo_id: "TheBloke/test".to_string(),
        filename: "model.Q4_K_M.gguf".to_string(),
        size_bytes: 1024 * 1024 * 1024,
        downloads: 1000,
        likes: 100,
        tags: vec!["gguf".to_string()],
    };

    assert_eq!(result.display_name(), "TheBloke/test/model.Q4_K_M.gguf");
    assert!((result.size_gb() - 1.0).abs() < 0.01);
}

#[test]
fn test_api_llm_nest_creation() {
    use llm_nest_rs::api::LlmNest;

    let app = LlmNest::new();
    assert!(app.is_ok(), "Failed to create LlmNest: {:?}", app.err());
}

#[test]
fn test_api_list_models() {
    use llm_nest_rs::api::LlmNest;

    let app = LlmNest::new().unwrap();
    let models = app.list_models();
    // Should return empty vec or actual models, not panic
    println!("Found {} models", models.len());
}

#[test]
fn test_api_get_model_not_found() {
    use llm_nest_rs::api::LlmNest;

    let app = LlmNest::new().unwrap();
    let result = app.get_model("nonexistent-model-12345");
    assert!(result.is_none());
}

#[test]
fn test_api_model_summary_fields() {
    use llm_nest_rs::api::LlmNest;

    let app = LlmNest::new().unwrap();
    let models = app.list_models();

    if let Some(model) = models.first() {
        assert!(!model.name.is_empty());
        assert!(model.size_gb >= 0.0);
        assert!(!model.quant_type.is_empty());
        assert!(!model.status.is_empty());
    }
}
