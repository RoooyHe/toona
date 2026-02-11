use makepad_widgets::*;
use crate::kanban::models::State;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub CardItem = {{CardItem}} {
        <RoundedView> {
            width: Fill,
            height: 100,
            margin: {bottom: 10},
            padding: 15,
            flow: Down,
            spacing: 8,
            draw_bg: {
                color: #FFFFFFFF
            }

            <RoundedView> {
                width: Fill,
                height: Fit,
                flow: Right,
                align: {y: 0.5},

                card_title_input = <TextInput> {
                    width: Fill,
                    height: 35,
                    text: "卡片标题",
                    draw_text: {
                        color: #333333FF,
                        text_style: {
                            font_size: 16.0,
                        }
                    }
                    draw_bg: {
                        color: #F8F9FAFF
                    }
                    draw_cursor: {
                        color: #333333FF
                    }
                }

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 5,

                    detail_btn = <Button> {
                        width: 50,
                        height: 30,
                        text: "详情",
                        draw_bg: {
                            color: #x4ECDC4
                        }
                        draw_text: {
                            color: #FFFFFF
                        }
                    }

                    delete_card_btn = <Button> {
                        width: 50,
                        height: 30,
                        text: "删除",
                        draw_bg: {
                            color: #FF6B6BFF
                        }
                        draw_text: {
                            color: #FFFFFFFF
                        }
                    }
                }
            }

            card_tags = <Label> {
                width: Fill,
                height: 20,
                text: "标签",
                draw_text: {
                    color: #666666FF,
                    text_style: {
                        font_size: 12.0,
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardItem {
    #[deref]
    view: View,
    #[rust]
    card_id: Option<i64>,
}

impl Widget for CardItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理鼠标点击事件，用于调试焦点问题
        if let Event::MouseDown(_) = event {
            println!("CardItem: 检测到鼠标点击事件");
        }
        
        // 处理卡片标题输入框事件
        if let Event::Actions(actions) = event {
            // 处理卡片标题输入框文本变化
            if let Some(text) = self.view.text_input(id!(card_title_input)).changed(actions) {
                if let Some(card_id) = self.card_id {
                    println!("CardItem: 卡片标题输入变化: '{}' (卡片ID: {})", text, card_id);
                    // 这里可以实时更新状态，但通常我们在失去焦点或回车时才保存
                }
            }
            
            // 处理卡片标题输入框回车
            if let Some((text, _)) = self.view.text_input(id!(card_title_input)).returned(actions) {
                if let Some(card_id) = self.card_id {
                    let state = scope.data.get_mut::<State>().unwrap();
                    
                    // 查找当前卡片的原始标题
                    let current_title = state.card_original_texts.get(&card_id).cloned().unwrap_or_default();
                    
                    if text.trim() != current_title && !text.trim().is_empty() {
                        println!("CardItem: 回车更新卡片标题: '{}' -> '{}' (卡片ID: {})", current_title, text.trim(), card_id);
                        // 设置待更新的卡片标题
                        state.pending_card_update = Some((card_id, text.trim().to_string()));
                    }
                }
            }
            
            // 处理卡片标题输入框失去焦点
            // 注意：Makepad 可能没有直接的 focus_lost 方法，我们可以通过其他方式检测
            // 暂时注释掉，使用其他方法
            /*
            if self.view.text_input(id!(card_title_input)).focus_lost(actions) {
                if let Some(card_id) = self.card_id {
                    let state = scope.data.get_mut::<State>().unwrap();
                    
                    // 查找当前卡片的原始标题
                    let current_title = state.card_original_texts.get(&card_id).cloned().unwrap_or_default();
                    let input_text = self.view.text_input(id!(card_title_input)).text();
                    
                    if input_text.trim() != current_title && !input_text.trim().is_empty() {
                        println!("CardItem: 失去焦点更新卡片标题: '{}' -> '{}' (卡片ID: {})", current_title, input_text.trim(), card_id);
                        // 设置待更新的卡片标题
                        state.pending_card_update = Some((card_id, input_text.trim().to_string()));
                    }
                }
            }
            */
            
            // 处理详情按钮点击
            if self.view.button(id!(detail_btn)).clicked(actions) {
                if let Some(card_id) = self.card_id {
                    let state = scope.data.get_mut::<State>().unwrap();
                    println!("CardItem: 详情按钮被点击！卡片ID: {}", card_id);
                    state.pending_detail_card_id = Some(card_id);
                    println!("CardItem: pending_detail_card_id 已设置为: {:?}", state.pending_detail_card_id);
                    // 触发重绘以确保事件被处理
                    cx.redraw_all();
                } else {
                    println!("CardItem: 详情按钮被点击，但 card_id 为 None！");
                }
            }
            
            // 处理删除按钮点击
            if self.view.button(id!(delete_card_btn)).clicked(actions) {
                if let Some(card_id) = self.card_id {
                    let state = scope.data.get_mut::<State>().unwrap();
                    println!("CardItem: 删除卡片 {}", card_id);
                    state.pending_delete_card_id = Some(card_id);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 在draw阶段从scope.props获取并存储card_id
        if let Some(card_id) = scope.props.get::<i64>() {
            self.card_id = Some(*card_id);
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}
