# 快速入门指南：GLM API 计划查询命令

**功能**: GLM API 计划查询命令
**日期**: 2026-01-13
**目标用户**: GLM API 用户

## 概述

GLM CLI 是一个命令行工具，用于快速查询智谱 AI GLM API 的计划使用情况。本文档帮助您在 5 分钟内完成安装、配置和首次查询。

---

## 前置条件

### 系统要求

- **操作系统**: Linux, macOS, 或 Windows
- **网络**: 能够访问 `https://open.bigmodel.cn`
- **API 密钥**: 有效的智谱 AI GLM API 密钥

### 获取 API 密钥

1. 访问智谱 AI 开放平台: https://open.bigmodel.cn
2. 注册/登录账户
3. 进入"API 密钥"管理页面
4. 创建新的 API 密钥
5. **重要**: 复制并妥善保存您的 API 密钥（格式类似 `sk.xxxxxxxxxxxxxx`）

---

## 安装

### 方式 1: 使用预编译二进制文件（推荐）

#### Linux/macOS

```bash
# 下载最新版本
curl -L https://github.com/your-org/glm-plan-cli/releases/latest/download/glm-cli-x86_64-unknown-linux-gnu -o glm

# 或 macOS
curl -L https://github.com/your-org/glm-plan-cli/releases/latest/download/glm-cli-x86_64-apple-darwin -o glm

# 添加执行权限
chmod +x glm

# 移动到 PATH
sudo mv glm /usr/local/bin/

# 验证安装
glm --version
```

#### Windows (PowerShell)

```powershell
# 下载最新版本
curl -L https://github.com/your-org/glm-plan-cli/releases/latest/download/glm-cli-x86_64-pc-windows-msvc.exe -o glm.exe

# 添加到 PATH（手动操作或使用以下命令）
# 将 glm.exe 移动到 PATH 中的目录，如 C:\Program Files\glm\

# 验证安装
.\glm.exe --version
```

### 方式 2: 从源码编译

```bash
# 克隆仓库
git clone https://github.com/your-org/glm-plan-cli.git
cd glm-plan-cli

# 编译（需要 Rust 工具链）
cargo build --release

# 二进制文件位于 target/release/glm
# 将其移动到 PATH
sudo cp target/release/glm /usr/local/bin/

# 验证安装
glm --version
```

---

## 配置

### 方式 1: 使用环境变量（推荐用于临时使用）

#### Linux/macOS (Bash/Zsh)

```bash
# 临时设置（当前会话有效）
export GLM_API_KEY="sk.xxxxxxxxxxxxxx"

# 永久设置（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export GLM_API_KEY="sk.xxxxxxxxxxxxxx"' >> ~/.bashrc
source ~/.bashrc
```

#### Windows (PowerShell)

```powershell
# 临时设置（当前会话有效）
$env:GLM_API_KEY="sk.xxxxxxxxxxxxxx"

# 永久设置
[System.Environment]::SetEnvironmentVariable('GLM_API_KEY', 'sk.xxxxxxxxxxxxxx', 'User')
```

### 方式 2: 使用配置文件（推荐用于长期使用）

#### 创建配置文件

```bash
# 创建配置目录
mkdir -p ~/.glm

# 创建配置文件
cat > ~/.glm/config.yaml << EOF
# GLM API 配置文件

# API 密钥（必填）
api_key: sk.xxxxxxxxxxxxxx

# API 端点 URL（可选，有默认值）
# api_url: https://open.bigmodel.cn/api/paas/v4

# 请求超时时间（秒，可选，默认 30）
# timeout: 30
EOF

# 设置安全的文件权限（仅用户可读写）
chmod 600 ~/.glm/config.yaml
```

#### 使用示例配置文件

```bash
# 复制示例配置文件
cp glm-config.example.yaml ~/.glm/config.yaml

# 编辑配置文件，填入您的 API 密钥
nano ~/.glm/config.yaml  # 或使用您喜欢的编辑器
```

### 配置优先级

当同时存在多个配置源时，优先级如下：

1. **环境变量** `GLM_API_KEY`（最高优先级）
2. **配置文件** `~/.glm/config.yaml`
3. **默认值**（仅用于 API URL 和超时设置）

---

## 使用

### 基本用法

#### 查询 API 计划信息（表格格式，默认）

```bash
glm
```

**输出示例**:

```
┌────────────────┬──────────────────┐
│ 项目           │ 值               │
├────────────────┼──────────────────┤
│ 计划名称       │ 高级版           │
├────────────────┼──────────────────┤
│ 总额度         │ 1,000,000 tokens │
├────────────────┼──────────────────┤
│ 已使用         │ 250,000 tokens   │
├────────────────┼──────────────────┤
│ 剩余           │ 750,000 tokens   │
├────────────────┼──────────────────┤
│ 使用进度       │ 25.0%            │
├────────────────┼──────────────────┤
│ 开始日期       │ 2026-01-01       │
├────────────────┼──────────────────┤
│ 结束日期       │ 2026-12-31       │
└────────────────┴──────────────────┘
```

### 高级用法

#### JSON 格式输出

```bash
glm --format json
```

**输出示例**:

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

#### 查看帮助信息

```bash
glm --help
```

**输出示例**:

