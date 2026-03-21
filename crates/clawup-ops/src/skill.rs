//! Skill management module.
//!
//! Handles skill installation, configuration, and registry operations.

use std::path::Path;

use clawup_core::paths::OpenClawPaths;
use clawup_schema::SkillEntry;

use crate::error::Result;

/// Install a skill from a source path or URL.
pub fn install(paths: &OpenClawPaths, entry: &SkillEntry) -> Result<()> {
    let skills_dir = paths.skills_dir();
    clawup_core::fs::ensure_dir(&skills_dir)?;

    let skill_dir = skills_dir.join(&entry.name);
    clawup_core::fs::ensure_dir(&skill_dir)?;

    if let Some(ref source) = entry.source {
        let source_path = clawup_core::fs::expand_path(source);
        let source_path = Path::new(&source_path);

        if source_path.exists() {
            // Local source — copy files
            copy_skill_files(source_path, &skill_dir)?;
        } else {
            // Could be a URL or git repo — placeholder for future implementation
            tracing::warn!("Remote skill sources are not yet supported: {}", source);
        }
    }

    Ok(())
}

/// Copy skill files from source to destination.
fn copy_skill_files(from: &Path, to: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(from).min_depth(1) {
        let entry = entry?;
        let rel = entry.path().strip_prefix(from)?;
        let dest = to.join(rel);

        if entry.file_type().is_dir() {
            clawup_core::fs::ensure_dir(&dest)?;
        } else {
            if let Some(parent) = dest.parent() {
                clawup_core::fs::ensure_dir(parent)?;
            }
            std::fs::copy(entry.path(), &dest)?;
        }
    }
    Ok(())
}

/// Check if a skill is installed.
pub fn is_installed(paths: &OpenClawPaths, name: &str) -> bool {
    paths.skills_dir().join(name).exists()
}

/// Uninstall a skill by removing its directory.
pub fn uninstall(paths: &OpenClawPaths, name: &str) -> Result<()> {
    let skill_dir = paths.skills_dir().join(name);
    if skill_dir.exists() {
        std::fs::remove_dir_all(&skill_dir)?;
    }
    Ok(())
}
