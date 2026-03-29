# AGENTS.md - Toona Development Guide

Toona is a Matrix chat client with Kanban board features, written in Rust using Makepad UI framework.

## Build Commands

```bash
cargo run              # Debug build and run
cargo run --release    # Release build
cargo run --features tsp              # Build with TSP wallet
cargo run --features log_timeline_diffs,log_room_list_diffs  # With logging

cargo fmt             # Format code
cargo clippy          # Run linter
cargo check           # Fast type check

cargo test            # Run all tests
cargo test test_name  # Run single test by name
```

## Project Structure

```
src/
├── lib.rs              # Library entry, module declarations
├── main.rs             # Binary entry
├── app.rs              # App widget, Live/LiveHook, action routing
├── sliding_sync.rs     # Matrix sliding sync worker
├── kanban/             # Kanban board (Space=List, Room=Card)
│   ├── state/          # kanban_state.rs, kanban_actions.rs
│   ├── models/         # Data models
│   ├── data/           # Repositories
│   └── components/     # UI components
├── home/, room/, login/, logout/, settings/  # Feature modules
├── shared/             # Shared UI components, styles
└── persistence/        # State persistence
```

## Code Style

### Formatting (rustfmt.toml)
- Max width: 100, Indent: 4 spaces, Edition: 2024
- `fn_single_line = true`, `use_try_shorthand = true`
- Hex literals: uppercase, imports: crate granularity

### Import Order
```rust
use std::{path::Path, sync::OnceLock};
use anyhow::{anyhow, bail, Result};
use matrix_sdk::ruma::{OwnedRoomId, RoomId};
use crate::{module::Helper, other_module::Thing};
```

### Naming
- Types/Enums: `PascalCase` | Functions/Methods: `snake_case`
- Variables: `snake_case` | Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case` | Enum variants: `PascalCase`

## Error Handling

```rust
// Application errors - anyhow
fn load_data() -> Result<Data> {
    anyhow::Ok(my_data)
}

// Library/structured errors - thiserror
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed: {0}")]
    Failed(String),
}

// Specific error enums for categorized errors
pub enum AppError {
    Recoverable(RecoverableError),
    Unrecoverable(String),
}
```

**No `unwrap()`/`expect()` in production** except tests or truly impossible cases.

## Makepad UI Framework

```rust
live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::shared::styles::*;

    MyWidget = {{MyWidget}} {
        <View> {
            width: Fill, height: Fit,
            my_label = <Label> { text: "Hello" }
            my_button = <Button> { text: "Click" }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MyWidget {
    #[deref] view: View,
}

impl Widget for MyWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if let Event::Actions(actions) = event {
            if self.view.button(ids!(my_button)).clicked(actions) {
                // Handle click
            }
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
```

### Widget Registration Order (in `live_design()`)
1. `makepad_widgets::live_design(cx)`
2. `crate::shared::live_design(cx)`
3. Other modules in dependency order

## Action Pattern

Actions routed in `handle_event`:
```rust
if let Event::Actions(actions) = event {
    for action in actions {
        if let Some(kanban_action) = action.downcast_ref::<KanbanActions>() {
            self.handle_kanban_action(cx, kanban_action.clone());
        }
    }
}
```

## State Management

- App state in `App` struct with `#[live]`/`#[rust]` fields
- `scope.data.get::<AppState>()` to access in widgets
- UI updates via `Cx::redraw()` and `SignalToUI::set_ui_signal()`

## Feature Flags

```rust
#[cfg(feature = "tsp")]
pub mod tsp;

#[cfg(not(feature = "tsp"))]
pub mod tsp_dummy;
```

Available: `tsp`, `hide_windows_console`, `log_room_list_diffs`, `log_timeline_diffs`, `log_space_service_diffs`

## Lints

Forbid: `keyword_idents_2024`, `non_ascii_idents`, `non_local_definitions`, `unsafe_op_in_unsafe_fn`

## Async Patterns

```rust
use tokio::{runtime::Handle, task::JoinHandle};

Handle::current().spawn(async move { /* work */ });

let result = crate::sliding_sync::block_on_async_with_timeout(
    Some(Duration::from_secs(3)),
    async move { /* work */ },
);
```

## Logging

```rust
use makepad_widgets::{log, error, warning};
log!("Info {:?}", var);
error!("Error: {e:?}");
warning!("Warning");
```
