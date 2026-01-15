use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use crate::kanban::data::models::*;

/// 看板应用动作
#[derive(Debug, Clone)]
pub enum KanbanActions {
    /// 加载看板列表
    LoadBoards,

    /// 选择看板
    SelectBoard(OwnedRoomId),

    /// 创建看板
    CreateBoard {
        name: String,
        description: Option<String>,
    },

    /// 更新看板
    UpdateBoard {
        board_id: OwnedRoomId,
        updates: BoardUpdateRequest,
    },

    /// 删除看板
    DeleteBoard { board_id: OwnedRoomId },

    /// 加载列表
    LoadLists { board_id: OwnedRoomId },

    /// 创建列表
    CreateList { board_id: OwnedRoomId, name: String },

    /// 更新列表
    UpdateList {
        board_id: OwnedRoomId,
        list_id: String,
        updates: ListUpdateRequest,
    },

    /// 删除列表
    DeleteList {
        board_id: OwnedRoomId,
        list_id: String,
    },

    /// 移动列表
    MoveList {
        board_id: OwnedRoomId,
        list_id: String,
        new_position: f64,
    },

    /// 加载卡片
    LoadCards {
        board_id: OwnedRoomId,
        list_id: String,
    },

    /// 创建卡片
    CreateCard {
        board_id: OwnedRoomId,
        list_id: String,
        name: String,
    },

    /// 更新卡片
    UpdateCard {
        board_id: OwnedRoomId,
        card_id: String,
        updates: CardUpdateRequest,
    },

    /// 删除卡片
    DeleteCard {
        board_id: OwnedRoomId,
        card_id: String,
    },

    /// 移动卡片
    MoveCard {
        board_id: OwnedRoomId,
        card_id: String,
        from_list: String,
        to_list: String,
        new_position: f64,
    },

    /// 归档卡片
    ArchiveCard {
        board_id: OwnedRoomId,
        card_id: String,
        archived: bool,
    },

    /// 设置筛选条件
    SetFilter(KanbanFilterState),

    /// 设置排序条件
    SetSort(KanbanSortState),

    /// 搜索卡片
    Search {
        board_id: OwnedRoomId,
        query: String,
    },

    /// 错误处理
    Error(String),

    /// 加载中
    Loading(bool),
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
    pub member_ids: Option<Vec<OwnedUserId>>,
    pub due_date: Option<Option<CardDueDate>>,
    pub is_starred: Option<bool>,
    pub archived: Option<bool>,
}
