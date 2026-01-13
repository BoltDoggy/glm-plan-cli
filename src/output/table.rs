//! 友好的使用情况输出
//!
//! 渲染更加用户友好的 API 使用情况显示。

use crate::api::{ApiData, LimitItem};
use chrono::{Utc, Timelike};

/// 渲染 API 使用情况信息
///
/// # 参数
///
/// * `data`: API 响应数据，包含所有限制项
///
/// # 返回
///
/// 格式化后的输出字符串
pub fn render_table(data: &ApiData) -> String {
    let mut output = String::new();

    // 遍历所有限制项
    for limit in &data.limits {
        output.push_str(&render_limit_item(limit));
        output.push_str("\n\n");
    }

    // 添加最近更新时间
    output.push_str(&format!("最近更新时间：{}", format_now()));

    output
}

/// 渲染单个限制项
fn render_limit_item(limit: &LimitItem) -> String {
    let title = match limit.limit_type.as_str() {
        "TIME_LIMIT" => "MCP每月额度",
        "TOKENS_LIMIT" => "每5小时使用限额",
        _ => &limit.limit_type,
    };

    let mut output = String::new();

    // 标题
    output.push_str(title);
    output.push_str("\n");

    // 百分比（显示在上方，不带 %，单独一行）
    let percentage = limit.percentage as u32;
    output.push_str(&format!("{}\n", percentage));

    // 进度条（在百分比下方）
    output.push_str(&render_progress_bar(percentage));
    output.push_str(" ");
    output.push_str(&format!("{}%", percentage));
    output.push_str("\n");

    // 已使用信息
    match limit.limit_type.as_str() {
        "TIME_LIMIT" => {
            // TIME_LIMIT 显示具体使用次数
            output.push_str(&format_number_with_used(
                limit.current_value,
                limit.usage,
            ));
            output.push_str("\n");
        }
        "TOKENS_LIMIT" => {
            // TOKENS_LIMIT 只显示"已使用"标签
            output.push_str("已使用\n");
        }
        _ => {
            output.push_str(&format_number(limit.current_value));
            output.push_str("\n");
        }
    }

    // 重置时间
    output.push_str(&render_reset_time(limit));

    output
}

/// 渲染百分比进度条
///
/// 使用 Unicode 字符创建一个类似这样的进度条：
/// ████████████░░░░░░░░ 68%
fn render_progress_bar(percentage: u32) -> String {
    const BAR_WIDTH: usize = 20;
    let filled = (percentage as usize * BAR_WIDTH / 100).min(BAR_WIDTH);
    let empty = BAR_WIDTH - filled;

    let filled_bar = "█".repeat(filled);
    let empty_bar = "░".repeat(empty);

    format!("{}{}", filled_bar, empty_bar)
}

/// 渲染重置时间
fn render_reset_time(limit: &LimitItem) -> String {
    match limit.limit_type.as_str() {
        "TIME_LIMIT" => {
            // MCP每月额度
            "重置时间：每月1号00:00重置\n".to_string()
        }
        "TOKENS_LIMIT" => {
            // 每5小时使用限额 - 计算下次重置时间
            if let Some(reset_ts) = limit.next_reset_time {
                let reset_time = chrono::DateTime::from_timestamp_millis(reset_ts);
                if let Some(dt) = reset_time {
                    return format!("重置时间：{:02}:{:02}\n",
                        dt.hour(), dt.minute());
                }
            }
            "重置时间：每5小时重置\n".to_string()
        }
        _ => String::new(),
    }
}

/// 格式化数字和已使用/总量
fn format_number_with_used(used: u64, total: u64) -> String {
    let used_str = format_number(used);
    let total_str = if total >= 1000000 {
        format!("{}M", total / 1000000)
    } else if total >= 1000 {
        format!("{}K", total / 1000)
    } else {
        format_number(total)
    };

    format!("{} / {} 次", used_str, total_str)
}

/// 格式化数字（添加千位分隔符）
fn format_number(num: u64) -> String {
    let num_str = num.to_string();
    let len = num_str.len();
    let mut result = String::new();

    for (i, c) in num_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    result
}

/// 格式化当前时间
fn format_now() -> String {
    let now = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_progress_bar() {
        assert_eq!(render_progress_bar(0), "░░░░░░░░░░░░░░░░░░░░");
        assert_eq!(render_progress_bar(50), "██████████░░░░░░░░░░");
        assert_eq!(render_progress_bar(100), "████████████████████");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(1000000), "1,000,000");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(0), "0");
    }

    #[test]
    fn test_format_number_with_used() {
        assert_eq!(format_number_with_used(164, 1000), "164 / 1K 次");
        assert_eq!(format_number_with_used(500, 500), "500 / 500 次");
        assert_eq!(format_number_with_used(1500000, 2000000), "1,500,000 / 2M 次");
    }

    #[test]
    fn test_render_limit_item_time_limit() {
        let limit = LimitItem {
            limit_type: "TIME_LIMIT".to_string(),
            unit: 5,
            number: 1,
            usage: 1000,
            current_value: 164,
            remaining: 836,
            percentage: 16.0,
            next_reset_time: None,
        };

        let output = render_limit_item(&limit);
        assert!(output.contains("MCP每月额度"));
        assert!(output.contains("16%"));
        assert!(output.contains("164 / 1K 次"));
        assert!(output.contains("每月1号00:00重置"));
    }

    #[test]
    fn test_render_limit_item_tokens_limit() {
        let limit = LimitItem {
            limit_type: "TOKENS_LIMIT".to_string(),
            unit: 3,
            number: 5,
            usage: 200000000,
            current_value: 132374032,
            remaining: 67625968,
            percentage: 66.0,
            next_reset_time: Some(1768328328345),
        };

        let output = render_limit_item(&limit);
        assert!(output.contains("每5小时使用限额"));
        assert!(output.contains("66%"));
        assert!(output.contains("已使用"));
        // 验证包含重置时间格式 (HH:MM)
        assert!(output.contains("重置时间："));
        assert!(output.contains(':')); // 时间格式包含冒号
    }
}
