//! 看板数据仓储层
//!
//! 实现与 Matrix SDK 的交互，提供看板、列表、卡片的数据操作

use matrix_sdk::{ruma::RoomId, Client};
use anyhow::{Result, anyhow};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::kanban::data::models::{KanbanBoard, KanbanList, KanbanCard, CardDueDate};

/// 看板仓储 trait
#[async_trait::async_trait]
pub trait BoardRepositoryTrait {
    async fn create_board(&self, name: &str, description: Option<&str>) -> Result<KanbanBoard>;
    async fn get_board(&self, client: &Client, room_id: &RoomId) -> Result<KanbanBoard>;
    async fn update_board(
        &self,
        client: &Client,
        room_id: &RoomId,
        updates: BoardUpdateRequest,
    ) -> Result<()>;
    async fn delete_board(&self, client: &Client, room_id: &RoomId) -> Result<()>;
    async fn get_boards(&self, client: &Client) -> Result<Vec<KanbanBoard>>;
}

/// 列表仓储 trait
#[async_trait::async_trait]
pub trait ListRepositoryTrait {
    async fn create_list(&self, board_id: &RoomId, name: &str) -> Result<KanbanList>;
    async fn update_list(
        &self,
        board_id: &RoomId,
        list_id: &str,
        updates: ListUpdateRequest,
    ) -> Result<()>;
    async fn delete_list(&self, board_id: &RoomId, list_id: &str) -> Result<()>;
    async fn move_list(&self, board_id: &RoomId, list_id: &str, new_position: f64) -> Result<()>;
}

/// 卡片仓储 trait
#[async_trait::async_trait]
pub trait CardRepositoryTrait {
    async fn create_card(
        &self,
        board_id: &RoomId,
        list_id: &str,
        title: &str,
    ) -> Result<KanbanCard>;
    async fn update_card(
        &self,
        board_id: &RoomId,
        card_id: &str,
        updates: CardUpdateRequest,
    ) -> Result<()>;
    async fn delete_card(&self, board_id: &RoomId, card_id: &str) -> Result<()>;
    async fn move_card(
        &self,
        board_id: &RoomId,
        card_id: &str,
        to_list_id: &str,
        new_position: f64,
    ) -> Result<()>;
    async fn get_cards(&self, board_id: &RoomId, list_id: &str) -> Result<Vec<KanbanCard>>;
}

/// 看板更新请求
#[derive(Debug, Clone, Default)]
pub struct BoardUpdateRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub background_color: Option<String>,
    pub background_image: Option<Option<String>>,
}

/// 列表更新请求
#[derive(Debug, Clone, Default)]
pub struct ListUpdateRequest {
    pub name: Option<String>,
    pub archived: Option<bool>,
}

/// 卡片更新请求
#[derive(Debug, Clone, Default)]
pub struct CardUpdateRequest {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub label_ids: Option<Vec<String>>,
    pub member_ids: Option<Vec<matrix_sdk::ruma::OwnedUserId>>,
    pub due_date: Option<Option<CardDueDate>>,
    pub is_starred: Option<bool>,
    pub archived: Option<bool>,
}

/// Matrix SDK 看板仓储实现
pub struct MatrixBoardRepository {
    /// 本地存储的看板列表
    local_boards: Arc<Mutex<Vec<KanbanBoard>>>,
}

