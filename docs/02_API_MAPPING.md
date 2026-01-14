# API 映射详细文档

> Toona 项目改造 - Matrix API 映射与扩展

## 文档信息

- **版本**: 1.0
- **创建日期**: 2026-01-14
- **状态**: 设计稿

---

## 目录

1. [概述](#1-概述)
2. [现有 API 分析](#2-现有-api-分析)
3. [API 扩展设计](#3-api-扩展设计)
4. [完整 API 映射表](#4-完整-api-映射表)
5. [请求处理流程](#5-请求处理流程)
6. [错误处理](#6-错误处理)
7. [示例代码](#7-示例代码)

---

## 1. 概述

### 1.1 文档目的

本文档详细描述如何将 Toona 项目中的 Matrix API 映射到 Trello 风格的项目管理功能。我们将复用现有的 `MatrixRequest` 枚举，并进行必要的扩展，以支持看板、列表和卡片的管理操作。

### 1.2 设计原则

- **最小化修改**: 尽可能复用现有 API，避免破坏现有功能
- **向后兼容**: 新增的 API 不影响现有的聊天功能
- **类型安全**: 使用 Rust 的类型系统确保 API 使用的正确性
- **异步优先**: 所有 API 调用都是异步的

### 1.3 API 分层

```
┌─────────────────────────────────────────────────────────────────┐
│                      API 调用分层架构                            │
├─────────────────────────────────────────────────────────────────┤
│  UI 层                                                           │
│  submit_async_request(MatrixRequest)                            │
│                           ↓                                      │
│  动作层 (Actions)                                                │
│  KanbanAction / ListAction / CardAction                         │
│                           ↓                                      │
│  请求层 (MatrixRequest 扩展)                                     │
│  KanbanRequest / ListRequest / CardRequest                      │
│                           ↓                                      │
│  Worker 层                                                       │
│  matrix_worker_task                                             │
│                           ↓                                      │
│  Matrix SDK                                                     │
│  matrix_sdk::Client                                             │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. 现有 API 分析

### 2.1 现有 MatrixRequest 枚举结构

当前 `src/sliding_sync.rs` 中定义了约 30+ 种请求类型：

```rust
// 现有 API 分类

// 认证相关
Login,
Logout,
SpawnSSOServer,

// 房间管理
CreateRoom,
JoinRoom,
LeaveRoom,
Knock,
InviteUser,
GetRoomPreview,
GetRoomMembers,
GetSuccessorRoomDetails,

// 消息操作
SendMessage,
EditMessage,
RedactMessage,
PaginateRoomTimeline,
FetchDetailsForEvent,
ToggleReaction,

// 状态管理
SendTypingNotice,
ReadReceipt,
FullyReadReceipt,
PinEvent,
SubscribeToTypingNotices,
SubscribeToPinnedEvents,
SubscribeToOwnUserReadReceiptsChanged,

// 用户管理
GetUserProfile,
IgnoreUser,
SyncRoomMemberList,

// 媒体相关
FetchAvatar,
FetchMedia,
GetUrlPreview,

// 其他
ResolveRoomAlias,
GetMatrixRoomLinkPillInfo,
GetNumberUnreadMessages,
GetRoomPowerLevels,
```

### 2.2 API 使用模式

所有 API 调用都遵循以下模式：

```rust
// 1. UI 层发起请求
submit_async_request(MatrixRequest::OperationName {
    param1: value1,
    param2: value2,
    // ...
})

// 2. Worker 线程处理
while let Some(request) = request_receiver.recv().await {
    match request {
        MatrixRequest::OperationName { param1, param2, .. } => {
            // 调用 Matrix SDK
            let result = operation(param1, param2).await;
            
            // 发送结果或错误
            sender.send(result.into()).unwrap();
        }
        // ...
    }
}
```

### 2.3 响应处理

```rust
// 结果通过 Actions 发送回 UI
match result {
    Ok(data) => {
        sender.send(Ok(KanbanAction::Update(data))).unwrap();
    }
    Err(e) => {
        sender.send(Err(e.to_string())).unwrap();
    }
}
```

---

## 3. API 扩展设计

### 3.1 扩展策略

我们通过以下方式扩展 API：

1. **新增枚举变体**: 在 `MatrixRequest` 中添加看板相关变体
2. **结构体参数**: 使用结构体封装复杂参数
3. **泛型响应**: 使用结果类型统一错误处理

### 3.2 新增请求类型

```rust
// src/sliding_sync.rs 新增

/// ============================================
/// 看板管理请求 (KanbanRequest)
/// ============================================

/// 看板相关操作请求
#[derive(Debug, Clone)]
pub enum KanbanRequest {
    /// 创建看板
    CreateBoard {
        name: String,
        description: Option<String>,
        /// 背景颜色
        background_color: Option<String>,
        /// 初始成员
        invite: Vec<String>,
    },
    
    /// 获取看板列表
    GetBoards {
        /// 是否包含归档的看板
        include_archived: bool,
    },
    
    /// 获取单个看板
    GetBoard {
        board_id: OwnedRoomId,
    },
    
    /// 更新看板
    UpdateBoard {
        board_id: OwnedRoomId,
        updates: BoardUpdateRequest,
    },
    
    /// 删除看板
    DeleteBoard {
        board_id: OwnedRoomId,
        /// 是否彻底删除
        permanent: bool,
    },
    
    /// 归档/取消归档看板
    ArchiveBoard {
        board_id: OwnedRoomId,
        archived: bool,
    },
    
    /// 添加看板成员
    AddBoardMember {
        board_id: OwnedRoomId,
        user_id: OwnedUserId,
        role: MemberRole,
    },
    
    /// 移除看板成员
    RemoveBoardMember {
        board_id: OwnedRoomId,
        user_id: OwnedUserId,
    },
    
    /// 更新看板成员角色
    UpdateBoardMemberRole {
        board_id: OwnedRoomId,
        user_id: OwnedUserId,
        role: MemberRole,
    },
    
    /// 添加看板标签
    AddBoardLabel {
        board_id: OwnedRoomId,
        label: BoardLabel,
    },
    
    /// 更新看板标签
    UpdateBoardLabel {
        board_id: OwnedRoomId,
        label_id: String,
        updates: LabelUpdateRequest,
    },
    
    /// 删除看板标签
    RemoveBoardLabel {
        board_id: OwnedRoomId,
        label_id: String,
    },
}

/// ============================================
/// 列表管理请求 (ListRequest)
/// ============================================

/// 列表相关操作请求
#[derive(Debug, Clone)]
pub enum ListRequest {
    /// 创建列表
    CreateList {
        board_id: OwnedRoomId,
        name: String,
        /// 插入位置
        position: Option<f64>,
    },
    
    /// 获取列表
    GetLists {
        board_id: OwnedRoomId,
    },
    
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
        /// 是否归档而不是删除
        archive: bool,
    },
    
    /// 移动列表
    MoveList {
        board_id: OwnedRoomId,
        list_id: String,
        new_position: f64,
    },
    
    /// 批量移动列表
    BatchMoveLists {
        board_id: OwnedRoomId,
        moves: Vec<ListMove>,
    },
    
    /// 复制列表
    CopyList {
        board_id: OwnedRoomId,
        list_id: String,
        new_name: String,
        /// 是否包含卡片
        copy_cards: bool,
    },
}

/// ============================================
/// 卡片管理请求 (CardRequest)
/// ============================================

/// 卡片相关操作请求
#[derive(Debug, Clone)]
pub enum CardRequest {
    /// 创建卡片
    CreateCard {
        board_id: OwnedRoomId,
        list_id: String,
        name: String,
        description: Option<String>,
        /// 位置
        position: Option<f64>,
        /// 成员
        member_ids: Vec<String>,
        /// 截止日期
        due_date: Option<String>,
        /// 标签
        label_ids: Vec<String>,
    },
    
    /// 获取卡片
    GetCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
    },
    
    /// 获取列表中的卡片
    GetCards {
        board_id: OwnedRoomId,
        list_id: String,
        /// 分页参数
        limit: u32,
        from: Option<String>,
    },
    
    /// 更新卡片
    UpdateCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        updates: CardUpdateRequest,
    },
    
    /// 删除卡片
    DeleteCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        /// 是否归档而不是删除
        archive: bool,
    },
    
    /// 移动卡片
    MoveCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        from_list_id: String,
        to_list_id: String,
        new_position: f64,
    },
    
    /// 批量移动卡片
    BatchMoveCards {
        board_id: OwnedRoomId,
        moves: Vec<CardMove>,
    },
    
    /// 复制卡片
    CopyCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        target_list_id: String,
        new_name: Option<String>,
    },
    
    /// 归档/取消归档卡片
    ArchiveCard {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        archived: bool,
    },
    
    /// 切换加星标
    ToggleStar {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        starred: bool,
    },
    
    /// 设置截止日期
    SetDueDate {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        due_date: Option<String>,
    },
    
    /// 标记截止日期完成
    CompleteDueDate {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        completed: bool,
    },
    
    /// 添加标签
    AddCardLabel {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        label_id: String,
    },
    
    /// 移除标签
    RemoveCardLabel {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        label_id: String,
    },
    
    /// 添加成员
    AddCardMember {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        user_id: OwnedUserId,
    },
    
    /// 移除成员
    RemoveCardMember {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        user_id: OwnedUserId,
    },
    
    /// 设置封面
    SetCover {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        cover: Option<CoverUpdate>,
    },
    
    /// 添加附件
    AddAttachment {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        attachment: AttachmentInfo,
    },
    
    /// 移除附件
    RemoveAttachment {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        attachment_id: String,
    },
    
    /// 添加评论
    AddComment {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        text: String,
    },
    
    /// 编辑评论
    EditComment {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        comment_id: String,
        text: String,
    },
    
    /// 删除评论
    DeleteComment {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        comment_id: String,
    },
    
    /// 添加检查清单
    AddChecklist {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        name: String,
    },
    
    /// 添加检查项
    AddChecklistItem {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        checklist_id: String,
        item: ChecklistItemInfo,
    },
    
    /// 切换检查项
    ToggleChecklistItem {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        checklist_id: String,
        item_id: String,
        completed: bool,
    },
    
    /// 删除检查清单
    RemoveChecklist {
        board_id: OwnedRoomId,
        card_id: OwnedEventId,
        checklist_id: String,
    },
}
```

### 3.3 请求参数结构体

```rust
// src/sliding_sync.rs 新增结构体

/// 看板更新请求
#[derive(Debug, Clone, Default)]
pub struct BoardUpdateRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub background_color: Option<String>,
    pub background_image: Option<Option<String>>,
    pub visibility: Option<BoardVisibility>,
}

