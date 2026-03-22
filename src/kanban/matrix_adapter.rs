//! Matrix 适配层 - 将 Matrix Space/Room 映射到 Kanban 数据结构
//!
//! 简化架构设计：
//! - Matrix Space = Kanban List (看板列表)
//! - Matrix Room = Kanban Card (卡片)
//! - Space 的子 Room = List 中的 Card
//!
//! Topic 标记：
//! - `[kanban-list]` - 标识一个 Space 是看板列表

use anyhow::{Context, Result};
use makepad_widgets::{log, error};
use matrix_sdk::{
    Client,
    ruma::{
        OwnedRoomId, RoomId,
    },
    Room,
};

/// Matrix 到 Kanban 的适配器
pub struct MatrixKanbanAdapter {
    client: Client,
}

impl MatrixKanbanAdapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// 获取所有看板 Space（带有 [kanban-list] topic 标记的 Space）
    pub async fn get_all_kanban_spaces(&self) -> Result<Vec<crate::kanban::state::kanban_state::KanbanList>> {
        let mut lists = Vec::new();

        // 获取用户的所有 Room
        let all_rooms = self.client.rooms();
        
        log!("Scanning {} rooms for kanban spaces...", all_rooms.len());
        
        for room in all_rooms {
            let room_id = room.room_id();
            
            // 检查 room 是否是 Space 类型
            let is_space = room.is_space();
            
            // 检查topic中是否有[kanban-list]标记
            if let Some(topic) = room.topic() {
                log!("Room {} - is_space: {}, topic: {}", room_id, is_space, topic);
                
                if topic.contains("[kanban-list]") {
                    // 提取列表名称（去掉标记）
                    let name = topic.trim_start_matches("[kanban-list]").trim();
                    let name = if name.is_empty() {
                        room.display_name().await?.to_string()
                    } else {
                        name.to_string()
                    };
                    
                    log!("Found kanban space: {} ({})", name, room_id);
                    
                    // 获取 Space 中的所有子 Room（卡片）
                    let card_rooms = self.get_space_children(&room).await?;
                    let card_ids: Vec<OwnedRoomId> = card_rooms.iter()
                        .map(|r| r.room_id().to_owned())
                        .collect();
                    
                    let card_count = card_ids.len();
                    
                    lists.push(crate::kanban::state::kanban_state::KanbanList {
                        id: room_id.to_owned(),
                        name,
                        card_ids,
                        position: 1000.0, // TODO: 从 state event 读取
                    });
                    
                    log!("Kanban list: {} ({}) with {} cards", lists.last().unwrap().name, room_id, card_count);
                }
            }
        }
        
        log!("Found {} kanban spaces total", lists.len());
        Ok(lists)
    }

    /// 创建新的看板 Space（列表）
    pub async fn create_space(&self, name: &str) -> Result<OwnedRoomId> {
        use matrix_sdk::ruma::{
            api::client::room::create_room::v3::{Request as CreateRoomRequest, RoomPreset},
            events::room::topic::RoomTopicEventContent,
        };

        log!("🏗️ Creating Space with name: {}", name);

        // 构建创建 Space 的请求
        let mut request = CreateRoomRequest::new();
        request.name = Some(name.to_string());
        request.preset = Some(RoomPreset::PrivateChat);
        request.is_direct = false;
        
        // 关键：通过 creation_content 设置 type 为 "m.space"
        // 根据 Matrix 规范，这是创建 Space 的正确方式
        let creation_json = serde_json::json!({
            "type": "m.space"
        });
        
        log!("🏗️ Setting creation_content with type: m.space");
        
        let raw_creation = serde_json::value::to_raw_value(&creation_json)
            .context("Failed to serialize creation content")?;
        request.creation_content = Some(matrix_sdk::ruma::serde::Raw::from_json(raw_creation));
        
        log!("🏗️ Sending create_room request to server...");
        
        // 创建 Space
        let room = self.client.create_room(request).await
            .context("Failed to create kanban space")?;

        let space_id = room.room_id().to_owned();
        
        log!("✅ Room created: {}", space_id);
        
        // 等待 SDK 同步新创建的 Space
        log!("⏳ Waiting for SDK to sync the new Space...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // 验证是否成功创建为 Space
        if let Some(created_room) = self.client.get_room(&space_id) {
            let is_space = created_room.is_space();
            log!("🔍 Verification: Room {} is_space = {}", space_id, is_space);
            
            if !is_space {
                error!("⚠️⚠️⚠️ WARNING: Room was NOT created as a Space!");
                error!("⚠️ This is a known server bug on test.palpo.im");
                error!("⚠️ The server ignores the 'type' field in creation_content");
                error!("⚠️ Cards will rely on local cache for persistence");
                // 不返回错误，继续执行
            } else {
                log!("✅ Confirmed: Room is a proper Space (is_space=true)");
            }
        } else {
            error!("❌ Room {} not found in client after creation!", space_id);
            return Err(anyhow::anyhow!("Room not found in client after creation"));
        }
        
        // 设置 topic（包含 [kanban-list] 标记）
        let topic_with_marker = format!("[kanban-list] {}", name);
        
        log!("📝 Setting topic: {}", topic_with_marker);
        
        let topic_content = RoomTopicEventContent::new(topic_with_marker);
        room.send_state_event(topic_content).await
            .context("Failed to set space topic")?;

        log!("✅✅✅ Successfully created kanban space: {} ({})", name, space_id);
        Ok(space_id)
    }

    /// 创建新卡片（Matrix Room）并添加到 Space
    pub async fn create_card(
        &self,
        space_id: &RoomId,
        title: &str,
    ) -> Result<OwnedRoomId> {
        use matrix_sdk::ruma::{
            api::client::room::create_room::v3::{Request as CreateRoomRequest, RoomPreset},
        };

        log!("🎴 create_card: Creating card '{}' in space {}", title, space_id);

        // 构建创建 Room 的请求
        let mut request = CreateRoomRequest::new();
        request.name = Some(title.to_string());
        request.preset = Some(RoomPreset::PrivateChat);
        request.is_direct = false;

        log!("🎴 Sending create_room request to Matrix server...");
        
        // 创建 Room
        let room = match self.client.create_room(request).await {
            Ok(room) => {
                log!("✓ create_room API call succeeded");
                room
            }
            Err(e) => {
                error!("❌ create_room API call failed: {:?}", e);
                return Err(e.into());
            }
        };

        let card_room_id = room.room_id().to_owned();
        
        log!("✓✓ Room created successfully: {}", card_room_id);

        // 计算新卡片的 position
        // 获取 Space 中现有卡片的最大 position，新卡片排在最后
        let position = match self.client.get_room(space_id) {
            Some(space_room) => {
                match self.get_card_list_from_state(&space_room).await {
                    Ok(card_ids) => {
                        let mut max_position = 0.0;
                        for card_id in card_ids {
                            if let Ok(card) = self.load_card(&card_id, space_id.to_owned()).await {
                                if card.position > max_position {
                                    max_position = card.position;
                                }
                            }
                        }
                        max_position + 1000.0
                    }
                    Err(e) => {
                        log!("⚠️ Failed to get existing cards, using default position: {:?}", e);
                        1000.0
                    }
                }
            }
            None => {
                log!("⚠️ Space not found, using default position");
                1000.0
            }
        };

        log!("📍 Calculated position for new card: {}", position);

        // 创建初始的 Card 元数据，使用计算出的 position
        let mut card = crate::kanban::state::kanban_state::KanbanCard::new(
            card_room_id.clone(),
            title.to_string(),
            space_id.to_owned(),
        );
        card.position = position;
        
        // 保存元数据到 Matrix State
        log!("💾 Saving initial card metadata...");
        if let Err(e) = self.save_card_metadata(&card).await {
            error!("⚠️ Failed to save card metadata: {:?}", e);
            // 不返回错误，继续执行
        }

        // 将卡片 Room 添加到 Space
        log!("🎴 Now adding card to space...");
        match self.add_card_to_space(space_id, &card_room_id).await {
            Ok(_) => {
                log!("✓✓✓ Successfully added card to space");
            }
            Err(e) => {
                error!("❌ Failed to add card to space: {:?}", e);
                return Err(e);
            }
        }

        log!("✓✓✓✓ Created kanban card: {} in space {} ({}) with position {}", title, space_id, card_room_id, position);
        Ok(card_room_id)
    }

    /// 从 Matrix Room 加载卡片（简化版）
    pub async fn load_card(&self, room_id: &RoomId, space_id: OwnedRoomId) -> Result<crate::kanban::state::kanban_state::KanbanCard> {
        let room = self.client
            .get_room(room_id)
            .context("Room not found")?;

        // 尝试从 State Event 加载完整元数据
        match self.load_card_metadata(&room).await {
            Ok(metadata) => {
                // 加载 TodoList
                let todos = self.load_card_todos(&room).await.unwrap_or_default();
                
                Ok(crate::kanban::state::kanban_state::KanbanCard {
                    id: room_id.to_owned(),
                    title: metadata.title,
                    description: metadata.description,
                    space_id,
                    position: metadata.position,
                    status: metadata.status,
                    tags: metadata.tags,
                    end_time: metadata.end_time,
                    todos,
                    created_at: metadata.created_at,
                    updated_at: metadata.updated_at,
                })
            }
            Err(_) => {
                // 如果没有元数据，使用默认值
                let display_name = room.display_name().await?;
                let title = display_name.to_string();
                
                Ok(crate::kanban::state::kanban_state::KanbanCard::new(
                    room_id.to_owned(),
                    title,
                    space_id,
                ))
            }
        }
    }

    /// 保存 Card 元数据到 Matrix Room State
    pub async fn save_card_metadata(&self, card: &crate::kanban::state::kanban_state::KanbanCard) -> Result<()> {
        use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
        
        log!("💾 [1/3] save_card_metadata called for {}", card.id);
        
        let room = self.client.get_room(&card.id)
            .context("Card room not found")?;
        
        // Create metadata JSON
        let metadata = serde_json::json!({
            "title": card.title,
            "description": card.description,
            "position": card.position,
            "status": card.status,
            "end_time": card.end_time,
            "tags": card.tags,
            "created_at": card.created_at,
            "updated_at": card.updated_at,
        });
        
        log!("💾 [2/3] Saving metadata as timeline message for {} - title: {}, status: {:?}, tags: {:?}, end_time: {:?}", 
            card.id, card.title, card.status, card.tags, card.end_time);
        
        // Send as a special timeline message with custom msgtype
        let metadata_json = serde_json::to_string(&metadata)
            .context("Failed to serialize metadata")?;
        
        // Use a custom message type that won't be displayed in chat UI
        let content = RoomMessageEventContent::text_plain(
            format!("__KANBAN_METADATA__:{}", metadata_json)
        );
        
        room.send(content).await?;
        
        log!("✅ [3/3] Saved card metadata successfully via timeline message");
        Ok(())
    }
    
    /// 保存 TodoList 到 Matrix Room State
    pub async fn save_card_todos(
        &self,
        card_id: &RoomId,
        todos: &[crate::kanban::state::kanban_state::TodoItem]
    ) -> Result<()> {
        let room = self.client.get_room(card_id)
            .context("Card room not found")?;
        
        let todos_content = serde_json::json!({
            "todos": todos,
        });
        
        log!("💾 Saving {} todos for card {}", todos.len(), card_id);
        log!("💾 Todos content: {:?}", todos_content);
        
        let response = room.send_state_event_raw(
            "m.kanban.card.todos",
            "",
            serde_json::value::to_raw_value(&todos_content)
                .context("Failed to serialize todos")?,
        ).await?;
        
        log!("✓ Saved todos successfully, event_id: {:?}", response.event_id);
        Ok(())
    }
    
    /// 从 Room Messages 加载元数据
    /// 使用 Matrix /messages API 直接读取最近的消息
    async fn load_card_metadata(&self, room: &Room) -> Result<CardMetadataRaw> {
        use matrix_sdk::ruma::api::client::message::get_message_events;
        use matrix_sdk::ruma::events::{AnySyncTimelineEvent, AnySyncMessageLikeEvent};
        use matrix_sdk::ruma::events::room::message::{SyncRoomMessageEvent, MessageType};
        
        let room_id = room.room_id();
        
        log!("📖 Loading card metadata from room messages {}", room_id);
        
        // 使用 Matrix /messages API 获取最近的消息
        let mut request = get_message_events::v3::Request::backward(room_id.to_owned());
        request.limit = 50.try_into().unwrap(); // 检查最近 50 条消息
        
        match self.client.send(request).await {
            Ok(response) => {
                log!("📖 Got {} messages from room", response.chunk.len());
                
                // 遍历消息查找 metadata（从最新到最旧，取第一个找到的）
                // Matrix /messages API 返回的消息是按时间倒序排列的（最新的在前）
                let mut found_metadata: Option<CardMetadataRaw> = None;
                
                for raw_event in response.chunk {
                    // 尝试反序列化为同步消息事件
                    if let Ok(event) = raw_event.deserialize_as::<AnySyncTimelineEvent>() {
                        if let AnySyncTimelineEvent::MessageLike(msg_event) = event {
                            if let AnySyncMessageLikeEvent::RoomMessage(room_msg) = msg_event {
                                if let SyncRoomMessageEvent::Original(original) = room_msg {
                                    if let MessageType::Text(text) = &original.content.msgtype {
                                        let body = &text.body;
                                        
                                        // 检查是否是 metadata 消息
                                        if let Some(json_str) = body.strip_prefix("__KANBAN_METADATA__:") {
                                            log!("📖 Found metadata message, parsing...");
                                            match serde_json::from_str::<CardMetadataRaw>(json_str) {
                                                Ok(metadata) => {
                                                    log!("✅ Loaded card metadata: title={}, tags={:?}, end_time={:?}", 
                                                        metadata.title, metadata.tags, metadata.end_time);
                                                    // 找到第一个（最新的）metadata 就返回
                                                    found_metadata = Some(metadata);
                                                    break;
                                                }
                                                Err(e) => {
                                                    error!("❌ Failed to parse metadata: {:?}", e);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if let Some(metadata) = found_metadata {
                    Ok(metadata)
                } else {
                    log!("⚠ No metadata message found in recent messages");
                    Err(anyhow::anyhow!("No card metadata found"))
                }
            }
            Err(e) => {
                error!("❌ Error loading messages: {:?}", e);
                Err(anyhow::anyhow!("Failed to load messages: {}", e))
            }
        }
    }
    
    /// 从 State Event 加载 TodoList
    async fn load_card_todos(&self, room: &Room) -> Result<Vec<crate::kanban::state::kanban_state::TodoItem>> {
        use matrix_sdk::ruma::api::client::state::get_state_events;
        
        let room_id = room.room_id();
        log!("📖 Loading todos from room {} (using server API)", room_id);
        
        // 使用服务器 API 直接获取所有 State Events
        let request = get_state_events::v3::Request::new(room_id.to_owned());
        
        match self.client.send(request).await {
            Ok(response) => {
                log!("📖 Got {} state events from server", response.room_state.len());
                
                // 查找 m.kanban.card.todos 事件
                for raw_event in response.room_state {
                    let json_str = raw_event.json().get();
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                        // 检查 event type
                        if let Some(event_type_str) = json.get("type").and_then(|v| v.as_str()) {
                            if event_type_str == "m.kanban.card.todos" {
                                log!("📖 Found m.kanban.card.todos event");
                                if let Some(content) = json.get("content") {
                                    log!("📖 State event content: {:?}", content);
                                    if let Some(todos_array) = content.get("todos").and_then(|v| v.as_array()) {
                                        log!("📖 Todos array length: {}", todos_array.len());
                                        let todos: Vec<crate::kanban::state::kanban_state::TodoItem> = todos_array
                                            .iter()
                                            .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                            .collect();
                                        log!("✅ Loaded {} todos successfully from server", todos.len());
                                        return Ok(todos);
                                    }
                                }
                            }
                        }
                    }
                }
                
                log!("📖 No m.kanban.card.todos event found in room state");
                Ok(Vec::new())
            }
            Err(e) => {
                error!("❌ Error loading state events from server: {:?}", e);
                Ok(Vec::new())
            }
        }
    }

    /// 将卡片添加到 Space（设置 Space 子关系）
    /// 
    /// 根据 Matrix Spaces 规范，需要设置双向关系：
    /// 1. 在 Space 中设置 m.space.child 事件（state_key = child_room_id）
    /// 2. 在 Child Room 中设置 m.space.parent 事件（state_key = parent_space_id）
    async fn add_card_to_space(&self, space_id: &RoomId, card_room_id: &RoomId) -> Result<()> {
        log!("🔗 add_card_to_space: Adding card {} to space {}", card_room_id, space_id);
        
        let space = self.client
            .get_room(space_id)
            .context("Space not found")?;
        
        let card_room = self.client
            .get_room(card_room_id)
            .context("Card room not found")?;

        log!("🔗 Found space and card room objects");
        
        // 验证这是一个真正的Space
        if !space.is_space() {
            error!("⚠️⚠️⚠️ WARNING: Room {} is NOT a Space (is_space=false)!", space_id);
            error!("⚠️ This is a known server bug on test.palpo.im");
            error!("⚠️ Continuing anyway to set parent-child relationship...");
        } else {
            log!("✓ Verified: Room {} is a proper Space (is_space=true)", space_id);
        }

        // ============================================================
        // 关键修复：设置双向 Space-Room 关系（符合 Matrix 规范）
        // ============================================================
        
        // 步骤 1: 在 Space 中设置 m.space.child 事件
        // state_key = child_room_id，表示这个 room 是 space 的子节点
        use matrix_sdk::ruma::events::space::child::SpaceChildEventContent;
        let child_content = SpaceChildEventContent::new(vec![]);
        
        log!("🔗 Step 1: Sending m.space.child event in space {} with state_key={}", space_id, card_room_id);
        match space.send_state_event_raw(
            "m.space.child",
            card_room_id.as_str(),  // state_key = child room id
            serde_json::value::to_raw_value(&child_content)
                .context("Failed to serialize space child content")?,
        ).await {
            Ok(response) => {
                log!("✓ Sent m.space.child event successfully, event_id: {:?}", response.event_id);
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            }
            Err(e) => {
                error!("❌ Failed to send m.space.child event: {:?}", e);
                error!("⚠️ This may prevent proper cross-device sync!");
            }
        }
        
        // 步骤 2: 在 Child Room 中设置 m.space.parent 事件
        // state_key = parent_space_id，表示这个 space 是 room 的父节点
        use matrix_sdk::ruma::events::space::parent::SpaceParentEventContent;
        let parent_content = SpaceParentEventContent::new(vec![]);
        
        log!("🔗 Step 2: Sending m.space.parent event in room {} with state_key={}", card_room_id, space_id);
        match card_room.send_state_event_raw(
            "m.space.parent",
            space_id.as_str(),  // state_key = parent space id
            serde_json::value::to_raw_value(&parent_content)
                .context("Failed to serialize space parent content")?,
        ).await {
            Ok(response) => {
                log!("✓ Sent m.space.parent event successfully, event_id: {:?}", response.event_id);
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            }
            Err(e) => {
                error!("❌ Failed to send m.space.parent event: {:?}", e);
                error!("⚠️ This may prevent proper cross-device sync!");
            }
        }
        
        log!("✓✓ Bidirectional Space-Room relationship established!");
        log!("   Space {} -> m.space.child[{}]", space_id, card_room_id);
        log!("   Room {} -> m.space.parent[{}]", card_room_id, space_id);

        // 策略 3（备用）: 使用自定义 m.kanban.cards 事件存储所有卡片 ID
        // 这是为了应对服务器不支持标准 Space 功能的情况
        log!("🔗 Backup: Reading existing card list from m.kanban.cards...");
        match self.get_card_list_from_state(&space).await {
            Ok(mut card_ids) => {
                log!("🔗 Found {} existing cards in state", card_ids.len());
                if !card_ids.contains(&card_room_id.to_owned()) {
                    card_ids.push(card_room_id.to_owned());
                    
                    let cards_content = serde_json::json!({
                        "card_ids": card_ids.iter().map(|id| id.as_str()).collect::<Vec<_>>()
                    });
                    
                    log!("🔗 Sending m.kanban.cards event with {} cards...", card_ids.len());
                    log!("🔗 Event content: {:?}", cards_content);
                    
                    match space.send_state_event_raw(
                        "m.kanban.cards",
                        "",
                        serde_json::value::to_raw_value(&cards_content)
                            .context("Failed to serialize kanban cards content")?,
                    ).await {
                        Ok(response) => {
                            log!("✓ m.kanban.cards backup event sent, event_id: {:?}", response.event_id);
                            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                        }
                        Err(e) => {
                            log!("⚠️ Failed to send m.kanban.cards backup event: {:?}", e);
                        }
                    }
                    
                    log!("✓ Added card {} to backup list (total: {} cards)", card_room_id, card_ids.len());
                } else {
                    log!("Card {} already in space {}", card_room_id, space_id);
                }
            }
            Err(e) => {
                log!("⚠ Warning: Failed to get existing card list: {:?}, creating new list", e);
                // 如果获取失败，创建新列表
                let cards_content = serde_json::json!({
                    "card_ids": vec![card_room_id.as_str()]
                });
                
                log!("🔗 Sending new m.kanban.cards event...");
                match space.send_state_event_raw(
                    "m.kanban.cards",
                    "",
                    serde_json::value::to_raw_value(&cards_content)
                        .context("Failed to serialize kanban cards content")?,
                ).await {
                    Ok(_) => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                        log!("✓ Created new backup card list in space {}", space_id);
                    }
                    Err(e) => {
                        log!("⚠️ Failed to send m.kanban.cards backup event: {:?}", e);
                    }
                }
            }
        }

        // 策略 4（最后备用）: 保存到本地缓存
        // 注意：这只是临时方案，不应该依赖它来实现跨设备同步
        log!("💾 Saving to local cache as final backup...");
        crate::kanban::local_cache::add_card_to_space_cache(space_id.to_owned(), card_room_id.to_owned());
        log!("✓ Saved to local cache");

        log!("🔗✓✓✓ add_card_to_space completed successfully");
        log!("   Primary: Bidirectional Matrix Space relationship set");
        log!("   Backup: Custom m.kanban.cards event + local cache");
        Ok(())
    }
    
    /// 从 m.kanban.cards 状态事件读取卡片列表
    async fn get_card_list_from_state(&self, space: &Room) -> Result<Vec<OwnedRoomId>> {
        use matrix_sdk::ruma::events::StateEventType;
        
        let space_id = space.room_id();
        log!("📖 get_card_list_from_state: Reading from space {}", space_id);
        
        // 使用自定义状态事件类型
        let event_type = StateEventType::from("m.kanban.cards");
        
        log!("📖 Calling space.get_state_event(m.kanban.cards, \"\")...");
        match space.get_state_event(event_type, "").await {
            Ok(Some(raw_event)) => {
                log!("📖 Found m.kanban.cards state event, parsing...");
                log!("📖 Raw event: {:?}", raw_event);
                // 将 RawAnySyncOrStrippedState 序列化为 JSON 字符串
                if let Ok(json_str) = serde_json::to_string(&raw_event) {
                    log!("📖 Event JSON string: {}", json_str);
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        log!("📖 Parsed JSON: {}", json);
                        // 检查 content.card_ids
                        if let Some(content) = json.get("content") {
                            if let Some(card_ids_array) = content.get("card_ids").and_then(|v| v.as_array()) {
                                let card_ids: Vec<OwnedRoomId> = card_ids_array
                                    .iter()
                                    .filter_map(|v| v.as_str())
                                    .filter_map(|s| OwnedRoomId::try_from(s).ok())
                                    .collect();
                                log!("✓ Parsed {} card IDs from m.kanban.cards state", card_ids.len());
                                for (i, id) in card_ids.iter().enumerate() {
                                    log!("  Card {}: {}", i+1, id);
                                }
                                return Ok(card_ids);
                            } else {
                                log!("⚠ content.card_ids not found or not an array");
                            }
                        } else {
                            log!("⚠ content field not found in JSON");
                        }
                    } else {
                        log!("⚠ Failed to parse JSON from string");
                    }
                } else {
                    log!("⚠ Failed to serialize raw_event to JSON string");
                }
                log!("⚠ Failed to parse m.kanban.cards state event");
                Ok(Vec::new())
            }
            Ok(None) => {
                log!("📖 No m.kanban.cards state event found in space {}", space_id);
                Ok(Vec::new())
            }
            Err(e) => {
                log!("❌ Error getting m.kanban.cards state: {:?}", e);
                Ok(Vec::new())
            }
        }
    }

    /// 获取 Space 的所有子 Room
    /// 
    /// 优先级策略：
    /// 1. 从标准 m.space.child 事件读取（符合 Matrix 规范，支持跨设备同步）
    /// 2. 从自定义 m.kanban.cards 事件读取（备用方案）
    /// 3. 从本地缓存读取（最后备用）
    async fn get_space_children(&self, space: &Room) -> Result<Vec<Room>> {
        let space_id = space.room_id();

        log!("👶 get_space_children: Getting children for space: {}", space_id);

        // 策略 1（推荐）: 从标准 m.space.child 事件读取
        // 这是 Matrix 规范的标准方式，支持跨设备同步
        log!("📖 Strategy 1: Reading from m.space.child state events...");
        let mut children_from_space_child = Vec::new();
        
        // 获取所有已加入的房间
        let all_rooms = self.client.rooms();
        log!("   Checking {} total rooms for m.space.child relationship...", all_rooms.len());
        
        for room in &all_rooms {
            let room_id = room.room_id();
            
            // 检查 Space 中是否有 m.space.child[room_id] 事件
            match space.get_state_event(
                matrix_sdk::ruma::events::StateEventType::SpaceChild,
                room_id.as_str()
            ).await {
                Ok(Some(_event)) => {
                    log!("   ✓ Found m.space.child[{}] in space {}", room_id, space_id);
                    children_from_space_child.push(room.clone());
                }
                Ok(None) => {
                    // 没有找到，这是正常的
                }
                Err(e) => {
                    log!("   ⚠ Error checking m.space.child for {}: {:?}", room_id, e);
                }
            }
        }
        
        if !children_from_space_child.is_empty() {
            log!("✓✓ Found {} children from m.space.child events (Matrix standard)", children_from_space_child.len());
            return Ok(children_from_space_child);
        } else {
            log!("📖 No children found via m.space.child, trying backup strategies...");
        }

        // 策略 2（备用）: 从自定义 m.kanban.cards 事件读取
        log!("📖 Strategy 2: Reading from m.kanban.cards backup event...");
        match self.get_card_list_from_state(space).await {
            Ok(card_ids) if !card_ids.is_empty() => {
                log!("✓ Found {} cards from m.kanban.cards backup", card_ids.len());
                let mut children = Vec::new();
                for card_id in &card_ids {
                    if let Some(room) = self.client.get_room(card_id) {
                        log!("  ✓ Found room object for card: {}", card_id);
                        children.push(room);
                    } else {
                        log!("  ⚠ WARNING: Card room {} not found in client!", card_id);
                    }
                }
                if !children.is_empty() {
                    log!("✓✓ Loaded {} child rooms from m.kanban.cards backup", children.len());
                    return Ok(children);
                }
            }
            Ok(_) => {
                log!("📖 No cards found in m.kanban.cards backup");
            }
            Err(e) => {
                log!("❌ Failed to read m.kanban.cards backup: {:?}", e);
            }
        }

        // 策略 3（最后备用）: 从本地缓存读取
        log!("📖 Strategy 3: Reading from local cache (last resort)...");
        let cached_card_ids = crate::kanban::local_cache::get_cards_from_cache(&space_id.to_owned());
        if !cached_card_ids.is_empty() {
            log!("✓ Found {} cards from local cache", cached_card_ids.len());
            let mut children = Vec::new();
            for card_id in &cached_card_ids {
                if let Some(room) = self.client.get_room(card_id) {
                    children.push(room);
                }
            }
            if !children.is_empty() {
                log!("✓✓ Loaded {} child rooms from local cache", children.len());
                return Ok(children);
            }
        }

        log!("⚠️ No children found for space {} using any strategy", space_id);
        Ok(Vec::new())
    }
    
    // ========== Phase 5: Activities Methods ==========
    
    /// 发送活动记录（Timeline Event）
    pub async fn send_activity(
        &self,
        card_id: &RoomId,
        activity_type: crate::kanban::state::kanban_state::ActivityType,
        text: String,
        metadata: Option<serde_json::Value>,
    ) -> Result<()> {
        let room = self.client.get_room(card_id)
            .context("Card room not found")?;
        
        log!("💬 Sending activity to card {}: type={:?}, text={}", card_id, activity_type, text);
        
        use crate::kanban::state::kanban_state::ActivityType;
        
        // 评论使用标准的 m.text 消息
        if matches!(activity_type, ActivityType::Comment) {
            use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
            let content = RoomMessageEventContent::text_plain(&text);
            room.send(content).await
                .context("Failed to send comment")?;
            log!("✓ Comment sent successfully");
            return Ok(());
        }
        
        // 系统活动使用自定义消息类型
        let activity_type_str = match activity_type {
            ActivityType::StatusChange => "status_change",
            ActivityType::TagAdded => "tag_added",
            ActivityType::TagRemoved => "tag_removed",
            ActivityType::TodoAdded => "todo_added",
            ActivityType::TodoCompleted => "todo_completed",
            ActivityType::TodoUncompleted => "todo_uncompleted",
            ActivityType::EndTimeSet => "end_time_set",
            ActivityType::EndTimeRemoved => "end_time_removed",
            ActivityType::DescriptionChanged => "description_changed",
            ActivityType::TitleChanged => "title_changed",
            ActivityType::Comment => unreachable!(),
        };
        
        let content = serde_json::json!({
            "msgtype": "m.kanban.card.activity",
            "body": text,
            "activity_type": activity_type_str,
            "metadata": metadata,
        });
        
        let raw_content = serde_json::value::to_raw_value(&content)
            .context("Failed to serialize activity content")?;
        
        room.send_raw("m.room.message", raw_content).await
            .context("Failed to send activity")?;
        
        log!("✓ System activity sent successfully");
        Ok(())
    }
    
    /// 加载活动记录（从Timeline Events）
    /// 
    /// 加载卡片的活动记录
    /// 从 Timeline 中加载评论和系统活动
    pub async fn load_activities(
        &self,
        card_id: &RoomId,
        limit: Option<usize>,
    ) -> Result<Vec<crate::kanban::state::kanban_state::CardActivity>> {
        let _room = self.client.get_room(card_id)
            .context("Card room not found")?;
        
        log!("📖 Loading activities from card {}", card_id);
        
        // 获取房间消息（使用 messages API）
        let mut request = matrix_sdk::ruma::api::client::message::get_message_events::v3::Request::new(
            card_id.to_owned(),
            matrix_sdk::ruma::api::Direction::Backward,
        );
        request.limit = limit.unwrap_or(50).try_into().unwrap();
        
        match self.client.send(request).await {
            Ok(response) => {
                log!("📖 Got {} messages from timeline", response.chunk.len());
                
                let mut activities = Vec::new();
                
                for raw_event in response.chunk {
                    // 反序列化事件
                    if let Ok(event) = raw_event.deserialize() {
                        if let Some(activity) = self.parse_activity_from_raw_event(&event) {
                            activities.push(activity);
                        }
                    }
                }
                
                // 按时间倒序排列（最新的在前）
                activities.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                
                log!("✅ Loaded {} activities from card", activities.len());
                Ok(activities)
            }
            Err(e) => {
                error!("❌ Failed to load activities: {}", e);
                Ok(Vec::new())
            }
        }
    }
    
    /// 从原始事件解析活动记录
    fn parse_activity_from_raw_event(
        &self,
        event: &matrix_sdk::ruma::events::AnyTimelineEvent,
    ) -> Option<crate::kanban::state::kanban_state::CardActivity> {
        use crate::kanban::state::kanban_state::{CardActivity, ActivityType};
        use matrix_sdk::ruma::events::AnyTimelineEvent;
        
        match event {
            AnyTimelineEvent::MessageLike(msg_event) => {
                use matrix_sdk::ruma::events::AnyMessageLikeEvent;
                
                match msg_event {
                    AnyMessageLikeEvent::RoomMessage(room_msg) => {
                        let original = room_msg.as_original()?;
                        let event_id = original.event_id.to_string();
                        let sender = original.sender.to_string();
                        let created_at = original.origin_server_ts.as_secs().into();
                        
                        // 获取消息内容
                        let content = &original.content;
                        
                        use matrix_sdk::ruma::events::room::message::MessageType;
                        match &content.msgtype {
                            MessageType::Text(text_content) => {
                                let body = text_content.body.clone();
                                
                                // 跳过卡片元数据消息
                                if body.starts_with("Card metadata:") || body.starts_with("Card Metadata:") {
                                    return None;
                                }
                                
                                // 普通评论
                                return Some(CardActivity {
                                    id: event_id,
                                    activity_type: ActivityType::Comment,
                                    text: body,
                                    metadata: None,
                                    created_at,
                                    user_id: sender,
                                });
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        
        None
    }
    
    // ========== Space 标签库管理 ==========
    
    /// 加载 Space 标签库
    pub async fn load_space_tags(&self, space_id: &RoomId) -> Result<Vec<crate::kanban::state::kanban_state::SpaceTag>> {
        use matrix_sdk::ruma::api::client::state::get_state_events;
        
        log!("📚 Loading tag library from space {}", space_id);
        
        // 使用服务器 API 直接获取所有 State Events
        let request = get_state_events::v3::Request::new(space_id.to_owned());
        
        match self.client.send(request).await {
            Ok(response) => {
                log!("📖 Got {} state events from server", response.room_state.len());
                
                // 查找 m.space.tag_library 事件
                for raw_event in response.room_state {
                    // 先转换为 JSON 查看事件类型
                    if let Ok(json_value) = serde_json::to_value(&raw_event) {
                        if let Some(event_type) = json_value.get("type").and_then(|v| v.as_str()) {
                            if event_type == "m.space.tag_library" {
                                log!("✅ Found m.space.tag_library event");
                                
                                // 解析事件内容
                                if let Some(content) = json_value.get("content") {
                                    if let Some(tags_value) = content.get("tags") {
                                        match serde_json::from_value::<Vec<crate::kanban::state::kanban_state::SpaceTag>>(tags_value.clone()) {
                                            Ok(tags) => {
                                                log!("✅ Successfully loaded {} tags from space", tags.len());
                                                for tag in &tags {
                                                    log!("  - Tag: id={}, name={}, color={}", tag.id, tag.name, tag.color);
                                                }
                                                return Ok(tags);
                                            }
                                            Err(e) => {
                                                error!("❌ Failed to parse tags: {}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                log!("⚠️ No m.space.tag_library event found in room state");
                Ok(Vec::new())
            }
            Err(e) => {
                error!("❌ Failed to load state events: {}", e);
                Ok(Vec::new())
            }
        }
    }
    
    /// 保存 Space 标签库
    pub async fn save_space_tags(&self, space_id: &RoomId, tags: Vec<crate::kanban::state::kanban_state::SpaceTag>) -> Result<()> {
        let space = self.client.get_room(space_id)
            .context("Space not found")?;
        
        log!("💾 Saving {} tags to space {}", tags.len(), space_id);
        
        // 构建标签库内容
        let content = serde_json::json!({
            "tags": tags,
            "version": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        
        // 使用 send_state_event_raw 发送自定义状态事件
        let event_type = "m.space.tag_library";
        let state_key = "";
        
        let raw_content = serde_json::value::to_raw_value(&content)
            .context("Failed to serialize tag library content")?;
        
        space.send_state_event_raw(event_type, state_key, raw_content).await
            .context("Failed to save tag library")?;
        
        log!("✅ Tag library saved successfully to Matrix");
        Ok(())
    }
    
    /// 添加标签到 Space
    pub async fn add_space_tag(&self, space_id: &RoomId, tag: crate::kanban::state::kanban_state::SpaceTag) -> Result<()> {
        log!("➕ Adding tag '{}' to space {}", tag.name, space_id);
        
        let mut tags = self.load_space_tags(space_id).await?;
        
        // 检查是否已存在同名标签
        if tags.iter().any(|t| t.name == tag.name) {
            return Err(anyhow::anyhow!("Tag with name '{}' already exists", tag.name));
        }
        
        tags.push(tag);
        self.save_space_tags(space_id, tags).await?;
        
        log!("✓ Tag added successfully");
        Ok(())
    }
    
    /// 更新 Space 标签
    pub async fn update_space_tag(&self, space_id: &RoomId, tag: crate::kanban::state::kanban_state::SpaceTag) -> Result<()> {
        log!("✏️ Updating tag '{}' in space {}", tag.id, space_id);
        
        let mut tags = self.load_space_tags(space_id).await?;
        
        if let Some(existing) = tags.iter_mut().find(|t| t.id == tag.id) {
            *existing = tag;
            self.save_space_tags(space_id, tags).await?;
            log!("✓ Tag updated successfully");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Tag not found: {}", tag.id))
        }
    }
    
    /// 删除 Space 标签
    pub async fn delete_space_tag(&self, space_id: &RoomId, tag_id: &str) -> Result<()> {
        log!("🗑️ Deleting tag '{}' from space {}", tag_id, space_id);
        
        let mut tags = self.load_space_tags(space_id).await?;
        let original_len = tags.len();
        
        tags.retain(|t| t.id != tag_id);
        
        if tags.len() == original_len {
            return Err(anyhow::anyhow!("Tag not found: {}", tag_id));
        }
        
        self.save_space_tags(space_id, tags).await?;
        
        log!("✓ Tag deleted successfully");
        Ok(())
    }
    
    /// 迁移旧格式标签（标签名称 -> 标签 ID）
    pub async fn migrate_card_tags(&self, _card: &mut crate::kanban::state::kanban_state::KanbanCard, _space_id: &RoomId) -> Result<()> {
        // 简化方案：直接使用标签名称，不需要迁移
        Ok(())
    }
}


/// 原始元数据结构（用于反序列化）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CardMetadataRaw {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_position")]
    pub position: f64,
    #[serde(default)]
    pub end_time: Option<u64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub status: crate::kanban::state::kanban_state::CardStatus,
    #[serde(default = "default_timestamp")]
    pub created_at: u64,
    #[serde(default = "default_timestamp")]
    pub updated_at: u64,
}

fn default_position() -> f64 {
    1000.0
}

fn default_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl MatrixKanbanAdapter {
    // ========== Phase 6: Drag and Drop Methods ==========
    
    /// 移动卡片到不同的 Space
    /// 
    /// 这个方法会：
    /// 1. 从旧 Space 移除卡片的父子关系
    /// 2. 在新 Space 建立卡片的父子关系
    /// 3. 更新卡片的元数据（space_id 和 position）
    pub async fn move_card(
        &self,
        card_id: &RoomId,
        source_space_id: &RoomId,
        target_space_id: &RoomId,
        card: &crate::kanban::state::kanban_state::KanbanCard,
    ) -> Result<()> {
        log!("🚚 move_card: Moving card {} from space {} to space {}", 
            card_id, source_space_id, target_space_id);
        
        // 如果源和目标是同一个 Space，只更新 position
        if source_space_id == target_space_id {
            log!("🚚 Same space, only updating position to {}", card.position);
            return self.save_card_metadata(card).await;
        }
        
        let card_room = self.client.get_room(card_id)
            .context("Card room not found")?;
        let source_space = self.client.get_room(source_space_id)
            .context("Source space not found")?;
        let target_space = self.client.get_room(target_space_id)
            .context("Target space not found")?;
        
        // ============================================================
        // 步骤 1: 从旧 Space 移除卡片
        // ============================================================
        
        log!("🚚 Step 1: Removing card from source space {}", source_space_id);
        
        // 1.1 删除 source space 的 m.space.child 事件
        match source_space.send_state_event_raw(
            "m.space.child",
            card_id.as_str(),
            serde_json::value::to_raw_value(&serde_json::json!({}))
                .context("Failed to serialize empty content")?,
        ).await {
            Ok(_) => {
                log!("✓ Removed m.space.child event from source space");
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
            Err(e) => {
                log!("⚠️ Failed to remove m.space.child from source space: {:?}", e);
            }
        }
        
        // 1.2 删除 card room 的 m.space.parent 事件（指向旧 space）
        match card_room.send_state_event_raw(
            "m.space.parent",
            source_space_id.as_str(),
            serde_json::value::to_raw_value(&serde_json::json!({}))
                .context("Failed to serialize empty content")?,
        ).await {
            Ok(_) => {
                log!("✓ Removed m.space.parent event from card room");
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
            Err(e) => {
                log!("⚠️ Failed to remove m.space.parent from card room: {:?}", e);
            }
        }
        
        // 1.3 更新 source space 的 m.kanban.cards 备用列表
        log!("🚚 Updating source space backup card list...");
        match self.get_card_list_from_state(&source_space).await {
            Ok(mut card_ids) => {
                card_ids.retain(|id| id != card_id);
                
                let cards_content = serde_json::json!({
                    "card_ids": card_ids.iter().map(|id| id.as_str()).collect::<Vec<_>>()
                });
                
                match source_space.send_state_event_raw(
                    "m.kanban.cards",
                    "",
                    serde_json::value::to_raw_value(&cards_content)
                        .context("Failed to serialize cards content")?,
                ).await {
                    Ok(_) => {
                        log!("✓ Updated source space backup card list");
                        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    }
                    Err(e) => {
                        log!("⚠️ Failed to update source space backup list: {:?}", e);
                    }
                }
            }
            Err(e) => {
                log!("⚠️ Failed to get source space card list: {:?}", e);
            }
        }
        
        // ============================================================
        // 步骤 2: 添加卡片到新 Space
        // ============================================================
        
        log!("🚚 Step 2: Adding card to target space {}", target_space_id);
        
        // 2.1 在 target space 创建 m.space.child 事件
        use matrix_sdk::ruma::events::space::child::SpaceChildEventContent;
        let child_content = SpaceChildEventContent::new(vec![]);
        
        match target_space.send_state_event_raw(
            "m.space.child",
            card_id.as_str(),
            serde_json::value::to_raw_value(&child_content)
                .context("Failed to serialize space child content")?,
        ).await {
            Ok(_) => {
                log!("✓ Created m.space.child event in target space");
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
            Err(e) => {
                error!("❌ Failed to create m.space.child in target space: {:?}", e);
                return Err(anyhow::anyhow!("Failed to add card to target space: {}", e));
            }
        }
        
        // 2.2 在 card room 创建 m.space.parent 事件（指向新 space）
        use matrix_sdk::ruma::events::space::parent::SpaceParentEventContent;
        let parent_content = SpaceParentEventContent::new(vec![]);
        
        match card_room.send_state_event_raw(
            "m.space.parent",
            target_space_id.as_str(),
            serde_json::value::to_raw_value(&parent_content)
                .context("Failed to serialize space parent content")?,
        ).await {
            Ok(_) => {
                log!("✓ Created m.space.parent event in card room");
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
            Err(e) => {
                error!("❌ Failed to create m.space.parent in card room: {:?}", e);
                return Err(anyhow::anyhow!("Failed to set card parent: {}", e));
            }
        }
        
        // 2.3 更新 target space 的 m.kanban.cards 备用列表
        log!("🚚 Updating target space backup card list...");
        match self.get_card_list_from_state(&target_space).await {
            Ok(mut card_ids) => {
                if !card_ids.contains(&card_id.to_owned()) {
                    card_ids.push(card_id.to_owned());
                    
                    let cards_content = serde_json::json!({
                        "card_ids": card_ids.iter().map(|id| id.as_str()).collect::<Vec<_>>()
                    });
                    
                    match target_space.send_state_event_raw(
                        "m.kanban.cards",
                        "",
                        serde_json::value::to_raw_value(&cards_content)
                            .context("Failed to serialize cards content")?,
                    ).await {
                        Ok(_) => {
                            log!("✓ Updated target space backup card list");
                            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                        }
                        Err(e) => {
                            log!("⚠️ Failed to update target space backup list: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                log!("⚠️ Failed to get target space card list: {:?}", e);
            }
        }
        
        // ============================================================
        // 步骤 3: 更新卡片元数据
        // ============================================================
        
        log!("🚚 Step 3: Updating card metadata with new space_id and position");
        self.save_card_metadata(card).await?;
        
        // 更新本地缓存
        let source_space_owned = source_space_id.to_owned();
        let card_id_owned = card_id.to_owned();
        crate::kanban::local_cache::remove_card_from_cache(&source_space_owned, &card_id_owned);
        crate::kanban::local_cache::add_card_to_space_cache(target_space_id.to_owned(), card_id.to_owned());
        
        log!("✅ Successfully moved card {} from space {} to space {}", 
            card_id, source_space_id, target_space_id);
        
        Ok(())
    }
}
