use thiserror::Error;

/// Unified error type for clawup.
///
/// Some variants are used only by legacy modules that will be replaced
/// by clawup-core / clawup-ops. See AGENTS.md "Known Technical Debt".
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ClawupError {
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("OpenClaw directory not found at: {0}")]
    OpenClawNotFound(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}