/// 标签更新请求
#[derive(Debug, Clone, Default)]
pub struct LabelUpdateRequest {
    pub name: Option<String>,
    pub color: Option<LabelColor>,
}

/// 列表更新请求
#[derive(Debug, Clone, Default)]
pub struct ListUpdateRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub card_limit: Option<Option<u32>>,
}

/// 卡片更新请求
#[derive(Debug, Clone, Default)]
pub struct CardUpdateRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub order_index: Option<f64>,
    pub due_date: Option<Option<CardDueDate>>,
    pub cover: Option<Option<CardCover>>,
}

/// 列表移动
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMove {
    pub list_id: String,
    pub new_position: f64,
}

/// 卡片移动
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardMove {
    pub card_id: OwnedEventId,
    pub from_list_id: String,
    pub to_list_id: String,
    pub new_position: f64,
}

/// 封面更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverUpdate {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    pub is_full_width: bool,
}

/// 附件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInfo {
    pub name: String,
    pub url: String,
    pub content_type: String,
    pub size: u64,
}

/// 检查项信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItemInfo {
    pub text: String,
    #[serde(default)]
    pub position: u32,
}

/// 成员角色
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemberRole {
    #[default]
    Member,
    Admin,
    Owner,
}

/// 卡片截止日期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDueDate {
    pub due_date: String,
    #[serde(default)]
    pub is_completed: bool,
}

