use std::collections::HashMap;
use matrix_sdk::ruma::OwnedRoomId;
use serde::{Deserialize, Serialize};
use makepad_widgets::log;

/// Card 状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardStatus {
    /// 未完成状态
    Pending,
    
    /// 已完成状态
    Completed,
    
    /// 已归档状态
    Archived,
}

impl CardStatus {
    /// 获取状态的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            CardStatus::Pending => "未完成",
            CardStatus::Completed => "已完成",
            CardStatus::Archived => "已归档",
        }
    }
    
    /// 获取状态的颜色
    pub fn color(&self) -> &'static str {
        match self {
            CardStatus::Pending => "#FFA500",    // 橙色
            CardStatus::Completed => "#61BD4F",  // 绿色
            CardStatus::Archived => "#95A5A6",   // 灰色
        }
    }
}

impl Default for CardStatus {
    fn default() -> Self {
        CardStatus::Pending
    }
}

/// Space 标签定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceTag {
    /// 标签唯一 ID
    pub id: String,
    
    /// 标签名称
    pub name: String,
    
    /// 标签颜色（十六进制）
    pub color: String,
    
    /// 标签描述（可选）
    pub description: Option<String>,
    
    /// 创建时间（Unix timestamp 秒）
    pub created_at: u64,
    
    /// 最后更新时间（Unix timestamp 秒）
    pub updated_at: u64,
}

impl SpaceTag {
    /// 创建新的 Space 标签
    pub fn new(name: String, color: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let random = uuid::Uuid::new_v4().to_string();
        let id = format!("tag_{}_{}", now, &random[..8]);
        
        Self {
            id,
            name,
            color,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// 更新标签
    pub fn update(&mut self, name: Option<String>, color: Option<String>, description: Option<String>) {
        if let Some(n) = name {
            self.name = n;
        }
        if let Some(c) = color {
            self.color = c;
        }
        if description.is_some() {
            self.description = description;
        }
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// 预定义标签颜色
pub const PREDEFINED_TAG_COLORS: &[(&str, &str)] = &[
    ("红色", "#EB5A46"),
    ("橙色", "#FF9F1A"),
    ("黄色", "#F2D600"),
    ("绿色", "#61BD4F"),
    ("青色", "#00C2E0"),
    ("蓝色", "#0079BF"),
    ("紫色", "#9775FA"),
    ("粉色", "#FF78CB"),
    ("灰色", "#95A5A6"),
    ("黑色", "#343434"),
];

/// 简化的看板列表（对应 Matrix Space）
#[derive(Debug, Clone)]
pub struct KanbanList {
    /// 列表 ID（Space ID）
    pub id: OwnedRoomId,
    
    /// 列表名称
    pub name: String,
    
    /// 卡片 ID 列表
    pub card_ids: Vec<OwnedRoomId>,
    
    /// 排序位置
    pub position: f64,
}

/// 简化的看板卡片（对应 Matrix Room）
#[derive(Debug, Clone)]
pub struct KanbanCard {
    /// 卡片 ID（Room ID）
    pub id: OwnedRoomId,
    
    /// 卡片标题
    pub title: String,
    
    /// 卡片描述（支持 Markdown）
    pub description: Option<String>,
    
    /// 所属列表 ID（Space ID）
    pub space_id: OwnedRoomId,
    
    /// 排序位置（用于拖拽排序）
    pub position: f64,
    
    /// 卡片状态
    pub status: CardStatus,
    
    // ========== Phase 1: 基础元数据 ==========
    
    /// 标签列表
    pub tags: Vec<String>,
    
    /// 截止时间（Unix timestamp 秒）
    pub end_time: Option<u64>,
    
    // ========== Phase 2: TodoList ==========
    
    /// 待办事项列表
    pub todos: Vec<TodoItem>,
    
    /// 创建时间（Unix timestamp 秒）
    pub created_at: u64,
    
    /// 最后更新时间（Unix timestamp 秒）
    pub updated_at: u64,
}

impl KanbanCard {
    /// 创建新的 Card
    pub fn new(id: OwnedRoomId, title: String, space_id: OwnedRoomId) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id,
            title,
            description: None,
            space_id,
            position: 1000.0,
            status: CardStatus::default(),
            tags: Vec::new(),
            end_time: None,
            todos: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// 更新时间戳
    pub fn touch(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// 是否已过期
    pub fn is_overdue(&self) -> bool {
        if let Some(end_time) = self.end_time {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now > end_time
        } else {
            false
        }
    }
    
    /// 获取 Todo 完成进度 (completed, total)
    pub fn todo_progress(&self) -> (usize, usize) {
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let total = self.todos.len();
        (completed, total)
    }
}

/// 待办事项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    /// Todo 唯一标识符
    pub id: String,
    
    /// 待办内容
    pub text: String,
    
    /// 是否完成
    pub completed: bool,
    
    /// 创建时间（Unix timestamp 秒）
    pub created_at: u64,
    
    /// 完成时间（Unix timestamp 秒）
    pub completed_at: Option<u64>,
}

/// 活动记录类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Comment,
    StatusChange,
    TagAdded,
    TagRemoved,
    TodoAdded,
    TodoCompleted,
    TodoUncompleted,
    EndTimeSet,
    EndTimeRemoved,
    DescriptionChanged,
    TitleChanged,
}

/// 活动记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardActivity {
    /// 活动ID（Timeline Event ID）
    pub id: String,
    
    /// 活动类型
    pub activity_type: ActivityType,
    
    /// 活动文本内容
    pub text: String,
    
    /// 活动元数据（可选）
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    
    /// 创建时间（Unix timestamp 秒）
    pub created_at: u64,
    
    /// 创建者用户ID
    pub user_id: String,
}

impl TodoItem {
    /// 创建新的 Todo
    pub fn new(text: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // 生成唯一 ID: todo_{timestamp}_{random}
        let random = uuid::Uuid::new_v4().to_string();
        let id = format!("todo_{}_{}", now, &random[..8]);
        
        Self {
            id,
            text,
            completed: false,
            created_at: now,
            completed_at: None,
        }
    }
    
    /// 切换完成状态
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
        if self.completed {
            self.completed_at = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
        } else {
            self.completed_at = None;
        }
    }
}

/// 拖拽状态
#[derive(Debug, Clone)]
pub struct DragState {
    /// 被拖拽的卡片 ID
    pub card_id: OwnedRoomId,
    
