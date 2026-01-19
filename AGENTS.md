# AGENTS.md - Robrix Matrix Chat Client

Guidance for agentic coding assistants working in this repository.

## Project Snapshot

Robrix is a multi-platform Matrix chat client built in Rust with Makepad + Robius.

- Language: Rust 2024 edition
- UI: Makepad (live_design! DSL)
- Main dependencies: matrix-sdk, matrix-sdk-ui, robius-* crates
- Build system: Cargo workspace with custom profiles
- Targets: macOS, Windows, Linux, Android, iOS

## Build, Run, Lint, Test
### Build & Run
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run app
cargo run

# Run with TSP feature
cargo run --features tsp

# Fast compile check
cargo check
```
### Tests (including single test)
```bash
# Run all tests
cargo test

# Run unit tests only (no integration tests)
cargo test --lib

# Run tests for a specific module
cargo test utils

# Run a single test by name (exact match)
cargo test test_human_readable_list_empty

# Run a specific test in a module (full path)
cargo test utils::tests::test_human_readable_list_empty

# Run tests with output
cargo test -- --nocapture

# Run tests with features
cargo test --features tsp

# Run doc tests
cargo test --doc

# Run integration tests
cargo test --test *
```

### Linting & Formatting
```bash
# Clippy lints
cargo clippy

# Clippy for all targets/features
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Check formatting only
cargo fmt -- --check
```

### Custom Profiles
```bash
# Optimized dev build
cargo build --profile debug-opt

# Release with thin LTO
cargo build --profile release-lto

# Distribution build with fat LTO
cargo build --profile distribution
```

## Code Style Guidelines

### Formatting
- Respect `rustfmt.toml` for formatting.
- Use 4-space indentation inside `live_design!` blocks.
- Keep lines readable; prefer breaking long chains across lines.

### Imports
- Order imports: `std` → third-party → `crate`.
- Use `use crate::` for local modules.
- Group complex imports with `{}` and remove unused imports.
- Example:
```rust
use std::path::PathBuf;
use chrono::{DateTime, Local};
use makepad_widgets::{Cx, Event};
use crate::{
    room::RoomInputBar,
    sliding_sync::SlidingSyncState,
};
```

### Naming & Types
- Modules/files: `snake_case`.
- Types/structs/enums: `PascalCase`.
- Functions/methods: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Live design IDs: `snake_case` for widgets, `PascalCase` for components.
- Avoid single-letter variable names outside short scopes.
- Generic types: `T`, `E`, `R` for common patterns.

### Error Handling
- Use `Result<T, E>` for fallible operations; `Option<T>` for absence.
- Use `anyhow::Result<T>` at application boundaries.
- Define typed errors with `thiserror` when needed.
- Add context with `.context("...")` from `anyhow`.
- Avoid `unwrap`/`expect` in production code (tests ok).

### Makepad Live Design
- Group related properties and use consistent ordering.
- Reuse shared styles from `shared/styles.rs`.
- Prefer existing patterns for widgets and themes.
- Use `live_design!` macro for UI definitions.
- Example component pattern:
```rust
use makepad_widgets::*;

live_design! {
    use link::theme::*;

    pub MyComponent = <View> {
        width: Fill, height: Fit
        draw_bg: { color: #fff }
    }
}
```

### Async & Concurrency
- Use `tokio` runtime for async operations.
- Prefer `?` for error propagation.
- Use `tokio::spawn` for background tasks and keep handles if needed.
- Follow Matrix SDK async patterns for timeline operations.

### Documentation & Comments
- Use `///` for public APIs and `//!` for module docs.
- Keep comments short and purposeful; avoid restating obvious code.
- Document all public types and functions.

### Testing
- Use `#[cfg(test)]` modules near the code under test.
- Prefer descriptive test names and include edge cases.
- Keep tests deterministic; avoid real network calls.
- Group tests in submodules: `#[cfg(test)] mod tests { ... }`.

### Feature Flags
- Gate feature-specific code with `#[cfg(feature = "...")]`.
- Provide safe defaults when features are off.

## Repo Layout (high level)

- `app.rs`: App entry point and state management.
- `utils.rs`: Shared helpers and utilities.
- `home/`: Main UI including rooms list, room screen, kanban board.
- `room/`: Room-specific UI components.
- `login/`: Login screen and authentication.
- `settings/`: Settings screen and preferences.
- `shared/`: Reusable widgets and styles.
- `kanban/`: Kanban board functionality (new feature).
- `sliding_sync.rs`, `avatar_cache.rs`, `media_cache.rs`: Matrix integration helpers.
- `persistence/`: Serialization and storage.
- `tsp/`, `verification/`: Optional feature modules.

### Key Files

- `src/home/home_screen.rs`: Main screen with rooms list and kanban view toggle
- `src/home/kanban_list_view.rs`: Kanban column component
- `src/home/kanban_card.rs`: Kanban card component
- `src/shared/styles.rs`: Shared style constants and themes

## Dependency Notes

- Makepad assets/resources are packaged via `package.metadata.packager` in `Cargo.toml`.
- Matrix SDK calls are async and should be handled with care.
- Key dependencies: `matrix-sdk`, `matrix-sdk-ui`, `tokio`, `anyhow`, `chrono`.

## Editor/Assistant Rules

- No `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` found in this repo.
- When making changes, run `cargo check` first to verify compilation.
- Use `cargo clippy` to catch common mistakes.
- Check `cargo fmt` before committing.

## Development Reminders

- Run `cargo check` and `cargo clippy` before committing.
- Be mindful of cross-platform resource handling.
- Respect existing Makepad patterns and style constants.
- When adding new UI components, register them in `home/mod.rs` live_design function.
- Test changes on both desktop and mobile if modifying UI.
