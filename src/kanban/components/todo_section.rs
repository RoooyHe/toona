use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 单个待办事项
    TodoItem = {{TodoItem}} {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 10,
        align: {y: 0.5},
        padding: {top: 5, bottom: 5, left: 5, right: 5},

        // 复选框（使用Button模拟）
        checkbox = <Button> {
            width: 20,
            height: 20,
            text: "",
            draw_bg: {
                color: #FFFFFF,
                border_color: #DFE1E6,
            }
        }

        // Todo文本
        todo_text = <Label> {
            width: Fill,
            height: Fit,
            text: "待办事项",
            draw_text: {
                color: #172B4D,
                text_style: <THEME_FONT_REGULAR>{font_size: 14}
            }
        }

        // 删除按钮
        delete_btn = <Button> {
            width: 50,
            height: 25,
            text: "删除",
            draw_bg: {
                color: #FF6B6B,
            }
            draw_text: {
                color: #FFFFFF,
                text_style: <THEME_FONT_REGULAR>{font_size: 12}
            }
        }
    }

    // 待办事项管理区域
    pub TodoSection = {{TodoSection}} {
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
                text: "待办事项",
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #5E6C84
                }
            }

            <View> { width: Fill, height: Fit }

            // 进度显示
            progress_label = <Label> {
                text: "0/0",
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    color: #5E6C84
                }
            }
        }

        // Todo列表（使用PortalList）
        todo_list = <PortalList> {
            width: Fill,
            height: 200,  // 给一个固定高度
            flow: Down,
            spacing: 5,

            TodoItem = <TodoItem> {}
        }

        // 添加新Todo区域
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            // 添加按钮
            add_todo_button = <Button> {
                width: Fit,
                height: 30,
                text: "+ 添加待办",
                draw_bg: {
                    color: #0079BF,
                    radius: 3.0,
                }
                draw_text: {
                    color: #FFFFFF,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }

            // 输入框（默认隐藏）
            add_todo_input_container = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5,
                visible: false,

                new_todo_input = <TextInput> {
                    width: Fill,
                    height: 35,
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

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10,

                    save_todo_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "保存",
                        draw_bg: {
                            color: #0079BF,
                            radius: 3.0,
                        }
                        draw_text: {
                            color: #FFFFFF,
                            text_style: <THEME_FONT_REGULAR>{font_size: 12}
                        }
                    }

                    cancel_todo_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "取消",
                        draw_bg: {
                            color: #95A5A6,
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
pub struct TodoItem {
    #[deref]
    view: View,
    #[rust]
    todo_id: String,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
}

impl Widget for TodoItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理复选框点击
            if self.view.button(ids!(checkbox)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("TodoItem: 切换待办 '{}' 的完成状态", self.todo_id);
                    cx.action(crate::kanban::KanbanActions::ToggleTodo {
                        card_id: card_id.clone(),
                        todo_id: self.todo_id.clone(),
                    });
                }
            }
            
            // 处理删除按钮
            if self.view.button(ids!(delete_btn)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("TodoItem: 删除待办 '{}'", self.todo_id);
                    cx.action(crate::kanban::KanbanActions::DeleteTodo {
                        card_id: card_id.clone(),
                        todo_id: self.todo_id.clone(),
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
pub struct TodoSection {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
    #[rust]
    is_adding: bool,
}

impl Widget for TodoSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理添加待办按钮
            if self.view.button(ids!(add_todo_button)).clicked(actions) {
                log!("TodoSection: 添加待办按钮被点击");
                self.is_adding = true;
                self.view.view(ids!(add_todo_input_container)).set_visible(cx, true);
                self.view.button(ids!(add_todo_button)).set_visible(cx, false);
                self.view.redraw(cx);
            }
            
            // 处理保存待办按钮
            if self.view.button(ids!(save_todo_button)).clicked(actions) {
                log!("TodoSection: 保存待办按钮被点击");
                let text = self.view.text_input(ids!(new_todo_input)).text();
                
                if !text.is_empty() {
                    if let Some(card_id) = &self.card_id {
                        log!("TodoSection: 添加待办 '{}' 到卡片 {}", text, card_id);
                        cx.action(crate::kanban::KanbanActions::AddTodo {
                            card_id: card_id.clone(),
                            text: text.to_string(),
                        });
                    }
                }
                
                // 重置输入框
                self.view.text_input(ids!(new_todo_input)).set_text(cx, "");
                self.is_adding = false;
                self.view.view(ids!(add_todo_input_container)).set_visible(cx, false);
                self.view.button(ids!(add_todo_button)).set_visible(cx, true);
                self.view.redraw(cx);
            }
            
            // 处理取消按钮
            if self.view.button(ids!(cancel_todo_button)).clicked(actions) {
                log!("TodoSection: 取消添加待办");
                self.view.text_input(ids!(new_todo_input)).set_text(cx, "");
                self.is_adding = false;
                self.view.view(ids!(add_todo_input_container)).set_visible(cx, false);
                self.view.button(ids!(add_todo_button)).set_visible(cx, true);
                self.view.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 AppState 获取 selected_card_id
        let todos: Vec<_> = if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            if let Some(selected_card_id) = &app_state.kanban_state.selected_card_id {
                self.card_id = Some(selected_card_id.clone());
                
                if let Some(card) = app_state.kanban_state.cards.get(selected_card_id) {
                    // 更新进度显示
                    let (completed, total) = card.todo_progress();
                    let progress_text = format!("{}/{}", completed, total);
                    self.view.label(ids!(progress_label)).set_text(cx, &progress_text);
                    
                    // log!("🎨 TodoSection draw_walk: card_id={}, todos_count={}", selected_card_id, card.todos.len());
                    
                    // 克隆todos列表
                    card.todos.clone()
                } else {
                    log!("⚠️ TodoSection: Card not found in state");
                    Vec::new()
                }
            } else {
                log!("⚠️ TodoSection: No selected_card_id");
                Vec::new()
            }
        } else {
            log!("⚠️ TodoSection: No AppState in scope");
            Vec::new()
        };

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // log!("🎨 TodoSection: Setting PortalList item range to {}", todos.len());
                list.set_item_range(cx, 0, todos.len());

                while let Some(todo_idx) = list.next_visible_item(cx) {
                    if todo_idx >= todos.len() {
                        continue;
                    }

                    // log!("🎨 TodoSection: Rendering todo #{}: '{}'", todo_idx, todos[todo_idx].text);

                    let todo_item_widget = list.item(cx, todo_idx, live_id!(TodoItem));
                    let todo = &todos[todo_idx];
                    
                    // 设置复选框状态（通过改变背景色和文本）
                    let checkbox_btn = todo_item_widget.button(ids!(checkbox));
                    if todo.completed {
                        checkbox_btn.set_text(cx, "✓");
                    } else {
                        checkbox_btn.set_text(cx, "");
                    }
                    
                    // 设置Todo文本
                    let todo_label = todo_item_widget.label(ids!(todo_text));
                    todo_label.set_text(cx, &todo.text);
                    
                    // 传递 todo_id 和 card_id 给 TodoItem
                    let todo_item_ref = todo_item_widget.as_todo_item();
                    if let Some(mut todo_item) = todo_item_ref.borrow_mut() {
                        todo_item.todo_id = todo.id.clone();
                        todo_item.card_id = self.card_id.clone();
                    }
                    
                    todo_item_widget.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }
}
