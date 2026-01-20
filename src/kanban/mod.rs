// Kanban module for Toona Matrix chat client
// Implements Trello-style kanban boards using Matrix rooms as backend

use makepad_widgets::Cx;

pub mod data;
pub mod api;
pub mod state;
pub mod drag_drop;

// Re-export main types for convenience
pub use data::models::*;
pub use state::kanban_state::*;
pub use state::kanban_actions::*;

pub fn live_design(_cx: &mut Cx) {
    // Kanban live design is handled by home screen components
}
