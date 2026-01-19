use makepad_widgets::*;
use matrix_sdk::ruma::OwnedRoomId;

use crate::{
    app::AppState,
    kanban::{KanbanActions, KanbanFilterState, KanbanSortState, SortDirection, SortField},
};
use crate::kanban::data::models::KanbanBoard;

live_design! {

    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::kanban::ui::components::boards_sidebar::BoardsSidebar;
    use crate::kanban::ui::components::board_header::BoardHeader;
    use crate::kanban::ui::components::board_toolbar::BoardToolbar;
    use crate::kanban::ui::components::kanban_list::KanbanList;
    use crate::kanban::ui::components::kanban_card::KanbanCard;

    pub KanbanApp = {{KanbanApp}} {
        width: Fill, height: Fill
        flow: Right

        // 看板侧边栏
        sidebar = <BoardsSidebar> {}

        // 主内容区域
        main_content = <View> {
            width: Fill, height: Fill
            flow: Down

            // 看板头部
            board_header = <BoardHeader> {}

            // 工具栏
            board_toolbar = <BoardToolbar> {}

            // 看板画布
            board_canvas = <View> {
                width: Fill, height: Fill
                flow: Right
                scroll: vec2(1.0, 0.0)

                padding: 12
                spacing: 12

                // 列表容器
                lists_container = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    align: {x: 0.0, y: 0.0}

                    list_todo = <KanbanList> {
                        header = {
                            list_title = { text: "待办" }
                            card_count = { text: "2" }
                        }
                        cards_container = {
                            card_1 = <KanbanCard> { card_title = { text: "整理需求" } }
                            card_2 = <KanbanCard> { card_title = { text: "UI 结构草图" } }
                        }
                        add_card_area = { visible: false }
                    }

                    list_doing = <KanbanList> {
                        header = {
                            list_title = { text: "进行中" }
                            card_count = { text: "2" }
                        }
                        cards_container = {
                            card_1 = <KanbanCard> { card_title = { text: "接口联调" } }
                            card_2 = <KanbanCard> { card_title = { text: "交互动效" } }
                        }
                        add_card_area = { visible: false }
                    }

                    list_done = <KanbanList> {
                        header = {
                            list_title = { text: "已完成" }
                            card_count = { text: "1" }
                        }
                        cards_container = {
                            card_1 = <KanbanCard> { card_title = { text: "提测上线" } }
                        }
                        add_card_area = { visible: false }
                    }
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

impl KanbanApp {
    fn sync_from_state(&mut self, cx: &mut Cx, app_state: &AppState) {
        self.current_board_id = app_state.kanban_state.current_board_id.clone();
        self.current_board = app_state.kanban_state.current_board().cloned();
        self.boards = app_state.kanban_state.boards.values().cloned().collect();

        let board_title = self
            .current_board
            .as_ref()
            .map(|board| board.name.as_str())
            .unwrap_or("未选择看板");
        self.view
            .label(ids!(main_content.board_header.title_area.board_title))
            .set_text(cx, board_title);
    }
}

impl Widget for KanbanApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::Actions(actions) = event {
            if self
                .view
                .button(ids!(sidebar.sidebar_header.add_board_button))
                .clicked(actions)
            {
                cx.action(KanbanActions::CreateBoard {
                    name: "新建看板".to_string(),
                    description: None,
                });
            }

            if self
                .view
                .button(ids!(main_content.board_header.action_buttons.filter_button))
                .clicked(actions)
                || self
                    .view
                    .button(ids!(main_content.board_toolbar.filter_button))
                    .clicked(actions)
            {
                cx.action(KanbanActions::SetFilter(KanbanFilterState {
                    keyword: None,
                    label_ids: Vec::new(),
                    member_ids: Vec::new(),
                    due_date: None,
                }));
            }

            if self
                .view
                .button(ids!(main_content.board_header.action_buttons.sort_button))
                .clicked(actions)
                || self
                    .view
                    .button(ids!(main_content.board_toolbar.sort_button))
                    .clicked(actions)
            {
                cx.action(KanbanActions::SetSort(KanbanSortState {
                    field: SortField::Position,
                    direction: SortDirection::Ascending,
                }));
            }

            let search_input = self.view.text_input(ids!(
                main_content.board_toolbar.search_container.search_input
            ));
            if let Some(query) = search_input.changed(actions) {
                if let Some(board_id) = self.current_board_id.clone() {
                    cx.action(KanbanActions::Search { board_id, query });
                }
            }

            if self
                .view
                .button(ids!(main_content.board_toolbar.view_toggle.board_view_btn))
                .clicked(actions)
            {
                self.view_mode = KanbanViewMode::Board;
            }

            if self
                .view
                .button(ids!(main_content.board_toolbar.view_toggle.list_view_btn))
                .clicked(actions)
            {
                self.view_mode = KanbanViewMode::Table;
            }
        }

        if let Some(app_state) = scope.data.get::<AppState>() {
            self.sync_from_state(cx, app_state);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(app_state) = scope.data.get::<AppState>() {
            self.sync_from_state(cx, app_state);
        }
        self.view.draw_walk(cx, scope, walk)
    }
}
