use anyhow::Result;

use crate::cli::context::CliContext;
use crate::cli::ui::output;
use crate::config::i18n::t_fmt;

pub fn list(ctx: &CliContext) -> Result<()> {
    output::print_models(ctx.registry.list_models());
    Ok(())
}

pub fn info(ctx: &CliContext, name: &str) -> Result<()> {
    match ctx.registry.get_model(name) {
        Some(model) => output::print_model_info(model),
        None => {
            output::print_error(&t_fmt("msg.model_not_found", &[("name", name)]));
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn search(_ctx: &CliContext, query: &str) -> Result<()> {
    let results = crate::core::registry::search::search_models(query, None)?;
    output::print_search_results(&results);
    Ok(())
}

pub fn remove(ctx: &mut CliContext, name: &str) -> Result<()> {
    match ctx.registry.delete_model(name)? {
        true => {
            output::print_success(&t_fmt("msg.model_deleted", &[("name", name)]));
        }
        false => {
            output::print_error(&t_fmt("msg.model_not_found", &[("name", name)]));
            std::process::exit(1);
        }
    }
    Ok(())
}
