use chrono;
use makepad_widgets::*;

use crate::models::State;
use crate::models::dto::{TodoDto, ActiveDto};
use crate::services::ApiService;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::space::SpaceList;
    use crate::components::card_modal::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                window: {
                    title: "Betula - Kanban Board"
                },
                body = <View> {
                    width: Fill,
                    height: Fill,
                    flow: Overlay,

                    // 主要内容
                    main_content = <View> {
                        width: Fill,
                        height: Fill,
                        flow: Down,
                        padding: 20,
                        spacing: 20,

                        <View> {
                            width: Fill,
                            height: Fit,
                            flow: Right,
                            spacing: 20,
                        }

                        <ScrollXYView> {
                            width: Fill,
                            height: Fill,
                            scroll_bars: <ScrollBars> {
                                show_scroll_x: true,
                                show_scroll_y: true,
                            }

                            <SpaceList> {}

                            create_button = <Button> {
                                text: "创建空间",
                                width: 120,
                                height: 40,
                            }
                        }
                    }

                    // 卡片详情模态框 - 使用组件
                    card_detail_modal = <CardDetailModal> {}
                }
            }
        }
    }
}

#[derive(Live)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

impl App {
    fn start_space_fetch(&mut self) {
        // 重置接收器以允许重复调用
        self.state.space_rx = None;

        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.space_signal.clone();
        self.state.space_rx = Some(rx);

        // 使用新的 ApiService
        ApiService::fetch_spaces(tx, signal);
    }

    fn handle_space_signal(&mut self, cx: &mut Cx) {
        if !self.state.space_signal.check_and_clear() {
            return;
        }

        // 收集所有接收到的数据
        let mut received_spaces = Vec::new();
        if let Some(rx) = &self.state.space_rx {
            while let Ok(spaces) = rx.try_recv() {
                received_spaces.push(spaces);
            }
        }

        // 处理接收到的数据
        for spaces in received_spaces {
            self.state.spaces_data = spaces;

            // 缓存所有卡片的原始文本
            self.state.card_original_texts.clear();
            let mut total_cards = 0;
            for space in &self.state.spaces_data {
                for card in &space.cards {
                    self.state
                        .card_original_texts
                        .insert(card.id, card.title.clone());
                    total_cards += 1;
                }
            }

            println!(
                "收到 {} 个空间的数据，共 {} 张卡片，通过 PortalList 实现真正无限制渲染",
                self.state.spaces_data.len(),
                total_cards
            );

            // 打印第一个空间的卡片信息用于调试
            if !self.state.spaces_data.is_empty() {
                let first_space = &self.state.spaces_data[0];
                println!(
                    "第一个空间 '{}' 有 {} 张卡片",
                    first_space.title,
                    first_space.cards.len()
                );
                for card in &first_space.cards {
                    println!("  - 卡片: {}", card.title);
                }
            }

            cx.redraw_all();
        }
    }

    fn start_create_space(&mut self) {
        // 重置接收器以允许重复调用
        self.state.create_space_rx = None;

        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.create_space_signal.clone();
        self.state.create_space_rx = Some(rx);

        // 使用新的 ApiService
        let title = format!("新空间 {}", chrono::Utc::now().format("%H:%M:%S"));
        ApiService::create_space(title, tx, signal);
    }

    fn handle_create_space_signal(&mut self, _cx: &mut Cx) {
        if !self.state.create_space_signal.check_and_clear() {
            return;
        }

        // 收集所有接收到的结果
        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.create_space_rx {
            while let Ok(success) = rx.try_recv() {
                received_results.push(success);
            }
        }

        // 处理接收到的结果
        for success in received_results {
            if success {
                // 创建成功后自动刷新空间列表
                self.start_space_fetch();
            }
        }
    }

    // 移除未使用的方法，这些方法已经被新的 ApiService 替代
    // hide_card_modal, find_card_by_id, find_space_id_by_card_id, create_card, update_card 等方法已不再需要

    fn delete_card(&mut self, card_id: i64) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.card_signal.clone();
        self.state.card_rx = Some(rx);

