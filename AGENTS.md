# AGENTS.md

Guidance for agentic coding assistants working in this repository.

## Project Overview

`checkupdates-external` is a Rust CLI tool that checks for package updates from
external (non-pacman) upstream sources (JetBrains, MongoDB, Chrome, Edge,
TeamViewer) and compares them against AUR versions. Output format:
`<package> <current_ver> -> <new_version>`.

Cargo workspace: one root binary crate + 7 library crates under `crates/`.

## Commands

```bash
# Build
cargo build                    # debug
cargo build --release          # release

# Code quality (run all before committing)
cargo fmt                      # apply formatting
cargo fmt -- --check           # check only (used in CI)
cargo clippy -- -D warnings    # lint; warnings are errors in CI

# Run
cargo run                      # check all products
cargo run -- --jb              # JetBrains only
cargo run -- --mongo           # MongoDB only
cargo run -- --chrome          # Chrome only
cargo run -- --edge            # Edge only
cargo run -- -a                # show all packages, even when versions match

# Tests (none exist yet)
cargo test --all
```

CI runs `cargo fmt -- --check`, `cargo build --verbose`, and
`cargo clippy -- -D warnings` on every push/PR.

There is no single-test command because there are no tests in the codebase.

## Architecture

```
src/main.rs              # CLI (clap derive), concurrent orchestration via join_all
crates/
  aur/                   # AUR RPC HTTP client (shared by all product crates)
    src/{lib,fetch,model}.rs
  yum/                   # YUM/RPM repo parser — shared infrastructure
    src/{lib,fetch,model,print}.rs
  chrome/                # Google Chrome (wraps yum)
  edge/                  # Microsoft Edge (wraps yum)
  mongodb/               # MongoDB (wraps yum)
  teamviewer/            # TeamViewer (wraps yum)
  jetbrains/             # JetBrains IDEs — custom XML parsing
    src/{lib,fetch,model,print}.rs
```

YUM-based products (`chrome`, `edge`, `mongodb`, `teamviewer`) all depend on the
`yum` crate. `jetbrains` depends directly on `aur`.

## Adding a New Product Checker

1. Create `crates/<name>/src/{lib.rs,fetch.rs}` following the existing pattern.
2. Add `<name> = { path = "crates/<name>" }` to the root `Cargo.toml`
   `[dependencies]` and `[workspace] members`.
3. For YUM-based sources: call `yum::fetch_yum_updates(url)` and return
   `Vec<YumUpdate>`.
4. For custom sources: implement parsing following the JetBrains pattern.
5. Add a `check_<name>_updates()` function in `src/main.rs`.
6. Push it into the `futures` vec in `main()`.
7. Optionally add a `--<name>` CLI flag to `Args`.

## Code Style

### Imports

Group imports: external crates first (alphabetical), then `crate::` imports.
No blank lines between items in the same group.

```rust
use crate::model::{YumPackage, YumUpdate};
use itertools::Itertools;
use reqwest::{Error, Response};
use semver::Version;
use std::io::Read;
```

### Formatting

Standard `rustfmt` defaults. 4-space indentation. Trailing commas in
multi-line expressions. Enforced by CI.

### Types and Data Structures

- Use owned `String` in structs and return types; use `&str` for parameters.
- Use `Option<String>` for optional fields (e.g., `Build::full_number`).
- Use boxed pinned futures for heterogeneous async task collections:
  ```rust
  type UpdateFuture =
      std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Error>> + Send>>;
  ```

### Naming Conventions

| Kind | Convention | Examples |
|---|---|---|
| Functions | `snake_case` | `fetch_aur_packages`, `print_yum_updates` |
| Fetch functions | `fetch_*` | `fetch_chrome_updates` |
| Print functions | `print_*` | `print_jetbrains_updates` |
| Orchestration | `check_*` | `check_chrome_updates` |
| Helper/pure | `get_*`, `remove_*`, `compare_*` | `get_package_build` |
| Structs | `PascalCase` | `AurPackage`, `YumUpdate`, `JetBrainsRepository` |
| Modules/files | `snake_case` | `fetch.rs`, `model.rs`, `print.rs` |
| Crates | short `snake_case` | `aur`, `yum`, `jetbrains` |

### Module Pattern

Every library crate uses the same `lib.rs` pattern:

```rust
mod fetch;
mod model;
mod print;

pub use fetch::fetch_jetbrains_updates;
pub use model::*;
pub use print::print_jetbrains_updates;
```

### Async / Concurrency

- Runtime: `tokio` with `features = ["full"]`, entry via `#[tokio::main]`.
- Top-level concurrency: `futures::future::join_all` over a `Vec` of boxed futures.
- No `tokio::spawn`, no channels, no `Arc`/`Mutex`.
- Within a fetch function, use simple linear `await` chains:
  ```rust
  reqwest::get(url).await?.text().await?
  ```

### XML Parsing (serde-xml-rs 0.8.x)

Attributes **must** use `@` prefix in serde rename:

```rust
#[serde(rename = "@name")]   // XML attribute
#[serde(rename = "element")] // nested XML element
```

### Error Handling

- All public async functions return `Result<T, reqwest::Error>`.
- Propagate with `?`; no custom error types, no `anyhow`, no `thiserror`.
- Use `.unwrap()` freely for internal invariants (unexpected parse failures,
  missing data) — this is a CLI tool, not a library; panics are acceptable.
- Top-level failures in `main.rs` use:
  ```rust
  result.unwrap_or_else(|_| panic!("Cannot fetch updates for {}!", name));
  ```

### Version Handling

- AUR versions include a release suffix: `1.2.3-1`.
- Upstream versions are clean: `1.2.3`.
- Strip the AUR suffix before comparing with `semver`.
- Filter out prerelease versions (those containing `~`).
- JetBrains build numbers require special parsing — see `jetbrains/src/model.rs`.

## Key Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `clap` | 4.5 (`derive`) | CLI argument parsing |
| `futures` | 0.3 | `join_all` for concurrency |
| `reqwest` | 0.13 (`gzip`, `json`) | HTTP client |
| `tokio` | 1.49 (`full`) | Async runtime |
| `serde` | 1.0 (`derive`) | Serialization |
| `serde-xml-rs` | 0.8.2 | XML deserialization |
| `semver` | 1.0 | Version comparison |
| `itertools` | 0.14 | Iterator utilities |
| `libflate` | 2.2 | gzip decompression |

## CI / GitHub Actions

- `rust.yml`: fmt check → build → clippy (all warnings as errors)
- `linter.yml`: super-linter for non-Rust files (Rust clippy disabled there)
- Actions are pinned to commit SHAs for security (zizmor enforced).
