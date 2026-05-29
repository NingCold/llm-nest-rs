use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "llmn", version, about = "Local LLM runtime and GGUF model management tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Model management
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },

    /// HuggingFace Hub integration
    Hub {
        #[command(subcommand)]
        action: HubAction,
    },

    /// Run a model (interactive chat or single-shot)
    Run {
        /// Model name or path
        model: String,

        /// Single-shot prompt (non-interactive mode)
        #[arg(short, long)]
        prompt: Option<String>,

        /// System prompt
        #[arg(short, long)]
        system: Option<String>,

        /// Max tokens to generate
        #[arg(long, default_value_t = 512)]
        max_tokens: u32,

        /// Temperature
        #[arg(long, default_value_t = 0.8)]
        temperature: f32,

        /// Context size
        #[arg(short = 'c', long)]
        ctx_size: Option<u32>,
    },

    /// Start OpenAI-compatible API server
    Serve {
        /// Model name or path
        model: String,

        /// Port to listen on
        #[arg(long, default_value_t = 8000)]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },

    /// Show version
    Version,

    /// Set language (zh/en)
    Lang {
        /// Language code
        lang: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ModelAction {
    /// List local models
    List,

    /// Show model details
    Info {
        /// Model name
        name: String,
    },

    /// Search local models
    Search {
        /// Search query
        query: String,
    },

    /// Delete a local model
    Remove {
        /// Model name
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum HubAction {
    /// Search HuggingFace Hub for GGUF models
    Search {
        /// Search query
        query: String,

        /// Max results
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },

    /// Download a model from HuggingFace Hub
    Get {
        /// Repository ID (e.g., TheBloke/Llama-2-7B-GGUF)
        repo_id: String,

        /// Filename to download
        #[arg(short, long)]
        file: Option<String>,
    },
}
