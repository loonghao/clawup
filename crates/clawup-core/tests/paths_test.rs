//! Tests for `clawup_core::paths` module.

use std::path::PathBuf;

use rstest::rstest;

use clawup_core::paths::OpenClawPaths;

// =============================================================================
// from_root / accessor tests
// =============================================================================

#[rstest]
fn from_root_stores_given_path() {
    let root = PathBuf::from("/tmp/test-openclaw");
    let paths = OpenClawPaths::from_root(root.clone());
    assert_eq!(paths.root(), root);
}

#[rstest]
fn config_file_is_under_root() {
    let root = PathBuf::from("/home/user/.openclaw");
    let paths = OpenClawPaths::from_root(root.clone());

    let config = paths.config_file();
    assert_eq!(config, root.join("openclaw.json"));
}

#[rstest]
fn workspace_dir_is_under_root() {
    let root = PathBuf::from("/home/user/.openclaw");
    let paths = OpenClawPaths::from_root(root.clone());

    let workspace = paths.workspace_dir();
    assert_eq!(workspace, root.join("workspace"));
}

#[rstest]
fn agent_workspace_dir_includes_agent_name() {
    let root = PathBuf::from("/home/user/.openclaw");
    let paths = OpenClawPaths::from_root(root.clone());

    let agent_ws = paths.agent_workspace_dir("code");
    assert_eq!(agent_ws, root.join("agents").join("code").join("workspace"));
}

#[rstest]
fn agent_workspace_dir_handles_special_agent_names() {
    let root = PathBuf::from("/tmp/oc");
    let paths = OpenClawPaths::from_root(root.clone());

    // Agent names with hyphens/underscores should work
    let ws = paths.agent_workspace_dir("code-review");
    assert_eq!(ws, root.join("agents/code-review/workspace"));

    let ws = paths.agent_workspace_dir("ops_deploy");
    assert_eq!(ws, root.join("agents/ops_deploy/workspace"));
}

#[rstest]
fn skills_dir_is_under_root() {
    let root = PathBuf::from("/opt/openclaw");
    let paths = OpenClawPaths::from_root(root.clone());

    assert_eq!(paths.skills_dir(), root.join("skills"));
}

#[rstest]
fn credentials_dir_is_under_root() {
    let root = PathBuf::from("/opt/openclaw");
    let paths = OpenClawPaths::from_root(root.clone());

    assert_eq!(paths.credentials_dir(), root.join("credentials"));
}

#[rstest]
fn memory_dir_is_under_root() {
    let root = PathBuf::from("/opt/openclaw");
    let paths = OpenClawPaths::from_root(root.clone());

    assert_eq!(paths.memory_dir(), root.join("memory"));
}

// =============================================================================
// detect tests (environment-dependent)
// =============================================================================

#[rstest]
fn detect_falls_back_to_home_dir() {
    // Ensure OPENCLAW_HOME is not set for this test
    // (We can't fully control the environment in parallel tests,
    //  but detect() should at least not error when home dir exists.)
    let original = std::env::var("OPENCLAW_HOME").ok();
    // SAFETY: This test runs single-threaded and we restore the var afterwards.
    unsafe {
        std::env::remove_var("OPENCLAW_HOME");
    }

    let result = OpenClawPaths::detect();
    // Should succeed — falls back to ~/.openclaw
    assert!(result.is_ok());

    let paths = result.unwrap();
    // Root should end with ".openclaw"
    assert!(paths.root().ends_with(".openclaw"));

    // Restore env var if it was set
    if let Some(val) = original {
        // SAFETY: restoring original environment state.
        unsafe {
            std::env::set_var("OPENCLAW_HOME", val);
        }
    }
}
