//! GLM API 客户端
//!
//! 提供与智谱 AI GLM API 交互的客户端实现。

use crate::api::{ApiResponse, ApiData};
use crate::config::Config;
use crate::error::CliError;
use anyhow::Result;
use std::time::Duration;

/// GLM API 客户端
pub struct GlmClient {
    /// HTTP 客户端
    client: reqwest::Client,
    /// API 配置
    config: Config,
}

impl GlmClient {
    /// 创建新的 GLM API 客户端
    ///
    /// # 参数
    ///
    /// * `config`: API 配置
    pub fn new(config: Config) -> Result<Self> {
        // 构建 HTTP 客户端，设置超时
        let timeout = Duration::from_secs(config.timeout);
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| CliError::NetworkError(format!("创建 HTTP 客户端失败: {}", e)))?;

        Ok(GlmClient { client, config })
    }

    /// 获取 API 使用情况
    ///
    /// 发送 GET 请求到 `/api/monitor/usage/quota/limit` 端点，
    /// 获取当前用户的 API 使用情况信息。
    ///
    /// # 返回
    ///
    /// 成功时返回 `ApiData`，失败时返回 `CliError`
    pub async fn fetch_usage(&self) -> Result<ApiData> {
        let url = format!("{}/monitor/usage/quota/limit", self.config.api_url);

        // 掩码 API 密钥用于日志（显示前 5 个字符）
        let _masked_key = if self.config.api_key.len() > 5 {
            format!("{}****{}", &self.config.api_key[..5], &self.config.api_key[self.config.api_key.len()-4..])
        } else {
            "****".to_string()
        };

        // 发送 HTTP GET 请求
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", format!("glm-cli/{}", env!("CARGO_PKG_VERSION")))
            .send()
            .await
            .map_err(|e| {
                // 提供友好的中文错误消息
                if e.is_timeout() {
                    CliError::NetworkError(
                        format!("API 请求超时（{}秒）\n\n原因: 服务器在 {} 秒内未响应\n\n建议:\n1. 请检查网络连接是否正常\n2. 请稍后重试\n3. 如问题持续，请联系支持团队",
                            self.config.timeout, self.config.timeout)
                    )
                } else if e.is_connect() {
                    CliError::NetworkError(
                        format!("无法连接到服务器\n\n原因: 连接失败\n\n建议:\n1. 请检查网络连接\n2. 请确认 API URL 正确: {}\n3. 请检查防火墙设置",
                            self.config.api_url)
                    )
                } else {
                    CliError::NetworkError(format!("网络请求失败: {}", e))
                }
            })?;

        let status = response.status();

        // 检查 HTTP 状态码
        if !status.is_success() {
            // 尝试解析错误响应
            let error_text = response.text().await.unwrap_or_else(|_| "无法读取错误响应".to_string());
            return self.handle_error_response(status.as_u16(), error_text);
        }

        // 解析成功响应
        let api_response: ApiResponse = response
            .json()
            .await
            .map_err(|e| CliError::NetworkError(format!("解析 API 响应失败: {}", e)))?;

        Ok(api_response.data)
    }

    /// 处理错误响应
    ///
    /// 将 API 错误代码映射到友好的中文错误消息
    fn handle_error_response(&self, code: u16, error_text: String) -> Result<ApiData> {
        // 尝试解析错误响应
        let error_resp: serde_json::Value = serde_json::from_str(&error_text)
            .unwrap_or_else(|_| serde_json::json!({"code": code, "msg": error_text}));

        let msg = error_resp["msg"]
            .as_str()
            .unwrap_or("未知错误")
            .to_string();

        // 根据错误代码提供友好的中文消息
        let (description, suggestion) = match code {
            400 => (
                "请求格式错误",
                "请检查请求格式，确保所有参数正确".to_string()
            ),
            401 => (
                "认证失败",
                "API 密钥无效或已过期\n\n建议:\n1. 请检查 API 密钥是否正确配置\n2. 请确认 API 密钥未过期\n3. 请访问智谱 AI 平台重新生成密钥".to_string()
            ),
            403 => (
                "无权限访问",
                "您的账户无权限访问此资源\n\n建议:\n请确认您的 API 密钥有访问权限\n如问题持续，请联系客服".to_string()
            ),
            404 => (
                "API 端点不存在",
                format!("API 端点不存在\n\n建议:\n请检查 API URL 配置是否正确: {}", self.config.api_url)
            ),
            429 => (
                "请求过于频繁",
                "请求过于频繁，已被限流\n\n建议:\n请稍后再试（建议等待 1-2 秒）".to_string()
            ),
            500 => (
                "服务器内部错误",
                "服务器内部错误\n\n建议:\n请稍后重试\n如问题持续，请联系支持团队".to_string()
            ),
            502 => (
                "网关错误",
                "网关错误\n\n建议:\n请稍后重试".to_string()
            ),
            503 => (
                "服务暂时不可用",
                "服务暂时不可用\n\n建议:\n请稍后重试".to_string()
            ),
            504 => (
                "网关超时",
                "网关超时\n\n建议:\n请稍后重试".to_string()
            ),
            _ => (
                "未知错误",
                "发生未知错误\n\n建议:\n请稍后重试\n如问题持续，请联系支持团队".to_string()
            ),
        };

        let full_msg = if !msg.is_empty() && msg != "未知错误" {
            format!("{}\n\n原因: {}", description, msg)
        } else {
            description.to_string()
        };

        let error_msg = format!("{}\n\n{}", full_msg, suggestion);

        Err(CliError::ApiError { code, msg: error_msg }.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            api_key: "test_api_key".to_string(),
            api_url: default_api_url(),
            timeout: 30,
        }
    }

    #[test]
    fn test_client_creation() {
        let config = create_test_config();
        let client = GlmClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_creation_with_timeout() {
        let config = Config {
            api_key: "test_api_key".to_string(),
            api_url: default_api_url(),
            timeout: 60,
        };

        let client = GlmClient::new(config);
        assert!(client.is_ok());
    }
}

fn default_api_url() -> String {
    "https://bigmodel.cn/api".to_string()
}
