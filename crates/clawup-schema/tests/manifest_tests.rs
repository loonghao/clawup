use std::collections::HashMap;

use clawup_schema::*;
use rstest::rstest;

// =============================================================================
// Manifest roundtrip tests
// =============================================================================

#[rstest]
fn test_minimal_manifest_roundtrip() {
    let manifest = Manifest {
        meta: Meta {
            schema_version: "1".to_string(),
            description: None,
        },
        gateway: None,
        models: None,
        agents: None,
        tools: None,
        bindings: None,
        channels: None,
        session: None,
        skills: None,
        cron: None,
        hooks: None,
        discovery: None,
        profiles: None,
        env: None,
    };

    let toml_str = toml::to_string_pretty(&manifest).unwrap();
    let parsed: Manifest = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.meta.schema_version, "1");
    assert!(parsed.meta.description.is_none());
    assert!(parsed.gateway.is_none());
    assert!(parsed.agents.is_none());
    assert!(parsed.models.is_none());
    assert!(parsed.tools.is_none());
    assert!(parsed.session.is_none());
    assert!(parsed.discovery.is_none());
}

#[rstest]
fn test_full_manifest_roundtrip() {
    let manifest = Manifest {
        meta: Meta {
            schema_version: "1".to_string(),
            description: Some("Test project".to_string()),
        },
        gateway: Some(Gateway {
            mode: Some("local".to_string()),
            bind: Some("loopback".to_string()),
            port: Some(18789),
            auth: Some(GatewayAuth {
                mode: Some("token".to_string()),
                token: Some("my-secret-token".to_string()),
            }),
            reload: Some(GatewayReload {
                mode: Some("hybrid".to_string()),
            }),
            tailscale: None,
        }),
        models: Some(ModelsConfig {
            mode: Some("merge".to_string()),
            providers: Some(HashMap::from([(
                "anthropic".to_string(),
                ModelProvider {
                    mode: Some("api_key".to_string()),
                    api_key: Some("${ANTHROPIC_API_KEY}".to_string()),
                    base_url: None,
                    email: None,
                    alias: None,
                },
            )])),
            order: Some(HashMap::from([(
                "anthropic".to_string(),
                vec!["anthropic:api".to_string()],
            )])),
        }),
        agents: Some(AgentsConfig {
            defaults: Some(AgentDefaults {
                model: Some("gpt-4".to_string()),
                approval_mode: Some("auto-edit".to_string()),
                max_turns: Some(10),
                workspace: Some("~/.openclaw/workspace".to_string()),
                compaction: Some(CompactionConfig {
                    model: Some("openrouter/anthropic/claude-sonnet-4-5".to_string()),
                    reserve_tokens_floor: Some(20000),
                    memory_flush: Some(MemoryFlushConfig {
                        enabled: Some(true),
                    }),
                }),
                memory_search: Some(MemorySearchConfig {
                    enabled: Some(true),
                    provider: Some("local".to_string()),
                    model: Some("all-MiniLM-L6-v2".to_string()),
                    query: Some(MemorySearchQueryConfig {
                        hybrid: Some(HybridSearchConfig {
                            enabled: Some(true),
                            vector_weight: Some(0.7),
                            text_weight: Some(0.3),
                        }),
                    }),
                    sync: Some(MemorySearchSyncConfig { watch: Some(true) }),
                }),
                sandbox: None,
            }),
            list: Some(vec![AgentDefinition {
                name: "coder".to_string(),
                role: Some("Developer".to_string()),
                instructions: Some("Write code".to_string()),
                model: Some("gpt-4".to_string()),
                approval_mode: Some("suggest".to_string()),
                max_turns: Some(5),
                workspace: Some("~/.openclaw/agents/coder/workspace".to_string()),
                subagents: Some(vec!["helper".to_string()]),
                sandbox: Some(SandboxConfig {
                    enabled: Some(true),
                    mode: Some("non-main".to_string()),
                    backend: None,
                    scope: Some("agent".to_string()),
                    workspace_access: None,
                    workspace_root: None,
                    permissions: Some(vec!["read".to_string(), "write".to_string()]),
                    ssh: None,
                    prune: None,
                }),
                identity: Some(IdentityConfig {
                    name: Some("Coder".to_string()),
                    persona: Some("a helpful developer".to_string()),
                    template: Some("default".to_string()),
                }),
                tools: Some(AgentToolsConfig {
                    allow: Some(vec!["group:fs".to_string(), "exec".to_string()]),
                    deny: Some(vec!["browser".to_string()]),
                }),
                compaction: None,
                memory_search: None,
            }]),
        }),
        tools: Some(ToolsConfig {
            profile: Some("coding".to_string()),
            allow: None,
            deny: Some(vec!["group:automation".to_string()]),
            fs: Some(ToolsFsConfig {
                workspace_only: Some(true),
            }),
            exec: Some(ToolsExecConfig {
                security: Some("ask".to_string()),
                ask: Some("always".to_string()),
            }),
            elevated: Some(ToolsElevatedConfig {
                enabled: Some(false),
                allow_from: None,
            }),
            by_provider: None,
            sandbox: None,
        }),
        bindings: Some(vec![Binding {
            pattern: Some("*.rs".to_string()),
            agent: "coder".to_string(),
            description: Some("Rust files".to_string()),
        }]),
        channels: Some(HashMap::from([(
            "whatsapp".to_string(),
            ChannelConfig {
                enabled: None,
                token: None,
                bot_token: None,
                allow_from: Some(vec!["+15555550123".to_string()]),
                block_from: None,
                auto_reply: Some(true),
                allow_groups: None,
                dm_policy: Some("pairing".to_string()),
                group_policy: None,
                groups: Some(HashMap::from([(
                    "*".to_string(),
                    ChannelGroupConfig {
                        require_mention: Some(true),
                    },
                )])),
                group_allow_from: None,
                agent: None,
                dm: None,
                extra: None,
            },
        )])),
        session: Some(SessionConfig {
            dm_scope: Some("per-channel-peer".to_string()),
            thread_bindings: Some(ThreadBindingsConfig {
                enabled: Some(true),
                idle_hours: Some(24),
                max_age_hours: Some(0),
            }),
            reset: Some(SessionResetConfig {
                mode: Some("daily".to_string()),
                at_hour: Some(4),
                idle_minutes: Some(120),
            }),
        }),
        skills: Some(SkillsConfig {
            bundled: Some(BundledSkills {
                enabled: Some(vec!["developer".to_string()]),
            }),
            community: Some(CommunitySkills {
                allow: Some(true),
                sources: Some(vec!["https://skills.example.com".to_string()]),
            }),
            entries: Some(vec![SkillEntry {
                name: "my-skill".to_string(),
                source: Some("./skills/my-skill".to_string()),
                enabled: Some(true),
                config: Some(HashMap::from([("key".to_string(), "value".to_string())])),
            }]),
        }),
        cron: Some(CronConfig {
            enabled: Some(true),
            max_concurrent_runs: Some(2),
            session_retention: Some("24h".to_string()),
            run_log: Some(CronRunLogConfig {
                max_bytes: Some("2mb".to_string()),
                keep_lines: Some(2000),
            }),
            jobs: Some(vec![CronJob {
                name: "daily-check".to_string(),
                schedule: "0 0 * * *".to_string(),
                command: Some("clawup doctor".to_string()),
                agent: Some("ops".to_string()),
            }]),
        }),
        hooks: Some(HooksConfig {
            enabled: Some(true),
            token: Some("shared-secret".to_string()),
            path: Some("/hooks".to_string()),
            max_body_bytes: Some(262144),
            default_session_key: Some("hook:ingress".to_string()),
            allow_request_session_key: Some(false),
            allowed_session_key_prefixes: Some(vec!["hook:".to_string()]),
            allowed_agent_ids: Some(vec!["hooks".to_string(), "main".to_string()]),
            presets: Some(vec!["gmail".to_string()]),
            transforms_dir: Some("~/.openclaw/hooks/transforms".to_string()),
            mappings: Some(vec![HookMapping {
                r#match: Some(HookMatchConfig {
                    path: Some("gmail".to_string()),
                }),
                action: Some("agent".to_string()),
                agent_id: Some("hooks".to_string()),
                wake_mode: Some("now".to_string()),
                name: Some("Gmail".to_string()),
                session_key: None,
                message_template: None,
                deliver: Some(true),
                channel: Some("last".to_string()),
                model: None,
            }]),
            pre_apply: Some(vec!["echo pre".to_string()]),
            post_apply: Some(vec!["echo post".to_string()]),
            pre_sync: None,
            post_sync: None,
        }),
        discovery: Some(DiscoveryConfig {
            mdns: Some(MdnsConfig {
                mode: Some("minimal".to_string()),
            }),
            wide_area: Some(WideAreaConfig {
                enabled: Some(true),
            }),
        }),
        profiles: None,
        env: Some(HashMap::from([("MY_VAR".to_string(), "value".to_string())])),
    };

    let toml_str = toml::to_string_pretty(&manifest).unwrap();
    let parsed: Manifest = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.meta.schema_version, "1");
    assert_eq!(parsed.meta.description.as_deref(), Some("Test project"));

    // Gateway
    let gw = parsed.gateway.as_ref().unwrap();
    assert_eq!(gw.mode.as_deref(), Some("local"));
    assert_eq!(gw.bind.as_deref(), Some("loopback"));
    assert_eq!(gw.port, Some(18789));
    let auth = gw.auth.as_ref().unwrap();
    assert_eq!(auth.mode.as_deref(), Some("token"));
    assert_eq!(auth.token.as_deref(), Some("my-secret-token"));

    // Models
    let models = parsed.models.as_ref().unwrap();
    assert_eq!(models.mode.as_deref(), Some("merge"));
    let providers = models.providers.as_ref().unwrap();
    assert!(providers.contains_key("anthropic"));
    assert_eq!(
        providers["anthropic"].api_key.as_deref(),
        Some("${ANTHROPIC_API_KEY}")
    );

    // Agents
    let agents = parsed.agents.as_ref().unwrap();
    let defaults = agents.defaults.as_ref().unwrap();
    assert_eq!(defaults.max_turns, Some(10));
    assert_eq!(defaults.workspace.as_deref(), Some("~/.openclaw/workspace"));
    let compaction = defaults.compaction.as_ref().unwrap();
    assert_eq!(compaction.reserve_tokens_floor, Some(20000));
    let mem = defaults.memory_search.as_ref().unwrap();
    assert_eq!(mem.enabled, Some(true));
    assert_eq!(mem.provider.as_deref(), Some("local"));

    let list = agents.list.as_ref().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "coder");
    assert_eq!(list[0].subagents.as_ref().unwrap(), &["helper"]);
    assert_eq!(
        list[0].workspace.as_deref(),
        Some("~/.openclaw/agents/coder/workspace")
    );
    let tools = list[0].tools.as_ref().unwrap();
    assert_eq!(tools.allow.as_ref().unwrap(), &["group:fs", "exec"]);

    // Tools
    let tools_cfg = parsed.tools.as_ref().unwrap();
    assert_eq!(tools_cfg.profile.as_deref(), Some("coding"));
    assert_eq!(tools_cfg.fs.as_ref().unwrap().workspace_only, Some(true));
    assert_eq!(
        tools_cfg.exec.as_ref().unwrap().security.as_deref(),
        Some("ask")
    );

    // Session
    let session = parsed.session.as_ref().unwrap();
    assert_eq!(session.dm_scope.as_deref(), Some("per-channel-peer"));
    let tb = session.thread_bindings.as_ref().unwrap();
    assert_eq!(tb.enabled, Some(true));
    assert_eq!(tb.idle_hours, Some(24));
    let reset = session.reset.as_ref().unwrap();
    assert_eq!(reset.mode.as_deref(), Some("daily"));
    assert_eq!(reset.at_hour, Some(4));

    // Channels
    let channels = parsed.channels.as_ref().unwrap();
    let wa = channels.get("whatsapp").unwrap();
    assert_eq!(wa.dm_policy.as_deref(), Some("pairing"));
    assert_eq!(wa.auto_reply, Some(true));
    let groups = wa.groups.as_ref().unwrap();
    assert_eq!(groups["*"].require_mention, Some(true));

    // Cron
    let cron = parsed.cron.as_ref().unwrap();
    assert_eq!(cron.enabled, Some(true));
    assert_eq!(cron.max_concurrent_runs, Some(2));
    let jobs = cron.jobs.as_ref().unwrap();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].name, "daily-check");

    // Hooks
    let hooks = parsed.hooks.as_ref().unwrap();
    assert_eq!(hooks.enabled, Some(true));
    assert_eq!(hooks.token.as_deref(), Some("shared-secret"));
    let mappings = hooks.mappings.as_ref().unwrap();
    assert_eq!(mappings.len(), 1);
    assert_eq!(mappings[0].action.as_deref(), Some("agent"));

    // Discovery
    let discovery = parsed.discovery.as_ref().unwrap();
    assert_eq!(
        discovery.mdns.as_ref().unwrap().mode.as_deref(),
        Some("minimal")
    );
    assert_eq!(discovery.wide_area.as_ref().unwrap().enabled, Some(true));

    // Bindings
    let bindings = parsed.bindings.as_ref().unwrap();
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].agent, "coder");

    let env = parsed.env.as_ref().unwrap();
    assert_eq!(env.get("MY_VAR").unwrap(), "value");
}

