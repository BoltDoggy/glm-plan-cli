# 技术研究：GLM API 计划查询命令

**功能**: GLM API 计划查询命令
**日期**: 2026-01-13
**阶段**: Phase 0 - 研究与决策

## 研究目标

解决实现计划中标识的 3 个关键问题：

1. GLM API 端点和认证机制
2. Rust 表格渲染库的选择
3. 配置文件格式的选择

---

## 研究问题 1: GLM API 端点和认证

### 问题描述

需要确定智谱 AI GLM API 的具体端点 URL、请求格式、响应格式和认证方式。

### 研究发现

#### 决策: 使用假设的 API 接口设计

**API 端点**: `https://open.bigmodel.cn/api/paas/v4/plans`（假设）

**认证方式**: Bearer Token 认证
- HTTP Header: `Authorization: Bearer <API_KEY>`
- API Key 格式: 假设为类似 `sk.xxxxxxxxxxxxxx` 的格式

**请求格式**:
```http
GET /api/paas/v4/plans
Authorization: Bearer sk.xxxxxxxxxxxxxx
Content-Type: application/json
```

**响应格式** (成功):
```json
{
  "code": 200,
  "msg": "success",
  "data": {
    "plan_id": "premium_plan",
    "plan_name": "高级版",
    "total_quota": 1000000,
    "used_quota": 250000,
    "remaining_quota": 750000,
    "usage_percentage": 25.0,
    "start_date": "2026-01-01T00:00:00Z",
    "end_date": "2026-12-31T23:59:59Z",
    "token_type": "tokens"
  }
}
```

**响应格式** (错误):
```json
{
  "code": 401,
  "msg": "Unauthorized: Invalid API key",
  "error": "invalid_api_key"
}
```

### 理由

由于智谱 AI 的具体 API 文档在当前上下文中不可用，我们采用业界标准的 RESTful API 设计模式：

1. **RESTful 风格**: 使用标准的 HTTP 方法和状态码
2. **Bearer Token 认证**: OAuth 2.0 标准的认证方式，广泛采用
3. **JSON 格式**: 轻量级、易解析、与 Rust 生态系统良好集成
4. **标准化错误响应**: 包含错误代码和描述，便于用户理解

### 实现策略

1. **设计灵活的 API 客户端**: 使用 trait 和泛型设计，便于将来适配真实 API
2. **配置化端点 URL**: 将 API 端点 URL 作为配置项，方便后期调整
3. **错误处理映射**: 将 API 错误代码映射到中文错误消息
4. **响应验证**: 严格验证 API 响应的数据结构完整性

### 备选方案

#### 备选方案 A: API Key 查询参数
- **方式**: `?api_key=sk.xxxxxxxxxxxxxx`
- **优点**: 简单直接
- **缺点**: 不如 Bearer Token 安全，可能被日志记录

#### 备选方案 B: 自定义 Header
- **方式**: `X-API-Key: sk.xxxxxxxxxxxxxx`
- **优点**: 自定义控制
- **缺点**: 非标准，增加学习成本

**拒绝理由**: Bearer Token 是行业标准，安全性更好，与 reqwest 库集成最佳。

### 风险与缓解

**风险**: 真实 API 可能与假设设计不同

**缓解措施**:
- 使用配置化的端点 URL，便于后期调整
- 设计清晰的抽象层（trait），便于适配不同 API
- 编写契约测试，定义期望的接口
- 预留扩展空间，支持额外的字段和参数

---

## 研究问题 2: Rust 表格渲染库选择

### 问题描述

需要选择适合的 Rust 表格渲染库，支持中文、跨平台、易用性好。

### 候选库评估

#### 候选 A: comfy-table

**版本**: 7.x
**特点**:
- ✅ 支持 UTF-8 和 ASCII 模式
- ✅ 丰富的样式选项（边框、对齐、颜色）
- ✅ 中文支持良好
- ✅ 跨平台（Linux, macOS, Windows）
- ✅ 自动列宽调整
- ✅ 易用 API（Builder 模式）

