use serde::{Deserialize, Serialize};
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};

/// 看板数据模型（简化版）
/// Board 是一个虚拟概念，用于组织多个 Space（列表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    /// 看板 ID（可以是一个特殊的 Room ID 或本地生成的 ID）
    pub id: String,

    /// 看板名称
    pub name: String,

    /// 看板描述
    pub description: Option<String>,

    /// Space ID 列表（每个 Space 是一个列表）
    pub space_ids: Vec<OwnedRoomId>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

impl KanbanBoard {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: None,
            space_ids: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// 看板列表（对应 Matrix Space）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanList {
    /// 列表 ID（对应 Matrix Space ID）
    pub id: OwnedRoomId,

    /// 列表名称
    pub name: String,

    /// 所属看板 ID
    pub board_id: String,

    /// 卡片 ID 列表（Space 的子 Room）
    pub card_ids: Vec<OwnedRoomId>,

    /// 排序位置
    pub position: f64,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 看板卡片（对应 Matrix Room）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCard {
    /// 卡片 ID（对应 Matrix Room ID）
    pub id: OwnedRoomId,

    /// 卡片标题
    pub title: String,

    /// 卡片描述
    pub description: Option<String>,

    /// 所属列表 ID（对应 Matrix Space ID）
    pub space_id: OwnedRoomId,

    /// 所属看板 ID
    pub board_id: String,

    /// 排序位置
    pub position: f64,

    /// 标签
    pub labels: Vec<String>,

    /// 成员 ID 列表
    pub member_ids: Vec<OwnedUserId>,

    /// 截止日期
    pub due_date: Option<String>,

    /// 是否已完成
    pub is_completed: bool,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

impl KanbanCard {
    pub fn new(title: &str, space_id: OwnedRoomId, board_id: String) -> Self {
        Self {
            id: OwnedRoomId::try_from("!temp:matrix.local").unwrap(), // 临时ID，创建后会被替换
            title: title.to_string(),
            description: None,
            space_id,
            board_id,
            position: 1000.0,
            labels: Vec::new(),
            member_ids: Vec::new(),
            due_date: None,
            is_completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
