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
pub mod local_cache;

// Re-export main types for convenience
// 使用简化的数据模型
pub use state::kanban_state::{KanbanList, KanbanCard, KanbanAppState};
pub use state::kanban_actions::KanbanActions;
pub use matrix_adapter::MatrixKanbanAdapter;

pub fn live_design(cx: &mut Cx) {
    components::live_design(cx);
}
