use makepad_widgets::*;
use crate::kanban::KanbanActions;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub BoardCard = <View> {
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
        
        // 处理点击事件
        if let Event::MouseDown(_) = event {
            if let Some(board_id_str) = &self.board_id {
                log!("BoardCard: Clicked on board {}", board_id_str);
                // 解析 board_id 字符串为 OwnedRoomId
                if let Ok(board_id) = board_id_str.parse() {
                    cx.action(KanbanActions::SelectBoard(board_id));
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 scope.props 获取 board_id
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
                // 获取看板数据
                let boards: Vec<_> = {
                    if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                        app_state.kanban_state.boards.values().cloned().collect()
                    } else {
                        Vec::new()
                    }
                };
                
                list.set_item_range(cx, 0, boards.len());

                while let Some(board_idx) = list.next_visible_item(cx) {
                    if board_idx >= boards.len() {
                        continue;
                    }

                    let board = &boards[board_idx];

                    let board_item = list.item(cx, board_idx, live_id!(Board));

                    // 设置看板名称
                    board_item
                        .label(ids!(board_name_label))
                        .set_text(cx, &board.name);
                    
                    // 传递 board_id 给 BoardCard 并立即绘制
                    let board_id_str = board.id.to_string();
                    let mut board_scope = Scope::with_props(&board_id_str);
                    board_item.draw_all(cx, &mut board_scope);
                }
            }
        }
        DrawStep::done()
    }
}
