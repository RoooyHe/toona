use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    // 看板卡片组件
    pub KanbanCard = {{KanbanCard}} {
        flow: Down,
        width: Fill,
        min_height: 40,
        show_bg: true,
        draw_bg: {
            color: #FFFFFF
            border_radius: 3
        },
        box_shadow: {
            color: #091E420F
            x: 0, y: 1, blur: 2, spread: 0
        },
        cursor: Pointer,
        padding: 8,
        spacing: 4,

        // 卡片内容区
        content = {
            flow: Down,
            width: Fill,
            height: Fit,
            spacing: 4,
        }

        // 标签行（可选）
        labels_row = {
            flow: Right,
            width: Fill, height: Fit,
            spacing: 4,
            visible: false,
        }

        // 卡片标题
        card_title = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                color: #172B4D
            },
            wrap: Word,
            text: "卡片标题"
        }

        // 描述预览
        description_preview = <Label> {
            width: Fill, height: Fit,
            visible: false,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 12}
                color: #5E6C84
            },
            wrap: Word,
            text: ""
        }

        // 徽章行（可选）
        badges_row = {
            flow: Right,
            width: Fill, height: Fit,
            spacing: 4,
            visible: false,
        }

        // 底部信息区
        footer = {
            flow: Right,
            width: Fill, height: 24,
            align: {x: 1.0, y: 0.5},
            spacing: 4,

            // 成员头像占位
            members_row = <View> {
                width: Fit, height: 24,
                flow: Right,
                spacing: -8,
                visible: false,
            }

            // 加星标占位
            star_icon = <View> {
                width: 16, height: 16,
                visible: false,
                show_bg: true,
                draw_bg: {
                    color: #FFAB00
                    border_radius: 2
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanCard {
    #[deref]
    view: View,

    /// 卡片 ID
    #[rust]
    id: String,

    /// 卡片数据
    #[rust]
    card_data: Option<KanbanCardData>,

    /// 点击回调
    #[rust]
    on_click: Option<Box<dyn FnMut()>>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelColor {
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Blue,
    Sky,
    Lime,
    Pink,
    Black,
}

impl LabelColor {
    pub fn to_hex(&self) -> &'static str {
        match self {
            LabelColor::Green => "#61BD4F",
            LabelColor::Yellow => "#F2D600",
            LabelColor::Orange => "#FF9F1A",
            LabelColor::Red => "#EB5A46",
            LabelColor::Purple => "#C377E0",
            LabelColor::Blue => "#0079BF",
            LabelColor::Sky => "#00C2E0",
            LabelColor::Lime => "#51E898",
            LabelColor::Pink => "#FF78CB",
            LabelColor::Black => "#344563",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardLabel {
    pub id: String,
    pub color: LabelColor,
    pub name: String,
}

impl KanbanCard {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            view: View::new(cx),
            id: String::new(),
            card_data: None,
            on_click: None,
        }
    }

    pub fn set_card(&mut self, card: &KanbanCardData) {
        self.id = card.id.clone();
        self.card_data = Some(card.clone());
    }

    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_click = Some(Box::new(callback));
    }
}

impl Widget for KanbanCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.view.handle_event(cx, event, _scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
