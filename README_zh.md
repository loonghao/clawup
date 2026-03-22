# 🦀 clawup

> 就像 `rustup`，但用于 [OpenClaw](https://openclaw.ai/)。一个 Rust CLI 工具，用于管理 OpenClaw 的配置、Agents、Skills、Soul 和 Profiles。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 功能特性

- **声明式配置** — 在单个 `clawup.toml` 中定义整个 OpenClaw 设置
- **多 Agent 管理** — 配置多个 Agent，支持独立工作区、身份和模型覆盖
- **Skills 管理** — 安装、启用/禁用和配置内置及社区 Skills
- **Soul 模板** — 使用 Tera 模板管理 SOUL.md、IDENTITY.md 等 Soul 文件
- **Profile 切换** — 在 `dev`、`staging`、`production` 配置之间即时切换
- **团队同步** — 通过 Git 共享配置，确保团队一致性
- **健康检查** — `clawup doctor` 验证整个 OpenClaw 安装状态

## 快速开始

```bash
# 初始化新的 clawup 配置
clawup init

# 应用配置到 OpenClaw
clawup apply

# 切换 Profile
clawup profile switch production

# 检查安装健康状态
clawup doctor
```

## 安装

### 一键安装（推荐）

**Linux / macOS：**

```bash
curl -fsSL https://raw.githubusercontent.com/loonghao/clawup/main/install.sh | sh
```

**Windows (PowerShell)：**

```powershell
irm https://raw.githubusercontent.com/loonghao/clawup/main/install.ps1 | iex
```

### 安装选项

| 环境变量 | 描述 | 默认值 |
|---------|------|-------|
| `CLAWUP_VERSION` | 指定安装版本 | `latest` |
| `CLAWUP_INSTALL` | 安装目录 | `~/.clawup/bin` |
| `CLAWUP_MUSL` | 在 Linux 上优先使用 musl 构建 | 自动检测 |
| `CLAWUP_NO_PATH` | 跳过自动添加到 PATH（Windows） | `0` |

使用示例：

```bash
# 安装指定版本
CLAWUP_VERSION=0.1.6 curl -fsSL https://raw.githubusercontent.com/loonghao/clawup/main/install.sh | sh

# 安装到自定义目录
CLAWUP_INSTALL=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/loonghao/clawup/main/install.sh | sh
```

### 从源码安装

```bash
cargo install clawup
```

### 预构建二进制文件

从 [GitHub Releases](https://github.com/loonghao/clawup/releases) 下载预构建的二进制文件。

| 平台 | 架构 | 目标 |
|------|-----|------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Linux | x86_64 (静态) | `x86_64-unknown-linux-musl` |
| Linux | ARM64 | `aarch64-unknown-linux-gnu` |
| Linux | ARM64 (静态) | `aarch64-unknown-linux-musl` |
| macOS | Intel | `x86_64-apple-darwin` |
| macOS | Apple Silicon | `aarch64-apple-darwin` |
| Windows | x86_64 | `x86_64-pc-windows-msvc` |
| Windows | ARM64 | `aarch64-pc-windows-msvc` |

## 配置

`clawup` 使用声明式 TOML 配置文件（`clawup.toml`）：

```toml
[meta]
schema_version = "1"
description = "我的 OpenClaw 设置"

[gateway]
provider = "openrouter"
model = "anthropic/claude-sonnet-4"

[agents.defaults]
model = "anthropic/claude-sonnet-4"
approval_mode = "auto-edit"

[[agents.list]]
name = "code"
role = "高级软件工程师"
instructions = "专注于干净、经过测试的代码"

[[agents.list]]
name = "review"
role = "代码审查员"
instructions = "全面审查 PR"

[skills.bundled]
enabled = ["developer", "computer"]

[[skills.entries]]
name = "my-custom-skill"
source = "~/.openclaw/skills/my-custom-skill"
```

## CLI 命令

| 命令 | 描述 |
|------|------|
| `clawup init` | 初始化新配置 |
| `clawup apply` | 应用配置到 OpenClaw |
| `clawup sync` | 与 Git 远程同步配置 |
| `clawup diff` | 显示本地与已应用配置的差异 |
| `clawup doctor` | OpenClaw 安装健康检查 |
| `clawup agent list` | 列出已配置的 Agents |
| `clawup agent add` | 添加新 Agent |
| `clawup skill list` | 列出已安装的 Skills |
| `clawup skill add` | 安装新 Skill |
| `clawup soul show` | 显示 Soul 文件 |
| `clawup profile switch` | 切换活动 Profile |
| `clawup config show` | 显示当前配置 |

## 开发

```bash
# 构建
vx just build

# 测试
vx just test

# 代码检查
vx just lint

# 运行
vx just run --help
```

## 许可证

MIT © [Hal](https://github.com/loonghao)
