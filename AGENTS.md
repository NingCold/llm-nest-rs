# llm-nest-rs AGENTS.md

## 项目定位

本地优先的 LLM Runtime 与 GGUF 模型管理工具。不是聊天机器人，是系统工具链（类似 ollama）。Rust 版本，从 Python 版本移植。

- Python 参考实现：`~/project/mine/llm-nest/llm-nest-python`
- 二进制名称：`llmn`
- 当前状态：基础功能已实现（模型管理、Hub 集成、CLI），推理功能需要 `runtime` feature

## 关键约束

- Rust edition 2024，stable 工具链（1.85+）
- 模型存储：`~/.llmn/models`
- 缓存目录：`~/.cache/llm-nest`
- 必须完全离线可运行，不默认依赖云服务/GPU/CUDA
- 支持 WSL、Linux、本地开发

## 开发环境

```bash
# cargo 不在默认 PATH 中，需要先 source
source ~/.cargo/env

# 构建（默认不含推理功能）
cargo build

# 构建含推理功能（需要 libclang-dev 系统依赖）
cargo build --features runtime

# 运行
cargo run

# 测试
cargo test

# Lint & 格式化
cargo clippy && cargo fmt

# 检查所有（提交前）
cargo clippy && cargo test && cargo fmt --check
```

## 依赖注意事项

- `reqwest` 必须使用 `rustls-tls` feature，避免 OpenSSL 系统依赖问题
  ```toml
  reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
  ```
- `llama-cpp-2` 是可选依赖（feature `runtime`），需要系统安装 `libclang-dev`
- 保持依赖少，优先标准库

## Feature Flags

- `default`：仅模型管理和 Hub 功能
- `runtime`：启用 llama.cpp 推理后端（需要 `libclang-dev`）

## 架构原则（从 Python 版本继承）

- 小模块、显式逻辑、扁平抽象
- 禁止全局可变状态，用 Context 对象和显式依赖传递
- CLI 层必须薄，业务逻辑放 `core` 模块
- Runtime 抽象必须 Backend-Agnostic，不让 llama.cpp 细节污染架构
- 错误不静默吞掉，要给出可操作提示
- 使用 `anyhow` 进行错误处理，`thiserror` 定义自定义错误类型

## 目录结构

```
src/
├── main.rs              # 入口点
├── cli/                 # CLI 命令（clap）
│   ├── app.rs           # CLI 定义
│   ├── context.rs       # CliContext
│   ├── commands/        # 命令实现
│   └── ui/              # 输出格式化
├── core/                # 业务逻辑
│   ├── models/          # 模型数据类型、GGUF 解析
│   ├── runtime/         # 推理后端（feature-gated）
│   ├── storage/         # 文件系统操作
│   └── registry/        # 模型注册表
├── hub/                 # HuggingFace Hub 集成
├── config/              # 配置与 i18n
└── utils/               # 工具函数
```

## 测试

- 优先集成测试和真实文件系统测试
- 减少过度 Mock
- 使用 `tempfile` 进行临时目录测试
- CI 用极小 GGUF 测试模型，避免下载大模型

## 避免

- 不必要的框架、过度工程化、过早插件系统
- 复杂魔法宏、深层继承、隐式副作用
- 默认 CUDA/GPU/联网行为
- `unwrap()` 在库代码中，使用 `?` 或 `expect` 并说明原因
