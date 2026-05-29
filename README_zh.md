# llm-nest-rs

[English](./README.md) | [中文](./README_zh.md)

本地 LLM 运行时与 GGUF 模型管理工具。类似 `ollama`——轻量、离线优先、无需守护进程。

## 功能特性

- **模型管理** — 从 HuggingFace Hub 搜索、下载、列出和删除 GGUF 模型
- **交互式聊天** — REPL 模式，支持多轮对话和流式输出
- **API 服务器** — 兼容 OpenAI 的 `/v1/chat/completions` 端点，支持 SSE 流式响应
- **离线优先** — 无需联网即可运行，默认不依赖 GPU/CUDA

## 安装

### 从源码构建

```bash
git clone https://github.com/NingCold/llm-nest-rs.git
cd llm-nest-rs

# 基础构建（模型管理 + Hub）
cargo build --release

# 含推理功能构建（需要 libclang-dev）
cargo build --release --features runtime

# 安装
cargo install --path .
```

### 系统依赖

使用推理功能（`--features runtime`）时需要：
```bash
# Ubuntu/Debian
sudo apt install libclang-dev

# Fedora
sudo dnf install clang-devel

# macOS
brew install llvm
```

## 快速开始

### 下载模型

```bash
llmn hub get tensorblock/tinyllama-15M-GGUF -f tinyllama-15M-Q4_K_M.gguf
```

### 列出本地模型

```bash
llmn model list
```

### 运行交互式聊天

```bash
llmn run tinyllama-15M-Q4_K_M
```

```
Loading model: tinyllama-15M-Q4_K_M (Q4_K_M, 0.0GB) ...
Type 'exit' or press Ctrl+C to quit

You: Hello
Assistant: Hello! How can I help you?

You: exit
Bye!
```

### 单次推理

```bash
llmn run tinyllama-15M-Q4_K_M -p "What is 2+2?" --max-tokens 50
```

### 使用系统提示词

```bash
llmn run tinyllama-15M-Q4_K_M -s "You are a helpful assistant."
```

## API 服务器

启动兼容 OpenAI 的 HTTP 服务器：

```bash
llmn serve tinyllama-15M-Q4_K_M --port 8000
```

### 端点

| 端点 | 方法 | 描述 |
|------|------|------|
| `/health` | GET | 健康检查 |
| `/v1/models` | GET | 列出已加载的模型 |
| `/v1/chat/completions` | POST | 聊天补全（流式 + 非流式） |

### 使用 curl

```bash
# 非流式
curl http://127.0.0.1:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages": [{"role": "user", "content": "Hello"}]}'

# 流式 (SSE)
curl http://127.0.0.1:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages": [{"role": "user", "content": "Hello"}], "stream": true}'
```

### 使用 OpenAI Python SDK

```python
from openai import OpenAI

client = OpenAI(base_url="http://127.0.0.1:8000/v1", api_key="none")
resp = client.chat.completions.create(
    model="tinyllama-15M-Q4_K_M",
    messages=[{"role": "user", "content": "Hello"}],
)
print(resp.choices[0].message.content)
```

## 命令

```
llmn model list                          # 列出本地模型
llmn model search <query>                # 搜索本地 + Hub 模型
llmn model info <name>                   # 显示模型详情
llmn model remove <name>                 # 删除本地模型

llmn hub search <query>                  # 搜索 HuggingFace Hub
llmn hub get <repo_id> [-f filename]     # 从 Hub 下载模型

llmn run <model> [-p prompt] [-s system] # 交互式聊天或单次推理
llmn serve <model> [--port 8000]         # 启动 API 服务器

llmn lang zh                             # 切换到中文
llmn lang en                             # 切换到英文
llmn version                             # 显示版本
```

## 存储

模型存储在 `~/.llmn/models/`。配置文件位于 `~/.config/llm-nest/config.json`。

## 开发

```bash
# 基础构建
cargo build

# 含推理功能构建
cargo build --features runtime

# 运行测试
cargo test -- --test-threads=1

# Lint & 格式化
cargo clippy && cargo fmt

# 检查所有（提交前）
cargo clippy && cargo test && cargo fmt --check
```

## 许可证

MIT
