# 数据模型：GLM API 计划查询命令

**功能**: GLM API 计划查询命令
**日期**: 2026-01-13
**阶段**: Phase 1 - 数据模型设计

## 概述

本文档定义 GLM API 计划查询命令涉及的所有数据实体、字段、验证规则和状态转换。

---

## 核心实体

### 1. ApiPlan (API 计划信息)

**描述**: 代表用户的 GLM API 使用计划，包含额度、使用情况和有效期信息。

#### 字段定义

| 字段名 | 类型 | 可空 | 描述 | 示例值 |
|--------|------|------|------|--------|
| `plan_id` | String | 否 | 计划唯一标识符 | `"premium_plan"` |
| `plan_name` | String | 否 | 计划名称 | `"高级版"` |
| `total_quota` | u64 | 否 | 总额度（token 数或请求数） | `1000000` |
| `used_quota` | u64 | 否 | 已使用量 | `250000` |
| `remaining_quota` | u64 | 否 | 剩余量（计算字段） | `750000` |
| `usage_percentage` | f64 | 否 | 使用百分比（0-100） | `25.0` |
| `start_date` | DateTime<Utc> | 否 | 计划开始日期 | `2026-01-01T00:00:00Z` |
| `end_date` | DateTime<Utc> | 否 | 计划结束日期 | `2026-12-31T23:59:59Z` |
| `token_type` | String | 否 | 额度类型（`"tokens"` 或 `"requests"`） | `"tokens"` |

#### 验证规则

1. **total_quota**: 必须 > 0
2. **used_quota**: 必须 >= 0 且 <= total_quota
3. **remaining_quota**: 必须等于 total_quota - used_quota
4. **usage_percentage**: 必须 >= 0 且 <= 100
5. **end_date**: 必须 >= start_date

#### 计算字段

- `remaining_quota = total_quota - used_quota`
- `usage_percentage = (used_quota / total_quota) * 100.0`

#### 序列化示例 (JSON)

```json
{
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
```

#### Rust 数据结构

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiPlan {
    pub plan_id: String,
    pub plan_name: String,
    pub total_quota: u64,
    pub used_quota: u64,
    pub remaining_quota: u64,
    pub usage_percentage: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub token_type: String,
}

impl ApiPlan {
    /// 验证 ApiPlan 数据的完整性
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.total_quota == 0 {
            return Err(ValidationError::InvalidTotalQuota);
        }
        if self.used_quota > self.total_quota {
            return Err(ValidationError::UsedQuotaExceedsTotal);
        }
        if self.remaining_quota != self.total_quota - self.used_quota {
            return Err(ValidationError::InvalidRemainingQuota);
        }
        if self.usage_percentage < 0.0 || self.usage_percentage > 100.0 {
            return Err(ValidationError::InvalidUsagePercentage);
        }
        if self.end_date < self.start_date {
            return Err(ValidationError::InvalidDateRange);
        }
        Ok(())
    }
}
```

---

### 2. Config (配置信息)

**描述**: 用户的工具配置，包含 API 密钥和可选的连接参数。

#### 字段定义

| 字段名 | 类型 | 可空 | 默认值 | 描述 | 示例值 |
|--------|------|------|--------|------|--------|
| `api_key` | String | 否 | N/A | GLM API 密钥 | `"sk.xxxxxxxxxxxxxx"` |
| `api_url` | String | 是 | `"https://open.bigmodel.cn/api/paas/v4"` | API 端点 URL | `"https://open.bigmodel.cn/api/paas/v4"` |
| `timeout` | u64 | 是 | `30` | 请求超时时间（秒） | `30` |

#### 验证规则

1. **api_key**: 非空字符串，格式匹配 `/^sk\..+/`
2. **api_url**: 有效的 HTTPS URL
3. **timeout**: 必须 > 0 且 <= 300（5 分钟）

#### 配置来源优先级

1. 环境变量 `GLM_API_KEY`
2. 环境变量 `GLM_API_URL`（可选）
3. 环境变量 `GLM_TIMEOUT`（可选）
4. 配置文件 `~/.glm/config.yaml`
5. 默认值

#### 配置文件格式 (YAML)

```yaml
# GLM API 配置文件
# 请将此文件复制到 ~/.glm/config.yaml 并填入您的 API 密钥

# API 密钥（必填）
api_key: sk.xxxxxxxxxxxxxx

# API 端点 URL（可选，有默认值）
# api_url: https://open.bigmodel.cn/api/paas/v4

# 请求超时时间（秒，可选，默认 30）
# timeout: 30
```

#### Rust 数据结构

```rust
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub api_key: String,
    #[serde(default = "default_api_url")]
    pub api_url: String,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_api_url() -> String {
    "https://open.bigmodel.cn/api/paas/v4".to_string()
}

fn default_timeout() -> u64 {
    30
}