// =============================================================================
// TOML parsing from string
// =============================================================================

#[rstest]
fn test_parse_minimal_toml() {
    let toml_str = r#"
[meta]
schema_version = "1"
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    assert_eq!(manifest.meta.schema_version, "1");
    assert!(manifest.gateway.is_none());
}

#[rstest]
fn test_parse_gateway_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[gateway]
mode = "local"
bind = "loopback"
port = 18789

[gateway.auth]
mode = "token"
token = "my-token"

[gateway.reload]
mode = "hybrid"
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let gw = manifest.gateway.unwrap();
    assert_eq!(gw.mode.as_deref(), Some("local"));
    assert_eq!(gw.port, Some(18789));
    let auth = gw.auth.unwrap();
    assert_eq!(auth.mode.as_deref(), Some("token"));
    assert_eq!(auth.token.as_deref(), Some("my-token"));
    let reload = gw.reload.unwrap();
    assert_eq!(reload.mode.as_deref(), Some("hybrid"));
}

#[rstest]
fn test_parse_models_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[models]
mode = "merge"

[models.providers.anthropic_api]
mode = "api_key"
api_key = "${ANTHROPIC_API_KEY}"

[models.providers.openai]
mode = "api_key"
api_key = "${OPENAI_API_KEY}"
base_url = "https://api.openai.com"

