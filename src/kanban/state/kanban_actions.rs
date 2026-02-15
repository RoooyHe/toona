use matrix_sdk::ruma::OwnedRoomId;
use crate::kanban::state::kanban_state::{KanbanList, KanbanCard};

/// 看板应用动作（简化版）
#[derive(Debug, Clone)]
pub enum KanbanActions {
    /// 加载所有列表（Space）
    LoadLists,
    
    /// 列表已加载
    ListLoaded(KanbanList),
    
    /// 卡片已加载
    CardLoaded(KanbanCard),
    
    /// 创建新列表（Space）
    CreateList {
        name: String,
    },
    
    /// 创建新卡片（Room）
    CreateCard {
        space_id: OwnedRoomId,
        title: String,
    },
    
    /// 移动卡片到不同列表
    MoveCard {
        card_id: OwnedRoomId,
        target_space_id: OwnedRoomId,
        position: f64,
    },
    
    /// 更新卡片标题
    UpdateCardTitle {
        card_id: OwnedRoomId,
        title: String,
    },
    
    /// 更新卡片描述
    UpdateCardDescription {
        card_id: OwnedRoomId,
        description: Option<String>,
    },
    
    /// 删除卡片
    DeleteCard {
        card_id: OwnedRoomId,
    },
    
    /// 设置加载状态
    Loading(bool),
    
    /// 错误
    Error(String),
}
