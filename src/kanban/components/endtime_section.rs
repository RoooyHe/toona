use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 截止时间管理区域
    pub EndTimeSection = {{EndTimeSection}} {
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
                text: "截止时间",
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                    color: #5E6C84
                }
            }

            <View> { width: Fill, height: Fit }
        }

        // 时间显示区域
        time_display = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 10,
            align: {y: 0.5},

            time_label = <Label> {
                width: Fill,
                height: Fit,
                text: "未设置截止时间",
                draw_text: {
                    color: #95A5A6,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }

            clear_button = <Button> {
                width: 60,
                height: 25,
                text: "清除",
                visible: false,
                draw_bg: {
                    color: #FF6B6B,
                    radius: 3.0,
                }
                draw_text: {
                    color: #FFFFFF,
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                }
            }
        }

        // 设置时间区域
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,

            // 设置按钮
            set_time_button = <Button> {
                width: Fit,
                height: 30,
                text: "⏰ 设置截止时间",
                draw_bg: {
                    color: #FF9F43,
                    radius: 3.0,
                }
                draw_text: {
                    color: #FFFFFF,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }

            // 输入框（默认隐藏）
            set_time_input_container = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5,
                visible: false,

                <Label> {
                    text: "输入日期时间（格式: YYYY-MM-DD HH:MM）",
                    draw_text: {
                        color: #5E6C84,
                        text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    }
                }

                time_input = <TextInput> {
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

                    save_time_button = <Button> {
                        width: 60,
                        height: 30,
                        text: "保存",
                        draw_bg: {
                            color: #FF9F43,
                            radius: 3.0,
                        }
                        draw_text: {
                            color: #FFFFFF,
                            text_style: <THEME_FONT_REGULAR>{font_size: 12}
                        }
                    }

                    cancel_time_button = <Button> {
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
pub struct EndTimeSection {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
    #[rust]
    is_setting: bool,
}

impl Widget for EndTimeSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理设置时间按钮
            if self.view.button(ids!(set_time_button)).clicked(actions) {
                log!("EndTimeSection: 设置时间按钮被点击");
                self.is_setting = true;
                self.view.view(ids!(set_time_input_container)).set_visible(cx, true);
                self.view.button(ids!(set_time_button)).set_visible(cx, false);
                
                // 预填充当前时间
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let datetime_str = format_timestamp(now + 86400); // 默认明天
                self.view.text_input(ids!(time_input)).set_text(cx, &datetime_str);
                
                self.view.redraw(cx);
            }
            
            // 处理保存时间按钮
            if self.view.button(ids!(save_time_button)).clicked(actions) {
                log!("EndTimeSection: 保存时间按钮被点击");
                let text = self.view.text_input(ids!(time_input)).text();
                
                if let Some(timestamp) = parse_datetime(&text) {
                    if let Some(card_id) = &self.card_id {
                        log!("EndTimeSection: 设置截止时间 {} 到卡片 {}", timestamp, card_id);
                        cx.action(crate::kanban::KanbanActions::SetEndTime {
                            card_id: card_id.clone(),
                            end_time: timestamp,
                        });
                    }
                } else {
                    log!("EndTimeSection: 无效的日期时间格式");
                    // TODO: 显示错误提示
                }
                
                // 重置输入框
                self.view.text_input(ids!(time_input)).set_text(cx, "");
                self.is_setting = false;
                self.view.view(ids!(set_time_input_container)).set_visible(cx, false);
                self.view.button(ids!(set_time_button)).set_visible(cx, true);
                self.view.redraw(cx);
            }
            
            // 处理取消按钮
            if self.view.button(ids!(cancel_time_button)).clicked(actions) {
                log!("EndTimeSection: 取消设置时间");
                self.view.text_input(ids!(time_input)).set_text(cx, "");
                self.is_setting = false;
                self.view.view(ids!(set_time_input_container)).set_visible(cx, false);
                self.view.button(ids!(set_time_button)).set_visible(cx, true);
                self.view.redraw(cx);
            }
            
            // 处理清除按钮
            if self.view.button(ids!(clear_button)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("EndTimeSection: 清除截止时间");
                    cx.action(crate::kanban::KanbanActions::ClearEndTime {
                        card_id: card_id.clone(),
                    });
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 AppState 获取 selected_card_id
        if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            if let Some(selected_card_id) = &app_state.kanban_state.selected_card_id {
                self.card_id = Some(selected_card_id.clone());
                
                // 获取卡片数据
                if let Some(card) = app_state.kanban_state.cards.get(selected_card_id) {
                    // 更新时间显示
                    if let Some(end_time) = card.end_time {
                        let datetime_str = format_timestamp(end_time);
                        let is_overdue = card.is_overdue();
                        
                        let display_text = if is_overdue {
                            format!("⚠️ 已过期: {}", datetime_str)
                        } else {
                            format!("📅 {}", datetime_str)
                        };
                        
                        self.view.label(ids!(time_label)).set_text(cx, &display_text);
                        
                        // 显示清除按钮
                        self.view.button(ids!(clear_button)).set_visible(cx, true);
                    } else {
                        self.view.label(ids!(time_label)).set_text(cx, "未设置截止时间");
                        self.view.button(ids!(clear_button)).set_visible(cx, false);
                    }
                }
            }
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

/// 将Unix timestamp转换为可读的日期时间字符串
fn format_timestamp(timestamp: u64) -> String {
    use std::time::{UNIX_EPOCH, Duration};
    
    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime_secs = datetime.duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // 简化的日期时间格式化（实际应用中应使用chrono库）
    let days_since_epoch = datetime_secs / 86400;
    let seconds_today = datetime_secs % 86400;
    let hours = seconds_today / 3600;
    let minutes = (seconds_today % 3600) / 60;
    
    // 简单计算年月日（不考虑闰年等复杂情况）
    let year = 1970 + (days_since_epoch / 365);
    let day_of_year = days_since_epoch % 365;
    let month = (day_of_year / 30) + 1;
    let day = (day_of_year % 30) + 1;
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}", year, month, day, hours, minutes)
}

/// 解析日期时间字符串为Unix timestamp
/// 格式: YYYY-MM-DD HH:MM
fn parse_datetime(s: &str) -> Option<u64> {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    
    if date_parts.len() != 3 || time_parts.len() != 2 {
        return None;
    }
    
    let year: i64 = date_parts[0].parse().ok()?;
    let month: i64 = date_parts[1].parse().ok()?;
    let day: i64 = date_parts[2].parse().ok()?;
    let hour: i64 = time_parts[0].parse().ok()?;
    let minute: i64 = time_parts[1].parse().ok()?;
    
    // 简化的时间戳计算（不考虑闰年、时区等）
    let days_since_epoch = (year - 1970) * 365 + (month - 1) * 30 + (day - 1);
    let seconds = days_since_epoch * 86400 + hour * 3600 + minute * 60;
    
    if seconds < 0 {
        None
    } else {
        Some(seconds as u64)
    }
}
