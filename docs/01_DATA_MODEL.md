# 数据模型设计文档

> Toona 项目改造 - 数据模型详细设计

## 文档信息

- **版本**: 1.0
- **创建日期**: 2026-01-14
- **状态**: 设计稿

---

## 目录

1. [设计原则](#1-设计原则)
2. [核心数据结构](#2-核心数据结构)
3. [扩展字段定义](#3-扩展字段定义)
4. [仓储层设计](#4-仓储层设计)
5. [状态管理](#5-状态管理)
6. [持久化方案](#6-持久化方案)

---

## 1. 设计原则

### 1.1 设计目标

- **最小化 Matrix API 扩展**: 尽可能复用现有 API，不修改后端
- **清晰的领域划分**: 分离 Board、List、Card 的职责
- **高效的数据访问**: 设计合理的仓储层，优化查询性能
- **可扩展性**: 支持未来添加新字段和新功能
- **类型安全**: 使用 Rust 的强类型系统

### 1.2 数据分层

```
┌─────────────────────────────────────────────────────────────────┐
│                        数据分层架构                              │
├─────────────────────────────────────────────────────────────────┤
│  UI 层 (View)                                                   │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  KanbanBoardView    │    KanbanListView    │ CardView       ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  状态层 (State)                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  KanbanAppState     │    BoardState        │ ListState      ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  领域层 (Domain)                                                │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  KanbanBoard        │    KanbanList        │ KanbanCard     ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  基础设施层 (Infrastructure)                                     │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  BoardRepository    │    ListRepository    │ CardRepository ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  存储层 (Storage)                                               │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Matrix SDK (Room/Message)  │  Local Storage (排序/缓存)    ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### 1.3 数据来源

| 数据类型 | 来源 | 同步方式 |
|---------|------|----------|
| 看板基本信息 | Matrix Room | 实时同步 |
| 看板成员 | Matrix Room Members | 实时同步 |
| 看板标签 | Matrix Room Tags | 实时同步 |
| 卡片信息 | Matrix Message | 实时同步 |
| 列表结构 | Local Storage | 手动同步 |
| 排序索引 | Local Storage | 手动同步 |
| 截止日期 | Message Extension | 实时同步 |
| 筛选状态 | Session Storage | 临时存储 |

---

## 2. 核心数据结构

### 2.1 看板 (KanbanBoard)

看板是最高层级的数据结构，对应 Matrix 中的一个 Room。

```rust
// src/kanban/board.rs

use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use makepad_widgets::*;
use matrix_sdk::ruma::{
    OwnedRoomId, 
    RoomId, 
    CanonicalAlias,
    RoomJoinRule,
    UInt,
};

/// 看板元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    /// 看板唯一标识 (对应 Room ID)
    pub id: OwnedRoomId,
    
    /// 看板名称 (对应 Room 名称)
    pub name: String,
    
    /// 看板描述 (对应 Room Topic)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// 看板图标/头像 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    
    /// 看板背景颜色
    #[serde(default = "default_background_color")]
    pub background_color: String,
    
    /// 背景图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    
    /// 标签定义列表
    #[serde(default)]
    pub labels: Vec<BoardLabel>,
    
    /// 成员 ID 列表
    #[serde(default)]
    pub member_ids: Vec<String>,
    
    /// 管理员 ID 列表
    #[serde(default)]
    pub admin_ids: Vec<String>,
    
    /// 列表 ID 列表 (应用层维护)
    #[serde(default)]
    pub list_ids: Vec<String>,
    
    /// 排序方式
    #[serde(default)]
    pub sort_method: BoardSortMethod,
    
    /// 看板可见性
    #[serde(default)]
    pub visibility: BoardVisibility,
    
    /// 是否已归档
    #[serde(default)]
    pub is_archived: bool,
    
    /// 创建者 ID
    pub created_by: String,
    
    /// 创建时间 (ISO 8601)
    pub created_at: String,
    
    /// 最后更新时间 (ISO 8601)
    pub updated_at: String,
}

fn default_background_color() -> String {
    "#0079BF".to_string() // Trello 默认蓝色
}

/// 看板标签定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoardLabel {
    /// 标签 ID
    pub id: String,
    
    /// 标签名称
    pub name: String,
    
    /// 标签颜色
    pub color: LabelColor,
    
    /// 使用次数 (用于排序)
    #[serde(default)]
    pub usage_count: u32,
}

impl Default for BoardLabel {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            color: LabelColor::Blue,
            usage_count: 0,
        }
    }
}

/// 标签颜色枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
            LabelColor::Purple => "#C377E0",
            LabelColor::Blue => "#0079BF",
            LabelColor::Sky => "#00C2E0",
            LabelColor::Lime => "#51E898",
            LabelColor::Pink => "#FF78CB",
            LabelColor::Black => "#344563",
        }.clone()
    }
}

