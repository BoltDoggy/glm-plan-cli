# 实现计划：GLM API 计划查询命令

**分支**: `001-glm-cli-command` | **日期**: 2026-01-13 | **规格**: [spec.md](./spec.md)
**输入**: 来自 `/specs/001-glm-cli-command/spec.md` 的功能规格说明

**注意**: 本模板由 `/speckit.plan` 命令填充。有关执行工作流，请参阅 `.specify/templates/commands/plan.md`。

## 摘要

本功能实现一个 Rust 命令行工具，用于查询智谱 AI GLM API 的计划使用情况。核心功能包括：通过命令行查询 API 计划信息（总额度、已使用量、剩余量、有效期），支持多种输出格式（表格、JSON），环境变量和配置文件两种认证方式，以及完整的中文错误提示和使用帮助。

技术方法：使用 Rust 构建单个可执行文件，利用 `clap` 进行命令行参数解析，`reqwest` 处理 HTTP 请求，`tokio` 提供异步运行时支持。配置管理支持环境变量优先、配置文件备选的策略。输出格式化模块支持表格和 JSON 两种格式。

## 技术上下文

**语言/版本**: Rust 1.75+ (稳定版本)
**主要依赖**:
- `clap` { version = "4.5", features = ["derive"] }: 命令行参数解析
- `reqwest` { version = "0.12", features = ["json"] }: HTTP 客户端
- `tokio` { version = "1.0", features = ["full"] }: 异步运行时
- `serde` { version = "1.0", features = ["derive"] }: 序列化/反序列化
- `serde_json` = "1.0": JSON 支持
- `thiserror` = "1.0": 错误处理
- `anyhow` = "1.0": 错误上下文
- `comfy-table` = "7.0": 表格输出（需要研究最佳选择）
- `dirs` = "5.0": 获取用户主目录

**存储**: N/A（无持久化存储）
**测试**: `cargo test`（内置）+ 单元测试 + 集成测试
**目标平台**:
- 主要: Linux, macOS, Windows（跨平台 CLI 工具）
- Rust 支持: x86_64-unknown-linux-gnu, x86_64-apple-darwin, x86_64-pc-windows-msvc

**项目类型**: single（单个 CLI 工具）

**性能目标**:
- API 查询响应时间: < 5 秒（端到端）
- 错误信息显示: < 3 秒
- 内存使用: < 50MB
- 工具启动时间: < 100ms

**约束**:
- 必须完全中文化（用户可见文本）
- 30 秒 API 请求超时
- API 密钥不能硬编码
- 必须在低配置系统上运行（< 512MB RAM）

**规模/范围**:
- 代码量: 约 2000-3000 行 Rust 代码
- 模块数: 4-6 个核心模块
- 测试用例: 20-30 个测试
- 用户故事: 3 个（P1: 查询, P2: 多格式, P3: 帮助文档）

**NEEDS CLARIFICATION**:
1. **GLM API 端点和认证**: 具体的 API 端点 URL、请求格式、响应格式（假设 1: RESTful 端点 + API Key 认证）
2. **表格渲染库**: 选择 `comfy-table` 还是其他表格库（需要研究最佳实践）
3. **配置文件格式**: YAML vs TOML vs JSON（假设 6: YAML，但需要验证）

## 宪章检查

*门槛：必须在 Phase 0 研究前通过。Phase 1 设计后重新检查。*

- [x] **中文优先**: 所有用户可见文本使用中文（CLI 输出、错误提示、帮助文档）
  - **计划**: 所有字符串字面量使用中文，错误消息模块化，使用格式化宏支持参数
- [x] **简洁实用**: 功能范围清晰，无过度设计，依赖最小化
  - **计划**: 仅使用必需的依赖（clap, reqwest, tokio, serde），单一可执行文件，无插件系统
- [x] **安全第一**: API 密钥管理方案明确（环境变量或配置文件），无硬编码敏感信息
  - **计划**: 环境变量优先，配置文件备选，配置文件路径加入 .gitignore，密钥掩码处理
- [x] **错误友好**: 错误处理策略清晰，包含可操作的中文错误信息
  - **计划**: 使用 `thiserror` 定义错误类型，每个错误包含：描述、原因、解决建议，错误代码映射
- [x] **可测试性**: 核心模块测试策略明确，测试覆盖关键场景
  - **计划**: 单元测试覆盖配置解析、API 客户端、输出格式化；集成测试覆盖完整用户场景；模拟 API 响应

**宪章合规状态**: ✅ **通过** - 所有必要检查点已规划，无违规项

## 项目结构

### 文档（本功能）

```text
specs/001-glm-cli-command/
├── plan.md              # 本文件（/speckit.plan 命令输出）
├── research.md          # Phase 0 输出（/speckit.plan 命令）
├── data-model.md        # Phase 1 输出（/speckit.plan 命令）
├── quickstart.md        # Phase 1 输出（/speckit.plan 命令）
├── contracts/           # Phase 1 输出（/speckit.plan 命令）
└── tasks.md             # Phase 2 输出（/speckit.tasks 命令 - 非 /speckit.plan 创建）
```

### 源代码（仓库根目录）

```text
# 选项 1: 单个项目（DEFAULT - 已选择）
src/
├── main.rs              # CLI 入口点，命令行参数解析
├── config.rs            # 配置管理（环境变量、配置文件）
├── api/
│   ├── mod.rs           # API 模块入口
│   ├── client.rs        # GLM API 客户端实现
│   └── types.rs         # API 请求/响应数据类型
├── output/
│   ├── mod.rs           # 输出模块入口
│   ├── table.rs         # 表格格式输出
│   └── json.rs          # JSON 格式输出
└── error.rs             # 错误类型定义（使用 thiserror）

tests/
├── contract/            # 契约测试（API 响应格式）
├── integration/         # 集成测试（完整用户场景）
└── unit/                # 单元测试（各模块功能）

glm-config.example.yaml  # 示例配置文件
```

**结构决策**: 选择单一项目结构，因为这是一个独立的 CLI 工具，不需要前后端分离。源代码位于 `src/` 目录，按照功能模块组织（配置、API 客户端、输出格式化、错误处理）。测试分离到 `tests/` 目录，按测试类型分类（契约、集成、单元）。

## 复杂性跟踪

> **仅当宪章检查有必须证明的违规时填写**

**无需填写** - 宪章检查全部通过，无违规项。
