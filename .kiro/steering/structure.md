# Project Structure

## Entry Points

- `src/main.rs`: Executable entry point, calls `app_main!()`
- `src/lib.rs`: Library root, module declarations and project directory setup
- `src/app.rs`: Top-level application state and event handling

## Module Organization

### Core Application
- `app.rs`: Main app struct, event loop, state management, modal coordination
- `persistence/`: App state, Matrix state, and TSP state serialization/deserialization
- `utils.rs`: Shared utility functions
- `temp_storage.rs`: Temporary data storage

### UI Modules
- `home/`: Main interface (home screen, room screen, rooms list, sidebar, spaces)
- `login/`: Login screen and status modal
- `logout/`: Logout confirmation modal
- `settings/`: Settings screen and account settings
- `profile/`: User profile display and caching
- `room/`: Room-specific UI (input bar, reply preview, typing notices, display filters)
- `shared/`: Reusable UI components (avatar, buttons, modals, tooltips, image viewer)

### Feature Modules
- `kanban/`: Task management functionality
  - `api/`: Kanban API requests and repository interfaces
  - `components/`: UI components (boards list, cards, modals, sections)
  - `data/`: Data models and repositories
  - `drag_drop/`: Drag-and-drop logic and order management
  - `models/`: DTOs and state models
  - `state/`: Kanban state management and actions
  - `matrix_adapter.rs`: Bridge between Kanban and Matrix protocol

### Matrix Integration
- `sliding_sync.rs`: Matrix sliding sync implementation
- `space_service_sync.rs`: Space service synchronization
- `avatar_cache.rs`: Avatar image caching
- `media_cache.rs`: Media file caching
- `verification.rs`: E2E encryption verification
- `verification_modal.rs`: Verification UI modal

### Optional Features
- `tsp/`: TSP wallet integration (when `tsp` feature enabled)
- `tsp_dummy/`: Placeholder TSP widgets (when `tsp` feature disabled)

### Supporting Modules
- `location.rs`: Location/geolocation functionality
- `event_preview.rs`: Timeline event preview generation
- `join_leave_room_modal.rs`: Room join/leave confirmation UI

## Architectural Patterns

### Makepad Live Design
- UI defined using Makepad's declarative DSL in `live_design!` macros
- Each module calls `live_design(cx)` to register widgets
- Registration order matters: `makepad_widgets` → `shared` → feature modules

### Widget References
- Widgets accessed via `WidgetRef` and ID paths using `ids!()` macro
- Example: `self.ui.modal(ids!(verification_modal)).open(cx)`

### State Management
- `AppState`: Top-level app state (selected room, logged-in status, kanban state)
- State passed to widgets via `Scope::with_data()`
- Actions propagate state changes through the widget tree

### Action System
- Widgets emit typed actions (e.g., `LoginAction`, `RoomsListAction`, `KanbanActions`)
- `handle_actions()` in `app.rs` processes all actions
- Actions can be widget-specific or global

### Async Integration
- Tokio runtime started in `handle_startup()`
- `submit_async_request()` bridges sync UI thread with async Matrix operations
- Background tasks communicate via `crossbeam-channel`

### Modal Management
- Modals wrapped in `<Modal>` component with overlay behavior
- Opened/closed via `.open(cx)` and `.close(cx)` methods
- Multiple modals: login, verification, join/leave, image viewer, tooltips

### Caching Strategy
- Thread-local caches for avatars, user profiles, timelines
- Cleared via `clear_*_cache(cx)` functions on logout
- LRU caching for media files

## Naming Conventions

- Modules: snake_case (e.g., `home_screen.rs`)
- Structs/Enums: PascalCase (e.g., `AppState`, `RoomsListAction`)
- Functions: snake_case (e.g., `handle_startup()`)
- Widget IDs: snake_case (e.g., `ids!(main_window)`)
- Live design IDs: snake_case or PascalCase depending on context

## Resource Organization

- `resources/icons/`: SVG icons
- `resources/img/`: PNG images (avatars, logos, provider icons)
- `packaging/`: Platform-specific packaging assets
- Resources copied to `dist/resources/` during build via `robius-packaging-commands`

## Configuration Files

- `Cargo.toml`: Dependencies, features, profiles, packaging metadata
- `.cargo/config.toml`: Global rustflags for ruma identifiers
- `rust-toolchain.toml`: Stable channel specification
- `rustfmt.toml`: Code formatting rules
