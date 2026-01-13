//! API 数据类型定义
//!
//! 定义与 GLM API 交互时使用的所有数据结构。

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 单个额度限制项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LimitItem {
    /// 限制类型（TIME_LIMIT 或 TOKENS_LIMIT）
    #[serde(rename = "type")]
    pub limit_type: String,

    /// 单位
    pub unit: u64,

    /// 数量
    pub number: u64,

    /// 总额度
    pub usage: u64,

    /// 当前已使用量
    #[serde(rename = "currentValue")]
    pub current_value: u64,

    /// 剩余量
    pub remaining: u64,

    /// 使用百分比
    pub percentage: f64,

    /// 下次重置时间（仅 TOKENS_LIMIT 有此字段）
    #[serde(rename = "nextResetTime", skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<i64>,
}

/// API 响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiData {
    /// 额度限制列表
    pub limits: Vec<LimitItem>,
}

/// API 成功响应
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    /// HTTP 状态码（200 表示成功）
    pub code: u16,

    /// 响应消息
    pub msg: String,

    /// 响应数据
    pub data: ApiData,

    /// 是否成功
    pub success: bool,
}

/// 显示用的 API 计划信息（从 TOKENS_LIMIT 提取）
#[derive(Debug, Clone)]
pub struct ApiPlan {
    /// 限制类型
    pub plan_name: String,

    /// 总额度（token 数）
    pub total_quota: u64,

    /// 已使用量
    pub used_quota: u64,

    /// 剩余量
    pub remaining_quota: u64,

    /// 使用百分比（0-100）
    pub usage_percentage: f64,

    /// 下次重置时间（可选）
    pub next_reset_time: Option<DateTime<Utc>>,
}

impl ApiPlan {
    /// 从 LimitItem 创建 ApiPlan
    pub fn from_limit_item(item: &LimitItem) -> Self {
        ApiPlan {
            plan_name: match item.limit_type.as_str() {
                "TOKENS_LIMIT" => "Token 额度".to_string(),
                "TIME_LIMIT" => "时间限制".to_string(),
                _ => item.limit_type.clone(),
            },
            total_quota: item.usage,
            used_quota: item.current_value,
            remaining_quota: item.remaining,
            usage_percentage: item.percentage,
            next_reset_time: item.next_reset_time.and_then(|ts| DateTime::from_timestamp_millis(ts)),
        }
    }

    /// 验证 ApiPlan 数据的完整性
    pub fn validate(&self) -> Result<(), String> {
        if self.total_quota == 0 {
            return Err("总额度必须大于 0".to_string());
        }

        if self.used_quota > self.total_quota {
            return Err(format!(
                "已使用量 ({}) 不能超过总额度 ({})",
                self.used_quota, self.total_quota
            ));
        }

        if self.usage_percentage < 0.0 || self.usage_percentage > 100.0 {
            return Err(format!(
                "使用百分比必须在 0-100 之间，当前: {}",
                self.usage_percentage
            ));
        }

        Ok(())
    }
}

/// API 错误响应
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    /// 错误代码
    pub code: u16,

    /// 错误消息
    pub msg: String,

    /// 错误类型
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_limit_item() -> LimitItem {
        LimitItem {
            limit_type: "TOKENS_LIMIT".to_string(),
            unit: 3,
            number: 5,
            usage: 200000000,
            current_value: 132374032,
            remaining: 67625968,
            percentage: 66.0,
            next_reset_time: Some(1768328328345),
        }
    }

    #[test]
    fn test_api_plan_from_limit_item() {
        let item = create_test_limit_item();
        let plan = ApiPlan::from_limit_item(&item);

        assert_eq!(plan.plan_name, "Token 额度");
        assert_eq!(plan.total_quota, 200000000);
        assert_eq!(plan.used_quota, 132374032);
        assert_eq!(plan.remaining_quota, 67625968);
        assert_eq!(plan.usage_percentage, 66.0);
        assert!(plan.next_reset_time.is_some());
    }

    #[test]
    fn test_api_plan_validate_success() {
        let item = create_test_limit_item();
        let plan = ApiPlan::from_limit_item(&item);
        assert!(plan.validate().is_ok());
    }

    #[test]
    fn test_api_plan_validate_zero_total() {
        let mut item = create_test_limit_item();
        item.usage = 0;
        let plan = ApiPlan::from_limit_item(&item);
        assert!(plan.validate().is_err());
    }

    #[test]
    fn test_api_plan_validate_exceeds_total() {
        let mut item = create_test_limit_item();
        item.current_value = 300000000;
        let plan = ApiPlan::from_limit_item(&item);
        assert!(plan.validate().is_err());
    }

    #[test]
    fn test_api_plan_validate_invalid_percentage() {
        let mut item = create_test_limit_item();
        item.percentage = 150.0;
        let plan = ApiPlan::from_limit_item(&item);
        assert!(plan.validate().is_err());
    }

    #[test]
    fn test_api_plan_time_limit() {
        let item = LimitItem {
            limit_type: "TIME_LIMIT".to_string(),
            unit: 5,
            number: 1,
            usage: 1000,
            current_value: 164,
            remaining: 836,
            percentage: 16.0,
            next_reset_time: None,
        };
        let plan = ApiPlan::from_limit_item(&item);
        assert_eq!(plan.plan_name, "时间限制");
        assert!(plan.next_reset_time.is_none());
    }
}