impl Config {
    /// 从环境变量和配置文件加载配置
    /// 优先级: 环境变量 > 配置文件 > 默认值
    pub fn load() -> Result<Self> {
        // 1. 尝试从环境变量加载
        if let Ok(api_key) = std::env::var("GLM_API_KEY") {
            return Ok(Config {
                api_key,
                api_url: std::env::var("GLM_API_URL")
                    .unwrap_or_else(|_| default_api_url()),
                timeout: std::env::var("GLM_TIMEOUT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(default_timeout),
            });
        }

        // 2. 尝试从配置文件加载
        let config_path = dirs::home_dir()
            .expect("无法确定主目录")
            .join(".glm/config.yaml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_yaml::from_str(&content)?;
            return Ok(config);
        }

        // 3. 未找到配置
        Err(anyhow::anyhow!(
            "未找到 API 密钥配置。请设置环境变量 GLM_API_KEY 或创建配置文件 ~/.glm/config.yaml"
        ))
    }

    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        if !self.api_key.starts_with("sk.") {
            return Err(anyhow::anyhow!("API 密钥格式无效，应以 'sk.' 开头"));
        }
        if self.api_key.len() < 10 {
            return Err(anyhow::anyhow!("API 密钥长度不足"));
        }
        if !self.api_url.starts_with("https://") {
            return Err(anyhow::anyhow!("API URL 必须使用 HTTPS"));
        }
        if self.timeout == 0 || self.timeout > 300 {
            return Err(anyhow::anyhow!("超时时间必须在 1-300 秒之间"));
        }
        Ok(())
    }
}
```

---

### 3. OutputFormat (输出格式)

**描述**: 定义支持的输出格式枚举。

#### 值定义

| 值 | 描述 | CLI 参数 |
|----|------|----------|
| `Table` | 表格格式（默认） | `--format table` 或不指定 |
| `Json` | JSON 格式 | `--format json` |

#### Rust 数据结构

```rust
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Deserialize, Serialize)]
pub enum OutputFormat {
    /// 表格格式（默认）
    #[serde(rename = "table")]
    Table,
    /// JSON 格式
    #[serde(rename = "json")]
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Table
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}
```

---

## API 响应数据结构

### 成功响应

```rust
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub code: u16,
    pub msg: String,
    pub data: ApiPlan,
}
```

### 错误响应

```rust
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub code: u16,
    pub msg: String,
    pub error: String,
}
```

---

## 错误类型

### 错误枚举

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("API 错误 ({code}): {msg}")]
    ApiError { code: u16, msg: String },

    #[error("数据验证错误: {0}")]
    ValidationError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

### 错误代码映射

| API 错误代码 | 中文描述 | 解决建议 |
|------------|---------|----------|
| 401 | 认证失败，API 密钥无效或已过期 | 请检查 API 密钥是否正确配置 |
| 403 | 无权限访问 | 请确认您的 API 密钥有访问权限 |
| 404 | API 端点不存在 | 请检查 API URL 配置是否正确 |
| 429 | 请求过于频繁，已被限流 | 请稍后再试 |
| 500 | 服务器内部错误 | 请稍后重试，如持续出现问题请联系支持 |
| 503 | 服务暂时不可用 | 请稍后重试 |

---

## 数据流图

```text
用户执行命令
    ↓
加载配置 (环境变量 → 配置文件 → 默认值)
    ↓
验证配置 (api_key 格式, URL 有效性)
    ↓
发送 API 请求 (GET /api/paas/v4/plans)
    ↓
接收 API 响应
    ↓
验证响应数据 (total_quota > 0, used_quota <= total_quota, etc.)
    ↓
格式化输出 (Table 或 JSON)
    ↓
显示给用户
```

---

## 状态转换

### 配置加载状态

```text
开始
  ↓
检查环境变量 GLM_API_KEY
  ├─ 存在 → 加载环境变量配置
  └─ 不存在 → 检查配置文件 ~/.glm/config.yaml
      ├─ 存在 → 加载配置文件
      └─ 不存在 → 返回错误："未找到 API 密钥配置"
  ↓
验证配置
  ├─ 有效 → 返回 Config
  └─ 无效 → 返回验证错误
```

---

## 数据完整性保证

### API 响应验证

1. **必需字段检查**: 确保 `total_quota`, `used_quota`, `start_date`, `end_date` 存在
2. **数值范围检查**: `total_quota > 0`, `0 <= used_quota <= total_quota`
3. **计算字段验证**: `remaining_quota = total_quota - used_quota`
4. **日期逻辑**: `end_date >= start_date`

### 配置验证

1. **API 密钥格式**: 以 `sk.` 开头，长度 >= 10
2. **URL 有效性**: 必须是 HTTPS URL
3. **超时范围**: 1-300 秒

---

## 总结

### 实体汇总

| 实体 | 用途 | 存储位置 |
|------|------|----------|
| `ApiPlan` | API 计划信息 | 运行时（API 响应） |
| `Config` | 用户配置 | 环境变量或配置文件 |
| `OutputFormat` | 输出格式 | 命令行参数 |

### 关系

- `Config` → 用于构建 API 请求 → 获取 `ApiPlan`
- `ApiPlan` → 根据 `OutputFormat` → 格式化输出

### 下一步

1. **API 契约定义**: 详细定义 API 请求/响应格式
2. **快速入门指南**: 用户配置和使用文档

---

**数据模型完成日期**: 2026-01-13
**状态**: ✅ 数据模型设计完成，可以进入 API 契约定义
