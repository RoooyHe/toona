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
        padding: {top: 10, bottom: 10, left: 10, right: 10}
        draw_bg: {
            color: #F0F0F0
        }

        // 用户信息行
        <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 6,
            align: {y: 0.5}

            icon_label = <Label> {
                width: Fit,
                height: Fit,
                text: "💬"
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 14}
                    color: #000000
                }
            }

            activity_type_label = <Label> {
                width: Fit,
                height: Fit,
                text: "评论"
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 12}
                    color: #666666
                }
            }

            user_name = <Label> {
                width: Fit,
                height: Fit,
                text: "用户名"
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 12}
                    color: #000000
                }
            }

            time_label = <Label> {
                width: Fit,
                height: Fit,
                text: "5分钟前"
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 11}
                    color: #999999
                }
            }
        }

        comment_text = <Label> {
            width: Fill,
            height: Fit,
            text: "评论内容"
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{font_size: 13}
                color: #333333
                wrap: Word
            }
        }
    }

    // 活动记录区域
    pub ActiveSection = {{ActiveSection}} {
        <View> {
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

            // 活动列表区域
            activities_list = <PortalList> {
                width: Fill,
                height: 300,
                flow: Down,
                spacing: 5,

                ActivityItem = <ActivityItem> {}
            }

            // 评论输入区域（固定在底部）
            comment_area = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 8,

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
}

#[derive(Live, LiveHook, Widget)]
pub struct ActiveSection {
    #[deref]
    view: View,
}

impl Widget for ActiveSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {

        // 先处理事件，再传递给子组件
        if let Event::Actions(actions) = event {

            let app_state = scope.data.get::<crate::app::AppState>();

            if let Some(state) = app_state {

                if let Some(selected_card_id) = &state.kanban_state.selected_card_id {

                    let send_btn = self.view.button(ids!(send_button));
                    let is_clicked = send_btn.clicked(actions);

                    if is_clicked {
                        log!("ActiveSection send_button clicked!");
                        let comment_text = self.view.text_input(ids!(comment_input)).text();
                        log!("ActiveSection: comment_text='{}'", comment_text);

                        if !comment_text.trim().is_empty() {
                            log!(
                                "ActiveSection: sending AddComment with text='{}'",
                                comment_text
                            );
                            cx.action(
                                crate::kanban::state::kanban_actions::KanbanActions::AddComment {
                                    card_id: selected_card_id.clone(),
                                    text: comment_text.clone(),
                                },
                            );

                            self.view.text_input(ids!(comment_input)).set_text(cx, "");
                            cx.redraw_all();
                        } else {
                            log!("ActiveSection: comment_text is empty, not sending");
                        }
                    }
                }
            }
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let activities = if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            if let Some(selected_card_id) = &app_state.kanban_state.selected_card_id {
                let activities = app_state
                    .kanban_state
                    .activities
                    .get(selected_card_id)
                    .cloned()
                    .unwrap_or_default();
                log!(
                    "ActiveSection draw_walk: card_id={}, activities count={}",
                    selected_card_id,
                    activities.len()
                );
                for (idx, activity) in activities.iter().enumerate() {
                    log!(
                        "  [{}] id={}, text='{}'",
                        idx,
                        activity.id,
                        activity.text
                    );
                }
                activities
            } else {
                log!("ActiveSection draw_walk: no selected_card_id");
                Vec::new()
            }
        } else {
            log!("ActiveSection draw_walk: no app_state");
            Vec::new()
        };

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            log!("ActiveSection draw_walk: got item step");
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                log!(
                    "ActiveSection draw_walk: found PortalList, setting item_range to {}",
                    activities.len()
                );
                list.set_item_range(cx, 0, activities.len());

                while let Some(activity_idx) = list.next_visible_item(cx) {
                    log!(
                        "ActiveSection draw_walk: rendering activity at index {}",
                        activity_idx
                    );
                    if activity_idx >= activities.len() {
                        log!("  WARNING: activity_idx {} >= activities.len() {}", activity_idx, activities.len());
                        continue;
                    }

                    let activity_item_widget = list.item(cx, activity_idx, live_id!(ActivityItem));
                    let activity = &activities[activity_idx];

                    // 设置活动类型图标
                    let icon_symbol = activity.activity_type.icon_symbol();
                    let activity_type_text = activity.activity_type.display_text();

                    let icon_label = activity_item_widget.label(ids!(icon_label));
                    icon_label.set_text(cx, icon_symbol);

                    // 设置活动类型标签
                    activity_item_widget
                        .label(ids!(activity_type_label))
                        .set_text(cx, activity_type_text);

                    // 设置用户名
                    activity_item_widget
                        .label(ids!(user_name))
                        .set_text(cx, &activity.user_id);

                    // 格式化时间
                    let time_str = format_relative_time(activity.created_at);
                    activity_item_widget
                        .label(ids!(time_label))
                        .set_text(cx, &time_str);

                    // 设置评论内容
                    activity_item_widget
                        .label(ids!(comment_text))
                        .set_text(cx, &activity.text);

                    activity_item_widget.draw_all(cx, scope);
                    log!("  ✅ Rendered activity: '{}'", activity.text);
                }
            } else {
                log!("ActiveSection draw_walk: item is not a PortalList");
            }
        }
        DrawStep::done()
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
