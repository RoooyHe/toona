use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    
    // 标签管理模态框
    pub TagManagementModal = {{TagManagementModal}} {
        width: Fit,
        height: Fit,

        <RoundedView> {
            width: 400,
            height: Fit,
            max_height: 500,
            padding: 20,
            flow: Down,
            spacing: 15,
            draw_bg: {
                color: #FFFFFF
            }

            // 标题栏
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                align: {y: 0.5},
                
                <Label> {
                    width: Fill,
                    text: "管理标签",
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 18}
                        color: #172B4D
                    }
                }
                
                close_button = <Button> {
                    width: 30,
                    height: 30,
                    text: "×",
                    draw_bg: {
                        color: #00000000
                    }
                    draw_text: {
                        color: #5E6C84,
                        text_style: <THEME_FONT_BOLD>{font_size: 20}
                    }
                }
            }

            // 当前标签库
            <Label> {
                text: "标签库（点击添加到卡片）:",
                draw_text: {
                    color: #5E6C84,
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                }
            }
            
            // 标签列表容器（使用按钮显示可点击的标签）
            tags_list_container = <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 8,
                padding: {top: 5, bottom: 5},
                
                // 标签按钮会在这里动态添加
                // 由于 Makepad 限制，我们使用预定义的按钮槽位
                tag_btn_0 = <Button> {
                    visible: false,
                    height: 30,
                    padding: {left: 12, right: 12},
                    text: "",
                    draw_bg: {
                        color: #0079BF,
                        radius: 3.0,
                    }
                    draw_text: {
                        color: #FFFFFF,
                        text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    }
                }
                
                tag_btn_1 = <Button> {
                    visible: false,
                    height: 30,
                    padding: {left: 12, right: 12},
                    text: "",
                    draw_bg: {
                        color: #0079BF,
                        radius: 3.0,
                    }
                    draw_text: {
                        color: #FFFFFF,
                        text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    }
                }
                
                tag_btn_2 = <Button> {
                    visible: false,
                    height: 30,
                    padding: {left: 12, right: 12},
                    text: "",
                    draw_bg: {
                        color: #0079BF,
                        radius: 3.0,
                    }
                    draw_text: {
                        color: #FFFFFF,
                        text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    }
                }
                
                tag_btn_3 = <Button> {
                    visible: false,
                    height: 30,
                    padding: {left: 12, right: 12},
                    text: "",
                    draw_bg: {
                        color: #0079BF,
                        radius: 3.0,
                    }
                    draw_text: {
                        color: #FFFFFF,
                        text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    }
                }
                
                tag_btn_4 = <Button> {
                    visible: false,
                    height: 30,
                    padding: {left: 12, right: 12},
                    text: "",
                    draw_bg: {
                        color: #0079BF,
                        radius: 3.0,
                    }
                    draw_text: {
                        color: #FFFFFF,
                        text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    }
                }
            }
            
            // 空状态提示
            empty_tags_label = <Label> {
                visible: true,
                text: "暂无标签，请先创建标签",
                draw_text: {
                    color: #95A5A6,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }

            // 分隔线
            <View> {
                width: Fill,
                height: 1,
                draw_bg: {
                    color: #DFE1E6
                }
            }

            // 创建新标签区域
            <Label> {
                text: "创建新标签:",
                draw_text: {
                    color: #5E6C84,
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                }
            }
            
            // 标签名称输入
            <Label> {
                text: "标签名称:",
                draw_text: {
                    color: #5E6C84,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }
            
            tag_name_input = <TextInput> {
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

            // 颜色选择
            <Label> {
                text: "选择颜色:",
                draw_text: {
                    color: #5E6C84,
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                }
            }
            
            // 颜色按钮网格
            color_grid = <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 8,
                
                color_btn_0 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #EB5A46,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_1 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #FF9F1A,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_2 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #F2D600,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_3 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #61BD4F,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_4 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #00C2E0,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
            }
            
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 8,
                
                color_btn_5 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #0079BF,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_6 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #9775FA,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_7 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #FF78CB,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_8 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #95A5A6,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
                
                color_btn_9 = <Button> {
                    width: 40,
                    height: 40,
                    text: "",
                    draw_bg: {
                        color: #343434,
                        radius: 4.0,
                        border_width: 0.0,
                    }
                }
            }

            // 选中颜色预览
            selected_color_preview = <View> {
                width: Fill,
                height: 40,
                margin: {top: 10},
                draw_bg: {
                    color: #0079BF,
                    radius: 3.0,
                }
            }

            // 按钮区域
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {x: 1.0, y: 0.5}

                create_button = <Button> {
                    width: 100,
                    height: 36,
                    text: "创建标签",
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
            
            // 分隔线
            <View> {
                width: Fill,
                height: 1,
                margin: {top: 10, bottom: 10},
                draw_bg: {
                    color: #DFE1E6
                }
            }
            
            // 添加标签到卡片区域
            <Label> {
                text: "添加标签到卡片:",
                draw_text: {
                    color: #5E6C84,
                    text_style: <THEME_FONT_BOLD>{font_size: 14}
                }
            }
            
            add_tag_input = <TextInput> {
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
            
            <Label> {
                text: "提示: 输入标签名称，按回车添加",
                draw_text: {
                    color: #95A5A6,
                    text_style: <THEME_FONT_REGULAR>{font_size: 11}
                }
            }
        }
    }
}

// 预定义颜色
const PREDEFINED_COLORS: &[(&str, &str)] = &[
    ("红色", "#EB5A46"),
    ("橙色", "#FF9F1A"),
    ("黄色", "#F2D600"),
    ("绿色", "#61BD4F"),
    ("青色", "#00C2E0"),
    ("蓝色", "#0079BF"),
    ("紫色", "#9775FA"),
    ("粉色", "#FF78CB"),
    ("灰色", "#95A5A6"),
    ("黑色", "#343434"),
];

#[derive(Live, LiveHook, Widget)]
pub struct TagManagementModal {
    #[deref]
    view: View,
    #[rust]
    space_id: Option<matrix_sdk::ruma::OwnedRoomId>,
    #[rust]
    card_id: Option<matrix_sdk::ruma::OwnedRoomId>,
    #[rust]
    selected_color_index: usize,
    #[rust]
    tags_list_initialized: bool,
    #[rust]
    tag_buttons: Vec<(String, String)>, // (tag_id, tag_name)
}

impl Widget for TagManagementModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        if let Event::Actions(actions) = event {
            // 处理关闭按钮
            if self.view.button(ids!(close_button)).clicked(actions) {
                log!("TagManagementModal: 关闭模态框");
                cx.action(crate::kanban::KanbanActions::CloseTagManagementModal);
            }
            
            // 处理标签按钮点击（添加标签到卡片）
            for i in 0..5 {
                let button_id = match i {
                    0 => ids!(tag_btn_0),
                    1 => ids!(tag_btn_1),
                    2 => ids!(tag_btn_2),
                    3 => ids!(tag_btn_3),
                    4 => ids!(tag_btn_4),
                    _ => continue,
                };
                
                if self.view.button(button_id).clicked(actions) {
                    if i < self.tag_buttons.len() {
                        let (tag_id, tag_name) = &self.tag_buttons[i];
                        log!("TagManagementModal: 点击标签按钮 '{}'", tag_name);
                        
                        if let Some(card_id) = &self.card_id {
                            cx.action(crate::kanban::KanbanActions::AddTagToCard {
                                card_id: card_id.clone(),
                                tag_id: tag_id.clone(),
                            });
                        }
                    }
                }
            }
            
            // 处理颜色选择按钮
            for i in 0..10 {
                let button_id = match i {
                    0 => ids!(color_btn_0),
                    1 => ids!(color_btn_1),
                    2 => ids!(color_btn_2),
                    3 => ids!(color_btn_3),
                    4 => ids!(color_btn_4),
                    5 => ids!(color_btn_5),
                    6 => ids!(color_btn_6),
                    7 => ids!(color_btn_7),
                    8 => ids!(color_btn_8),
                    9 => ids!(color_btn_9),
                    _ => continue,
                };
                if self.view.button(button_id).clicked(actions) {
                    log!("TagManagementModal: 选择颜色 {}", i);
                    self.selected_color_index = i;
                    self.update_color_preview(cx);
                }
            }
            
            // 处理创建标签按钮
            if self.view.button(ids!(create_button)).clicked(actions) {
                log!("TagManagementModal: 创建按钮被点击");
                let tag_name = self.view.text_input(ids!(tag_name_input)).text();
                log!("TagManagementModal: 标签名称 = '{}'", tag_name);
                log!("TagManagementModal: space_id = {:?}", self.space_id);
                
                if !tag_name.trim().is_empty() {
                    if let Some(space_id) = &self.space_id {
                        let color = PREDEFINED_COLORS[self.selected_color_index].1.to_string();
                        log!("TagManagementModal: 创建标签 '{}' 颜色 {}", tag_name.trim(), color);
                        
                        cx.action(crate::kanban::KanbanActions::CreateSpaceTag {
                            space_id: space_id.clone(),
                            name: tag_name.trim().to_string(),
                            color,
                        });
                        
                        // 不要立即清空输入框，等下一帧再清空
                        // 这样可以避免在事件处理过程中清空导致的问题
                    } else {
                        log!("TagManagementModal: space_id 为 None，无法创建标签");
                    }
                } else {
                    log!("TagManagementModal: 标签名称为空，不创建");
                }
            }
            
            // 处理添加标签输入框的回车键
            if let Some((text, _)) = self.view.text_input(ids!(add_tag_input)).returned(actions) {
                if !text.trim().is_empty() {
                    if let (Some(card_id), Some(space_id)) = (&self.card_id, &self.space_id) {
                        log!("TagManagementModal: 添加标签 '{}' 到卡片", text.trim());
                        
                        // 查找标签 ID
                        // 这里需要从 AppState 获取标签库
                        cx.action(crate::kanban::KanbanActions::AddTagToCardByName {
                            card_id: card_id.clone(),
                            space_id: space_id.clone(),
                            tag_name: text.trim().to_string(),
                        });
                        
                        // 清空输入框
                        self.view.text_input(ids!(add_tag_input)).set_text(cx, "");
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 AppState 获取 space_id 和 card_id（只在第一次设置）
        if self.space_id.is_none() || self.card_id.is_none() {
            if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                if let Some(selected_card_id) = &app_state.kanban_state.selected_card_id {
                    if let Some(card) = app_state.kanban_state.cards.get(selected_card_id) {
                        self.space_id = Some(card.space_id.clone());
                        self.card_id = Some(selected_card_id.clone());
                        self.tags_list_initialized = false; // 重置标志
                        log!("TagManagementModal: 从 AppState 获取数据 space_id='{}', card_id='{}'", 
                            card.space_id, selected_card_id);
                    }
                }
            }
        }
        
        // 更新标签按钮显示（只在第一次或数据变化时）
        if !self.tags_list_initialized {
            if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                if let Some(space_id) = &self.space_id {
                    if let Some(tags) = app_state.kanban_state.space_tags.get(space_id) {
                        // 清空旧的标签按钮数据
                        self.tag_buttons.clear();
                        
                        // 隐藏所有按钮
                        for i in 0..5 {
                            let button_id = match i {
                                0 => ids!(tag_btn_0),
                                1 => ids!(tag_btn_1),
                                2 => ids!(tag_btn_2),
                                3 => ids!(tag_btn_3),
                                4 => ids!(tag_btn_4),
                                _ => continue,
                            };
                            self.view.button(button_id).set_visible(cx, false);
                        }
                        
                        if tags.is_empty() {
                            // 显示空状态
                            self.view.label(ids!(empty_tags_label)).set_visible(cx, true);
                        } else {
                            // 隐藏空状态
                            self.view.label(ids!(empty_tags_label)).set_visible(cx, false);
                            
                            // 显示标签按钮（最多5个）
                            for (i, tag) in tags.iter().take(5).enumerate() {
                                let button_id = match i {
                                    0 => ids!(tag_btn_0),
                                    1 => ids!(tag_btn_1),
                                    2 => ids!(tag_btn_2),
                                    3 => ids!(tag_btn_3),
                                    4 => ids!(tag_btn_4),
                                    _ => continue,
                                };
                                
                                // 设置按钮文本和颜色
                                self.view.button(button_id).set_text(cx, &tag.name);
                                if let Ok(color) = parse_hex_color(&tag.color) {
                                    self.view.button(button_id).apply_over(cx, live! {
                                        draw_bg: { color: (color) }
                                    });
                                }
                                self.view.button(button_id).set_visible(cx, true);
                                
                                // 保存标签信息
                                self.tag_buttons.push((tag.id.clone(), tag.name.clone()));
                            }
                        }
                        
                        self.tags_list_initialized = true;
                    }
                }
            }
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

impl TagManagementModal {
    fn update_color_preview(&mut self, cx: &mut Cx) {
        let color_hex = PREDEFINED_COLORS[self.selected_color_index].1;
        if let Ok(color) = parse_hex_color(color_hex) {
            self.view.view(ids!(selected_color_preview)).apply_over(cx, live! {
                draw_bg: { color: (color) }
            });
        }
        self.view.redraw(cx);
    }
    
    /// 重置模态框状态
    pub fn reset(&mut self) {
        self.space_id = None;
        self.card_id = None;
        self.selected_color_index = 5;
        self.tags_list_initialized = false;
        self.tag_buttons.clear();
    }
}

impl TagManagementModalRef {
    pub fn set_data(&self, cx: &mut Cx, space_id: matrix_sdk::ruma::OwnedRoomId, card_id: matrix_sdk::ruma::OwnedRoomId) {
        log!("TagManagementModalRef::set_data: space_id='{}', card_id='{}'", space_id, card_id);
        if let Some(mut inner) = self.borrow_mut() {
            inner.space_id = Some(space_id.clone());
            inner.card_id = Some(card_id.clone());
            inner.selected_color_index = 5; // 默认蓝色
            inner.update_color_preview(cx);
            log!("TagManagementModalRef::set_data: 数据设置成功");
        } else {
            log!("TagManagementModalRef::set_data: 无法获取 inner，borrow_mut 失败");
        }
    }
    
    pub fn reset(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.reset();
        }
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
