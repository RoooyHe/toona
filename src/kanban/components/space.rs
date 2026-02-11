use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::kanban::components::card_list::CardList;

    pub SpaceColumn = {{SpaceColumn}} {
        <RoundedView> {
            width: 300,
            height: 600,
            padding: 15,
            flow: Down,
            spacing: 10,
            draw_bg: {
                color: #E8F4FDFF
            }

            <RoundedView> {
                width: Fill,
                height: Fit,
                flow: Right,
                align: {y: 0.5},

                space_title_input = <TextInput> {
                    width: Fill,
                    height: 35,
                    text: "空间标题",
                    draw_text: {
                        color: #333333FF,
                        text_style: {
                            font_size: 18.0,
                        }
                    }
                    draw_bg: {
                        color: #F8F9FAFF
                    }
                    draw_cursor: {
                        color: #333333FF
                    }
                }
            }

            <ScrollXYView> {
                width: Fill,
                height: Fill,
                flow: Down,
                scroll_bars: <ScrollBars> {
                    show_scroll_x: false,
                    show_scroll_y: true,
                }

                <CardList> {}
            }

            create_button = <Button> {
                text: "创建卡片",
                width: 120,
                height: 40,
                margin: {top: 10}
            }
        }
    }

    pub SpaceList = {{SpaceList}} {
        spaces = <PortalList> {
            flow: Right,
            spacing: 20,

            Space = <SpaceColumn> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SpaceColumn {
    #[deref]
    view: View,
    #[rust]
    list_id: Option<String>,
}

impl Widget for SpaceColumn {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理列表标题输入框事件
        if let Event::Actions(actions) = event {
            // 处理标题输入框文本变化
            if let Some(text) = self.view.text_input(ids!(space_title_input)).changed(actions) {
                if let Some(list_id) = &self.list_id {
                    println!("SpaceColumn: 列表标题输入变化: '{}' (列表ID: {})", text, list_id);
                }
            }
            
            // 处理标题输入框回车
            if let Some((text, _)) = self.view.text_input(ids!(space_title_input)).returned(actions) {
                if let Some(list_id) = &self.list_id {
                    if !text.trim().is_empty() {
                        println!("SpaceColumn: 回车更新列表标题: '{}' (列表ID: {})", text.trim(), list_id);
                        // TODO: 触发更新列表标题的 Action
                    }
                }
            }
            
            // 处理创建卡片按钮点击
            if self.view.button(ids!(create_button)).clicked(actions) {
                if let Some(list_id) = &self.list_id {
                    println!("SpaceColumn: 在列表 {} 中创建新卡片", list_id);
                    // TODO: 触发创建卡片的 Action
                    cx.redraw_all();
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 scope.props 获取 list_id
        if let Some(list_id) = scope.props.get::<String>() {
            self.list_id = Some(list_id.clone());
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SpaceList {
    #[deref]
    view: View,
}

impl Widget for SpaceList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // 获取列表数据并克隆，避免借用冲突
                let lists: Vec<_> = {
                    if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                        let state = &app_state.kanban_state;
                        let board_lists = state.current_board_lists();
                        log!("SpaceList: Found {} lists in current board", board_lists.len());
                        if let Some(board_id) = &state.current_board_id {
                            log!("SpaceList: Current board ID: {}", board_id);
                            if let Some(board) = state.boards.get(board_id) {
                                log!("SpaceList: Board '{}' has {} list_ids:", board.name, board.list_ids.len());
                                for list_id in &board.list_ids {
                                    log!("  - board.list_ids contains: '{}'", list_id);
                                }
                            }
                        } else {
                            log!("SpaceList: No current board selected");
                        }
                        log!("SpaceList: Total boards: {}, Total lists in state.lists: {}", 
                            state.boards.len(), state.lists.len());
                        log!("SpaceList: Keys in state.lists HashMap:");
                        for key in state.lists.keys() {
                            log!("  - state.lists key: '{}'", key);
                        }
                        board_lists.into_iter().map(|l| l.clone()).collect()
                    } else {
                        // 如果没有 AppState，返回空列表
                        log!("SpaceList: No AppState in scope!");
                        Vec::new()
                    }
                };
                
                log!("SpaceList: Setting item range to {}", lists.len());
                list.set_item_range(cx, 0, lists.len());

                while let Some(list_idx) = list.next_visible_item(cx) {
                    if list_idx >= lists.len() {
                        continue;
                    }

                    let space_item = list.item(cx, list_idx, live_id!(Space));
                    let kanban_list = &lists[list_idx];

                    // 设置列表标题
                    space_item
                        .text_input(ids!(space_title_input))
                        .set_text(cx, &kanban_list.name);

                    // 设置背景颜色
                    let colors = [
                        0xE8F4FDFFu32, // 浅蓝色
                        0xF0FDF4FFu32, // 浅绿色
                        0xFEF3C7FFu32, // 浅黄色
                        0xFDF2F8FFu32, // 浅粉色
                        0xF3E8FFFFu32, // 浅紫色
                        0xFFF1F2FFu32, // 浅红色
                        0xE0F2FEFFu32, // 浅青色
                        0xF0FFF4FFu32, // 浅薄荷绿
                    ];
                    let color_index = list_idx % colors.len();
                    let bg_color = colors[color_index];

                    space_item.apply_over(
                        cx,
                        live! {
                            draw_bg: {
                                color: (bg_color)
                            }
                        },
                    );

                    // 传递 list_id 给 SpaceColumn
                    let list_id = kanban_list.id.clone();
                    if let Some(app_state_mut) = scope.data.get_mut::<crate::app::AppState>() {
                        let kanban_state = &mut app_state_mut.kanban_state;
                        let mut space_scope = Scope::with_data_props(kanban_state, &list_id);
                        space_item.draw_all(cx, &mut space_scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}


