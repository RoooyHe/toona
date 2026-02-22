use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub CardItem = {{CardItem}} {
        <RoundedView> {
            width: Fill,
            height: 100,
            margin: {bottom: 10},
            padding: 15,
            flow: Down,
            spacing: 8,
            draw_bg: {
                color: #FFFFFFFF
            }

            <RoundedView> {
                width: Fill,
                height: Fit,
                flow: Right,
                align: {y: 0.5},

                card_title_input = <TextInput> {
                    width: Fill,
                    height: 35,
                    text: "卡片标题",
                    draw_text: {
                        color: #333333FF,
                        text_style: {
                            font_size: 16.0,
                        }
                    }
                    draw_bg: {
                        color: #F8F9FAFF
                    }
                    draw_cursor: {
                        color: #333333FF
                    }
                }

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 5,

                    detail_btn = <Button> {
                        width: 50,
                        height: 30,
                        text: "详情",
                        draw_bg: {
                            color: #x4ECDC4
                        }
                        draw_text: {
                            color: #FFFFFF
                        }
                    }

                    delete_card_btn = <Button> {
                        width: 50,
                        height: 30,
                        text: "删除",
                        draw_bg: {
                            color: #FF6B6BFF
                        }
                        draw_text: {
                            color: #FFFFFFFF
                        }
                    }
                }
            }

            card_tags = <Label> {
                width: Fill,
                height: 20,
                text: "标签",
                draw_text: {
                    color: #666666FF,
                    text_style: {
                        font_size: 12.0,
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardItem {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<String>,
}

impl Widget for CardItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理卡片标题输入框事件
        if let Event::Actions(actions) = event {
            // 处理卡片标题输入框回车 - 更新卡片标题
            if let Some((text, _)) = self.view.text_input(ids!(card_title_input)).returned(actions) {
                if let Some(card_id) = &self.card_id {
                    if !text.trim().is_empty() {
                        log!("CardItem: 更新卡片标题: '{}' (卡片ID: {})", text.trim(), card_id);
                        
                        // 解析 card_id 为 OwnedRoomId
                        if let Ok(room_id) = matrix_sdk::ruma::RoomId::parse(card_id.as_str()) {
                            cx.action(crate::kanban::KanbanActions::UpdateCardTitle {
                                card_id: room_id,
                                title: text.trim().to_string(),
                            });
                        }
                    }
                }
            }
            
            // 处理详情按钮点击
            if self.view.button(ids!(detail_btn)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("CardItem: 显示卡片详情，卡片ID: {}", card_id);
                    
                    // 解析 card_id 为 OwnedRoomId
                    if let Ok(room_id) = matrix_sdk::ruma::RoomId::parse(card_id.as_str()) {
                        cx.action(crate::kanban::KanbanActions::ShowCardDetail {
                            card_id: room_id,
                        });
                    }
                    cx.redraw_all();
                }
            }
            
            // 处理删除按钮点击
            if self.view.button(ids!(delete_card_btn)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("CardItem: 删除卡片 {}", card_id);
                    
                    // 解析 card_id 为 OwnedRoomId
                    if let Ok(room_id) = matrix_sdk::ruma::RoomId::parse(card_id.as_str()) {
                        cx.action(crate::kanban::KanbanActions::DeleteCard {
                            card_id: room_id,
                        });
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 scope.props 获取 card_id (OwnedRoomId)
        if let Some(card_id) = scope.props.get::<matrix_sdk::ruma::OwnedRoomId>() {
            self.card_id = Some(card_id.to_string());
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}
