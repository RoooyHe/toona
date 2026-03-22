use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    // 单个标签卡片（TagChip）
    TagChip = {{TagChip}} {
        width: Fit,
        height: Fit,
        flow: Right,
        spacing: 5,
        align: {y: 0.5},
        padding: {top: 6, bottom: 6, left: 12, right: 12},
        margin: {right: 8, bottom: 8},
        draw_bg: {
            color: #0079BF,  // 默认蓝色背景
            radius: 3.0,
        }

        // 标签文本
        tag_text = <Label> {
            width: Fit,
            height: Fit,
            text: "标签",
            draw_text: {
                color: #FFFFFF,  // 白色文字
                text_style: <THEME_FONT_REGULAR>{font_size: 13}
            }
        }

        // 删除按钮
        remove_btn = <Button> {
            width: 20,
            height: 20,
            margin: {left: 4},
            text: "×",
            draw_bg: {
                color: #00000000  // 透明背景
            }
            draw_text: {
                color: #FFFFFF,  // 白色 X
                text_style: <THEME_FONT_BOLD>{font_size: 18}
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
                    color: #5E6C84
                }
            }

            <View> { width: Fill, height: Fit }
            
            // 下拉按钮
            dropdown_button = <Button> {
                width: 30,
                height: 30,
                text: "▼",
                draw_bg: {
                    color: #EBECF0,
                    radius: 3.0,
                }
                draw_text: {
                    color: #172B4D,
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                }
            }
        }

        // 已选标签显示区域
        tags_container = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 0,
            
            // 标签卡片会动态生成
        }
        
        // 空状态提示
        empty_label = <Label> {
            width: Fill,
            height: Fit,
            padding: {top: 10, bottom: 10},
            text: "暂无标签，点击 ▼ 添加标签",
            visible: false,
            draw_text: {
                color: #95A5A6,
                text_style: <THEME_FONT_REGULAR>{font_size: 13}
            }
        }
    }
}

// TagChip 组件
#[derive(Live, LiveHook, Widget)]
pub struct TagChip {
    #[deref]
    view: View,
    #[rust]
    tag_id: String,
    #[rust]
    tag_name: String,
    #[rust]
    tag_color: String,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
}

impl Widget for TagChip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理删除按钮
            if self.view.button(ids!(remove_btn)).clicked(actions) {
                if let Some(card_id) = &self.card_id {
                    log!("TagChip: 删除标签 '{}' (ID: {})", self.tag_name, self.tag_id);
                    cx.action(crate::kanban::KanbanActions::RemoveTagFromCard {
                        card_id: card_id.clone(),
                        tag_id: self.tag_id.clone(),
                    });
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 设置标签文本
        self.view.label(ids!(tag_text)).set_text(cx, &self.tag_name);
        
        // 设置背景颜色
        if let Ok(color) = parse_hex_color(&self.tag_color) {
            self.view.apply_over(cx, live! {
                draw_bg: { color: (color) }
            });
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

impl TagChipRef {
    pub fn set_data(&self, cx: &mut Cx, tag_id: String, tag_name: String, tag_color: String, card_id: matrix_sdk::ruma::OwnedRoomId) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.tag_id = tag_id;
            inner.tag_name = tag_name;
            inner.tag_color = tag_color;
            inner.card_id = Some(card_id);
            inner.view.redraw(cx);
        }
    }
}

// TagSection 组件
#[derive(Live, LiveHook, Widget)]
pub struct TagSection {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
    #[rust]
    space_id: Option<matrix_sdk::ruma::OwnedRoomId>,
}

impl Widget for TagSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理下拉按钮
            if self.view.button(ids!(dropdown_button)).clicked(actions) {
                log!("TagSection: 下拉按钮被点击");
                if let (Some(space_id), Some(card_id)) = (&self.space_id, &self.card_id) {
                    cx.action(crate::kanban::KanbanActions::ShowTagManagementModal {
                        space_id: space_id.clone(),
                        card_id: card_id.clone(),
                    });
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 AppState 获取标签信息
        let tag_display_text = if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            if let Some(selected_card_id) = &app_state.kanban_state.selected_card_id {
                self.card_id = Some(selected_card_id.clone());
                
                if let Some(card) = app_state.kanban_state.cards.get(selected_card_id) {
                    self.space_id = Some(card.space_id.clone());
                    
                    // 获取 Space 标签库
                    let space_tags = app_state.kanban_state.space_tags
                        .get(&card.space_id)
                        .cloned()
                        .unwrap_or_default();
                    
                    log!("TagSection: Found card with {} tags, space has {} tags", 
                        card.tags.len(), space_tags.len());
                    
                    // 根据标签 ID 查找标签详情并生成显示文本
                    let tag_names: Vec<String> = card.tags.iter()
                        .filter_map(|tag_id| {
                            space_tags.iter()
                                .find(|t| &t.id == tag_id)
                                .map(|t| t.name.clone())
                        })
                        .collect();
                    
                    if tag_names.is_empty() {
                        None
                    } else {
                        Some(format!("标签: {}", tag_names.join(", ")))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        // 设置显示内容
        if let Some(text) = tag_display_text {
            // 临时使用简单的文本显示，后续会改为 TagChip
            self.view.label(ids!(empty_label)).set_text(cx, &text);
            self.view.label(ids!(empty_label)).set_visible(cx, true);
        } else {
            self.view.label(ids!(empty_label)).set_text(cx, "暂无标签，点击 ▼ 添加标签");
            self.view.label(ids!(empty_label)).set_visible(cx, true);
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

// 辅助函数：解析十六进制颜色
fn parse_hex_color(hex: &str) -> Result<Vec4, String> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Err(format!("Invalid hex color: {}", hex));
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
    
    Ok(vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0))
}
