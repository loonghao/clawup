# AGENTS.md — AI Agent Guide for clawup

> This document is intended for AI coding agents (Claude Code, Copilot, CodeBuddy, Cursor, etc.)
> working on the **clawup** codebase. It provides architecture context, conventions, and
> operational guidance to help you contribute effectively.

## Project Overview

**clawup** is a Rust CLI tool — like `rustup`, but for [OpenClaw](https://openclaw.ai/).
It manages OpenClaw configurations, agents, skills, souls, and profiles through a declarative
`clawup.toml` file.

- **Repository**: <https://github.com/loonghao/clawup>
- **OpenClaw website**: <https://openclaw.ai/>
- **Language**: Rust (edition 2024)
- **License**: MIT

## Architecture

The project uses a **Rust workspace with a 4-layer architecture**:

```
┌─────────────────────────────────────────────────────┐
│  clawup (CLI binary)                                │  Layer 4 — User-facing CLI
│  clap / dialoguer / indicatif / color-eyre          │
├─────────────────────────────────────────────────────┤
│  clawup-ops (business operations)                   │  Layer 3 — High-level logic
│  agent / skill / soul / profile / diff / template   │
├─────────────────────────────────────────────────────┤
│  clawup-core (core infrastructure)                  │  Layer 2 — Foundation
│  ManifestOps trait / OpenClawPaths / fs / error     │
├─────────────────────────────────────────────────────┤
│  clawup-schema (pure data types)                    │  Layer 1 — Zero-logic types
│  Manifest / AgentDefinition / SkillEntry / ...      │
└─────────────────────────────────────────────────────┘
```

**Dependency direction** (strict, no cycles):
`clawup` → `clawup-ops` → `clawup-core` → `clawup-schema`

### Crate Responsibilities

| Crate | Path | Purpose |
|-------|------|---------|
| `clawup-schema` | `crates/clawup-schema/` | Pure data types (`Manifest`, `AgentDefinition`, etc.). Only `serde` dependency. No logic. |
| `clawup-core` | `crates/clawup-core/` | Core infrastructure: `ManifestOps` trait (CRUD), `OpenClawPaths` (path detection), filesystem helpers, `CoreError`. |
| `clawup-ops` | `crates/clawup-ops/` | Business operations: agent workspace scaffolding, skill install/uninstall, soul template rendering (Tera), profile switching, diff, template generation. |
| `clawup` | `crates/clawup/` | CLI binary: clap command definitions, TUI interactions (dialoguer/indicatif), error reporting (color-eyre). |

### Key Types

| Type | Crate | Description |
|------|-------|-------------|
| `Manifest` | clawup-schema | Root struct for `clawup.toml` |
| `AgentDefinition` | clawup-schema | Agent with name, role, instructions, model, approval_mode, etc. |
| `SkillEntry` | clawup-schema | Skill with name, source, enabled flag, config |
| `ManifestOps` | clawup-core | Extension trait adding load/save/CRUD to `Manifest` |
| `OpenClawPaths` | clawup-core | Detects OpenClaw install location (`$OPENCLAW_HOME` or `~/.openclaw/`) |
| `OpsError` | clawup-ops | Error type covering git2, tera, walkdir errors |
| `CoreError` | clawup-core | Error type for core operations |

### Design Patterns

- **Extension Trait Pattern**: `ManifestOps` is defined in `clawup-core` as a trait implemented for `Manifest` (from `clawup-schema`). This avoids orphan rule violations while keeping schema types logic-free.
- **Error Layering**: Each crate has its own error type (`CoreError`, `OpsError`, `ClawupError`) with `From` conversions flowing upward.
- **Template Strategy**: Built-in templates (`default`, `multi-agent`, `team`) are generated programmatically in `clawup-ops::template`, not read from files at runtime.

## Directory Layout

```
crates/
├── clawup-schema/
│   └── src/
│       ├── lib.rs          # mod manifest; pub use manifest::*
│       └── manifest.rs     # All clawup.toml struct definitions
│
├── clawup-core/
│   └── src/
│       ├── lib.rs          # Re-exports
│       ├── error.rs        # CoreError
│       ├── fs.rs           # ensure_dir, read_optional, expand_path
│       ├── manifest.rs     # ManifestOps trait + impl
│       └── paths.rs        # OpenClawPaths
│
├── clawup-ops/
│   └── src/
│       ├── lib.rs          # Re-exports
│       ├── error.rs        # OpsError
│       ├── agent.rs        # scaffold_workspace, generate_soul, generate_identity
│       ├── skill.rs        # install, uninstall, copy_skill_files, is_installed
│       ├── soul.rs         # read/write/list soul files, render_template (Tera)
│       ├── profile.rs      # current_profile, set_current_profile, clear_profile
│       ├── template.rs     # from_template (default/multi-agent/team)
│       └── diff.rs         # compute_diff, unified_diff, diff_files
│
└── clawup/
    ├── src/
    │   ├── main.rs         # Entry point (clap + color-eyre + tracing)
    │   ├── error.rs        # ClawupError
    │   ├── cli/
    │   │   ├── mod.rs      # Cli struct + Commands enum + dispatch()
    │   │   ├── init.rs     # clawup init
    │   │   ├── apply.rs    # clawup apply
    │   │   ├── doctor.rs   # clawup doctor
    │   │   ├── agent.rs    # clawup agent {list|add|remove|show|set}
    │   │   ├── skill.rs    # clawup skill {list|add|remove|enable|disable|update}
    │   │   ├── soul.rs     # clawup soul {show|edit|diff|from-template}
    │   │   ├── profile.rs  # clawup profile {list|switch|create}
    │   │   ├── config.rs   # clawup config {get|set|show|merge}
    │   │   ├── sync.rs     # clawup sync [stub]
    │   │   ├── diff.rs     # clawup diff [stub]
    │   │   └── export.rs   # clawup export [stub]
    │   ├── manifest/       # (Legacy — to be replaced by clawup-core)
    │   ├── openclaw/       # (Legacy — to be replaced by clawup-core)
    │   ├── agent/          # (Legacy — to be replaced by clawup-ops)
    │   ├── skill/          # (Legacy — to be replaced by clawup-ops)
    │   ├── soul/           # (Legacy — to be replaced by clawup-ops)
    │   ├── profile/        # (Legacy — to be replaced by clawup-ops)
    │   └── utils/          # (Legacy — to be replaced by clawup-core)
    ├── templates/          # Tera template files for `clawup init`
    │   ├── default/
    │   ├── multi-agent/
    │   └── team/
    └── tests/
        └── cli_test.rs     # Integration tests (assert_cmd)
```

> **Note**: Modules marked `(Legacy)` in `crates/clawup/src/` contain duplicated code that
> will be replaced by calls to `clawup-core` and `clawup-ops` in an upcoming refactoring task.

## Technology Stack

| Domain | Technology |
|--------|-----------|
| CLI framework | clap 4 (derive macros) |
| Serialization | serde + toml + serde_json + json5 |
| Error handling | thiserror (library crates), color-eyre (CLI crate) |
| TUI / UX | dialoguer (fuzzy-select), indicatif, console, comfy-table |
| Diff engine | similar 2 (inline mode) |
| Templating | tera 1 |
| Git | git2 0.19 |
| Filesystem | dirs, shellexpand, walkdir, globset |
| Logging | tracing + tracing-subscriber (env-filter) |
| Env vars | dotenvy |
| Testing | rstest, assert_fs, assert_cmd, predicates |

## Development Commands

All commands use `vx` as the environment manager and `just` as the task runner:

```bash
vx just build          # Build the project
vx just build-release  # Build in release mode
vx just test           # Run all workspace tests
vx just test-verbose   # Run tests with output
vx just lint           # Run clippy (with -D warnings)
vx just fmt            # Format code
vx just fmt-check      # Check formatting
vx just check          # Full check: fmt-check + lint + test
vx just run --help     # Run the CLI
vx just clean          # Clean build artifacts
vx just install        # Install locally
```

**Important**: Always use `vx` to wrap commands. Direct `cargo`, `git`, `npm` etc. should
be called through `vx`:

```bash
vx cargo check --workspace
vx cargo test -p clawup-ops
vx git status
```

## Coding Conventions

### Rust Style

- **Edition**: 2024
- **Error types**: Use `thiserror` for library crates; `color-eyre` for the CLI binary
- **Testing**: Use `rstest` framework. Place tests in `tests/` directories, not inline
- **Imports**: Group by std → external crates → internal crates, separated by blank lines

### Adding New Features

1. **Data types** → Add to `clawup-schema` (keep it pure, serde-only)
2. **Core logic** (paths, fs, manifest CRUD) → Add to `clawup-core`
3. **Business operations** (agent/skill/soul/profile workflows) → Add to `clawup-ops`
4. **CLI commands** → Add to `clawup/src/cli/`
5. **Never** add heavy dependencies (git2, tera, etc.) to `clawup-schema` or `clawup-core`

### Configuration Schema

The `clawup.toml` schema is defined in `clawup-schema/src/manifest.rs`. Key sections:

```toml
[meta]                    # schema_version, description
[gateway]                 # provider, model, api_key_env, base_url
[agents.defaults]         # Default model, approval_mode, max_turns
[[agents.list]]           # Individual agent definitions
[[bindings]]              # File-pattern → agent routing
[[channels]]              # Communication channels
[skills.bundled]          # Built-in skills (enabled list)
[skills.community]        # Community skills (allow, sources)
[[skills.entries]]        # Individual skill entries
[[cron]]                  # Scheduled jobs
[hooks]                   # pre_apply, post_apply, pre_sync, post_sync
[profiles.<name>]         # Named profile overrides
[env]                     # Environment variable mappings
```

### OpenClaw Paths

`OpenClawPaths` detects the OpenClaw installation:
- Checks `$OPENCLAW_HOME` environment variable first
- Falls back to `~/.openclaw/`
- Provides methods: `config_file()`, `workspace_dir()`, `agent_workspace_dir(name)`,
  `skills_dir()`, `credentials_dir()`, `memory_dir()`

### Soul Files

OpenClaw uses markdown "soul files" to define agent personality. Known files:
- `SOUL.md` — Core personality and behavior
- `IDENTITY.md` — Agent identity metadata
- `USER.md` — User preferences
- `AGENTS.md` — Multi-agent coordination
- `BOOT.md` — Startup instructions

## CLI Commands Reference

| Command | Status | Description |
|---------|--------|-------------|
| `clawup init` | ✅ Implemented | Initialize new configuration (interactive/non-interactive, template selection) |
| `clawup apply` | ✅ Basic | Apply configuration to OpenClaw (dry-run, profile, force) |
| `clawup doctor` | ✅ Implemented | Health check for OpenClaw installation |
| `clawup agent list\|add\|remove\|show\|set` | ✅ Implemented | Agent CRUD operations |
| `clawup skill list\|add\|remove\|enable\|disable` | ✅ Implemented | Skill management |
| `clawup soul show\|edit\|diff\|from-template` | ⚠️ Partial | Soul file management |
| `clawup profile list\|switch\|create` | ⚠️ Partial | Profile switching |
| `clawup config get\|set\|show\|merge` | ✅ Most | Configuration operations |
| `clawup sync` | ❌ Stub | Git sync (not yet implemented) |
| `clawup diff` | ❌ Stub | Configuration diff (not yet implemented) |
| `clawup export` | ❌ Stub | Configuration export (not yet implemented) |

## Known Technical Debt

1. **Code Duplication**: The `clawup` CLI crate contains legacy modules (`manifest/`, `openclaw/`,
   `agent/`, `skill/`, `soul/`, `profile/`, `utils/`) that duplicate logic now in `clawup-core`
   and `clawup-ops`. These will be removed when the CLI is refactored to use the library crates.

2. **Stub Commands**: `sync`, `diff`, and `export` commands have placeholder implementations.

3. **Remote Skill Sources**: Skill installation currently only supports local file copying.
   Remote sources (Git, HTTP) are not yet implemented.

## CI/CD

The project uses [rust-actions-toolkit v4](https://github.com/loonghao/rust-actions-toolkit)
reusable workflows for CI and release, and [release-please](https://github.com/googleapis/release-please)
for automated version management.

### Workflow Files

| File | Trigger | Purpose |
|------|---------|---------|
| `.github/workflows/ci.yml` | PR to `main`, push to `main`/`develop` | Code quality + cross-platform tests (reusable-ci.yml@v4) |
| `.github/workflows/release.yml` | Tag push `v*` | Build 8 platform binaries + upload to GitHub Release (reusable-release.yml@v4) |
| `.github/workflows/release-please.yml` | Push to `main` | Auto version bump PR, changelog generation, GitHub Release creation |
| `release-please-config.json` | — | Per-package release-please configuration (Rust workspace) |
| `.release-please-manifest.json` | — | Version manifest tracking current versions of all crates |

### Release Flow

```
Push to main → release-please creates/updates Release PR (version bump + changelog)
             → On merge: creates GitHub Release + v* tag
             → v* tag triggers release.yml → builds binaries for 8 platforms
```

### Conventional Commits

release-please uses [Conventional Commits](https://www.conventionalcommits.org/) to determine
version bumps. Commit messages should follow this format:

- `feat: ...` → minor version bump
- `fix: ...` → patch version bump
- `feat!: ...` or `BREAKING CHANGE:` → major version bump (after v1.0)
- `chore:`, `docs:`, `ci:`, `refactor:`, `test:`, `style:` → no version bump

### Required GitHub Secrets

| Secret | Required | Purpose |
|--------|----------|---------|
| `RELEASE_PLZ_TOKEN` | Optional | GitHub PAT for cross-workflow triggers (falls back to `GITHUB_TOKEN`) |

## Testing

```bash
# Run all tests
vx just test

# Run tests for a specific crate
vx cargo test -p clawup-schema
vx cargo test -p clawup-core
vx cargo test -p clawup-ops
vx cargo test -p clawup

# Run with output
vx just test-verbose

# Run full CI checks locally (mirrors GitHub Actions)
vx just ci
```

Integration tests are in `crates/clawup/tests/cli_test.rs` and use `assert_cmd` + `predicates`.
