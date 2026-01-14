# AGENTS.md - Robrix Matrix Chat Client

This file contains essential information for agentic coding agents working on the Robrix Matrix chat client codebase.

## Project Overview

**Robrix** is a multi-platform Matrix chat client written in pure Rust using the Makepad UI framework and the Project Robius app development framework. It runs on macOS, Windows, Linux, Android, and iOS.

- **Language**: Rust (edition 2024)
- **UI Framework**: Makepad
- **Main Dependencies**: matrix-sdk, matrix-sdk-ui, robius-* crates
- **Build System**: Cargo with custom profiles
- **Architecture**: Modular with separate modules for UI components, Matrix integration, and app logic

## Build Commands

### Basic Commands
```bash
# Standard debug build
cargo build

# Release build  
cargo build --release

# Run the application
cargo run

# Run specific binary with features
cargo run --features tsp

# Check compilation without building
cargo check
```

### Testing Commands
```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test utils

# Run single test by name
cargo test test_human_readable_list_empty

# Run tests with output
cargo test -- --nocapture

# Run tests with specific features
cargo test --features tsp

# Run only unit tests (ignore integration tests)
cargo test --lib
```

### Linting and Quality
```bash
# Run clippy lints
cargo clippy

# Run clippy with all targets and features
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Check format without applying changes
cargo fmt -- --check
```

### Custom Build Profiles
```bash
# Optimized development build with debug info
cargo build --profile debug-opt

# Release with thin LTO
cargo build --profile release-lto

# Distribution build with fat LTO
cargo build --profile distribution
```

## Code Style Guidelines

### Imports
- Group imports: std external, then third-party, then local modules
- Use `use crate::` for local module imports
- Keep imports organized and remove unused imports
- Complex imports with many items should use `{}` grouping

```rust
use std::{borrow::Cow, ops::{Deref, DerefMut}, time::SystemTime};
use serde::{Deserialize, Serialize};
use makepad_widgets::*;
use matrix_sdk::ruma::{OwnedRoomId, RoomId};
use crate::{
    avatar_cache::clear_avatar_cache,
    home::{main_desktop_ui::MainDesktopUiAction, /* ... */},
};
```

### Naming Conventions
- **Modules**: `snake_case` (e.g., `utils.rs`, `app_state.rs`)
- **Types/Structs**: `PascalCase` (e.g., `RoomNameId`, `AppState`)
- **Functions/Methods**: `snake_case` (e.g., `human_readable_list`, `load_png_or_jpg`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `COLOR_PRIMARY`, `ICON_ADD`)
- **Live Design IDs**: `snake_case` for widgets, `PascalCase` for components

### Error Handling
- Use `Result<T, Error>` for fallible operations
- Leverage `anyhow::Result<T>` for application-level errors
- Use `Option<T>` for values that may be absent
- Chain errors with context using `.context()` from anyhow
- Custom error types should implement `std::error::Error` and `Display`

```rust
pub fn load_png_or_jpg(img: &ImageRef, cx: &mut Cx, data: &[u8]) -> Result<(), ImageError> {
    // Implementation with proper error handling
    match imghdr::from_bytes(data) {
        Some(imghdr::Type::Png) => img.load_png_from_data(cx, data),
        _ => {
            error!("Unsupported image format");
            Err(ImageError::UnsupportedFormat)
        }
    }
}
```

### Documentation and Comments
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Keep comments concise and purposeful
- Document complex algorithms and non-obvious logic
- Include examples in documentation where helpful

### Live Design (Makepad DSL)
- Use consistent indentation (4 spaces)
- Group related properties together
- Use meaningful widget IDs
- Follow the established pattern for styling and themes
- Reference constants defined in `shared/styles.rs`

```rust
live_design! {
    use link::theme::*;
    use crate::shared::styles::*;
    
    MyWidget = <View> {
        width: Fill,
        height: Fit,
        
        draw_bg: {
            color: (COLOR_SECONDARY)
        }
        
        content = <View> {
            flow: Down,
            padding: 10.0,
            spacing: 5.0
        }
    }
}
```

### Testing
- Write unit tests for utility functions and complex logic
- Use `#[cfg(test)]` for test-only code
- Test edge cases and error conditions
- Use descriptive test names that explain what they test
- Group related tests in modules

```rust
#[cfg(test)]
mod tests_human_readable_list {
    use super::*;
    
    #[test]
    fn test_human_readable_list_empty() {
        let names: Vec<&str> = Vec::new();
        let result = human_readable_list(&names, 3);
        assert_eq!(result, "");
    }
}
```

### Async and Concurrency
- Use `tokio` runtime for async operations
- Handle async results properly with `?` operator or explicit error handling
- Use `tokio::spawn` for background tasks
- Leverage Matrix SDK's async patterns for timeline operations

### Feature Flags
- Respect feature gate organization (`#[cfg(feature = "tsp")]`)
- Provide dummy implementations for optional features
- Test code paths with and without features enabled

## Project Structure

### Core Modules
- `app.rs` - Main application entry point and state management
- `utils.rs` - Shared utility functions and helpers
- `lib.rs` - Library exports and project configuration

### UI Modules
- `home/` - Main home screen, room list, navigation
- `room/` - Individual room view and message handling
- `shared/` - Reusable UI components and widgets
- `login/` - Authentication and login screens
- `settings/` - Application settings and preferences

### Integration Modules
- `sliding_sync.rs` - Matrix Sliding Sync protocol
- `avatar_cache.rs` - Avatar image caching
- `media_cache.rs` - Media file caching
- `persistence/` - State serialization and storage

### Optional Features
- `tsp/` - Trust Spanning Protocol wallet integration
- `verification/` - User/device verification flows

## Key Dependencies and Patterns

### Matrix SDK Integration
- Use `matrix-sdk` for core Matrix operations
- Leverage `matrix-sdk-ui` for timeline management
- Handle async Matrix operations with proper error handling
- Use sliding sync for efficient timeline updates

### Makepad Patterns
- Use `live_design!` macro for UI definitions
- Implement `LiveHook` for widget lifecycle
- Use `WidgetRef` for widget references
- Handle events through the event system

### State Management
- Use `AppState` for global application state
- Leverage `OnceLock` for singletons
- Implement proper serialization for persistent state
- Use async channels for inter-component communication

## Development Tips

1. **Always run `cargo check` and `cargo clippy` before committing**
2. **Test Matrix-related operations carefully** - they involve network I/O
3. **Use the existing patterns for new UI components** - don't reinvent widget structures
4. **Mind the async/await boundaries** between UI and Matrix operations
5. **Respect feature flags** when adding new functionality
6. **Use the utility functions in `utils.rs`** instead of duplicating logic

## Common Gotchas

- Makepad's live design syntax has specific requirements - check existing examples
- Matrix SDK operations are async and require proper runtime setup
- Image handling needs special attention to formats and caching
- Cross-platform compatibility requires careful resource management
- Feature flags affect which code gets compiled - test all relevant configurations