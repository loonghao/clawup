//! Agent management module.
//!
//! Handles agent workspace scaffolding, subagent configuration,
//! and agent-specific file generation.

use clawup_core::paths::OpenClawPaths;
use clawup_schema::AgentDefinition;

use crate::error::Result;

/// Scaffold the workspace for an agent.
///
/// Creates the agent workspace directory and generates default
/// `SOUL.md` and optional `IDENTITY.md` files.
pub fn scaffold_workspace(paths: &OpenClawPaths, agent: &AgentDefinition) -> Result<()> {
    let workspace = paths.agent_workspace_dir(&agent.name);
    clawup_core::fs::ensure_dir(&workspace)?;

    // Create default soul files
    let soul_file = workspace.join("SOUL.md");
    if !soul_file.exists() {
        let content = generate_soul_content(agent);
        std::fs::write(&soul_file, content)?;
    }

    // Create identity file if identity config is provided
    if agent.identity.is_some() {
        let identity_file = workspace.join("IDENTITY.md");
        if !identity_file.exists() {
            let content = generate_identity_content(agent);
            std::fs::write(&identity_file, content)?;
        }
    }

    Ok(())
}

/// Generate SOUL.md content for an agent.
fn generate_soul_content(agent: &AgentDefinition) -> String {
    let role = agent.role.as_deref().unwrap_or("AI Assistant");
    let instructions = agent
        .instructions
        .as_deref()
        .unwrap_or("Follow best practices and write clean code.");

    format!("# Soul\n\nYou are a {role}.\n\n## Instructions\n\n{instructions}\n")
}

/// Generate IDENTITY.md content for an agent.
fn generate_identity_content(agent: &AgentDefinition) -> String {
    let identity = agent.identity.as_ref().unwrap();
    let name = identity.name.as_deref().unwrap_or(&agent.name);
    let persona = identity
        .persona
        .as_deref()
        .unwrap_or("a helpful AI assistant");

    format!("# Identity\n\nName: {name}\n\nYou are {persona}.\n")
}

/// Check if an agent's workspace exists and is properly set up.
pub fn is_workspace_valid(paths: &OpenClawPaths, agent_name: &str) -> bool {
    let workspace = paths.agent_workspace_dir(agent_name);
    workspace.exists() && workspace.join("SOUL.md").exists()
}

/// List all agent workspace directories.
pub fn list_workspaces(paths: &OpenClawPaths) -> Result<Vec<String>> {
    let agents_dir = paths.root().join("agents");
    if !agents_dir.exists() {
        return Ok(vec![]);
    }

    let mut names = vec![];
    for entry in std::fs::read_dir(&agents_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir()
            && let Some(name) = entry.file_name().to_str()
        {
            names.push(name.to_string());
        }
    }
    Ok(names)
}
