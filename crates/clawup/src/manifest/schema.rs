use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root manifest structure representing a `clawup.toml` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub meta: Meta,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<AgentsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<Binding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<SkillsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<Vec<CronJob>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<HooksConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<HashMap<String, toml::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

/// Metadata about the configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub schema_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Gateway / LLM provider configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Gateway {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_env: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

/// Agents configuration section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<AgentDefaults>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AgentDefinition>>,
}

/// Default settings applied to all agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<u32>,
}

/// Individual agent definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefinition {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagents: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<SandboxConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityConfig>,
}

/// Sandbox configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// Identity configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

/// Binding rule for routing tasks to agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    pub agent: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Channel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

/// Skills configuration section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled: Option<BundledSkills>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub community: Option<CommunitySkills>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<SkillEntry>>,
}

/// Bundled skills configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundledSkills {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Vec<String>>,
}

/// Community skills configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunitySkills {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
}

/// Individual skill entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEntry {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

/// Cron job definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    pub name: String,
    pub schedule: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
}

/// Hooks configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_apply: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_apply: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_sync: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_sync: Option<Vec<String>>,
}