```
GLM API 计划查询工具 v1.0.0

用法:
  glm [选项]

选项:
  -f, --format <格式>  输出格式 [默认: table] [可选值: table, json]
  -h, --help          显示帮助信息
  -V, --version       显示版本号

配置:
  API 密钥可以通过以下方式配置（优先级从高到低）:
  1. 环境变量 GLM_API_KEY
  2. 配置文件 ~/.glm/config.yaml

示例:
  glm                    # 查询计划信息（表格格式）
  glm --format json      # 查询计划信息（JSON 格式）
  glm -f table           # 等同于 glm（表格格式）

环境变量:
  GLM_API_KEY            API 密钥
  GLM_API_URL            API 端点 URL（可选）
  GLM_TIMEOUT            请求超时时间（秒，可选）

更多信息:
  https://github.com/your-org/glm-plan-cli
```

#### 查看版本号

```bash
glm --version
```

**输出示例**:

```
glm 1.0.0
```

---

## 常见问题

### 问题 1: 提示"未找到 API 密钥配置"

**原因**: 未配置 API 密钥或配置无效

**解决方案**:

1. 确认已设置环境变量:
   ```bash
   echo $GLM_API_KEY
   ```
   应显示您的 API 密钥

2. 或确认配置文件存在:
   ```bash
   cat ~/.glm/config.yaml
   ```
   应包含 `api_key: sk.xxxxxxxxxxxxxx`

3. 重新配置（参考"配置"章节）

### 问题 2: 提示"认证失败，API 密钥无效"

**原因**: API 密钥错误或已过期

**解决方案**:

1. 检查 API 密钥是否正确（以 `sk.` 开头）
2. 确认 API 密钥未过期（登录智谱 AI 平台检查）
3. 重新生成 API 密钥并更新配置

### 问题 3: 提示"网络连接失败"

**原因**: 无法访问智谱 AI API 服务器

**解决方案**:

1. 检查网络连接:
   ```bash
   ping open.bigmodel.cn
   ```

2. 检查防火墙设置

3. 如果使用代理，配置代理环境变量:
   ```bash
   export HTTP_PROXY=http://proxy.example.com:8080
   export HTTPS_PROXY=http://proxy.example.com:8080
   ```

### 问题 4: 提示"API 请求超时"

**原因**: 服务器响应时间过长

**解决方案**:

1. 稍后重试
2. 增加超时时间（在配置文件中设置）:
   ```yaml
   timeout: 60  # 增加到 60 秒
   ```

### 问题 5: 表格显示乱码

**原因**: 终端不支持 UTF-8 字符

**解决方案**:

1. 使用 JSON 格式输出:
   ```bash
   glm --format json
   ```

2. 或设置终端编码为 UTF-8:
   ```bash
   export LANG=en_US.UTF-8
   ```

---

## 进阶使用

### 在脚本中使用

```bash
#!/bin/bash
# 检查 API 使用情况，如果超过 80% 则告警

USAGE=$(glm --format json | jq '.usage_percentage')
THRESHOLD=80

if (( $(echo "$USAGE > $THRESHOLD" | bc -l) )); then
    echo "警告：API 使用率已达到 ${USAGE}%"
    # 发送告警通知...
fi
```

### 定期检查（使用 cron）

```bash
# 编辑 crontab
crontab -e

# 每天上午 9 点检查一次
0 9 * * * /usr/local/bin/glm --format json > /var/log/glm-usage.log
```

### 与其他工具集成

```bash
# 使用 jq 处理 JSON 输出
glm --format json | jq '.remaining_quota'

# 输出: 750000

# 检查剩余额度是否足够
glm --format json | jq '.remaining_quota > 100000'

# 输出: true
```

---

## 卸载

### Linux/macOS

```bash
# 删除二进制文件
sudo rm /usr/local/bin/glm

# 删除配置文件（可选）
rm -rf ~/.glm

# 删除环境变量配置（如果已添加到 ~/.bashrc 或 ~/.zshrc）
# 手动编辑文件并删除相关行
```

### Windows

```powershell
# 删除二进制文件
Remove-Item "C:\Program Files\glm\glm.exe"

# 删除配置文件（可选）
Remove-Item -Recurse -Force "$env:USERPROFILE\.glm"

# 删除环境变量（如果已设置）
[System.Environment]::SetEnvironmentVariable('GLM_API_KEY', $null, 'User')
```

---

## 获取帮助

### 命令行帮助

```bash
glm --help
```

### 在线资源

- **GitHub 仓库**: https://github.com/your-org/glm-plan-cli
- **问题反馈**: https://github.com/your-org/glm-plan-cli/issues
- **智谱 AI 开放平台**: https://open.bigmodel.cn

### 联系支持

如果遇到问题：

1. 查看本文档的"常见问题"章节
2. 搜索或提交 GitHub Issue
3. 联系智谱 AI 技术支持

---

## 下一步

现在您已经完成了 GLM CLI 的安装和配置，可以：

1. ✅ 查询您的 API 计划信息
2. ✅ 监控使用进度
3. ✅ 集成到自动化脚本
4. ✅ 定期检查避免超限

祝您使用愉快！🎉

---

**快速入门指南完成日期**: 2026-01-13
**版本**: 1.0.0
**状态**: ✅ 完成
