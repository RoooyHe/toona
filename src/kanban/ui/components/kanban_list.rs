use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    // 看板列表组件
    pub KanbanList = {{KanbanList}} {
        flow: Down,
        width: 272,
        show_bg: true,
        draw_bg: {
            color: #EBECF0
            border_radius: 3
        },

        // 列表头部
        header = {
            flow: Right,
            width: Fill,
            height: 32,
            align: {x: 0.0, y: 0.5},
            padding: 8,
            spacing: 4,

            // 列表标题
            list_title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #172B4D
                },
                text: "列表名称"
            }

            // 卡片数量
            card_count = <Label> {
                width: Fit, height: Fit,
                margin: {left: 4},
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    color: #5E6C84
                },
                text: ""
            }

            // 更多按钮
            more_button = <Button> {
                width: 24, height: 24,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    color: #5E6C84
                },
                text: "..."
            }
        }

        // 卡片容器
        cards_container = {
            flow: Down,
            width: Fill,
            height: Fit,
            max_height: 800,
            padding: 4,
            spacing: 4,
        }

        // 添加卡片区域
        add_card_area = {
            flow: Down,
            width: Fill,
            padding: 4,

            quick_add_input = <TextInput> {
                width: Fill, height: 32,
                show_bg: true,
                draw_bg: {
                    color: #FFFFFF
                    border_radius: 3
                },
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    color: #172B4D
                },
                placeholder: "添加卡片..."
            }

            add_button_row = {
                flow: Right,
                width: Fill, height: Fit,
                spacing: 4,
                margin: {top: 4},

                add_button = <Button> {
                    width: Fit, height: 32,
                    padding: 8,
                    show_bg: true,
                    draw_bg: {
                        color: #0079BF
                        border_radius: 3
                    },
                    draw_text: {
                        text_style: <THEME_FONT_REGULAR>{font_size: 13}
                        color: #FFFFFF
                    },
                    text: "添加卡片"
                }

                cancel_button = <Button> {
                    width: 32, height: 32,
                    show_bg: true,
                    draw_bg: {
                        color: #EBECF0
                        border_radius: 3
                    },
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 16}
                        color: #5E6C84
                    },
                    text: "×"
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanList {
    #[deref]
    view: View,

    /// 列表 ID
    #[rust]
    id: String,

    /// 列表数据
    #[rust]
    list_data: Option<KanbanListData>,

    /// 回调
    #[rust]
    on_card_click: Option<Box<dyn FnMut(String)>>,
    #[rust]
    on_add_card: Option<Box<dyn FnMut(String)>>,
    #[rust]
    on_list_more: Option<Box<dyn FnMut()>>,
}

#[derive(Debug, Clone)]
pub struct KanbanListData {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub cards: Vec<KanbanCardData>,
    pub card_count: u32,
}

#[derive(Debug, Clone)]
pub struct KanbanCardData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub label_ids: Vec<String>,
    pub member_ids: Vec<String>,
    pub due_date: Option<String>,
    pub cover_url: Option<String>,
    pub comment_count: u32,
    pub is_archived: bool,
}

impl KanbanList {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            view: View::new(cx),
            id: String::new(),
            list_data: None,
            on_card_click: None,
            on_add_card: None,
            on_list_more: None,
        }
    }

    pub fn set_list(&mut self, list: &KanbanListData) {
        self.id = list.id.clone();
        self.list_data = Some(list.clone());
    }

    pub fn set_on_card_click<F>(&mut self, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        self.on_card_click = Some(Box::new(callback));
    }

    pub fn set_on_add_card<F>(&mut self, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        self.on_add_card = Some(Box::new(callback));
    }

    pub fn set_on_list_more<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_list_more = Some(Box::new(callback));
    }
}

impl Widget for KanbanList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.view.handle_event(cx, event, _scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
