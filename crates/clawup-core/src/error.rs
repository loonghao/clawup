//! Core error types for clawup.

use thiserror::Error;

/// Core errors that can occur in fundamental operations.
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("OpenClaw directory not found at: {0}")]
    OpenClawNotFound(String),

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

/// Convenient Result alias for core operations.
pub type Result<T> = std::result::Result<T, CoreError>;