/// 看板排序方式
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoardSortMethod {
    #[default]
    Manual,    // 手动排序
    Alphabetical, // 字母顺序
    CreatedAt, // 创建时间
    UpdatedAt, // 更新时间
}

/// 看板可见性
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoardVisibility {
    #[default]
    Private,   // 私有
    Workspace, // 工作区可见
    Public,    // 公开
}

/// 看板响应式状态 (用于 UI)
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_state(panic_recovery)]
pub struct KanbanBoardState {
    /// 当前看板
    #[live]
    pub current_board: Option<KanbanBoard>,
    
    /// 看板列表
    #[live]
    pub boards: Vec<KanbanBoard>,
    
    /// 加载状态
    #[live]
    pub is_loading: bool,
    
    /// 错误信息
    #[live]
    pub error: Option<String>,
    
    /// 选中的列表 ID
    #[live]
    pub selected_list_id: Option<String>,
    
    /// 选中的卡片 ID
    #[live]
    pub selected_card_id: Option<String>,
    
    /// 筛选状态
    #[live]
    pub filter: BoardFilter,
}

/// 看板筛选条件
#[derive(Debug, Clone, Default, Serialize, Deserialize, LiveHook)]
pub struct BoardFilter {
    /// 成员筛选
    pub member_ids: Vec<String>,
    
    /// 标签筛选
    pub label_ids: Vec<String>,
    
    /// 关键词搜索
    pub keyword: String,
    
    /// 截止日期筛选
    pub due_date_filter: DueDateFilter,
    
    /// 是否只显示我负责的
    pub assigned_to_me: bool,
    
    /// 是否只显示未完成的
    pub show_uncompleted: bool,
}

/// 截止日期筛选
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum DueDateFilter {
    #[default]
    All,
    Overdue,        // 已过期
    Today,          // 今天
    ThisWeek,       // 本周
    ThisMonth,      // 本月
    NoDueDate,      // 无截止日期
}
```

### 2.2 看板列表 (KanbanList)

列表是看板中的列，用于组织卡片。

```rust
// src/kanban/list.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::kanban::card::KanbanCard;

/// 看板列表 (应用层数据结构)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanbanList {
    /// 列表 ID (UUID，应用层生成)
    pub id: String,
    
    /// 所属看板 ID
    pub board_id: String,
    
    /// 列表名称
    pub name: String,
    
    /// 列表颜色 (用于区分)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    
    /// 排序索引 (用于拖拽排序)
    pub order_index: f64,
    
    /// 卡片列表 (懒加载，不存储完整数据)
    #[serde(default)]
    pub cards: Vec<KanbanCard>,
    
    /// 卡片数量
    #[serde(default)]
    pub card_count: u32,
    
    /// 待完成卡片数量
    #[serde(default)]
    pub incomplete_card_count: u32,
    
    /// 卡片数量限制 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_limit: Option<u32>,
    
    /// 是否已归档
    #[serde(default)]
    pub is_archived: bool,
    
    /// 是否是系统列表 (如"已完成")
    #[serde(default)]
    pub is_system: bool,
    
    /// 创建时间
    pub created_at: String,
    
    /// 最后更新时间
    pub updated_at: String,
}

impl KanbanList {
    /// 创建新列表
    pub fn new(board_id: &str, name: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            board_id: board_id.to_string(),
            name: name.to_string(),
            order_index: 0.0,
            cards: Vec::new(),
            card_count: 0,
            incomplete_card_count: 0,
            is_archived: false,
            is_system: false,
            created_at: now.clone(),
            updated_at: now,
        }
    }
    
    /// 创建"已完成"系统列表
    pub fn create_completed_list(board_id: &str) -> Self {
        let mut list = Self::new(board_id, "已完成");
        list.is_system = true;
        list.color = Some("#61BD4F".to_string());
        list
    }
    
    /// 检查是否达到卡片限制
    pub fn is_at_limit(&self) -> bool {
        self.card_limit.map_or(false, |limit| self.card_count >= limit)
    }
}

/// 列表响应式状态
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_state(panic_recovery)]
pub struct KanbanListState {
    /// 当前列表
    #[live]
    pub current_list: Option<KanbanList>,
    
    /// 看板中的所有列表
    #[live]
    pub lists: Vec<KanbanList>,
    
    /// 正在编辑的列表 ID
    #[live]
    pub editing_list_id: Option<String>,
    
    /// 新卡片输入框可见性
    #[live]
    pub show_new_card_input: bool,
    
    /// 新卡片标题
    #[live]
    pub new_card_title: String,
}

