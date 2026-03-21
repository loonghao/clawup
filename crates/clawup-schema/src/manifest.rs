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
///
/// Maps to `agents.defaults.compaction` in `openclaw.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionConfig {
    /// Model to use for summarization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Reserve token floor — trigger compaction when remaining tokens below this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_tokens_floor: Option<u64>,

    /// Memory flush configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_flush: Option<MemoryFlushConfig>,
}

/// Memory flush sub-config within compaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFlushConfig {
    /// Whether memory flushing is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Memory search / RAG retrieval configuration.
///
/// Maps to `agents.defaults.memorySearch` in `openclaw.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchConfig {
    /// Whether memory search is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Embedding provider: "local", "openai", "gemini", "voyage".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Embedding model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Query configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<MemorySearchQueryConfig>,

    /// Sync / watch configuration.
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

    /// Semantic vector similarity weight (0.0–1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_weight: Option<f64>,

    /// BM25 keyword weight (0.0–1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<f64>,
}

/// Memory search file-watching sync config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchSyncConfig {
    /// Watch for file changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<bool>,
}

/// Sandbox configuration for an agent.
///
/// Maps to `agents.defaults.sandbox` or per-agent sandbox in `openclaw.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Whether sandbox is enabled (legacy field).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Sandbox mode: "off", "non-main", "all".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Sandbox backend: "local", "ssh".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,

    /// Session scope: "session", "agent", "shared".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Workspace access: "read-only", "read-write", "none".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access: Option<String>,

    /// Workspace root directory for sandboxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_root: Option<String>,

    /// Permissions list (legacy field).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    /// SSH backend configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SandboxSshConfig>,

    /// Pruning configuration for sandbox sessions.
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
    /// Idle hours before pruning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_hours: Option<u32>,

    /// Max age in days before pruning.
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
    /// Allowed tools or tool groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,

    /// Denied tools or tool groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
}

// =============================================================================
// Tools — Global tool permissions and profiles
// =============================================================================

/// Global tools configuration.
///
/// Maps to `tools.*` in `openclaw.json`.
/// Controls which tools are available and their security settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    /// Tool profile preset: "messaging", "minimal", "standard", "coding", "extended".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    /// Explicitly allowed tools or groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,

    /// Explicitly denied tools or groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,

    /// Filesystem tool settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs: Option<ToolsFsConfig>,

    /// Shell execution settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<ToolsExecConfig>,

    /// Elevated (host-level) execution settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated: Option<ToolsElevatedConfig>,

    /// Per-provider tool profile overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_provider: Option<HashMap<String, ToolsProviderOverride>>,

    /// Sandbox-specific tool settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<ToolsSandboxConfig>,
}

/// Filesystem tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsFsConfig {
    /// Restrict file operations to workspace only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_only: Option<bool>,
}

/// Shell execution tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsExecConfig {
    /// Security policy: "deny", "ask", "allow".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,

    /// Ask policy: "always", "never", "dangerous".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask: Option<String>,
}

/// Elevated execution configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsElevatedConfig {
    /// Whether elevated execution is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Allowed callers by channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<HashMap<String, Vec<String>>>,
}

/// Per-provider tool profile override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsProviderOverride {
    /// Override profile for this provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    /// Additional allowed tools for this provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,

    /// Additional denied tools for this provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
}

/// Sandbox-specific tool configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsSandboxConfig {
    /// Tool settings within sandbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<AgentToolsConfig>,
}

// =============================================================================
// Channels — Communication platform configuration
// =============================================================================

/// Per-channel configuration.
///
/// Maps to `channels.<name>` in `openclaw.json`.
/// Each channel (whatsapp, telegram, discord, slack, etc.) has its own config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Whether this channel is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Bot token or API token (supports `${ENV_VAR}` syntax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Bot token (alternative field name used by some channels).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_token: Option<String>,

    /// Allowed sender IDs/numbers whitelist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<Vec<String>>,

    /// Blocked sender IDs/numbers blacklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_from: Option<Vec<String>>,

    /// Whether to auto-reply to messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reply: Option<bool>,

    /// Whether to allow group chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_groups: Option<bool>,

    /// DM policy: "pairing", "allowlist", "open", "disabled".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_policy: Option<String>,

    /// Group policy: "allowlist", "open", "disabled".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_policy: Option<String>,

    /// Group-specific configurations keyed by group name or "*".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<HashMap<String, ChannelGroupConfig>>,

    /// Group allow-from whitelist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_allow_from: Option<Vec<String>>,

    /// Bound agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// DM-specific configuration (for Discord etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm: Option<ChannelDmConfig>,

    /// Additional channel-specific key-value config.
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub extra: Option<HashMap<String, toml::Value>>,
}

