---

description: "任务列表：GLM API 计划查询命令实现"
---

# 任务列表：GLM API 计划查询命令

**输入**: 来自 `/specs/001-glm-cli-command/` 的设计文档
**前提条件**: plan.md, spec.md, research.md, data-model.md, contracts/api.md

**测试**: 本功能规格未明确要求测试，因此任务列表专注于实现。如需测试，可后续添加。

**组织方式**: 任务按用户故事分组，以实现每个故事的独立实现和测试。

## 格式说明: `[ID] [P?] [Story?] 描述`

- **[P]**: 可并行执行（不同文件，无依赖）
- **[Story]**: 任务所属的用户故事（如 US1, US2, US3）
- 所有描述包含具体文件路径

## 路径约定

- **单一项目**: `src/`, `tests/` 位于仓库根目录
- 以下路径基于 plan.md 中的项目结构

---

## Phase 1: 项目初始化（共享基础设施）

**目的**: 项目初始化和基础结构搭建

- [X] T001 创建 Rust 项目结构（src/, tests/ 目录）
- [X] T002 初始化 Cargo 项目，添加依赖到 Cargo.toml（clap, reqwest, tokio, serde, serde_json, serde_yaml, thiserror, anyhow, comfy-table, dirs, chrono）
- [X] T003 [P] 配置 .gitignore，排除目标文件和配置文件（target/, .glm/config.yaml）
- [X] T004 [P] 创建 README.md 基础结构（项目标题、简短描述）

---

## Phase 2: 基础设施（阻塞性前置条件）

**目的**: 所有用户故事实现前必须完成的核心基础设施

**⚠️ 关键**: 在此阶段完成前，不能开始任何用户故事的实现

- [X] T005 在 src/error.rs 中定义错误类型 CliError（使用 thiserror）
- [X] T006 [P] 在 src/config.rs 中实现 Config 结构体和加载逻辑（环境变量优先，配置文件备选）
- [X] T007 [P] 在 src/config.rs 中实现 Config::validate 方法（API 密钥格式、URL 有效性、超时范围）
- [X] T008 [P] 在 src/api/types.rs 中定义 API 数据类型（ApiPlan, ApiResponse, ApiErrorResponse）
- [X] T009 [P] 在 src/api/types.rs 中实现 ApiPlan::validate 方法（数据完整性验证）
- [X] T010 在 src/api/mod.rs 中创建 API 模块导出（pub use types::*）
- [X] T011 创建示例配置文件 glm-config.example.yaml（包含注释和示例）

**检查点**: 基础设施就绪 - 用户故事实现现在可以并行开始

---

## Phase 3: 用户故事 1 - 查询 API 计划使用情况 (优先级: P1) 🎯 MVP

**目标**: 实现核心查询功能，用户可通过命令行查询 GLM API 计划信息

**独立测试**: 执行 `glm` 命令，验证显示完整的计划信息（总额度、已使用量、剩余量、有效期）

### 实现任务

- [X] T012 [US1] 在 src/api/client.rs 中实现 GlmClient 结构体（持有 Config 和 reqwest::Client）
- [X] T013 [US1] 在 src/api/client.rs 中实现 GlmClient::new 方法（构建客户端，设置超时和 User-Agent）
- [X] T014 [US1] 在 src/api/client.rs 中实现 GlmClient::fetch_plan 方法（发送 GET 请求到 /api/paas/v4/plans）
- [X] T015 [US1] 在 src/api/client.rs 中实现 Bearer Token 认证逻辑（设置 Authorization Header）
- [X] T016 [US1] 在 src/api/client.rs 中实现 HTTP 错误处理（映射 401, 403, 404, 429, 5xx 到中文错误消息）
- [X] T017 [US1] 在 src/api/client.rs 中实现超时处理（30 秒超时，友好中文提示）
- [X] T018 [US1] 在 src/api/client.rs 中实现网络错误处理（连接失败、DNS 解析失败）
- [X] T019 [US1] 在 src/output/table.rs 中实现 render_table 函数（使用 comfy-table 渲染 ApiPlan）
- [X] T020 [US1] 在 src/output/table.rs 中实现中文字符处理（UTF-8 模式，ASCII 降级）
- [X] T021 [US1] 在 src/output/mod.rs 中创建输出模块导出（pub use table::*）
- [X] T022 [US1] 在 src/main.rs 中实现命令行参数解析（使用 clap derive API）
- [X] T023 [US1] 在 src/main.rs 中实现配置加载逻辑（调用 Config::load）
- [X] T024 [US1] 在 src/main.rs 中实现配置验证逻辑（调用 config.validate()，中文错误提示）
- [X] T025 [US1] 在 src/main.rs 中实现 API 客户端调用（GlmClient::fetch_plan）
- [X] T026 [US1] 在 src/main.rs 中实现 API 响应验证（调用 ApiPlan::validate，数据完整性检查）
- [X] T027 [US1] 在 src/main.rs 中实现表格格式化输出（调用 render_table）
- [X] T028 [US1] 在 src/main.rs 中实现主函数逻辑（完整执行流程：加载配置 → 查询 API → 显示结果）
- [X] T029 [US1] 在 src/main.rs 中实现错误处理和中文错误消息（所有错误场景：配置错误、网络错误、API 错误、数据验证错误）
- [X] T030 [US1] 在 src/main.rs 中实现 API 密钥掩码逻辑（日志和错误信息中不泄露完整密钥）