/// 列表移动操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMoveOperation {
    pub list_id: String,
    pub from_position: f64,
    pub to_position: f64,
}
```

### 2.3 看板卡片 (KanbanCard)

卡片是看板中的基本单元，对应 Matrix 中的一个 Message。

```rust
// src/kanban/card.rs

use serde::{Deserialize, Serialize};
use matrix_sdk::ruma::{
    OwnedEventId, 
    EventId,
    OwnedUserId,
    DateTime,
};
use crate::kanban::card_extensions::CardExtensions;

/// 看板卡片 (对应 Matrix Message)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCard {
    /// 卡片 ID (对应 Event ID)
    pub id: OwnedEventId,
    
    /// 卡片标题 (消息的 body)
    pub title: String,
    
    /// 卡片描述 (消息内容，可能包含格式化)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// 原始消息类型
    pub message_type: CardMessageType,
    
    /// 所属列表 ID
    pub list_id: String,
    
    /// 排序索引 (应用层维护)
    pub order_index: f64,
    
    /// 标签 ID 列表
    #[serde(default)]
    pub label_ids: Vec<String>,
    
    /// 负责人 ID 列表 (@提及)
    #[serde(default)]
    pub member_ids: Vec<String>,
    
    /// 创建者 ID
    pub created_by: String,
    
    /// 截止日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<CardDueDate>,
    
    /// 封面图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<CardCover>,
    
    /// 附件列表
    #[serde(default)]
    pub attachments: Vec<CardAttachment>,
    
    /// 检查清单列表
    #[serde(default)]
    pub checklists: Vec<CardChecklist>,
    
    /// 活动记录
    #[serde(default)]
    pub activities: Vec<CardActivity>,
    
    /// 评论数量
    #[serde(default)]
    pub comment_count: u32,
    
    /// 已读成员 ID 列表
    #[serde(default)]
    pub read_member_ids: Vec<String>,
    
    /// 是否加星标
    #[serde(default)]
    pub is_starred: bool,
    
    /// 是否已归档
    #[serde(default)]
    pub is_archived: bool,
    
    /// 创建时间
    pub created_at: String,
    
    /// 最后更新时间
    pub updated_at: String,
}

/// 卡片消息类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CardMessageType {
    Text,          // 文本消息
    Image,         // 图片
    File,          // 文件
    Link,          // 链接
    Checklist,     // 检查清单
    System,        // 系统消息 (如卡片移动)
}

/// 卡片截止日期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDueDate {
    /// 截止日期时间
    pub due_date: String,
    
    /// 是否已完成
    #[serde(default)]
    pub is_completed: bool,
    
    /// 是否已提醒
    #[serde(default)]
    pub is_reminded: bool,
    
    /// 提醒时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_time: Option<String>,
}

/// 卡片封面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCover {
    /// 封面图片 URL
    pub url: String,
    
    /// 封面颜色 (作为占位符)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    
    /// 是否全宽
    #[serde(default)]
    pub is_full_width: bool,
    
    /// 亮度
    #[serde(default)]
    pub brightness: u8,
}

/// 卡片附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAttachment {
    /// 附件 ID
    pub id: String,
    
    /// 附件名称
    pub name: String,
    
    /// 附件 URL
    pub url: String,
    
    /// MIME 类型
    pub content_type: String,
    
    /// 文件大小 (字节)
    pub size: u64,
    
    /// 缩略图 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    
    /// 添加者 ID
    pub added_by: String,
    
    /// 添加时间
    pub created_at: String,
}

/// 卡片检查清单
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardChecklist {
    /// 检查清单 ID
    pub id: String,
    
    /// 检查清单名称
    pub name: String,
    
    /// 检查项列表
    pub items: Vec<ChecklistItem>,
    
    /// 创建时间
    pub created_at: String,
}

/// 检查清单项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    /// 项 ID
    pub id: String,
    
    /// 项内容
    pub text: String,
    
    /// 是否已完成
    #[serde(default)]
    pub is_completed: bool,
    
    /// 完成者 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<String>,
    
    /// 完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// 卡片活动记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardActivity {
    /// 活动 ID
    pub id: String,
    
    /// 活动类型
    pub activity_type: ActivityType,
    
    /// 活动描述
    pub description: String,
    
    /// 执行者 ID
    pub actor_id: String,
    
    /// 活动时间
    pub created_at: String,
    
    /// 附加数据 (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// 活动类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivityType {
    Created,           // 创建卡片
    Updated,           // 更新卡片
    Moved,             // 移动卡片
    Archived,          // 归档卡片
    Unarchived,        // 取消归档
    MemberAdded,       // 添加成员
    MemberRemoved,     // 移除成员
    LabelAdded,        // 添加标签
    LabelRemoved,      // 移除标签
    DueDateSet,        // 设置截止日期
    DueDateChanged,    // 修改截止日期
    DueDateCompleted,  // 截止日期完成
    AttachmentAdded,   // 添加附件
    AttachmentRemoved, // 移除附件
    ChecklistAdded,    // 添加检查清单
    CommentAdded,      // 添加评论
    CommentEdited,     // 编辑评论
    CommentDeleted,    // 删除评论
    Copied,            // 复制卡片
    Deleted,           // 删除卡片
}

