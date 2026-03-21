//! Tests for `clawup_core::fs` module.

use std::fs;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use rstest::rstest;

use clawup_core::fs::{ensure_dir, expand_path, read_optional};

// =============================================================================
// ensure_dir tests
// =============================================================================

#[rstest]
fn ensure_dir_creates_new_directory() {
    let tmp = TempDir::new().unwrap();
    let target = tmp.child("new_dir");
    assert!(!target.path().exists());

    ensure_dir(target.path()).unwrap();

    assert!(target.path().exists());
    assert!(target.path().is_dir());
}

#[rstest]
fn ensure_dir_creates_nested_directories() {
    let tmp = TempDir::new().unwrap();
    let target = tmp.child("a/b/c");
    assert!(!target.path().exists());

    ensure_dir(target.path()).unwrap();

    assert!(target.path().exists());
    assert!(target.path().is_dir());
}

#[rstest]
fn ensure_dir_succeeds_if_already_exists() {
    let tmp = TempDir::new().unwrap();
    let target = tmp.child("existing");
    fs::create_dir_all(target.path()).unwrap();
    assert!(target.path().exists());

    // Should not error when directory already exists
    ensure_dir(target.path()).unwrap();

    assert!(target.path().exists());
}

// =============================================================================
// read_optional tests
// =============================================================================

#[rstest]
fn read_optional_returns_content_for_existing_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("hello.txt");
    file.write_str("hello world").unwrap();

    let result = read_optional(file.path()).unwrap();
    assert_eq!(result, Some("hello world".to_string()));
}

#[rstest]
fn read_optional_returns_none_for_missing_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("nonexistent.txt");

    let result = read_optional(file.path()).unwrap();
    assert_eq!(result, None);
}

#[rstest]
fn read_optional_returns_empty_string_for_empty_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("empty.txt");
    file.write_str("").unwrap();

    let result = read_optional(file.path()).unwrap();
    assert_eq!(result, Some("".to_string()));
}

#[rstest]
fn read_optional_handles_utf8_content() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.child("unicode.txt");
    file.write_str("你好世界 🌍").unwrap();

    let result = read_optional(file.path()).unwrap();
    assert_eq!(result, Some("你好世界 🌍".to_string()));
}

// =============================================================================
// expand_path tests
// =============================================================================

#[rstest]
fn expand_path_returns_plain_path_unchanged() {
    let result = expand_path("/some/absolute/path");
    assert_eq!(result, "/some/absolute/path");
}

#[rstest]
fn expand_path_expands_tilde() {
    let result = expand_path("~/documents");
    // Should not start with ~ anymore
    assert!(!result.starts_with('~'));
    assert!(result.ends_with("documents"));
}

#[rstest]
fn expand_path_handles_invalid_env_var_gracefully() {
    // When an env var doesn't exist, shellexpand returns the original
    let result = expand_path("$CLAWUP_NONEXISTENT_VAR_12345/foo");
    // Should return original string since the env var doesn't exist
    assert!(result.contains("foo"));
}
