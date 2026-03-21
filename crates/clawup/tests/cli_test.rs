use assert_cmd::Command;
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
