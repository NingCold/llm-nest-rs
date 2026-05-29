#[cfg(feature = "runtime")]
mod runtime_tests {
    use llm_nest_rs::api::{ChatMessage, GenerateOptions, LlmRuntime};
    use std::path::PathBuf;

    fn tinyllama_path() -> PathBuf {
        dirs::home_dir()
            .unwrap()
            .join(".llmn/models/tinyllama-15M-Q3_K_M.gguf")
    }

    #[tokio::test]
    async fn test_runtime_creation() {
        let runtime = LlmRuntime::new();
        assert!(!runtime.is_loaded());
    }

    #[tokio::test]
    async fn test_runtime_with_options() {
        let runtime = LlmRuntime::with_options(256, 0.7);
        assert!(!runtime.is_loaded());
    }

    #[tokio::test]
    async fn test_runtime_load_model() {
        let model_path = tinyllama_path();
        if !model_path.exists() {
            eprintln!("Skipping test: model not found at {:?}", model_path);
            return;
        }

        let mut runtime = LlmRuntime::new();
        let result = runtime.load_model(&model_path).await;
        assert!(result.is_ok(), "Failed to load model: {:?}", result.err());
        assert!(runtime.is_loaded());
    }

    #[tokio::test]
    async fn test_runtime_generate() {
        let model_path = tinyllama_path();
        if !model_path.exists() {
            eprintln!("Skipping test: model not found at {:?}", model_path);
            return;
        }

        let mut runtime = LlmRuntime::with_options(50, 0.8);
        runtime.load_model(&model_path).await.unwrap();

        let result = runtime.generate("Hello").await;
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());

        let text = result.unwrap();
        assert!(!text.is_empty());
        println!("Generated: {}", text);
    }

    #[tokio::test]
    async fn test_runtime_chat() {
        let model_path = tinyllama_path();
        if !model_path.exists() {
            eprintln!("Skipping test: model not found at {:?}", model_path);
            return;
        }

        let mut runtime = LlmRuntime::with_options(50, 0.8);
        runtime.load_model(&model_path).await.unwrap();

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
        }];

        let result = runtime.chat(&messages).await;
        assert!(result.is_ok(), "Failed to chat: {:?}", result.err());

        let response = result.unwrap();
        assert!(!response.is_empty());
        println!("Chat response: {}", response);
    }

    #[tokio::test]
    async fn test_runtime_load_with_options() {
        let model_path = tinyllama_path();
        if !model_path.exists() {
            eprintln!("Skipping test: model not found at {:?}", model_path);
            return;
        }

        let options = GenerateOptions {
            max_tokens: 30,
            temperature: 0.5,
            system_prompt: Some("You are a helpful assistant.".to_string()),
            ..Default::default()
        };

        let mut runtime = LlmRuntime::new();
        let result = runtime.load_model_with_config(&model_path, options).await;
        assert!(
            result.is_ok(),
            "Failed to load model with options: {:?}",
            result.err()
        );
        assert!(runtime.is_loaded());
    }

    #[tokio::test]
    async fn test_runtime_unload() {
        let model_path = tinyllama_path();
        if !model_path.exists() {
            eprintln!("Skipping test: model not found at {:?}", model_path);
            return;
        }

        let mut runtime = LlmRuntime::new();
        runtime.load_model(&model_path).await.unwrap();
        assert!(runtime.is_loaded());

        runtime.unload().await.unwrap();
        assert!(!runtime.is_loaded());
    }

    #[tokio::test]
    async fn test_runtime_generate_not_loaded() {
        let runtime = LlmRuntime::new();
        let result = runtime.generate("Hello").await;
        assert!(result.is_err(), "Should fail when model not loaded");
    }
}