[models.order]
anthropic = ["anthropic_api"]
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let models = manifest.models.unwrap();
    assert_eq!(models.mode.as_deref(), Some("merge"));

    let providers = models.providers.unwrap();
    assert_eq!(providers.len(), 2);
    assert_eq!(providers["anthropic_api"].mode.as_deref(), Some("api_key"));
    assert_eq!(
        providers["openai"].base_url.as_deref(),
        Some("https://api.openai.com")
    );

    let order = models.order.unwrap();
    assert_eq!(order["anthropic"], vec!["anthropic_api"]);
}

#[rstest]
fn test_parse_agents_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[agents.defaults]
model = "gpt-4"
approval_mode = "auto-edit"
workspace = "~/.openclaw/workspace"

[agents.defaults.compaction]
model = "anthropic/claude-sonnet-4-5"
reserve_tokens_floor = 20000

[agents.defaults.compaction.memory_flush]
enabled = true

[[agents.list]]
name = "code"
role = "Developer"
instructions = "Write clean code"
workspace = "~/.openclaw/agents/code"

[[agents.list]]
name = "review"
role = "Reviewer"
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let agents = manifest.agents.unwrap();

    let defaults = agents.defaults.unwrap();
    assert_eq!(defaults.model.as_deref(), Some("gpt-4"));
    assert_eq!(defaults.approval_mode.as_deref(), Some("auto-edit"));
    assert_eq!(defaults.workspace.as_deref(), Some("~/.openclaw/workspace"));

    let compaction = defaults.compaction.unwrap();
    assert_eq!(compaction.reserve_tokens_floor, Some(20000));
    assert_eq!(compaction.memory_flush.unwrap().enabled, Some(true));

    let list = agents.list.unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].name, "code");
    assert_eq!(
        list[0].workspace.as_deref(),
        Some("~/.openclaw/agents/code")
    );
    assert_eq!(list[1].name, "review");
    assert!(list[1].instructions.is_none());
}

