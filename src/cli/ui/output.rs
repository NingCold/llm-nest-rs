use crate::config::i18n::t;

pub fn print_models(models: &[crate::core::models::model_info::ModelInfo]) {
    if models.is_empty() {
        println!("{}", t("msg.no_models"));
        return;
    }

    println!(
        "{:<40} {:>10} {:>10} {:>12}",
        t("table.name"),
        t("table.size"),
        t("table.quant"),
        t("table.status"),
    );
    println!("{}", "-".repeat(76));

    for model in models {
        println!(
            "{:<40} {:>8.2}GB {:>10} {:>12}",
            model.name,
            model.size_gb(),
            model.quant_type,
            model.status,
        );
    }
}

pub fn print_model_info(model: &crate::core::models::model_info::ModelInfo) {
    println!("Name:        {}", model.name);
    println!("Path:        {}", model.path.display());
    println!("Size:        {:.2} GB", model.size_gb());
    println!("Quant:       {}", model.quant_type);
    println!("Status:      {}", model.status);
    println!("Source:      {}", model.source);
    if !model.metadata.arch.is_empty() {
        println!("Architecture: {}", model.metadata.arch);
    }
    if model.metadata.context_length > 0 {
        println!("Context:     {}", model.metadata.context_length);
    }
    if model.metadata.vocab_size > 0 {
        println!("Vocab:       {}", model.metadata.vocab_size);
    }
    if model.metadata.block_count > 0 {
        println!("Blocks:      {}", model.metadata.block_count);
    }
}

pub fn print_hub_results(results: &[crate::hub::result::HubModelResult]) {
    if results.is_empty() {
        println!("No results found");
        return;
    }

    println!(
        "{:<45} {:>10} {:>12}",
        t("table.file"),
        t("table.size"),
        t("table.downloads"),
    );
    println!("{}", "-".repeat(70));

    for result in results {
        println!(
            "{:<45} {:>8.2}GB {:>12}",
            result.filename,
            result.size_gb(),
            result.downloads,
        );
    }
}

pub fn print_search_results(models: &[crate::core::models::model_info::ModelInfo]) {
    print_models(models);
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", t("msg.error").replace("{msg}", ""), msg);
}

pub fn print_info(msg: &str) {
    println!("{msg}");
}

pub fn print_success(msg: &str) {
    println!("{msg}");
}
