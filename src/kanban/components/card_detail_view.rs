use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 简化的卡片详情查看器
    pub CardDetailView = {{CardDetailView}} {
        <RoundedView> {
            width: 600,
            height: 400,
            padding: 20,
            flow: Down,
            spacing: 15,
            draw_bg: {
                color: #FFFFFF
            }

            // 标题栏
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                align: {y: 0.5},

                <Label> {
                    text: "卡片详情",
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 20}
                        color: #172B4D
                    }
                }

                <View> { width: Fill, height: Fit }

                close_button = <Button> {
                    text: "关闭",
                    width: 60,
                    height: 35,
                }
            }

            // 卡片标题
            <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5,

                <Label> {
                    text: "标题",
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 14}
                        color: #5E6C84
                    }
                }

                card_title_label = <Label> {
                    width: Fill,
                    height: Fit,
                    text: "卡片标题",
                    draw_text: {
                        text_style: <THEME_FONT_REGULAR>{font_size: 16}
                        color: #172B4D
                    }
                }
            }

            // 卡片描述
            <View> {
                width: Fill,
                height: Fill,
                flow: Down,
                spacing: 5,

                <Label> {
                    text: "描述",
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 14}
                        color: #5E6C84
                    }
                }

                <ScrollYView> {
                    width: Fill,
                    height: Fill,
                    scroll_bars: <ScrollBars> {
                        show_scroll_y: true
                    }

                    card_description_label = <Label> {
                        width: Fill,
                        height: Fit,
                        text: "暂无描述",
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                            wrap: Word
                        }
                    }
                }
            }

            // 操作按钮
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,

                edit_button = <Button> {
                    text: "编辑",
                    width: 80,
                    height: 35,
                    draw_bg: {
                        color: #0079BF
                    }
                    draw_text: {
                        color: #FFFFFF
                    }
                }

                delete_button = <Button> {
                    text: "删除",
                    width: 80,
                    height: 35,
                    draw_bg: {
                        color: #FF6B6B
                    }
                    draw_text: {
                        color: #FFFFFF
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardDetailView {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
}

impl Widget for CardDetailView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理关闭按钮
            if self.view.button(ids!(close_button)).clicked(actions) {
                log!("CardDetailView: 关闭按钮被点击");
                // 发送关闭事件
                cx.action(CardDetailViewAction::Close);
            }
            
            // 处理编辑按钮
            if self.view.button(ids!(edit_button)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("CardDetailView: 编辑按钮被点击, card_id={}", card_id);
                    // TODO: 打开编辑模式
                }
            }
            
            // 处理删除按钮
            if self.view.button(ids!(delete_button)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("CardDetailView: 删除按钮被点击, card_id={}", card_id);
                    cx.action(crate::kanban::KanbanActions::DeleteCard {
                        card_id: card_id.clone(),
                    });
                    cx.action(CardDetailViewAction::Close);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 scope.props 获取 card_id
        if let Some(card_id) = scope.props.get::<matrix_sdk::ruma::OwnedRoomId>() {
            self.card_id = Some(card_id.clone());
            
            // 从 AppState 获取卡片数据
            if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                if let Some(card) = app_state.kanban_state.cards.get(card_id) {
                    // 设置标题
                    self.view
                        .label(ids!(card_title_label))
                        .set_text(cx, &card.title);
                    
                    // 设置描述
                    let description = card.description.as_deref().unwrap_or("暂无描述");
                    self.view
                        .label(ids!(card_description_label))
                        .set_text(cx, description);
                }
            }
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

/// 卡片详情视图的 Action
#[derive(Clone, Debug, DefaultNone)]
pub enum CardDetailViewAction {
    Close,
    None,
}
