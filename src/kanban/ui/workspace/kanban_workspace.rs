use crate::kanban::data::models::KanbanBoard;

#[derive(Debug, Clone, Default)]
pub struct KanbanWorkspace {
    /// 当前看板
    pub current_board: Option<KanbanBoard>,

    /// 看板列表
    pub boards: Vec<KanbanBoard>,

    /// 侧边栏可见性
    pub sidebar_visible: bool,
}

impl KanbanWorkspace {
    pub fn new() -> Self {
        Self {
            current_board: None,
            boards: Vec::new(),
            sidebar_visible: true,
        }
    }

    pub fn set_boards(&mut self, boards: Vec<KanbanBoard>) {
        self.boards = boards;
    }

    pub fn set_current_board(&mut self, board: &KanbanBoard) {
        self.current_board = Some(board.clone());
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
    }
}