/// 卡片响应式状态
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_state(panic_recovery)]
pub struct KanbanCardState {
    /// 当前卡片
    #[live]
    pub current_card: Option<KanbanCard>,
    
    /// 正在查看的卡片 ID
    #[live]
    pub viewing_card_id: Option<OwnedEventId>,
    
    /// 正在编辑的卡片 ID
    #[live]
    pub editing_card_id: Option<OwnedEventId>,
    
    /// 卡片详情弹窗可见性
    #[live]
    pub show_card_modal: bool,
    
    /// 编辑模式
    #[live]
    pub edit_mode: CardEditMode,
    
    /// 加载状态
    #[live]
    pub is_loading: bool,
}

/// 卡片编辑模式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, LiveHook)]
pub enum CardEditMode {
    View,       // 查看模式
    Title,      // 编辑标题
    Description, // 编辑描述
    Labels,     // 编辑标签
    Members,    // 编辑成员
    DueDate,    // 编辑截止日期
    Attachments, // 编辑附件
    Checklists, // 编辑检查清单
    Comments,   // 编辑评论
}
```

### 2.4 成员数据

```rust
// src/kanban/member.rs

use serde::{Deserialize, Serialize};
use matrix_sdk::ruma::OwnedUserId;

/// 看板成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardMember {
    /// 用户 ID
    pub id: OwnedUserId,
    
    /// 显示名称
    pub display_name: String,
    
    /// 头像 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    
    /// 成员类型
    pub member_type: MemberType,
    
    /// 角色
    pub role: MemberRole,
    
    /// 加入时间
    pub joined_at: String,
}

