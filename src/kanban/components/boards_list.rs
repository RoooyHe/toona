use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub BoardCard = {{BoardCard}} {
        width: 250,
        height: 150,
        margin: 10,
        padding: 15,
        show_bg: true,
        draw_bg: {
            color: #0079BF
        }
        
        flow: Down
        
        board_name_label = <Label> {
            width: Fill,
            height: Fit,
            text: "看板名称"
            draw_text: {
                text_style: <THEME_FONT_BOLD>{font_size: 18}
                color: #FFFFFF
            }
        }
        
        <View> {
            width: Fill,
            height: Fit,
            margin: {top: 10}
            
            <Label> {
                text: "点击查看详情"
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    color: #FFFFFFCC
                }
            }
        }
    }

    pub BoardsList = {{BoardsList}} {
        <PortalList> {
            width: Fill, height: Fill
            flow: Right
            spacing: 15
            padding: 10
            
            Board = <BoardCard> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardCard {
    #[deref]
    view: View,
    #[rust]
    board_id: Option<String>,
}

impl Widget for BoardCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理点击事件 - 使用 Hit 来检测点击
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(f) => {
                if f.was_tap() {
                    // 简化架构：不再需要 SelectBoard，直接显示所有列表
                    log!("BoardCard: Clicked (simplified architecture - no board selection needed)");
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 scope.props 获取 board_id（简化架构后不再使用）
        if let Some(board_id) = scope.props.get::<String>() {
            self.board_id = Some(board_id.clone());
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardsList {
    #[deref]
    view: View,
}

impl Widget for BoardsList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // 简化架构：显示所有列表（Space）而不是看板
                let lists: Vec<_> = {
                    if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                        app_state.kanban_state.all_lists().into_iter().map(|l| l.clone()).collect()
                    } else {
                        Vec::new()
                    }
                };
                
                list.set_item_range(cx, 0, lists.len());

                while let Some(list_idx) = list.next_visible_item(cx) {
                    if list_idx >= lists.len() {
                        continue;
                    }

                    let kanban_list = &lists[list_idx];

                    let list_item = list.item(cx, list_idx, live_id!(Board));

                    // 设置列表名称
                    list_item
                        .label(ids!(board_name_label))
                        .set_text(cx, &kanban_list.name);
                    
                    // 传递 space_id 给 BoardCard 并立即绘制
                    let space_id_str = kanban_list.id.to_string();
                    if let Some(app_state) = scope.data.get_mut::<crate::app::AppState>() {
                        let mut list_scope = Scope::with_data_props(app_state, &space_id_str);
                        list_item.draw_all(cx, &mut list_scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}