impl MatrixBoardRepository {
    pub fn new() -> Self {
        Self {
            local_boards: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MatrixBoardRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BoardRepositoryTrait for MatrixBoardRepository {
    async fn create_board(&self, name: &str, description: Option<&str>) -> Result<KanbanBoard> {
        let board = KanbanBoard::new(name);
        let mut boards = self.local_boards.lock().await;
        boards.push(board.clone());
        Ok(board)
    }

    async fn get_board(&self, client: &Client, room_id: &RoomId) -> Result<KanbanBoard> {
        let room = client
            .get_room(room_id)
            .ok_or_else(|| anyhow!("Room not found: {}", room_id))?;

        let name = room.name().unwrap_or_default().to_string();
        let topic = room.topic();

        let board = KanbanBoard {
            id: room_id.to_owned(),
            name,
            description: topic.map(|t| t.to_string()),
            background_color: "#0079BF".to_string(),
            ..Default::default()
        };

        Ok(board)
    }

    async fn update_board(
        &self,
        client: &Client,
        room_id: &RoomId,
        updates: BoardUpdateRequest,
    ) -> Result<()> {
        // TODO: 实现实际的 Matrix SDK 调用
        // - 更新名称使用 set_room_name
        // - 更新描述使用 set_room_topic
        // - 更新背景颜色需要发送状态事件
        Ok(())
    }

    async fn delete_board(&self, client: &Client, room_id: &RoomId) -> Result<()> {
        // 使用 LeaveRoom API
        if let Some(room) = client.get_room(room_id) {
            room.leave().await?;
        }
        Ok(())
    }

    async fn get_boards(&self, client: &Client) -> Result<Vec<KanbanBoard>> {
        // 从 Client 获取所有房间
        let rooms = client.rooms();

        let mut boards = Vec::new();

        // 遍历所有房间，筛选可能的看板
        // 注意：这需要实际实现与 Room 的交互
        // 目前返回空列表作为占位
        for room in rooms {
            let name = room.name().unwrap_or_default().to_string();
            let board = KanbanBoard {
                id: room.room_id().to_owned(),
                name,
                ..Default::default()
            };
            boards.push(board);
        }

        Ok(boards)
    }
}

/// Matrix SDK 列表仓储实现
pub struct MatrixListRepository {
    /// 本地存储的列表数据
    local_lists: Arc<Mutex<Vec<KanbanList>>>,
}

impl MatrixListRepository {
    pub fn new() -> Self {
        Self {
            local_lists: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MatrixListRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ListRepositoryTrait for MatrixListRepository {
    async fn create_list(&self, board_id: &RoomId, name: &str) -> Result<KanbanList> {
        let list = KanbanList::new(name, board_id.to_owned());
        let mut lists = self.local_lists.lock().await;
        lists.push(list.clone());
        Ok(list)
    }

    async fn update_list(
        &self,
        board_id: &RoomId,
        list_id: &str,
        updates: ListUpdateRequest,
    ) -> Result<()> {
        let mut lists = self.local_lists.lock().await;
        if let Some(list) = lists
            .iter_mut()
            .find(|l| l.id == list_id && &l.board_id == board_id)
        {
            if let Some(name) = updates.name {
                list.name = name;
                list.updated_at = chrono::Utc::now().to_rfc3339();
            }
            if let Some(archived) = updates.archived {
                list.is_archived = archived;
            }
        }
        Ok(())
    }

    async fn delete_list(&self, board_id: &RoomId, list_id: &str) -> Result<()> {
        let mut lists = self.local_lists.lock().await;
        lists.retain(|l| !(l.id == list_id && &l.board_id == board_id));
        Ok(())
    }

    async fn move_list(&self, board_id: &RoomId, list_id: &str, new_position: f64) -> Result<()> {
        let mut lists = self.local_lists.lock().await;
        if let Some(list) = lists
            .iter_mut()
            .find(|l| l.id == list_id && &l.board_id == board_id)
        {
            list.position = new_position;
            list.updated_at = chrono::Utc::now().to_rfc3339();
        }
        Ok(())
    }
}

/// Matrix SDK 卡片仓储实现
pub struct MatrixCardRepository {
    /// 本地存储的卡片数据
    local_cards: Arc<Mutex<Vec<KanbanCard>>>,
}

impl MatrixCardRepository {
    pub fn new() -> Self {
        Self {
            local_cards: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MatrixCardRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl CardRepositoryTrait for MatrixCardRepository {
    async fn create_card(
        &self,
        board_id: &RoomId,
        list_id: &str,
        title: &str,
    ) -> Result<KanbanCard> {
        let card = KanbanCard::new(title, list_id.to_string(), board_id.to_owned());
        let mut cards = self.local_cards.lock().await;
        cards.push(card.clone());
        Ok(card)
    }

    async fn update_card(
        &self,
        board_id: &RoomId,
        card_id: &str,
        updates: CardUpdateRequest,
    ) -> Result<()> {
        let mut cards = self.local_cards.lock().await;
        if let Some(card) = cards
            .iter_mut()
            .find(|c| c.id == card_id && &c.board_id == board_id)
        {
            if let Some(title) = updates.title {
                card.title = title;
            }
            if let Some(desc) = updates.description {
                card.description = desc;
            }
            if let Some(labels) = updates.label_ids {
                card.label_ids = labels;
            }
            if let Some(members) = updates.member_ids {
                card.member_ids = members;
            }
            if let Some(due_date) = updates.due_date {
                card.due_date = due_date;
            }
            if let Some(starred) = updates.is_starred {
                card.is_starred = starred;
            }
            if let Some(archived) = updates.archived {
                card.is_archived = archived;
            }
            card.updated_at = chrono::Utc::now().to_rfc3339();
        }
        Ok(())
    }

    async fn delete_card(&self, board_id: &RoomId, card_id: &str) -> Result<()> {
        let mut cards = self.local_cards.lock().await;
        cards.retain(|c| !(c.id == card_id && &c.board_id == board_id));
        Ok(())
    }

    async fn move_card(
        &self,
        board_id: &RoomId,
        card_id: &str,
        to_list_id: &str,
        new_position: f64,
    ) -> Result<()> {
        let mut cards = self.local_cards.lock().await;
        if let Some(card) = cards
            .iter_mut()
            .find(|c| c.id == card_id && &c.board_id == board_id)
        {
            card.list_id = to_list_id.to_string();
            card.position = new_position;
            card.updated_at = chrono::Utc::now().to_rfc3339();
        }
        Ok(())
    }

    async fn get_cards(&self, board_id: &RoomId, list_id: &str) -> Result<Vec<KanbanCard>> {
        let cards = self.local_cards.lock().await;
        let filtered: Vec<KanbanCard> = cards
            .iter()
            .filter(|c| &c.board_id == board_id && c.list_id == list_id)
            .cloned()
            .collect();
        Ok(filtered)
    }
}

/// 仓储工厂，用于创建和管理仓储实例
pub struct RepositoryFactory {
    pub board_repository: MatrixBoardRepository,
    pub list_repository: MatrixListRepository,
    pub card_repository: MatrixCardRepository,
}

impl RepositoryFactory {
    pub fn new() -> Self {
        Self {
            board_repository: MatrixBoardRepository::new(),
            list_repository: MatrixListRepository::new(),
            card_repository: MatrixCardRepository::new(),
        }
    }
}

impl Default for RepositoryFactory {
    fn default() -> Self {
        Self::new()
    }
}