/// 卡片封面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCover {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    pub is_full_width: bool,
}
```

### 3.4 响应类型

```rust
// src/sliding_sync.rs 新增

/// API 响应类型
#[derive(Debug, Clone)]
pub enum KanbanResponse {
    /// 看板相关响应
    Board(KanbanBoard),
    Boards(Vec<KanbanBoard>),
    
    /// 列表相关响应
    List(KanbanList),
    Lists(Vec<KanbanList>),
    
    /// 卡片相关响应
    Card(KanbanCard),
    Cards(Vec<KanbanCard>),
    
    /// 操作成功 (无返回数据)
    Success,
    
    /// 操作失败
    Error(String),
}

impl From<Result<KanbanBoard, String>> for KanbanResponse {
    fn from(result: Result<KanbanBoard, String>) -> Self {
        match result {
            Ok(board) => KanbanResponse::Board(board),
            Err(e) => KanbanResponse::Error(e),
        }
    }
}

impl From<Result<Vec<KanbanBoard>, String>> for KanbanResponse {
    fn from(result: Result<Vec<KanbanBoard>, String>) -> Self {
        match result {
            Ok(boards) => KanbanResponse::Boards(boards),
            Err(e) => KanbanResponse::Error(e),
        }
    }
}

impl From<Result<KanbanList, String>> for KanbanResponse {
    fn from(result: Result<KanbanList, String>) -> Self {
        match result {
            Ok(list) => KanbanResponse::List(list),
            Err(e) => KanbanResponse::Error(e),
        }
    }
}

impl From<Result<Vec<KanbanList>, String>> for KanbanResponse {
    fn from(result: Result<Vec<KanbanList>, String>) -> Self {
        match result {
            Ok(lists) => KanbanResponse::Lists(lists),
            Err(e) => KanbanResponse::Error(e),
        }
    }
}

impl From<Result<KanbanCard, String>> for KanbanResponse {
    fn from(result: Result<KanbanCard, String>) -> Self {
        match result {
            Ok(card) => KanbanResponse::Card(card),
            Err(e) => KanbanResponse::Error(e),
        }
    }
}

impl From<Result<Vec<KanbanCard>, String>> for KanbanResponse {
    fn from(result: Result<Vec<KanbanCard>, String>) -> Self {
        match result {
            Ok(cards) => KanbanResponse::Cards(cards),
            Err(e) => KanbanResponse::Error(e),
        }
    }
}

