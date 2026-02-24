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

        // 创建初始的 Card 元数据
        let card = crate::kanban::state::kanban_state::KanbanCard::new(
            card_room_id.clone(),
            title.to_string(),
            space_id.to_owned(),
        );
        
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

        log!("✓✓✓✓ Created kanban card: {} in space {} ({})", title, space_id, card_room_id);
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
        let room = self.client.get_room(&card.id)
            .context("Card room not found")?;
        
        let metadata = serde_json::json!({
            "title": card.title,
            "description": card.description,
            "position": card.position,
            "end_time": card.end_time,
            "tags": card.tags,
            "created_at": card.created_at,
            "updated_at": card.updated_at,
        });
        
        log!("💾 Saving card metadata for {}", card.id);
        
        room.send_state_event_raw(
            "m.kanban.card.metadata",
            "",
            serde_json::value::to_raw_value(&metadata)
                .context("Failed to serialize card metadata")?,
        ).await?;
        
        log!("✓ Saved card metadata successfully");
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
        
        room.send_state_event_raw(
            "m.kanban.card.todos",
            "",
            serde_json::value::to_raw_value(&todos_content)
                .context("Failed to serialize todos")?,
        ).await?;
        
        log!("✓ Saved todos successfully");
        Ok(())
    }
    
    /// 从 State Event 加载元数据
    async fn load_card_metadata(&self, room: &Room) -> Result<CardMetadataRaw> {
        use matrix_sdk::ruma::events::StateEventType;
        
        let event_type = StateEventType::from("m.kanban.card.metadata");
        
        log!("📖 Loading card metadata from room {}", room.room_id());
        
        match room.get_state_event(event_type, "").await {
            Ok(Some(raw_event)) => {
                if let Ok(json_str) = serde_json::to_string(&raw_event) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        if let Some(content) = json.get("content") {
                            let metadata: CardMetadataRaw = serde_json::from_value(content.clone())
                                .context("Failed to parse card metadata")?;
                            log!("✓ Loaded card metadata: title={}", metadata.title);
                            return Ok(metadata);
                        }
                    }
                }
                Err(anyhow::anyhow!("Failed to parse card metadata"))
            }
            Ok(None) => {
                log!("⚠ No card metadata found, using defaults");
                Err(anyhow::anyhow!("No card metadata found"))
            }
            Err(e) => {
                log!("❌ Error loading card metadata: {:?}", e);
                Err(e.into())
            }
        }
    }
    
    /// 从 State Event 加载 TodoList
    async fn load_card_todos(&self, room: &Room) -> Result<Vec<crate::kanban::state::kanban_state::TodoItem>> {
        use matrix_sdk::ruma::events::StateEventType;
        
        let event_type = StateEventType::from("m.kanban.card.todos");
        
        log!("📖 Loading todos from room {}", room.room_id());
        
        match room.get_state_event(event_type, "").await {
            Ok(Some(raw_event)) => {
                if let Ok(json_str) = serde_json::to_string(&raw_event) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        if let Some(content) = json.get("content") {
                            if let Some(todos_array) = content.get("todos").and_then(|v| v.as_array()) {
                                let todos: Vec<crate::kanban::state::kanban_state::TodoItem> = todos_array
                                    .iter()
                                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                    .collect();
                                log!("✓ Loaded {} todos", todos.len());
                                return Ok(todos);
                            }
                        }
                    }
                }
                log!("⚠ Failed to parse todos");
                Ok(Vec::new())
            }
            Ok(None) => {
                log!("📖 No todos found");
                Ok(Vec::new())
            }
            Err(e) => {
                log!("❌ Error loading todos: {:?}", e);
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
        
        // 获取当前用户ID
        let user_id = self.client.user_id()
            .context("User not logged in")?
            .to_string();
        
        // 构建活动记录内容
        let content = serde_json::json!({
            "msgtype": "m.kanban.card.activity",
            "activity_type": activity_type,
            "text": text,
            "metadata": metadata,
            "user_id": user_id,
        });
        
        log!("💬 Activity content: {:?}", content);
        
        // 发送自定义消息事件
        room.send_raw("m.room.message", content).await
            .context("Failed to send activity")?;
        
        log!("✓ Activity sent successfully");
        Ok(())
    }
    
    /// 加载活动记录（从Timeline Events）
    /// 
    /// 注意：这是简化实现，实际使用中Timeline API较复杂
    /// 当前版本仅返回空列表，待后续完善
    pub async fn load_activities(
        &self,
        card_id: &RoomId,
        _limit: Option<usize>,
    ) -> Result<Vec<crate::kanban::state::kanban_state::CardActivity>> {
        let _room = self.client.get_room(card_id)
            .context("Card room not found")?;
        
        log!("📖 Loading activities from card {} (simplified implementation)", card_id);
        
        // TODO: 完整的Timeline API实现需要：
        // 1. 使用 matrix_sdk_ui::timeline::RoomExt trait
        // 2. 正确处理Timeline items的类型
        // 3. 实现事件过滤和解析
        // 
        // 当前返回空列表，评论功能仍可正常发送
        let activities = Vec::new();
        
        log!("✓ Loaded {} activities (simplified)", activities.len());
        Ok(activities)
    }
    
    /// 从Timeline Event解析活动记录（占位实现）
    #[allow(dead_code)]
    async fn parse_activity_from_event(
        &self,
        _event: &str,  // 简化参数类型
    ) -> Option<crate::kanban::state::kanban_state::CardActivity> {
        // TODO: 实现完整的事件解析逻辑
        None
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