    /// 卡片原始所属的 Space ID
    pub source_space_id: OwnedRoomId,
    
    /// 卡片在原列表中的位置
    pub source_position: f64,
    
    /// 拖拽开始时间（Unix timestamp 毫秒）
    pub start_time: u64,
}

/// 看板应用状态
#[derive(Debug, Clone, Default)]
pub struct KanbanAppState {
    /// 列表数据缓存（Space ID -> KanbanList）
    pub lists: HashMap<OwnedRoomId, KanbanList>,

    /// 卡片数据缓存（Room ID -> KanbanCard）
    pub cards: HashMap<OwnedRoomId, KanbanCard>,

    /// 活动记录缓存（Card ID -> Activities）
    pub activities: HashMap<OwnedRoomId, Vec<CardActivity>>,

    /// Space 标签库（Space ID -> Tags）
    pub space_tags: HashMap<OwnedRoomId, Vec<SpaceTag>>,

    /// 当前选中的卡片 ID（用于显示详情）
    pub selected_card_id: Option<OwnedRoomId>,

    /// 拖拽状态
    pub drag_state: Option<DragState>,

    /// 加载状态
    pub loading: bool,

    /// 错误信息
    pub error: Option<String>,
}

impl KanbanAppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取所有列表（按位置排序）
    pub fn all_lists(&self) -> Vec<&KanbanList> {
        let mut lists: Vec<&KanbanList> = self.lists.values().collect();
        lists.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        lists
    }

    /// 获取列表中的卡片
    pub fn list_cards(&self, space_id: &OwnedRoomId) -> Vec<&KanbanCard> {
        if let Some(list) = self.lists.get(space_id) {
            list.card_ids
                .iter()
                .filter_map(|card_id| self.cards.get(card_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 添加或更新列表
    pub fn upsert_list(&mut self, list: KanbanList) {
        // 如果列表已存在，保留现有的卡片 ID 和可能更新的名称
        if let Some(existing_list) = self.lists.get(&list.id) {
            let mut updated_list = list;
            
            // 如果新列表的卡片为空，保留现有卡片
            if updated_list.card_ids.is_empty() && !existing_list.card_ids.is_empty() {
                updated_list.card_ids = existing_list.card_ids.clone();
            }
            
            // 如果新列表名称是"新列表"（默认值），但现有列表有不同的名称，
            // 则保留现有名称（可能是用户刚刚编辑的）
            if updated_list.name == "新列表" && existing_list.name != "新列表" {
                log!("🔄 upsert_list: 保留现有列表名称 '{}' 而不是使用默认值 '新列表'", existing_list.name);
                updated_list.name = existing_list.name.clone();
            }
            
            self.lists.insert(updated_list.id.clone(), updated_list);
        } else {
            // 新列表，直接插入
            self.lists.insert(list.id.clone(), list);
        }
    }

    /// 只更新列表名称，保留现有卡片
    pub fn update_list_name(&mut self, list_id: &OwnedRoomId, new_name: String) {
        if let Some(list) = self.lists.get_mut(list_id) {
            list.name = new_name;
        }
    }

    /// 添加或更新卡片
    pub fn upsert_card(&mut self, card: KanbanCard) {
        self.cards.insert(card.id.clone(), card);
    }

    /// 设置加载状态
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    /// 设置错误信息
    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }
}
