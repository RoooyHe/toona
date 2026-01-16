use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    // 看板工具栏组件
    pub BoardToolbar = {{BoardToolbar}} {
        flow: Right,
        width: Fill,
        height: 40,
        align: {x: 0.0, y: 0.5},
        spacing: 4,
        padding: 8,
        show_bg: true,
        draw_bg: { color: #F4F5F7 },

        // 筛选按钮
        filter_button = <Button> {
            width: Fit, height: 32,
            padding: 8,
            show_bg: true,
            draw_bg: {
                color: #FFFFFF
                border_radius: 3
                border_size: 1
                border_color: #DFE1E6
            },
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 13}
                color: #5E6C84
            },
            text: "筛选"
        }

        // 排序按钮
        sort_button = <Button> {
            width: Fit, height: 32,
            padding: 8,
            show_bg: true,
            draw_bg: {
                color: #FFFFFF
                border_radius: 3
                border_size: 1
                border_color: #DFE1E6
            },
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 13}
                color: #5E6C84
            },
            text: "排序"
        }

        // 搜索框容器
        search_container = {
            width: 180, height: 32,
            flow: Right,
            margin: {left: 8},
            show_bg: true,
            draw_bg: {
                color: #FFFFFF
                border_radius: 3
                border_size: 1
                border_color: #DFE1E6
            },
            padding: 4,

            search_input = <TextInput> {
                width: Fill, height: Fill,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    color: #172B4D
                },
                placeholder: "搜索卡片..."
            }
        }

        // 视图切换
        view_toggle = {
            width: Fit, height: 32,
            flow: Right,
            margin: {left: 8},
            spacing: 0,

            board_view_btn = <Button> {
                width: 32, height: 32,
                show_bg: true,
                draw_bg: { color: #FFFFFF },
                draw_text: { color: #5E6C84 },
                text: "▦"
            }

            list_view_btn = <Button> {
                width: 32, height: 32,
                show_bg: true,
                draw_bg: { color: #EBECF0 },
                draw_text: { color: #5E6C84 },
                text: "☰"
            }
        }

        // 归档按钮
        archive_button = <Button> {
            width: Fit, height: 32,
            padding: 8,
            margin: {left: 8},
            show_bg: true,
            draw_bg: {
                color: #FFFFFF
                border_radius: 3
                border_size: 1
                border_color: #DFE1E6
            },
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 13}
                color: #5E6C84
            },
            text: "归档"
        }

        // 更多按钮
        more_button = <Button> {
            width: 32, height: 32,
            show_bg: true,
            draw_bg: {
                color: #FFFFFF
                border_radius: 3
                border_size: 1
                border_color: #DFE1E6
            },
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 16}
                color: #5E6C84
            },
            text: "..."
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardToolbar {
    #[deref]
    view: View,

    /// 搜索关键词
    #[rust]
    search_query: String,

    /// 当前视图模式
    #[rust]
    view_mode: BoardViewMode,

    /// 回调
    #[rust]
    on_filter: Option<Box<dyn FnMut()>>,
    #[rust]
    on_sort: Option<Box<dyn FnMut()>>,
    #[rust]
    on_search: Option<Box<dyn FnMut(String)>>,
    #[rust]
    on_view_change: Option<Box<dyn FnMut(BoardViewMode)>>,
    #[rust]
    on_archive: Option<Box<dyn FnMut()>>,
    #[rust]
    on_more: Option<Box<dyn FnMut()>>,
}

impl BoardToolbar {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            view: View::new(cx),
            search_query: String::new(),
            view_mode: BoardViewMode::Board,
            on_filter: None,
            on_sort: None,
            on_search: None,
            on_view_change: None,
            on_archive: None,
            on_more: None,
        }
    }

    pub fn get_search_query(&self) -> &str {
        &self.search_query
    }

    pub fn get_view_mode(&self) -> BoardViewMode {
        self.view_mode
    }

    pub fn set_on_filter<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_filter = Some(Box::new(callback));
    }

    pub fn set_on_sort<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_sort = Some(Box::new(callback));
    }

    pub fn set_on_search<F>(&mut self, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        self.on_search = Some(Box::new(callback));
    }

    pub fn set_on_view_change<F>(&mut self, callback: F)
    where
        F: FnMut(BoardViewMode) + 'static,
    {
        self.on_view_change = Some(Box::new(callback));
    }

    pub fn set_on_archive<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_archive = Some(Box::new(callback));
    }

    pub fn set_on_more<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_more = Some(Box::new(callback));
    }
}

/// 视图模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoardViewMode {
    #[default]
    Board, // 看板视图
    List,  // 列表视图
}

impl Widget for BoardToolbar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.view.handle_event(cx, event, _scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
