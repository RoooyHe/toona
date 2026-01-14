# Toona 项目改造流程文档

> 将 Matrix 聊天客户端改造为类 Trello 项目管理工具

## 文档信息

- **原项目**: Toona (Matrix 聊天客户端)
- **目标**: 类 Trello 项目管理工具
- **约束**: 后端接口不能改变也不能增加
- **协议**: Matrix Protocol (matrix-sdk-rust)
- **UI 框架**: Makepad (Rust)

---

## 目录

1. [架构对比分析](#1-架构对比分析)
2. [核心概念映射](#2-核心概念映射)
3. [改造方案设计](#3-改造方案设计)
4. [数据模型改造](#4-数据模型改造)
5. [API 映射说明](#5-api-映射说明)
6. [分阶段实施计划](#6-分阶段实施计划)
7. [关键技术实现](#7-关键技术实现)
8. [风险与注意事项](#8-风险与注意事项)

---

## 1. 架构对比分析

### 1.1 Trello vs Matrix 架构对比

| 功能维度 | Trello | Matrix (当前) | 改造可行性 |
|---------|--------|---------------|-----------|
| 看板 (Board) | 核心容器 | Room/Space | ✅ 可映射 |
| 列表 (List) | 列分类 | Room Topics | ⚠️ 应用层实现 |
| 卡片 (Card) | 任务单元 | Room Message | ✅ 可映射 |
| 成员管理 | 协作者 | Room Member | ✅ 可复用 |
| 拖拽排序 | 核心交互 | 不支持 | ⚠️ 应用层模拟 |
| 实时协作 | WebSocket | Matrix Sync | ✅ 可复用 |
| 附件管理 | 文件上传 | Media Upload | ✅ 可复用 |
| 标签系统 | Labels | Room Tags | ✅ 可复用 |
| 截止日期 | Due Date | Event Props | ⚠️ 扩展字段 |
| 检查清单 | Checklists | 不原生支持 | ⚠️ 消息实现 |

### 1.2 改造后架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                    改造后的 Toona 架构                            │
├─────────────────────────────────────────────────────────────────┤
│  UI 层 (Makepad Widgets)                                        │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    新增组件                                   ││
│  │  board_view/          │ card_editor/  │ drag_drop/          ││
│  │  - board_workspace    │ - card_modal  │ - drag_handler      ││
│  │  - board_list        │ - card_form   │ - drop_zone         ││
│  │  - board_card        │               │                     ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  扩展的事件/动作处理 (Actions)                                    │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  新增 Action:                      修改 Action:              ││
│  │  - BoardAction        (看板操作)   - RoomAction → ListAction ││
│  │  - ListAction         (列表操作)   - MessageAction → CardActi││
│  │  - CardAction         (卡片操作)                             ││
│  │  - DragDropAction     (拖拽操作)                             ││
│  │  - KanbanFilterAction (过滤操作)                             ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  扩展的请求提交 (submit_async_request)                           │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              扩展 MatrixRequest 枚举                          ││
│  │  新增:                          复用:                         ││
│  │  - CreateBoard (创建看板)       - SendMessage (发送卡片描述)  ││
│  │  - MoveCard (移动卡片)          - GetRoomMembers (获取成员)   ││
│  │  - SortList (排序列表)          - PaginateTimeline (分页)     ││
│  │  - BatchUpdateOrder (批量排序)  - FetchMedia (附件管理)       ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  应用层数据抽象层 (新增)                                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  kanban_data_manager.rs:                                      ││
│  │  - BoardRepository    (看板仓储)                             ││
│  │  - ListRepository     (列表仓储)                             ││
│  │  - CardRepository     (卡片仓储)                             ││
│  │  - OrderManager       (排序管理器)                           ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  现有 Worker 线程 (Tokio Runtime)                                │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  matrix_worker_task + kanban_worker_task (新增)              ││
│  └─────────────────────────────────────────────────────────────┘│
│                           ↓                                      │
│  Matrix SDK (matrix-sdk-rust)                                    │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Room → Message → Event 的映射                               ││
│  │  扩展: Board ←→ List ←→ Card 的内存模型                      ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. 核心概念映射

### 2.1 概念映射表

#### 看板层 (Board Layer)

```rust
// Trello 概念 → Matrix 映射
Trello          Matrix              实现方式
────────────────────────────────────────────────────────────
Board     →    Room + Space        Room 作为看板
Board ID  →    Room.id             直接复用
Board Name→    Room.name           直接复用
Board Desc→    Room.topic          直接复用
Members   →    Room.members        直接复用
Labels    →    Room.tags           直接复用
Background→    Room.avatar/colour  直接复用
Visibility→    Room.join_rule      直接复用
```

#### 列表层 (List Layer)

```rust
// Trello 概念 → 应用层映射
Trello          应用层              实现方式
────────────────────────────────────────────────────────────
List      →    KanbanList          新数据结构
List ID   →    uuid::Uuid          生成 UUID
List Name →    String              直接复用
List Order→    f32 (order_index)   排序索引
Cards     →    Vec<Card>           卡片列表
Closed    →    bool                直接复用
```

#### 卡片层 (Card Layer)

```rust
// Trello 概念 → Matrix 消息映射
Trello          Matrix              实现方式
────────────────────────────────────────────────────────────
Card      →    RoomMessage         Room 消息
Card ID   →    Event.id            直接复用
Card Name →    message.body        消息文本
Card Desc →    message.formatted   格式化消息
Due Date  →    Extension Field     扩展字段
Labels    →    Message Tags        消息标签
Members   →    Mentions (@用户)    @提及
Attachments→    Message Media      媒体消息
Checklists→    Sub-Events          子事件消息
Comments  →    Thread Replies      线程回复
Position  →    Extension Field     扩展字段 (order)
```

### 2.2 扩展字段定义

为弥补 Matrix 协议不支持的字段，使用消息的 `extensions` 字段：

```rust
// src/kanban/card_extensions.rs

use serde::{Deserialize, Serialize};

/// 卡片扩展字段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardExtensions {
    /// 卡片排序索引 (用于拖拽排序)
    pub order_index: f64,
    
    /// 截止日期 (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    
    /// 所属列表 ID
    pub list_id: String,
    
    /// 标签 ID 列表
    #[serde(default)]
    pub labels: Vec<String>,
    
    /// 封面图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    
    /// 检查清单完成度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_progress: Option<ChecklistProgress>,
    
    /// 附件数量
    #[serde(default)]
    pub attachment_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistProgress {
    pub total: u32,
    pub completed: u32,
}
```

---

## 3. 改造方案设计

### 3.1 整体改造策略

#### 渐进式改造原则

```
Phase 1: 基础框架
├── 新增 kanban 模块
├── 定义 Board/List/Card 数据结构
└── 创建数据抽象层

Phase 2: 看板管理
├── 实现看板 CRUD (基于 Room API)
├── 实现列表管理 (基于应用层)
└── 改造 RoomList 组件为 BoardList

Phase 3: 卡片管理
├── 实现卡片 CRUD (基于 Message API)
├── 改造 Timeline 组件为 CardList
└── 实现卡片详情编辑

Phase 4: 交互增强
├── 实现拖拽排序 (应用层)
├── 实现筛选过滤
└── 实现批量操作

Phase 5: 协作功能
├── 成员管理复用
├── 实时同步复用
└── 通知系统复用
```

### 3.2 目录结构改造

```
src/
├── kanban/                      # 新增: 看板模块
│   ├── mod.rs                   # 模块入口
│   ├── board.rs                 # 看板数据结构
│   ├── list.rs                  # 列表数据结构
│   ├── card.rs                  # 卡片数据结构
│   ├── board_repository.rs      # 看板仓储
│   ├── list_repository.rs       # 列表仓储
│   ├── card_repository.rs       # 卡片仓储
│   ├── order_manager.rs         # 排序管理器
│   ├── kanban_service.rs        # 看板服务
│   └── kanban_action.rs         # 看板动作
│
├── kanban_ui/                   # 新增: 看板 UI 组件
│   ├── mod.rs
│   ├── board_view.rs            # 看板主视图
│   ├── board_workspace.rs       # 工作区组件
│   ├── kanban_board.rs          # 单个看板视图
│   ├── kanban_list.rs           # 列表组件
│   ├── kanban_card.rs           # 卡片组件
│   ├── card_modal.rs            # 卡片详情弹窗
│   ├── card_form.rs             # 卡片编辑表单
│   ├── drag_drop_handler.rs     # 拖拽处理器
│   ├── drop_zone.rs             # 放置区域
│   ├── kanban_toolbar.rs        # 工具栏
│   ├── filter_bar.rs            # 筛选栏
│   └── kanban_empty_state.rs    # 空状态
│
├── home/
│   ├── board_screen.rs          # 改造: 房间屏幕 → 看板屏幕
│   ├── main_desktop_ui.rs       # 添加看板视图入口
│   └── main_mobile_ui.rs        # 添加看板视图入口
│
├── sliding_sync.rs              # 扩展 MatrixRequest 枚举
│
└── persistence/
    └── kanban_state.rs          # 新增: 看板状态持久化
```

---

## 4. 数据模型改造

### 4.1 看板数据结构

```rust
// src/kanban/board.rs

use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use makepad_widgets::*;
use matrix_sdk::ruma::{OwnedRoomId, RoomId};

live_design! {
    // 看板 UI 样式定义
    kanban_board_styles = {
        board_container = {
            flow: Down,
            spacing: 10.0,
            background_color: #F0F2F5,
            scroll: {x: true, y: false}
        }
    }
}

/// 看板元数据 (对应 Matrix Room)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    /// 看板 ID (Room ID)
    pub id: OwnedRoomId,
    
    /// 看板名称
    pub name: String,
    
    /// 看板描述
    pub description: String,
    
    /// 背景颜色
    pub background_color: String,
    
    /// 标签定义
    pub labels: Vec<BoardLabel>,
    
    /// 成员 ID 列表
    pub member_ids: Vec<String>,
    
    /// 列表 ID 列表 (应用层维护)
    pub list_ids: Vec<String>,
    
    /// 是否已归档
    pub is_archived: bool,
    
    /// 创建时间
    pub created_at: String,
    
    /// 更新时间
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardLabel {
    pub id: String,
    pub name: String,
    pub color: String,
}

/// 看板状态 (响应式)
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
}
```

### 4.2 列表数据结构

```rust
// src/kanban/list.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 看板列表 (应用层数据结构)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanbanList {
    /// 列表 ID (UUID)
    pub id: String,
    
    /// 所属看板 ID
    pub board_id: String,
    
    /// 列表名称
    pub name: String,
    
    /// 排序索引
    pub order_index: f64,
    
    /// 卡片列表
    pub cards: Vec<KanbanCard>,
    
    /// 是否已归档
    pub is_archived: bool,
    
    /// 卡片数量限制 (可选)
    pub card_limit: Option<u32>,
}

impl KanbanList {
    pub fn new(board_id: &str, name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            board_id: board_id.to_string(),
            name: name.to_string(),
            order_index: 0.0,
            cards: Vec::new(),
            is_archived: false,
            card_limit: None,
        }
    }
}
```

### 4.3 卡片数据结构

```rust
// src/kanban/card.rs

use serde::{Deserialize, Serialize};
use matrix_sdk::ruma::OwnedEventId;

/// 看板卡片 (对应 Matrix Message)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCard {
    /// 卡片 ID (Event ID)
    pub id: OwnedEventId,
    
    /// 卡片标题
    pub title: String,
    
    /// 卡片描述 (消息内容)
    pub description: String,
    
    /// 所属列表 ID
    pub list_id: String,
    
    /// 排序索引
    pub order_index: f64,
    
    /// 标签 ID 列表
    pub labels: Vec<String>,
    
    /// 成员 ID 列表 (@提及)
    pub member_ids: Vec<String>,
    
    /// 截止日期
    pub due_date: Option<String>,
    
    /// 附件
    pub attachments: Vec<CardAttachment>,
    
    /// 检查清单
    pub checklists: Vec<Checklist>,
    
    /// 封面图片
    pub cover_url: Option<String>,
    
    /// 评论数量
    pub comment_count: u32,
    
    /// 是否已归档
    pub is_archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAttachment {
    pub id: String,
    pub name: String,
    pub url: String,
    pub content_type: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checklist {
    pub id: String,
    pub name: String,
    pub items: Vec<ChecklistItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: String,
    pub text: String,
    pub is_completed: bool,
}
```

### 4.4 看板仓储模式

```rust
// src/kanban/board_repository.rs

use crate::sliding_sync::submit_async_request;
use super::*;

pub struct BoardRepository;

impl BoardRepository {
    /// 获取所有看板 (基于 Room 列表)
    pub async fn fetch_all_boards(
        &self,
        home_server_rooms: Vec<BasicRoomDetails>,
    ) -> Vec<KanbanBoard> {
        home_server_rooms
            .into_iter()
            .map(|room| self.map_room_to_board(room))
            .collect()
    }
    
    /// 创建新看板 (创建 Room)
    pub async fn create_board(
        &self,
        name: String,
        description: String,
    ) -> Result<KanbanBoard, String> {
        // 使用现有 CreateRoom API
        let request = MatrixRequest::CreateRoom {
            name: name.clone(),
            topic: Some(description),
            is_public: false,
            invite: Vec::new(),
            room_alias: None,
            federation: true,
        };
        
        submit_async_request(request)
            .await
            .map_err(|e| e.to_string())
            .and_then(|result| {
                // 解析 Room ID
                // 创建 KanbanBoard
                todo!()
            })
    }
    
    /// 更新看板信息 (更新 Room 状态)
    pub async fn update_board(
        &self,
        board_id: &RoomId,
        updates: BoardUpdates,
    ) -> Result<(), String> {
        // 使用现有 Room 状态更新 API
        todo!()
    }
    
    /// 删除看板 (关闭 Room)
    pub async fn delete_board(&self, board_id: &RoomId) -> Result<(), String> {
        // 使用现有 LeaveRoom API
        todo!()
    }
    
    /// 映射 Room 到看板
    fn map_room_to_board(&self, room: BasicRoomDetails) -> KanbanBoard {
        KanbanBoard {
            id: room.room_id,
            name: room.name,
            description: room.topic.unwrap_or_default(),
            background_color: room.avatar.map_or_default_color(),
            labels: Vec::new(),
            member_ids: room.active_members,
            list_ids: Vec::new(), // 从持久化恢复
            is_archived: false,
            created_at: room.created_at,
            updated_at: room.updated_at,
        }
    }
}

pub struct CardRepository;

impl CardRepository {
    /// 创建卡片 (发送消息)
    pub async fn create_card(
        &self,
        room_id: &RoomId,
        list_id: &str,
        title: String,
    ) -> Result<KanbanCard, String> {
        let content = RoomMessageEventContent::text_markdown(&title);
        let extensions = CardExtensions {
            list_id: list_id.to_string(),
            order_index: 0.0,
            ..Default::default()
        };
        
        // 使用现有 SendMessage API，附加扩展字段
        todo!()
    }
    
    /// 获取卡片列表 (分页获取消息)
    pub async fn fetch_cards(
        &self,
        room_id: &RoomId,
        list_id: &str,
    ) -> Vec<KanbanCard> {
        // 使用现有 PaginateRoomTimeline API
        // 过滤指定 list_id 的消息
        todo!()
    }
    
    /// 更新卡片 (编辑消息)
    pub async fn update_card(
        &self,
        room_id: &RoomId,
        card_id: &EventId,
        updates: CardUpdates,
    ) -> Result<(), String> {
        // 使用现有 EditMessage API
        todo!()
    }
    
    /// 移动卡片 (编辑消息 + 更新列表)
    pub async fn move_card(
        &self,
        room_id: &RoomId,
        card_id: &EventId,
        from_list_id: &str,
        to_list_id: &str,
        new_index: f64,
    ) -> Result<(), String> {
        // 更新卡片扩展字段中的 list_id 和 order_index
        todo!()
    }
    
    /// 删除卡片 (删除消息)
    pub async fn delete_card(
        &self,
        room_id: &RoomId,
        card_id: &EventId,
    ) -> Result<(), String> {
        // 使用现有 RedactMessage API
        todo!()
    }
}
```

---

## 5. API 映射说明

### 5.1 MatrixRequest 扩展

```rust
// 扩展 src/sliding_sync.rs 中的 MatrixRequest 枚举

/// 看板管理请求
#[derive(Debug, Clone)]
pub enum KanbanRequest {
    /// 创建看板
    CreateBoard {
        name: String,
        description: String,
    },
    
    /// 更新看板
    UpdateBoard {
        board_id: OwnedRoomId,
        name: Option<String>,
        description: Option<String>,
        background_color: Option<String>,
    },
    
    /// 删除看板
    DeleteBoard {
        board_id: OwnedRoomId,
    },
    
    /// 归档看板
    ArchiveBoard {
        board_id: OwnedRoomId,
    },
}

/// 列表管理请求
#[derive(Debug, Clone)]
pub enum ListRequest {
    /// 创建列表
    CreateList {
        board_id: OwnedRoomId,
        name: String,
        position: Option<f64>,
    },
    
    /// 更新列表
    UpdateList {
        board_id: OwnedRoomId,
        list_id: String,
        name: Option<String>,
        position: Option<f64>,
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
    
    /// 归档列表
    ArchiveList {
        board_id: OwnedRoomId,
        list_id: String,
    },
}

/// 卡片管理请求
#[derive(Debug, Clone)]
pub enum CardRequest {
    /// 创建卡片
    CreateCard {
        board_id: OwnedRoomId,
        list_id: String,
        title: String,
        description: Option<String>,
        due_date: Option<String>,
        member_ids: Vec<String>,
    },
    
    /// 更新卡片
    UpdateCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        title: Option<String>,
        description: Option<String>,
        due_date: Option<String>,
        labels: Option<Vec<String>>,
    },
    
    /// 移动卡片
    MoveCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        from_list_id: String,
        to_list_id: String,
        new_position: f64,
    },
    
    /// 删除卡片
    DeleteCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
    },
    
    /// 归档卡片
    ArchiveCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
    },
    
    /// 批量移动卡片
    BatchMoveCards {
        board_id: OwnedRoomId,
        moves: Vec<CardMove>,
    },
}

/// 卡片移动操作
#[derive(Debug, Clone)]
pub struct CardMove {
    pub card_id: OwnedEventId,
    pub from_list_id: String,
    pub to_list_id: String,
    pub new_position: f64,
}
```

### 5.2 API 映射矩阵

| Trello 操作 | Matrix API | 改造方式 |
|-------------|------------|----------|
| 创建看板 | `CreateRoom` | 直接复用 |
| 重命名看板 | `SendStateEvent` | 复用状态事件 API |
| 删除看板 | `LeaveRoom` | 直接复用 |
| 创建列表 | (应用层) | 新增数据存储 |
| 重命名列表 | (应用层) | 更新数据存储 |
| 删除列表 | (应用层) | 更新数据存储 |
| 移动列表 | (应用层) | 更新排序索引 |
| 创建卡片 | `SendMessage` | 直接复用 |
| 编辑卡片 | `EditMessage` | 直接复用 |
| 删除卡片 | `RedactMessage` | 直接复用 |
| 移动卡片 | `EditMessage` | 更新扩展字段 |
| 添加成员 | `InviteUser` | 直接复用 |
| 移除成员 | `KickUser` | 直接复用 |
| 添加标签 | `SendStateEvent` | 复用标签 API |
| 设置截止日期 | (应用层) | 扩展字段 |
| 上传附件 | `UploadMedia` | 直接复用 |
| 添加评论 | `SendMessage` (线程) | 直接复用 |
| 添加检查项 | (应用层) | 子消息实现 |
| 拖拽排序 | (应用层) | 应用层模拟 |

### 5.3 现有 API 复用策略

#### 房间管理 API 复用

```rust
// 1. 创建看板 (复用 CreateRoom)
submit_async_request(MatrixRequest::CreateRoom {
    name: board_name,
    topic: board_description,
    is_public: false,
    invite: members,
    room_alias: None,
    federation: true,
}).await;

// 2. 获取看板列表 (复用 RoomList)
let rooms = app_state.matrix_state.session
    .as_ref()
    .unwrap()
    .room_list_service
    .get_rooms();

// 3. 更新看板信息 (复用 RoomState API)
// 通过发送状态事件更新 Room 元数据

// 4. 归档/删除看板 (复用 LeaveRoom)
```

#### 消息 API 复用

```rust
// 1. 创建卡片 (复用 SendMessage)
let message = RoomMessageEventContent::text_markdown(&card_title);
let extensions = CardExtensions {
    list_id: list_id.to_string(),
    order_index: next_position(),
    ..Default::default()
};
submit_async_request(MatrixRequest::SendMessage {
    room_id: room_id.clone(),
    message: message.into(),
    replied_to: None,
}).await;

// 2. 编辑卡片 (复用 EditMessage)
submit_async_request(MatrixRequest::EditMessage {
    room_id: room_id.clone(),
    event_id: card_event_id.clone(),
    new_content: new_content.into(),
}).await;

// 3. 删除卡片 (复用 RedactMessage)
submit_async_request(MatrixRequest::RedactMessage {
    room_id: room_id.clone(),
    event_id: card_event_id.clone(),
    reason: Some("Card deleted".to_string()),
}).await;

// 4. 分页获取卡片 (复用 PaginateRoomTimeline)
let timeline = submit_async_request(MatrixRequest::PaginateRoomTimeline {
    room_id: room_id.clone(),
    event_id: None, // 从最新消息开始
    limit: 50,
    direction: Direction::Backward,
}).await;
```

---

## 6. 分阶段实施计划

### 6.1 第一阶段：基础框架

**目标**: 建立看板应用的基础架构

**任务清单**:

```
□ 1.1 创建 kanban 模块目录结构
   ├─ src/kanban/
   ├─ src/kanban_ui/
   └─ 更新 src/lib.rs 模块导入

□ 1.2 定义核心数据结构
   ├─ KanbanBoard (看板)
   ├─ KanbanList (列表)
   └─ KanbanCard (卡片)

□ 1.3 实现仓储层
   ├─ BoardRepository
   ├─ ListRepository
   └─ CardRepository

□ 1.4 实现排序管理器
   └─ OrderManager (Lexorank 算法)

□ 1.5 扩展 MatrixRequest 枚举
   ├─ KanbanRequest
   ├─ ListRequest
   └─ CardRequest

□ 1.6 添加看板状态持久化
   └─ kanban_state.rs
```

**时间估算**: 2-3 天

**验收标准**:
- [ ] 所有数据结构定义完成
- [ ] 编译通过
- [ ] 单元测试覆盖核心逻辑

### 6.2 第二阶段：看板管理

**目标**: 实现看板的 CRUD 功能

**任务清单**:

```
□ 2.1 创建 BoardList 组件 (改造 RoomList)
   ├─ 复用 rooms_list.rs 结构
   └─ 适配看板数据模型

□ 2.2 创建 BoardView 组件
   ├─ 看板卡片布局
   ├─ 看板预览图
   └─ 看板信息展示

□ 2.3 实现看板列表页面
   ├─ 看板网格/列表视图
   ├─ 新建看板按钮
   └─ 筛选和搜索

□ 2.4 实现看板详情页面
   ├─ 看板背景设置
   ├─ 标签管理
   └─ 成员管理

□ 2.5 集成到主界面
   ├─ 侧边栏添加看板入口
   └─ 桌面/移动端适配
```

**时间估算**: 3-4 天

**验收标准**:
- [ ] 可以创建、查看、编辑、删除看板
- [ ] 看板列表展示正确
- [ ] 与现有房间列表共存

### 6.3 第三阶段：卡片管理

**目标**: 实现列表和卡片的 CRUD 功能

**任务清单**:

```
□ 3.1 创建 KanbanList 组件
   ├─ 列表头部 (名称、菜单)
   ├─ 卡片容器
   └─ 添加卡片按钮

□ 3.2 创建 KanbanCard 组件
   ├─ 卡片标题
   ├─ 标签徽章
   ├─ 成员头像
   ├─ 截止日期
   └─ 封面图片

□ 3.3 实现卡片详情弹窗
   ├─ 描述编辑
   ├─ 标签选择
   ├─ 成员选择
   ├─ 截止日期选择
   └─ 附件管理

□ 3.4 实现检查清单功能
   ├─ 添加检查项
   ├─ 切换完成状态
   └─ 进度展示

□ 3.5 实现评论功能
   ├─ 评论列表
   └─ 添加评论

□ 3.6 实现列表管理
   ├─ 添加列表
   ├─ 重命名列表
   └─ 删除列表
```

**时间估算**: 5-6 天

**验收标准**:
- [ ] 可以创建、查看、编辑、删除列表和卡片
- [ ] 卡片详情功能完整
- [ ] 列表管理功能完整

### 6.4 第四阶段：交互增强

**目标**: 实现拖拽排序和筛选功能

**任务清单**:

```
□ 4.1 实现拖拽系统
   ├─ 拖拽句柄组件
   ├─ 卡片拖拽 (跨列表)
   ├─ 列表拖拽 (看板内)
   └─ 拖拽预览效果

□ 4.2 实现排序管理器
   ├─ Lexorank 算法
   ├─ 批量更新排序
   └─ 冲突处理

□ 4.3 实现筛选栏
   ├─ 成员筛选
   ├─ 标签筛选
   ├─ 截止日期筛选
   └─ 关键词搜索

□ 4.4 实现批量操作
   ├─ 多选卡片
   ├─ 批量移动
   ├─ 批量归档
   └─ 批量删除

□ 4.5 实现快捷键
   ├─ 快速添加卡片
   ├─ 快速搜索
   └─ 导航快捷键
```

**时间估算**: 4-5 天

**验收标准**:
- [ ] 拖拽操作流畅
- [ ] 筛选功能正确
- [ ] 批量操作正常

### 6.5 第五阶段：协作与同步

**目标**: 实现实时协作和高级功能

**任务清单**:

```
□ 5.1 复用实时同步
   ├─ Matrix Sync 实时更新
   ├─ 卡片变更实时通知
   └─ 成员变更同步

□ 5.2 实现@提及功能
   ├─ @用户输入建议
   └─ 通知触发

□ 5.3 实现活动日志
   ├─ 卡片变更记录
   ├─ 成员操作记录
   └─ 显示位置 (卡片详情)

□ 5.4 实现归档管理
   ├─ 归档卡片
   ├─ 归档列表
   └─ 恢复功能

□ 5.5 实现权限管理
   ├─ 看板管理员
   ├─ 普通成员
   └─ 观察者
```

**时间估算**: 3-4 天

**验收标准**:
- [ ] 实时同步正常
- [ ] @提及功能完整
- [ ] 权限管理正确

### 6.6 阶段总结

| 阶段 | 任务数 | 时间估算 | 累计时间 |
|------|--------|----------|----------|
| Phase 1: 基础框架 | 6 | 2-3 天 | 2-3 天 |
| Phase 2: 看板管理 | 5 | 3-4 天 | 5-7 天 |
| Phase 3: 卡片管理 | 6 | 5-6 天 | 10-13 天 |
| Phase 4: 交互增强 | 5 | 4-5 天 | 14-18 天 |
| Phase 5: 协作同步 | 5 | 3-4 天 | 17-22 天 |

**总计**: 17-22 个工作日

---

## 7. 关键技术实现

### 7.1 拖拽排序实现

使用 Lexorank 算法实现高效的拖拽排序：

```rust
// src/kanban/order_manager.rs

use super::*;

/// 排序管理器 (Lexorank 算法)
pub struct OrderManager;

impl OrderManager {
    /// 计算新位置的排序值
    pub fn calculate_new_position(
        &self,
        before: Option<f64>,
        after: Option<f64>,
    ) -> f64 {
        match (before, after) {
            (Some(b), Some(a)) => (b + a) / 2.0,
            (Some(b), None) => b + 1000.0,
            (None, Some(a)) => a - 1000.0,
            (None, None) => 0.0,
        }
    }
    
    /// 批量重新排序
    pub fn reorder_all(
        &self,
        items: &mut Vec<impl Sortable>,
    ) -> Vec<SortUpdate> {
        items.sort_by(|a, b| a.order_index().partial_cmp(&b.order_index()).unwrap());
        
        items.iter()
            .enumerate()
            .map(|(i, item)| {
                let new_order = (i as f64) * 100.0;
                item.update_order(new_order)
            })
            .collect()
    }
    
    /// 插入时计算位置
    pub fn insert_at_position(
        &self,
        position: f64,
        existing_positions: &[f64],
    ) -> f64 {
        let neighbors = self.find_neighbors(position, existing_positions);
        self.calculate_new_position(neighbors.before, neighbors.after)
    }
    
    fn find_neighbors(
        &self,
        position: f64,
        positions: &[f64],
    ) -> Neighbors {
        let mut before = None;
        let mut after = None;
        
        for &p in positions {
            if p < position {
                if before.is_none() || p > before.unwrap() {
                    before = Some(p);
                }
            } else if p > position {
                if after.is_none() || p < after.unwrap() {
                    after = Some(p);
                }
            }
        }
        
        Neighbors { before, after }
    }
}

struct Neighbors {
    before: Option<f64>,
    after: Option<f64>,
}

trait Sortable {
    fn order_index(&self) -> f64;
    fn update_order(&self, new_order: f64) -> SortUpdate;
}

struct SortUpdate {
    id: String,
    new_order: f64,
}
```

### 7.2 拖拽处理器

```rust
// src/kanban_ui/drag_drop_handler.rs

use makepad_widgets::*;

#[derive(Debug, Clone, LiveHook)]
pub struct DragDropHandler {
    /// 当前拖拽的卡片
    pub dragging_card: Option<KanbanCard>,
    
    /// 拖拽源列表 ID
    pub source_list_id: Option<String>,
    
    /// 拖拽目标位置
    pub drop_target: Option<DropTarget>,
    
    /// 拖拽状态
    pub is_dragging: bool,
}

#[derive(Debug, Clone)]
pub struct DropTarget {
    pub list_id: String,
    pub card_id: Option<String>,
    pub position: DropPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DropPosition {
    Before,
    After,
    Into,
    Empty,
}

impl DragDropHandler {
    /// 开始拖拽
    pub fn start_drag(&mut self, card: KanbanCard, source_list_id: String) {
        self.dragging_card = Some(card);
        self.source_list_id = Some(source_list_id);
        self.is_dragging = true;
    }
    
    /// 更新拖拽目标
    pub fn update_drop_target(
        &mut self,
        list_id: String,
        card_id: Option<String>,
        position: DropPosition,
    ) {
        self.drop_target = Some(DropTarget {
            list_id,
            card_id,
            position,
        });
    }
    
    /// 执行放置
    pub fn perform_drop(
        &mut self,
        board_id: &RoomId,
    ) -> Option<CardRequest> {
        let (card, source) = match (self.dragging_card.take(), self.source_list_id.take()) {
            (Some(card), Some(source)) => (card, source),
            _ => return None,
        };
        
        let target = self.drop_target.take()?;
        
        // 如果是同列表内移动
        if source == target.list_id {
            Some(CardRequest::MoveCard {
                board_id: board_id.clone(),
                card_id: card.id,
                from_list_id: source,
                to_list_id: target.list_id,
                new_position: target.calculate_position(),
            })
        } else {
            // 跨列表移动
            Some(CardRequest::MoveCard {
                board_id: board_id.clone(),
                card_id: card.id,
                from_list_id: source,
                to_list_id: target.list_id,
                new_position: target.calculate_position(),
            })
        }
    }
    
    /// 取消拖拽
    pub fn cancel_drag(&mut self) {
        self.dragging_card = None;
        self.source_list_id = None;
        self.drop_target = None;
        self.is_dragging = false;
    }
}

impl DropTarget {
    fn calculate_position(&self) -> f64 {
        // 根据位置类型计算新的排序索引
        match self.position {
            DropPosition::Before => -100.0,
            DropPosition::After => 100.0,
            DropPosition::Into => 0.0,
            DropPosition::Empty => 0.0,
        }
    }
}
```

### 7.3 看板状态管理

```rust
// src/kanban/kanban_service.rs

use super::*;

pub struct KanbanService {
    board_repository: BoardRepository,
    list_repository: ListRepository,
    card_repository: CardRepository,
    order_manager: OrderManager,
}

impl KanbanService {
    pub fn new() -> Self {
        Self {
            board_repository: BoardRepository,
            list_repository: ListRepository,
            card_repository: CardRepository,
            order_manager: OrderManager,
        }
    }
    
    /// 获取看板卡片 (按列表分组)
    pub async fn get_board_cards(
        &self,
        board_id: &RoomId,
    ) -> Result<Vec<KanbanList>, String> {
        // 获取所有列表
        let lists = self.list_repository.get_lists(board_id).await?;
        
        // 并行获取每个列表的卡片
        let mut lists_with_cards = Vec::new();
        
        for list in lists {
            let cards = self.card_repository
                .fetch_cards(board_id, &list.id)
                .await?;
            
            lists_with_cards.push(KanbanList {
                cards,
                ..list
            });
        }
        
        Ok(lists_with_cards)
    }
    
    /// 移动卡片 (跨列表)
    pub async fn move_card(
        &self,
        board_id: &RoomId,
        card_id: &EventId,
        from_list_id: &str,
        to_list_id: &str,
        new_position: f64,
    ) -> Result<(), String> {
        // 更新卡片位置
        self.card_repository
            .move_card(board_id, card_id, from_list_id, to_list_id, new_position)
            .await?;
        
        // 触发实时同步
        self.notify_card_moved(board_id, card_id, from_list_id, to_list_id).await;
        
        Ok(())
    }
    
    /// 批量移动卡片 (拖拽排序)
    pub async fn batch_move_cards(
        &self,
        board_id: &RoomId,
        moves: Vec<CardMove>,
    ) -> Result<(), String> {
        for move_op in moves {
            self.card_repository
                .move_card(
                    board_id,
                    &move_op.card_id,
                    &move_op.from_list_id,
                    &move_op.to_list_id,
                    move_op.new_position,
                )
                .await?;
        }
        
        // 批量更新后重新排序
        self.reorder_all_lists(board_id).await?;
        
        Ok(())
    }
    
    /// 重新排序列表中的所有卡片
    async fn reorder_all_lists(&self, board_id: &RoomId) -> Result<(), String> {
        let lists = self.list_repository.get_lists(board_id).await?;
        
        for list in lists {
            let mut cards = list.cards;
            let updates = self.order_manager.reorder_all(&mut cards);
            
            for update in updates {
                self.card_repository
                    .update_card_position(board_id, &update.id, update.new_order)
                    .await?;
            }
        }
        
        Ok(())
    }
    
    async fn notify_card_moved(
        &self,
        board_id: &RoomId,
        card_id: &EventId,
        from_list_id: &str,
        to_list_id: &str,
    ) {
        // 发送自定义事件通知其他客户端
        // 使用现有的 SendStateEvent API
    }
}
```

### 7.4 UI 组件集成

```rust
// src/kanban_ui/board_view.rs

use super::*;

live_design! {
    kanban_board_view = {{BoardView}} {
        flow: Down,
        width: Fill,
        height: Fill,
        
        board_toolbar = {
            height: 50,
            background_color: #FFFFFF,
            border_bottom: 1, #E0E0E0,
        }
        
        board_content = {
            flow: Right,
            width: Fill,
            height: Fill,
            scroll: {x: true, y: false},
            spacing: 12,
            padding: 16,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct BoardView {
    #[live] toolbar: KanbanToolbar,
    #[live] content: FlowBox,
    #[live] lists: Vec<KanbanList>,
}

impl BoardView {
    /// 渲染看板内容
    pub fn render(&mut self, lists: Vec<KanbanList>) {
        self.lists = lists;
        self.content = FlowBox {
            width: Fill,
            height: Fill,
            spacing: 12,
            ..Default::default()
        };
        
        for list in &self.lists {
            let list_component = KanbanList::new();
            list_component.set_list(list);
            self.content.add_child(list_component);
        }
        
        // 添加"添加列表"按钮
        let add_list_btn = self.create_add_list_button();
        self.content.add_child(add_list_btn);
    }
    
    fn create_add_list_button(&mut self) -> KanbanList {
        KanbanList::create_add_list()
    }
}

#[derive(Debug, Clone, LiveHook)]
pub struct KanbanList {
    #[live] header: Label,
    #[live] cards: VecBox,
    #[live] add_card_btn: Button,
}

impl KanbanList {
    pub fn new() -> Self {
        Self {
            header: Label {
                text: "待办事项".to_string(),
                ..Default::default()
            },
            cards: VecBox::new(),
            add_card_btn: Button::with_label("+ 添加卡片"),
        }
    }
    
    pub fn set_list(&mut self, list: &KanbanListData) {
        self.header.set_text(&list.name);
        
        // 渲染卡片
        for card in &list.cards {
            let card_component = KanbanCard::new();
            card_component.set_card(card);
            self.cards.add_child(card_component);
        }
    }
    
    pub fn create_add_list() -> Self {
        Self {
            header: Label {
                text: "+ 添加列表".to_string(),
                ..Default::default()
            },
            cards: VecBox::new(),
            add_card_btn: Button::empty(),
        }
    }
}
```

---

## 8. 风险与注意事项

### 8.1 技术风险

| 风险 | 可能性 | 影响 | 应对措施 |
|------|--------|------|----------|
| Matrix 消息不支持复杂查询 | 高 | 中 | 应用层索引和缓存 |
| 拖拽性能问题 | 中 | 低 | 虚拟化列表 + 增量更新 |
| 离线同步冲突 | 中 | 高 | 乐观更新 + 冲突解决 |
| 大量消息性能 | 低 | 中 | 分页加载 + 懒加载 |
| 排序索引溢出 | 低 | 中 | 定期重建索引 |

### 8.2 架构限制

1. **消息顺序**: Matrix 消息按时间排序，不支持任意位置插入
   - **解决方案**: 使用应用层 `order_index` 字段维护顺序

2. **列表间移动**: Matrix 不支持消息跨房间移动
   - **解决方案**: 需要删除原消息，创建新消息

3. **原子操作**: 批量移动不是原子操作
   - **解决方案**: 实现事务和回滚机制

4. **实时协作**: 没有操作转换 (OT) 或 CRDT
   - **解决方案**: 乐观更新 + 最后写入获胜

### 8.3 兼容性考虑

- **Homeserver 兼容性**: 所有 Matrix homeserver 应该都能工作
- **性能要求**: 大量卡片时可能需要优化
- **离线支持**: 需要应用层缓存策略
- **多设备同步**: 依赖 Matrix Sync 的标准行为

### 8.4 测试策略

```
单元测试:
├─ OrderManager 排序算法
├─ CardRepository 数据访问
├─ KanbanService 业务逻辑
└─ DragDropHandler 拖拽逻辑

集成测试:
├─ API 映射正确性
├─ 数据同步一致性
└─ 拖拽操作正确性

端到端测试:
├─ 完整看板流程
├─ 多人协作场景
└─ 离线重连恢复

性能测试:
├─ 大看板渲染性能
├─ 大量卡片滚动性能
└─ 批量操作响应时间
```

---

## 附录

### A. 相关文件路径

| 文件 | 路径 | 说明 |
|------|------|------|
| API 定义 | `src/sliding_sync.rs:350-500` | MatrixRequest 枚举 |
| 消息处理 | `src/sliding_sync.rs:1200-1250` | 消息发送实现 |
| 时间线更新 | `src/home/room_screen.rs:2474-2570` | TimelineUpdate 枚举 |
| 消息发送 UI | `src/room/room_input_bar.rs:250-300` | 消息编辑组件 |
| 房间列表 | `src/home/rooms_list.rs:150-250` | 房间列表管理 |
| 状态持久化 | `src/persistence/matrix_state.rs:50-150` | 会话持久化 |

### B. 外部资源

- [Matrix Protocol Spec](https://spec.matrix.org/)
- [matrix-sdk-rust](https://docs.rs/matrix-sdk/latest/matrix_sdk/)
- [Makepad Framework](https://github.com/makepad/makepad)
- [Trello API](https://developer.atlassian.com/cloud/trello/rest/api-group-cards/)

### C. 改造检查清单

```
基础设施:
□ 创建 kanban 模块
□ 定义数据结构
□ 实现仓储层
□ 扩展 MatrixRequest

看板功能:
□ 看板列表页面
□ 看板详情页面
□ 看板 CRUD 操作

卡片功能:
□ 卡片列表渲染
□ 卡片详情弹窗
□ 列表管理

交互功能:
□ 拖拽排序
□ 筛选搜索
□ 批量操作

协作功能:
□ 实时同步
□ @提及
□ 权限管理
```

---

> 文档生成时间: 2026-01-14
> 版本: 1.0