#[rstest]
fn test_parse_tools_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[tools]
profile = "coding"
deny = ["group:automation", "group:runtime"]

[tools.fs]
workspace_only = true

[tools.exec]
security = "deny"
ask = "always"

[tools.elevated]
enabled = false

[tools.by_provider.openai_gpt5]
profile = "minimal"

[tools.by_provider.google]
allow = ["group:fs", "sessions_list"]
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let tools = manifest.tools.unwrap();

    assert_eq!(tools.profile.as_deref(), Some("coding"));
    assert_eq!(
        tools.deny.as_ref().unwrap(),
        &["group:automation", "group:runtime"]
    );
    assert_eq!(tools.fs.as_ref().unwrap().workspace_only, Some(true));
    assert_eq!(
        tools.exec.as_ref().unwrap().security.as_deref(),
        Some("deny")
    );
    assert_eq!(tools.exec.as_ref().unwrap().ask.as_deref(), Some("always"));
    assert_eq!(tools.elevated.as_ref().unwrap().enabled, Some(false));

    let by_provider = tools.by_provider.as_ref().unwrap();
    assert_eq!(
        by_provider["openai_gpt5"].profile.as_deref(),
        Some("minimal")
    );
    assert_eq!(
        by_provider["google"].allow.as_ref().unwrap(),
        &["group:fs", "sessions_list"]
    );
}