impl BoardMember {
    pub fn from_room_member(member: &RoomMember) -> Self {
        Self {
            id: member.user_id().to_owned(),
            display_name: member.display_name().unwrap_or_default().to_string(),
            avatar_url: member.avatar_url().map(|u| u.to_string()),
            member_type: MemberType::Normal,
            role: MemberRole::Member,
            joined_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// 成员类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemberType {
    Normal,     // 普通成员
    Bot,        // 机器人
    Service,    // 服务账户
}

/// 成员角色
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemberRole {
    #[default]
    Member,     // 普通成员
    Admin,      // 管理员
    Owner,      // 所有者
}
```

---

## 3. 扩展字段定义

### 3.1 卡片扩展字段

用于存储 Matrix 协议不支持的字段。

```rust
// src/kanban/card_extensions.rs

use serde::{Deserialize, Serialize};
use crate::kanban::card::ActivityType;

/// 卡片扩展字段 (存储在 Message 的 extensions 中)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardExtensions {
    /// 卡片所属列表 ID
    pub list_id: String,
    
    /// 排序索引
    pub order_index: f64,
    
    /// 标签 ID 列表
    #[serde(default)]
    pub label_ids: Vec<String>,
    
    /// 负责人 ID 列表
    #[serde(default)]
    pub member_ids: Vec<String>,
    
    /// 截止日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<ExtensionDueDate>,
    
    /// 封面
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<ExtensionCover>,
    
    /// 附件数量
    #[serde(default)]
    pub attachment_count: u32,
    
    /// 检查清单进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_progress: Option<ChecklistProgress>,
    
    /// 是否加星标
    #[serde(default)]
    pub is_starred: bool,
    
    /// 源卡片 ID (复制/移动时使用)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_card_id: Option<String>,
    
    /// 活动记录 (简化版)
    #[serde(default)]
    pub activities: Vec<ExtensionActivity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionDueDate {
    pub due_date: String,
    #[serde(default)]
    pub is_completed: bool,
    #[serde(default)]
    pub is_reminded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCover {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    pub is_full_width: bool,
    #[serde(default)]
    pub brightness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistProgress {
    pub checklist_id: String,
    pub total: u32,
    pub completed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionActivity {
    pub activity_type: ActivityType,
    pub actor_id: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
```

### 3.2 列表扩展字段

```rust
// src/kanban/list_extensions.rs

use serde::{Deserialize, Serialize};

/// 列表扩展字段 (存储在本地)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListExtensions {
    /// 列表颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    
    /// 卡片数量限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_limit: Option<u32>,
    
    /// 列表图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    
    /// 自定义样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ListStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListStyle {
    /// 背景颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    
    /// 字体颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    
    /// 边框颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
}
```

### 3.3 看板扩展字段

```rust
// src/kanban/board_extensions.rs

use serde::{Deserialize, Serialize};
use crate::kanban::BoardSortMethod;

/// 看板扩展字段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoardExtensions {
    /// 背景图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    
    /// 背景缩放模式
    #[serde(default)]
    pub background_fit: BackgroundFit,
    
    /// 排序方式
    #[serde(default)]
    pub sort_method: BoardSortMethod,
    
    /// 快捷键配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<BoardShortcuts>,
    
    /// 视图配置
    #[serde(default)]
    pub view_settings: BoardViewSettings,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackgroundFit {
    #[default]
    Fill,      // 填充
    Fit,       // 适应
    Tile,      // 平铺
    Stretch,   // 拉伸
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardShortcuts {
    #[serde(default)]
    pub quick_add: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub archive: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardViewSettings {
    /// 显示卡片计数
    #[serde(default)]
    pub show_card_counts: bool,
    
    /// 显示标签
    #[serde(default)]
    pub show_labels: bool,
    
    /// 显示成员头像
    #[serde(default)]
    pub show_member_avatars: bool,
    
    /// 显示截止日期
    #[serde(default)]
    pub show_due_dates: bool,
    
    /// 紧凑模式
    #[serde(default)]
    pub compact_mode: bool,
}
```

---

## 4. 仓储层设计

### 4.1 仓储接口定义

```rust
// src/kanban/repository/traits.rs

use async_trait::async_trait;
use super::*;

/// 看板仓储接口
#[async_trait]
pub trait BoardRepositoryTrait {
    /// 获取所有看板
    async fn get_all_boards(&self) -> Result<Vec<KanbanBoard>, String>;
    
    /// 获取看板
    async fn get_board(&self, board_id: &RoomId) -> Result<KanbanBoard, String>;
    
    /// 创建看板
    async fn create_board(
        &self, 
        name: String, 
        description: Option<String>
    ) -> Result<KanbanBoard, String>;
    
    /// 更新看板
    async fn update_board(
        &self, 
        board_id: &RoomId, 
        updates: BoardUpdates
    ) -> Result<KanbanBoard, String>;
    
    /// 删除看板
    async fn delete_board(&self, board_id: &RoomId) -> Result<(), String>;
    
    /// 归档看板
    async fn archive_board(&self, board_id: &RoomId) -> Result<(), String>;
}

/// 列表仓储接口
#[async_trait]
pub trait ListRepositoryTrait {
    /// 获取列表
    async fn get_lists(&self, board_id: &RoomId) -> Result<Vec<KanbanList>, String>;
    
    /// 获取列表
    async fn get_list(&self, board_id: &RoomId, list_id: &str) -> Result<KanbanList, String>;
    
    /// 创建列表
    async fn create_list(
        &self, 
        board_id: &RoomId, 
        name: String
    ) -> Result<KanbanList, String>;
    
    /// 更新列表
    async fn update_list(
        &self, 
        board_id: &RoomId, 
        list_id: &str, 
        updates: ListUpdates
    ) -> Result<KanbanList, String>;
    
    /// 删除列表
    async fn delete_list(
        &self, 
        board_id: &RoomId, 
        list_id: &str
    ) -> Result<(), String>;
    
    /// 移动列表
    async fn move_list(
        &self, 
        board_id: &RoomId, 
        list_id: &str, 
        new_position: f64
    ) -> Result<(), String>;
}

/// 卡片仓储接口
#[async_trait]
pub trait CardRepositoryTrait {
    /// 获取卡片
    async fn get_cards(
        &self, 
        board_id: &RoomId, 
        list_id: &str
    ) -> Result<Vec<KanbanCard>, String>;
    
    /// 获取卡片
    async fn get_card(
        &self, 
        board_id: &RoomId, 
        card_id: &EventId
    ) -> Result<KanbanCard, String>;
    
    /// 创建卡片
    async fn create_card(
        &self, 
        board_id: &RoomId, 
        list_id: &str, 
        title: String
    ) -> Result<KanbanCard, String>;
    
    /// 更新卡片
    async fn update_card(
        &self, 
        board_id: &RoomId, 
        card_id: &EventId, 
        updates: CardUpdates
    ) -> Result<KanbanCard, String>;
    
    /// 删除卡片
    async fn delete_card(
        &self, 
        board_id: &RoomId, 
        card_id: &EventId
    ) -> Result<(), String>;
    
    /// 移动卡片
    async fn move_card(
        &self, 
        board_id: &RoomId, 
        card_id: &EventId, 
        from_list_id: &str, 
        to_list_id: &str, 
        new_position: f64
    ) -> Result<(), String>;
    
    /// 批量移动卡片
    async fn batch_move_cards(
        &self, 
        board_id: &RoomId, 
        moves: Vec<CardMoveOperation>
    ) -> Result<(), String>;
}

/// 看板更新
#[derive(Debug, Clone, Default)]
pub struct BoardUpdates {
    pub name: Option<String>,
    pub description: Option<String>,
    pub background_color: Option<String>,
    pub background_image: Option<String>,
    pub visibility: Option<BoardVisibility>,
}

/// 列表更新
#[derive(Debug, Clone, Default)]
pub struct ListUpdates {
    pub name: Option<String>,
    pub color: Option<String>,
    pub card_limit: Option<Option<u32>>,
}

/// 卡片更新
#[derive(Debug, Clone, Default)]
pub struct CardUpdates {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub label_ids: Option<Vec<String>>,
    pub member_ids: Option<Vec<String>>,
    pub due_date: Option<Option<CardDueDate>>,
    pub cover: Option<Option<CardCover>>,
    pub is_archived: Option<bool>,
    pub is_starred: Option<bool>,
}

/// 卡片移动操作
#[derive(Debug, Clone)]
pub struct CardMoveOperation {
    pub card_id: OwnedEventId,
    pub from_list_id: String,
    pub to_list_id: String,
    pub new_position: f64,
}
```

### 4.2 仓储实现

```rust
// src/kanban/repository/mod.rs

mod traits;
pub use traits::*;

mod board_repository;
pub use board_repository::BoardRepository;

mod list_repository;
pub use list_repository::ListRepository;

mod card_repository;
pub use card_repository::CardRepository;

/// 仓储工厂
pub struct RepositoryFactory {
    app_state: Rc<AppState>,
}

impl RepositoryFactory {
    pub fn new(app_state: Rc<AppState>) -> Self {
        Self { app_state }
    }
    
    pub fn board_repository(&self) -> BoardRepository {
        BoardRepository::new(self.app_state.clone())
    }
    
    pub fn list_repository(&self) -> ListRepository {
        ListRepository::new(self.app_state.clone())
    }
    
    pub fn card_repository(&self) -> CardRepository {
        CardRepository::new(self.app_state.clone())
    }
}
```

---

## 5. 状态管理

### 5.1 全局状态

```rust
// src/kanban/kanban_state.rs

use std::rc::Rc;
use super::*;

/// 看板应用全局状态
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_state(panic_recovery)]
pub struct KanbanAppState {
    /// 看板状态
    #[live]
    pub board_state: KanbanBoardState,
    
    /// 列表状态
    #[live]
    pub list_state: KanbanListState,
    
    /// 卡片状态
    #[live]
    pub card_state: KanbanCardState,
    
    /// 拖拽状态
    #[live]
    pub drag_drop_state: DragDropState,
    
    /// 筛选状态
    #[live]
    pub filter_state: KanbanFilterState,
    
    /// UI 状态
    #[live]
    pub ui_state: KanbanUIState,
}

impl KanbanAppState {
    pub fn new() -> Self {
        Self {
            board_state: KanbanBoardState::default(),
            list_state: KanbanListState::default(),
            card_state: KanbanCardState::default(),
            drag_drop_state: DragDropState::default(),
            filter_state: KanbanFilterState::default(),
            ui_state: KanbanUIState::default(),
        }
    }
    
    /// 获取当前看板
    pub fn current_board(&self) -> Option<&KanbanBoard> {
        self.board_state.current_board.as_ref()
    }
    
    /// 获取当前列表
    pub fn current_list(&self) -> Option<&KanbanList> {
        self.list_state.current_list.as_ref()
    }
    
    /// 获取当前卡片
    pub fn current_card(&self) -> Option<&KanbanCard> {
        self.card_state.current_card.as_ref()
    }
}

/// 拖拽状态
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_state(panic_recovery)]
pub struct DragDropState {
    /// 是否正在拖拽
    #[live]
    pub is_dragging: bool,
    
    /// 拖拽类型
    #[live]
    pub drag_type: DragType,
    
    /// 拖拽源
    #[live]
    pub drag_source: Option<DragSource>,
    
    /// 拖拽目标
    #[live]
    pub drop_target: Option<DropTarget>,
    
    /// 拖拽预览组件
    #[live]
    pub drag_preview: Option<LivePtr>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, LiveHook)]
pub enum DragType {
    None,
    Card,   // 拖拽卡片
    List,   // 拖拽列表
}

/// 拖拽源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragSource {
    pub board_id: OwnedRoomId,
    pub list_id: String,
    pub card_id: Option<String>,
    pub position: f64,
}

/// 放置目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropTarget {
    pub board_id: OwnedRoomId,
    pub list_id: String,
    pub card_id: Option<String>,
    pub position: DropPosition,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DropPosition {
    Before,      // 放置在目标之前
    After,       // 放置在目标之后
    Into,        // 放置在目标内部
    Empty,       // 放置在空列表
}

/// 筛选状态
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_state(panic_recovery)]
pub struct KanbanFilterState {
    /// 成员筛选
    #[live]
    pub member_filter: Vec<String>,
    
    /// 标签筛选
    #[live]
    pub label_filter: Vec<String>,
    
    /// 关键词搜索
    #[live]
    pub keyword: String,
    
    /// 截止日期筛选
    #[live]
    pub due_date_filter: DueDateFilter,
    
    /// 是否激活
    #[live]
    pub is_active: bool,
}

impl Default for KanbanFilterState {
    fn default() -> Self {
        Self {
            member_filter: Vec::new(),
            label_filter: Vec::new(),
            keyword: String::new(),
            due_date_filter: DueDateFilter::All,
            is_active: false,
        }
    }
}

/// UI 状态
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_state(panic_recovery)]
pub struct KanbanUIState {
    /// 是否显示侧边栏
    #[live]
    pub show_sidebar: bool,
    
