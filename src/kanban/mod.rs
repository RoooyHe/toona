// Kanban module for Toona Matrix chat client
// Implements Trello-style kanban boards using Matrix rooms as backend

use makepad_widgets::Cx;

pub mod data;
pub mod api;
pub mod state;
pub mod drag_drop;
pub mod matrix_adapter;
pub mod models;
pub mod components;

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

pub fn live_design(cx: &mut Cx) {
    components::live_design(cx);
}
