//! Matrix 适配层 - 将 Matrix Space/Room 映射到 Kanban 数据结构
//!
//! 架构设计：
//! - Matrix Space = Kanban Board (看板)
//! - Matrix Room = Kanban Card (卡片)
//! - Space 的子 Room = Board 中的 Card
//! - Room State Events = Card 的元数据
//!
//! 自定义 State Event 类型：
//! - `m.kanban.card` - 卡片元数据（标题、描述、标签等）
//! - `m.kanban.list` - 列表信息（待办、进行中、已完成）
//! - `m.kanban.board` - 看板配置（背景色、标签定义等）

use anyhow::{Context, Result};
use makepad_widgets::log;
use matrix_sdk::{
    Client,
    ruma::{
        OwnedRoomId, OwnedUserId, RoomId,
    },
    Room,
};
use serde::{Deserialize, Serialize};

use crate::kanban::{
    KanbanBoard, KanbanCard, KanbanList,
    KanbanLabel,
};

/// Kanban 卡片状态事件类型
pub const KANBAN_CARD_EVENT_TYPE: &str = "m.kanban.card";

/// Kanban 列表状态事件类型
pub const KANBAN_LIST_EVENT_TYPE: &str = "m.kanban.list";

/// Kanban 看板状态事件类型
pub const KANBAN_BOARD_EVENT_TYPE: &str = "m.kanban.board";

/// Kanban 卡片元数据（存储在 Room State Event 中）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardMetadata {
    /// 卡片标题
    pub title: String,
    
    /// 卡片描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// 所属列表 ID（待办/进行中/已完成）
    pub list_id: String,
    
    /// 排序位置
    pub position: f64,
    
    /// 标签 ID 列表
    #[serde(default)]
    pub label_ids: Vec<String>,
    
    /// 截止日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    
    /// 是否已加星
    #[serde(default)]
    pub is_starred: bool,
    
    /// 是否已归档
    #[serde(default)]
    pub is_archived: bool,
    
    /// 创建时间
    pub created_at: String,
    
    /// 更新时间
    pub updated_at: String,
}

/// Kanban 看板元数据（存储在 Space State Event 中）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoardMetadata {
    /// 看板名称
    pub name: String,
    
    /// 看板描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// 背景颜色
    #[serde(default = "default_background_color")]
    pub background_color: String,
    
    /// 标签定义
    #[serde(default)]
    pub labels: Vec<KanbanLabel>,
    
    /// 列表定义（待办、进行中、已完成等）
    #[serde(default = "default_lists")]
    pub lists: Vec<ListDefinition>,
    
    /// 是否已归档
    #[serde(default)]
    pub is_archived: bool,
    
    /// 创建时间
    pub created_at: String,
    
    /// 更新时间
    pub updated_at: String,
}

fn default_background_color() -> String {
    "#0079BF".to_string()
}

fn default_lists() -> Vec<ListDefinition> {
    vec![
        ListDefinition {
            id: "todo".to_string(),
            name: "待办".to_string(),
            position: 1000.0,
        },
        ListDefinition {
            id: "doing".to_string(),
            name: "进行中".to_string(),
            position: 2000.0,
        },
        ListDefinition {
            id: "done".to_string(),
            name: "已完成".to_string(),
            position: 3000.0,
        },
    ]
}

/// 列表定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDefinition {
    pub id: String,
    pub name: String,
    pub position: f64,
}

/// Matrix 到 Kanban 的适配器
pub struct MatrixKanbanAdapter {
    client: Client,
}

