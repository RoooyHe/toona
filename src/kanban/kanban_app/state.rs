use matrix_sdk::ruma::OwnedRoomId;

use crate::kanban::data::models::KanbanBoard;

#[derive(Debug, Clone, Default)]
pub enum KanbanViewMode {
    #[default]
    Board,
    Calendar,
    Timeline,
    Gallery,
    Table,
}

#[derive(Debug, Clone)]
pub struct KanbanViewState {
    pub current_board_id: Option<OwnedRoomId>,
    pub current_board: Option<KanbanBoard>,
    pub boards: Vec<KanbanBoard>,
    pub view_mode: KanbanViewMode,
    pub sidebar_visible: bool,
}

impl Default for KanbanViewState {
    fn default() -> Self {
        Self {
            current_board_id: None,
            current_board: None,
            boards: Vec::new(),
            view_mode: KanbanViewMode::Board,
            sidebar_visible: true,
        }
    }
}
