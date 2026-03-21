# 🦀 clawup

> Like `rustup`, but for [OpenClaw](https://github.com/claw-project/openclaw). A Rust CLI tool for managing OpenClaw configurations, agents, skills, souls, and profiles.

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

```bash
cargo install clawup
```

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
just build

# Test
just test

# Lint
just lint

# Run
just run --help
```

## License

MIT © [Hal](https://github.com/loonghao)
