//! 错误类型定义
//!
//! 使用 thiserror 定义所有可能的错误类型，
//! 提供清晰的中文错误消息。

use thiserror::Error;

/// CLI 错误类型
#[derive(Error, Debug)]
pub enum CliError {
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 网络错误
    #[error("网络错误: {0}")]
    NetworkError(String),

    /// API 错误
    #[error("API 错误 (代码 {code}): {msg}")]
    ApiError { code: u16, msg: String },

    /// 数据验证错误
    #[error("数据验证错误: {0}")]
    ValidationError(String),

    /// IO 错误
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// YAML 解析错误
    #[error("YAML 解析错误: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// HTTP 请求错误
    #[error("HTTP 请求错误: {0}")]
    HttpError(#[from] reqwest::Error),
}

/// API 错误响应类型
#[derive(Debug, Clone)]
pub struct ApiErrorResponse {
    pub code: u16,
    pub msg: String,
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CliError::ConfigError("测试错误".to_string());
        assert_eq!(format!("{}", err), "配置错误: 测试错误");
    }

    #[test]
    fn test_api_error() {
        let err = CliError::ApiError {
            code: 401,
            msg: "未授权".to_string(),
        };
        assert!(format!("{}", err).contains("401"));
        assert!(format!("{}", err).contains("未授权"));
    }
}
