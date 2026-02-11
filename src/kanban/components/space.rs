use makepad_widgets::*;
use crate::kanban::models::State;

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
    space_idx: Option<usize>,
}

impl Widget for SpaceColumn {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理空间标题输入框事件
        if let Event::Actions(actions) = event {
            // 处理空间标题输入框文本变化
            if let Some(text) = self.view.text_input(id!(space_title_input)).changed(actions) {
                if let Some(space_idx) = self.space_idx {
                    let state = scope.data.get_mut::<State>().unwrap();
                    
                    if space_idx < state.spaces_data.len() {
                        let space_id = state.spaces_data[space_idx].id;
                        println!("SpaceColumn: 空间标题输入变化: '{}' (空间ID: {})", text, space_id);
                        // 这里可以实时更新状态，但通常我们在失去焦点或回车时才保存
                    }
                }
            }
            
            // 处理空间标题输入框回车
            if let Some((text, _)) = self.view.text_input(id!(space_title_input)).returned(actions) {
                if let Some(space_idx) = self.space_idx {
                    let state = scope.data.get_mut::<State>().unwrap();
                    
                    if space_idx < state.spaces_data.len() {
                        let space_id = state.spaces_data[space_idx].id;
                        let current_title = &state.spaces_data[space_idx].title;
                        
                        if text.trim() != current_title && !text.trim().is_empty() {
                            println!("SpaceColumn: 回车更新空间标题: '{}' -> '{}' (空间ID: {})", current_title, text.trim(), space_id);
                            // 设置待更新的空间标题
                            state.pending_space_update = Some((space_id, text.trim().to_string()));
                        }
                    }
                }
            }
            
            // 处理空间标题输入框失去焦点
            // 注意：Makepad 可能没有直接的 focus_lost 方法，我们可以通过其他方式检测
            // 暂时注释掉，使用其他方法
            /*
            if self.view.text_input(id!(space_title_input)).focus_lost(actions) {
                if let Some(space_idx) = self.space_idx {
                    let state = scope.data.get_mut::<State>().unwrap();
                    
                    if space_idx < state.spaces_data.len() {
                        let space_id = state.spaces_data[space_idx].id;
                        let current_title = &state.spaces_data[space_idx].title;
                        let input_text = self.view.text_input(id!(space_title_input)).text();
                        
                        if input_text.trim() != current_title && !input_text.trim().is_empty() {
                            println!("SpaceColumn: 失去焦点更新空间标题: '{}' -> '{}' (空间ID: {})", current_title, input_text.trim(), space_id);
                            // 设置待更新的空间标题
                            state.pending_space_update = Some((space_id, input_text.trim().to_string()));
                        }
                    }
                }
            }
            */
        }
        
        // 处理鼠标点击事件，用于调试焦点问题
        if let Event::MouseDown(_) = event {
            println!("SpaceColumn: 检测到鼠标点击事件");
        }
        
        // 处理添加卡片按钮点击事件
        if let Event::Actions(actions) = event {
            if self.view.button(id!(create_button)).clicked(actions) {
                println!("SpaceColumn: 检测到创建卡片按钮点击");
                
                // 使用存储的space_idx
                if let Some(space_idx) = self.space_idx {
                    let state = scope.data.get_mut::<State>().unwrap();
                    
                    if space_idx < state.spaces_data.len() {
                        let space_id = state.spaces_data[space_idx].id;
                        let space_title = &state.spaces_data[space_idx].title;
                        
                        println!("SpaceColumn: 在空间 '{}' (ID: {}, 索引: {}) 中添加新卡片输入框", space_title, space_id, space_idx);
                        
                        // 直接添加新卡片输入框状态
                        state.new_card_inputs.insert(space_id, String::new());
                        cx.redraw_all();
                    } else {
                        println!("SpaceColumn: 空间索引 {} 超出范围", space_idx);
                    }
                } else {
                    println!("SpaceColumn: space_idx 未设置");
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 在draw阶段从scope.props获取并存储space_idx
        if let Some(space_idx) = scope.props.get::<usize>() {
            self.space_idx = Some(*space_idx);
        }
        
        // 确保事件能正确传递到子组件
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
        // 首先让view处理事件，这会将事件传递给子组件
        self.view.handle_event(cx, event, scope);
        
        // 简化：让主应用程序处理按钮点击
        // 这里不处理按钮事件，交给App处理
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            let state = scope.data.get_mut::<State>().unwrap();

            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, state.spaces_data.len());

                while let Some(space_idx) = list.next_visible_item(cx) {
                    if space_idx >= state.spaces_data.len() {
                        continue;
                    }

                    let space_item = list.item(cx, space_idx, live_id!(Space));
                    let space = &state.spaces_data[space_idx];

                    // 只在需要时设置空间标题输入框的文本（避免覆盖用户输入）
                    let current_text = space_item.text_input(id!(space_title_input)).text();
                    if current_text.is_empty() || current_text == "空间标题" {
                        space_item
                            .text_input(id!(space_title_input))
                            .set_text(cx, &space.title);
                    }

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
                    let color_index = space_idx % colors.len();
                    let bg_color = colors[color_index];

                    space_item.apply_over(
                        cx,
                        live! {
                            draw_bg: {
                                color: (bg_color)
                            }
                        },
                    );

                    // 为 SpaceColumn 传递 space_idx
                    let mut space_scope = Scope::with_data_props(state, &space_idx);
                    space_item.draw_all(cx, &mut space_scope);
                }
            }
        }
        DrawStep::done()
    }
}