impl From<Result<(), String>> for KanbanResponse {
    fn from(result: Result<(), String>) -> Self {
        match result {
            Ok(()) => KanbanResponse::Success,
            Err(e) => KanbanResponse::Error(e),
        }
    }
}
```

---

## 4. 完整 API 映射表

### 4.1 看板操作映射

| Trello 操作 | Matrix API | 请求类型 | 实现说明 |
|-------------|------------|----------|----------|
| 创建看板 | `CreateRoom` | `KanbanRequest::CreateBoard` | 复用 Room 创建 API，添加看板元数据 |
| 重命名看板 | `SendStateEvent` | `KanbanRequest::UpdateBoard` | 发送状态事件更新 Room 名称 |
| 修改描述 | `SendStateEvent` | `KanbanRequest::UpdateBoard` | 发送状态事件更新 Room 主题 |
| 修改背景 | `SendStateEvent` | `KanbanRequest::UpdateBoard` | 发送状态事件更新背景 |
| 删除看板 | `LeaveRoom` | `KanbanRequest::DeleteBoard` | 离开 Room，或标记为归档 |
| 归档看板 | `SendStateEvent` | `KanbanRequest::ArchiveBoard` | 发送归档状态事件 |
| 获取看板 | `RoomList` | `KanbanRequest::GetBoard` | 从 Room 列表筛选 |
| 获取列表 | - | `KanbanRequest::GetBoards` | 从本地存储获取 |
| 添加成员 | `InviteUser` | `KanbanRequest::AddBoardMember` | 直接复用 |
| 移除成员 | `KickUser` | `KanbanRequest::RemoveBoardMember` | 直接复用 |
| 添加标签 | `SendStateEvent` | `KanbanRequest::AddBoardLabel` | 发送标签状态事件 |
| 移除标签 | `SendStateEvent` | `KanbanRequest::RemoveBoardLabel` | 发送状态事件删除 |

### 4.2 列表操作映射

| Trello 操作 | Matrix API | 请求类型 | 实现说明 |
|-------------|------------|----------|----------|
| 创建列表 | - | `ListRequest::CreateList` | 本地存储，不调用 Matrix API |
| 重命名列表 | - | `ListRequest::UpdateList` | 本地存储更新 |
| 删除列表 | - | `ListRequest::DeleteList` | 本地存储更新 |
| 移动列表 | - | `ListRequest::MoveList` | 更新 order_index |
| 复制列表 | - | `ListRequest::CopyList` | 本地存储操作 |
| 归档列表 | - | `ListRequest::DeleteList` | 设置 is_archived 标志 |

### 4.3 卡片操作映射

| Trello 操作 | Matrix API | 请求类型 | 实现说明 |
|-------------|------------|----------|----------|
| 创建卡片 | `SendMessage` | `CardRequest::CreateCard` | 发送消息，附加扩展字段 |
| 编辑标题 | `EditMessage` | `CardRequest::UpdateCard` | 复用消息编辑 |
| 编辑描述 | `EditMessage` | `CardRequest::UpdateCard` | 复用消息编辑 |
| 删除卡片 | `RedactMessage` | `CardRequest::DeleteCard` | 复用消息删除 |
| 移动卡片 | `EditMessage` | `CardRequest::MoveCard` | 更新扩展字段中的 list_id |
| 批量移动 | `BatchEdit` | `CardRequest::BatchMoveCards` | 批量更新扩展字段 |
| 归档卡片 | `SendStateEvent` | `CardRequest::ArchiveCard` | 发送归档状态事件 |
| 加星标 | `SendStateEvent` | `CardRequest::ToggleStar` | 发送星标状态事件 |
| 设置截止日期 | - | `CardRequest::SetDueDate` | 更新扩展字段 |
| 添加标签 | `SendStateEvent` | `CardRequest::AddCardLabel` | 发送标签状态事件 |
| 移除标签 | `SendStateEvent` | `CardRequest::RemoveCardLabel` | 发送状态事件 |
| 添加成员 | `@mention` | `CardRequest::AddCardMember` | 发送带 @mention 的消息 |
| 移除成员 | `EditMessage` | `CardRequest::RemoveCardMember` | 移除 @mention |
| 设置封面 | `UploadMedia` | `CardRequest::SetCover` | 上传图片，更新扩展字段 |
| 添加附件 | `UploadMedia` | `CardRequest::AddAttachment` | 上传媒体，发送附件消息 |
| 移除附件 | `RedactMessage` | `CardRequest::RemoveAttachment` | 删除附件消息 |
| 添加评论 | `SendMessage` | `CardRequest::AddComment` | 发送线程回复 |
| 编辑评论 | `EditMessage` | `CardRequest::EditComment` | 复用消息编辑 |
| 删除评论 | `RedactMessage` | `CardRequest::DeleteComment` | 复用消息删除 |
| 添加检查清单 | - | `CardRequest::AddChecklist` | 发送子消息 |
| 添加检查项 | `SendMessage` | `CardRequest::AddChecklistItem` | 发送子消息 |
| 切换检查项 | `EditMessage` | `CardRequest::ToggleChecklistItem` | 更新子消息 |
| 删除检查清单 | `RedactMessage` | `CardRequest::RemoveChecklist` | 删除子消息 |
| 复制卡片 | - | `CardRequest::CopyCard` | 读取后创建新卡片 |

### 4.4 查询操作映射

| Trello 操作 | Matrix API | 请求类型 | 实现说明 |
|-------------|------------|----------|----------|
| 获取看板 | `RoomListService` | `KanbanRequest::GetBoards` | 复用 Room 列表 |
| 获取列表 | - | `ListRequest::GetLists` | 从本地存储获取 |
| 获取卡片 | `PaginateTimeline` | `CardRequest::GetCards` | 复用分页 API |
| 搜索卡片 | `RoomSearch` | `SearchCards` | 复用搜索 API |
| 获取成员 | `GetRoomMembers` | `KanbanRequest::GetBoard` | 复用成员获取 |
| 获取详情 | `FetchDetails` | `CardRequest::GetCard` | 复用详情获取 |

---

## 5. 请求处理流程

### 5.1 Worker 线程处理逻辑

```rust
// src/sliding_sync.rs