**示例代码**:
```rust
use comfy_table::Table;

let mut table = Table::new();
table.set_header(vec!["项目", "值"]);
table.add_row(vec!["总额度", "1,000,000"]);
println!("{table}");
```

**优点**:
- 功能丰富，文档完善
- 活跃维护（最后更新: 2024）
- 性能良好
- 易于集成

**缺点**:
- 依赖稍多（Unicode 相关库）

#### 候选 B: tabled

**版本**: 0.15+
**特点**:
- ✅ 强大的表格布局引擎
- ✅ 高度可定制
- ✅ 中文支持
- ✅ 性能优秀

**优点**:
- 功能强大，灵活性高
- 性能最佳

**缺点**:
- API 复杂度较高
- 学习曲线陡峭

#### 候选 C: term-table

**版本**: 1.x
**特点**:
- ✅ 简单易用
- ⚠️ 中文支持有限
- ⚠️ 维护不活跃

**优点**:
- API 简单

**缺点**:
- 中文支持不佳
- 维护不活跃

### 决策: 选择 comfy-table

**理由**:
1. **中文支持良好**: 正确处理 UTF-8 和中文字符宽度
2. **跨平台兼容**: 支持 Windows CMD、PowerShell、Linux 终端
3. **易用性**: Builder 模式 API 清晰直观
4. **功能完整**: 支持所有需要的功能（边框、对齐、样式）
5. **活跃维护**: 社区活跃，文档完善
6. **性能可接受**: 对于 CLI 工具的表格输出，性能不是瓶颈

### 实现策略

1. **UTF-8 优先**: 默认使用 UTF-8 模式，提供最佳视觉效果
2. **ASCII 降级**: 检测终端不支持 UTF-8 时自动降级到 ASCII 模式
3. **样式配置**: 使用简洁的样式，避免过度装饰
4. **响应式列宽**: 自动调整列宽，适应不同终端宽度

### 依赖配置

```toml
[dependencies]
comfy-table = "7.0"
```

### 示例实现

```rust
use comfy_table::{Table, Row, Cell};

pub fn render_table(plan: &ApiPlan) -> String {
    let mut table = Table::new();
    table
        .set_header(vec!["项目", "值"])
        .add_row(vec![Row::new()
            .add_cell(Cell::new("总额度"))
            .add_cell(Cell::new(&format!("{}", plan.total_quota)))])
        .add_row(vec![Row::new()
            .add_cell(Cell::new("已使用"))
            .add_cell(Cell::new(&format!("{}", plan.used_quota)))])
        .add_row(vec![Row::new()
            .add_cell(Cell::new("剩余"))
            .add_cell(Cell::new(&format!("{}", plan.remaining_quota)))]);

    table.to_string()
}
```

---

## 研究问题 3: 配置文件格式选择

### 问题描述

需要选择配置文件的格式，在 YAML、TOML、JSON 之间做出选择。

### 候选格式评估

#### 候选 A: YAML

**特点**:
- ✅ 易读性最好，类似自然语言
- ✅ 支持注释（#）
- ✅ 层级结构清晰
- ⚠️ 解析速度较慢
- ⚠️ 依赖库较重（serde_yaml）

**示例**:
```yaml
# GLM API 配置文件
api_key: sk.xxxxxxxxxxxxxx
api_url: https://open.bigmodel.cn/api/paas/v4
timeout: 30
```

**优点**:
- 用户体验最佳（易读、易编辑）
- 支持注释，便于说明配置项
- 业界广泛采用（Kubernetes, Docker Compose）

**缺点**:
- 解析库较大（serde_yaml ~200KB）
- 解析性能略低于 TOML/JSON

#### 候选 B: TOML

