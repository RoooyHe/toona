use makepad_widgets::*;
use crate::kanban::models::State;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::kanban::components::card_item::CardItem;

    pub CardList = {{CardList}} {
        <View> {
            width: Fill,
            height: Fill,  // 改为Fill，确保有足够的高度
            flow: Down,
            spacing: 5,
            
            cards = <PortalList> {
                flow: Down,
                spacing: 5,

                Card = <CardItem> {}
            }
            
            // 新卡片输入框
            new_card_input = <View> {
                width: Fill,
                height: Fit,
                margin: {bottom: 8, top: 5},
                padding: 10,
                flow: Down,
                spacing: 5,
                visible: false,

                <RoundedView> {
                    width: Fill,
                    height: 40,
                    draw_bg: {
                        color: #F0F8FFFF
                    }
                    
                    new_card_text_input = <TextInput> {
                        width: Fill,
                        height: Fill,
                        text: "",
                        draw_text: {
                            color: #333333FF,
                            text_style: {
                                font_size: 14.0,
                            }
                        }
                        draw_bg: {
                            color: #FFFFFFFF
                        }
                        draw_cursor: {
                            color: #333333FF
                        }
                    }
                }
                
                <Label> {
                    text: "输入卡片标题，按回车保存",
                    draw_text: {
                        color: #888888FF,
                        text_style: {
                            font_size: 11.0,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardList {
    #[deref]
    view: View,
    #[rust]
    space_idx: Option<usize>,
}

impl Widget for CardList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理新卡片输入框的事件
        if let Event::Actions(actions) = event {
            // 处理新卡片输入框文本变化事件（用于实时更新状态）
            if let Some(text) = self.view.text_input(id!(new_card_text_input)).changed(actions) {
                let state = scope.data.get_mut::<State>().unwrap();
                
                // 使用存储的space_idx
                let space_id = if let Some(space_idx) = self.space_idx {
                    if space_idx < state.spaces_data.len() {
                        state.spaces_data[space_idx].id
                    } else {
                        println!("CardList: space_idx {} 超出范围，使用第一个空间", space_idx);
                        if !state.spaces_data.is_empty() {
                            state.spaces_data[0].id
                        } else {
                            return; // 没有空间数据，直接返回
                        }
                    }
                } else {
                    println!("CardList: space_idx 未设置，使用第一个空间");
                    if !state.spaces_data.is_empty() {
                        state.spaces_data[0].id
                    } else {
                        return; // 没有空间数据，直接返回
                    }
                };
                
                // 更新输入框状态
                state.new_card_inputs.insert(space_id, text.clone());
                println!("CardList: 更新输入框文本到空间 {}: '{}'", space_id, text);
            }
            
            // 处理回车键创建新卡片
            if let Some((text, _)) = self.view.text_input(id!(new_card_text_input)).returned(actions) {
                if !text.trim().is_empty() {
                    let state = scope.data.get_mut::<State>().unwrap();
                    
                    // 使用存储的space_idx
                    let space_id = if let Some(space_idx) = self.space_idx {
                        if space_idx < state.spaces_data.len() {
                            state.spaces_data[space_idx].id
                        } else {
                            println!("CardList: space_idx {} 超出范围，使用第一个空间", space_idx);
                            if !state.spaces_data.is_empty() {
                                state.spaces_data[0].id
                            } else {
                                return; // 没有空间数据，直接返回
                            }
                        }
                    } else {
                        println!("CardList: space_idx 未设置，使用第一个空间");
                        if !state.spaces_data.is_empty() {
                            state.spaces_data[0].id
                        } else {
                            return; // 没有空间数据，直接返回
                        }
                    };
                    
                    println!("新卡片输入框回车，创建卡片: '{}' 到空间: {}", text.trim(), space_id);
                    // 设置待创建的卡片
                    state.pending_create_card = Some((space_id, text.trim().to_string()));
                    // 清空输入框并隐藏
                    self.view.text_input(id!(new_card_text_input)).set_text(cx, "");
                    state.new_card_inputs.remove(&space_id);
                }
            }
            
            // 处理新卡片输入框失去焦点
            // 注意：Makepad 可能没有直接的 focus_lost 方法，我们可以通过其他方式检测
            // 暂时注释掉，使用其他方法
            /*
            if self.view.text_input(id!(new_card_text_input)).focus_lost(actions) {
                let state = scope.data.get_mut::<State>().unwrap();
                let input_text = self.view.text_input(id!(new_card_text_input)).text();
                
                if !input_text.trim().is_empty() {
                    // 使用存储的space_idx
                    let space_id = if let Some(space_idx) = self.space_idx {
                        if space_idx < state.spaces_data.len() {
                            state.spaces_data[space_idx].id
                        } else {
                            println!("CardList: space_idx {} 超出范围，使用第一个空间", space_idx);
                            if !state.spaces_data.is_empty() {
                                state.spaces_data[0].id
                            } else {
                                return; // 没有空间数据，直接返回
                            }
                        }
                    } else {
                        println!("CardList: space_idx 未设置，使用第一个空间");
                        if !state.spaces_data.is_empty() {
                            state.spaces_data[0].id
                        } else {
                            return; // 没有空间数据，直接返回
                        }
                    };
                    
                    println!("新卡片输入框失去焦点，创建卡片: '{}' 到空间: {}", input_text.trim(), space_id);
                    // 设置待创建的卡片
                    state.pending_create_card = Some((space_id, input_text.trim().to_string()));
                    // 清空输入框并隐藏
                    self.view.text_input(id!(new_card_text_input)).set_text(cx, "");
                    state.new_card_inputs.remove(&space_id);
                }
            }
            */
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 在draw阶段从scope.props获取并存储space_idx
        if let Some(space_idx) = scope.props.get::<usize>() {
            self.space_idx = Some(*space_idx);
        }
        
        // 先获取需要的数据
        let (_space_idx, _space_id, has_new_input, current_text, cards_data) = {
            let state = scope.data.get_mut::<State>().unwrap();
            
            // 使用存储的space_idx
            let (space_idx, space_id) = if let Some(space_idx) = self.space_idx {
                if space_idx < state.spaces_data.len() {
                    (space_idx, state.spaces_data[space_idx].id)
                } else {
                    println!("CardList draw_walk: space_idx {} 超出范围，使用第一个空间", space_idx);
                    if !state.spaces_data.is_empty() {
                        (0, state.spaces_data[0].id)
                    } else {
                        return DrawStep::done(); // 没有空间数据，直接返回
                    }
                }
            } else {
                if !state.spaces_data.is_empty() {
                    (0, state.spaces_data[0].id)
                } else {
                    return DrawStep::done(); // 没有空间数据，直接返回
                }
            };
            
            let has_new_input = state.new_card_inputs.contains_key(&space_id);
            let default_text = String::new();
            let current_text = state.new_card_inputs.get(&space_id).unwrap_or(&default_text).clone();
            let cards_data = state.spaces_data[space_idx].cards.clone();
            
            // 调试信息
            if space_idx == 0 {
                println!("CardList draw_walk: 空间 {} 有 {} 张卡片", space_id, cards_data.len());
            }
            
            (space_idx, space_id, has_new_input, current_text, cards_data)
        };

        // 处理新卡片输入框的可见性
        if has_new_input {
            println!("CardList: 显示新卡片输入框，空间ID: {}", _space_id);
            self.view.view(id!(new_card_input)).apply_over(cx, live! { visible: true });
            
            let text_input = self.view.text_input(id!(new_card_text_input));
            text_input.set_text(cx, &current_text);
        } else {
            self.view.view(id!(new_card_input)).apply_over(cx, live! { visible: false });
        }

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, cards_data.len());
                
                // 调试信息
                if cards_data.len() > 0 {
                    println!("CardList PortalList: 设置 {} 张卡片", cards_data.len());
                }

                while let Some(card_idx) = list.next_visible_item(cx) {
                    if card_idx >= cards_data.len() {
                        continue;
                    }

                    let card_item = list.item(cx, card_idx, live_id!(Card));
                    let card = &cards_data[card_idx];
                    
                    println!("渲染卡片 {}: {}", card_idx, card.title);

                    // 只在需要时设置卡片标题输入框的文本（避免覆盖用户输入）
                    let current_text = card_item.text_input(id!(card_title_input)).text();
                    if current_text.is_empty() || current_text == "卡片标题" {
                        card_item
                            .text_input(id!(card_title_input))
                            .set_text(cx, &card.title);
                    }

                    // 设置标签信息
                    let tags_text = if card.tags.is_empty() {
                        "无标签".to_string()
                    } else {
                        card.tags
                            .iter()
                            .map(|tag| tag.title.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    card_item
                        .label(id!(card_tags))
                        .set_text(cx, &format!("标签: {}", tags_text));

                    // 为CardItem传递card_id
                    let mut card_scope = Scope::with_data_props(scope.data.get_mut::<State>().unwrap(), &card.id);
                    card_item.draw_all(cx, &mut card_scope);
                }
            }
        }
        DrawStep::done()
    }
}