/// 处理看板相关请求
async fn handle_kanban_request(
    &self,
    request: KanbanRequest,
    sender: &tokio::sync::mpsc::Sender<Result<KanbanResponse, String>>,
) {
    match request {
        KanbanRequest::CreateBoard { name, description, background_color, invite } => {
            let result = self.create_board(name, description, background_color, invite).await;
            sender.send(result.into()).unwrap();
        }
        
        KanbanRequest::GetBoards { include_archived } => {
            let result = self.get_boards(include_archived).await;
            sender.send(result.into()).unwrap();
        }
        
        KanbanRequest::GetBoard { board_id } => {
            let result = self.get_board(&board_id).await;
            sender.send(result.into()).unwrap();
        }
        
        KanbanRequest::UpdateBoard { board_id, updates } => {
            let result = self.update_board(&board_id, updates).await;
            sender.send(result.into()).unwrap();
        }
        
        KanbanRequest::DeleteBoard { board_id, permanent } => {
            let result = self.delete_board(&board_id, permanent).await;
            sender.send(result.into()).unwrap();
        }
        
        // ... 其他请求类型
    }
}

/// 创建看板
async fn create_board(
    &self,
    name: String,
    description: Option<String>,
    background_color: Option<String>,
    invite: Vec<String>,
) -> Result<KanbanBoard, String> {
    // 1. 调用 Matrix SDK 创建 Room
    let request = CreateRoomRequest::new()
        .name(name.clone())
        .topic(description.clone())
        .invite(invite.iter().map(|s| UserId::parse(s)).collect::<Result<_, _>>()?);
    
    let room = self.client.create_room(request).await
        .map_err(|e| e.to_string())?;
    
    // 2. 设置看板背景
    if let Some(color) = background_color {
        let bg_event = BackgroundColorStateEventContent::new(color);
        self.client.send_state_event(&room.room_id(), &bg_event, "").await
            .map_err(|e| e.to_string())?;
    }
    
    // 3. 构建看板对象
    let board = KanbanBoard {
        id: room.room_id().to_owned(),
        name,
        description,
        background_color: background_color.unwrap_or_else(|| default_background_color()),
        labels: Vec::new(),
        member_ids: invite,
        list_ids: Vec::new(),
        is_archived: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        ..Default::default()
    };
    
    // 4. 保存到本地存储
    self.persistence.save_board(&board.id, &board).await?;
    
    Ok(board)
}

/// 获取看板列表
async fn get_boards(
    &self,
    include_archived: bool,
) -> Result<Vec<KanbanBoard>, String> {
    // 1. 获取所有 Room
    let rooms = self.client.room_list_service().entries();
    
    // 2. 转换为看板
    let mut boards: Vec<KanbanBoard> = rooms
        .iter()
        .filter(|room| {
            // 过滤归档的看板
            if !include_archived {
                !room.is_low_priority() && !room.is_hidden_from_home()
            } else {
                true
            }
        })
        .map(|room| self.map_room_to_board(room))
        .collect();
    
    // 3. 按名称排序
    boards.sort_by(|a, b| a.name.cmp(&b.name));
    
    // 4. 从本地存储恢复列表结构
    for board in &mut boards {
        if let Some(persistence) = self.persistence.load_board(&board.id).await? {
            board.list_ids = persistence.lists.iter().map(|l| l.list.id.clone()).collect();
        }
    }
    
    Ok(boards)
}

/// 映射 Room 到看板
fn map_room_to_board(&self, room: &RoomListEntry) -> KanbanBoard {
    KanbanBoard {
        id: room.room_id().to_owned(),
        name: room.name().unwrap_or_default().to_string(),
        description: room.topic().map(|s| s.to_string()),
        background_color: room.avatar_url()
            .as_ref()
            .map_or_else(|| default_background_color(), |_| "#0079BF".to_string()),
        labels: Vec::new(),
        member_ids: room.active_members().clone(),
        list_ids: Vec::new(),
        is_archived: false,
        created_at: room.created_at().unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
        updated_at: room.updated_at().unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    }
}
```

### 5.2 列表操作处理

```rust
// src/sliding_sync.rs

/// 处理列表请求
async fn handle_list_request(
    &self,
    request: ListRequest,
    sender: &tokio::sync::mpsc::Sender<Result<KanbanResponse, String>>,
) {
    match request {
        ListRequest::CreateList { board_id, name, position } => {
            let result = self.create_list(&board_id, &name, position).await;
            sender.send(result.into()).unwrap();
        }
        
        ListRequest::GetLists { board_id } => {
            let result = self.get_lists(&board_id).await;
            sender.send(result.into()).unwrap();
        }
        
        ListRequest::UpdateList { board_id, list_id, updates } => {
            let result = self.update_list(&board_id, &list_id, updates).await;
            sender.send(result.into()).unwrap();
        }
        
        ListRequest::DeleteList { board_id, list_id, archive } => {
            let result = self.delete_list(&board_id, &list_id, archive).await;
            sender.send(result.into()).unwrap();
        }
        
        ListRequest::MoveList { board_id, list_id, new_position } => {
            let result = self.move_list(&board_id, &list_id, new_position).await;
            sender.send(result.into()).unwrap();
        }
        
        ListRequest::BatchMoveLists { board_id, moves } => {
            let result = self.batch_move_lists(&board_id, moves).await;
            sender.send(result.into()).unwrap();
        }
        
        ListRequest::CopyList { board_id, list_id, new_name, copy_cards } => {
            let result = self.copy_list(&board_id, &list_id, &new_name, copy_cards).await;
            sender.send(result.into()).unwrap();
        }
    }
}

