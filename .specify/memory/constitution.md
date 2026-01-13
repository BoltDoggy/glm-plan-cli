<!--
SYNC IMPACT REPORT
==================
Version change: [INITIAL] → 1.0.0
Modified principles: N/A (initial version)
Added sections:
  - Core Principles (5 principles defined)
  - Development Standards
  - Quality Assurance
  - Governance
Removed sections: N/A (initial version)
Templates requiring updates:
  ✅ .specify/templates/plan-template.md - Reviewed, Constitution Check section present
  ✅ .specify/templates/spec-template.md - Reviewed, Requirements section aligns with principles
  ✅ .specify/templates/tasks-template.md - Reviewed, task organization supports principle-driven development
  ✅ CLAUDE.md - Reviewed, project guidance aligns with constitution principles
Follow-up TODOs: None
-->

# GLM-Plan-CLI 项目宪章

## 核心原则

### 一、中文优先 (NON-NEGOTIABLE)

所有面向用户的输出、文档、代码注释和交互必须使用简体中文。

**规则**：
- CLI 工具的所有输出信息（包括帮助、错误、提示）必须是中文
- 代码注释（`///` 和 `//`）必须使用中文
- 用户文档、README、开发指南必须使用中文
- 与用户的所有交互必须使用中文

**理由**：这是一个面向中文用户群体的工具，中文用户体验是核心价值。硬编码的英文文本会严重影响可用性和用户体验。

### 二、简洁实用 (Simplicity)

CLI 工具必须保持简单、专注、易用。

**规则**：
- 单一职责：工具专注于查询 GLM API 计划使用情况，不添加无关功能
- 避免过度工程化：不为假设的未来需求添加复杂抽象
- 优先使用标准库和成熟的第三方库，避免重复造轮子
- API 设计清晰直观，减少学习成本

**理由**：CLI 工具的价值在于简单高效。过度复杂的设计会增加维护成本、降低可靠性，并使工具难以使用。

### 三、安全第一 (Security First)

API 密钥和敏感信息必须得到妥善保护。

**规则**：
- 严禁将 API 密钥硬编码在代码中
- 使用环境变量或配置文件管理敏感信息（配置文件必须加入 .gitignore）
- 配置文件必须包含适当的权限检查
- 错误信息不得泄露敏感数据（如完整的 API 密钥、Token 等）

**理由**：API 密钥泄露可能导致严重的安全问题和经济损失。安全是 CLI 工具的底线要求。

### 四、错误友好 (Error Friendliness)

所有错误都必须提供清晰、可操作的中文提示。

**规则**：
- 错误信息必须包含：发生了什么、为什么发生、如何解决
- 使用用户友好的语言，避免技术术语堆砌
- 提供明确的错误代码，便于问题追踪
- 在适当场景提供解决方案建议（如网络错误时提示检查网络连接）

**理由**：良好的错误处理能显著降低用户挫败感，减少支持成本，提升工具的专业度。

### 五、可测试性 (Testability)

代码必须易于测试，核心逻辑必须包含测试。

**规则**：
- API 客户端、配置解析、输出格式化等核心模块必须有单元测试
- 集成测试覆盖完整的用户使用场景
- 测试使用中文输入和断言，确保中文处理的正确性
- 测试必须覆盖正常流程和错误场景

**理由**：测试是保证代码质量的基础。CLI 工具的可靠性直接取决于测试覆盖的完整性。

## 开发标准

### 技术栈约束

- **语言**：Rust（使用稳定版本）
- **核心依赖**：
  - `clap`：命令行参数解析
  - `reqwest` 或 `attohttpc`：HTTP 客户端
  - `tokio`：异步运行时（如需要）
  - `serde`：序列化/反序列化
  - `anyhow` 或 `thiserror`：错误处理
- **禁止引入**：与核心功能无关的大型依赖

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量，修复所有警告
- 函数和模块保持小而专注（单个函数不超过 50 行）
- 使用清晰的命名（中文注释说明复杂逻辑）

### 文档要求

- 所有公开 API 必须有文档注释（`///`，中文）
- README 包含：安装说明、使用示例、配置方法、常见问题
- 代码注释解释"为什么"而非"是什么"

## 质量保证

### 提交前检查清单

- [ ] `cargo check` 通过，无编译错误
- [ ] `cargo clippy` 通过，无警告
- [ ] `cargo fmt` 已执行，代码格式统一
- [ ] `cargo test` 通过，所有测试通过
- [ ] 新增功能包含中文测试用例
- [ ] 错误信息使用中文并包含解决方案
- [ ] 无硬编码的 API 密钥或敏感信息

### 发布标准

- 所有核心功能有完整测试覆盖
- README 文档完整且最新
- 错误处理覆盖所有已知的失败场景
- 代码通过 clippy 检查，无警告

## 治理

### 宪章地位

本宪章是项目开发的最高指导原则，优先级高于其他开发实践和习惯。

### 修订流程

1. **提案**：修订建议必须说明理由和影响范围
2. **审查**：评估对现有代码和开发流程的影响
3. **批准**：重大修订需要明确批准
4. **迁移**：提供清晰的迁移指南和实施计划
5. **版本**：按照语义化版本规则更新宪章版本

### 版本规则

- **MAJOR**：移除或重新定义核心原则（向后不兼容）
- **MINOR**：新增原则或显著扩展现有原则
- **PATCH**：措辞优化、澄清、非实质性修改

### 合规审查

- 所有代码审查必须验证是否符合宪章原则
- 发现违规必须立即修复或记录例外理由
- 例外理由必须在代码注释中明确说明

### 运行时指导

开发过程中参考 `CLAUDE.md` 获取具体的开发指导和上下文信息。

---

**Version**: 1.0.0 | **Ratified**: 2026-01-13 | **Last Amended**: 2026-01-13
