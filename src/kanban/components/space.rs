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

                space_title_label = <Button> {
                    width: Fill,
                    height: 35,
                    text: "空间标题",
                    draw_bg: {
                        color: #F8F9FAFF
                    }
                    draw_text: {
                        color: #333333FF,
                        text_style: {
                            font_size: 18.0,
                        }
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
    #[rust]
    is_drop_target: bool,
}

impl Widget for SpaceColumn {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // 处理拖拽放置事件
        match event {
            Event::MouseMove(e) => {
                // 检查是否有拖拽状态
                if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                    if app_state.kanban_state.drag_state.is_some() {
                        // 检测鼠标是否在当前 Space 上方
                        let rect = self.view.area().rect(cx);
                        let is_over = rect.contains(e.abs);

                        if is_over != self.is_drop_target {
                            self.is_drop_target = is_over;
                            cx.redraw_all();

                            if is_over {
                                log!(
                                    "SpaceColumn: 拖拽进入 Space {}",
                                    self.list_id.as_ref().unwrap_or(&"unknown".to_string())
                                );
                            }
                        }
                    }
                }
            }
            Event::MouseUp(e) => {
                // 检查是否有拖拽状态且鼠标在当前 Space 上方
                if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                    if let Some(drag_state) = &app_state.kanban_state.drag_state {
                        let rect = self.view.area().rect(cx);
                        let is_over = rect.contains(e.abs);

                        if is_over {
                            if let Some(space_id_str) = &self.list_id {
                                if let Ok(target_space_id) =
                                    matrix_sdk::ruma::RoomId::parse(space_id_str.as_str())
                                {
                                    log!("SpaceColumn: 放置卡片到 Space {}", target_space_id);

                                    // 计算目标位置（放到列表末尾）
                                    let target_position = if let Some(list) =
                                        app_state.kanban_state.lists.get(&target_space_id)
                                    {
                                        // 获取列表中最后一张卡片的位置
                                        let last_position = list
                                            .card_ids
                                            .iter()
                                            .filter_map(|card_id| {
                                                app_state.kanban_state.cards.get(card_id)
                                            })
                                            .map(|card| card.position)
                                            .max_by(|a, b| a.partial_cmp(b).unwrap())
                                            .unwrap_or(0.0);
                                        last_position + 1000.0
                                    } else {
                                        1000.0
                                    };

                                    cx.action(crate::kanban::KanbanActions::DropCard {
                                        card_id: drag_state.card_id.clone(),
                                        target_space_id,
                                        target_position,
                                    });
                                }
                            }
                        } else if drag_state.source_space_id.to_string()
                            == *self.list_id.as_ref().unwrap_or(&String::new())
                        {
                            // 如果鼠标不在任何 Space 上方，且当前是源 Space，取消拖拽
                            log!("SpaceColumn: 取消拖拽");
                            cx.action(crate::kanban::KanbanActions::CancelDragCard);
                        }

                        self.is_drop_target = false;
                        cx.redraw_all();
                    }
                }
            }
            _ => {}
        }

        // 处理列表标题按钮点击事件
        if let Event::Actions(actions) = event {
            // 处理标题按钮点击 - 打开编辑模态框
            if self.view.button(ids!(space_title_label)).clicked(actions) {
                if let Some(space_id) = scope.props.get::<matrix_sdk::ruma::OwnedRoomId>() {
                    let current_name = self.view.button(ids!(space_title_label)).text();
                    log!(
                        "SpaceColumn: 点击标题按钮，打开编辑模态框: '{}'",
                        current_name
                    );

                    // 发送 action 到 app.rs 打开编辑模态框
                    cx.action(crate::kanban::KanbanActions::ShowEditListName {
                        list_id: space_id.clone(),
                        current_name: current_name.to_string(),
                    });
                }
            }

            // 只在按钮被点击时才输出日志
            if self.view.button(ids!(create_button)).clicked(actions) {
                log!("🎯🎯🎯 SpaceColumn: 创建卡片按钮被点击!!!");

                // 简化架构：Space = List，直接使用 space_id
                // 从 scope.props 获取 space_id
                if let Some(space_id) = scope.props.get::<matrix_sdk::ruma::OwnedRoomId>() {
                    log!("🎯 SpaceColumn: 在列表 {} 中创建新卡片", space_id);
                    // 使用 cx.action() 而不是 cx.widget_action() 以便 action 能传递到 app.rs
                    cx.action(crate::kanban::KanbanActions::CreateCard {
                        space_id: space_id.clone(),
                        title: "新卡片".to_string(),
                    });
                    log!("🎯 SpaceColumn: CreateCard action sent!");
                } else {
                    log!("❌ SpaceColumn: 没有找到 space_id in scope.props");
                }
                cx.redraw_all();
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 从 scope.props 获取 space_id (OwnedRoomId) 并保存为字符串
        if let Some(space_id) = scope.props.get::<matrix_sdk::ruma::OwnedRoomId>() {
            self.list_id = Some(space_id.to_string());
        }

        // 如果是拖拽目标，高亮边框（暂时移除 live! 宏的使用）
        // TODO: 添加视觉反馈

        // 直接使用 scope 绘制，这样 CardList 可以从 scope.props 获取 space_id
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
        // 先让 view 处理事件（这会传递给 PortalList 和其子项）
        self.view.handle_event(cx, event, scope);

        // 然后显式地让每个 space_item 处理事件
        if let Some(mut list) = self.view.portal_list(ids!(spaces)).borrow_mut() {
            let lists: Vec<_> = {
                if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                    let state = &app_state.kanban_state;
                    state.all_lists().into_iter().map(|l| l.clone()).collect()
                } else {
                    Vec::new()
                }
            };

            for list_idx in 0..lists.len() {
                let space_item = list.item(cx, list_idx, live_id!(Space));
                let kanban_list = &lists[list_idx];
                let list_id = kanban_list.id.clone();

                if let Some(app_state) = scope.data.get_mut::<crate::app::AppState>() {
                    let mut space_scope = Scope::with_data_props(app_state, &list_id);
                    space_item.handle_event(cx, event, &mut space_scope);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // 简化架构：直接获取所有列表（Space）
                let lists: Vec<_> = {
                    if let Some(app_state) = scope.data.get::<crate::app::AppState>() {
                        let state = &app_state.kanban_state;
                        state.all_lists().into_iter().map(|l| l.clone()).collect()
                    } else {
                        Vec::new()
                    }
                };

                list.set_item_range(cx, 0, lists.len());

                while let Some(list_idx) = list.next_visible_item(cx) {
                    if list_idx >= lists.len() {
                        continue;
                    }

                    let space_item = list.item(cx, list_idx, live_id!(Space));
                    let kanban_list = &lists[list_idx];

                    // 设置列表标题
                    space_item
                        .button(ids!(space_title_label))
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
                    if let Some(app_state) = scope.data.get_mut::<crate::app::AppState>() {
                        let mut space_scope = Scope::with_data_props(app_state, &list_id);
                        space_item.draw_all(cx, &mut space_scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}
