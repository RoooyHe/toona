use makepad_widgets::*;

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
}

impl Widget for CardList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理新卡片输入框的事件
        if let Event::Actions(actions) = event {
            // 处理新卡片输入框文本变化事件（用于实时更新状态）
            if let Some(_text) = self.view.text_input(ids!(new_card_text_input)).changed(actions) {
                // TODO: 实现新卡片输入状态管理
                // let state = scope.data.get_mut::<KanbanAppState>().unwrap();
            }
            
            // 处理回车键创建新卡片
            if let Some((text, _)) = self.view.text_input(ids!(new_card_text_input)).returned(actions) {
                if !text.trim().is_empty() {
                    println!("新卡片输入框回车，创建卡片: '{}'", text.trim());
                    // TODO: 触发创建卡片的 Action
                    // cx.widget_action(widget_uid, &scope.path, KanbanAction::CreateCard(...));
                    self.view.text_input(ids!(new_card_text_input)).set_text(cx, "");
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 scope.props 获取 space_id (简化架构：Space = List)
        let space_id = if let Some(space_id) = scope.props.get::<matrix_sdk::ruma::OwnedRoomId>() {
            space_id.clone()
        } else {
            return DrawStep::done();
        };
        
        // 获取卡片数据并克隆，避免借用冲突
        let cards: Vec<_> = if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
            app_state.kanban_state.list_cards(&space_id).into_iter().map(|c| c.clone()).collect()
        } else {
            Vec::new()
        };
        
        log!("CardList: space_id='{}', found {} cards", space_id, cards.len());

        // 隐藏新卡片输入框（暂时不实现）
        self.view.view(ids!(new_card_input)).apply_over(cx, live! { visible: false });

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, cards.len());

                while let Some(card_idx) = list.next_visible_item(cx) {
                    if card_idx >= cards.len() {
                        continue;
                    }

                    let card_item = list.item(cx, card_idx, live_id!(Card));
                    let card = &cards[card_idx];
                    
                    // 设置卡片标题
                    card_item
                        .text_input(ids!(card_title_input))
                        .set_text(cx, &card.title);

                    // 设置标签信息（暂时显示为空）
                    card_item
                        .label(ids!(card_tags))
                        .set_text(cx, "标签: 无");

                    // 传递 card_id 给 CardItem
                    let card_id = card.id.clone();
                    if let Some(app_state) = scope.data.get_mut::<crate::app::AppState>() {
                        let mut card_scope = Scope::with_data_props(&mut app_state.kanban_state, &card_id);
                        card_item.draw_all(cx, &mut card_scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}
