# AGENTS.md - Toona Matrix Chat Client

Guidance for agentic coding assistants working in this repository.

## Project Snapshot

Toona is a multi-platform Matrix chat client with kanban board features, built in Rust with Makepad + Robius.

- **Language**: Rust 2024 edition
- **UI**: Makepad (live_design! DSL)
- **Targets**: macOS, Windows, Linux, Android, iOS
- **Package Name**: toona
- **Product Name**: Robrix (for distribution)

## Build Commands

```powershell
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

```powershell
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

```powershell
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

```powershell
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
├── lib.rs                   # Library entry point
├── main.rs                  # Program entry point
├── app.rs                   # App entry point, state management
├── utils.rs                 # Shared utility functions
├── sliding_sync.rs          # Matrix Sliding Sync logic
├── space_service_sync.rs    # Matrix Space Service Sync
├── avatar_cache.rs          # Avatar caching
├── media_cache.rs           # Media caching
├── location.rs              # Location functionality
├── event_preview.rs         # Event preview generation
├── temp_storage.rs          # Temporary data storage
├── verification.rs          # E2EE verification
├── verification_modal.rs    # Verification modal UI
├── join_leave_room_modal.rs # Join/leave room modal
│
├── home/                    # Main UI (rooms, kanban, navigation)
│   ├── mod.rs               # Register new UI components here
│   ├── home_screen.rs       # Main screen with kanban toggle
│   ├── main_desktop_ui.rs   # Desktop-specific UI
│   ├── main_mobile_ui.rs    # Mobile-specific UI
│   ├── room_screen.rs       # Room chat screen
│   ├── rooms_list.rs        # Rooms list component
│   ├── rooms_sidebar.rs     # Sidebar with rooms
│   ├── rooms_list_header.rs # Rooms list header
│   ├── rooms_list_entry.rs  # Individual room entry
│   ├── spaces_bar.rs        # Spaces navigation bar
│   ├── space_lobby.rs       # Space lobby view
│   ├── search_messages.rs   # Message search
│   ├── invite_screen.rs     # Invite users screen
│   ├── navigation_tab_bar.rs # Tab bar navigation
│   ├── light_themed_dock.rs # Dock component
│   ├── welcome_screen.rs    # Welcome screen
│   ├── loading_pane.rs      # Loading indicator
│   ├── editing_pane.rs      # Editing mode pane
│   ├── new_message_context_menu.rs # Context menu
│   ├── event_reaction_list.rs # Reaction display
│   ├── link_preview.rs      # Link preview widget
│   ├── location_preview.rs   # Location preview
│   ├── room_image_viewer.rs # Image viewer
│   ├── room_read_receipt.rs # Read receipts
│   ├── edited_indicator.rs   # Edited indicator
│   ├── add_room.rs          # Add room dialog
│   └── tombstone_footer.rs  # Tombstone message footer
│
├── kanban/                  # Kanban board functionality
│   ├── mod.rs               # Module entry, re-exports main types
│   ├── app.rs               # Kanban app integration
│   ├── matrix_adapter.rs    # Matrix protocol adapter
│   ├── api/                 # API integration layer
│   │   ├── kanban_requests.rs
│   │   ├── repositories.rs
│   │   └── mod.rs
│   ├── components/          # Kanban UI components
│   │   ├── mod.rs
│   │   ├── space.rs         # Kanban space/board container
│   │   ├── card_list.rs     # Card list column
│   │   ├── card_item.rs     # Individual card item
│   │   ├── card_modal.rs    # Card detail/edit modal
│   │   ├── modal_header.rs  # Modal header component
│   │   ├── card_info_section.rs  # Card info section
│   │   ├── tag_section.rs    # Tag editing section
│   │   ├── todo_section.rs   # Todo checklist section
│   │   └── active_section.rs # Active users section
│   ├── data/                # Data models and storage
│   │   ├── models.rs
│   │   ├── repositories.rs
│   │   └── mod.rs
│   ├── drag_drop/           # Drag and drop logic
│   │   ├── drag_handler.rs
│   │   ├── order_manager.rs
│   │   └── mod.rs
│   ├── models/              # Kanban domain models
│   │   └── mod.rs
│   └── state/               # Kanban state management
│       ├── kanban_state.rs
│       ├── kanban_actions.rs
│       └── mod.rs
│
├── room/                    # Room-specific UI
│   ├── mod.rs
│   ├── room_input_bar.rs    # Message input component
│   ├── reply_preview.rs     # Reply preview widget
│   ├── room_display_filter.rs # Room display filter
│   └── typing_notice.rs     # Typing indicator
│
├── login/                   # Login screen
├── logout/                  # Logout flow
├── settings/                # Settings screen
├── profile/                 # User profile
├── shared/                  # Reusable widgets and styles
├── persistence/            # Serialization and storage
│
├── tsp/                     # TSP wallet integration [feature: tsp]
│   ├── create_did_modal.rs
│   ├── did_qr_code.rs
│   ├── import_identity_modal.rs
│   ├── mod.rs
│   ├── send_coin_modal.rs
│   ├── settings.rs
│   ├── token_balance.rs
│   ├── tsp_action_bar.rs
│   ├── tsp_chat_bar.rs
│   ├── tsp_link_handler.rs
│   └── tsp_signing_modal.rs
│
└── tsp_dummy/               # TSP placeholder module
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `tsp` | Enable experimental TSP wallet support |
| `hide_windows_console` | Hide console on Windows |
| `log_room_list_diffs` | Log RoomList diffs |
| `log_timeline_diffs` | Log timeline diffs |
| `log_space_service_diffs` | Log SpaceService diffs |

## Key Dependencies
- `matrix-sdk`, `matrix-sdk-ui` - Matrix protocol
- `makepad-widgets` - UI framework
- `robius-*` crates - Robius app framework (robius-use-makepad, robius-open, robius-directories, robius-location)
- `tokio` - Async runtime
- `anyhow` - Error handling
- `chrono` - Date/time
- `serde` - Serialization
- `ruma` - Matrix protocol types
- `eyeball`, `eyeball-im` - Observable collections (same as matrix-sdk-ui)
- `imbl` - Immutable collections (same as matrix-sdk-ui)
- `dashmap` - Concurrent HashMap
- `blurhash` - Image blurhash generation

## Editor/Assistant Rules

1. Run `cargo check` before committing
2. Use `cargo clippy` to catch common mistakes
3. New UI components must be registered in `home/mod.rs` live_design function
4. When modifying UI, test on both desktop and mobile
5. No `.cursor/rules/` or `.github/copilot-instructions.md` in this repo
6. Do NOT generate documentation files (*.md) in the project

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

### Kanban Component Pattern
Kanban components use a modal-based architecture:
```rust
// Components hierarchy:
KanbanSpace (board container)
├── CardList (column)
│   └── CardItem (individual card)
└── CardModal (detail/edit modal)
    ├── ModalHeader
    ├── CardInfoSection
    ├── TagSection
    ├── TodoSection
    └── ActiveSection
```

## Development Reminders
- Makepad assets packaged via `package.metadata.packager` in Cargo.toml
- Matrix SDK calls are async - handle with care
- Register new widgets in `home/mod.rs` live_design function
- TSP feature requires specific dependencies and patches (see Cargo.toml)
- Use `gitcode.com` mirrors for internal dependencies
- Project uses Robius framework for platform abstractions
- Package metadata configured for cargo-packager (distributes as "Robrix")