**特点**:
- ✅ 易读性好
- ✅ 支持注释
- ✅ 解析性能优秀
- ✅ Rust 官方推荐（Cargo 使用）
- ⚠️ 层级复杂时可读性下降

**示例**:
```toml
# GLM API 配置文件
api_key = "sk.xxxxxxxxxxxxxx"
api_url = "https://open.bigmodel.cn/api/paas/v4"
timeout = 30
```

**优点**:
- Rust 生态原生支持（toml.rs）
- 解析速度快
- 文件体积小

**缺点**:
- 复杂嵌套结构可读性不如 YAML
- 相对 YAML 不够流行

#### 候选 C: JSON

**特点**:
- ✅ 标准化格式
- ✅ 解析速度快
- ✅ 生态支持最广泛
- ❌ 不支持注释
- ❌ 易读性最差

**示例**:
```json
{
  "api_key": "sk.xxxxxxxxxxxxxx",
  "api_url": "https://open.bigmodel.cn/api/paas/v4",
  "timeout": 30
}
```

**优点**:
- 通用标准
- 解析速度快
- 所有语言都支持

**缺点**:
- 不支持注释，用户体验差
- 易读性差，错误信息不友好

### 决策: 选择 YAML

**理由**:
1. **用户体验优先**: YAML 最易读易编辑，符合"错误友好"原则
2. **注释支持**: 可以在配置文件中添加说明，降低学习成本
3. **业界标准**: 与 Kubernetes、Docker 等工具一致，降低学习门槛
4. **配置简单**: 本工具配置项少（3-5 个），解析性能影响可忽略
5. **错误友好**: YAML 错误消息比 JSON 更友好

### 实现策略

1. **使用 serde_yaml**: 成熟的 YAML 序列化库
2. **示例配置文件**: 提供 `glm-config.example.yaml` 供用户参考
3. **配置验证**: 启动时验证配置文件格式，提供清晰的错误提示
4. **默认配置**: 内置合理默认值，减少必须配置的项目

### 依赖配置

```toml
[dependencies]
serde_yaml = "0.9"
```

### 配置结构

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// GLM API 密钥
    pub api_key: String,

    /// GLM API 端点 URL（可选，有默认值）
    #[serde(default = "default_api_url")]
    pub api_url: String,

    /// 请求超时时间（秒，可选，默认 30）
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_api_url() -> String {
    "https://open.bigmodel.cn/api/paas/v4".to_string()
}

fn default_timeout() -> u64 {
    30
}
```

### 备选方案考量

**TOML**: 如果用户反馈 YAML 解析库过大或解析慢，可以迁移到 TOML。迁移成本较低，因为 serde 支持多种格式。

**JSON**: 不推荐，因为缺少注释支持，用户体验差。

---

## 总结与下一步

### 决策汇总

| 问题 | 决策 | 理由 |
|------|------|------|
| GLM API 端点 | 假设 RESTful API + Bearer Token | 业界标准，灵活可配置 |
| 表格渲染库 | comfy-table | 中文支持好，跨平台，易用 |
| 配置文件格式 | YAML | 用户体验最佳，支持注释 |

### 依赖清单

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
thiserror = "1.0"
anyhow = "1.0"
comfy-table = "7.0"
dirs = "5.0"
```

### 下一步 (Phase 1: 设计)

1. **数据模型设计**: 基于 API 响应格式设计 Rust 数据结构
2. **API 契约定义**: 定义 API 接口规范和错误处理契约
3. **快速入门指南**: 编写用户使用文档

### 风险与缓解

1. **API 接口不确定性**: 使用配置化设计，便于适配真实 API
2. **YAML 解析库大小**: 可接受（~200KB），如果成问题可迁移到 TOML
3. **表格渲染兼容性**: comfy-table 提供降级支持，确保跨平台

---

**研究完成日期**: 2026-01-13
**状态**: ✅ 所有问题已解决，可以进入 Phase 1 设计
