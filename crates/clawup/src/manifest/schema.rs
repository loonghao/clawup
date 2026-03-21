use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root manifest structure representing a `clawup.toml` file.
///
/// Maps to OpenClaw's `openclaw.json` configuration sections.
/// See: <https://docs.openclaw.ai/gateway/configuration-reference>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub meta: Meta,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gateway>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<ModelsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<AgentsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<Binding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<HashMap<String, ChannelConfig>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<SkillsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<CronConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<HooksConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery: Option<DiscoveryConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<HashMap<String, toml::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

// =============================================================================
// Meta
// =============================================================================

/// Metadata about the configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub schema_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// =============================================================================
// Gateway — OpenClaw gateway server configuration
// =============================================================================

/// Gateway configuration controlling the OpenClaw server.
///
/// Maps to `gateway.*` in `openclaw.json`.
/// Includes network binding, authentication, and hot-reload settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Gateway {
    /// Gateway running mode (e.g. "local", "remote").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Bind address (e.g. "loopback", "lan", "0.0.0.0").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,

    /// Listening port (default: 18789).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// Authentication configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<GatewayAuth>,

    /// Hot-reload configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload: Option<GatewayReload>,

    /// Tailscale integration settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tailscale: Option<HashMap<String, toml::Value>>,
}

/// Gateway authentication settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayAuth {
    /// Auth mode (e.g. "token", "none").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Auth token string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Gateway hot-reload configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayReload {
    /// Reload mode: "hybrid" (default), "hot", "restart", "off".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

// =============================================================================
// Models — Multi-provider model routing
// =============================================================================

/// Model provider and routing configuration.
///
/// Maps to `models.*` in `openclaw.json`.
/// Supports multiple providers and failover ordering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    /// Merge mode for model configuration ("merge" or "replace").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Provider definitions keyed by provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<HashMap<String, ModelProvider>>,

    /// Provider failover order, keyed by provider family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<HashMap<String, Vec<String>>>,
}

/// Individual model provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProvider {
    /// Auth mode (e.g. "api_key", "oauth").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// API key (supports `${ENV_VAR}` syntax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// Base URL for the provider API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// OAuth email for subscription-based providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Provider display alias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

// =============================================================================
// Agents — Agent definitions and defaults
// =============================================================================

/// Agents configuration section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<AgentDefaults>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AgentDefinition>>,
}

/// Default settings applied to all agents.
///
/// Maps to `agents.defaults` in `openclaw.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefaults {
    /// Default model (simple string or model config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Default approval mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_mode: Option<String>,

    /// Max conversation turns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_turns: Option<u32>,

    /// Default workspace path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,

    /// Compaction (context summarization) settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction: Option<CompactionConfig>,

    /// Memory search / retrieval settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_search: Option<MemorySearchConfig>,

    /// Default sandbox configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<SandboxConfig>,
}

/// Individual agent definition.
///
/// Maps to entries in `agents.list[]` in `openclaw.json`.
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

    /// Agent workspace directory override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagents: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<SandboxConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityConfig>,

    /// Per-agent tool allow/deny overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<AgentToolsConfig>,

    /// Per-agent compaction settings override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compaction: Option<CompactionConfig>,

    /// Per-agent memory search settings override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_search: Option<MemorySearchConfig>,
}

/// Compaction (context summarization) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_tokens_floor: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_flush: Option<MemoryFlushConfig>,
}

/// Memory flush sub-config within compaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFlushConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Memory search / RAG retrieval configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<MemorySearchQueryConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<MemorySearchSyncConfig>,
}

/// Memory search query settings (hybrid search).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchQueryConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<HybridSearchConfig>,
}

/// Hybrid search weighting configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_weight: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<f64>,
}

/// Memory search file-watching sync config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchSyncConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<bool>,
}

/// Sandbox configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_root: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SandboxSshConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune: Option<SandboxPruneConfig>,
}

/// SSH backend configuration for sandboxing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxSshConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_root: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_host_key_checking: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_host_keys: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_hosts_file: Option<String>,
}

/// Sandbox session pruning configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPruneConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_hours: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_days: Option<u32>,
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

/// Per-agent tool allow/deny configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
}

// =============================================================================
// Tools — Global tool permissions and profiles
// =============================================================================

/// Global tools configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs: Option<ToolsFsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<ToolsExecConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated: Option<ToolsElevatedConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_provider: Option<HashMap<String, ToolsProviderOverride>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<ToolsSandboxConfig>,
}

/// Filesystem tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsFsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_only: Option<bool>,
}

/// Shell execution tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsExecConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask: Option<String>,
}

/// Elevated execution configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsElevatedConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<HashMap<String, Vec<String>>>,
}

/// Per-provider tool profile override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsProviderOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
}

/// Sandbox-specific tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsSandboxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<AgentToolsConfig>,
}

// =============================================================================
// Channels — Communication platform configuration
// =============================================================================

/// Per-channel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_from: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reply: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_groups: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_policy: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_policy: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<HashMap<String, ChannelGroupConfig>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_allow_from: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm: Option<ChannelDmConfig>,

    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub extra: Option<HashMap<String, toml::Value>>,
}

/// Group-specific channel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelGroupConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_mention: Option<bool>,
}

/// DM-specific channel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<Vec<String>>,
}

// =============================================================================
// Session — Conversation session management
// =============================================================================

/// Session management configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_scope: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_bindings: Option<ThreadBindingsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<SessionResetConfig>,
}

/// Thread binding configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadBindingsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_hours: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_hours: Option<u32>,
}

/// Automatic session reset configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResetConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_hour: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_minutes: Option<u32>,
}

// =============================================================================
// Skills
// =============================================================================

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

// =============================================================================
// Bindings — Task routing rules
// =============================================================================

/// Binding rule for routing tasks to agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    pub agent: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// =============================================================================
// Cron — Scheduled tasks
// =============================================================================

/// Top-level cron configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_retention: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_log: Option<CronRunLogConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<CronJob>>,
}

/// Cron run log configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronRunLogConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_lines: Option<u32>,
}

/// Individual cron job definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    pub name: String,
    pub schedule: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
}

// =============================================================================
// Hooks — Webhook and lifecycle hook configuration
// =============================================================================

/// Hooks configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_body_bytes: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_session_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_request_session_key: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_session_key_prefixes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_agent_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<HookMapping>>,

    // --- Simple lifecycle hooks (clawup-specific) ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_apply: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_apply: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_sync: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_sync: Option<Vec<String>>,
}

/// Webhook mapping rule for incoming hook requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookMapping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<HookMatchConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Hook match criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookMatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

// =============================================================================
// Discovery — Network discovery configuration
// =============================================================================

/// Network discovery configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdns: Option<MdnsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_area: Option<WideAreaConfig>,
}

/// mDNS configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// Wide-area DNS-SD configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WideAreaConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
