# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`checkupdates-external` is a Rust CLI tool that checks for updates from external (non-Arch) repositories for packages available in the Arch Linux AUR. It outputs updates in the format: `<package> <current_ver> -> <new_version>`.

## Architecture

This is a Cargo workspace with a main binary and product-specific library crates:

- **Main binary** (`src/main.rs`): 
  - CLI interface using `clap` with derive macros
  - Orchestrates concurrent checks using `futures::join!`
  - Selective product checking via `--jb`, `--mongo`, `--chrome` flags (combinable)
  - `--all` flag to show all packages even when versions match

- **Product crates** (`crates/*/`): Each handles a specific external source
  - `chrome/`: Google Chrome (stable, beta, dev, canary)
  - `edge/`: Microsoft Edge (stable, beta, dev)
  - `jetbrains/`: All JetBrains IDEs (stable and EAP channels)
  - `mongodb/`: MongoDB packages
  - `teamviewer/`: TeamViewer

- **Shared crates**:
  - `crates/yum/`: Common YUM repository XML parsing
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
cargo run                    # Check all products
cargo run -- --jb            # JetBrains only
cargo run -- --mongo         # MongoDB only
cargo run -- --chrome        # Chrome only
cargo run -- --jb --chrome   # Multiple products
cargo run -- --all           # Show all packages (even matching versions)

# Testing (currently no tests exist)
cargo test --all
```

## Key Development Patterns

### Adding a new product checker

1. Create new crate under `crates/` with standard structure
2. Add to workspace in root `Cargo.toml`: `newproduct = { path = "crates/newproduct" }`
3. For YUM-based products:
   - Use `yum::fetch_yum_updates()` with the repository URL
   - Return `Vec<YumUpdate>`
4. For custom formats:
   - Implement custom parsing with serde
   - Follow JetBrains pattern for complex XML
5. Add check function in `main.rs` following existing pattern
6. Add to concurrent execution in main's `join!` or futures vector
7. Optionally add CLI flag for selective checking

### XML Parsing with serde-xml-rs 0.8.x

**Important**: XML attributes must use `@` prefix in serde rename:
```rust
#[serde(rename = "@name")]    // For attributes
#[serde(rename = "element")]   // For nested elements
```

### Version handling

- AUR versions include release suffix (e.g., "1.2.3-1")
- Upstream versions are typically clean (e.g., "1.2.3")
- JetBrains uses complex build numbers requiring special parsing
- Pre-release versions (containing '~') are filtered out

### Error handling

- All async functions return `Result<T, reqwest::Error>`
- No custom error types
- Critical failures use `.expect()` with descriptive messages

## CI/CD

GitHub Actions runs on push/PR to main:
1. **Rust workflow** (`rust.yml`): 
   - Format check with `cargo fmt -- --check`
   - Build with `cargo build --verbose`
   - Clippy with `cargo clippy -- -D warnings`
2. **Super-linter** (`linter.yml`): General code quality (Rust linters disabled)