//! Schema definitions for clawup configuration files.
//!
//! This crate contains all data types used in `clawup.toml` manifests.
//! It is intentionally kept lightweight with minimal dependencies (only serde + serialization).

mod manifest;

pub use manifest::*;
