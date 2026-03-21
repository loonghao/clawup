//! Soul management module.
//!
//! Handles soul file templates, rendering, and merging.

use std::path::Path;

use color_eyre::Result;

/// Known soul file names.
pub const SOUL_FILES: &[&str] = &["SOUL.md", "IDENTITY.md", "USER.md", "AGENTS.md", "BOOT.md"];

/// Read a soul file from a workspace directory.
pub fn read_soul_file(workspace: &Path, filename: &str) -> Result<Option<String>> {
    crate::utils::fs::read_optional(&workspace.join(filename))
}

/// Write a soul file to a workspace directory.
pub fn write_soul_file(workspace: &Path, filename: &str, content: &str) -> Result<()> {
    let path = workspace.join(filename);
    if let Some(parent) = path.parent() {
        crate::utils::fs::ensure_dir(parent)?;
    }
    std::fs::write(&path, content)?;
    Ok(())
}

/// List all soul files in a workspace directory.
pub fn list_soul_files(workspace: &Path) -> Result<Vec<String>> {
    let mut found = vec![];
    for name in SOUL_FILES {
        if workspace.join(name).exists() {
            found.push(name.to_string());
        }
    }
    Ok(found)
}

/// Render a soul template using Tera.
pub fn render_template(template_str: &str, context: &tera::Context) -> Result<String> {
    let mut tera = tera::Tera::default();
    tera.add_raw_template("soul", template_str)?;
    let rendered = tera.render("soul", context)?;
    Ok(rendered)
}
