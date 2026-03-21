mod schema;

pub use schema::*;

use color_eyre::Result;
use std::path::Path;

use crate::error::ClawupError;

impl Manifest {
    /// Load a manifest from a TOML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(ClawupError::ConfigNotFound(path.display().to_string()).into());
        }
        let content = std::fs::read_to_string(path)?;
        let manifest: Manifest = toml::from_str(&content)?;
        Ok(manifest)
    }

    /// Save the manifest to a TOML file.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create a manifest from a built-in template.
    pub fn from_template(template: &str, description: &str) -> Result<Self> {
        let mut manifest = match template {
            "multi-agent" => Self::multi_agent_template(),
            "team" => Self::team_template(),
            _ => Self::default_template(),
        };
        manifest.meta.description = Some(description.to_string());
        Ok(manifest)
    }

    fn default_template() -> Self {
        Self {
            meta: Meta {
                schema_version: "1".to_string(),
                description: None,
            },
            gateway: Some(Gateway {
                mode: Some("local".to_string()),
                bind: Some("loopback".to_string()),
                port: None,
                auth: None,
                reload: None,
                tailscale: None,
            }),
            models: None,
            agents: Some(AgentsConfig {
                defaults: Some(AgentDefaults {
                    model: Some("anthropic/claude-sonnet-4".to_string()),
                    approval_mode: Some("auto-edit".to_string()),
                    max_turns: None,
                    workspace: None,
                    compaction: None,
                    memory_search: None,
                    sandbox: None,
                }),
                list: Some(vec![AgentDefinition {
                    name: "code".to_string(),
                    role: Some("Software Engineer".to_string()),
                    instructions: Some("Write clean, well-tested code".to_string()),
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
                }]),
            }),
            tools: None,
            bindings: None,
            channels: None,
            session: None,
            skills: Some(SkillsConfig {
                bundled: Some(BundledSkills {
                    enabled: Some(vec!["developer".to_string(), "computer".to_string()]),
                }),
                community: None,
                entries: None,
            }),
            cron: None,
            hooks: None,
            discovery: None,
            profiles: None,
            env: None,
        }
    }

    fn multi_agent_template() -> Self {
        Self {
            meta: Meta {
                schema_version: "1".to_string(),
                description: None,
            },
            gateway: Some(Gateway {
                mode: Some("local".to_string()),
                bind: Some("loopback".to_string()),
                port: None,
                auth: None,
                reload: None,
                tailscale: None,
            }),
            models: None,
            agents: Some(AgentsConfig {
                defaults: Some(AgentDefaults {
                    model: Some("anthropic/claude-sonnet-4".to_string()),
                    approval_mode: Some("auto-edit".to_string()),
                    max_turns: None,
                    workspace: None,
                    compaction: None,
                    memory_search: None,
                    sandbox: None,
                }),
                list: Some(vec![
                    AgentDefinition {
                        name: "code".to_string(),
                        role: Some("Senior Software Engineer".to_string()),
                        instructions: Some("Focus on clean, tested code with solid architecture".to_string()),
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
                    },
                    AgentDefinition {
                        name: "review".to_string(),
                        role: Some("Code Reviewer".to_string()),
                        instructions: Some("Review PRs for correctness, style, and security".to_string()),
                        model: None,
                        approval_mode: Some("suggest".to_string()),
                        max_turns: None,
                        workspace: None,
                        subagents: None,
                        sandbox: None,
                        identity: None,
                        tools: None,
                        compaction: None,
                        memory_search: None,
                    },
                    AgentDefinition {
                        name: "ops".to_string(),
                        role: Some("DevOps Engineer".to_string()),
                        instructions: Some("Manage CI/CD, infrastructure, and deployments".to_string()),
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
                    },
                ]),
            }),
            tools: None,
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
            session: None,
            skills: Some(SkillsConfig {
                bundled: Some(BundledSkills {
                    enabled: Some(vec!["developer".to_string(), "computer".to_string()]),
                }),
                community: None,
                entries: None,
            }),
            cron: None,
            hooks: None,
            discovery: None,
            profiles: None,
            env: None,
        }
    }

    fn team_template() -> Self {
        let mut manifest = Self::multi_agent_template();
        manifest.profiles = Some(
            [
                ("dev".to_string(), toml::Value::Table({
                    let mut t = toml::map::Map::new();
                    t.insert("description".to_string(), toml::Value::String("Development environment".to_string()));
                    t
                })),
                ("staging".to_string(), toml::Value::Table({
                    let mut t = toml::map::Map::new();
                    t.insert("description".to_string(), toml::Value::String("Staging environment".to_string()));
                    t
                })),
                ("production".to_string(), toml::Value::Table({
                    let mut t = toml::map::Map::new();
                    t.insert("description".to_string(), toml::Value::String("Production environment".to_string()));
                    t
                })),
            ]
            .into_iter()
            .collect(),
        );
        manifest
    }

    // --- Agent operations ---

    /// Find an agent by name.
    pub fn find_agent(&self, name: &str) -> Result<&AgentDefinition> {
        self.agents
            .as_ref()
            .and_then(|a| a.list.as_ref())
            .and_then(|list| list.iter().find(|a| a.name == name))
            .ok_or_else(|| ClawupError::AgentNotFound(name.to_string()).into())
    }

    /// Add a new agent.
    pub fn add_agent(&mut self, name: &str, role: Option<&str>, model: Option<&str>) -> Result<()> {
        let agents = self.agents.get_or_insert_with(|| AgentsConfig {
            defaults: None,
            list: Some(vec![]),
        });
        let list = agents.list.get_or_insert_with(Vec::new);

        if list.iter().any(|a| a.name == name) {
            return Err(ClawupError::Other(format!("Agent '{}' already exists", name)).into());
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

    /// Remove an agent by name.
    pub fn remove_agent(&mut self, name: &str) -> Result<()> {
        let list = self
            .agents
            .as_mut()
            .and_then(|a| a.list.as_mut())
            .ok_or_else(|| ClawupError::AgentNotFound(name.to_string()))?;

        let pos = list
            .iter()
            .position(|a| a.name == name)
            .ok_or_else(|| ClawupError::AgentNotFound(name.to_string()))?;

        list.remove(pos);
        Ok(())
    }

    /// Set a property on an agent.
    pub fn set_agent_property(&mut self, name: &str, key: &str, value: &str) -> Result<()> {
        let list = self
            .agents
            .as_mut()
            .and_then(|a| a.list.as_mut())
            .ok_or_else(|| ClawupError::AgentNotFound(name.to_string()))?;

        let agent = list
            .iter_mut()
            .find(|a| a.name == name)
            .ok_or_else(|| ClawupError::AgentNotFound(name.to_string()))?;

        match key {
            "role" => agent.role = Some(value.to_string()),
            "model" => agent.model = Some(value.to_string()),
            "approval_mode" => agent.approval_mode = Some(value.to_string()),
            "instructions" => agent.instructions = Some(value.to_string()),
            "workspace" => agent.workspace = Some(value.to_string()),
            _ => {
                return Err(
                    ClawupError::Other(format!("Unknown agent property: {}", key)).into(),
                )
            }
        }

        Ok(())
    }

    // --- Skill operations ---

    /// Add a skill entry.
    pub fn add_skill(&mut self, name: &str, source: Option<&str>) -> Result<()> {
        let skills = self.skills.get_or_insert_with(|| SkillsConfig {
            bundled: None,
            community: None,
            entries: Some(vec![]),
        });
        let entries = skills.entries.get_or_insert_with(Vec::new);

        if entries.iter().any(|s| s.name == name) {
            return Err(ClawupError::Other(format!("Skill '{}' already exists", name)).into());
        }

        entries.push(SkillEntry {
            name: name.to_string(),
            source: source.map(|s| s.to_string()),
            enabled: Some(true),
            config: None,
        });

        Ok(())
    }

    /// Remove a skill entry.
    pub fn remove_skill(&mut self, name: &str) -> Result<()> {
        let entries = self
            .skills
            .as_mut()
            .and_then(|s| s.entries.as_mut())
            .ok_or_else(|| ClawupError::SkillNotFound(name.to_string()))?;

        let pos = entries
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| ClawupError::SkillNotFound(name.to_string()))?;

        entries.remove(pos);
        Ok(())
    }

    /// Enable or disable a skill.
    pub fn toggle_skill(&mut self, name: &str, enabled: bool) -> Result<()> {
        let entries = self
            .skills
            .as_mut()
            .and_then(|s| s.entries.as_mut())
            .ok_or_else(|| ClawupError::SkillNotFound(name.to_string()))?;

        let entry = entries
            .iter_mut()
            .find(|s| s.name == name)
            .ok_or_else(|| ClawupError::SkillNotFound(name.to_string()))?;

        entry.enabled = Some(enabled);
        Ok(())
    }

    // --- Config value operations ---

    /// Get a value by dot-notation key.
    pub fn get_value(&self, key: &str) -> Result<String> {
        let toml_value = toml::Value::try_from(self)?;
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &toml_value;

        for part in &parts {
            current = current
                .get(part)
                .ok_or_else(|| ClawupError::Other(format!("Key '{}' not found", key)))?;
        }

        Ok(match current {
            toml::Value::String(s) => s.clone(),
            other => other.to_string(),
        })
    }

    /// Set a value by dot-notation key.
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        let parts: Vec<&str> = key.split('.').collect();
        match parts.as_slice() {
            ["meta", "description"] => {
                self.meta.description = Some(value.to_string());
            }
            ["gateway", "mode"] => {
                self.gateway.get_or_insert_with(Default::default).mode =
                    Some(value.to_string());
            }
            ["gateway", "bind"] => {
                self.gateway.get_or_insert_with(Default::default).bind =
                    Some(value.to_string());
            }
            ["gateway", "port"] => {
                let port: u16 = value.parse().map_err(|_| {
                    ClawupError::Other(format!("Invalid port value: {}", value))
                })?;
                self.gateway.get_or_insert_with(Default::default).port = Some(port);
            }
            _ => {
                return Err(
                    ClawupError::Other(format!("Setting key '{}' is not yet supported", key))
                        .into(),
                );
            }
        }
        Ok(())
    }
}