#[rstest]
fn test_parse_session_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[session]
dm_scope = "per-channel-peer"

[session.thread_bindings]
enabled = true
idle_hours = 24
max_age_hours = 0

[session.reset]
mode = "daily"
at_hour = 4
idle_minutes = 120
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let session = manifest.session.unwrap();

    assert_eq!(session.dm_scope.as_deref(), Some("per-channel-peer"));

    let tb = session.thread_bindings.unwrap();
    assert_eq!(tb.enabled, Some(true));
    assert_eq!(tb.idle_hours, Some(24));
    assert_eq!(tb.max_age_hours, Some(0));

    let reset = session.reset.unwrap();
    assert_eq!(reset.mode.as_deref(), Some("daily"));
    assert_eq!(reset.at_hour, Some(4));
    assert_eq!(reset.idle_minutes, Some(120));
}

#[rstest]
fn test_parse_channels_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[channels.whatsapp]
allow_from = ["+15555550123"]
auto_reply = true
dm_policy = "pairing"

[channels.whatsapp.groups."*"]
require_mention = true

[channels.telegram]
enabled = true
bot_token = "123456:ABC"
allow_from = ["123456789"]

[channels.discord]
enabled = true
token = "discord-token"

[channels.discord.dm]
allow_from = ["123456789012345678"]
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let channels = manifest.channels.unwrap();

    let wa = channels.get("whatsapp").unwrap();
    assert_eq!(wa.dm_policy.as_deref(), Some("pairing"));
    assert_eq!(wa.auto_reply, Some(true));
    assert_eq!(wa.allow_from.as_ref().unwrap(), &["+15555550123"]);
    let groups = wa.groups.as_ref().unwrap();
    assert_eq!(groups["*"].require_mention, Some(true));

    let tg = channels.get("telegram").unwrap();
    assert_eq!(tg.enabled, Some(true));
    assert_eq!(tg.bot_token.as_deref(), Some("123456:ABC"));

    let dc = channels.get("discord").unwrap();
    assert_eq!(dc.token.as_deref(), Some("discord-token"));
    let dm = dc.dm.as_ref().unwrap();
    assert_eq!(dm.allow_from.as_ref().unwrap(), &["123456789012345678"]);
}