**检查点**: 此时，用户故事 1 应该完全功能化且可独立测试

---

## Phase 4: 用户故事 2 - 多种输出格式支持 (优先级: P2)

**目标**: 支持 `--format` 参数，允许用户选择输出格式（table、json）

**独立测试**: 执行 `glm --format table` 和 `glm --format json`，验证输出格式正确且内容一致

### 实现任务

- [ ] T031 [P] [US2] 在 src/api/types.rs 中定义 OutputFormat 枚举（Table, Json，实现 ValueEnum 和 Default）
- [ ] T032 [P] [US2] 在 src/output/json.rs 中实现 render_json 函数（序列化 ApiPlan 为 JSON）
- [ ] T033 [P] [US2] 在 src/output/mod.rs 中添加 JSON 模块导出（pub use json::*）
- [ ] T034 [US2] 在 src/main.rs 中添加 --format 参数到 clap Parser（derive，默认 Table）
- [ ] T035 [US2] 在 src/main.rs 中实现格式参数验证（检查有效值，列出支持的格式）
- [ ] T036 [US2] 在 src/main.rs 中实现条件输出逻辑（根据 OutputFormat 调用 render_table 或 render_json）
- [ ] T037 [US2] 在 src/main.rs 中实现无效格式参数的错误处理（中文错误消息 + 支持的格式列表）

**检查点**: 此时，用户故事 1 和 2 都应该独立工作

---

## Phase 5: 用户故事 3 - 详细的错误提示和使用帮助 (优先级: P3)

**目标**: 提供 `--help` 和 `--version` 参数，完善所有错误场景的中文提示

**独立测试**: 执行 `glm --help` 和 `glm --version`，验证显示完整的中文帮助信息和版本号

### 实现任务

- [ ] T038 [US3] 在 Cargo.toml 中设置项目版本号（1.0.0）
- [ ] T039 [US3] 在 src/main.rs 中实现 --help 参数（使用 clap derive，完整的中文使用说明）
- [ ] T040 [US3] 在 src/main.rs 中实现 --version 参数（显示版本号）
- [ ] T041 [US3] 在 src/main.rs 中完善配置文件格式错误的中文提示（指出具体错误位置和修正建议）
- [ ] T042 [US3] 在 src/api/client.rs 中完善 API 超时错误的中文提示（问题描述、原因、解决建议）
- [ ] T043 [US3] 在 src/api/client.rs 中完善 API 限流错误的中文提示（可操作的解决建议）
- [ ] T044 [US3] 在 src/api/client.rs 中完善所有 API 错误代码的中文映射（400, 401, 403, 404, 429, 500, 502, 503, 504）

**检查点**: 所有用户故事现在都应该独立功能化

---

## Phase 6: 完善与跨领域关注点

**目的**: 影响多个用户故事的改进和完善

- [ ] T045 [P] 更新 README.md，添加完整的安装说明、使用示例和配置方法
- [ ] T046 [P] 在 glm-config.example.yaml 中添加详细注释（每个配置项的说明和示例）
- [ ] T047 代码清理和重构（确保函数 < 50 行，模块职责清晰）
- [ ] T048 性能优化（启动时间 < 100ms，内存使用 < 50MB）
- [ ] T049 运行 cargo clippy，修复所有警告
- [ ] T050 运行 cargo fmt，统一代码格式
- [ ] T051 安全加固（验证配置文件权限 600，API 密钥不泄露到日志）
- [ ] T052 验证宪章合规性（中文输出、安全第一、错误友好）

