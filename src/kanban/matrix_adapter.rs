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
use makepad_widgets::log;
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
        
        for room in all_rooms {
            let room_id = room.room_id();
            
            // 检查topic中是否有[kanban-list]标记
            if let Some(topic) = room.topic() {
                if topic.contains("[kanban-list]") {
                    // 提取列表名称（去掉标记）
                    let name = topic.trim_start_matches("[kanban-list]").trim();
                    let name = if name.is_empty() {
                        room.display_name().await?.to_string()
                    } else {
                        name.to_string()
                    };
                    
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
                    
                    log!("Found kanban list: {} ({}) with {} cards", lists.last().unwrap().name, room_id, card_count);
                }
            }
        }

        Ok(lists)
    }

    /// 创建新的看板 Space（列表）
    pub async fn create_space(&self, name: &str) -> Result<OwnedRoomId> {
        use matrix_sdk::ruma::{
            api::client::room::create_room::v3::{Request as CreateRoomRequest, RoomPreset},
            events::room::topic::RoomTopicEventContent,
        };

        // 构建创建 Space 的请求
        let mut request = CreateRoomRequest::new();
        request.name = Some(name.to_string());
        request.preset = Some(RoomPreset::PrivateChat);
        request.is_direct = false;
        
        // 创建 Space
        let room = self.client.create_room(request).await
            .context("Failed to create kanban space")?;

        let space_id = room.room_id().to_owned();
        
        // 设置 topic（包含 [kanban-list] 标记）
        let topic_with_marker = format!("[kanban-list] {}", name);
        
        log!("Setting topic for space {}: {}", space_id, topic_with_marker);
        
        let topic_content = RoomTopicEventContent::new(topic_with_marker);
        room.send_state_event(topic_content).await
            .context("Failed to set space topic")?;

        log!("Created kanban space: {} ({})", name, space_id);
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

        // 构建创建 Room 的请求
        let mut request = CreateRoomRequest::new();
        request.name = Some(title.to_string());
        request.preset = Some(RoomPreset::PrivateChat);
        request.is_direct = false;

        // 创建 Room
        let room = self.client.create_room(request).await
            .context("Failed to create card room")?;

        let card_room_id = room.room_id().to_owned();

        // 将卡片 Room 添加到 Space
        self.add_card_to_space(space_id, &card_room_id).await?;

        log!("Created kanban card: {} in space {} ({})", title, space_id, card_room_id);
        Ok(card_room_id)
    }

    /// 从 Matrix Room 加载卡片（简化版）
    pub async fn load_card(&self, room_id: &RoomId, space_id: OwnedRoomId) -> Result<crate::kanban::state::kanban_state::KanbanCard> {
        let room = self.client
            .get_room(room_id)
            .context("Room not found")?;

        let display_name = room.display_name().await?;
        let title = display_name.to_string();
        
        // TODO: 从 state event 读取描述和位置
        Ok(crate::kanban::state::kanban_state::KanbanCard {
            id: room_id.to_owned(),
            title,
            description: None,
            space_id,
            position: 1000.0,
        })
    }

    /// 将卡片添加到 Space（设置 Space 子关系）
    async fn add_card_to_space(&self, space_id: &RoomId, card_room_id: &RoomId) -> Result<()> {
        let space = self.client
            .get_room(space_id)
            .context("Space not found")?;

        use matrix_sdk::ruma::events::space::child::SpaceChildEventContent;
        
        let child_content = SpaceChildEventContent::new(vec![]);
        
        space.send_state_event_raw(
            "m.space.child",
            card_room_id.as_str(),
            serde_json::value::to_raw_value(&child_content)
                .context("Failed to serialize space child content")?,
        ).await.context("Failed to add card to space")?;

        log!("Added card {} to space {}", card_room_id, space_id);
        Ok(())
    }

    /// 获取 Space 的所有子 Room
    async fn get_space_children(&self, space: &Room) -> Result<Vec<Room>> {
        let mut children = Vec::new();

        // 获取所有房间并检查它们是否是这个 Space 的子房间
        // 通过检查 m.space.child 状态事件来确定父子关系
        let all_rooms = self.client.rooms();
        
        for room in all_rooms {
            // 检查 space 中是否有指向这个 room 的 m.space.child 事件
            // state_key 应该是 room 的 ID
            let room_id_str = room.room_id().as_str();
            
            // 尝试获取 m.space.child 状态事件
            match space.get_state_event(matrix_sdk::ruma::events::StateEventType::SpaceChild, room_id_str).await {
                Ok(Some(_)) => {
                    // 找到了 m.space.child 事件，说明这是一个子房间
                    log!("Found child room: {} in space {}", room_id_str, space.room_id());
                    children.push(room);
                }
                Ok(None) => {
                    // 没有找到事件，不是子房间
                }
                Err(e) => {
                    // 获取状态事件失败，可能是权限问题或网络问题
                    log!("Failed to get space child event for {}: {:?}", room_id_str, e);
                }
            }
        }
        
        log!("Found {} child rooms in space {}", children.len(), space.room_id());
        Ok(children)
    }
}