#[rstest]
fn test_parse_cron_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[cron]
enabled = true
max_concurrent_runs = 2
session_retention = "24h"

[cron.run_log]
max_bytes = "2mb"
keep_lines = 2000

[[cron.jobs]]
name = "check"
schedule = "0 * * * *"

[[cron.jobs]]
name = "backup"
schedule = "0 0 * * *"
agent = "ops"
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let cron = manifest.cron.unwrap();

    assert_eq!(cron.enabled, Some(true));
    assert_eq!(cron.max_concurrent_runs, Some(2));
    assert_eq!(cron.session_retention.as_deref(), Some("24h"));

    let run_log = cron.run_log.unwrap();
    assert_eq!(run_log.max_bytes.as_deref(), Some("2mb"));
    assert_eq!(run_log.keep_lines, Some(2000));

    let jobs = cron.jobs.unwrap();
    assert_eq!(jobs.len(), 2);
    assert_eq!(jobs[0].name, "check");
    assert!(jobs[0].agent.is_none());
    assert_eq!(jobs[1].name, "backup");
    assert_eq!(jobs[1].agent.as_deref(), Some("ops"));
}

#[rstest]
fn test_parse_hooks_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[hooks]
enabled = true
token = "shared-secret"
path = "/hooks"
max_body_bytes = 262144
pre_apply = ["echo pre-apply"]
post_apply = ["echo post-apply"]
pre_sync = ["echo pre-sync"]
post_sync = ["echo post-sync"]
allowed_agent_ids = ["hooks", "main"]
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let hooks = manifest.hooks.unwrap();

    assert_eq!(hooks.enabled, Some(true));
    assert_eq!(hooks.token.as_deref(), Some("shared-secret"));
    assert_eq!(hooks.path.as_deref(), Some("/hooks"));
    assert_eq!(hooks.max_body_bytes, Some(262144));
    assert_eq!(hooks.pre_apply.unwrap(), vec!["echo pre-apply"]);
    assert_eq!(hooks.post_apply.unwrap(), vec!["echo post-apply"]);
    assert_eq!(hooks.pre_sync.unwrap(), vec!["echo pre-sync"]);
    assert_eq!(hooks.post_sync.unwrap(), vec!["echo post-sync"]);
    assert_eq!(hooks.allowed_agent_ids.unwrap(), vec!["hooks", "main"]);
}

#[rstest]
fn test_parse_discovery_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[discovery.mdns]
mode = "minimal"

[discovery.wide_area]
enabled = true
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let discovery = manifest.discovery.unwrap();

    assert_eq!(
        discovery.mdns.as_ref().unwrap().mode.as_deref(),
        Some("minimal")
    );
    assert_eq!(discovery.wide_area.as_ref().unwrap().enabled, Some(true));
}

#[rstest]
fn test_parse_skills_section() {
    let toml_str = r#"
[meta]
schema_version = "1"

[skills.bundled]
enabled = ["developer", "computer"]

[skills.community]
allow = true
sources = ["https://skills.example.com"]

[[skills.entries]]
name = "my-tool"
source = "./tools/my-tool"
enabled = true
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let skills = manifest.skills.unwrap();

    let bundled = skills.bundled.unwrap();
    assert_eq!(bundled.enabled.unwrap(), vec!["developer", "computer"]);

    let community = skills.community.unwrap();
    assert_eq!(community.allow, Some(true));

    let entries = skills.entries.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "my-tool");
    assert_eq!(entries[0].enabled, Some(true));
}

// =============================================================================
// skip_serializing_if behavior
// =============================================================================

