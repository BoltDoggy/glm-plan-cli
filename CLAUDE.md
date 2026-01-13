# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

GLM-Plan-CLI 是一个使用 Rust 实现的命令行工具，用于查询智谱 AI (Zhipu AI) GLM API 的计划使用情况。

**重要**: 这是一个新初始化的项目，实际的 Rust 代码结构尚未实现。

## 语言要求

**必须严格遵守以下规范**:
- 所有对话、文档、代码注释必须使用简体中文
- 与用户的所有交互均使用中文
- CLI 工具的所有输出信息必须使用中文
- 错误提示、帮助文档、用户引导均使用中文

## 开发命令

当 Rust 项目结构搭建完成后，使用以下命令：

```bash
# 构建项目
cargo build

# 运行 CLI 工具
cargo run

# 运行测试
cargo test

# 检查代码（不构建）
cargo check

# 代码检查和优化建议
cargo clippy

# 格式化代码
cargo fmt
```

## 项目架构

当前项目处于初始化阶段，预期架构将包含：

- **src/main.rs**: CLI 入口点，处理命令行参数解析
- **src/api/*.rs**: GLM API 客户端实现
- **src/config/*.rs**: 配置管理（API 密钥等）
- **src/output/*.rs**: 输出格式化（表格、JSON 等）

预期将使用常见的 Rust CLI 工具：
- `clap` 或类似库：命令行参数解析
- `reqwest` 或 `attohttpc`: HTTP 客户端
- `tokio` 或类似异步运行时（如需要）

## 开发注意事项

1. **保持中文输出**: 所有用户可见的字符串必须是中文
2. **API 密钥安全**: 不将 API 密钥硬编码在代码中，使用环境变量或配置文件
3. **错误处理**: 提供清晰的中文错误信息
4. **文档注释**: 使用 `///` 或 `//` 的所有注释必须是中文

## 状态

- ✅ 项目文档已创建
- ⏳ Rust 项目结构待实现
- ⏳ API 客户端待实现
- ⏳ CLI 界面待实现