    /// 当前视图模式
    #[live]
    pub view_mode: KanbanViewMode,
    
    /// 是否显示归档
    #[live]
    pub show_archived: bool,
    
    /// 批量选择模式
    #[live]
    pub bulk_select_mode: bool,
    
    /// 选中的卡片 ID 列表
    #[live]
    pub selected_card_ids: Vec<OwnedEventId>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, LiveHook)]
pub enum KanbanViewMode {
    Board,   // 看板视图
    List,    // 列表视图
    Calendar, // 日历视图
    Timeline, // 时间线视图
}
```

### 5.2 状态持久化

```rust
// src/kanban/persistence.rs

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::*;

/// 看板持久化存储
pub struct KanbanPersistence {
    /// 存储目录
    storage_dir: PathBuf,
}

impl KanbanPersistence {
    pub fn new(storage_dir: PathBuf) -> Self {
        Self { storage_dir }
    }
    
    /// 获取存储路径
    fn get_storage_path(&self, board_id: &RoomId) -> PathBuf {
        self.storage_dir.join(format!("kanban_{}.json", board_id))
    }
    
    /// 保存看板数据
    pub async fn save_board(
        &self, 
        board_id: &RoomId, 
        data: &KanbanPersistenceData
    ) -> Result<(), String> {
        let path = self.get_storage_path(board_id);
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| e.to_string())?;
        
