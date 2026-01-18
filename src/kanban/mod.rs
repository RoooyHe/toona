// Kanban module for Toona Matrix chat client
// Implements Trello-style kanban boards using Matrix rooms as backend

use makepad_widgets::Cx;

pub mod kanban_app;
pub mod data;
pub mod state;
pub mod api;
pub mod ui;
pub mod drag_drop;

// Re-export main types for convenience
pub use kanban_app::KanbanApp;
pub use data::models::*;
pub use state::kanban_state::*;
pub use state::kanban_actions::*;

pub fn live_design(cx: &mut Cx) {
    ui::live_design(cx);
    kanban_app::live_design(cx);
}
