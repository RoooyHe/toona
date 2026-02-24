use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 活动记录项
    pub ActivityItem = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 5,
        padding: {top: 10, bottom: 10, left: 0, right: 0}

        // 用户信息行
        <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 10,
            align: {y: 0.5}

            user_name = <Label> {
                width: Fit,
                height: Fit,
                text: "用户名"
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 13}
                    color: #2C3E50
                }
            }

            time_label = <Label> {
                width: Fit,
                height: Fit,
                text: "5分钟前"
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 11}
                    color: #95A5A6
                }
            }
        }

        // 评论内容
        comment_text = <Label> {
            width: Fill,
            height: Fit,
            text: "评论内容"
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 13}
                color: (#x34495E)
                wrap: Word
            }
        }
    }

    // 活动记录区域
    pub ActiveSection = <View> {
        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 10,

        // 标题
        <Label> {
            width: Fill,
            height: Fit,
            text: "活动记录"
            draw_text: {
                text_style: <THEME_FONT_BOLD>{font_size: 14}
                color: #5E6C84
            }
        }

        // 活动列表（滚动区域）
        <ScrollYView> {
            width: Fill,
            height: Fill,
            scroll_bars: <ScrollBars> {
                show_scroll_y: true
            }

            activities_list = <PortalList> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 0,

                ActivityItem = <ActivityItem> {}
            }
        }

        // 评论输入区域
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 8,
            padding: {top: 10}

            <Label> {
                width: Fill,
                height: Fit,
                text: "添加评论"
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 12}
                    color: #5E6C84
                }
            }

            comment_input = <TextInput> {
                width: Fill,
                height: 80,
                text: ""
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    color: #2C3E50
                    wrap: Word
                }
                draw_bg: {
                    color: #F8F9FA
                }
                draw_cursor: {
                    color: #2C3E50
                }
            }

            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {x: 1.0, y: 0.5}

                send_button = <Button> {
                    width: 80,
                    height: 32,
                    text: "发送"
                    draw_bg: {
                        color: #45B7D1
                    }
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 13}
                        color: #FFFFFF
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ActiveSection {
    #[deref]
    view: View,
}

impl Widget for ActiveSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let activities = if let Some(state) = scope.data.get::<crate::kanban::state::kanban_state::KanbanAppState>() {
            if let Some(selected_card_id) = &state.selected_card_id {
                state.activities.get(selected_card_id).cloned().unwrap_or_default()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, activities.len());

                while let Some(activity_idx) = list.next_visible_item(cx) {
                    if activity_idx >= activities.len() {
                        continue;
                    }

                    let activity_item_widget = list.item(cx, activity_idx, live_id!(ActivityItem));
                    let activity = &activities[activity_idx];
                    
                    // 设置用户名
                    activity_item_widget.label(ids!(user_name)).set_text(cx, &activity.user_id);
                    
                    // 格式化时间
                    let time_str = format_relative_time(activity.created_at);
                    activity_item_widget.label(ids!(time_label)).set_text(cx, &time_str);
                    
                    // 设置评论内容
                    activity_item_widget.label(ids!(comment_text)).set_text(cx, &activity.text);
                    
                    activity_item_widget.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for ActiveSection {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let kanban_state = scope.data.get::<crate::kanban::state::kanban_state::KanbanAppState>();
        
        if let Some(state) = kanban_state {
            if let Some(selected_card_id) = &state.selected_card_id {
                // 处理发送按钮点击
                if self.button(ids!(send_button)).clicked(actions) {
                    let comment_text = self.text_input(ids!(comment_input)).text();
                    
                    if !comment_text.trim().is_empty() {
                        // 发送评论
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            crate::kanban::state::kanban_actions::KanbanActions::AddComment {
                                card_id: selected_card_id.clone(),
                                text: comment_text.clone(),
                            },
                        );
                        
                        // 清空输入框
                        self.text_input(ids!(comment_input)).set_text(cx, "");
                        self.redraw(cx);
                    }
                }
            }
        }
    }
}

/// 格式化相对时间
fn format_relative_time(timestamp: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if now < timestamp {
        return "刚刚".to_string();
    }
    
    let diff = now - timestamp;
    
    if diff < 60 {
        format!("{}秒前", diff)
    } else if diff < 3600 {
        format!("{}分钟前", diff / 60)
    } else if diff < 86400 {
        format!("{}小时前", diff / 3600)
    } else if diff < 2592000 {
        format!("{}天前", diff / 86400)
    } else {
        format!("{}个月前", diff / 2592000)
    }
}
