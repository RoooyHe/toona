use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::shared::styles::*;

    // 卡片基本信息区域（标题、描述、状态）
    pub CardInfoSection = {{CardInfoSection}} {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 15,
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 15,

        // 卡片标题
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {y: 0.5}

                <Label> {
                    width: Fill,
                    height: Fit,
                    text: "标题"
                    draw_text: {
                        color: #666666
                        text_style: {
                            font_size: 14.0
                        }
                    }
                }

                edit_title_button = <Button> {
                    width: 60,
                    height: 25,
                    text: "编辑"
                    draw_bg: {
                        color: #45B7D1
                    }
                    draw_text: {
                        color: #FFFFFF
                        text_style: {
                            font_size: 12.0
                        }
                    }
                }
            }

            // 标题显示区域
            card_title_label = <Label> {
                width: Fill,
                height: Fit,
                text: "卡片标题"
                draw_text: {
                    color: #333333
                    text_style: {
                        font_size: 16.0
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
                    height: 40,
                    text: "",
                }

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10,

                    save_title_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "保存"
                        draw_bg: {
                            color: #45B7D1
                        }
                        draw_text: {
                            color: #FFFFFF
                            text_style: {
                                font_size: 12.0
                            }
                        }
                    }

                    cancel_title_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "取消"
                        draw_bg: {
                            color: #95A5A6
                        }
                        draw_text: {
                            color: #FFFFFF
                            text_style: {
                                font_size: 12.0
                            }
                        }
                    }
                }
            }
        }

        // 卡片描述
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {y: 0.5}

                <Label> {
                    width: Fill,
                    height: Fit,
                    text: "描述"
                    draw_text: {
                        color: #666666
                        text_style: {
                            font_size: 14.0
                        }
                    }
                }

                edit_description_button = <Button> {
                    width: 60,
                    height: 25,
                    text: "编辑"
                    draw_bg: {
                        color: #45B7D1
                    }
                    draw_text: {
                        color: #FFFFFF
                        text_style: {
                            font_size: 12.0
                        }
                    }
                }
            }

            // 描述显示区域
            card_description_label = <Label> {
                width: Fill,
                height: Fit,
                text: "暂无描述"
                draw_text: {
                    color: #333333
                    text_style: {
                        font_size: 14.0
                    }
                }
            }

            // 描述编辑区域
            description_edit_container = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5,
                visible: false,

                card_description_input = <SimpleTextInput> {
                    width: Fill,
                    height: 80,
                    text: "",
                }

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10,

                    save_description_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "保存"
                        draw_bg: {
                            color: #45B7D1
                        }
                        draw_text: {
                            color: #FFFFFF
                            text_style: {
                                font_size: 12.0
                            }
                        }
                    }

                    cancel_description_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "取消"
                        draw_bg: {
                            color: #95A5A6
                        }
                        draw_text: {
                            color: #FFFFFF
                            text_style: {
                                font_size: 12.0
                            }
                        }
                    }
                }
            }
        }

        // 卡片状态
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            <Label> {
                width: Fill,
                height: Fit,
                text: "状态"
                draw_text: {
                    color: #666666
                    text_style: {
                        font_size: 14.0
                    }
                }
            }

            card_status = <Label> {
                width: Fill,
                height: Fit,
                text: "进行中"
                draw_text: {
                    color: #333333
                    text_style: {
                        font_size: 14.0
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardInfoSection {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
    #[rust]
    is_editing_title: bool,
    #[rust]
    is_editing_description: bool,
}

impl Widget for CardInfoSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            log!("CardInfoSection: Received actions, card_id: {:?}", self.card_id);
            
            // 处理标题编辑按钮
            if self.view.button(ids!(edit_title_button)).clicked(actions) {
                log!("CardInfoSection: 点击编辑标题按钮");
                self.is_editing_title = true;
                
                // 获取当前标题并设置到输入框
                let current_title = self.view.label(ids!(card_title_label)).text();
                self.view.text_input(ids!(card_title_input)).set_text(cx, &current_title);
                
                // 显示编辑区域，隐藏显示区域
                self.view.view(ids!(title_edit_container)).set_visible(cx, true);
                self.view.label(ids!(card_title_label)).set_visible(cx, false);
                self.view.button(ids!(edit_title_button)).set_visible(cx, false);
                
                // 设置焦点到输入框
                self.view.text_input(ids!(card_title_input)).set_key_focus(cx);
                
                self.view.redraw(cx);
            }
            
            // 处理标题保存按钮
            if self.view.button(ids!(save_title_button)).clicked(actions) {
                log!("CardInfoSection: 点击保存标题按钮");
                let new_title = self.view.text_input(ids!(card_title_input)).text();
                
                if !new_title.trim().is_empty() {
                    if let Some(card_id) = &self.card_id {
                        log!("CardInfoSection: 保存新标题 '{}' (卡片ID: {})", new_title.trim(), card_id);
                        cx.action(crate::kanban::KanbanActions::UpdateCardTitle {
                            card_id: card_id.clone(),
                            title: new_title.trim().to_string(),
                        });
                        
                        // 更新显示标签
                        self.view.label(ids!(card_title_label)).set_text(cx, new_title.trim());
                    }
                }
                
                // 隐藏编辑区域，显示显示区域
                self.is_editing_title = false;
                self.view.view(ids!(title_edit_container)).set_visible(cx, false);
                self.view.label(ids!(card_title_label)).set_visible(cx, true);
                self.view.button(ids!(edit_title_button)).set_visible(cx, true);
                
                self.view.redraw(cx);
            }
            
            // 处理标题取消按钮
            if self.view.button(ids!(cancel_title_button)).clicked(actions) {
                log!("CardInfoSection: 取消编辑标题");
                self.is_editing_title = false;
                
                // 隐藏编辑区域，显示显示区域
                self.view.view(ids!(title_edit_container)).set_visible(cx, false);
                self.view.label(ids!(card_title_label)).set_visible(cx, true);
                self.view.button(ids!(edit_title_button)).set_visible(cx, true);
                
                self.view.redraw(cx);
            }
            
            // 处理描述编辑按钮
            if self.view.button(ids!(edit_description_button)).clicked(actions) {
                log!("CardInfoSection: 点击编辑描述按钮");
                self.is_editing_description = true;
                
                // 获取当前描述并设置到输入框
                let current_desc = self.view.label(ids!(card_description_label)).text();
                self.view.text_input(ids!(card_description_input)).set_text(cx, &current_desc);
                
                // 显示编辑区域，隐藏显示区域
                self.view.view(ids!(description_edit_container)).set_visible(cx, true);
                self.view.label(ids!(card_description_label)).set_visible(cx, false);
                self.view.button(ids!(edit_description_button)).set_visible(cx, false);
                
                // 设置焦点到输入框
                self.view.text_input(ids!(card_description_input)).set_key_focus(cx);
                
                self.view.redraw(cx);
            }
            
            // 处理描述保存按钮
            if self.view.button(ids!(save_description_button)).clicked(actions) {
                log!("CardInfoSection: 点击保存描述按钮");
                let new_desc = self.view.text_input(ids!(card_description_input)).text();
                
                if let Some(card_id) = &self.card_id {
                    let desc_option = if new_desc.trim().is_empty() {
                        None
                    } else {
                        Some(new_desc.trim().to_string())
                    };
                    
                    log!("CardInfoSection: 保存新描述 '{:?}' (卡片ID: {})", desc_option, card_id);
                    cx.action(crate::kanban::KanbanActions::UpdateCardDescription {
                        card_id: card_id.clone(),
                        description: desc_option.clone(),
                    });
                    
                    // 更新显示标签
                    let display_text = desc_option.unwrap_or_else(|| "暂无描述".to_string());
                    self.view.label(ids!(card_description_label)).set_text(cx, &display_text);
                }
                
                // 隐藏编辑区域，显示显示区域
                self.is_editing_description = false;
                self.view.view(ids!(description_edit_container)).set_visible(cx, false);
                self.view.label(ids!(card_description_label)).set_visible(cx, true);
                self.view.button(ids!(edit_description_button)).set_visible(cx, true);
                
                self.view.redraw(cx);
            }
            
            // 处理描述取消按钮
            if self.view.button(ids!(cancel_description_button)).clicked(actions) {
                log!("CardInfoSection: 取消编辑描述");
                self.is_editing_description = false;
                
                // 隐藏编辑区域，显示显示区域
                self.view.view(ids!(description_edit_container)).set_visible(cx, false);
                self.view.label(ids!(card_description_label)).set_visible(cx, true);
                self.view.button(ids!(edit_description_button)).set_visible(cx, true);
                
                self.view.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 AppState 获取 selected_card_id
        if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            if let Some(card_id) = &app_state.kanban_state.selected_card_id {
                // 只在 card_id 改变时更新
                let should_update = self.card_id.as_ref() != Some(card_id);
                
                if should_update {
                    self.card_id = Some(card_id.clone());
                    log!("CardInfoSection: Got card_id from AppState: {}", card_id);
                    
                    // 从 AppState 获取卡片数据并更新显示
                    if let Some(card) = app_state.kanban_state.cards.get(card_id) {
                        log!("CardInfoSection: Found card data, title: {}", card.title);
                        self.view.label(ids!(card_title_label)).set_text(cx, &card.title);
                        
                        let desc_text = card.description.as_deref().unwrap_or("暂无描述");
                        self.view.label(ids!(card_description_label)).set_text(cx, desc_text);
                    } else {
                        log!("CardInfoSection: Card not found in AppState");
                    }
                }
            } else {
                log!("CardInfoSection: No selected_card_id in AppState");
            }
        } else {
            log!("CardInfoSection: No AppState in scope");
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

impl CardInfoSectionRef {
    pub fn set_card_data(
        &self,
        cx: &mut Cx,
        card_id: matrix_sdk::ruma::OwnedRoomId,
        title: &str,
        description: Option<&str>,
    ) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.card_id = Some(card_id);
            
            // 设置标题
            inner.view.label(ids!(card_title_label)).set_text(cx, title);
            
            // 设置描述
            let desc_text = description.unwrap_or("暂无描述");
            inner.view.label(ids!(card_description_label)).set_text(cx, desc_text);
        }
    }
}
