# Clawup development commands

# Default recipe - show available commands
default:
    @just --list

# Build the project
build:
    vx cargo build

# Build in release mode
build-release:
    vx cargo build --release

# Run all tests
test:
    vx cargo test --workspace

# Run tests with output
test-verbose:
    vx cargo test --workspace -- --nocapture

# Run clippy lints
lint:
    vx cargo clippy --workspace --all-targets -- -D warnings

# Format code
fmt:
    vx cargo fmt --all

# Check formatting
fmt-check:
    vx cargo fmt --all -- --check

# Run the CLI
run *ARGS:
    vx cargo run --bin clawup -- {{ARGS}}

# Check everything (format, lint, test)
check: fmt-check lint test

# Clean build artifacts
clean:
    vx cargo clean

# Install locally
install:
    vx cargo install --path crates/clawup