        // 使用新的 ApiService
        ApiService::delete_card(card_id, tx, signal);
    }

    fn fetch_card_detail(&mut self, card_id: i64) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.card_detail_signal.clone();
        self.state.card_detail_rx = Some(rx);

        println!("fetch_card_detail: 开始获取卡片详情 {}", card_id);

        // 使用新的 ApiService
        ApiService::fetch_card_detail(card_id, tx, signal);

        // 同时获取全部标签
        self.fetch_all_tags();
    }

    fn fetch_all_tags(&mut self) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.tags_signal.clone();
        self.state.tags_rx = Some(rx);

        println!("fetch_all_tags: 开始获取全部标签");

        // 使用新的 ApiService
        ApiService::fetch_all_tags(tx, signal);
    }

    fn create_card_from_input(&mut self, space_id: i64, title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.card_signal.clone();
        self.state.card_rx = Some(rx);

        println!("create_card_from_input: 开始创建卡片 '{}' 到空间 {}", title, space_id);

        // 使用新的 ApiService
        ApiService::create_card(space_id, title, tx, signal);
    }

    fn handle_card_signal(&mut self, _cx: &mut Cx) {
        if !self.state.card_signal.check_and_clear() {
            return;
        }

        println!("handle_card_signal: 收到卡片信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.card_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_card_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_card_signal: 卡片创建成功，刷新数据");
                self.start_space_fetch(); // 刷新数据
            } else {
                println!("handle_card_signal: 卡片创建失败");
            }
        }
    }

    fn update_space_title(&mut self, space_id: i64, new_title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.space_update_signal.clone();
        self.state.space_update_rx = Some(rx);

        // 使用新的 ApiService
        ApiService::update_space_title(space_id, new_title, tx, signal);
    }

    fn update_card_title(&mut self, card_id: i64, new_title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.card_update_signal.clone();
        self.state.card_update_rx = Some(rx);

        // 使用新的 ApiService
        ApiService::update_card_title(card_id, new_title, tx, signal);
    }

    fn update_card_description(&mut self, card_id: i64, new_description: String) {
        // 获取当前卡片的完整信息
        if let Some(card_detail) = &self.state.card_detail_data {
            let (tx, rx) = std::sync::mpsc::channel();
            let signal = self.state.card_update_signal.clone();
            self.state.card_update_rx = Some(rx);

            println!("update_card_description: 更新卡片 {} 的描述", card_id);

            // 使用新的 ApiService，传递完整的卡片信息
            ApiService::update_card_description(
                card_id, 
                card_detail.title.clone(),  // 保持原标题
                new_description, 
                card_detail.status,  // 保持原状态
                tx, 
                signal
            );
        }
    }

    fn handle_space_update_signal(&mut self, _cx: &mut Cx) {
        if !self.state.space_update_signal.check_and_clear() {
            return;
        }

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.space_update_rx {
            while let Ok(success) = rx.try_recv() {
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                self.start_space_fetch(); // 刷新数据
            }
        }
    }

    fn handle_card_update_signal(&mut self, _cx: &mut Cx) {
        if !self.state.card_update_signal.check_and_clear() {
            return;
        }

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.card_update_rx {
            while let Ok(success) = rx.try_recv() {
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                self.start_space_fetch(); // 刷新数据
            }
        }
    }

    fn handle_card_detail_signal(&mut self, cx: &mut Cx) {
        if !self.state.card_detail_signal.check_and_clear() {
            return;
        }

        println!("handle_card_detail_signal: 收到卡片详情信号");

        let mut received_details = Vec::new();
        if let Some(rx) = &self.state.card_detail_rx {
            while let Ok(card_detail) = rx.try_recv() {
                println!("handle_card_detail_signal: 收到卡片详情数据: {:?}", card_detail);
                received_details.push(card_detail);
            }
        }

        for card_detail in received_details {
            // 更新模态框内容
            self.ui.label(id!(card_title)).set_text(cx, &card_detail.title);
            
            let description = card_detail.description.as_deref().unwrap_or("暂无描述");
            self.ui.label(id!(card_description_label)).set_text(cx, description);
            
            let status_text = match card_detail.status {
                Some(true) => "已完成",
                Some(false) => "进行中",
                None => "未设置",
            };
            self.ui.label(id!(card_status)).set_text(cx, status_text);
            
            if card_detail.tags.is_empty() {
                self.ui.label(id!(card_tags)).set_text(cx, "暂无标签");
            } else {
                let tags_text = card_detail.tags.iter()
                    .map(|tag| tag.title.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                self.ui.label(id!(card_tags)).set_text(cx, &tags_text);
            }
            
            if card_detail.todos.is_empty() {
                self.ui.label(id!(card_todos)).set_text(cx, "暂无待办事项");
            } else {
                let todos_text = card_detail.todos.iter()
                    .map(|todo| {
                        let status = if todo.completed.unwrap_or(false) { "✓" } else { "○" };
                        format!("{} {}", status, todo.title)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                self.ui.label(id!(card_todos)).set_text(cx, &todos_text);
            }
            
            if card_detail.active.is_empty() {
                self.ui.label(id!(card_active)).set_text(cx, "暂无活动记录");
            } else {
                let active_text = card_detail.active.iter()
                    .map(|active| {
                        if let Some(start_time) = &active.start_time {
                            format!("{} (开始时间: {})", active.title, start_time)
                        } else {
                            active.title.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                self.ui.label(id!(card_active)).set_text(cx, &active_text);
            }

            // 存储当前卡片详情数据
            self.state.card_detail_data = Some(card_detail.clone());

            // 更新Todo显示
            self.update_todo_items(cx, &card_detail.todos);

            // 更新Active显示
            self.update_active_items(cx, &card_detail.active);

            println!("handle_card_detail_signal: 模态框数据已更新");
        }
    }

    fn update_todo_items(&mut self, cx: &mut Cx, todos: &[TodoDto]) {
        // 存储 Todo 数据到 state（用于事件处理和渲染）
        self.state.current_todos = todos.to_vec();
        
        // 打印 Todo 信息
        for (index, todo) in todos.iter().enumerate() {
            println!("update_todo_items: Todo {} - '{}' (完成: {})", 
                index + 1, todo.title, todo.completed.unwrap_or(false));
        }
        
        cx.redraw_all();
    }

    fn handle_tags_signal(&mut self, cx: &mut Cx) {
        if !self.state.tags_signal.check_and_clear() {
            return;
        }

        println!("handle_tags_signal: 收到标签信号");

        let mut received_tags = Vec::new();
        if let Some(rx) = &self.state.tags_rx {
            while let Ok(tags) = rx.try_recv() {
                println!("handle_tags_signal: 收到 {} 个标签", tags.len());
                received_tags.push(tags);
            }
        }

        for tags in received_tags {
            self.state.all_tags = tags;
            println!("handle_tags_signal: 标签数据已更新，共 {} 个标签", self.state.all_tags.len());
            
            // 打印所有标签信息
            for tag in &self.state.all_tags {
                println!("  - 标签: {} (ID: {}, 颜色: {:?})", tag.title, tag.id, tag.color);
            }
            
            // 更新标签按钮
            self.update_tag_buttons(cx);
            
            cx.redraw_all();
        }
    }

    fn update_tag_buttons(&mut self, cx: &mut Cx) {
        // 标签数据已存储在 self.state.all_tags 中
        
        // 打印标签信息
        for (index, tag) in self.state.all_tags.iter().enumerate() {
            println!("update_tag_buttons: 标签 {} - '{}' (ID: {})", index + 1, tag.title, tag.id);
        }
        
        // 更新固定的 3 个标签按钮文本
        if self.state.all_tags.len() > 0 {
            self.ui.button(id!(tag_button_1)).set_text(cx, &self.state.all_tags[0].title);
        }
        if self.state.all_tags.len() > 1 {
            self.ui.button(id!(tag_button_2)).set_text(cx, &self.state.all_tags[1].title);
        }
        if self.state.all_tags.len() > 2 {
            self.ui.button(id!(tag_button_3)).set_text(cx, &self.state.all_tags[2].title);
        }
        
        println!("update_tag_buttons: 已更新 {} 个标签按钮", self.state.all_tags.len().min(3));
        
        cx.redraw_all();
    }

    fn handle_card_tags_update_signal(&mut self, _cx: &mut Cx) {
        if !self.state.card_tags_update_signal.check_and_clear() {
            return;
        }

        println!("handle_card_tags_update_signal: 收到卡片标签更新信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.card_tags_update_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_card_tags_update_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_card_tags_update_signal: 标签更新成功，刷新数据");
                self.start_space_fetch(); // 刷新数据
                // 重新获取卡片详情
                if let Some(card_detail) = &self.state.card_detail_data {
                    self.fetch_card_detail(card_detail.id);
                }
            } else {
                println!("handle_card_tags_update_signal: 标签更新失败");
            }
        }
    }

    fn add_tag_to_card(&mut self, card_id: i64, tag_id: i64) {
        if let Some(card_detail) = &self.state.card_detail_data {
            // 检查标签是否已存在
            if card_detail.tags.iter().any(|tag| tag.id == tag_id) {
                println!("add_tag_to_card: 标签 {} 已存在于卡片 {}", tag_id, card_id);
                return;
            }

            // 找到要添加的标签
            if let Some(tag_to_add) = self.state.all_tags.iter().find(|tag| tag.id == tag_id) {
                let mut new_tags = card_detail.tags.clone();
                new_tags.push(tag_to_add.clone());

                println!("add_tag_to_card: 为卡片 {} 添加标签 '{}'", card_id, tag_to_add.title);

                let (tx, rx) = std::sync::mpsc::channel();
                let signal = self.state.card_tags_update_signal.clone();
                self.state.card_tags_update_rx = Some(rx);

                // 使用新的 ApiService
                ApiService::update_card_tags(
                    card_id,
                    card_detail.title.clone(),
                    card_detail.description.clone(),
                    card_detail.status,
                    new_tags,
                    tx,
                    signal,
                );
            }
        }
    }

    fn create_new_tag(&mut self, title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.create_tag_signal.clone();
        self.state.create_tag_rx = Some(rx);

        println!("create_new_tag: 创建新标签 '{}'", title);

        // 使用新的 ApiService
        ApiService::create_tag(title, tx, signal);
    }

    fn handle_create_tag_signal(&mut self, _cx: &mut Cx) {
        if !self.state.create_tag_signal.check_and_clear() {
            return;
        }

        println!("handle_create_tag_signal: 收到创建标签信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.create_tag_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_create_tag_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_create_tag_signal: 标签创建成功，刷新标签列表");
                self.fetch_all_tags(); // 重新获取标签列表
                // 清空输入框
                self.state.new_tag_input.clear();
                self.state.show_new_tag_input = false;
            } else {
                println!("handle_create_tag_signal: 标签创建失败");
            }
        }
    }

    fn create_new_todo(&mut self, title: String) {
        if let Some(card_detail) = &self.state.card_detail_data {
            let (tx, rx) = std::sync::mpsc::channel();
            let signal = self.state.create_todo_signal.clone();
            self.state.create_todo_rx = Some(rx);

            println!("create_new_todo: 创建新Todo '{}'", title);

            // 使用新的 ApiService
            ApiService::create_todo(card_detail.id, title, tx, signal);
        }
    }

    fn toggle_todo(&mut self, todo_id: i64, title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.update_todo_signal.clone();
        self.state.update_todo_rx = Some(rx);

        println!("toggle_todo: 切换Todo状态 {}", todo_id);

        // 使用新的 ApiService
        ApiService::update_todo(todo_id, title, tx, signal);
    }

    fn delete_todo(&mut self, todo_id: i64) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.delete_todo_signal.clone();
        self.state.delete_todo_rx = Some(rx);

        println!("delete_todo: 删除Todo {}", todo_id);

        // 使用新的 ApiService
        ApiService::delete_todo(todo_id, tx, signal);
    }

    fn handle_create_todo_signal(&mut self, _cx: &mut Cx) {
        if !self.state.create_todo_signal.check_and_clear() {
            return;
        }

        println!("handle_create_todo_signal: 收到创建Todo信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.create_todo_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_create_todo_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_create_todo_signal: Todo创建成功，刷新卡片详情");
                if let Some(card_detail) = &self.state.card_detail_data {
                    self.fetch_card_detail(card_detail.id); // 重新获取卡片详情
                }
                // 清空输入框
                self.state.new_todo_input.clear();
                self.state.show_new_todo_input = false;
            } else {
                println!("handle_create_todo_signal: Todo创建失败");
            }
        }
    }

    fn handle_update_todo_signal(&mut self, _cx: &mut Cx) {
        if !self.state.update_todo_signal.check_and_clear() {
            return;
        }

        println!("handle_update_todo_signal: 收到更新Todo信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.update_todo_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_update_todo_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_update_todo_signal: Todo更新成功，刷新卡片详情");
                if let Some(card_detail) = &self.state.card_detail_data {
                    self.fetch_card_detail(card_detail.id); // 重新获取卡片详情
                }
            } else {
                println!("handle_update_todo_signal: Todo更新失败");
            }
        }
    }

    fn handle_delete_todo_signal(&mut self, _cx: &mut Cx) {
        if !self.state.delete_todo_signal.check_and_clear() {
            return;
        }

        println!("handle_delete_todo_signal: 收到删除Todo信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.delete_todo_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_delete_todo_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_delete_todo_signal: Todo删除成功，刷新卡片详情");
                if let Some(card_detail) = &self.state.card_detail_data {
                    self.fetch_card_detail(card_detail.id); // 重新获取卡片详情
                }
            } else {
                println!("handle_delete_todo_signal: Todo删除失败");
            }
        }
    }

    fn create_new_active(&mut self, title: String) {
        if let Some(card_detail) = &self.state.card_detail_data {
            let (tx, rx) = std::sync::mpsc::channel();
            let signal = self.state.create_active_signal.clone();
            self.state.create_active_rx = Some(rx);

            println!("create_new_active: 创建新Active '{}'", title);

            // 使用新的 ApiService
            ApiService::create_active(card_detail.id, title, tx, signal);
        }
    }

    fn delete_active(&mut self, active_id: i64) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.delete_active_signal.clone();
        self.state.delete_active_rx = Some(rx);

        println!("delete_active: 删除Active {}", active_id);

        // 使用新的 ApiService
        ApiService::delete_active(active_id, tx, signal);
    }

    fn update_active_items(&mut self, cx: &mut Cx, actives: &[ActiveDto]) {
        // 存储 Active 数据到 state（用于事件处理和渲染）
        self.state.current_actives = actives.to_vec();
        
        // 打印 Active 信息
        for (index, active) in actives.iter().enumerate() {
            println!("update_active_items: Active {} - '{}' (开始时间: {:?})", 
                index + 1, active.title, active.start_time);
        }
        
        cx.redraw_all();
    }

    fn handle_create_active_signal(&mut self, _cx: &mut Cx) {
        if !self.state.create_active_signal.check_and_clear() {
            return;
        }

        println!("handle_create_active_signal: 收到创建Active信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.create_active_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_create_active_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_create_active_signal: Active创建成功，刷新卡片详情");
                if let Some(card_detail) = &self.state.card_detail_data {
                    self.fetch_card_detail(card_detail.id); // 重新获取卡片详情
                }
                // 清空输入框
                self.state.new_active_input.clear();
                self.state.show_new_active_input = false;
            } else {
                println!("handle_create_active_signal: Active创建失败");
            }
        }
    }

    fn handle_delete_active_signal(&mut self, _cx: &mut Cx) {
        if !self.state.delete_active_signal.check_and_clear() {
            return;
        }

        println!("handle_delete_active_signal: 收到删除Active信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.delete_active_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_delete_active_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_delete_active_signal: Active删除成功，刷新卡片详情");
                if let Some(card_detail) = &self.state.card_detail_data {
                    self.fetch_card_detail(card_detail.id); // 重新获取卡片详情
                }
            } else {
                println!("handle_delete_active_signal: Active删除失败");
            }
        }
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::components::live_design(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.start_space_fetch();
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.start_space_fetch();
            }
            Event::Signal => {
                self.handle_space_signal(cx);
                self.handle_create_space_signal(cx);
                self.handle_card_signal(cx);
                self.handle_space_update_signal(cx);
                self.handle_card_update_signal(cx);
                self.handle_card_detail_signal(cx);
                self.handle_tags_signal(cx);
                self.handle_card_tags_update_signal(cx);
                self.handle_create_tag_signal(cx);
                self.handle_create_todo_signal(cx);
                self.handle_update_todo_signal(cx);
                self.handle_delete_todo_signal(cx);
                self.handle_create_active_signal(cx);
                self.handle_delete_active_signal(cx);
            }
            Event::KeyDown(key_event) => {
                // 添加键盘快捷键测试
                if key_event.key_code == KeyCode::KeyA {
                    println!("键盘快捷键A被按下，测试添加输入框");
                    if !self.state.spaces_data.is_empty() {
                        let space_id = self.state.spaces_data[0].id;
                        self.state.new_card_inputs.insert(space_id, String::new());
                        println!("通过键盘快捷键添加输入框到空间: {}", space_id);
                        cx.redraw_all();
                    }
                }
            }
            _ => {}
        }

        self.match_event(cx, event);
        let mut scope = Scope::with_data(&mut self.state);
        self.ui.handle_event(cx, event, &mut scope);

        // 处理待处理的按钮点击 - 移除pending_add_card_space_id处理，现在直接在SpaceColumn中处理

        if let Some(card_id) = self.state.pending_edit_card_id.take() {
            println!("编辑卡片: {}", card_id);
        }

        if let Some(card_id) = self.state.pending_delete_card_id.take() {
            println!("删除卡片: {}", card_id);
            self.delete_card(card_id);
        }

        // 处理卡片详情查看
        if let Some(card_id) = self.state.pending_detail_card_id.take() {
            println!("App.handle_event: 检测到 pending_detail_card_id = {}", card_id);
            println!("App.handle_event: 开始获取卡片详情...");
            self.fetch_card_detail(card_id);
            // 打开模态框
            println!("App.handle_event: 尝试打开模态框...");
            
            // CardDetailModal 本身就是 Modal
            let modal_ref = self.ui.modal(id!(card_detail_modal));
            modal_ref.open(cx);
            println!("App.handle_event: 模态框已调用 open()");
            
            // 强制重绘
            cx.redraw_all();
            println!("App.handle_event: 已触发 redraw_all()");
        }

        // 处理待处理的更新
        if let Some((space_id, new_title)) = self.state.pending_space_update.take() {
            println!("更新空间标题: {} -> {}", space_id, new_title);
            self.update_space_title(space_id, new_title);
        }

        if let Some((card_id, new_title)) = self.state.pending_card_update.take() {
            println!("更新卡片标题: {} -> {}", card_id, new_title);
            self.update_card_title(card_id, new_title);
        }

        // 处理新卡片创建
        if let Some((space_id, title)) = self.state.pending_create_card.take() {
            if !title.trim().is_empty() {
                println!("创建新卡片: {} 到空间: {}", title, space_id);
                self.create_card_from_input(space_id, title.trim().to_string());
                // 清除输入框状态
                self.state.new_card_inputs.remove(&space_id);
                cx.redraw_all();
            }
        }

        // 处理新标签创建
        if let Some(title) = self.state.pending_create_tag.take() {
            if !title.trim().is_empty() {
                println!("创建新标签: {}", title);
                self.create_new_tag(title.trim().to_string());
            }
        }

        // 处理新Todo创建
        if let Some(title) = self.state.pending_create_todo.take() {
            if !title.trim().is_empty() {
                println!("创建新Todo: {}", title);
                self.create_new_todo(title.trim().to_string());
            }
        }

        // 处理Todo状态切换
        if let Some((todo_id, _completed)) = self.state.pending_toggle_todo.take() {
            // 从 current_todos 中找到对应的 Todo
            if let Some(todo) = self.state.current_todos.iter().find(|t| t.id == todo_id) {
                println!("切换Todo状态: {}", todo_id);
                self.toggle_todo(todo_id, todo.title.clone());
            }
        }

        // 处理Todo删除
        if let Some(todo_id) = self.state.pending_delete_todo.take() {
            println!("删除Todo: {}", todo_id);
            self.delete_todo(todo_id);
        }

        // 处理Active创建
        if let Some(title) = self.state.pending_create_active.take() {
            if !title.trim().is_empty() {
                println!("创建新Active: {}", title);
                self.create_new_active(title.trim().to_string());
            }
        }

        // 处理Active删除
        if let Some(active_id) = self.state.pending_delete_active.take() {
            println!("删除Active: {}", active_id);
            self.delete_active(active_id);
        }
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 处理刷新按钮点击
        if self.ui.button(id!(refresh_button)).clicked(&actions) {
            self.start_space_fetch();
        }

        // 处理创建空间按钮点击
        if self.ui.button(id!(create_button)).clicked(&actions) {
            self.start_create_space();
        }

        // 处理模态框关闭按钮
        if self.ui.button(id!(close_button)).clicked(&actions) {
            println!("关闭卡片详情模态框");
            // CardDetailModal 本身就是 Modal
            let modal_ref = self.ui.modal(id!(card_detail_modal));
            modal_ref.close(cx);
            // 隐藏标签下拉框
            self.ui.view(id!(tag_dropdown)).set_visible(cx, false);
        }

        // 处理添加标签按钮
        if self.ui.button(id!(add_tag_button)).clicked(&actions) {
            println!("点击添加标签按钮");
            let dropdown = self.ui.view(id!(tag_dropdown));
            let is_visible = dropdown.visible();
            dropdown.set_visible(cx, !is_visible);
            cx.redraw_all();
        }

        // 处理描述编辑按钮
        if self.ui.button(id!(edit_description_button)).clicked(&actions) {
            println!("点击编辑描述按钮");
            // 显示编辑区域，隐藏显示区域
            self.ui.view(id!(description_edit_container)).set_visible(cx, true);
            
            // 将当前描述填充到输入框
            if let Some(card_detail) = &self.state.card_detail_data {
                let description = card_detail.description.as_deref().unwrap_or("");
                self.ui.text_input(id!(card_description_input)).set_text(cx, description);
            }
            
            cx.redraw_all();
        }

        // 处理保存描述按钮
        if self.ui.button(id!(save_description_button)).clicked(&actions) {
            println!("点击保存描述按钮");
            let description = self.ui.text_input(id!(card_description_input)).text();
            
            if let Some(card_detail) = &self.state.card_detail_data {
                println!("保存描述: '{}'", description);
                self.update_card_description(card_detail.id, description);
                
                // 隐藏编辑区域
                self.ui.view(id!(description_edit_container)).set_visible(cx, false);
            }
            
            cx.redraw_all();
        }

        // 处理取消描述编辑按钮
        if self.ui.button(id!(cancel_description_button)).clicked(&actions) {
            println!("点击取消描述编辑按钮");
            // 隐藏编辑区域
            self.ui.view(id!(description_edit_container)).set_visible(cx, false);
            cx.redraw_all();
        }

        // 处理新增标签按钮
        if self.ui.button(id!(new_tag_button)).clicked(&actions) {
            println!("点击新增标签按钮");
            let container = self.ui.view(id!(new_tag_input_container));
            let is_visible = container.visible();
            container.set_visible(cx, !is_visible);
            
            if !is_visible {
                // 显示输入框时清空内容
                self.ui.text_input(id!(new_tag_input)).set_text(cx, "");
                self.state.new_tag_input.clear();
            }
            cx.redraw_all();
        }

        // 处理新增标签输入框事件
        if let Some(text) = self.ui.text_input(id!(new_tag_input)).changed(&actions) {
            self.state.new_tag_input = text;
        }

        // 处理新增标签输入框回车
        if let Some((text, _)) = self.ui.text_input(id!(new_tag_input)).returned(&actions) {
            if !text.trim().is_empty() {
                println!("创建新标签: '{}'", text.trim());
                self.create_new_tag(text.trim().to_string());
                self.ui.view(id!(new_tag_input_container)).set_visible(cx, false);
                cx.redraw_all();
            }
        }

        // 处理标签按钮点击 - 为卡片添加标签
        if self.ui.button(id!(tag_button_1)).clicked(&actions) {
            if let Some(card_detail) = &self.state.card_detail_data {
                if self.state.all_tags.len() > 0 {
                    let tag_id = self.state.all_tags[0].id;
                    println!("点击标签按钮1，添加标签 {} 到卡片 {}", tag_id, card_detail.id);
                    self.add_tag_to_card(card_detail.id, tag_id);
                }
            }
        }
        
        if self.ui.button(id!(tag_button_2)).clicked(&actions) {
            if let Some(card_detail) = &self.state.card_detail_data {
                if self.state.all_tags.len() > 1 {
                    let tag_id = self.state.all_tags[1].id;
                    println!("点击标签按钮2，添加标签 {} 到卡片 {}", tag_id, card_detail.id);
                    self.add_tag_to_card(card_detail.id, tag_id);
                }
            }
        }
        
        if self.ui.button(id!(tag_button_3)).clicked(&actions) {
            if let Some(card_detail) = &self.state.card_detail_data {
                if self.state.all_tags.len() > 2 {
                    let tag_id = self.state.all_tags[2].id;
                    println!("点击标签按钮3，添加标签 {} 到卡片 {}", tag_id, card_detail.id);
                    self.add_tag_to_card(card_detail.id, tag_id);
                }
            }
        }

        // 处理添加待办按钮
        if self.ui.button(id!(add_todo_button)).clicked(&actions) {
            println!("点击添加待办按钮");
            let dropdown = self.ui.view(id!(todo_dropdown));
            let is_visible = dropdown.visible();
            dropdown.set_visible(cx, !is_visible);
            cx.redraw_all();
        }

        // 处理新增Todo按钮
        if self.ui.button(id!(new_todo_button)).clicked(&actions) {
            println!("点击新增Todo按钮");
            let container = self.ui.view(id!(new_todo_input_container));
            let is_visible = container.visible();
            container.set_visible(cx, !is_visible);
            
            if !is_visible {
                // 显示输入框时清空内容
                self.ui.text_input(id!(new_todo_input)).set_text(cx, "");
                self.state.new_todo_input.clear();
            }
            cx.redraw_all();
        }

        // 处理新增Todo输入框事件
        if let Some(text) = self.ui.text_input(id!(new_todo_input)).changed(&actions) {
            self.state.new_todo_input = text;
        }

        // 处理新增Todo输入框回车
        if let Some((text, _)) = self.ui.text_input(id!(new_todo_input)).returned(&actions) {
            if !text.trim().is_empty() {
                println!("创建新Todo: '{}'", text.trim());
                self.create_new_todo(text.trim().to_string());
                self.ui.view(id!(new_todo_input_container)).set_visible(cx, false);
                cx.redraw_all();
            }
        }

        // 处理Todo复选框和删除按钮点击 - 现在通过 PortalList 在 draw_walk 中处理
        // Todo数据存储在 self.state.current_todos 中

        // 处理添加Active按钮
        if self.ui.button(id!(add_active_button)).clicked(&actions) {
            println!("点击添加Active按钮");
            let dropdown = self.ui.view(id!(active_dropdown));
            let is_visible = dropdown.visible();
            dropdown.set_visible(cx, !is_visible);
            cx.redraw_all();
        }

        // 处理新增Active按钮
        if self.ui.button(id!(new_active_button)).clicked(&actions) {
            println!("点击新增Active按钮");
            let container = self.ui.view(id!(new_active_input_container));
            let is_visible = container.visible();
            container.set_visible(cx, !is_visible);
            
            if !is_visible {
                // 显示输入框时清空内容
                self.ui.text_input(id!(new_active_input)).set_text(cx, "");
                self.state.new_active_input.clear();
            }
            cx.redraw_all();
        }

        // 处理新增Active输入框事件
        if let Some(text) = self.ui.text_input(id!(new_active_input)).changed(&actions) {
            self.state.new_active_input = text;
        }

        // 处理新增Active输入框回车
        if let Some((text, _)) = self.ui.text_input(id!(new_active_input)).returned(&actions) {
            if !text.trim().is_empty() {
                println!("创建新Active: '{}'", text.trim());
                self.create_new_active(text.trim().to_string());
                self.ui.view(id!(new_active_input_container)).set_visible(cx, false);
                cx.redraw_all();
            }
        }

        // 处理Active删除按钮点击 - 现在通过 PortalList 在 draw_walk 中处理
        // Active数据存储在 self.state.current_actives 中

        // 处理创建卡片按钮点击 - 尝试直接访问
        if self.ui.button(id!(create_button)).clicked(&actions) {
            println!("App: 检测到创建卡片按钮点击");
            if !self.state.spaces_data.is_empty() {
                let space_id = self.state.spaces_data[0].id; // 暂时使用第一个空间
                self.state.new_card_inputs.insert(space_id, String::new());
                println!("App: 添加新卡片输入框到空间ID: {}", space_id);
                cx.redraw_all();
            }
        }

        // 处理测试添加卡片按钮点击
        if self.ui.button(id!(test_add_card_button)).clicked(&actions) {
            if !self.state.spaces_data.is_empty() {
                let space_id = self.state.spaces_data[0].id;
                println!("测试：添加新卡片输入框到空间 {}", space_id);
                self.state.new_card_inputs.insert(space_id, String::new());
            }
        }

        // 处理卡片相关的按钮点击
        for space in &self.state.spaces_data {
            for card in &space.cards {
                // 处理删除卡片按钮点击
                if self.ui.button(id!(delete_card_btn)).clicked(&actions) {
                    self.state.pending_delete_card_id = Some(card.id);
                    return; // 只处理第一个匹配的按钮
                }
            }
        }
    }
}

app_main!(App);
