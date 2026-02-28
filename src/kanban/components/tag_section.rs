use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 单个标签
    TagItem = {{TagItem}} {
        width: Fit,
        height: Fit,
        flow: Right,
        spacing: 8,
        align: {y: 0.5},
        padding: {top: 8, bottom: 8, left: 15, right: 15},
        margin: {right: 10, bottom: 10},
        draw_bg: {
            color: #x4ECDC4,
            radius: 16.0,
        }

        // 标签文本
        tag_text = <Label> {
            width: Fit,
            height: Fit,
            text: "标签",
            draw_text: {
                color: #FFFFFF,
                text_style: <THEME_FONT_REGULAR>{font_size: 15}
            }
        }

        // 删除按钮
        remove_btn = <Button> {
            width: 24,
            height: 24,
            margin: {left: 5},
            text: "×",
            draw_bg: {
                color: #00000000,
            }
            draw_text: {
                color: #FFFFFF,
                text_style: <THEME_FONT_BOLD>{font_size: 20}
            }
        }
    }

    // 标签管理区域
    pub TagSection = {{TagSection}} {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10,

        // 标题栏
        <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 10,
            align: {y: 0.5},

            <Label> {
                text: "标签",
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #x5E6C84
                }
            }

            <View> { width: Fill, height: Fit }
        }

        // 标签列表容器
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,
            
            // 标签列表
            tag_list = <PortalList> {
                width: Fill,
                height: Fit,
                flow: RightWrap,
                spacing: 5,
                padding: {top: 5, bottom: 5},

                TagItem = <TagItem> {}
            }

            // 空状态提示
            empty_label = <Label> {
                width: Fill,
                height: Fit,
                padding: {top: 10, bottom: 10},
                text: "暂无标签",
                visible: false,
                draw_text: {
                    color: #x95A5A6,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }
        }

        // 添加新标签区域
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            // 添加按钮
            add_tag_button = <Button> {
                width: Fit,
                height: 30,
                text: "+ 添加标签",
                draw_bg: {
                    color: #x4ECDC4,
                    radius: 3.0,
                }
                draw_text: {
                    color: #FFFFFF,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }

            // 输入框（默认隐藏）
            add_tag_input_container = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5,
                visible: false,

                new_tag_input = <TextInput> {
                    width: Fill,
                    height: 35,
                    text: "",
                    draw_text: {
                        color: #x172B4D,
                        text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    }
                    draw_bg: {
                        color: #FFFFFF,
                        border_color: #xDFE1E6,
                        border_width: 2.0,
                        radius: 3.0,
                    }
                    draw_cursor: {
                        color: #x172B4D
                    }
                }

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10,

                    save_tag_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "保存",
                        draw_bg: {
                            color: #x4ECDC4,
                            radius: 3.0,
                        }
                        draw_text: {
                            color: #FFFFFF,
                            text_style: <THEME_FONT_REGULAR>{font_size: 12}
                        }
                    }

                    cancel_tag_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "取消",
                        draw_bg: {
                            color: #x95A5A6,
                            radius: 3.0,
                        }
                        draw_text: {
                            color: #FFFFFF,
                            text_style: <THEME_FONT_REGULAR>{font_size: 12}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TagItem {
    #[deref]
    view: View,
    #[rust]
    tag_text: String,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
}

impl Widget for TagItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理删除按钮
            if self.view.button(ids!(remove_btn)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("TagItem: 删除标签 '{}'", self.tag_text);
                    cx.action(crate::kanban::KanbanActions::RemoveTag {
                        card_id: card_id.clone(),
                        tag: self.tag_text.clone(),
                    });
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TagSection {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
    #[rust]
    is_adding: bool,
}

impl Widget for TagSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理添加标签按钮
            if self.view.button(ids!(add_tag_button)).clicked(actions) {
                log!("TagSection: 添加标签按钮被点击");
                self.is_adding = true;
                self.view.view(ids!(add_tag_input_container)).set_visible(cx, true);
                self.view.button(ids!(add_tag_button)).set_visible(cx, false);
                self.view.redraw(cx);
            }
            
            // 处理保存标签按钮
            if self.view.button(ids!(save_tag_button)).clicked(actions) {
                log!("TagSection: 保存标签按钮被点击");
                let text = self.view.text_input(ids!(new_tag_input)).text();
                
                if !text.trim().is_empty() {
                    if let Some(card_id) = &self.card_id {
                        log!("TagSection: 添加标签 '{}' 到卡片 {}", text.trim(), card_id);
                        cx.action(crate::kanban::KanbanActions::AddTag {
                            card_id: card_id.clone(),
                            tag: text.trim().to_string(),
                        });
                    }
                }
                
                // 重置输入框
                self.view.text_input(ids!(new_tag_input)).set_text(cx, "");
                self.is_adding = false;
                self.view.view(ids!(add_tag_input_container)).set_visible(cx, false);
                self.view.button(ids!(add_tag_button)).set_visible(cx, true);
                self.view.redraw(cx);
            }
            
            // 处理取消按钮
            if self.view.button(ids!(cancel_tag_button)).clicked(actions) {
                log!("TagSection: 取消添加标签");
                self.view.text_input(ids!(new_tag_input)).set_text(cx, "");
                self.is_adding = false;
                self.view.view(ids!(add_tag_input_container)).set_visible(cx, false);
                self.view.button(ids!(add_tag_button)).set_visible(cx, true);
                self.view.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 AppState 获取 selected_card_id
        let tags: Vec<String> = if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            if let Some(selected_card_id) = &app_state.kanban_state.selected_card_id {
                self.card_id = Some(selected_card_id.clone());
                
                if let Some(card) = app_state.kanban_state.cards.get(selected_card_id) {
                    log!("🏷️ TagSection draw_walk: card_id={}, tags={:?}", selected_card_id, card.tags);
                    card.tags.clone()
                } else {
                    log!("⚠️ TagSection: Card not found in state!");
                    Vec::new()
                }
            } else {
                log!("⚠️ TagSection: No selected_card_id!");
                Vec::new()
            }
        } else {
            log!("⚠️ TagSection: No AppState in scope!");
            Vec::new()
        };

        log!("🏷️ TagSection: Rendering {} tags", tags.len());

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, tags.len());

                while let Some(tag_idx) = list.next_visible_item(cx) {
                    if tag_idx >= tags.len() {
                        continue;
                    }

                    let tag_item_widget = list.item(cx, tag_idx, live_id!(TagItem));
                    let tag = &tags[tag_idx];
                    
                    log!("🏷️ TagSection: Rendering tag #{}: '{}'", tag_idx, tag);
                    
                    // 设置标签文本
                    tag_item_widget.label(ids!(tag_text)).set_text(cx, tag);
                    
                    // 传递 tag_text 和 card_id 给 TagItem
                    let tag_item_ref = tag_item_widget.as_tag_item();
                    if let Some(mut tag_item) = tag_item_ref.borrow_mut() {
                        tag_item.tag_text = tag.clone();
                        tag_item.card_id = self.card_id.clone();
                    }
                    
                    tag_item_widget.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        
        // 在 draw_walk 之后设置可见性
        if tags.is_empty() {
            log!("🏷️ TagSection: Showing empty_label");
            self.view.label(ids!(empty_label)).set_visible(cx, true);
        } else {
            log!("🏷️ TagSection: Hiding empty_label, showing {} tags", tags.len());
            self.view.label(ids!(empty_label)).set_visible(cx, false);
        }
        
        DrawStep::done()
    }
}
