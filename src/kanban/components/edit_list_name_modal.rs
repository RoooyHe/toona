use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub EditListNameModal = {{EditListNameModal}} {
        width: Fit,
        height: Fit,

        <RoundedView> {
            width: 400,
            height: Fit,
            padding: 20,
            flow: Down,
            spacing: 15,
            draw_bg: {
                color: #FFFFFF
            }

            // 标题
            <Label> {
                width: Fill,
                height: Fit,
                text: "编辑列表名称",
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 18}
                    color: #172B4D
                }
            }

            // 输入框
            list_name_input = <TextInput> {
                width: Fill,
                height: 40,
                text: "",
                draw_text: {
                    color: #172B4D,
                    text_style: <THEME_FONT_REGULAR>{font_size: 14}
                }
                draw_bg: {
                    color: #FFFFFF,
                    border_color: #DFE1E6,
                    border_width: 2.0,
                    radius: 3.0,
                }
                draw_cursor: {
                    color: #172B4D
                }
            }

            // 按钮区域
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {x: 1.0, y: 0.5}

                cancel_button = <Button> {
                    width: 80,
                    height: 36,
                    text: "取消",
                    draw_bg: {
                        color: #95A5A6,
                        radius: 3.0,
                    }
                    draw_text: {
                        color: #FFFFFF,
                        text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    }
                }

                save_button = <Button> {
                    width: 80,
                    height: 36,
                    text: "保存",
                    draw_bg: {
                        color: #0079BF,
                        radius: 3.0,
                    }
                    draw_text: {
                        color: #FFFFFF,
                        text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct EditListNameModal {
    #[deref]
    view: View,
    #[rust]
    list_id: Option<matrix_sdk::ruma::OwnedRoomId>,
}

impl Widget for EditListNameModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理保存按钮
            if self.view.button(ids!(save_button)).clicked(actions) {
                let text = self.view.text_input(ids!(list_name_input)).text();
                
                if !text.trim().is_empty() {
                    if let Some(list_id) = &self.list_id {
                        log!("EditListNameModal: 保存列表名称 '{}' (列表ID: {})", text.trim(), list_id);
                        cx.action(crate::kanban::KanbanActions::UpdateListName {
                            list_id: list_id.clone(),
                            name: text.trim().to_string(),
                        });
                    }
                }
            }
            
            // 处理取消按钮
            if self.view.button(ids!(cancel_button)).clicked(actions) {
                log!("EditListNameModal: 取消编辑");
            }
            
            // 处理回车键
            if let Some((text, _)) = self.view.text_input(ids!(list_name_input)).returned(actions) {
                if !text.trim().is_empty() {
                    if let Some(list_id) = &self.list_id {
                        log!("EditListNameModal: 回车保存列表名称 '{}' (列表ID: {})", text.trim(), list_id);
                        cx.action(crate::kanban::KanbanActions::UpdateListName {
                            list_id: list_id.clone(),
                            name: text.trim().to_string(),
                        });
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl EditListNameModalRef {
    pub fn set_data(&self, cx: &mut Cx, list_id: matrix_sdk::ruma::OwnedRoomId, current_name: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.list_id = Some(list_id);
            inner.view.text_input(ids!(list_name_input)).set_text(cx, current_name);
        }
    }
}
