//! GLM API 计划查询命令行工具
//!
//! 用于查询智谱 AI GLM API 的计划使用情况。

use anyhow::Result;
use clap::Parser;
use glm::api::GlmClient;
use glm::config::Config;
use glm::output::render_table;

/// GLM API 计划查询工具
#[derive(Parser, Debug)]
#[command(name = "glm")]
#[command(about = "GLM API 计划查询工具", long_about = None)]
#[command(version)]
struct Args {
    /// 输出格式
    #[arg(short = 'f', long = "format", default_value = "table")]
    format: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    // 加载配置
    let config = Config::load().map_err(|e| anyhow::anyhow!("配置加载失败: {}", e))?;

    // 验证配置
    config
        .validate()
        .map_err(|e| anyhow::anyhow!("配置验证失败: {}", e))?;

    // 创建 API 客户端
    let client = GlmClient::new(config.clone())
        .map_err(|e| anyhow::anyhow!("创建 API 客户端失败: {}", e))?;

    // 获取 API 使用情况
    let usage_data = client
        .fetch_usage()
        .await
        .map_err(|e| anyhow::anyhow!("获取使用情况失败: {}", e))?;

    // 格式化输出
    let output = render_table(&usage_data);
    println!("{}", output);

    Ok(())
}
