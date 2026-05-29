use std::collections::HashMap;
use std::sync::LazyLock;

use super::settings;

static LANG_ZH: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("app.name", "llm-nest");
    m.insert("app.description", "本地 LLM 运行时与 GGUF 模型管理工具");
    m.insert("cmd.model", "模型管理");
    m.insert("cmd.model.list", "列出本地模型");
    m.insert("cmd.model.info", "显示模型详情");
    m.insert("cmd.model.search", "搜索模型");
    m.insert("cmd.model.remove", "删除模型");
    m.insert("cmd.hub", "HuggingFace Hub");
    m.insert("cmd.hub.search", "搜索 Hub 模型");
    m.insert("cmd.hub.get", "从 Hub 下载模型");
    m.insert("cmd.run", "运行模型");
    m.insert("cmd.serve", "启动 API 服务器");
    m.insert("cmd.version", "显示版本");
    m.insert("cmd.lang", "设置语言");
    m.insert("msg.loading_model", "正在加载模型: {name}");
    m.insert("msg.model_loaded", "模型加载完成");
    m.insert("msg.no_models", "未找到本地模型");
    m.insert("msg.model_deleted", "模型已删除: {name}");
    m.insert("msg.model_not_found", "未找到模型: {name}");
    m.insert("msg.searching", "正在搜索...");
    m.insert("msg.downloading", "正在下载...");
    m.insert("msg.error", "错误: {msg}");
    m.insert("msg.exit_hint", "输入 'exit' 或按 Ctrl+C 退出");
    m.insert("msg.goodbye", "再见！");
    m.insert("table.name", "名称");
    m.insert("table.size", "大小");
    m.insert("table.quant", "量化");
    m.insert("table.status", "状态");
    m.insert("table.downloads", "下载量");
    m.insert("table.repo", "仓库");
    m.insert("table.file", "文件");
    m
});

static LANG_EN: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("app.name", "llm-nest");
    m.insert("app.description", "Local LLM runtime and GGUF model management tool");
    m.insert("cmd.model", "Model management");
    m.insert("cmd.model.list", "List local models");
    m.insert("cmd.model.info", "Show model details");
    m.insert("cmd.model.search", "Search models");
    m.insert("cmd.model.remove", "Delete a model");
    m.insert("cmd.hub", "HuggingFace Hub");
    m.insert("cmd.hub.search", "Search Hub models");
    m.insert("cmd.hub.get", "Download model from Hub");
    m.insert("cmd.run", "Run model");
    m.insert("cmd.serve", "Start API server");
    m.insert("cmd.version", "Show version");
    m.insert("cmd.lang", "Set language");
    m.insert("msg.loading_model", "Loading model: {name}");
    m.insert("msg.model_loaded", "Model loaded");
    m.insert("msg.no_models", "No local models found");
    m.insert("msg.model_deleted", "Model deleted: {name}");
    m.insert("msg.model_not_found", "Model not found: {name}");
    m.insert("msg.searching", "Searching...");
    m.insert("msg.downloading", "Downloading...");
    m.insert("msg.error", "Error: {msg}");
    m.insert("msg.exit_hint", "Type 'exit' or press Ctrl+C to quit");
    m.insert("msg.goodbye", "Bye!");
    m.insert("table.name", "Name");
    m.insert("table.size", "Size");
    m.insert("table.quant", "Quant");
    m.insert("table.status", "Status");
    m.insert("table.downloads", "Downloads");
    m.insert("table.repo", "Repo");
    m.insert("table.file", "File");
    m
});

pub fn t(key: &str) -> String {
    let lang = settings::get_lang();
    let map = match lang.as_str() {
        "zh" => &*LANG_ZH,
        _ => &*LANG_EN,
    };
    map.get(key).unwrap_or(&key).to_string()
}

pub fn t_fmt(key: &str, args: &[(&str, &str)]) -> String {
    let mut result = t(key);
    for (k, v) in args {
        result = result.replace(&format!("{{{k}}}"), v);
    }
    result
}
