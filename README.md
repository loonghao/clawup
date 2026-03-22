# 🦀 clawup

> Like `rustup`, but for [OpenClaw](https://openclaw.ai/). A Rust CLI tool for managing OpenClaw configurations, agents, skills, souls, and profiles.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Declarative Configuration** — Define your entire OpenClaw setup in a single `clawup.toml`
- **Multi-Agent Management** — Configure multiple agents with independent workspaces, identities, and model overrides
- **Skills Management** — Install, enable/disable, and configure bundled & community skills
- **Soul Templating** — Manage SOUL.md, IDENTITY.md, and other soul files with Tera templates
- **Profile Switching** — Instantly switch between `dev`, `staging`, `production` configurations
- **Team Sync** — Share configurations via Git for consistent team-wide setups
- **Health Checks** — `clawup doctor` validates your entire OpenClaw installation

## Quick Start

```bash
# Initialize a new clawup configuration
clawup init

# Apply configuration to OpenClaw
clawup apply

# Switch between profiles
clawup profile switch production

# Check installation health
clawup doctor
```

## Installation

### Quick Install (Recommended)

**Linux / macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/loonghao/clawup/main/install.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/loonghao/clawup/main/install.ps1 | iex
```

### Install Options

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `CLAWUP_VERSION` | Specific version to install | `latest` |
| `CLAWUP_INSTALL` | Installation directory | `~/.clawup/bin` |
| `CLAWUP_MUSL` | Prefer musl build on Linux | auto-detect |
| `CLAWUP_NO_PATH` | Skip auto-adding to PATH (Windows) | `0` |

Example with options:

```bash
# Install specific version
CLAWUP_VERSION=0.1.9 curl -fsSL https://raw.githubusercontent.com/loonghao/clawup/main/install.sh | sh

# Install to custom directory
CLAWUP_INSTALL=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/loonghao/clawup/main/install.sh | sh
```

### From Source

```bash
cargo install clawup
```

### Pre-built Binaries

Download pre-built binaries from [GitHub Releases](https://github.com/loonghao/clawup/releases).

| Platform | Architecture | Target |
|----------|-------------|--------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Linux | x86_64 (static) | `x86_64-unknown-linux-musl` |
| Linux | ARM64 | `aarch64-unknown-linux-gnu` |
| Linux | ARM64 (static) | `aarch64-unknown-linux-musl` |
| macOS | Intel | `x86_64-apple-darwin` |
| macOS | Apple Silicon | `aarch64-apple-darwin` |
| Windows | x86_64 | `x86_64-pc-windows-msvc` |
| Windows | ARM64 | `aarch64-pc-windows-msvc` |

## Configuration

`clawup` uses a declarative TOML configuration file (`clawup.toml`):

```toml
[meta]
schema_version = "1"
description = "My OpenClaw setup"

[gateway]
provider = "openrouter"
model = "anthropic/claude-sonnet-4"

[agents.defaults]
model = "anthropic/claude-sonnet-4"
approval_mode = "auto-edit"

[[agents.list]]
name = "code"
role = "Senior Software Engineer"
instructions = "Focus on clean, tested code"

[[agents.list]]
name = "review"
role = "Code Reviewer"
instructions = "Review PRs thoroughly"

[skills.bundled]
enabled = ["developer", "computer"]

[[skills.entries]]
name = "my-custom-skill"
source = "~/.openclaw/skills/my-custom-skill"
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `clawup init` | Initialize a new configuration |
| `clawup apply` | Apply configuration to OpenClaw |
| `clawup sync` | Sync configuration with Git remote |
| `clawup diff` | Show differences between local and applied |
| `clawup doctor` | Health check for OpenClaw installation |
| `clawup agent list` | List configured agents |
| `clawup agent add` | Add a new agent |
| `clawup skill list` | List installed skills |
| `clawup skill add` | Install a new skill |
| `clawup soul show` | Display soul files |
| `clawup profile switch` | Switch active profile |
| `clawup config show` | Show current configuration |

## Development

```bash
# Build
vx just build

# Test
vx just test

# Lint
vx just lint

# Run
vx just run --help
```

## License

MIT © [Hal](https://github.com/loonghao)
