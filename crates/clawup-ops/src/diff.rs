//! Diff module.
//!
//! Provides configuration diff utilities using the `similar` crate
//! for comparing manifest files and configuration states.

use similar::{ChangeTag, TextDiff};

use crate::error::Result;

/// A single line change in a diff.
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// The type of change.
    pub tag: DiffTag,
    /// The content of the line.
    pub content: String,
}

/// Tag indicating the type of change for a diff line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffTag {
    /// Line is the same in both versions.
    Equal,
    /// Line was deleted (only in old version).
    Delete,
    /// Line was inserted (only in new version).
    Insert,
}

/// Compute a line-based diff between two strings.
///
/// Returns a list of `DiffLine` entries representing the changes.
pub fn compute_diff(old: &str, new: &str) -> Vec<DiffLine> {
    let diff = TextDiff::from_lines(old, new);
    diff.iter_all_changes()
        .map(|change| DiffLine {
            tag: match change.tag() {
                ChangeTag::Equal => DiffTag::Equal,
                ChangeTag::Delete => DiffTag::Delete,
                ChangeTag::Insert => DiffTag::Insert,
            },
            content: change.value().to_string(),
        })
        .collect()
}

/// Generate a unified diff string (like `diff -u`).
pub fn unified_diff(old: &str, new: &str, old_label: &str, new_label: &str) -> String {
    let diff = TextDiff::from_lines(old, new);
    diff.unified_diff()
        .header(old_label, new_label)
        .to_string()
}

/// Check if two configuration strings are identical.
pub fn is_equal(old: &str, new: &str) -> bool {
    old == new
}

/// Compare two manifest files by path and return a unified diff.
pub fn diff_files(
    old_path: &std::path::Path,
    new_path: &std::path::Path,
) -> Result<Option<String>> {
    let old_content = std::fs::read_to_string(old_path)?;
    let new_content = std::fs::read_to_string(new_path)?;

    if is_equal(&old_content, &new_content) {
        return Ok(None);
    }

    let old_label = old_path.display().to_string();
    let new_label = new_path.display().to_string();
    Ok(Some(unified_diff(
        &old_content,
        &new_content,
        &old_label,
        &new_label,
    )))
}
