//! 看板 API 层
//!
//! 提供高级别的看板操作 API，封装数据层与 Matrix SDK 的交互

use crate::kanban::data::repositories::{
    BoardRepositoryTrait, ListRepositoryTrait, CardRepositoryTrait,
    MatrixBoardRepository, MatrixListRepository, MatrixCardRepository,
    BoardUpdateRequest, ListUpdateRequest, CardUpdateRequest,
};
use crate::kanban::data::models::{KanbanBoard, KanbanList, KanbanCard};
use matrix_sdk::{ruma::RoomId, Client};
use anyhow::Result;
use std::sync::Arc;

/// 看板 API
pub struct KanbanApi {
    board_repository: Arc<MatrixBoardRepository>,
    list_repository: Arc<MatrixListRepository>,
    card_repository: Arc<MatrixCardRepository>,
}

impl KanbanApi {
    pub fn new() -> Self {
        Self {
            board_repository: Arc::new(MatrixBoardRepository::new()),
            list_repository: Arc::new(MatrixListRepository::new()),
            card_repository: Arc::new(MatrixCardRepository::new()),
        }
    }

    /// 创建看板
    pub async fn create_board(&self, client: &Client, name: &str, description: Option<&str>) -> Result<KanbanBoard> {
        self.board_repository.create_board(client, name, description).await
    }

    /// 获取看板
    pub async fn get_board(&self, client: &Client, room_id: &RoomId) -> Result<KanbanBoard> {
        self.board_repository.get_board(client, room_id).await
    }

    /// 更新看板
    pub async fn update_board(&self, client: &Client, room_id: &RoomId, updates: BoardUpdateRequest) -> Result<()> {
        self.board_repository.update_board(client, room_id, updates).await
    }

    /// 删除看板
    pub async fn delete_board(&self, client: &Client, room_id: &RoomId) -> Result<()> {
        self.board_repository.delete_board(client, room_id).await
    }

    /// 获取所有看板
    pub async fn get_boards(&self, client: &Client) -> Result<Vec<KanbanBoard>> {
        self.board_repository.get_boards(client).await
    }

    /// 创建列表
    pub async fn create_list(&self, board_id: &RoomId, name: &str) -> Result<KanbanList> {
        self.list_repository.create_list(board_id, name).await
    }

    /// 更新列表
    pub async fn update_list(&self, board_id: &RoomId, list_id: &str, updates: ListUpdateRequest) -> Result<()> {
        self.list_repository.update_list(board_id, list_id, updates).await
    }

    /// 删除列表
    pub async fn delete_list(&self, board_id: &RoomId, list_id: &str) -> Result<()> {
        self.list_repository.delete_list(board_id, list_id).await
    }

    /// 移动列表
    pub async fn move_list(&self, board_id: &RoomId, list_id: &str, new_position: f64) -> Result<()> {
        self.list_repository.move_list(board_id, list_id, new_position).await
    }

    /// 创建卡片
    pub async fn create_card(&self, client: &Client, board_id: &RoomId, list_id: &str, title: &str) -> Result<KanbanCard> {
        self.card_repository.create_card(client, board_id, list_id, title).await
    }

    /// 更新卡片
    pub async fn update_card(&self, client: &Client, board_id: &RoomId, card_id: &str, updates: CardUpdateRequest) -> Result<()> {
        self.card_repository.update_card(client, board_id, card_id, updates).await
    }

    /// 删除卡片
    pub async fn delete_card(&self, board_id: &RoomId, card_id: &str) -> Result<()> {
        self.card_repository.delete_card(board_id, card_id).await
    }

    /// 移动卡片
    pub async fn move_card(&self, board_id: &RoomId, card_id: &str, to_list_id: &str, new_position: f64) -> Result<()> {
        self.card_repository.move_card(board_id, card_id, to_list_id, new_position).await
    }

    /// 获取卡片列表
    pub async fn get_cards(&self, board_id: &RoomId, list_id: &str) -> Result<Vec<KanbanCard>> {
        self.card_repository.get_cards(board_id, list_id).await
    }
}

impl Default for KanbanApi {
    fn default() -> Self {
        Self::new()
    }
}
