//! Tests for `clawup_core::manifest` module (ManifestOps trait).

use assert_fs::TempDir;
use assert_fs::prelude::*;
use rstest::rstest;

use clawup_core::manifest::ManifestOps;
use clawup_schema::*;

// =============================================================================
// Helpers
// =============================================================================

/// Create a minimal valid Manifest for testing.
fn minimal_manifest() -> Manifest {
    Manifest {
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
    }
}

/// Create a Manifest with a single "code" agent.
fn manifest_with_agent() -> Manifest {
    let mut m = minimal_manifest();
    m.agents = Some(AgentsConfig {
        defaults: None,
        list: Some(vec![AgentDefinition {
            name: "code".to_string(),
            role: Some("Engineer".to_string()),
            instructions: Some("Write clean code".to_string()),
            model: Some("gpt-4".to_string()),
            approval_mode: None,
            max_turns: None,
            workspace: None,
            subagents: None,
            sandbox: None,
            identity: None,
            tools: None,
            compaction: None,
            memory_search: None,
        }]),
    });
    m
}

/// Create a Manifest with a single "linter" skill.
fn manifest_with_skill() -> Manifest {
    let mut m = minimal_manifest();
    m.skills = Some(SkillsConfig {
        bundled: None,
        community: None,
        entries: Some(vec![SkillEntry {
            name: "linter".to_string(),
            source: Some("https://example.com/linter".to_string()),
            enabled: Some(true),
            config: None,
        }]),
    });
    m
}

/// Minimal TOML string for loading tests.
const MINIMAL_TOML: &str = r#"
[meta]
schema_version = "1"
"#;

/// TOML with an agent defined.
const TOML_WITH_AGENT: &str = r#"
[meta]
schema_version = "1"
description = "Test manifest"

[[agents.list]]
name = "code"
role = "Engineer"
instructions = "Write clean code"
model = "gpt-4"
"#;

// =============================================================================
// Load / Save tests
// =============================================================================

#[rstest]
fn load_minimal_toml() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("clawup.toml");
    file.write_str(MINIMAL_TOML).unwrap();

    let manifest = Manifest::load(file.path()).unwrap();
    assert_eq!(manifest.meta.schema_version, "1");
    assert!(manifest.agents.is_none());
}

#[rstest]
fn load_toml_with_agent() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("clawup.toml");
    file.write_str(TOML_WITH_AGENT).unwrap();

    let manifest = Manifest::load(file.path()).unwrap();
    assert_eq!(manifest.meta.description.as_deref(), Some("Test manifest"));

    let agent = manifest.find_agent("code").unwrap();
    assert_eq!(agent.role.as_deref(), Some("Engineer"));
    assert_eq!(agent.model.as_deref(), Some("gpt-4"));
}

#[rstest]
fn load_returns_error_for_missing_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("nonexistent.toml");

    let result = Manifest::load(file.path());
    assert!(result.is_err());
}

#[rstest]
fn load_returns_error_for_invalid_toml() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("bad.toml");
    file.write_str("this is not valid toml {{{{").unwrap();

    let result = Manifest::load(file.path());
    assert!(result.is_err());
}

#[rstest]
fn save_and_reload_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("clawup.toml");

    let mut manifest = minimal_manifest();
    manifest.meta.description = Some("Roundtrip test".to_string());
    manifest.save(file.path()).unwrap();

    let reloaded = Manifest::load(file.path()).unwrap();
    assert_eq!(reloaded.meta.schema_version, "1");
    assert_eq!(reloaded.meta.description.as_deref(), Some("Roundtrip test"));
}

#[rstest]
fn save_roundtrip_preserves_agents() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("clawup.toml");

    let manifest = manifest_with_agent();
    manifest.save(file.path()).unwrap();

    let reloaded = Manifest::load(file.path()).unwrap();
    let agent = reloaded.find_agent("code").unwrap();
    assert_eq!(agent.name, "code");
    assert_eq!(agent.role.as_deref(), Some("Engineer"));
}

#[rstest]
fn save_roundtrip_preserves_skills() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("clawup.toml");

    let manifest = manifest_with_skill();
    manifest.save(file.path()).unwrap();

    let reloaded = Manifest::load(file.path()).unwrap();
    let entries = reloaded.skills.unwrap().entries.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "linter");
    assert_eq!(entries[0].enabled, Some(true));
}

// =============================================================================
// Agent CRUD tests
// =============================================================================

#[rstest]
fn find_agent_returns_matching_agent() {
    let manifest = manifest_with_agent();
    let agent = manifest.find_agent("code").unwrap();
    assert_eq!(agent.name, "code");
}