/// 创建列表
async fn create_list(
    &self,
    board_id: &RoomId,
    name: &str,
    position: Option<f64>,
) -> Result<KanbanList, String> {
    // 1. 创建列表对象
    let position = position.unwrap_or_else(|| {
        let lists = self.get_lists(board_id).await.unwrap_or_default();
        lists.iter().map(|l| l.order_index).fold(0.0, |max, p| max.max(p)) + 1000.0
    });
    
    let list = KanbanList::new(board_id.to_string().as_str(), name);
    let mut list = KanbanList {
        order_index: position,
        ..list
    };
    
    // 2. 加载现有持久化数据
    let mut persistence = self.persistence
        .load_board(board_id)
        .await?
        .unwrap_or_default();
    
    // 3. 添加列表
    persistence.lists.push(KanbanListPersistence {
        list: list.clone(),
        extensions: ListExtensions::default(),
    });
    
    // 4. 更新看板的 list_ids
    let board_meta = &mut persistence.board_meta;
    board_meta.list_ids.push(list.id.clone());
    
    // 5. 保存到本地存储
    self.persistence.save_board(board_id, &persistence).await?;
    
    Ok(list)
}

/// 获取列表
async fn get_lists(&self, board_id: &RoomId) -> Result<Vec<KanbanList>, String> {
    // 从本地存储获取
    let persistence = self.persistence
        .load_board(board_id)
        .await?
        .unwrap_or_default();
    
    // 按 order_index 排序
    let mut lists: Vec<_> = persistence.lists
        .into_iter()
        .map(|p| p.list)
        .filter(|l| !l.is_archived)
        .collect();
    
    lists.sort_by(|a, b| a.order_index.partial_cmp(&b.order_index).unwrap());
    
    Ok(lists)
}

/// 移动列表
async fn move_list(
    &self,
    board_id: &RoomId,
    list_id: &str,
    new_position: f64,
) -> Result<(), String> {
    // 加载持久化数据
    let mut persistence = self.persistence
        .load_board(board_id)
        .await?
        .unwrap_or_default();
    
    // 更新列表位置
    for list_persistence in &mut persistence.lists {
        if list_persistence.list.id == list_id {
            list_persistence.list.order_index = new_position;
            list_persistence.list.updated_at = chrono::Utc::now().to_rfc3339();
            break;
        }
    }
    
    // 重新排序所有列表
    persistence.lists.sort_by(|a, b| a.list.order_index.partial_cmp(&b.list.order_index).unwrap());
    
    // 更新看板的 list_ids
    persistence.board_meta.list_ids = persistence.lists
        .iter()
        .map(|p| p.list.id.clone())
        .collect();
    
    // 保存
    self.persistence.save_board(board_id, &persistence).await?;
    
    Ok(())
}
```

### 5.3 卡片操作处理

```rust
// src/sliding_sync.rs

/// 处理卡片请求
async fn handle_card_request(
    &self,
    request: CardRequest,
    sender: &tokio::sync::mpsc::Sender<Result<KanbanResponse, String>>,
) {
    match request {
        CardRequest::CreateCard { board_id, list_id, name, description, position, member_ids, due_date, label_ids } => {
            let result = self.create_card(&board_id, &list_id, &name, description, position, member_ids, due_date, label_ids).await;
            sender.send(result.into()).unwrap();
        }
        
        CardRequest::GetCards { board_id, list_id, limit, from } => {
            let result = self.get_cards(&board_id, &list_id, limit, from).await;
            sender.send(result.into()).unwrap();
        }
        
        CardRequest::UpdateCard { board_id, card_id, updates } => {
            let result = self.update_card(&board_id, &card_id, updates).await;
            sender.send(result.into()).unwrap();
        }
        
        CardRequest::MoveCard { board_id, card_id, from_list_id, to_list_id, new_position } => {
            let result = self.move_card(&board_id, &card_id, &from_list_id, &to_list_id, new_position).await;
            sender.send(result.into()).unwrap();
        }
        
        CardRequest::BatchMoveCards { board_id, moves } => {
            let result = self.batch_move_cards(&board_id, moves).await;
            sender.send(result.into()).unwrap();
        }
        
        CardRequest::DeleteCard { board_id, card_id, archive } => {
            let result = self.delete_card(&board_id, &card_id, archive).await;
            sender.send(result.into()).unwrap();
        }
        
        CardRequest::AddComment { board_id, card_id, text } => {
            let result = self.add_comment(&board_id, &card_id, &text).await;
            sender.send(result.into()).unwrap();
        }
        
        // ... 其他请求类型
    }
}

