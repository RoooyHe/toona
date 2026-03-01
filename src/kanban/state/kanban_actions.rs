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
    
    /// 更新列表名称
    UpdateListName {
        list_id: OwnedRoomId,
        name: String,
    },
    
    /// 显示编辑列表名称模态框
    ShowEditListName {
        list_id: OwnedRoomId,
        current_name: String,
    },
    
    /// 创建新卡片（Room）
    CreateCard {
        space_id: OwnedRoomId,
        title: String,
    },
    
    /// 显示卡片详情
    ShowCardDetail {
        card_id: OwnedRoomId,
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
    
    // ========== Phase 2: TodoList Actions ==========
    
    /// 添加待办事项
    AddTodo {
        card_id: OwnedRoomId,
        text: String,
    },
    
    /// 切换待办事项完成状态
    ToggleTodo {
        card_id: OwnedRoomId,
        todo_id: String,
    },
    
    /// 更新待办事项文本
    UpdateTodoText {
        card_id: OwnedRoomId,
        todo_id: String,
        text: String,
    },
    
    /// 删除待办事项
    DeleteTodo {
        card_id: OwnedRoomId,
        todo_id: String,
    },
    
    // ========== Phase 3: Tags Actions ==========
    
    /// 添加标签
    AddTag {
        card_id: OwnedRoomId,
        tag: String,
    },
    
    /// 删除标签
    RemoveTag {
        card_id: OwnedRoomId,
        tag: String,
    },
    
    // ========== Phase 4: EndTime Actions ==========
    
    /// 设置截止时间
    SetEndTime {
        card_id: OwnedRoomId,
        end_time: u64,  // Unix timestamp in seconds
    },
    
    /// 清除截止时间
    ClearEndTime {
        card_id: OwnedRoomId,
    },
    
    // ========== Phase 5: Activities Actions ==========
    
    /// 添加评论
    AddComment {
        card_id: OwnedRoomId,
        text: String,
    },
    
    /// 活动记录已加载
    ActivitiesLoaded {
        card_id: OwnedRoomId,
        activities: Vec<crate::kanban::state::kanban_state::CardActivity>,
    },
    
    /// 设置加载状态
    Loading(bool),
    
    /// 错误
    Error(String),
}
