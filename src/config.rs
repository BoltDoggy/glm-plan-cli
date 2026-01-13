//! 配置管理
//!
//! 支持从环境变量和配置文件加载配置，
//! 环境变量优先级高于配置文件。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

/// 用户配置
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    "https://bigmodel.cn/api".to_string()
}

fn default_timeout() -> u64 {
    30
}

impl Config {
    /// 从环境变量和配置文件加载配置
    ///
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
            .ok_or_else(||
                anyhow::anyhow!("无法确定主目录")
            )?
            .join(".glm/config.yaml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_yaml::from_str(&content)?;
            return Ok(config);
        }

        // 3. 未找到配置
        Err(anyhow::anyhow!(
            "未找到 API 密钥配置。\n\n请选择以下方式之一配置：\n\
             1. 设置环境变量: export GLM_API_KEY=\"sk.xxxxxxxxxxxxxx\"\n\
             2. 创建配置文件: ~/.glm/config.yaml\n\n\
             配置文件示例:\n\
             api_key: sk.xxxxxxxxxxxxxx\n\
             api_url: https://open.bigmodel.cn/api/paas/v4\n\
             timeout: 30"
        ))
    }

    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // 验证 API 密钥格式
        if self.api_key.len() < 10 {
            return Err(anyhow::anyhow!(
                "API 密钥长度不足\n\n原因: API 密钥长度必须至少 10 个字符\n\n建议: 请确认您的 API 密钥完整"
            ));
        }

        // 验证 URL 格式
        if !self.api_url.starts_with("https://") {
            return Err(anyhow::anyhow!(
                "API URL 必须使用 HTTPS\n\n原因: 当前 URL: {}\n\n建议: 请使用 HTTPS 协议的 URL",
                self.api_url
            ));
        }

        // 验证超时范围
        if self.timeout == 0 || self.timeout > 300 {
            return Err(anyhow::anyhow!(
                "超时时间必须在 1-300 秒之间\n\n原因: 当前设置: {} 秒\n\n建议: 设置为 30 秒",
                self.timeout
            ));
        }

        Ok(())
    }

    /// 获取配置文件路径
    pub fn config_file_path() -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("无法确定主目录"))?
            .join(".glm/config.yaml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_api_url() {
        let url = default_api_url();
        assert_eq!(url, "https://bigmodel.cn/api");
    }

    #[test]
    fn test_default_timeout() {
        let timeout = default_timeout();
        assert_eq!(timeout, 30);
    }

    #[test]
    fn test_config_validate_short_key() {
        let config = Config {
            api_key: "short".to_string(),
            api_url: default_api_url(),
            timeout: default_timeout(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_valid_key() {
        let config = Config {
            api_key: "my_valid_api_key_12345".to_string(),
            api_url: default_api_url(),
            timeout: 30,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_invalid_url() {
        let config = Config {
            api_key: "valid_api_key".to_string(),
            api_url: "http://insecure.com".to_string(),
            timeout: default_timeout(),
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_invalid_timeout() {
        let config = Config {
            api_key: "valid_api_key".to_string(),
            api_url: default_api_url(),
            timeout: 0,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_timeout_too_large() {
        let config = Config {
            api_key: "valid_api_key".to_string(),
            api_url: default_api_url(),
            timeout: 400,
        };
        assert!(config.validate().is_err());
    }
}
