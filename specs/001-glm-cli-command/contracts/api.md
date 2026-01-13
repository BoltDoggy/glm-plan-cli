# API 契约定义：GLM API 计划查询

**功能**: GLM API 计划查询命令
**日期**: 2026-01-13
**版本**: 1.0.0

## 概述

本文档定义 GLM API 计划查询客户端与智谱 AI GLM API 服务之间的接口契约。包括请求格式、响应格式、错误处理和超时机制。

---

## API 端点

### 查询计划信息

**端点**: `GET /api/paas/v4/plans`
**完整 URL**: `https://open.bigmodel.cn/api/paas/v4/plans`
**认证**: Bearer Token
**超时**: 30 秒

### 请求格式

#### HTTP Headers

```http
GET /api/paas/v4/plans HTTP/1.1
Host: open.bigmodel.cn
Authorization: Bearer sk.xxxxxxxxxxxxxx
Content-Type: application/json
Accept: application/json
User-Agent: glm-cli/1.0.0
```

#### Header 说明

| Header | 值 | 必需 | 说明 |
|--------|-----|------|------|
| `Authorization` | `Bearer <API_KEY>` | 是 | Bearer Token 认证 |
| `Content-Type` | `application/json` | 是 | 请求内容类型 |
| `Accept` | `application/json` | 是 | 期望的响应类型 |
| `User-Agent` | `glm-cli/<version>` | 是 | 客户端标识 |

#### 请求参数

无（Query Parameters 或 Request Body 均不需要）

---

## 响应格式

### 成功响应

#### HTTP Status Code

`200 OK`

#### Response Headers

```http
Content-Type: application/json
Content-Length: <size>
```

#### Response Body

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

#### 字段说明

| 路径 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `code` | number | 是 | HTTP 状态码（200 表示成功） |
| `msg` | string | 是 | 响应消息（`"success"`） |
| `data` | object | 是 | 计划数据对象 |
| `data.plan_id` | string | 是 | 计划唯一标识符 |
| `data.plan_name` | string | 是 | 计划名称（中文） |
| `data.total_quota` | number | 是 | 总额度（>= 0） |
| `data.used_quota` | number | 是 | 已使用量（>= 0 且 <= total_quota） |
| `data.remaining_quota` | number | 是 | 剩余量（= total_quota - used_quota） |
| `data.usage_percentage` | number | 是 | 使用百分比（0-100） |
| `data.start_date` | string | 是 | 计划开始日期（ISO 8601 格式） |
| `data.end_date` | string | 是 | 计划结束日期（ISO 8601 格式） |
| `data.token_type` | string | 是 | 额度类型（`"tokens"` 或 `"requests"`） |

---

### 错误响应

#### 通用错误格式

所有错误响应遵循统一格式：

```json
{
  "code": <error_code>,
  "msg": "<error_message>",
  "error": "<error_type>"
}
```

#### 错误代码映射

| HTTP Code | `code` | `error` | 中文消息 | 用户友好描述 | 解决建议 |
|-----------|--------|---------|----------|-------------|----------|
| 400 | 400 | `bad_request` | 请求格式错误 | 请求参数格式不正确 | 请检查请求格式 |
| 401 | 401 | `unauthorized` | 认证失败 | API 密钥无效或已过期 | 请检查 API 密钥配置 |
| 403 | 403 | `forbidden` | 无权限 | 您的账户无权限访问此资源 | 请联系客服确认权限 |
| 404 | 404 | `not_found` | 端点不存在 | API 端点不存在 | 请检查 API URL 配置 |
| 429 | 429 | `rate_limit` | 请求过于频繁 | 请求过于频繁，已被限流 | 请稍后再试 |
| 500 | 500 | `internal_error` | 服务器错误 | 服务器内部错误 | 请稍后重试 |
| 502 | 502 | `bad_gateway` | 网关错误 | 网关错误 | 请稍后重试 |
| 503 | 503 | `service_unavailable` | 服务不可用 | 服务暂时不可用 | 请稍后重试 |
| 504 | 504 | `gateway_timeout` | 网关超时 | 网关超时 | 请稍后重试 |

