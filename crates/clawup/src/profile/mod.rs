//! Profile management module.
//!
//! Handles switching between configuration profiles (dev, staging, production, etc.).

use std::path::Path;

use color_eyre::Result;

use crate::error::ClawupError;

/// Profile state file name.
const PROFILE_STATE_FILE: &str = ".clawup-profile";

/// Get the currently active profile name.
pub fn current_profile(config_dir: &Path) -> Result<Option<String>> {
    let state_file = config_dir.join(PROFILE_STATE_FILE);
    if state_file.exists() {
        let content = std::fs::read_to_string(&state_file)?;
        let name = content.trim().to_string();
        if name.is_empty() {
            Ok(None)
        } else {
            Ok(Some(name))
        }
    } else {
        Ok(None)
    }
}

/// Set the active profile.
pub fn set_current_profile(config_dir: &Path, name: &str) -> Result<()> {
    let state_file = config_dir.join(PROFILE_STATE_FILE);
    std::fs::write(&state_file, name)?;
    Ok(())
}

/// Clear the active profile (revert to default).
pub fn clear_profile(config_dir: &Path) -> Result<()> {
    let state_file = config_dir.join(PROFILE_STATE_FILE);
    if state_file.exists() {
        std::fs::remove_file(&state_file)?;
    }
    Ok(())
}
