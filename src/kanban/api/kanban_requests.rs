use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use crate::kanban::data::models::{KanbanBoard, KanbanList, KanbanCard};

/// 看板操作请求
#[derive(Debug, Clone)]
pub enum KanbanRequest {
    /// 创建看板
    CreateBoard {
        name: String,
        description: Option<String>,
        background_color: Option<String>,
        invite: Vec<String>,
    },

    /// 获取看板列表
    GetBoards { include_archived: bool },

    /// 获取单个看板
    GetBoard { board_id: OwnedRoomId },

    /// 更新看板
    UpdateBoard {
        board_id: OwnedRoomId,
        // TODO: 简化架构后不再需要
        // updates: crate::kanban::state::kanban_actions::BoardUpdateRequest,
    },

    /// 删除看板
    DeleteBoard {
        board_id: OwnedRoomId,
        permanent: bool,
    },

    /// 归档看板
    ArchiveBoard {
        board_id: OwnedRoomId,
        archived: bool,
    },

    /// 添加成员
    AddMember {
        board_id: OwnedRoomId,
        user_id: OwnedUserId,
    },

    /// 移除成员
    RemoveMember {
        board_id: OwnedRoomId,
        user_id: OwnedUserId,
    },
}

/// 看板响应
#[derive(Debug, Clone)]
pub enum KanbanResponse {
    Board(KanbanBoard),
    Boards(Vec<KanbanBoard>),
    List(KanbanList),
    Lists(Vec<KanbanList>),
    Card(KanbanCard),
    Cards(Vec<KanbanCard>),
    Success,
    Error(String),
}
