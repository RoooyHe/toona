use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::shared::styles::*;

    pub CardItem = {{CardItem}} {
        <RoundedView> {
            width: Fill,
            height: Fit,
            margin: {bottom: 10},
            padding: 15,
            flow: Down,
            spacing: 8,
            draw_bg: {
                color: #FFFFFFFF
            }

            // 标题显示区域
            title_display_container = <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                align: {y: 0.5},
                spacing: 10,
                visible: true,

                card_title_label = <Label> {
                    width: Fill,
                    height: Fit,
                    text: "卡片标题",
                    draw_text: {
                        color: #333333FF,
                        text_style: {
                            font_size: 14.0,
                        }
                    }
                }

                <View> {
                    width: Fit,
                    height: Fit,
                    flow: Right,
                    spacing: 5,

                    edit_title_btn = <Button> {
                        width: 50,
                        height: 30,
                        text: "编辑",
                        draw_bg: {
                            color: #FFA500FF
                        }
                        draw_text: {
                            color: #FFFFFF
                        }
                    }

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

            // 标题编辑区域
            title_edit_container = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5,
                visible: false,

                card_title_input = <SimpleTextInput> {
                    width: Fill,
                    height: 35,
                    text: "",
                }

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 5,

                    save_title_btn = <Button> {
                        width: 50,
                        height: 30,
                        text: "保存",
                        draw_bg: {
                            color: #45B7D1
                        }
                        draw_text: {
                            color: #FFFFFF
                        }
                    }

                    cancel_title_btn = <Button> {
                        width: 50,
                        height: 30,
                        text: "取消",
                        draw_bg: {
                            color: #95A5A6
                        }
                        draw_text: {
                            color: #FFFFFF
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
    #[rust]
    is_editing: bool,
    #[rust]
    original_title: String,
    #[rust]
    drag_start_pos: Option<DVec2>,
    #[rust]
    is_dragging: bool,
}

impl Widget for CardItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // 先让子组件处理事件（按钮、输入框等）
        self.view.handle_event(cx, event, scope);

        // 只有在不处于编辑模式时才处理拖拽事件
        if !self.is_editing {
            match event {
                Event::MouseDown(e) => {
                    // 检查是否点击在按钮上
                    let edit_btn_hit = self
                        .view
                        .button(ids!(edit_title_btn))
                        .area()
                        .rect(cx)
                        .contains(e.abs);
                    let detail_btn_hit = self
                        .view
                        .button(ids!(detail_btn))
                        .area()
                        .rect(cx)
                        .contains(e.abs);
                    let delete_btn_hit = self
                        .view
                        .button(ids!(delete_card_btn))
                        .area()
                        .rect(cx)
                        .contains(e.abs);

                    // 如果点击在按钮上，不启动拖拽
                    if !edit_btn_hit && !detail_btn_hit && !delete_btn_hit {
                        self.drag_start_pos = Some(e.abs);
                    }
                }
                Event::MouseMove(e) => {
                    if let Some(start_pos) = self.drag_start_pos {
                        let distance = (e.abs - start_pos).length();

                        // 移动距离超过 5px 时开始拖拽
                        if distance > 5.0 && !self.is_dragging {
                            if let Some(card_id_str) = &self.card_id {
                                if let Ok(card_id) =
                                    matrix_sdk::ruma::RoomId::parse(card_id_str.as_str())
                                {
                                    log!("CardItem: 开始拖拽卡片 {}", card_id);

                                    // 从 AppState 获取卡片信息
                                    if let Some(app_state) =
                                        scope.data.get::<crate::app::AppState>()
                                    {
                                        if let Some(card) =
                                            app_state.kanban_state.cards.get(&card_id)
                                        {
                                            cx.action(
                                                crate::kanban::KanbanActions::StartDragCard {
                                                    card_id: card_id.clone(),
                                                    space_id: card.space_id.clone(),
                                                    position: card.position,
                                                },
                                            );
                                            self.is_dragging = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Event::MouseUp(_) => {
                    if self.is_dragging {
                        log!("CardItem: 结束拖拽");
                        // 拖拽结束由 Space 组件处理（检测放置目标）
                        self.is_dragging = false;
                    }
                    self.drag_start_pos = None;
                }
                _ => {}
            }
        }

        if let Event::Actions(actions) = event {
            // 处理编辑按钮点击 - 显示编辑区域
            if self.view.button(ids!(edit_title_btn)).clicked(actions) {
                self.is_editing = true;
                // 保存原始标题
                self.original_title = self.view.label(ids!(card_title_label)).text();
                // 设置输入框文本
                self.view
                    .text_input(ids!(card_title_input))
                    .set_text(cx, &self.original_title);
                // 显示编辑区域，隐藏显示区域
                self.view
                    .view(ids!(title_edit_container))
                    .set_visible(cx, true);
                self.view
                    .view(ids!(title_display_container))
                    .set_visible(cx, false);
                // 设置焦点到输入框
                self.view
                    .text_input(ids!(card_title_input))
                    .set_key_focus(cx);
                log!("CardItem: 开始编辑卡片标题");
            }

            // 处理保存按钮点击
            if self.view.button(ids!(save_title_btn)).clicked(actions) {
                let new_title = self.view.text_input(ids!(card_title_input)).text();
                if let Some(card_id) = &self.card_id {
                    if !new_title.trim().is_empty() && new_title.trim() != self.original_title {
                        log!(
                            "CardItem: 保存卡片标题: '{}' (卡片ID: {})",
                            new_title.trim(),
                            card_id
                        );

                        // 解析 card_id 为 OwnedRoomId
                        if let Ok(room_id) = matrix_sdk::ruma::RoomId::parse(card_id.as_str()) {
                            cx.action(crate::kanban::KanbanActions::UpdateCardTitle {
                                card_id: room_id,
                                title: new_title.trim().to_string(),
                            });
                        }
                        // 更新显示的标题
                        self.view
                            .label(ids!(card_title_label))
                            .set_text(cx, new_title.trim());
                    }
                }
                // 隐藏编辑区域，显示显示区域
                self.view
                    .view(ids!(title_edit_container))
                    .set_visible(cx, false);
                self.view
                    .view(ids!(title_display_container))
                    .set_visible(cx, true);
                self.is_editing = false;
            }

            // 处理取消按钮点击
            if self.view.button(ids!(cancel_title_btn)).clicked(actions) {
                log!("CardItem: 取消编辑卡片标题");
                // 隐藏编辑区域，显示显示区域
                self.view
                    .view(ids!(title_edit_container))
                    .set_visible(cx, false);
                self.view
                    .view(ids!(title_display_container))
                    .set_visible(cx, true);
                self.is_editing = false;
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
                        cx.action(crate::kanban::KanbanActions::DeleteCard { card_id: room_id });
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