#### 错误响应示例

**示例 1: 认证失败 (401)**

```json
{
  "code": 401,
  "msg": "Unauthorized: Invalid API key",
  "error": "unauthorized"
}
```

**示例 2: 请求过于频繁 (429)**

```json
{
  "code": 429,
  "msg": "Too many requests, rate limit exceeded",
  "error": "rate_limit"
}
```

**示例 3: 服务器错误 (500)**

```json
{
  "code": 500,
  "msg": "Internal server error",
  "error": "internal_error"
}
```

---

## 超时机制

### 客户端超时设置

- **连接超时**: 10 秒
- **读取超时**: 30 秒
- **总超时**: 30 秒

### 超时错误处理

当请求超时时，客户端应返回友好的中文错误消息：

```text
错误：API 请求超时
原因：服务器在 30 秒内未响应
建议：
1. 请检查网络连接是否正常
2. 请稍后重试
3. 如问题持续，请联系支持团队
```

---

## 重试策略

### 不重试的错误

以下错误不应自动重试：
- `400 Bad Request` - 请求格式错误
- `401 Unauthorized` - 认证失败
- `403 Forbidden` - 无权限
- `404 Not Found` - 端点不存在

### 可重试的错误

以下错误可以重试（最多 3 次，指数退避）：
- `429 Too Many Requests` - 限流（退避: 1s, 2s, 4s）
- `500 Internal Server Error` - 服务器错误（退避: 1s, 2s, 4s）
- `502 Bad Gateway` - 网关错误（退避: 1s, 2s, 4s）
- `503 Service Unavailable` - 服务不可用（退避: 1s, 2s, 4s）
- `504 Gateway Timeout` - 网关超时（退避: 1s, 2s, 4s）
- 网络错误（连接失败、DNS 解析失败）

---

## 数据验证规则

### 客户端验证（发送请求前）

1. **API 密钥格式**:
   - 必须以 `sk.` 开头
   - 长度必须 >= 10 个字符
   - 不能包含空白字符

2. **URL 格式**:
   - 必须使用 HTTPS 协议
   - 必须是有效的 URL 格式

3. **超时设置**:
   - 必须 > 0 且 <= 300 秒

### 服务端响应验证（接收响应后）

1. **HTTP 状态码验证**:
   - 成功: `200 OK`
   - 错误: `4xx`, `5xx`

2. **响应体验证**:
   - 必须是有效的 JSON 格式
   - 必须包含 `code`, `msg` 字段
   - 成功响应必须包含 `data` 字段

3. **数据完整性验证**:
   - `data.total_quota` 必须 > 0
   - `data.used_quota` 必须 >= 0 且 <= `total_quota`
   - `data.remaining_quota` 必须 = `total_quota - used_quota`
   - `data.usage_percentage` 必须 >= 0 且 <= 100
   - `data.end_date` 必须 >= `data.start_date`

---

## 安全要求

### API 密钥保护

1. **传输安全**:
   - 必须使用 HTTPS 协议
   - API 密钥在 HTTP Header 中传输（`Authorization: Bearer <token>`）

2. **存储安全**:
   - 配置文件权限: `600`（仅用户可读写）
   - 配置文件路径: `~/.glm/config.yaml`
   - 配置文件必须加入 `.gitignore`

3. **日志安全**:
   - **禁止**在日志中完整输出 API 密钥
   - **禁止**在错误消息中泄露 API 密钥
   - 可以显示掩码后的密钥（如 `sk.******xxxx`）

### 请求签名（未来扩展）

当前版本不需要请求签名。如果未来 API 需要签名，支持扩展。

---

## 版本控制

### API 版本

