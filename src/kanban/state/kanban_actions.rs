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
    CreateList { name: String },

    /// 更新列表名称
    UpdateListName { list_id: OwnedRoomId, name: String },

    /// 显示编辑列表名称模态框
    ShowEditListName {
        list_id: OwnedRoomId,
        current_name: String,
    },

    /// 关闭编辑列表名称模态框
    CloseEditListNameModal,

    /// 创建新卡片（Room）
    CreateCard {
        space_id: OwnedRoomId,
        title: String,
    },

    /// 显示卡片详情
    ShowCardDetail { card_id: OwnedRoomId },

    /// 移动卡片到不同列表
    MoveCard {
        card_id: OwnedRoomId,
        target_space_id: OwnedRoomId,
        position: f64,
    },

    /// 更新卡片标题
    UpdateCardTitle { card_id: OwnedRoomId, title: String },

    /// 更新卡片描述
    UpdateCardDescription {
        card_id: OwnedRoomId,
        description: Option<String>,
    },

    /// 删除卡片
    DeleteCard { card_id: OwnedRoomId },

    /// 更新卡片状态
    UpdateCardStatus {
        card_id: OwnedRoomId,
        status: crate::kanban::state::kanban_state::CardStatus,
    },

    // ========== Phase 2: TodoList Actions ==========
    /// 添加待办事项
    AddTodo { card_id: OwnedRoomId, text: String },

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
    /// 添加标签（旧版本，使用标签名称）
    AddTag { card_id: OwnedRoomId, tag: String },

    /// 删除标签（旧版本，使用标签名称）
    RemoveTag { card_id: OwnedRoomId, tag: String },

    // ========== Space 标签管理 Actions ==========
    /// 加载 Space 标签库
    LoadSpaceTags { space_id: OwnedRoomId },

    /// Space 标签已加载
    SpaceTagsLoaded {
        space_id: OwnedRoomId,
        tags: Vec<crate::kanban::state::kanban_state::SpaceTag>,
    },

    /// 创建 Space 标签
    CreateSpaceTag {
        space_id: OwnedRoomId,
        name: String,
        color: String,
    },

    /// 更新 Space 标签
    UpdateSpaceTag {
        space_id: OwnedRoomId,
        tag: crate::kanban::state::kanban_state::SpaceTag,
    },

    /// 删除 Space 标签
    DeleteSpaceTag {
        space_id: OwnedRoomId,
        tag_id: String,
    },

    /// 添加标签到 Card（使用标签 ID）
    AddTagToCard {
        card_id: OwnedRoomId,
        tag_id: String,
    },

    /// 从 Card 移除标签（使用标签 ID）
    RemoveTagFromCard {
        card_id: OwnedRoomId,
        tag_id: String,
    },

    /// 通过标签名称添加标签到 Card（会查找或创建标签）
    AddTagToCardByName {
        card_id: OwnedRoomId,
        space_id: OwnedRoomId,
        tag_name: String,
    },

    /// 显示标签管理模态框
    ShowTagManagementModal {
        space_id: OwnedRoomId,
        card_id: OwnedRoomId,
    },

    /// 关闭标签管理模态框
    CloseTagManagementModal,

    // ========== Phase 4: EndTime Actions ==========
    /// 设置截止时间
    SetEndTime {
        card_id: OwnedRoomId,
        end_time: u64, // Unix timestamp in seconds
    },

    /// 清除截止时间
    ClearEndTime { card_id: OwnedRoomId },

    // ========== Phase 5: Activities Actions ==========
    /// 添加评论
    AddComment { card_id: OwnedRoomId, text: String },

    /// 活动记录已加载
    ActivitiesLoaded {
        card_id: OwnedRoomId,
        activities: Vec<crate::kanban::state::kanban_state::CardActivity>,
    },

    // ========== Phase 6: Drag and Drop Actions ==========
    /// 开始拖拽卡片
    StartDragCard {
        card_id: OwnedRoomId,
        space_id: OwnedRoomId,
        position: f64,
    },

    /// 结束拖拽（放置卡片）
    DropCard {
        card_id: OwnedRoomId,
        target_space_id: OwnedRoomId,
        target_position: f64,
    },

    /// 取消拖拽
    CancelDragCard,

    /// 卡片移动失败（用于回滚）
    MoveCardFailed {
        card_id: OwnedRoomId,
        original_space_id: OwnedRoomId,
        original_position: f64,
        error: String,
    },

    /// 设置加载状态
    Loading(bool),

    /// 错误
    Error(String),
}
