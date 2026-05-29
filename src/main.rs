use anyhow::Result;
use clap::Parser;

use llm_nest_rs::cli::app::{Cli, Commands, HubAction, ModelAction};
use llm_nest_rs::cli::context::CliContext;
use llm_nest_rs::cli::ui::output;
use llm_nest_rs::config;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Model { action } => {
            let mut ctx = CliContext::new(None)?;

            match action {
                ModelAction::List => llm_nest_rs::cli::commands::model::list(&ctx)?,
                ModelAction::Info { name } => llm_nest_rs::cli::commands::model::info(&ctx, &name)?,
                ModelAction::Search { query } => llm_nest_rs::cli::commands::model::search(&ctx, &query)?,
                ModelAction::Remove { name } => llm_nest_rs::cli::commands::model::remove(&mut ctx, &name)?,
            }
        }

        Commands::Hub { action } => match action {
            HubAction::Search { query, limit } => {
                llm_nest_rs::cli::commands::hub::search(&query, limit).await?
            }
            HubAction::Get { repo_id, file } => {
                llm_nest_rs::cli::commands::hub::get(&repo_id, file.as_deref()).await?
            }
        },

        Commands::Run {
            model,
            prompt,
            system,
            max_tokens,
            temperature,
            ctx_size,
        } => {
            let ctx = CliContext::new(None)?;
            llm_nest_rs::cli::commands::run::run(
                &ctx,
                &model,
                prompt.as_deref(),
                system.as_deref(),
                max_tokens,
                temperature,
                ctx_size,
            )
            .await?;
        }

        Commands::Serve {
            model,
            host,
            port,
        } => {
            let ctx = CliContext::new(None)?;
            llm_nest_rs::cli::commands::serve::serve(&ctx, &model, &host, port).await?;
        }

        Commands::Version => {
            println!("llmn {}", env!("CARGO_PKG_VERSION"));
        }

        Commands::Lang { lang } => {
            config::settings::set_lang(&lang)?;
            output::print_success(&format!("Language set to: {lang}"));
        }
    }

    Ok(())
}