- **当前版本**: `v4`（在 URL 路径中）
- **版本策略**: URL 路径版本控制

### 客户端版本

- **当前版本**: `1.0.0`
- **User-Agent**: `glm-cli/1.0.0`

---

## 测试契约

### 契约测试用例

#### 测试用例 1: 成功获取计划信息

**请求**:
```http
GET /api/paas/v4/plans
Authorization: Bearer sk.valid_api_key
```

**预期响应**:
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

#### 测试用例 2: API 密钥无效

**请求**:
```http
GET /api/paas/v4/plans
Authorization: Bearer sk.invalid_key
```

**预期响应**:
```json
{
  "code": 401,
  "msg": "Unauthorized: Invalid API key",
  "error": "unauthorized"
}
```

#### 测试用例 3: 请求过于频繁

**请求**:
```http
GET /api/paas/v4/plans
Authorization: Bearer sk.rate_limited_key
```

**预期响应**:
```json
{
  "code": 429,
  "msg": "Too many requests, rate limit exceeded",
  "error": "rate_limit"
}
```

---

## OpenAPI 规范（可选）

如果需要机器可读的 API 规范，可以使用以下 OpenAPI 3.0 定义：

```yaml
openapi: 3.0.0
info:
  title: GLM API
  version: 4.0.0
  description: 智谱 AI GLM API

servers:
  - url: https://open.bigmodel.cn/api/paas/v4
    description: 生产环境

paths:
  /plans:
    get:
      summary: 查询计划信息
      description: 获取当前用户的 API 使用计划信息
      security:
        - BearerAuth: []
      responses:
        '200':
          description: 成功获取计划信息
          content:
            application/json:
              schema:
                type: object
                properties:
                  code:
                    type: integer
                    example: 200
                  msg:
                    type: string
                    example: success
                  data:
                    $ref: '#/components/schemas/ApiPlan'
        '401':
          description: 认证失败
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '429':
          description: 请求过于频繁
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'

components:
  securitySchemes:
    BearerAuth:
      type: http
      scheme: bearer
      bearerFormat: API Key

  schemas:
    ApiPlan:
      type: object
      required:
        - plan_id
        - plan_name
        - total_quota
        - used_quota
        - remaining_quota
        - usage_percentage
        - start_date
        - end_date
        - token_type
      properties:
        plan_id:
          type: string
          example: premium_plan
        plan_name:
          type: string
          example: 高级版
        total_quota:
          type: integer
          minimum: 1
          example: 1000000
        used_quota:
          type: integer
          minimum: 0
          example: 250000
        remaining_quota:
          type: integer
          minimum: 0
          example: 750000
        usage_percentage:
          type: number
          minimum: 0
          maximum: 100
          example: 25.0
        start_date:
          type: string
          format: date-time
          example: 2026-01-01T00:00:00Z
        end_date:
          type: string
          format: date-time
          example: 2026-12-31T23:59:59Z
        token_type:
          type: string
          enum: [tokens, requests]
          example: tokens

    ErrorResponse:
      type: object
      required:
        - code
        - msg
        - error
      properties:
        code:
          type: integer
          example: 401
        msg:
          type: string
          example: Unauthorized: Invalid API key
        error:
          type: string
          example: unauthorized
```

---

## 总结

### 契约要点

1. **认证方式**: Bearer Token（HTTP Header）
2. **请求格式**: GET 请求，无参数
3. **响应格式**: JSON，包含 `code`, `msg`, `data`
4. **超时设置**: 30 秒总超时
5. **错误处理**: 统一错误格式，中文友好消息
6. **安全要求**: HTTPS + API 密钥保护

### 下一步

1. **快速入门指南**: 用户配置和使用文档
2. **实现**: 基于 API 契约实现客户端代码

---

**API 契约完成日期**: 2026-01-13
**状态**: ✅ API 契约定义完成，可以进入实现阶段
