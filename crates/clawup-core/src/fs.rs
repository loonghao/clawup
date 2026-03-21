//! Filesystem utility functions.

use std::path::Path;

use crate::error::CoreError;

/// Ensure a directory exists, creating it if necessary.
pub fn ensure_dir(path: &Path) -> Result<(), CoreError> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Read a file to string, returning `None` if it doesn't exist.
pub fn read_optional(path: &Path) -> Result<Option<String>, CoreError> {
    if path.exists() {
        Ok(Some(std::fs::read_to_string(path)?))
    } else {
        Ok(None)
    }
}

/// Expand shell variables and tilde in a path string.
pub fn expand_path(path: &str) -> String {
    shellexpand::full(path)
        .map(|s| s.into_owned())
        .unwrap_or_else(|_| path.to_string())
}