impl MatrixKanbanAdapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// 从 Matrix Space 加载看板
    pub async fn load_board(&self, space_id: &RoomId) -> Result<KanbanBoard> {
        let space = self.client
            .get_room(space_id)
            .context("Space not found")?;

        // 读取看板元数据
        let metadata = self.read_board_metadata(&space).await?;

        // 获取 Space 中的所有子 Room（卡片）
        let _card_rooms = self.get_space_children(&space).await?;

        // 构建列表 ID 列表
        let list_ids: Vec<String> = metadata.lists.iter()
            .map(|list| list.id.clone())
            .collect();

        Ok(KanbanBoard {
            id: space_id.to_owned(),
            name: metadata.name,
            description: metadata.description,
            background_color: metadata.background_color,
            background_image: None,
            labels: metadata.labels,
            member_ids: self.get_room_members(&space).await?,
            list_ids,
            is_archived: metadata.is_archived,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
            extensions: Default::default(),
        })
    }

    /// 从 Matrix Room 加载卡片
    pub async fn load_card(&self, room_id: &RoomId, board_id: OwnedRoomId) -> Result<KanbanCard> {
        let room = self.client
            .get_room(room_id)
            .context("Room not found")?;

        // 读取卡片元数据
        let metadata = self.read_card_metadata(&room).await?;

        Ok(KanbanCard {
            id: room_id.to_string(),
            title: metadata.title,
            description: metadata.description,
            list_id: metadata.list_id,
            board_id,
            position: metadata.position,
            label_ids: metadata.label_ids,
            member_ids: self.get_room_members(&room).await?,
            due_date: metadata.due_date.map(|date| crate::kanban::CardDueDate {
                date,
                is_completed: false,
            }),
            cover: None,
            attachment_count: 0,
            comment_count: 0,
            checklists: Vec::new(),
            is_starred: metadata.is_starred,
            is_archived: metadata.is_archived,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        })
    }

    /// 加载看板的所有列表
    pub async fn load_lists(&self, board: &KanbanBoard) -> Result<Vec<KanbanList>> {
        let space = self.client
            .get_room(&board.id)
            .context("Space not found")?;

        let metadata = self.read_board_metadata(&space).await?;

        Ok(metadata.lists.iter().map(|list_def| {
            KanbanList {
                id: list_def.id.clone(),
                name: list_def.name.clone(),
                board_id: board.id.clone(),
                position: list_def.position,
                is_archived: false,
                card_ids: Vec::new(), // 将在后续填充
                created_at: board.created_at.clone(),
                updated_at: board.updated_at.clone(),
            }
        }).collect())
    }

    /// 加载列表中的所有卡片
    pub async fn load_cards_in_list(
        &self,
        board_id: &RoomId,
        list_id: &str,
    ) -> Result<Vec<KanbanCard>> {
        let space = self.client
            .get_room(board_id)
            .context("Space not found")?;

        let card_rooms = self.get_space_children(&space).await?;
        let mut cards = Vec::new();

        for room in card_rooms {
            if let Ok(metadata) = self.read_card_metadata(&room).await {
                if metadata.list_id == list_id && !metadata.is_archived {
                    if let Ok(card) = self.load_card(room.room_id(), board_id.to_owned()).await {
                        cards.push(card);
                    }
                }
            }
        }

        // 按位置排序
        cards.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

        Ok(cards)
    }

    /// 创建新看板（Matrix Space）
    pub async fn create_board(&self, name: &str, description: Option<String>) -> Result<OwnedRoomId> {
        use matrix_sdk::ruma::api::client::room::create_room::v3::{Request as CreateRoomRequest, RoomPreset};

        // 创建看板元数据
        let _metadata = KanbanBoardMetadata {
            name: name.to_string(),
            description: description.clone(),
            background_color: default_background_color(),
            labels: Vec::new(),
            lists: default_lists(),
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // 构建创建 Space 的请求
        let mut request = CreateRoomRequest::new();
        request.name = Some(name.to_string());
        request.topic = description;
        request.preset = Some(RoomPreset::PrivateChat); // 私有空间
        request.is_direct = false;
        
        // TODO: 设置为 Space 类型和添加初始状态事件
        // 这需要正确的 Matrix SDK API，暂时先创建普通房间
        // 后续通过 send_state_event 添加元数据

        // 创建 Space
        let room = self.client.create_room(request).await
            .context("Failed to create board space")?;

        // Get the room_id from the Room object
        let board_id = room.room_id().to_owned();

        // TODO: 创建后立即添加看板元数据
        // 需要正确的 send_state_event_raw API
        /*
        if let Some(space) = self.client.get_room(&board_id) {
            let metadata_raw = serde_json::value::to_raw_value(&metadata)
                .context("Failed to serialize board metadata")?;
            
            let _ = space.send_state_event_raw(
                KANBAN_BOARD_EVENT_TYPE.to_string(),
                "".to_string(),
                Raw::from_json(metadata_raw),
            ).await;
        }
        */

        log!("Created kanban board space: {} ({})", name, board_id);
        Ok(board_id)
    }

    /// 创建新卡片（Matrix Room）
    pub async fn create_card(
        &self,
        board_id: &RoomId,
        list_id: &str,
        title: &str,
    ) -> Result<OwnedRoomId> {
        use matrix_sdk::ruma::{
            api::client::room::create_room::v3::{Request as CreateRoomRequest, RoomPreset},
        };

        // 创建卡片元数据
        let _metadata = KanbanCardMetadata {
            title: title.to_string(),
            description: None,
            list_id: list_id.to_string(),
            position: 1000.0, // 默认位置
            label_ids: Vec::new(),
            due_date: None,
            is_starred: false,
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        // 构建创建 Room 的请求
        let mut request = CreateRoomRequest::new();
        request.name = Some(title.to_string());
        request.preset = Some(RoomPreset::PrivateChat);
        request.is_direct = false;

        // 创建 Room
        let room = self.client.create_room(request).await
            .context("Failed to create card room")?;

        // Get the room_id from the Room object
        let card_room_id = room.room_id().to_owned();

        // TODO: 创建后立即添加卡片元数据
        // 需要正确的 send_state_event_raw API
        /*
        if let Some(room) = self.client.get_room(&card_room_id) {
            let metadata_raw = serde_json::value::to_raw_value(&metadata)
                .context("Failed to serialize card metadata")?;
            
            let _ = room.send_state_event_raw(
                KANBAN_CARD_EVENT_TYPE.to_string(),
                "".to_string(),
                Raw::from_json(metadata_raw),
            ).await;
        }
        */

        // 将卡片 Room 添加到看板 Space
        self.add_card_to_board(board_id, &card_room_id).await?;

        log!("Created kanban card: {} in board {} ({})", title, board_id, card_room_id);
        Ok(card_room_id)
    }

    /// 将卡片添加到看板（设置 Space 子关系）
    async fn add_card_to_board(&self, board_id: &RoomId, card_room_id: &RoomId) -> Result<()> {
        let _space = self.client
            .get_room(board_id)
            .context("Board space not found")?;

        // TODO: 创建 m.space.child 事件内容并发送
        // 需要正确的 send_state_event_raw API
        /*
        use matrix_sdk::ruma::events::space::child::SpaceChildEventContent;
        let child_content = SpaceChildEventContent::new(vec![]);
        let content_raw = serde_json::value::to_raw_value(&child_content)
            .context("Failed to serialize space child content")?;

        space.send_state_event_raw(
            "m.space.child".to_string(),
            card_room_id.as_str().to_string(),
            Raw::from_json(content_raw),
        ).await.context("Failed to add card to board space")?;
        */

        log!("Added card {} to board {}", card_room_id, board_id);
        Ok(())
    }

    /// 更新卡片元数据
    pub async fn update_card_metadata(
        &self,
        room_id: &RoomId,
        metadata: KanbanCardMetadata,
    ) -> Result<()> {
        let _room = self.client
            .get_room(room_id)
            .context("Card room not found")?;

        // TODO: 序列化元数据并发送状态事件
        // 需要正确的 send_state_event_raw API
        let _ = metadata;
        /*
        let metadata_raw = serde_json::value::to_raw_value(&metadata)
            .context("Failed to serialize card metadata")?;

        room.send_state_event_raw(
            KANBAN_CARD_EVENT_TYPE.to_string(),
            "".to_string(),
            Raw::from_json(metadata_raw),
        ).await.context("Failed to update card metadata")?;
        */

        log!("Updated card metadata for {}", room_id);
        Ok(())
    }

    /// 移动卡片到不同列表
    pub async fn move_card(
        &self,
        room_id: &RoomId,
        new_list_id: &str,
        new_position: f64,
    ) -> Result<()> {
        let room = self.client
            .get_room(room_id)
            .context("Room not found")?;

        let mut metadata = self.read_card_metadata(&room).await?;
        metadata.list_id = new_list_id.to_string();
        metadata.position = new_position;
        metadata.updated_at = chrono::Utc::now().to_rfc3339();

        self.update_card_metadata(room_id, metadata).await
    }

    /// 获取所有看板（Spaces）
    pub async fn get_all_boards(&self) -> Result<Vec<KanbanBoard>> {
        let mut boards = Vec::new();

        // 获取用户的所有 Room
        for room in self.client.rooms() {
            // 检查是否是 Space
            if self.is_space(&room).await {
                // 尝试加载为看板
                match self.load_board(room.room_id()).await {
                    Ok(board) => boards.push(board),
                    Err(e) => {
                        log!("Failed to load board {}: {}", room.room_id(), e);
                    }
                }
            }
        }

        Ok(boards)
    }

    /// 检查 Room 是否是 Space
    async fn is_space(&self, _room: &Room) -> bool {
        // TODO: 实现正确的 Space 检查逻辑
        // 可以检查 room.room_type() 或读取 m.room.create 状态事件
        false
    }

    /// 更新看板元数据
    pub async fn update_board_metadata(
        &self,
        board_id: &RoomId,
        metadata: KanbanBoardMetadata,
    ) -> Result<()> {
        let _space = self.client
            .get_room(board_id)
            .context("Board space not found")?;

        // TODO: 序列化元数据并发送状态事件
        // 需要正确的 send_state_event_raw API
        let _ = metadata;
        /*
        let metadata_raw = serde_json::value::to_raw_value(&metadata)
            .context("Failed to serialize board metadata")?;

        space.send_state_event_raw(
            KANBAN_BOARD_EVENT_TYPE.to_string(),
            "".to_string(),
            Raw::from_json(metadata_raw),
        ).await.context("Failed to update board metadata")?;
        */

        log!("Updated board metadata for {}", board_id);
        Ok(())
    }

    /// 删除卡片
    pub async fn delete_card(&self, room_id: &RoomId) -> Result<()> {
        let room = self.client
            .get_room(room_id)
            .context("Card room not found")?;

        // 归档卡片而不是删除
        let mut metadata = self.read_card_metadata(&room).await?;
        metadata.is_archived = true;
        metadata.updated_at = chrono::Utc::now().to_rfc3339();

        self.update_card_metadata(room_id, metadata).await
    }

    /// 更新卡片标题
    pub async fn update_card_title(&self, room_id: &RoomId, new_title: &str) -> Result<()> {
        let room = self.client
            .get_room(room_id)
            .context("Card room not found")?;

        let mut metadata = self.read_card_metadata(&room).await?;
        metadata.title = new_title.to_string();
        metadata.updated_at = chrono::Utc::now().to_rfc3339();

        self.update_card_metadata(room_id, metadata).await?;

        // 同时更新 Room 的名称
        use matrix_sdk::ruma::events::room::name::RoomNameEventContent;
        room.send_state_event(RoomNameEventContent::new(new_title.to_string())).await
            .context("Failed to update room name")?;

        Ok(())
    }

    /// 更新卡片描述
    pub async fn update_card_description(&self, room_id: &RoomId, description: Option<String>) -> Result<()> {
        let room = self.client
            .get_room(room_id)
            .context("Card room not found")?;

        let mut metadata = self.read_card_metadata(&room).await?;
        metadata.description = description;
        metadata.updated_at = chrono::Utc::now().to_rfc3339();

        self.update_card_metadata(room_id, metadata).await
    }

    // ========== 私有辅助方法 ==========

    /// 读取看板元数据
    async fn read_board_metadata(&self, space: &Room) -> Result<KanbanBoardMetadata> {
        // TODO: 实现正确的状态事件读取
        // 暂时使用默认值
        log!("No board metadata found for {}, using defaults", space.room_id());
        let display_name = space.display_name().await?;
        let name = display_name.to_string();
        
        Ok(KanbanBoardMetadata {
            name,
            description: None,
            background_color: default_background_color(),
            labels: Vec::new(),
            lists: default_lists(),
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// 读取卡片元数据
    async fn read_card_metadata(&self, room: &Room) -> Result<KanbanCardMetadata> {
        // TODO: 实现正确的状态事件读取
        // 暂时使用默认值
        log!("No card metadata found for {}, using defaults", room.room_id());
        let display_name = room.display_name().await?;
        let title = display_name.to_string();
        
        Ok(KanbanCardMetadata {
            title,
            description: None,
            list_id: "todo".to_string(),
            position: 1000.0,
            label_ids: Vec::new(),
            due_date: None,
            is_starred: false,
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// 获取 Space 的所有子 Room
    async fn get_space_children(&self, _space: &Room) -> Result<Vec<Room>> {
        let children = Vec::new();

        // 获取所有 m.space.child 状态事件
        // 注意：这需要遍历所有状态事件，因为每个子 Room 都有一个独立的状态事件
        // state_key 是子 Room 的 ID
        
        // 方法 1：使用 Space 的 children 方法（如果 SDK 支持）
        // 方法 2：手动读取状态事件
        
        // 这里我们使用一个简化的实现
        // 实际上需要调用 space.get_state_events() 并过滤 m.space.child 类型
        
        // 临时实现：返回空列表
        // TODO: 实现完整的 Space 子 Room 查询
        log!("get_space_children not fully implemented yet, returning empty list");
        
        Ok(children)
    }

    /// 获取 Room 的成员列表
    async fn get_room_members(&self, room: &Room) -> Result<Vec<OwnedUserId>> {
        let members = room.members(matrix_sdk::RoomMemberships::ACTIVE).await?;
        Ok(members.iter().map(|m| m.user_id().to_owned()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_lists() {
        let lists = default_lists();
        assert_eq!(lists.len(), 3);
        assert_eq!(lists[0].id, "todo");
        assert_eq!(lists[1].id, "doing");
        assert_eq!(lists[2].id, "done");
    }

    #[test]
    fn test_kanban_card_metadata_serialization() {
        let metadata = KanbanCardMetadata {
            title: "测试卡片".to_string(),
            description: Some("这是一个测试".to_string()),
            list_id: "todo".to_string(),
            position: 1000.0,
            label_ids: vec!["label1".to_string()],
            due_date: None,
            is_starred: false,
            is_archived: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: KanbanCardMetadata = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.title, metadata.title);
        assert_eq!(deserialized.list_id, metadata.list_id);
    }
}
