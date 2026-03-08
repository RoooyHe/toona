use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 单个标签
    TagItem = {{TagItem}} {
        width: Fit,
        height: Fit,
        flow: Right,
        spacing: 5,
        align: {y: 0.5},
        padding: {top: 8, bottom: 8, left: 16, right: 16},
        margin: {right: 8, bottom: 8},
        draw_bg: {
            color: #0079BF,  // 蓝色背景
            radius: 12.0,
        }

        // 标签文本
        tag_text = <Label> {
            width: Fit,
            height: Fit,
            text: "标签",
            draw_text: {
                color: #FFFFFF,  // 白色文字
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
            }
        }

        // 删除按钮
        remove_btn = <Button> {
            width: 22,
            height: 22,
            margin: {left: 6},
            text: "×",
            draw_bg: {
                color: #00000000,  // 透明背景
                radius: 11.0,
            }
            draw_text: {
                color: #FFFFFF,  // 白色 X
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
        tags_container = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,
            
            // 标签显示区域（临时使用 Label 显示）
            tags_display_label = <Label> {
                width: Fill,
                height: Fit,
                text: "",
                visible: false,
                draw_text: {
                    color: #0079BF,
                    text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    wrap: Word
                }
            }

            // 空状态提示
            empty_label = <Label> {
                width: Fill,
                height: Fit,
                padding: {top: 10, bottom: 10},
                text: "暂无标签",
                visible: true,
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
        // 从 AppState 获取 selected_card_id 和 tags
        let (tags, _card_id) = if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            if let Some(selected_card_id) = &app_state.kanban_state.selected_card_id {
                self.card_id = Some(selected_card_id.clone());
                
                if let Some(card) = app_state.kanban_state.cards.get(selected_card_id) {
                    (card.tags.clone(), Some(selected_card_id.clone()))
                } else {
                    (Vec::new(), None)
                }
            } else {
                (Vec::new(), None)
            }
        } else {
            (Vec::new(), None)
        };
        
        // 设置可见性和内容
        if tags.is_empty() {
            self.view.label(ids!(tags_display_label)).set_visible(cx, false);
            self.view.label(ids!(empty_label)).set_visible(cx, true);
        } else {
            // 显示标签（临时用逗号分隔的文本）
            let tags_text = format!("标签: {}", tags.join(", "));
            self.view.label(ids!(tags_display_label)).set_text(cx, &tags_text);
            self.view.label(ids!(tags_display_label)).set_visible(cx, true);
            self.view.label(ids!(empty_label)).set_visible(cx, false);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}
