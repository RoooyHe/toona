# Technology Stack

## Language & Edition

- Rust 2024 edition (stable toolchain)
- Requires Rust 1.79+ for packaging

## Core Frameworks

- **UI Framework**: Makepad (declarative DSL-based UI)
- **App Framework**: Robius (cross-platform abstractions)
- **Matrix SDK**: matrix-rust-sdk (with sliding sync support)
- **Async Runtime**: Tokio with multi-threaded runtime

## Key Dependencies

- `makepad-widgets`: UI components with serde support
- `matrix-sdk`: E2E encryption, SQLite storage, SSO login, rustls-tls
- `matrix-sdk-ui`: Timeline and UI utilities
- `ruma`: Matrix types and events (with compat-optional feature)
- `robius-*`: Platform abstractions (open, directories, location)
- `tokio`: Async runtime (macros, rt-multi-thread)
- `serde`/`serde_json`: Serialization
- `crossbeam-channel`, `crossbeam-queue`: Concurrency primitives
- `imbl`, `eyeball`, `eyeball-im`: Reactive data structures

## Optional Features

- `tsp`: Trust Spanning Protocol wallet support (requires tsp_sdk, quinn, aws-lc-rs)
- `hide_windows_console`: Hides console window on Windows
- `log_room_list_diffs`, `log_timeline_diffs`, `log_space_service_diffs`: Debug logging

## Build Configuration

### Cargo Config
- Global rustflag: `--cfg ruma_identifiers_storage=\"Arc\"`

### Common Commands

```bash
# Development build and run
cargo run

# Release build
cargo run --release

# With TSP support
cargo run --features tsp

# Check for errors
cargo check

# Run tests
cargo test

# Format code (uses rustfmt.toml config)
cargo fmt

# Lint
cargo clippy
```

### Mobile Builds

Requires `cargo-makepad`:
```bash
cargo install --force --git https://github.com/makepad/makepad.git --branch dev cargo-makepad
```

**Android:**
```bash
cargo makepad android install-toolchain
cargo makepad android run -p toona --release
```

**iOS:**
```bash
rustup toolchain install nightly
cargo makepad apple ios install-toolchain
cargo makepad apple ios --org=<org-id> --app=toona run-sim -p toona --release
```

### Packaging

```bash
# Install packager
cargo +stable install --force cargo-packager

# Create distribution package
cargo packager --release

# Windows (hide console)
RUSTFLAGS="--cfg hide_windows_console" cargo packager --release
```

## Code Style

- Edition: 2021
- Max width: 100 characters
- Tab spaces: 4 (soft tabs)
- Imports: Crate-level granularity, grouped as StdExternalCrate
- Function single line: enabled
- Spaces around ranges: enabled
- Try shorthand: enabled

## Lints

### Rust Lints (forbid)
- `keyword_idents_2024`
- `non_ascii_idents`
- `non_local_definitions`
- `unsafe_op_in_unsafe_fn`

### Clippy (allow)
- `blocks_in_conditions`
- `collapsible_if`, `collapsible_else_if`
- `module_name_repetitions`
- `too_many_arguments`
- `uninlined_format_args`
