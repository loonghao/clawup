use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_help_output() {
    Command::cargo_bin("clawup")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("clawup"));
}

#[test]
fn test_version_output() {
    Command::cargo_bin("clawup")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("clawup"));
}

#[test]
fn test_init_non_interactive() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    // Verify clawup.toml was created
    assert!(temp.path().join("clawup.toml").exists());
}

#[test]
fn test_doctor_without_openclaw() {
    Command::cargo_bin("clawup")
        .unwrap()
        .arg("doctor")
        .assert()
        .success();
}

// =============================================================================
// Bug #2: init gracefully degrades in non-TTY (piped) environments
// =============================================================================

#[test]
fn test_init_piped_stdin_auto_non_interactive() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Simulate piped stdin by writing to stdin (makes is_terminal() return false)
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init"])
        .arg(temp.path())
        .write_stdin("")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    // Should still create the config file using defaults
    assert!(temp.path().join("clawup.toml").exists());
}

#[test]
fn test_init_with_template_flag_piped() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--template", "multi-agent"])
        .arg(temp.path())
        .write_stdin("")
        .assert()
        .success()
        .stdout(predicate::str::contains("multi-agent"));

    assert!(temp.path().join("clawup.toml").exists());

    // Verify the multi-agent template was used (should have multiple agents)
    let content = std::fs::read_to_string(temp.path().join("clawup.toml")).unwrap();
    assert!(content.contains("code"));
    assert!(content.contains("review"));
}

// =============================================================================
// Bug #4: profile create works
// =============================================================================

#[test]
fn test_profile_create() {
    let temp = assert_fs::TempDir::new().unwrap();

    // First, init a project
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    // Create a profile
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["profile", "create", "staging"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));

    // Verify the profile exists in the file
    let content = std::fs::read_to_string(temp.path().join("clawup.toml")).unwrap();
    assert!(content.contains("[profiles.staging]"));
}

#[test]
fn test_profile_create_duplicate() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Init with team template (which has profiles)
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive", "--template", "team"])
        .arg(temp.path())
        .assert()
        .success();

    // Try creating a profile that already exists
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["profile", "create", "dev"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"));
}

#[test]
fn test_profile_create_from_existing() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Init with team template (which has "dev", "staging", "production" profiles)
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive", "--template", "team"])
        .arg(temp.path())
        .assert()
        .success();

    // Create a new profile based on "dev"
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["profile", "create", "qa", "--from", "dev"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));

    // Verify the new profile exists
    let content = std::fs::read_to_string(temp.path().join("clawup.toml")).unwrap();
    assert!(content.contains("qa"));
}

#[test]
fn test_profile_list() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive", "--template", "team"])
        .arg(temp.path())
        .assert()
        .success();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["profile", "list"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("dev"));
}

// =============================================================================
// Bug #5: config merge works
// =============================================================================

#[test]
fn test_config_merge() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Init a project
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    // Create a merge source file
    let merge_source = temp.child("override.toml");
    merge_source
        .write_str(
            r#"
[meta]
description = "Merged description"
"#,
        )
        .unwrap();

    // Merge the override
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["config", "merge", "override.toml"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("merged"));

    // Verify the merge took effect
    let content = std::fs::read_to_string(temp.path().join("clawup.toml")).unwrap();
    assert!(content.contains("Merged description"));
}

#[test]
fn test_config_merge_deep() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Init a project
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    // Create a merge source that adds new gateway settings
    let merge_source = temp.child("overlay.toml");
    merge_source
        .write_str(
            r#"
[meta]
schema_version = "1"

[gateway]
port = 9999
"#,
        )
        .unwrap();

    // Merge
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["config", "merge", "overlay.toml"])
        .current_dir(temp.path())
        .assert()
        .success();

    // Verify the deep merge: port added, existing gateway fields preserved
    let content = std::fs::read_to_string(temp.path().join("clawup.toml")).unwrap();
    assert!(content.contains("9999"));
    // Original gateway settings should still be there
    assert!(content.contains("loopback"));
}

// =============================================================================
// Config get/set/show integration tests
// =============================================================================

#[test]
fn test_config_get() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["config", "get", "meta.schema_version"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_config_set() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["config", "set", "meta.description", "Updated by test"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Set"));

    // Verify
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["config", "get", "meta.description"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated by test"));
}

#[test]
fn test_config_show() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    // TOML format
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["config", "show"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("schema_version"));

    // JSON format
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["config", "show", "--format", "json"])
        .current_dir(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("schema_version"));
}

// =============================================================================
// Skill CLI integration tests (bundled skill toggle)
// =============================================================================

#[test]
fn test_skill_list() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["skill", "list"])
        .current_dir(temp.path())
        .assert()
        .success();
}

#[test]
fn test_skill_disable_bundled() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Init — default template has bundled skills: developer, computer
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    // Disable bundled skill "developer"
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["skill", "disable", "developer"])
        .current_dir(temp.path())
        .assert()
        .success();

    // Verify the skill was removed from bundled enabled list
    let content = std::fs::read_to_string(temp.path().join("clawup.toml")).unwrap();
    // "developer" should no longer appear in enabled list
    // "computer" should still be there
    assert!(content.contains("computer"));
}

#[test]
fn test_skill_enable_bundled() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Init
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["init", "--non-interactive"])
        .arg(temp.path())
        .assert()
        .success();

    // First disable, then re-enable
    Command::cargo_bin("clawup")
        .unwrap()
        .args(["skill", "disable", "developer"])
        .current_dir(temp.path())
        .assert()
        .success();

    Command::cargo_bin("clawup")
        .unwrap()
        .args(["skill", "enable", "developer"])
        .current_dir(temp.path())
        .assert()
        .success();

    let content = std::fs::read_to_string(temp.path().join("clawup.toml")).unwrap();
    assert!(content.contains("developer"));
}
