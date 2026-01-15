use serde::{Deserialize, Serialize};
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};

/// 看板数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    /// 看板 ID (对应 Matrix Room ID)
    pub id: OwnedRoomId,

    /// 看板名称
    pub name: String,

    /// 看板描述
    pub description: Option<String>,

    /// 背景颜色
    pub background_color: String,

    /// 背景图片 URL
    pub background_image: Option<String>,

    /// 标签列表
    pub labels: Vec<KanbanLabel>,

    /// 成员 ID 列表
    pub member_ids: Vec<OwnedUserId>,

    /// 列表 ID 列表 (按顺序)
    pub list_ids: Vec<String>,

    /// 是否已归档
    pub is_archived: bool,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,

    /// 扩展数据 (本地存储)
    pub extensions: BoardExtensions,
}

impl Default for KanbanBoard {
    fn default() -> Self {
        Self {
            id: matrix_sdk::ruma::OwnedRoomId::try_from("!dummy:matrix.local").unwrap(),
            name: String::new(),
            description: None,
            background_color: "#0079BF".to_string(),
            background_image: None,
            labels: Vec::new(),
            member_ids: Vec::new(),
            list_ids: Vec::new(),
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            extensions: BoardExtensions::default(),
        }
    }
}

impl KanbanBoard {
    /// 创建新看板
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }

    /// 获取成员数量
    pub fn member_count(&self) -> usize {
        self.member_ids.len()
    }

    /// 获取列表数量
    pub fn list_count(&self) -> usize {
        self.list_ids.len()
    }
}

/// 看板标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanLabel {
    /// 标签 ID
    pub id: String,

    /// 标签名称
    pub name: String,

    /// 标签颜色
    pub color: LabelColor,
}

/// 标签颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabelColor {
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Blue,
    Sky,
    Lime,
    Pink,
    Black,
}

impl LabelColor {
    pub fn to_hex(&self) -> &'static str {
        match self {
            LabelColor::Green => "#61BD4F",
            LabelColor::Yellow => "#F2D600",
            LabelColor::Orange => "#FF9F1A",
            LabelColor::Red => "#EB5A46",
            LabelColor::Purple => "#9775FA",
            LabelColor::Blue => "#0079BF",
            LabelColor::Sky => "#00C2E0",
            LabelColor::Lime => "#51E898",
            LabelColor::Pink => "#FF78CB",
            LabelColor::Black => "#343434",
        }
    }
}

/// 看板扩展数据 (仅本地存储)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardExtensions {
    /// 视图设置
    pub view_settings: ViewSettings,
    /// 过滤状态
    pub filter_state: Option<KanbanFilterState>,
    /// 排序状态
    pub sort_state: Option<KanbanSortState>,
}

/// 视图设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSettings {
    /// 卡片显示方式
    pub card_view_mode: CardViewMode,
    /// 显示已完成卡片
    pub show_completed: bool,
    /// 每页卡片数
    pub page_size: u32,
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            card_view_mode: CardViewMode::Detailed,
            show_completed: true,
            page_size: 50,
        }
    }
}

/// 卡片视图模式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CardViewMode {
    Compact,  // 紧凑模式
    Detailed, // 详细模式
    Cover,    // 封面模式
}

/// 看板列表数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanList {
    /// 列表 ID
    pub id: String,

    /// 列表名称
    pub name: String,

    /// 看板 ID
    pub board_id: OwnedRoomId,

    /// 排序位置
    pub position: f64,

    /// 是否已归档
    pub is_archived: bool,

    /// 卡片 ID 列表 (按顺序)
    pub card_ids: Vec<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

impl Default for KanbanList {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            board_id: matrix_sdk::ruma::OwnedRoomId::try_from("!dummy:matrix.local").unwrap(),
            position: 1000.0,
            is_archived: false,
            card_ids: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl KanbanList {
    pub fn new(name: &str, board_id: OwnedRoomId) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            board_id,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }

    pub fn card_count(&self) -> usize {
        self.card_ids.len()
    }
}

/// 看板卡片数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCard {
    /// 卡片 ID
    pub id: String,

    /// 卡片标题
    pub title: String,

    /// 卡片描述
    pub description: Option<String>,

    /// 所属列表 ID
    pub list_id: String,

    /// 所属看板 ID
    pub board_id: OwnedRoomId,

    /// 排序位置
    pub position: f64,

    /// 标签 ID 列表
    pub label_ids: Vec<String>,

    /// 成员 ID 列表
    pub member_ids: Vec<OwnedUserId>,

    /// 截止日期
    pub due_date: Option<CardDueDate>,

    /// 封面图片
    pub cover: Option<CardCover>,

    /// 附件数量
    pub attachment_count: u32,

    /// 评论数量
    pub comment_count: u32,

    /// 检查清单
    pub checklists: Vec<CardChecklist>,

    /// 是否已加星
    pub is_starred: bool,

    /// 是否已归档
    pub is_archived: bool,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

impl Default for KanbanCard {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            description: None,
            list_id: String::new(),
            board_id: matrix_sdk::ruma::OwnedRoomId::try_from("!dummy:matrix.local").unwrap(),
            position: 1000.0,
            label_ids: Vec::new(),
            member_ids: Vec::new(),
            due_date: None,
            cover: None,
            attachment_count: 0,
            comment_count: 0,
            checklists: Vec::new(),
            is_starred: false,
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl KanbanCard {
    pub fn new(title: &str, list_id: String, board_id: OwnedRoomId) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            list_id,
            board_id,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }

    pub fn is_completed(&self) -> bool {
        self.checklists.iter().all(|cl| cl.is_completed())
    }
}

/// 卡片截止日期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDueDate {
    pub date: String,
    pub is_completed: bool,
}

/// 卡片封面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCover {
    pub url: String,
    pub height: u32,
}

/// 卡片检查清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardChecklist {
    pub id: String,
    pub name: String,
    pub items: Vec<ChecklistItem>,
}

impl CardChecklist {
    pub fn is_completed(&self) -> bool {
        self.items.iter().all(|item| item.is_checked)
    }

    pub fn completed_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_checked).count()
    }
}

/// 检查清单项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: String,
    pub name: String,
    pub is_checked: bool,
}

/// 看板过滤状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanFilterState {
    pub keyword: Option<String>,
    pub label_ids: Vec<String>,
    pub member_ids: Vec<OwnedUserId>,
    pub due_date: Option<DueDateFilter>,
}

/// 截止日期过滤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DueDateFilter {
    Overdue,
    Today,
    Tomorrow,
    ThisWeek,
    NextWeek,
    NoDue,
    Completed,
}

/// 看板排序状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanSortState {
    pub field: SortField,
    pub direction: SortDirection,
}

/// 排序字段
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortField {
    Position,
    CreatedAt,
    UpdatedAt,
    Title,
    DueDate,
}

/// 排序方向
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}
