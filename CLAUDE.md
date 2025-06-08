# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`checkupdates-external` is a Rust CLI tool that checks for updates from external (non-Arch) repositories for packages available in the Arch Linux AUR. It outputs updates in the format: `<package> <current_ver> -> <new_version>`.

## Architecture

This is a Cargo workspace with a main binary and product-specific library crates:

- **Main binary** (`src/main.rs`): Orchestrates concurrent checks across all products using `futures::join!`
- **Product crates** (`crates/*/`): Each handles a specific external source (Chrome, Edge, JetBrains, MongoDB, TeamViewer)
- **Shared crates**:
  - `crates/yum/`: Common YUM repository parsing functionality
  - `crates/aur/`: Centralized AUR package fetching

Each product crate follows this pattern:
- `lib.rs`: Re-exports public API
- `fetch.rs`: Async functions to fetch version information
- `model.rs`: Data structures (when needed)
- `print.rs`: Output formatting (when needed)

## Essential Commands

```bash
# Building
cargo build                    # Debug build
cargo build --release         # Release build

# Code Quality
cargo fmt                     # Format code
cargo fmt -- --check         # Check formatting
cargo clippy -- -D warnings  # Lint with warnings as errors

# Running
cargo run                    # Run debug version
./target/release/checkupdates-external  # Run release binary
```

## Key Development Patterns

### Adding a new product checker

1. Create new crate under `crates/` with standard structure
2. Add to workspace members in root `Cargo.toml`
3. Implement async fetch function returning `Vec<(name, current_ver, new_ver)>`
4. Add check function call in `main.rs` using `futures::join!`

### Working with versions

- **YUM-based products** (Chrome, Edge, MongoDB, TeamViewer): Use shared `yum` crate for XML parsing
- **JetBrains**: Custom XML parsing from updates endpoint
- Always fetch both upstream and AUR versions for comparison

### Async operations

- All network calls use `reqwest` with `Result<T, reqwest::Error>`
- Main uses Tokio runtime (`#[tokio::main]`)
- Concurrent execution via `futures::join!` for all product checks

### Error handling

- Network errors bubble up through Result chain
- Main uses `.expect()` for critical failures
- No custom error types - relies on `reqwest::Error`

## Dependencies

Core dependencies across crates:
- `reqwest` (with features: ["gzip", "json"])
- `tokio` (async runtime)
- `serde` + `serde_json` + `serde-xml-rs` (serialization)
- `libflate` (gzip decompression)
- `itertools` (utility functions)

## CI/CD

GitHub Actions runs on push/PR to main:
1. **Rust workflow**: Format check → Build → Clippy
2. **Super-linter**: General code quality checks