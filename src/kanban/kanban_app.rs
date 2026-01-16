use makepad_widgets::*;
use matrix_sdk::ruma::OwnedRoomId;
use crate::kanban::data::models::KanbanBoard;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub KanbanApp = {{KanbanApp}} {
        width: Fill, height: Fill
        flow: Right

        // 看板侧边栏
        sidebar = <BoardsSidebar> {}

        // 主内容区域
        main_content = {
            width: Fill, height: Fill
            flow: Down

            // 看板头部
            board_header = <BoardHeader> {}

            // 工具栏
            board_toolbar = <BoardToolbar> {}

            // 看板画布
            board_canvas = {
                width: Fill, height: Fill
                flow: Right
                scroll: {x: true, y: false}
                padding: 12
                spacing: 12

                // 列表容器
                lists_container = {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    align: {x: 0.0, y: 0.0}
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum KanbanViewMode {
    #[default]
    Board,
    Calendar,
    Timeline,
    Gallery,
    Table,
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanApp {
    #[deref]
    view: View,

    /// 当前看板 ID
    #[rust]
    current_board_id: Option<OwnedRoomId>,

    /// 当前看板数据
    #[rust]
    current_board: Option<KanbanBoard>,

    /// 看板列表
    #[rust]
    boards: Vec<KanbanBoard>,

    /// 视图模式
    #[rust]
    view_mode: KanbanViewMode,

    /// 侧边栏可见性
    #[rust]
    sidebar_visible: bool,

    /// 回调
    #[rust]
    on_board_select: Option<Box<dyn FnMut(&OwnedRoomId)>>,
    #[rust]
    on_create_board: Option<Box<dyn FnMut()>>,
    #[rust]
    on_add_list: Option<Box<dyn FnMut()>>,
    #[rust]
    on_add_card: Option<Box<dyn FnMut(&str)>>,
    #[rust]
    on_card_click: Option<Box<dyn FnMut(&str)>>,
    #[rust]
    on_menu_open: Option<Box<dyn FnMut()>>,
}

impl KanbanApp {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            view: View::new(cx),
            current_board_id: None,
            current_board: None,
            boards: Vec::new(),
            view_mode: KanbanViewMode::Board,
            sidebar_visible: true,
            on_board_select: None,
            on_create_board: None,
            on_add_list: None,
            on_add_card: None,
            on_card_click: None,
            on_menu_open: None,
        }
    }

    pub fn set_current_board(&mut self, board: Option<&KanbanBoard>) {
        self.current_board = board.cloned();
        self.current_board_id = board.as_ref().map(|b| b.id.clone());
    }

    pub fn set_boards(&mut self, boards: Vec<KanbanBoard>) {
        self.boards = boards;
    }

    pub fn set_view_mode(&mut self, mode: KanbanViewMode) {
        self.view_mode = mode;
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
    }

    pub fn set_on_board_select<F>(&mut self, callback: F)
    where
        F: FnMut(&OwnedRoomId) + 'static,
    {
        self.on_board_select = Some(Box::new(callback));
    }

    pub fn set_on_create_board<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_create_board = Some(Box::new(callback));
    }

    pub fn set_on_add_list<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_add_list = Some(Box::new(callback));
    }

    pub fn set_on_add_card<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + 'static,
    {
        self.on_add_card = Some(Box::new(callback));
    }

    pub fn set_on_card_click<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + 'static,
    {
        self.on_card_click = Some(Box::new(callback));
    }

    pub fn set_on_menu_open<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_menu_open = Some(Box::new(callback));
    }
}

impl Widget for KanbanApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.view.handle_event(cx, event, _scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}