/// Group-specific channel configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelGroupConfig {
    /// Whether the bot must be @mentioned to respond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_mention: Option<bool>,
}

/// DM-specific channel configuration (e.g. for Discord).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDmConfig {
    /// Allowed DM sender IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<Vec<String>>,
}

// =============================================================================
// Session — Conversation session management
// =============================================================================

/// Session management configuration.
///
/// Maps to `session.*` in `openclaw.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// DM session scope: "main", "per-peer", "per-channel-peer", "per-account-channel-peer".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_scope: Option<String>,

    /// Thread binding settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_bindings: Option<ThreadBindingsConfig>,

    /// Automatic session reset settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<SessionResetConfig>,
}

/// Thread binding configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadBindingsConfig {
    /// Whether thread bindings are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Idle hours before unbinding a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_hours: Option<u32>,

    /// Max age in hours (0 = no limit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_hours: Option<u32>,
}

/// Automatic session reset configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResetConfig {
    /// Reset mode: "daily", "idle", "off".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Hour of day for daily reset (0–23).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_hour: Option<u32>,

    /// Idle minutes before reset.
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
///
/// Maps to `cron.*` in `openclaw.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronConfig {
    /// Whether cron is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Maximum number of concurrent cron runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<u32>,

    /// Session retention duration (e.g. "24h").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_retention: Option<String>,

    /// Run log configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_log: Option<CronRunLogConfig>,

    /// Individual cron job definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<CronJob>>,
}

/// Cron run log configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronRunLogConfig {
    /// Maximum log size (e.g. "2mb").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<String>,

    /// Number of log lines to keep.
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
///
/// Maps to `hooks.*` in `openclaw.json`.
/// Supports both simple lifecycle hooks and full webhook configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksConfig {
    /// Whether hooks are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Shared authentication token for webhook endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Webhook endpoint path (e.g. "/hooks").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Maximum request body size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_body_bytes: Option<u64>,

    /// Default session key for incoming hooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_session_key: Option<String>,

    /// Allow custom session keys in requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_request_session_key: Option<bool>,

    /// Allowed session key prefixes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_session_key_prefixes: Option<Vec<String>>,

    /// Allowed agent IDs for hook processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_agent_ids: Option<Vec<String>>,

    /// Preset hook configurations (e.g. "gmail").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<String>>,

    /// Directory for custom transform scripts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms_dir: Option<String>,

    /// Webhook mapping rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<HookMapping>>,

    // --- Simple lifecycle hooks (clawup-specific) ---
    /// Commands to run before apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_apply: Option<Vec<String>>,

    /// Commands to run after apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_apply: Option<Vec<String>>,

    /// Commands to run before sync.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_sync: Option<Vec<String>>,

    /// Commands to run after sync.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_sync: Option<Vec<String>>,
}

/// Webhook mapping rule for incoming hook requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookMapping {
    /// Match criteria for the incoming request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<HookMatchConfig>,

    /// Action to take: "agent", "webhook", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Target agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// Wake mode: "now", "idle", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_mode: Option<String>,

    /// Display name for the mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Session key template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,

    /// Message template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_template: Option<String>,

    /// Whether to deliver the message to the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<bool>,

    /// Channel to use for delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    /// Model override for this hook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Hook match criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookMatchConfig {
    /// URL path to match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

// =============================================================================
// Discovery — Network discovery configuration
// =============================================================================

/// Network discovery configuration.
///
/// Maps to `discovery.*` in `openclaw.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// mDNS local network discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdns: Option<MdnsConfig>,

    /// Wide-area DNS-SD discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_area: Option<WideAreaConfig>,
}

/// mDNS configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsConfig {
    /// mDNS mode: "minimal", "full", "off".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// Wide-area DNS-SD configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WideAreaConfig {
    /// Whether wide-area discovery is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