        tokio::fs::write(&path, json)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    /// 加载看板数据
    pub async fn load_board(
        &self, 
        board_id: &RoomId
    ) -> Result<Option<KanbanPersistenceData>, String> {
        let path = self.get_storage_path(board_id);
        
        if !path.exists() {
            return Ok(None);
        }
        
        let json = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| e.to_string())?;
        
        serde_json::from_str(&json)
            .map_err(|e| e.to_string())
            .map(Some)
    }
    
    /// 删除看板数据
    pub async fn delete_board(&self, board_id: &RoomId) -> Result<(), String> {
        let path = self.get_storage_path(board_id);
        
        if path.exists() {
            tokio::fs::remove_file(&path)
                .await
                .map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }
    
    /// 获取所有看板数据文件
    pub async fn list_boards(&self) -> Result<Vec<RoomId>, String> {
        let mut boards = Vec::new();
        
        let entries = tokio::fs::read_dir(&self.storage_dir)
            .await
            .map_err(|e| e.to_string())?;
        
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            if file_name.starts_with("kanban_") && file_name.ends_with(".json") {
                let room_id_str = file_name
                    .trim_start_matches("kanban_")
                    .trim_end_matches(".json");
                
                if let Ok(room_id) = RoomId::parse(room_id_str) {
                    boards.push(room_id);
                }
            }
        }
        
        Ok(boards)
    }
}

/// 看板持久化数据结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanbanPersistenceData {
    /// 看板元数据 (从 Matrix 同步)
    pub board_meta: KanbanBoard,
    
    /// 列表结构 (本地存储)
    pub lists: Vec<KanbanListPersistence>,
    
    /// 卡片扩展数据 (本地存储)
    pub cards: Vec<KanbanCardPersistence>,
    
    /// 最后同步时间
    pub last_sync: String,
}

/// 列表持久化数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanbanListPersistence {
    pub list: KanbanList,
    pub extensions: ListExtensions,
}

/// 卡片持久化数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanbanCardPersistence {
    pub card: KanbanCard,
    pub extensions: CardExtensions,
}
```

---

## 6. 附录

### 6.1 数据类型转换

```rust
// src/kanban/converters.rs

use super::*;

/// 数据转换工具
pub struct KanbanConverter;