#[rstest]
fn find_agent_returns_error_for_unknown_name() {
    let manifest = manifest_with_agent();
    let result = manifest.find_agent("nonexistent");
    assert!(result.is_err());
}

#[rstest]
fn find_agent_returns_error_when_no_agents() {
    let manifest = minimal_manifest();
    let result = manifest.find_agent("code");
    assert!(result.is_err());
}

#[rstest]
fn add_agent_to_empty_manifest() {
    let mut manifest = minimal_manifest();

    manifest
        .add_agent("review", Some("Reviewer"), Some("gpt-4"))
        .unwrap();

    let agent = manifest.find_agent("review").unwrap();
    assert_eq!(agent.name, "review");
    assert_eq!(agent.role.as_deref(), Some("Reviewer"));
    assert_eq!(agent.model.as_deref(), Some("gpt-4"));
}

#[rstest]
fn add_agent_with_none_fields() {
    let mut manifest = minimal_manifest();

    manifest.add_agent("minimal", None, None).unwrap();

    let agent = manifest.find_agent("minimal").unwrap();
    assert_eq!(agent.name, "minimal");
    assert!(agent.role.is_none());
    assert!(agent.model.is_none());
}

#[rstest]
fn add_agent_to_existing_list() {
    let mut manifest = manifest_with_agent();

    manifest
        .add_agent("review", Some("Reviewer"), None)
        .unwrap();

    // Both agents should exist
    assert!(manifest.find_agent("code").is_ok());
    assert!(manifest.find_agent("review").is_ok());

    let list = manifest.agents.unwrap().list.unwrap();
    assert_eq!(list.len(), 2);
}

#[rstest]
fn add_agent_rejects_duplicate_name() {
    let mut manifest = manifest_with_agent();

    let result = manifest.add_agent("code", Some("Duplicate"), None);
    assert!(result.is_err());
}

#[rstest]
fn remove_agent_succeeds() {
    let mut manifest = manifest_with_agent();

    manifest.remove_agent("code").unwrap();

    let list = manifest.agents.unwrap().list.unwrap();
    assert!(list.is_empty());
}

#[rstest]
fn remove_agent_returns_error_for_unknown_name() {
    let mut manifest = manifest_with_agent();

    let result = manifest.remove_agent("nonexistent");
    assert!(result.is_err());
}

#[rstest]
fn remove_agent_returns_error_when_no_agents() {
    let mut manifest = minimal_manifest();

    let result = manifest.remove_agent("code");
    assert!(result.is_err());
}

#[rstest]
fn set_agent_property_role() {
    let mut manifest = manifest_with_agent();

    manifest
        .set_agent_property("code", "role", "Senior Engineer")
        .unwrap();

    let agent = manifest.find_agent("code").unwrap();
    assert_eq!(agent.role.as_deref(), Some("Senior Engineer"));
}

#[rstest]
fn set_agent_property_model() {
    let mut manifest = manifest_with_agent();

    manifest
        .set_agent_property("code", "model", "claude-4")
        .unwrap();

    let agent = manifest.find_agent("code").unwrap();
    assert_eq!(agent.model.as_deref(), Some("claude-4"));
}

#[rstest]
fn set_agent_property_approval_mode() {
    let mut manifest = manifest_with_agent();

    manifest
        .set_agent_property("code", "approval_mode", "suggest")
        .unwrap();

    let agent = manifest.find_agent("code").unwrap();
    assert_eq!(agent.approval_mode.as_deref(), Some("suggest"));
}

#[rstest]
fn set_agent_property_instructions() {
    let mut manifest = manifest_with_agent();

    manifest
        .set_agent_property("code", "instructions", "Write tests first")
        .unwrap();

    let agent = manifest.find_agent("code").unwrap();
    assert_eq!(agent.instructions.as_deref(), Some("Write tests first"));
}

#[rstest]
fn set_agent_property_workspace() {
    let mut manifest = manifest_with_agent();

    manifest
        .set_agent_property("code", "workspace", "/custom/workspace")
        .unwrap();

    let agent = manifest.find_agent("code").unwrap();
    assert_eq!(agent.workspace.as_deref(), Some("/custom/workspace"));
}

#[rstest]
fn set_agent_property_rejects_unknown_key() {
    let mut manifest = manifest_with_agent();

    let result = manifest.set_agent_property("code", "unknown_field", "value");
    assert!(result.is_err());
}

#[rstest]
fn set_agent_property_returns_error_for_unknown_agent() {
    let mut manifest = manifest_with_agent();

    let result = manifest.set_agent_property("nonexistent", "role", "Test");
    assert!(result.is_err());
}