#[rstest]
fn test_skip_serializing_none_fields() {
    let manifest = Manifest {
        meta: Meta {
            schema_version: "1".to_string(),
            description: None,
        },
        gateway: None,
        models: None,
        agents: None,
        tools: None,
        bindings: None,
        channels: None,
        session: None,
        skills: None,
        cron: None,
        hooks: None,
        discovery: None,
        profiles: None,
        env: None,
    };

    let toml_str = toml::to_string_pretty(&manifest).unwrap();

    // None fields should not appear in output
    assert!(!toml_str.contains("gateway"));
    assert!(!toml_str.contains("agents"));
    assert!(!toml_str.contains("bindings"));
    assert!(!toml_str.contains("channels"));
    assert!(!toml_str.contains("skills"));
    assert!(!toml_str.contains("cron"));
    assert!(!toml_str.contains("hooks"));
    assert!(!toml_str.contains("models"));
    assert!(!toml_str.contains("tools"));
    assert!(!toml_str.contains("session"));
    assert!(!toml_str.contains("discovery"));
    assert!(!toml_str.contains("description"));

    // meta section should be present
    assert!(toml_str.contains("[meta]"));
    assert!(toml_str.contains("schema_version"));
}

// =============================================================================
// JSON serialization
// =============================================================================

#[rstest]
fn test_json_serialization() {
    let manifest = Manifest {
        meta: Meta {
            schema_version: "1".to_string(),
            description: Some("JSON test".to_string()),
        },
        gateway: Some(Gateway {
            mode: Some("local".to_string()),
            bind: None,
            port: None,
            auth: None,
            reload: None,
            tailscale: None,
        }),
        models: None,
        agents: None,
        tools: None,
        bindings: None,
        channels: None,
        session: None,
        skills: None,
        cron: None,
        hooks: None,
        discovery: None,
        profiles: None,
        env: None,
    };

    let json_str = serde_json::to_string_pretty(&manifest).unwrap();
    let parsed: Manifest = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.meta.schema_version, "1");
    assert_eq!(parsed.meta.description.as_deref(), Some("JSON test"));
    assert_eq!(parsed.gateway.unwrap().mode.as_deref(), Some("local"));
}

// =============================================================================
// Edge cases
// =============================================================================

#[rstest]
fn test_agent_with_no_optional_fields() {
    let agent = AgentDefinition {
        name: "minimal".to_string(),
        role: None,
        instructions: None,
        model: None,
        approval_mode: None,
        max_turns: None,
        workspace: None,
        subagents: None,
        sandbox: None,
        identity: None,
        tools: None,
        compaction: None,
        memory_search: None,
    };

    let toml_str = toml::to_string(&agent).unwrap();
    assert!(toml_str.contains("name = \"minimal\""));
    assert!(!toml_str.contains("role"));
    assert!(!toml_str.contains("instructions"));
    assert!(!toml_str.contains("workspace"));
    assert!(!toml_str.contains("tools"));
}

#[rstest]
fn test_empty_agents_list() {
    let toml_str = r#"
[meta]
schema_version = "1"

[agents]
"#;
    let manifest: Manifest = toml::from_str(toml_str).unwrap();
    let agents = manifest.agents.unwrap();
    assert!(agents.defaults.is_none());
    assert!(agents.list.is_none());
}

#[rstest]
fn test_binding_with_description() {
    let binding = Binding {
        pattern: Some("*.py".to_string()),
        agent: "python-agent".to_string(),
        description: Some("Python files handler".to_string()),
    };

    let toml_str = toml::to_string(&binding).unwrap();
    let parsed: Binding = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.pattern.as_deref(), Some("*.py"));
    assert_eq!(parsed.agent, "python-agent");
    assert_eq!(parsed.description.as_deref(), Some("Python files handler"));
}

#[rstest]
fn test_gateway_default() {
    let gw = Gateway::default();
    assert!(gw.mode.is_none());
    assert!(gw.bind.is_none());
    assert!(gw.port.is_none());
    assert!(gw.auth.is_none());
    assert!(gw.reload.is_none());
}

