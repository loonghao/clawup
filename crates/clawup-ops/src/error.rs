//! Error types for the ops crate.

use thiserror::Error;

/// Errors that can occur during high-level operations.
#[derive(Error, Debug)]
pub enum OpsError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Template error: {0}")]
    Template(#[from] tera::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),

    #[error("Strip prefix error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),

    #[error("Core error: {0}")]
    Core(#[from] clawup_core::CoreError),

    #[error("{0}")]
    Other(String),
}

/// Convenient Result alias for ops operations.
pub type Result<T> = std::result::Result<T, OpsError>;
