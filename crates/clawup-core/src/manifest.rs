//! Manifest loading, saving, and CRUD operations.

use std::path::Path;

use clawup_schema::*;

use crate::error::CoreError;

/// Extension trait providing I/O and CRUD operations on [`Manifest`].
pub trait ManifestOps {
    /// Load a manifest from a TOML file.
    fn load(path: impl AsRef<Path>) -> Result<Manifest, CoreError>;

    /// Save the manifest to a TOML file.
    fn save(&self, path: impl AsRef<Path>) -> Result<(), CoreError>;

    // --- Agent operations ---

    /// Find an agent by name.
    fn find_agent(&self, name: &str) -> Result<&AgentDefinition, CoreError>;

    /// Add a new agent.
    fn add_agent(
        &mut self,
        name: &str,
        role: Option<&str>,
        model: Option<&str>,
    ) -> Result<(), CoreError>;

    /// Remove an agent by name.
    fn remove_agent(&mut self, name: &str) -> Result<(), CoreError>;

    /// Set a property on an agent.
    fn set_agent_property(&mut self, name: &str, key: &str, value: &str) -> Result<(), CoreError>;

    // --- Skill operations ---

    /// Add a skill entry.
    fn add_skill(&mut self, name: &str, source: Option<&str>) -> Result<(), CoreError>;

    /// Remove a skill entry.
    fn remove_skill(&mut self, name: &str) -> Result<(), CoreError>;

    /// Enable or disable a skill.
    fn toggle_skill(&mut self, name: &str, enabled: bool) -> Result<(), CoreError>;

    // --- Config value operations ---

    /// Get a value by dot-notation key.
    fn get_value(&self, key: &str) -> Result<String, CoreError>;

    /// Set a value by dot-notation key.
    fn set_value(&mut self, key: &str, value: &str) -> Result<(), CoreError>;
}

impl ManifestOps for Manifest {
    fn load(path: impl AsRef<Path>) -> Result<Manifest, CoreError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(CoreError::ConfigNotFound(path.display().to_string()));
        }
        let content = std::fs::read_to_string(path)?;
        let manifest: Manifest = toml::from_str(&content)?;
        Ok(manifest)
    }

    fn save(&self, path: impl AsRef<Path>) -> Result<(), CoreError> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    // --- Agent operations ---

    fn find_agent(&self, name: &str) -> Result<&AgentDefinition, CoreError> {
        self.agents
            .as_ref()
            .and_then(|a| a.list.as_ref())
            .and_then(|list| list.iter().find(|a| a.name == name))
            .ok_or_else(|| CoreError::Other(format!("Agent not found: {}", name)))
    }

    fn add_agent(
        &mut self,
        name: &str,
        role: Option<&str>,
        model: Option<&str>,
    ) -> Result<(), CoreError> {
        let agents = self.agents.get_or_insert_with(|| AgentsConfig {
            defaults: None,
            list: Some(vec![]),
        });
        let list = agents.list.get_or_insert_with(Vec::new);

        if list.iter().any(|a| a.name == name) {
            return Err(CoreError::Other(format!("Agent '{}' already exists", name)));
        }

        list.push(AgentDefinition {
            name: name.to_string(),
            role: role.map(|s| s.to_string()),
            instructions: None,
            model: model.map(|s| s.to_string()),
            approval_mode: None,
            max_turns: None,
            workspace: None,
            subagents: None,
            sandbox: None,
            identity: None,
            tools: None,
            compaction: None,
            memory_search: None,
        });

        Ok(())
    }

    fn remove_agent(&mut self, name: &str) -> Result<(), CoreError> {
        let list = self
            .agents
            .as_mut()
            .and_then(|a| a.list.as_mut())
            .ok_or_else(|| CoreError::Other(format!("Agent not found: {}", name)))?;

        let pos = list
            .iter()
            .position(|a| a.name == name)
            .ok_or_else(|| CoreError::Other(format!("Agent not found: {}", name)))?;

        list.remove(pos);
        Ok(())
    }

    fn set_agent_property(&mut self, name: &str, key: &str, value: &str) -> Result<(), CoreError> {
        let list = self
            .agents
            .as_mut()
            .and_then(|a| a.list.as_mut())
            .ok_or_else(|| CoreError::Other(format!("Agent not found: {}", name)))?;

        let agent = list
            .iter_mut()
            .find(|a| a.name == name)
            .ok_or_else(|| CoreError::Other(format!("Agent not found: {}", name)))?;

        match key {
            "role" => agent.role = Some(value.to_string()),
            "model" => agent.model = Some(value.to_string()),
            "approval_mode" => agent.approval_mode = Some(value.to_string()),
            "instructions" => agent.instructions = Some(value.to_string()),
            "workspace" => agent.workspace = Some(value.to_string()),
            _ => {
                return Err(CoreError::Other(format!("Unknown agent property: {}", key)));
            }
        }

        Ok(())
    }

    // --- Skill operations ---

    fn add_skill(&mut self, name: &str, source: Option<&str>) -> Result<(), CoreError> {
        let skills = self.skills.get_or_insert_with(|| SkillsConfig {
            bundled: None,
            community: None,
            entries: Some(vec![]),
        });
        let entries = skills.entries.get_or_insert_with(Vec::new);

        if entries.iter().any(|s| s.name == name) {
            return Err(CoreError::Other(format!("Skill '{}' already exists", name)));
        }

        entries.push(SkillEntry {
            name: name.to_string(),
            source: source.map(|s| s.to_string()),
            enabled: Some(true),
            config: None,
        });

        Ok(())
    }

    fn remove_skill(&mut self, name: &str) -> Result<(), CoreError> {
        let entries = self
            .skills
            .as_mut()
            .and_then(|s| s.entries.as_mut())
            .ok_or_else(|| CoreError::Other(format!("Skill not found: {}", name)))?;

        let pos = entries
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| CoreError::Other(format!("Skill not found: {}", name)))?;

        entries.remove(pos);
        Ok(())
    }

    fn toggle_skill(&mut self, name: &str, enabled: bool) -> Result<(), CoreError> {
        let entries = self
            .skills
            .as_mut()
            .and_then(|s| s.entries.as_mut())
            .ok_or_else(|| CoreError::Other(format!("Skill not found: {}", name)))?;

        let entry = entries
            .iter_mut()
            .find(|s| s.name == name)
            .ok_or_else(|| CoreError::Other(format!("Skill not found: {}", name)))?;

        entry.enabled = Some(enabled);
        Ok(())
    }

    // --- Config value operations ---

    fn get_value(&self, key: &str) -> Result<String, CoreError> {
        let toml_value = toml::Value::try_from(self)?;
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &toml_value;

        for part in &parts {
            current = current
                .get(part)
                .ok_or_else(|| CoreError::Other(format!("Key '{}' not found", key)))?;
        }

        Ok(match current {
            toml::Value::String(s) => s.clone(),
            other => other.to_string(),
        })
    }

    fn set_value(&mut self, key: &str, value: &str) -> Result<(), CoreError> {
        let parts: Vec<&str> = key.split('.').collect();
        match parts.as_slice() {
            ["meta", "description"] => {
                self.meta.description = Some(value.to_string());
            }
            ["gateway", "mode"] => {
                self.gateway.get_or_insert_with(Default::default).mode = Some(value.to_string());
            }
            ["gateway", "bind"] => {
                self.gateway.get_or_insert_with(Default::default).bind = Some(value.to_string());
            }
            _ => {
                return Err(CoreError::Other(format!(
                    "Setting key '{}' is not yet supported",
                    key
                )));
            }
        }
        Ok(())
    }
}
