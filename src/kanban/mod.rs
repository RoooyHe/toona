// Kanban module for Toona Matrix chat client
// Implements Trello-style kanban boards using Matrix rooms as backend

use makepad_widgets::Cx;

pub mod data;
pub mod api;
pub mod state;
pub mod drag_drop;
pub mod matrix_adapter;

// The following modules are from the old standalone betula app
// and are currently disabled due to incompatibility:
// - components (uses old Makepad API and different State structure)
// - models (DTO models for external API)
// - services (uses reqwest::blocking which requires feature flag)
//
// The actual Toona kanban UI is in src/home/kanban_*.rs files

// Re-export main types for convenience
pub use data::models::*;
pub use state::kanban_state::*;
pub use state::kanban_actions::*;
pub use matrix_adapter::{
    MatrixKanbanAdapter,
    KanbanCardMetadata,
    KanbanBoardMetadata,
    KANBAN_CARD_EVENT_TYPE,
    KANBAN_LIST_EVENT_TYPE,
    KANBAN_BOARD_EVENT_TYPE,
};

pub fn live_design(_cx: &mut Cx) {
    // Kanban UI components are registered in home/mod.rs
}