// =============================================================================
// Skill CRUD tests
// =============================================================================

#[rstest]
fn add_skill_to_empty_manifest() {
    let mut manifest = minimal_manifest();

    manifest
        .add_skill("formatter", Some("https://example.com/fmt"))
        .unwrap();

    let entries = manifest.skills.unwrap().entries.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name, "formatter");
    assert_eq!(
        entries[0].source.as_deref(),
        Some("https://example.com/fmt")
    );
    assert_eq!(entries[0].enabled, Some(true));
}

#[rstest]
fn add_skill_without_source() {
    let mut manifest = minimal_manifest();

    manifest.add_skill("builtin-tool", None).unwrap();

    let entries = manifest.skills.unwrap().entries.unwrap();
    assert_eq!(entries[0].name, "builtin-tool");
    assert!(entries[0].source.is_none());
}

#[rstest]
fn add_skill_to_existing_list() {
    let mut manifest = manifest_with_skill();

    manifest.add_skill("formatter", None).unwrap();

    let entries = manifest.skills.unwrap().entries.unwrap();
    assert_eq!(entries.len(), 2);
}

#[rstest]
fn add_skill_rejects_duplicate_name() {
    let mut manifest = manifest_with_skill();

    let result = manifest.add_skill("linter", None);
    assert!(result.is_err());
}

#[rstest]
fn remove_skill_succeeds() {
    let mut manifest = manifest_with_skill();

    manifest.remove_skill("linter").unwrap();

    let entries = manifest.skills.unwrap().entries.unwrap();
    assert!(entries.is_empty());
}

#[rstest]
fn remove_skill_returns_error_for_unknown_name() {
    let mut manifest = manifest_with_skill();

    let result = manifest.remove_skill("nonexistent");
    assert!(result.is_err());
}

#[rstest]
fn remove_skill_returns_error_when_no_skills() {
    let mut manifest = minimal_manifest();

    let result = manifest.remove_skill("linter");
    assert!(result.is_err());
}

#[rstest]
fn toggle_skill_disable() {
    let mut manifest = manifest_with_skill();

    manifest.toggle_skill("linter", false).unwrap();

    let entries = manifest.skills.unwrap().entries.unwrap();
    assert_eq!(entries[0].enabled, Some(false));
}

#[rstest]
fn toggle_skill_enable() {
    let mut manifest = manifest_with_skill();
    // Disable first, then re-enable
    manifest.toggle_skill("linter", false).unwrap();
    manifest.toggle_skill("linter", true).unwrap();

    let entries = manifest.skills.unwrap().entries.unwrap();
    assert_eq!(entries[0].enabled, Some(true));
}

#[rstest]
fn toggle_skill_returns_error_for_unknown_skill() {
    let mut manifest = manifest_with_skill();

    let result = manifest.toggle_skill("nonexistent", true);
    assert!(result.is_err());
}

// =============================================================================
// Config value get/set tests
// =============================================================================

#[rstest]
fn get_value_reads_schema_version() {
    let manifest = minimal_manifest();

    let value = manifest.get_value("meta.schema_version").unwrap();
    assert_eq!(value, "1");
}

#[rstest]
fn get_value_reads_meta_description() {
    let mut manifest = minimal_manifest();
    manifest.meta.description = Some("My project".to_string());

    let value = manifest.get_value("meta.description").unwrap();
    assert_eq!(value, "My project");
}

#[rstest]
fn get_value_returns_error_for_missing_key() {
    let manifest = minimal_manifest();

    let result = manifest.get_value("nonexistent.key");
    assert!(result.is_err());
}

#[rstest]
fn set_value_meta_description() {
    let mut manifest = minimal_manifest();

    manifest.set_value("meta.description", "Updated").unwrap();

    assert_eq!(manifest.meta.description.as_deref(), Some("Updated"));
}

#[rstest]
fn set_value_gateway_mode() {
    let mut manifest = minimal_manifest();
    assert!(manifest.gateway.is_none());

    manifest.set_value("gateway.mode", "local").unwrap();

    assert_eq!(
        manifest.gateway.as_ref().unwrap().mode.as_deref(),
        Some("local")
    );
}

#[rstest]
fn set_value_gateway_bind() {
    let mut manifest = minimal_manifest();

    manifest.set_value("gateway.bind", "loopback").unwrap();

    assert_eq!(
        manifest.gateway.as_ref().unwrap().bind.as_deref(),
        Some("loopback")
    );
}

#[rstest]
fn set_value_returns_error_for_unsupported_key() {
    let mut manifest = minimal_manifest();

    let result = manifest.set_value("unsupported.key", "value");
    assert!(result.is_err());
}