/// 创建卡片
async fn create_card(
    &self,
    board_id: &RoomId,
    list_id: &str,
    name: &str,
    description: Option<String>,
    position: Option<f64>,
    member_ids: Vec<String>,
    due_date: Option<String>,
    label_ids: Vec<String>,
) -> Result<KanbanCard, String> {
    // 1. 获取 Room
    let room = self.client.get_room(board_id)
        .ok_or_else(|| "Room not found".to_string())?;
    
    // 2. 构建消息内容
    let content = RoomMessageEventContent::text_markdown(name);
    
    // 3. 构建扩展字段
    let extensions = CardExtensions {
        list_id: list_id.to_string(),
        order_index: position.unwrap_or(0.0),
        member_ids,
        label_ids,
        due_date: due_date.map(|d| ExtensionDueDate {
            due_date: d,
            is_completed: false,
            is_reminded: false,
        }),
        ..Default::default()
    };
    
    // 4. 发送消息
    let timeline = room.timeline();
    let response = timeline.send(content).await
        .map_err(|e| e.to_string())?;
    
    let event_id = response.event_id;
    
    // 5. 构建卡片对象
    let card = KanbanCard {
        id: event_id,
        title: name.to_string(),
        description,
        list_id: list_id.to_string(),
        order_index: position.unwrap_or(0.0),
        member_ids,
        label_ids,
        due_date: due_date.map(|d| CardDueDate { due_date: d, is_completed: false }),
        is_archived: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        ..Default::default()
    };
    
    // 6. 保存扩展字段到本地存储
    self.save_card_extensions(board_id, &event_id, &extensions).await?;
    
    Ok(card)
}

/// 获取卡片
async fn get_cards(
    &self,
    board_id: &RoomId,
    list_id: &str,
    limit: u32,
    from: Option<String>,
) -> Result<Vec<KanbanCard>, String> {
    // 1. 获取 Room
    let room = self.client.get_room(board_id)
        .ok_or_else(|| "Room not found".to_string())?;
    
    // 2. 获取时间线
    let timeline = room.timeline();
    
    // 3. 分页获取消息
    let events: Vec<_> = timeline
        .latest_events()
        .await
        .map_err(|e| e.to_string())?
        .iter()
        .take(limit as usize)
        .filter_map(|item| item.as_event())
        .filter(|event| {
            // 过滤指定列表的卡片
            if let Some(extensions) = self.get_card_extensions(board_id, event.event_id()).await? {
                extensions.list_id == list_id
            } else {
                false
            }
        })
        .collect();
    
    // 4. 转换为卡片
    let mut cards: Vec<KanbanCard> = events
        .iter()
        .map(|event| {
            KanbanCard {
                id: event.event_id().to_owned(),
                title: extract_title(event),
                list_id: list_id.to_string(),
                order_index: 0.0, // 从扩展字段获取
                created_by: event.sender().to_string(),
                created_at: event.origin_server_ts()
                    .map(|ts| ts.to_rfc3339())
                    .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                updated_at: event.origin_server_ts()
                    .map(|ts| ts.to_rfc3339())
                    .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                ..Default::default()
            }
        })
        .collect();
    
    // 5. 从扩展字段补全信息
    for card in &mut cards {
        if let Some(extensions) = self.get_card_extensions(board_id, &card.id).await? {
            card.order_index = extensions.order_index;
            card.member_ids = extensions.member_ids;
            card.label_ids = extensions.label_ids;
            card.due_date = extensions.due_date.map(|d| CardDueDate {
                due_date: d.due_date,
                is_completed: d.is_completed,
            });
            card.is_starred = extensions.is_starred;
        }
    }
    
    // 6. 按 order_index 排序
    cards.sort_by(|a, b| a.order_index.partial_cmp(&b.order_index).unwrap());
    
    Ok(cards)
}

/// 移动卡片
async fn move_card(
    &self,
    board_id: &RoomId,
    card_id: &EventId,
    from_list_id: &str,
    to_list_id: &str,
    new_position: f64,
) -> Result<(), String> {
    // 1. 获取扩展字段
    let mut extensions = self.get_card_extensions(board_id, card_id)
        .await?
        .unwrap_or_default();
    
    // 2. 更新扩展字段
    extensions.list_id = to_list_id.to_string();
    extensions.order_index = new_position;
    
    // 3. 保存扩展字段
    self.save_card_extensions(board_id, card_id, &extensions).await?;
    
    // 4. 发送移动通知 (可选)
    self.notify_card_moved(board_id, card_id, from_list_id, to_list_id).await;
    
    Ok(())
}

/// 批量移动卡片
async fn batch_move_cards(
    &self,
    board_id: &RoomId,
    moves: Vec<CardMove>,
) -> Result<(), String> {
    for move_op in moves {
        self.move_card(
            board_id,
            &move_op.card_id,
            &move_op.from_list_id,
            &move_op.to_list_id,
            move_op.new_position,
        ).await?;
    }
    
    // 批量更新后重新排序
    self.reorder_list(board_id, &moves.first().unwrap().to_list_id).await?;
    
    Ok(())
}
```

---

## 6. 错误处理

### 6.1 错误类型定义

```rust
// src/sliding_sync.rs

/// 看板操作错误类型
#[derive(Debug, thiserror::Error)]
pub enum KanbanError {
    #[error("Room not found: {0}")]
    RoomNotFound(String),
    
    #[error("Card not found: {0}")]
    CardNotFound(String),
    