#[rstest]
fn test_sandbox_ssh_config() {
    let sandbox = SandboxConfig {
        enabled: None,
        mode: Some("non-main".to_string()),
        backend: Some("ssh".to_string()),
        scope: Some("agent".to_string()),
        workspace_access: Some("none".to_string()),
        workspace_root: Some("~/.openclaw/sandboxes".to_string()),
        permissions: None,
        ssh: Some(SandboxSshConfig {
            target: Some("user@host:22".to_string()),
            command: Some("ssh".to_string()),
            workspace_root: Some("/tmp/openclaw-sandboxes".to_string()),
            strict_host_key_checking: Some(true),
            update_host_keys: Some(true),
            identity_file: Some("~/.ssh/id_ed25519".to_string()),
            certificate_file: None,
            known_hosts_file: None,
        }),
        prune: Some(SandboxPruneConfig {
            idle_hours: Some(24),
            max_age_days: Some(7),
        }),
    };

    let toml_str = toml::to_string(&sandbox).unwrap();
    let parsed: SandboxConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.mode.as_deref(), Some("non-main"));
    assert_eq!(parsed.backend.as_deref(), Some("ssh"));
    let ssh = parsed.ssh.unwrap();
    assert_eq!(ssh.target.as_deref(), Some("user@host:22"));
    assert_eq!(ssh.strict_host_key_checking, Some(true));
    let prune = parsed.prune.unwrap();
    assert_eq!(prune.idle_hours, Some(24));
    assert_eq!(prune.max_age_days, Some(7));
}

#[rstest]
fn test_memory_search_roundtrip() {
    let mem = MemorySearchConfig {
        enabled: Some(true),
        provider: Some("local".to_string()),
        model: Some("all-MiniLM-L6-v2".to_string()),
        query: Some(MemorySearchQueryConfig {
            hybrid: Some(HybridSearchConfig {
                enabled: Some(true),
                vector_weight: Some(0.7),
                text_weight: Some(0.3),
            }),
        }),
        sync: Some(MemorySearchSyncConfig { watch: Some(true) }),
    };

    let toml_str = toml::to_string(&mem).unwrap();
    let parsed: MemorySearchConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.enabled, Some(true));
    assert_eq!(parsed.provider.as_deref(), Some("local"));
    let hybrid = parsed.query.unwrap().hybrid.unwrap();
    assert_eq!(hybrid.vector_weight, Some(0.7));
    assert_eq!(hybrid.text_weight, Some(0.3));
    assert_eq!(parsed.sync.unwrap().watch, Some(true));
}

#[rstest]
fn test_elevated_tools_with_allow_from() {
    let elevated = ToolsElevatedConfig {
        enabled: Some(true),
        allow_from: Some(HashMap::from([
            ("whatsapp".to_string(), vec!["+15555550123".to_string()]),
            (
                "discord".to_string(),
                vec!["1234567890".to_string(), "9876543210".to_string()],
            ),
        ])),
    };

    let toml_str = toml::to_string(&elevated).unwrap();
    let parsed: ToolsElevatedConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.enabled, Some(true));
    let allow_from = parsed.allow_from.unwrap();
    assert_eq!(allow_from["whatsapp"], vec!["+15555550123"]);
    assert_eq!(allow_from["discord"].len(), 2);
}

#[rstest]
fn test_hook_mapping_roundtrip() {
    let mapping = HookMapping {
        r#match: Some(HookMatchConfig {
            path: Some("gmail".to_string()),
        }),
        action: Some("agent".to_string()),
        agent_id: Some("hooks".to_string()),
        wake_mode: Some("now".to_string()),
        name: Some("Gmail Hook".to_string()),
        session_key: Some("hook:gmail:{{id}}".to_string()),
        message_template: Some("From: {{from}}".to_string()),
        deliver: Some(true),
        channel: Some("last".to_string()),
        model: Some("openai/gpt-5.2-mini".to_string()),
    };

    let toml_str = toml::to_string(&mapping).unwrap();
    let parsed: HookMapping = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.action.as_deref(), Some("agent"));
    assert_eq!(parsed.agent_id.as_deref(), Some("hooks"));
    assert_eq!(parsed.name.as_deref(), Some("Gmail Hook"));
    assert_eq!(parsed.deliver, Some(true));
    assert_eq!(parsed.model.as_deref(), Some("openai/gpt-5.2-mini"));
}