impl KanbanConverter {
    /// 将 Matrix Room 转换为看板
    pub fn room_to_board(room: &BasicRoomDetails) -> KanbanBoard {
        KanbanBoard {
            id: room.room_id.clone(),
            name: room.name.clone().unwrap_or_default(),
            description: room.topic.clone(),
            avatar_url: room.avatar_url.clone().map(|u| u.to_string()),
            background_color: room.avatar_url
                .as_ref()
                .map_or_else(|| default_background_color(), |_| "#0079BF".to_string()),
            labels: Vec::new(),
            member_ids: room.active_members.clone(),
            admin_ids: Vec::new(),
            list_ids: Vec::new(),
            sort_method: BoardSortMethod::Manual,
            visibility: BoardVisibility::Private,
            is_archived: false,
            created_by: room.creator_id.clone().map_or_else(String::new, |u| u.to_string()),
            created_at: room.created_at.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            updated_at: room.updated_at.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
        }
    }
    
    /// 将 Matrix Message 转换为卡片
    pub fn message_to_card(
        event: &TimelineItem, 
        list_id: &str
    ) -> Option<KanbanCard> {
        let event = event.as_event()?;
        let content = event.content()?;
        
        Some(KanbanCard {
            id: event.event_id().to_owned(),
            title: extract_title(event).unwrap_or_default(),
            description: extract_description(event),
            message_type: determine_message_type(event),
            list_id: list_id.to_string(),
            order_index: 0.0, // 需要从扩展字段获取
            label_ids: Vec::new(),
            member_ids: extract_mentions(event),
            created_by: event.sender().to_string(),
            due_date: None,
            cover: None,
            attachments: extract_attachments(event),
            checklists: Vec::new(),
            activities: Vec::new(),
            comment_count: 0,
            read_member_ids: Vec::new(),
            is_starred: false,
            is_archived: false,
            created_at: event.origin_server_ts().map_or_else(
                || chrono::Utc::now().to_rfc3339(),
                |ts| ts.to_rfc3339()
            ),
            updated_at: event.origin_server_ts().map_or_else(
                || chrono::Utc::now().to_rfc3339(),
                |ts| ts.to_rfc3339()
            ),
        })
    }
}

fn extract_title(event: &OriginalRoomMessageEvent) -> String {
    // 从消息内容提取标题
    if let Some(content) = event.content.as_message() {
        match content {
            MessageType::Text(text) => text.body.clone(),
            MessageType::Image(img) => img.body.clone().unwrap_or_else(|| "图片".to_string()),
            MessageType::File(file) => file.body.clone(),
            MessageType::Video(video) => video.body.clone().unwrap_or_else(|| "视频".to_string()),
            MessageType::Audio(audio) => audio.body.clone().unwrap_or_else(|| "音频".to_string()),
            _ => "卡片".to_string(),
        }
    } else {
        "卡片".to_string()
    }
}

fn determine_message_type(event: &OriginalRoomMessageEvent) -> CardMessageType {
    if let Some(content) = event.content.as_message() {
        match content {
            MessageType::Text(_) => CardMessageType::Text,
            MessageType::Image(_) => CardMessageType::Image,
            MessageType::File(_) => CardMessageType::File,
            MessageType::Location(_) => CardMessageType::Link,
            _ => CardMessageType::Text,
        }
    } else {
        CardMessageType::Text
    }
}
```

### 6.2 性能优化

```rust
// src/kanban/performance.rs

/// 性能优化配置
pub struct PerformanceConfig {
    /// 懒加载阈值
    pub lazy_load_threshold: u32,
    
    /// 虚拟化每页数量
    pub virtualization_page_size: u32,
    
    /// 缓存最大卡片数
    pub max_cached_cards: u32,
    
    /// 自动保存间隔 (毫秒)
    pub auto_save_interval: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            lazy_load_threshold: 50,
            virtualization_page_size: 20,
            max_cached_cards: 500,
            auto_save_interval: 30000,
        }
    }
}

/// 懒加载管理器
pub struct LazyLoadManager {
    /// 已加载的列表
    loaded_lists: HashSet<String>,
    
    /// 加载队列
    load_queue: Vec<String>,
}

impl LazyLoadManager {
    pub fn should_load(&self, list_id: &str) -> bool {
        !self.loaded_lists.contains(list_id)
    }
    
    pub fn mark_loaded(&mut self, list_id: &str) {
        self.loaded_lists.insert(list_id.to_string());
    }
    
    pub fn queue_load(&mut self, list_id: &str) {
        if !self.load_queue.contains(list_id) {
            self.load_queue.push(list_id.to_string());
        }
    }
}
```

---

> 文档版本: 1.0
> 最后更新: 2026-01-14
