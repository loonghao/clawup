use std::path::{Path, PathBuf};

use crate::error::ClawupError;

/// Represents the detected OpenClaw installation paths.
#[derive(Debug, Clone)]
pub struct OpenClawPaths {
    root: PathBuf,
}

impl OpenClawPaths {
    /// Detect the OpenClaw installation directory.
    ///
    /// Searches in order:
    /// 1. `OPENCLAW_HOME` environment variable
    /// 2. `~/.openclaw/`
    pub fn detect() -> Result<Self, ClawupError> {
        // Check environment variable first
        if let Ok(home) = std::env::var("OPENCLAW_HOME") {
            let path = PathBuf::from(home);
            if path.exists() {
                return Ok(Self { root: path });
            }
        }

        // Fall back to default location
        if let Some(home) = dirs::home_dir() {
            let default_path = home.join(".openclaw");
            if default_path.exists() {
                return Ok(Self { root: default_path });
            }
            // Return the default path even if it doesn't exist yet
            // (for init/doctor commands)
            return Ok(Self { root: default_path });
        }

        Err(ClawupError::OpenClawNotFound(
            "Could not determine home directory".to_string(),
        ))
    }

    /// Get the root OpenClaw directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get the main configuration file path (`openclaw.json`).
    pub fn config_file(&self) -> PathBuf {
        self.root.join("openclaw.json")
    }

    /// Get the workspace directory.
    pub fn workspace_dir(&self) -> PathBuf {
        self.root.join("workspace")
    }

    /// Get the workspace directory for a specific agent.
    pub fn agent_workspace_dir(&self, agent: &str) -> PathBuf {
        self.root.join("agents").join(agent).join("workspace")
    }

    /// Get the skills directory.
    pub fn skills_dir(&self) -> PathBuf {
        self.root.join("skills")
    }

    /// Get the credentials directory.
    pub fn credentials_dir(&self) -> PathBuf {
        self.root.join("credentials")
    }

    /// Get the memory directory.
    pub fn memory_dir(&self) -> PathBuf {
        self.root.join("memory")
    }
}
