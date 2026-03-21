//! Core logic for clawup.
//!
//! Provides error types, path detection, filesystem utilities,
//! and manifest CRUD operations.

pub mod error;
pub mod fs;
pub mod manifest;
pub mod paths;

pub use error::CoreError;
