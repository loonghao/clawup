//! Tests for `clawup_core::error` module.

use rstest::rstest;

use clawup_core::CoreError;

#[rstest]
fn config_not_found_display() {
    let err = CoreError::ConfigNotFound("/path/to/config.toml".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("Configuration file not found"));
    assert!(msg.contains("/path/to/config.toml"));
}

#[rstest]
fn invalid_config_display() {
    let err = CoreError::InvalidConfig("missing required field".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("Invalid configuration"));
    assert!(msg.contains("missing required field"));
}

#[rstest]
fn openclaw_not_found_display() {
    let err = CoreError::OpenClawNotFound("/home/user/.openclaw".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("OpenClaw directory not found"));
}

#[rstest]
fn other_error_display() {
    let err = CoreError::Other("something went wrong".to_string());
    let msg = format!("{err}");
    assert_eq!(msg, "something went wrong");
}

#[rstest]
fn io_error_converts_from_std() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let core_err: CoreError = io_err.into();
    let msg = format!("{core_err}");
    assert!(msg.contains("IO error"));
}

#[rstest]
fn core_error_is_debug() {
    let err = CoreError::ConfigNotFound("test".to_string());
    // Ensure Debug is implemented
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("ConfigNotFound"));
}
