# llm-nest-rs

[English](./README.md) | [中文](./README_zh.md)

Local LLM runtime and GGUF model management tool. Think of it as `ollama` for your terminal — lightweight, offline-first, no daemon required.

## Features

- **Model management** — search, download, list, and delete GGUF models from HuggingFace Hub
- **Interactive chat** — REPL mode with multi-turn conversation and streaming output
- **API server** — OpenAI-compatible `/v1/chat/completions` endpoint with SSE streaming
- **Offline-first** — works without internet, no GPU/CUDA required by default

## Install

### From source

```bash
git clone https://github.com/NingCold/llm-nest-rs.git
cd llm-nest-rs

# Basic build (model management + Hub)
cargo build --release

# With inference support (requires libclang-dev)
cargo build --release --features runtime

# Install
cargo install --path .
```

### System dependencies

For inference features (`--features runtime`):
```bash
# Ubuntu/Debian
sudo apt install libclang-dev

# Fedora
sudo dnf install clang-devel

# macOS
brew install llvm
```

## Quick Start

### Download a model

```bash
llmn hub get tensorblock/tinyllama-15M-GGUF -f tinyllama-15M-Q4_K_M.gguf
```

### List local models

```bash
llmn model list
```

### Run interactive chat

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

### Single-shot inference

```bash
llmn run tinyllama-15M-Q4_K_M -p "What is 2+2?" --max-tokens 50
```

### With system prompt

```bash
llmn run tinyllama-15M-Q4_K_M -s "You are a helpful assistant."
```

## API Server

Start an OpenAI-compatible HTTP server:

```bash
llmn serve tinyllama-15M-Q4_K_M --port 8000
```

### Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/v1/models` | GET | List loaded models |
| `/v1/chat/completions` | POST | Chat completion (stream + non-stream) |

### Usage with curl

```bash
# Non-streaming
curl http://127.0.0.1:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages": [{"role": "user", "content": "Hello"}]}'

# Streaming (SSE)
curl http://127.0.0.1:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages": [{"role": "user", "content": "Hello"}], "stream": true}'
```

### Usage with OpenAI Python SDK

```python
from openai import OpenAI

client = OpenAI(base_url="http://127.0.0.1:8000/v1", api_key="none")
resp = client.chat.completions.create(
    model="tinyllama-15M-Q4_K_M",
    messages=[{"role": "user", "content": "Hello"}],
)
print(resp.choices[0].message.content)
```

## Commands

```
llmn model list                          # List local models
llmn model search <query>                # Search local + Hub models
llmn model info <name>                   # Show model details
llmn model remove <name>                 # Delete a local model

llmn hub search <query>                  # Search HuggingFace Hub
llmn hub get <repo_id> [-f filename]     # Download model from Hub

llmn run <model> [-p prompt] [-s system] # Interactive chat or single-shot
llmn serve <model> [--port 8000]         # Start API server

llmn lang zh                             # Switch to Chinese
llmn lang en                             # Switch to English
llmn version                             # Show version
```

## Storage

Models are stored in `~/.llmn/models/`. Config is at `~/.config/llm-nest/config.json`.

## Development

```bash
# Basic build
cargo build

# With inference support
cargo build --features runtime

# Run tests
cargo test -- --test-threads=1

# Lint & format
cargo clippy && cargo fmt

# Check all (before commit)
cargo clippy && cargo test && cargo fmt --check
```

## License

MIT