    #[error("List not found: {0}")]
    ListNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Matrix SDK error: {0}")]
    MatrixError(#[from] matrix_sdk::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// 错误码定义
impl KanbanError {
    pub fn error_code(&self) -> &str {
        match self {
            KanbanError::RoomNotFound(_) => "KBN001",
            KanbanError::CardNotFound(_) => "KBN002",
            KanbanError::ListNotFound(_) => "KBN003",
            KanbanError::PermissionDenied(_) => "KBN004",
            KanbanError::InvalidOperation(_) => "KBN005",
            KanbanError::MatrixError(_) => "KBN006",
            KanbanError::SerializationError(_) => "KBN007",
            KanbanError::IoError(_) => "KBN008",
            KanbanError::Unknown(_) => "KBN000",
        }
    }
    
    pub fn is_retryable(&self) -> bool {
        match self {
            KanbanError::MatrixError(_) => true,
            KanbanError::IoError(_) => true,
            KanbanError::Unknown(_) => false,
            _ => false,
        }
    }
}
```

### 6.2 错误处理策略

```rust
// 错误处理示例

impl KanbanService {
    /// 带重试的操作
    async fn retry_operation<T, F, Fut>(&self, max_retries: u32, operation: F) -> Result<T, KanbanError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, KanbanError>>,
    {
        let mut last_error = None;
        
        for attempt in 0..max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e.clone());
                    if !e.is_retryable() || attempt == max_retries - 1 {
                        return Err(e);
                    }
                    // 指数退避
                    tokio::time::sleep(std::time::Duration::from_millis(100 * 2_u64.pow(attempt))).await;
                }
            }
        }
        
        last_error.unwrap_or_else(|| KanbanError::Unknown("Max retries exceeded".to_string()))
    }
    
    /// 错误恢复
    async fn handle_error(&self, error: &KanbanError, context: &str) {
        log::error!("[{}] Error: {:?}", context, error);
        
        match error {
            KanbanError::RoomNotFound(room_id) => {
                // 清理本地缓存
                self.persistence.delete_board(&RoomId::parse(room_id).unwrap()).await.ok();
            }
            KanbanError::PermissionDenied(_) => {
                // 通知用户权限不足
                self.notify_permission_error().await;
            }
            _ => {
                // 其他错误，记录并继续
            }
        }
    }
}
```

---

## 7. 示例代码

### 7.1 创建看板

```rust
// 示例：创建看板

let board = submit_async_request(KanbanRequest::CreateBoard {
    name: "产品规划".to_string(),
    description: Some("2024年产品路线图".to_string()),
    background_color: Some("#0079BF".to_string()),
    invite: vec!["@user1:matrix.com".to_string(), "@user2:matrix.com".to_string()],
}).await?;
```

### 7.2 获取看板列表

```rust
// 示例：获取看板列表

let boards = submit_async_request(KanbanRequest::GetBoards {
    include_archived: false,
}).await?;

// 处理看板列表
for board in boards {
    println!("看板: {} ({} 张卡片)", board.name, board.card_count);
}
```

### 7.3 创建列表

```rust
// 示例：在看板中创建列表

let list = submit_async_request(ListRequest::CreateList {
    board_id: board_id.clone(),
    name: "待办".to_string(),
    position: None, // 自动计算位置
}).await?;
```

### 7.4 创建卡片

```rust
// 示例：创建卡片

let card = submit_async_request(CardRequest::CreateCard {
    board_id: board_id.clone(),
    list_id: list.id.clone(),
    name: "完成用户调研".to_string(),
    description: Some("访谈10个目标用户，了解需求".to_string()),
    position: None,
    member_ids: vec!["@user1:matrix.com".to_string()],
    due_date: Some("2024-03-15".to_string()),
    label_ids: vec!["feature".to_string()],
}).await?;
```

### 7.5 移动卡片

```rust
// 示例：移动卡片到其他列表

submit_async_request(CardRequest::MoveCard {
    board_id: board_id.clone(),
    card_id: card.id.clone(),
    from_list_id: todo_list.id.clone(),
    to_list_id: doing_list.id.clone(),
    new_position: 500.0, // 在目标列表中的位置
}).await?;
```

### 7.6 批量操作

```rust
// 示例：批量移动多个卡片

let moves = vec![
    CardMove {
        card_id: card1.id.clone(),
        from_list_id: list1.id.clone(),
        to_list_id: list2.id.clone(),
        new_position: 100.0,
    },
    CardMove {
        card_id: card2.id.clone(),
        from_list_id: list1.id.clone(),
        to_list_id: list2.id.clone(),
        new_position: 200.0,
    },
];

submit_async_request(CardRequest::BatchMoveCards {
    board_id: board_id.clone(),
    moves,
}).await?;
```

---

## 附录

### A. 完整 API 列表

| 分类 | API 数量 | 说明 |
|------|----------|------|
| 看板管理 | 12 | 创建、更新、删除、归档等 |
| 列表管理 | 7 | 创建、更新、移动、复制等 |
| 卡片管理 | 25 | 创建、更新、移动、评论等 |
| 合计 | 44 | - |

### B. 性能考量

- **批量操作**: 使用批量 API 减少网络往返
- **懒加载**: 卡片按需加载，不一次性获取所有
- **本地缓存**: 列表结构本地缓存，减少 API 调用
- **乐观更新**: UI 先更新，后台异步同步

### C. 安全性

- **权限检查**: 所有操作前检查用户权限
- **输入验证**: 验证所有用户输入
- **速率限制**: 防止滥用 API
- **审计日志**: 记录敏感操作

---

> 文档版本: 1.0
> 最后更新: 2026-01-14
