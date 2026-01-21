# AGENTS.md - Toona Matrix Chat Client

Guidance for agentic coding assistants working in this repository.

## Project Snapshot

Toona is a multi-platform Matrix chat client with kanban board features, built in Rust with Makepad + Robius.

- **Language**: Rust 2024 edition
- **UI**: Makepad (live_design! DSL)
- **Targets**: macOS, Windows, Linux, Android, iOS

## Build Commands

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

## Test Commands

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run tests for a specific module
cargo test utils

# Run a single test by name
cargo test test_human_readable_list_empty

# Run a specific test (full path)
cargo test utils::tests::test_human_readable_list_empty

# Run tests with output
cargo test -- --nocapture

# Run tests with features
cargo test --features tsp

# Run doc tests
cargo test --doc
```

## Linting & Formatting

```bash
# Clippy lints
cargo clippy

# Clippy all targets/features
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

## Custom Build Profiles

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
- Respect `rustfmt.toml` for formatting
- Use 4-space indentation inside `live_design!` blocks

### Imports
Order: `std` → third-party → `crate`
```rust
use std::path::PathBuf;
use chrono::{DateTime, Local};
use makepad_widgets::{Cx, Event};
use crate::{
    room::RoomInputBar,
    sliding_sync::SlidingSyncState,
};
```

### Naming Conventions
- Modules/files: `snake_case`
- Types/structs/enums: `PascalCase`
- Functions/methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Live design IDs: `snake_case` for widgets, `PascalCase` for components

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `anyhow::Result<T>` at application boundaries
- Add context with `.context("...")` from anyhow
- Avoid `unwrap`/`expect` in production code

### Makepad Live Design
```rust
use makepad_widgets::*;

live_design! {
    use link::theme::*;

    pub KanbanCard = <View> {
        width: Fill, height: Fit
        show_bg: true
        draw_bg: { color: #FFFFFF }
    }
}
```

**Important**: This Makepad version does NOT support:
- `border_radius` in draw_bg
- `id:` fields in live_design!
- `scroll: {y: true}` - use `<ScrollYView>` instead
- `cursor: Pointer`

### Async & Concurrency
- Use `tokio` runtime for async operations
- Follow Matrix SDK async patterns for timeline operations

### Testing
- Use `#[cfg(test)]` modules near the code under test
- Keep tests deterministic; avoid real network calls

### Feature Flags
- Gate feature-specific code with `#[cfg(feature = "...")]`
- Provide safe defaults when features are off

## Repo Layout

```
src/
├── app.rs              # App entry point
├── home/               # Main UI (rooms, kanban, navigation)
│   ├── home_screen.rs  # Main screen with kanban toggle
│   ├── kanban_list_view.rs
│   ├── kanban_card.rs
│   └── mod.rs          # Register new UI components here
├── kanban/             # Kanban data models and state
│   ├── data/models.rs  # KanbanBoard, KanbanCard structs
│   └── state/          # KanbanState, KanbanActions
├── room/               # Room-specific UI
├── login/              # Login screen
├── settings/           # Settings screen
├── shared/             # Reusable widgets and styles
└── persistence/        # Serialization and storage
```

## Key Dependencies
- `matrix-sdk`, `matrix-sdk-ui` - Matrix protocol
- `makepad-widgets` - UI framework
- `tokio` - Async runtime
- `anyhow` - Error handling
- `chrono` - Date/time

## Editor/Assistant Rules

1. Run `cargo check` before committing
2. Use `cargo clippy` to catch common mistakes
3. New UI components must be registered in `home/mod.rs` live_design function
4. When modifying UI, test on both desktop and mobile
5. No `.cursor/rules/` or `.github/copilot-instructions.md` in this repo

## Common Patterns

### Adding a new UI component
1. Create component file in appropriate module
2. Add `pub mod component_name;` to module's mod.rs
3. Register in `home/mod.rs`: `component_name::live_design(cx);`
4. Use `<ComponentName>` in parent's live_design! block

### Makepad Widget Pattern
```rust
use makepad_widgets::*;

live_design! {
    pub MyComponent = <View> {
        width: Fill, height: Fit
        show_bg: true
        draw_bg: { color: #fff }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MyComponent {
    #[deref] view: View,
}

impl Widget for MyComponent {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
```

## Development Reminders
- Makepad assets packaged via `package.metadata.packager` in Cargo.toml
- Matrix SDK calls are async - handle with care
- Register new widgets in `home/mod.rs` live_design function