---

## 依赖关系与执行顺序

### 阶段依赖

- **Phase 1（项目初始化）**: 无依赖 - 可立即开始
- **Phase 2（基础设施）**: 依赖 Phase 1 完成 - 阻塞所有用户故事
- **Phase 3+（用户故事）**: 都依赖 Phase 2 完成
  - 用户故事可以按优先级顺序（P1 → P2 → P3）顺序实现
  - 或在有多个开发者时并行实现
- **Phase 6（完善）**: 依赖所有期望的用户故事完成

### 用户故事依赖

- **用户故事 1 (P1)**: Phase 2 完成后可开始 - 无其他故事依赖
- **用户故事 2 (P2)**: Phase 2 完成后可开始 - 可能与 US1 集成但应独立可测
- **用户故事 3 (P3)**: Phase 2 完成后可开始 - 可能与 US1/US2 集成但应独立可测

### 各用户故事内

- 核心数据类型 → 服务/客户端 → 输出格式化 → CLI 集成
- 核心实现 → 错误处理 → 验证
- 故事完成前移动到下一优先级

### 并行机会

- Phase 1 中所有标记 [P] 的任务可并行
- Phase 2 中所有标记 [P] 的任务可并行（在 Phase 2 内）
- Phase 2 完成后，所有用户故事可并行开始（如团队容量允许）
- 用户故事内标记 [P] 的任务可并行（如 T031/T032/T033）
- 不同用户故事可由不同团队成员并行工作

---

## 并行示例：用户故事 1

```bash
# 用户故事 1 的并行任务示例：
# 可并行的数据类型任务：
Task: "T008 在 src/api/types.rs 中定义 API 数据类型"
Task: "T006 在 src/config.rs 中实现 Config 结构体"

# 可并行的模块导出任务：
Task: "T010 在 src/api/mod.rs 中创建 API 模块导出"
Task: "T011 创建示例配置文件 glm-config.example.yaml"
```

---

## 并行示例：用户故事 2

```bash
# 用户故事 2 的并行任务示例：
# 可并行的输出格式任务：
Task: "T031 在 src/api/types.rs 中定义 OutputFormat 枚举"
Task: "T032 在 src/output/json.rs 中实现 render_json 函数"
Task: "T033 在 src/output/mod.rs 中添加 JSON 模块导出"
```

---

## 实现策略

### MVP 优先（仅用户故事 1）

1. 完成 Phase 1: 项目初始化
2. 完成 Phase 2: 基础设施（关键 - 阻塞所有故事）
3. 完成 Phase 3: 用户故事 1
4. **停止并验证**: 独立测试用户故事 1
5. 如就绪则部署/演示

### 增量交付

1. 完成项目初始化 + 基础设施 → 基础就绪
2. 添加用户故事 1 → 独立测试 → 部署/演示（MVP！）
3. 添加用户故事 2 → 独立测试 → 部署/演示
4. 添加用户故事 3 → 独立测试 → 部署/演示
5. 每个故事在不破坏前一个故事的情况下增加价值

### 并行团队策略

有多个开发者时：

1. 团队一起完成项目初始化 + 基础设施
2. 基础设施完成后：
   - 开发者 A: 用户故事 1
   - 开发者 B: 用户故事 2
   - 开发者 C: 用户故事 3
3. 故事独立完成和集成

---

## 说明

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以便追踪
- 每个用户故事应独立可完成和可测试
- 每个任务或逻辑组后提交
- 在任何检查点停止以独立验证故事
- 避免：模糊任务、同文件冲突、破坏独立性的跨故事依赖

---

## 任务统计

- **总任务数**: 52
- **Phase 1（项目初始化）**: 4 个任务
- **Phase 2（基础设施）**: 7 个任务
- **Phase 3（用户故事 1）**: 19 个任务
- **Phase 4（用户故事 2）**: 7 个任务
- **Phase 5（用户故事 3）**: 7 个任务
- **Phase 6（完善）**: 8 个任务

**并行机会**: 约 15 个任务标记为 [P]，可并行执行

**MVP 范围**: Phase 1-3（项目初始化 + 基础设施 + 用户故事 1）= 30 个任务

---

## 下一步

1. 从 Phase 1 开始，按顺序执行任务
2. 每个 Phase 完成后验证检查点
3. MVP 完成后（Phase 3）可暂停进行演示
4. 继续实现用户故事 2 和 3 以完成完整功能
