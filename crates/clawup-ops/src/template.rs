//! Template module.
//!
//! Provides built-in manifest templates (default, multi-agent, team)
//! for quick project initialization.

use clawup_schema::*;

use crate::error::Result;

/// Create a manifest from a built-in template name.
///
/// Supported templates:
/// - `"default"` — Single agent with bundled skills
/// - `"multi-agent"` — Three agents (code/review/ops) with bindings
/// - `"team"` — Multi-agent with dev/staging/production profiles
pub fn from_template(template: &str, description: &str) -> Result<Manifest> {
    let mut manifest = match template {
        "multi-agent" => multi_agent_template(),
        "team" => team_template(),
        _ => default_template(),
    };
    manifest.meta.description = Some(description.to_string());
    Ok(manifest)
}

/// Generate the default single-agent template.
fn default_template() -> Manifest {
    Manifest {
        meta: Meta {
            schema_version: "1".to_string(),
            description: None,
        },
        gateway: Some(Gateway {
            provider: Some("openrouter".to_string()),
            model: Some("anthropic/claude-sonnet-4".to_string()),
            api_key_env: Some("OPENROUTER_API_KEY".to_string()),
            base_url: None,
        }),
        agents: Some(AgentsConfig {
            defaults: Some(AgentDefaults {
                model: Some("anthropic/claude-sonnet-4".to_string()),
                approval_mode: Some("auto-edit".to_string()),
                max_turns: None,
            }),
            list: Some(vec![AgentDefinition {
                name: "code".to_string(),
                role: Some("Software Engineer".to_string()),
                instructions: Some("Write clean, well-tested code".to_string()),
                model: None,
                approval_mode: None,
                max_turns: None,
                subagents: None,
                sandbox: None,
                identity: None,
            }]),
        }),
        bindings: None,
        channels: None,
        skills: Some(SkillsConfig {
            bundled: Some(BundledSkills {
                enabled: Some(vec!["developer".to_string(), "computer".to_string()]),
            }),
            community: None,
            entries: None,
        }),
        cron: None,
        hooks: None,
        profiles: None,
        env: None,
    }
}

/// Generate the multi-agent template with code/review/ops agents.
fn multi_agent_template() -> Manifest {
    Manifest {
        meta: Meta {
            schema_version: "1".to_string(),
            description: None,
        },
        gateway: Some(Gateway {
            provider: Some("openrouter".to_string()),
            model: Some("anthropic/claude-sonnet-4".to_string()),
            api_key_env: Some("OPENROUTER_API_KEY".to_string()),
            base_url: None,
        }),
        agents: Some(AgentsConfig {
            defaults: Some(AgentDefaults {
                model: Some("anthropic/claude-sonnet-4".to_string()),
                approval_mode: Some("auto-edit".to_string()),
                max_turns: None,
            }),
            list: Some(vec![
                AgentDefinition {
                    name: "code".to_string(),
                    role: Some("Senior Software Engineer".to_string()),
                    instructions: Some(
                        "Focus on clean, tested code with solid architecture".to_string(),
                    ),
                    model: None,
                    approval_mode: None,
                    max_turns: None,
                    subagents: None,
                    sandbox: None,
                    identity: None,
                },
                AgentDefinition {
                    name: "review".to_string(),
                    role: Some("Code Reviewer".to_string()),
                    instructions: Some(
                        "Review PRs for correctness, style, and security".to_string(),
                    ),
                    model: None,
                    approval_mode: Some("suggest".to_string()),
                    max_turns: None,
                    subagents: None,
                    sandbox: None,
                    identity: None,
                },
                AgentDefinition {
                    name: "ops".to_string(),
                    role: Some("DevOps Engineer".to_string()),
                    instructions: Some(
                        "Manage CI/CD, infrastructure, and deployments".to_string(),
                    ),
                    model: None,
                    approval_mode: None,
                    max_turns: None,
                    subagents: None,
                    sandbox: None,
                    identity: None,
                },
            ]),
        }),
        bindings: Some(vec![
            Binding {
                pattern: Some("*.rs".to_string()),
                agent: "code".to_string(),
                description: None,
            },
            Binding {
                pattern: Some("*.yml".to_string()),
                agent: "ops".to_string(),
                description: None,
            },
        ]),
        channels: None,
        skills: Some(SkillsConfig {
            bundled: Some(BundledSkills {
                enabled: Some(vec!["developer".to_string(), "computer".to_string()]),
            }),
            community: None,
            entries: None,
        }),
        cron: None,
        hooks: None,
        profiles: None,
        env: None,
    }
}

/// Generate the team template (multi-agent + profiles).
fn team_template() -> Manifest {
    let mut manifest = multi_agent_template();
    manifest.profiles = Some(
        [
            (
                "dev".to_string(),
                toml::Value::Table({
                    let mut t = toml::map::Map::new();
                    t.insert(
                        "description".to_string(),
                        toml::Value::String("Development environment".to_string()),
                    );
                    t
                }),
            ),
            (
                "staging".to_string(),
                toml::Value::Table({
                    let mut t = toml::map::Map::new();
                    t.insert(
                        "description".to_string(),
                        toml::Value::String("Staging environment".to_string()),
                    );
                    t
                }),
            ),
            (
                "production".to_string(),
                toml::Value::Table({
                    let mut t = toml::map::Map::new();
                    t.insert(
                        "description".to_string(),
                        toml::Value::String("Production environment".to_string()),
                    );
                    t
                }),
            ),
        ]
        .into_iter()
        .collect(),
    );
    manifest
}
