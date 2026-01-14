# 分阶段实施细节文档

> Toona 项目改造 - 看板应用实施计划与细节

## 文档信息

- **版本**: 1.0
- **创建日期**: 2026-01-14
- **状态**: 设计稿

---

## 目录

1. [项目准备](#1-项目准备)
2. [阶段一：基础框架](#2-阶段一基础框架)
3. [阶段二：看板管理](#3-阶段二看板管理)
4. [阶段三：卡片管理](#4-阶段三卡片管理)
5. [阶段四：交互增强](#5-阶段四交互增强)
6. [阶段五：协作同步](#6-阶段五协作同步)
7. [测试策略](#7-测试策略)
8. [风险缓解](#8-风险缓解)
9. [里程碑与时间线](#9-里程碑与时间线)

---

## 1. 项目准备

### 1.1 开发环境检查

#### 必需工具

| 工具 | 最低版本 | 检查命令 | 安装说明 |
|------|----------|----------|----------|
| Rust | 1.75+ | `rustc --version` | rustup 安装 |
| Cargo | 1.75+ | `cargo --version` | rustup 安装 |
| Git | 2.0+ | `git --version` | Git 官网 |
| Code Editor | - | - | RustRover/VS Code |

#### 可选工具

| 工具 | 用途 | 安装说明 |
|------|------|----------|
| rust-analyzer | 代码补全 | VS Code 扩展 |
| rustfmt | 代码格式化 | `rustup component add rustfmt` |
| clippy | 代码检查 | `rustup component add clippy` |

#### 验证命令

```bash
# 检查 Rust 环境
rustc --version    # 应为 1.75 或更高
cargo --version    # 应为 1.75 或更高

# 检查工具链
rustup show
rustup toolchain list

# 检查 Makepad 依赖
cargo tree | grep makepad
```

### 1.2 项目结构准备

#### 创建目录结构

```
src/
├── kanban/
│   ├── mod.rs                    # 模块入口
│   ├── kanban_app.rs             # 应用主入口
│   ├── state/
│   │   ├── mod.rs
│   │   ├── kanban_state.rs       # 看板状态
│   │   └── kanban_actions.rs     # 看板动作
│   ├── data/
│   │   ├── mod.rs
│   │   ├── models.rs             # 数据模型
│   │   └── repositories.rs       # 数据仓储
│   ├── api/
│   │   ├── mod.rs
│   │   └── kanban_requests.rs    # API 请求
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── kanban_workspace.rs   # 工作区
│   │   └── components/           # 组件
│   └── drag_drop/
│       ├── mod.rs
│       ├── order_manager.rs      # 排序算法
│       └── drag_handler.rs       # 拖拽处理
│
└── persistence/
    ├── mod.rs
    └── kanban_persistence.rs     # 数据持久化
```

#### Cargo.toml 依赖更新

```toml
[package]
name = "toona"
version = "0.1.0"
edition = "2024"

[dependencies]
# 现有依赖
matrix-sdk = { version = "0.7", features = ["sliding-sync"] }
makepad-widgets = "0.3"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"

# 新增依赖
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
dashmap = "6.0"  # 并发 HashMap
lru = "1.0"      # LRU 缓存
```

### 1.3 现有代码分析

#### 需要修改的文件

| 文件 | 修改内容 | 优先级 |
|------|----------|--------|
| `src/sliding_sync.rs` | 添加看板相关 API | 高 |
| `src/app.rs` | 集成看板应用 | 高 |
| `src/lib.rs` | 导出新模块 | 高 |
| `src/home/home_screen.rs` | 添加看板入口 | 中 |
| `Cargo.toml` | 添加依赖 | 高 |

#### 需要创建的文件

| 文件 | 说明 | 优先级 |
|------|------|--------|
| `src/kanban/mod.rs` | 模块入口 | 高 |
| `src/kanban/data/models.rs` | 数据模型 | 高 |
| `src/kanban/api/requests.rs` | API 定义 | 高 |
| `src/kanban/ui/workspace.rs` | 工作区 | 高 |
| `src/kanban/drag_drop/mod.rs` | 拖拽系统 | 高 |

---

## 2. 阶段一：基础框架

**时间**: 第 1-3 天  
**目标**: 建立看板应用的基础框架

### 2.1 任务清单

#### Day 1: 模块搭建

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T1.1.1 | 创建 `src/kanban/` 目录结构 | 1h | - |
| T1.1.2 | 创建 `kanban/mod.rs` 模块入口 | 0.5h | - |
| T1.1.3 | 创建 `kanban_app.rs` 主入口 | 1h | - |
| T1.1.4 | 在 `lib.rs` 中导出 kanban 模块 | 0.5h | - |
| T1.1.5 | 添加必要依赖到 `Cargo.toml` | 0.5h | - |

#### Day 2: 数据模型

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T1.2.1 | 创建 `KanbanBoard` 结构体 | 1.5h | - |
| T1.2.2 | 创建 `KanbanList` 结构体 | 1h | T1.2.1 |
| T1.2.3 | 创建 `KanbanCard` 结构体 | 1.5h | T1.2.1 |
| T1.2.4 | 创建 `KanbanLabel` 结构体 | 0.5h | - |
| T1.2.5 | 实现 `Default` trait | 1h | T1.2.1-4 |

#### Day 3: 基础状态

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T1.3.1 | 创建 `KanbanAppState` 状态结构 | 1h | - |
| T1.3.2 | 创建 `KanbanActions` 动作枚举 | 1h | - |
| T1.3.3 | 实现状态管理与更新逻辑 | 1.5h | T1.3.1 |
| T1.3.4 | 编写基础单元测试 | 1h | T1.3.1-3 |

### 2.2 详细实现

#### 任务 T1.2.1: KanbanBoard 结构体

```rust
// src/kanban/data/models.rs

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use crate::kanban::data::LabelColor;

/// 看板数据模型
#[derive(Debug, Clone, Serialize, Deserialize, LiveHook)]
#[live_register_type(panic_recovery)]
pub struct KanbanBoard {
    /// 看板 ID (对应 Matrix Room ID)
    #[live]
    pub id: OwnedRoomId,
    
    /// 看板名称
    #[live]
    pub name: String,
    
    /// 看板描述
    #[live]
    pub description: Option<String>,
    
    /// 背景颜色
    #[live]
    pub background_color: String,
    
    /// 背景图片 URL
    #[live]
    pub background_image: Option<String>,
    
    /// 标签列表
    #[live]
    pub labels: Vec<KanbanLabel>,
    
    /// 成员 ID 列表
    #[live]
    pub member_ids: Vec<OwnedUserId>,
    
    /// 列表 ID 列表 (按顺序)
    #[live]
    pub list_ids: Vec<String>,
    
    /// 是否已归档
    #[live]
    pub is_archived: bool,
    
    /// 创建时间
    #[live]
    pub created_at: String,
    
    /// 更新时间
    #[live]
    pub updated_at: String,
    
    /// 扩展数据 (本地存储)
    #[live]
    pub extensions: BoardExtensions,
}

impl Default for KanbanBoard {
    fn default() -> Self {
        Self {
            id: OwnedRoomId::from("!dummy:matrix.local"),
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

/// 看板扩展数据 (仅本地存储)
#[derive(Debug, Clone, Default, Serialize, Deserialize, LiveHook)]
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

/// 卡片视图模式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CardViewMode {
    Compact,   // 紧凑模式
    Detailed,  // 详细模式
    Cover,     // 封面模式
}
```

#### 任务 T1.3.2: KanbanActions 动作枚举

```rust
// src/kanban/state/kanban_actions.rs

use super::*;

/// 看板应用动作
#[derive(Debug, Clone, LiveAction)]
pub enum KanbanActions {
    /// 加载看板列表
    LoadBoards,
    
    /// 选择看板
    SelectBoard(OwnedRoomId),
    
    /// 创建看板
    CreateBoard {
        name: String,
        description: Option<String>,
    },
    
    /// 更新看板
    UpdateBoard {
        board_id: OwnedRoomId,
        updates: BoardUpdateRequest,
    },
    
    /// 删除看板
    DeleteBoard {
        board_id: OwnedRoomId,
    },
    
    /// 加载列表
    LoadLists {
        board_id: OwnedRoomId,
    },
    
    /// 创建列表
    CreateList {
        board_id: OwnedRoomId,
        name: String,
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
    },
    
    /// 移动列表
    MoveList {
        board_id: OwnedRoomId,
        list_id: String,
        new_position: f64,
    },
    
    /// 加载卡片
    LoadCards {
        board_id: OwnedRoomId,
        list_id: String,
    },
    
    /// 创建卡片
    CreateCard {
        board_id: OwnedRoomId,
        list_id: String,
        name: String,
    },
    
    /// 更新卡片
    UpdateCard {
        board_id: OwnedRoomId,
        card_id: String,
        updates: CardUpdateRequest,
    },
    
    /// 删除卡片
    DeleteCard {
        board_id: OwnedRoomId,
        card_id: String,
    },
    
    /// 移动卡片
    MoveCard {
        board_id: OwnedRoomId,
        card_id: String,
        from_list: String,
        to_list: String,
        new_position: f64,
    },
    
    /// 归档卡片
    ArchiveCard {
        board_id: OwnedRoomId,
        card_id: String,
        archived: bool,
    },
    
    /// 设置筛选条件
    SetFilter(KanbanFilterState),
    
    /// 设置排序条件
    SetSort(KanbanSortState),
    
    /// 搜索卡片
    Search {
        board_id: OwnedRoomId,
        query: String,
    },
    
    /// 错误处理
    Error(String),
    
    /// 加载中
    Loading(bool),
}
```

### 2.3 验收标准

- [ ] `kanban/` 模块可正常导入
- [ ] 数据模型可通过序列化/反序列化
- [ ] 状态更新可正常触发
- [ ] 基础测试通过率 100%
- [ ] 代码覆盖率 > 80%

### 2.4 输出物

| 类型 | 文件 | 说明 |
|------|------|------|
| 代码 | `src/kanban/mod.rs` | 模块入口 |
| 代码 | `src/kanban/data/models.rs` | 数据模型 |
| 代码 | `src/kanban/state/kanban_actions.rs` | 动作定义 |
| 测试 | `tests/kanban_basic_test.rs` | 基础测试 |
| 文档 | `docs/01_DATA_MODEL.md` | 数据模型文档 |

---

## 3. 阶段二：看板管理

**时间**: 第 4-7 天  
**目标**: 实现看板的 CRUD 操作和 UI

### 3.1 任务清单

#### Day 4: API 与仓储

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T2.4.1 | 创建看板相关 API 请求类型 | 2h | T1.2.1 |
| T2.4.2 | 实现 BoardRepository | 2h | T2.4.1 |
| T2.4.3 | 集成 Matrix SDK 创建 Room | 1.5h | T2.4.2 |
| T2.4.4 | 实现看板列表查询 | 1h | T2.4.2 |

#### Day 5: UI 框架

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T2.5.1 | 创建 KanbanWorkspace 组件 | 2h | T1.1.3 |
| T2.5.2 | 创建 BoardsSidebar 组件 | 2h | T2.5.1 |
| T2.5.3 | 实现看板列表 UI | 1.5h | T2.5.2 |
| T2.5.4 | 创建 BoardHeader 组件 | 1h | T2.5.1 |

#### Day 6: 看板详情

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T2.6.1 | 创建 KanbanBoardView 组件 | 2h | T2.5.1 |
| T2.6.2 | 实现看板背景设置 | 1.5h | T2.6.1 |
| T2.6.3 | 创建 BoardToolbar 组件 | 1.5h | T2.6.1 |
| T2.6.4 | 实现筛选/搜索功能 | 1.5h | T2.6.3 |

#### Day 7: 看板菜单与整合

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T2.7.1 | 创建 BoardMenu 组件 | 1.5h | T2.6.1 |
| T2.7.2 | 实现成员管理 UI | 1.5h | T2.7.1 |
| T2.7.3 | 集成到主应用 | 2h | - |
| T2.7.4 | 端到端测试 | 1.5h | T2.7.3 |

### 3.2 详细实现

#### 任务 T2.4.1: 看板 API 请求

```rust
// src/kanban/api/board_requests.rs

use super::*;

/// 看板操作请求
#[derive(Debug, Clone)]
pub enum BoardRequest {
    /// 创建看板
    CreateBoard {
        name: String,
        description: Option<String>,
        background_color: Option<String>,
        invite: Vec<String>,
    },
    
    /// 获取看板列表
    GetBoards {
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

/// 看板更新请求
#[derive(Debug, Clone, Default)]
pub struct BoardUpdateRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub background_color: Option<String>,
    pub background_image: Option<Option<String>>,
}

/// 看板响应
#[derive(Debug, Clone)]
pub enum BoardResponse {
    Board(KanbanBoard),
    Boards(Vec<KanbanBoard>),
    Success,
    Error(String),
}
```

#### 任务 T2.5.1: KanbanWorkspace 组件

```rust
// src/kanban/ui/workspace/kanban_workspace.rs

live_design! {
    kanban_workspace = {{KanbanWorkspace}} {
        flow: Right,
        width: Fill,
        height: Fill,
        
        sidebar = {
            width: 272,
            height: Fill,
            background_color: #FFFFFF,
            border_right: 1, #DFE1E6,
        }
        
        main = {
            flow: Down,
            width: Fill,
            height: Fill,
            background_color: #F4F5F7,
        }
        
        header = {
            height: 48,
            background_color: #FFFFFF,
            border_bottom: 1, #DFE1E6,
        }
        
        toolbar = {
            height: 40,
            background_color: #F4F5F7,
        }
        
        content = {
            flow: Right,
            width: Fill,
            height: Fill,
            scroll: {x: true, y: false},
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct KanbanWorkspace {
    #[live]
    sidebar: BoardsSidebar,
    
    #[live]
    main: FlowBox,
    
    #[live]
    header: BoardHeader,
    
    #[live]
    toolbar: BoardToolbar,
    
    #[live]
    content: ScrollContainer,
    
    /// 当前看板
    #[live]
    current_board: Option<KanbanBoard>,
    
    /// 看板列表
    #[live]
    boards: Vec<KanbanBoard>,
    
    /// 侧边栏可见性
    #[live]
    sidebar_visible: bool,
    
    /// 回调
    on_board_select: Option<Box<dyn FnMut(&RoomId)>>,
    on_create_board: Option<Box<dyn FnMut()>>,
}

impl KanbanWorkspace {
    pub fn new() -> Self {
        Self {
            sidebar: BoardsSidebar::new(),
            main: FlowBox::new(),
            header: BoardHeader::new(),
            toolbar: BoardToolbar::new(),
            content: ScrollContainer::new(),
            current_board: None,
            boards: Vec::new(),
            sidebar_visible: true,
            on_board_select: None,
            on_create_board: None,
        }
    }
    
    pub fn set_boards(&mut self, boards: Vec<KanbanBoard>) {
        self.boards = boards;
        self.sidebar.set_boards(boards.clone());
    }
    
    pub fn set_current_board(&mut self, board: &KanbanBoard) {
        self.current_board = Some(board.clone());
        self.header.set_title(&board.name);
        self.content.set_children(vec![]); // 清空内容
        
        // 添加列表容器
        let lists_container = self.create_lists_container();
        self.content.set_children(vec![lists_container]);
    }
    
    fn create_lists_container(&self) -> FlowBox {
        FlowBox {
            flow: Right,
            width: Fill,
            height: Fill,
            spacing: 12,
            padding: 12,
            ..Default::default()
        }
    }
    
    pub fn set_on_board_select<F>(&mut self, callback: F)
    where
        F: FnMut(&RoomId) + 'static,
    {
        self.on_board_select = Some(Box::new(callback));
        self.sidebar.set_on_board_select(callback);
    }
    
    pub fn set_on_create_board<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_create_board = Some(Box::new(callback));
        self.sidebar.set_on_create_board(callback);
    }
}
```

### 3.3 验收标准

- [ ] 可以创建新看板
- [ ] 可以查看看板列表
- [ ] 可以选择并查看看板详情
- [ ] 可以修改看板名称/背景
- [ ] 可以管理看板成员
- [ ] 筛选和搜索功能正常
- [ ] 与 Matrix SDK 集成正常

### 3.4 输出物

| 类型 | 文件 | 说明 |
|------|------|------|
| 代码 | `src/kanban/api/board_requests.rs` | 看板 API |
| 代码 | `src/kanban/ui/workspace/kanban_workspace.rs` | 工作区 |
| 代码 | `src/kanban/ui/components/boards_sidebar.rs` | 侧边栏 |
| 代码 | `src/kanban/ui/components/board_header.rs` | 头部 |
| 测试 | `tests/board_crud_test.rs` | 看板测试 |

---

## 4. 阶段三：卡片管理

**时间**: 第 8-13 天  
**目标**: 实现列表和卡片的完整管理功能

### 4.1 任务清单

#### Day 8-9: 列表管理

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T3.8.1 | 创建列表 API 请求 | 2h | T2.4.1 |
| T3.8.2 | 实现 ListRepository | 2h | T3.8.1 |
| T3.8.3 | 创建 KanbanList 组件 | 3h | - |
| T3.8.4 | 实现列表 CRUD UI | 2h | T3.8.3 |
| T3.8.5 | 实现列表拖拽 | 2h | T3.8.3 |

#### Day 10-11: 卡片基础

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T3.10.1 | 创建卡片 API 请求 | 2h | T2.4.1 |
| T3.10.2 | 实现 CardRepository | 2h | T3.10.1 |
| T3.10.3 | 创建 KanbanCard 组件 | 3h | - |
| T3.10.4 | 实现卡片标题/描述 | 2h | T3.10.3 |
| T3.10.5 | 实现标签显示 | 1.5h | T3.10.3 |

#### Day 12-13: 卡片详情

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T3.12.1 | 创建 CardModal 组件 | 3h | T3.10.3 |
| T3.12.2 | 实现描述编辑器 | 2h | T3.12.1 |
| T3.12.3 | 实现成员选择 | 2h | T3.12.1 |
| T3.12.4 | 实现标签编辑 | 1.5h | T3.12.1 |
| T3.12.5 | 实现截止日期 | 1.5h | T3.12.1 |
| T3.12.6 | 实现附件功能 | 2h | T3.12.1 |
| T3.12.7 | 实现检查清单 | 2h | T3.12.1 |
| T3.12.8 | 实现评论功能 | 1.5h | T3.12.1 |

### 4.2 详细实现

#### 任务 T3.10.3: KanbanCard 组件

```rust
// src/kanban/ui/components/kanban_card.rs

live_design! {
    kanban_card = {{KanbanCard}} {
        flow: Down,
        width: Fill,
        min_height: 40,
        background_color: #FFFFFF,
        border_radius: 3,
        box_shadow: {
            color: #091E420F,
            x: 0,
            y: 1,
            blur: 2,
            spread: 0,
        },
        cursor: Cursor::Pointer,
        
        cover = {
            width: Fill,
            height: 0,
            visible: false,
        }
        
        content = {
            flow: Down,
            width: Fill,
            padding: 8,
            spacing: 4,
        }
        
        labels_row = {
            flow: Right,
            height: 0,
            visible: false,
            spacing: 4,
        }
        
        title = {
            draw_text: {
                text_style: {
                    font_size: 14,
                },
                color: #172B4D,
            }
            wrap: Word,
        }
        
        description_preview = {
            draw_text: {
                text_style: {
                    font_size: 12,
                },
                color: #5E6C84,
            }
            wrap: Word,
            visible: false,
        }
        
        badges_row = {
            flow: Right,
            height: 0,
            visible: false,
            spacing: 4,
        }
        
        footer = {
            flow: Right,
            height: 24,
            align: {x: 1.0, y: 0.5},
            spacing: 4,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct KanbanCard {
    #[live]
    cover: ImageBox,
    
    #[live]
    content: FlowBox,
    
    #[live]
    labels_row: FlowBox,
    
    #[live]
    title: Label,
    
    #[live]
    description_preview: Label,
    
    #[live]
    badges_row: FlowBox,
    
    #[live]
    footer: FlowBox,
    
    /// 卡片数据
    card: Option<KanbanCardData>,
    
    /// 点击回调
    on_click: Option<Box<dyn FnMut()>>,
    
    /// 拖拽回调
    on_drag_start: Option<Box<dyn FnMut()>>,
}

impl KanbanCard {
    pub fn new() -> Self {
        Self {
            cover: ImageBox::empty(),
            content: FlowBox::new(),
            labels_row: FlowBox::new(),
            title: Label::with_text("卡片标题"),
            description_preview: Label::empty(),
            badges_row: FlowBox::new(),
            footer: FlowBox::new(),
            card: None,
            on_click: None,
            on_drag_start: None,
        }
    }
    
    pub fn set_card(&mut self, card: &KanbanCardData) {
        self.card = Some(card.clone());
        
        self.title.set_text(&card.title);
        
        // 设置封面
        if let Some(cover) = &card.cover {
            self.cover.set_image_url(&cover.url);
            self.cover.set_height(cover_height(cover));
            self.cover.set_visible(true);
        }
        
        // 设置标签
        if !card.label_ids.is_empty() {
            self.render_labels(&card.label_ids);
        }
        
        // 设置描述预览
        if let Some(desc) = &card.description {
            if !desc.is_empty() {
                self.description_preview.set_text(desc);
                self.description_preview.set_visible(true);
            }
        }
        
        // 设置徽章
        self.render_badges(card);
        
        // 设置底部
        self.render_footer(card);
    }
    
    fn render_labels(&mut self, label_ids: &[String]) {
        self.labels_row = FlowBox {
            flow: Right,
            height: Fit,
            spacing: 4,
            ..Default::default()
        };
        
        for label_id in label_ids {
            let label = self.create_label(label_id);
            self.labels_row.add_child(label);
        }
        
        self.labels_row.set_visible(true);
    }
    
    fn create_label(&self, label_id: &str) -> FlowBox {
        // 根据标签 ID 查找颜色
        let color = self.get_label_color(label_id);
        
        FlowBox {
            width: 40,
            height: 8,
            background_color: color,
            border_radius: 2,
            ..Default::default()
        }
    }
    
    fn get_label_color(&self, label_id: &str) -> Color {
        // 从卡片数据中获取标签颜色
        self.card
            .as_ref()
            .and_then(|c| c.labels.iter().find(|l| l.id == label_id))
            .map(|l| l.color.to_color())
            .unwrap_or_else(|| color!("#0079BF"))
    }
    
    fn render_badges(&mut self, card: &KanbanCardData) {
        self.badges_row = FlowBox {
            flow: Right,
            height: Fit,
            spacing: 4,
            ..Default::default()
        };
        
        // 截止日期徽章
        if let Some(due_date) = &card.due_date {
            let badge = self.create_due_date_badge(due_date);
            self.badges_row.add_child(badge);
        }
        
        // 附件徽章
        if card.attachment_count > 0 {
            let badge = self.create_attachment_badge(card.attachment_count);
            self.badges_row.add_child(badge);
        }
        
        // 评论徽章
        if card.comment_count > 0 {
            let badge = self.create_comment_badge(card.comment_count);
            self.badges_row.add_child(badge);
        }
        
        self.badges_row.set_visible(true);
    }
    
    fn create_due_date_badge(&self, due_date: &CardDueDate) -> FlowBox {
        let (icon, color) = if due_date.is_completed {
            ("✓", color!("#61BD4F"))
        } else if is_overdue(due_date) {
            ("-clock", color!("#EB5A46"))
        } else {
            ("clock", color!("#FF9F1A"))
        };
        
        FlowBox {
            width: Fit,
            height: 20,
            flow: Right,
            spacing: 4,
            background_color: color.with_alpha(0.1),
            border_radius: 3,
            padding: 4,
            ..Default::default()
        }
    }
    
    fn render_footer(&mut self, card: &KanbanCardData) {
        self.footer = FlowBox {
            flow: Right,
            height: 24,
            align: {x: 1.0, y: 0.5},
            spacing: 4,
            ..Default::default()
        };
        
        // 成员头像
        if !card.member_ids.is_empty() {
            let avatars = self.create_member_avatars(&card.member_ids);
            self.footer.add_child(avatars);
        }
        
        // 加星标
        if card.is_starred {
            let star = self.create_star_icon();
            self.footer.add_child(star);
        }
    }
    
    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_click = Some(Box::new(callback));
    }
    
    pub fn set_on_drag_start<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_drag_start = Some(Box::new(callback));
    }
}
```

### 4.3 验收标准

- [ ] 可以创建/编辑/删除列表
- [ ] 可以拖拽调整列表顺序
- [ ] 可以创建/编辑/删除卡片
- [ ] 可以设置卡片标题、描述
- [ ] 可以添加/编辑/删除标签
- [ ] 可以添加/移除成员
- [ ] 可以设置截止日期
- [ ] 可以添加附件
- [ ] 可以使用检查清单
- [ ] 可以添加/编辑/删除评论
- [ ] 可以打开卡片详情弹窗

### 4.4 输出物

| 类型 | 文件 | 说明 |
|------|------|------|
| 代码 | `src/kanban/ui/components/kanban_list.rs` | 列表组件 |
| 代码 | `src/kanban/ui/components/kanban_card.rs` | 卡片组件 |
| 代码 | `src/kanban/ui/modal/card_modal.rs` | 卡片弹窗 |
| 代码 | `src/kanban/ui/modal/card_description.rs` | 描述编辑器 |
| 测试 | `tests/card_crud_test.rs` | 卡片测试 |

---

## 5. 阶段四：交互增强

**时间**: 第 14-18 天  
**目标**: 实现完整的拖拽排序和高级交互

### 5.1 任务清单

#### Day 14-15: 卡片拖拽

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T4.14.1 | 实现 OrderManager | 2h | - |
| T4.14.2 | 实现 DragDropState | 1.5h | - |
| T4.14.3 | 实现 DragHandler | 2h | T4.14.2 |
| T4.14.4 | 实现 DropZoneDetector | 1.5h | T4.14.1 |
| T4.14.5 | 实现卡片拖拽 UI | 2h | T4.14.3 |

#### Day 16: 拖拽预览与动画

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T4.16.1 | 实现 DragPreview | 1.5h | - |
| T4.16.2 | 实现 DropIndicator | 1.5h | T4.16.1 |
| T4.16.3 | 实现拖拽动画效果 | 2h | T4.16.2 |
| T4.16.4 | 实现触摸设备支持 | 1.5h | T4.16.3 |

#### Day 17-18: 批量操作与快捷键

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T4.17.1 | 实现批量移动 | 2h | T4.15.1 |
| T4.17.2 | 实现多选功能 | 2h | T4.17.1 |
| T4.17.3 | 实现批量操作菜单 | 1.5h | T4.17.2 |
| T4.17.4 | 实现快捷键 | 2h | - |
| T4.17.5 | 实现右键菜单 | 1.5h | T4.17.4 |

### 5.2 详细实现

#### 任务 T4.14.1: OrderManager

```rust
// src/kanban/drag_drop/order_manager.rs

use std::cmp::Ordering;

/// 排序管理器
#[derive(Debug, Default)]
pub struct SimpleOrderManager;

impl SimpleOrderManager {
    /// 初始排序值
    const INITIAL_ORDER: f64 = 1000.0;
    
    /// 排序间隔
    const ORDER_INTERVAL: f64 = 1000.0;
    
    /// 计算新位置
    pub fn calculate_new_position(
        &self,
        before_order: Option<f64>,
        after_order: Option<f64>,
    ) -> f64 {
        match (before_order, after_order) {
            (None, Some(after)) => {
                if after > Self::ORDER_INTERVAL {
                    after - Self::ORDER_INTERVAL
                } else {
                    self.reorder_and_insert(None, Some(after))
                }
            }
            (Some(before), None) => {
                before + Self::ORDER_INTERVAL
            }
            (Some(before), Some(after)) => {
                let middle = (before + after) / 2.0;
                if middle != before && middle != after {
                    middle
                } else {
                    self.reorder_and_insert(Some(before), Some(after))
                }
            }
            (None, None) => Self::INITIAL_ORDER,
        }
    }
    
    fn reorder_and_insert(
        &self,
        before_order: Option<f64>,
        after_order: Option<f64>,
    ) -> f64 {
        match (before_order, after_order) {
            (Some(before), Some(after)) => (before + after) / 2.0,
            (Some(before), None) => before + Self::ORDER_INTERVAL,
            (None, Some(after)) => after - Self::ORDER_INTERVAL,
            (None, None) => Self::INITIAL_ORDER,
        }
    }
    
    /// 批量重新排序
    pub fn reorder_all(&self, orders: &mut [f64]) {
        orders.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        for (i, order) in orders.iter_mut().enumerate() {
            *order = Self::INITIAL_ORDER + i as f64 * Self::ORDER_INTERVAL;
        }
    }
}

/// 可排序 trait
pub trait Sortable {
    fn id(&self) -> &str;
    fn order(&self) -> f64;
    fn set_order(&mut self, order: f64);
}
```

### 5.3 验收标准

- [ ] 可以在列表内拖拽卡片
- [ ] 可以跨列表拖拽卡片
- [ ] 拖拽时显示预览
- [ ] 拖拽时显示放置指示器
- [ ] 拖拽结束后卡片正确移动
- [ ] 支持触摸设备拖拽
- [ ] 可以批量选择和移动卡片
- [ ] 快捷键功能正常
- [ ] 右键菜单功能正常

### 5.4 输出物

| 类型 | 文件 | 说明 |
|------|------|------|
| 代码 | `src/kanban/drag_drop/order_manager.rs` | 排序算法 |
| 代码 | `src/kanban/drag_drop/drag_handler.rs` | 拖拽处理 |
| 代码 | `src/kanban/drag_drop/drop_zone.rs` | 放置区域 |
| 代码 | `src/kanban/drag_drop/drag_preview.rs` | 拖拽预览 |
| 测试 | `tests/drag_drop_test.rs` | 拖拽测试 |

---

## 6. 阶段五：协作同步

**时间**: 第 19-22 天  
**目标**: 实现实时同步和协作功能

### 6.1 任务清单

#### Day 19-20: 实时同步

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T5.19.1 | 实现实时事件订阅 | 2h | - |
| T5.19.2 | 实现状态同步逻辑 | 2h | T5.19.1 |
| T5.19.3 | 实现冲突处理 | 1.5h | T5.19.2 |
| T5.19.4 | 实现乐观更新 | 1.5h | T5.19.3 |

#### Day 21: 协作功能

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T5.21.1 | 实现 @提及功能 | 2h | - |
| T5.21.2 | 实现活动记录 | 1.5h | - |
| T5.21.3 | 实现通知系统 | 1.5h | T5.21.1 |
| T5.21.4 | 实现在线状态 | 1h | - |

#### Day 22: 优化与测试

| 任务 | 描述 | 预估时间 | 依赖 |
|------|------|----------|------|
| T5.22.1 | 性能优化 | 2h | - |
| T5.22.2 | 端到端测试 | 2h | - |
| T5.22.3 | Bug 修复 | 2h | - |

### 6.2 详细实现

#### 任务 T5.19.1: 实时事件订阅

```rust
// src/kanban/sync/realtime_sync.rs

use tokio::sync::broadcast;

/// 实时同步管理器
pub struct RealtimeSyncManager {
    /// 事件发送器
    event_sender: broadcast::Sender<KanbanEvent>,
    
    /// 事件接收器
    event_receiver: broadcast::Receiver<KanbanEvent>,
    
    /// 订阅者集合
    subscribers: Vec<tokio::sync::mpsc::Sender<KanbanEvent>>,
}

impl RealtimeSyncManager {
    pub fn new() -> Self {
        let (sender, receiver) = broadcast::channel(100);
        
        Self {
            event_sender: sender,
            event_receiver: receiver,
            subscribers: Vec::new(),
        }
    }
    
    /// 订阅看板事件
    pub async fn subscribe(
        &mut self,
        board_id: &RoomId,
    ) -> tokio::sync::mpsc::Receiver<KanbanEvent> {
        let (sender, receiver) = tokio::sync::mpsc::channel(50);
        self.subscribers.push(sender);
        receiver
    }
    
    /// 发布事件
    pub fn publish(&self, event: KanbanEvent) {
        let _ = self.event_sender.send(event);
    }
    
    /// 开始监听 Matrix 事件
    pub async fn start_listening(
        &self,
        client: &Client,
        board_id: &RoomId,
    ) -> Result<(), matrix_sdk::Error> {
        let room = client.get_room(board_id)
            .ok_or_else(|| matrix_sdk::Error::RoomNotFound(board_id.to_string()))?;
        
        // 监听时间线事件
        let mut timeline_stream = room.timeline().subscribe().await;
        
        tokio::spawn(async move {
            while let Some(event) = timeline_stream.next().await {
                match event {
                    TimelineEvent::Message(msg) => {
                        // 处理消息事件
                        if let Some(card_event) = msg.as_card_event() {
                            // 发布卡片更新事件
                        }
                    }
                    TimelineEvent::State(state) => {
                        // 处理状态事件
                        if state.is_list_update() {
                            // 发布列表更新事件
                        }
                    }
                    _ => {}
                }
            }
        });
        
        Ok(())
    }
}

/// 看板事件
#[derive(Debug, Clone)]
pub enum KanbanEvent {
    CardCreated { board_id: RoomId, card: KanbanCard },
    CardUpdated { board_id: RoomId, card: KanbanCard },
    CardMoved { board_id: RoomId, card_id: String, from: String, to: String },
    CardDeleted { board_id: RoomId, card_id: String },
    ListCreated { board_id: RoomId, list: KanbanList },
    ListUpdated { board_id: RoomId, list: KanbanList },
    ListMoved { board_id: RoomId, list_id: String, position: f64 },
    ListDeleted { board_id: RoomId, list_id: String },
    MemberJoined { board_id: RoomId, user_id: UserId },
    MemberLeft { board_id: RoomId, user_id: UserId },
}
```

### 6.3 验收标准

- [ ] 其他用户操作实时可见
- [ ] 乐观更新正确应用
- [ ] 冲突时正确处理
- [ ] @提及功能正常
- [ ] 活动记录正确显示
- [ ] 通知正确发送
- [ ] 在线状态正确显示

### 6.4 输出物

| 类型 | 文件 | 说明 |
|------|------|------|
| 代码 | `src/kanban/sync/realtime_sync.rs` | 实时同步 |
| 代码 | `src/kanban/sync/optimistic_update.rs` | 乐观更新 |
| 测试 | `tests/realtime_sync_test.rs` | 同步测试 |

---

## 7. 测试策略

### 7.1 测试金字塔

```
                    /\
                   /  \
                  /    \
                 /  E2E \      端到端测试 (10%)
                /________\
               /          \
              /  Integration \  集成测试 (30%)
             /________________\
            /                    \
           /      Unit Tests     \  单元测试 (60%)
          /________________________\
```

### 7.2 测试覆盖范围

| 类型 | 覆盖内容 | 工具 |
|------|----------|------|
| 单元测试 | 数据模型、排序算法 | cargo test |
| 集成测试 | API 调用、状态管理 | cargo test |
| UI 测试 | 组件渲染、交互 | Makepad 测试框架 |
| E2E 测试 | 完整用户流程 | 手动测试 |

### 7.3 关键测试用例

#### 数据模型测试

```rust
// tests/kanban_model_test.rs

#[test]
fn test_board_creation() {
    let board = KanbanBoard::new("测试看板");
    
    assert_eq!(board.name, "测试看板");
    assert!(board.lists.is_empty());
    assert!(!board.is_archived);
}

#[test]
fn test_card_ordering() {
    let mut cards = vec![
        KanbanCard { order_index: 1000.0, ..Default::default() },
        KanbanCard { order_index: 3000.0, ..Default::default() },
        KanbanCard { order_index: 2000.0, ..Default::default() },
    ];
    
    let manager = SimpleOrderManager;
    let orders: Vec<f64> = cards.iter().map(|c| c.order_index).collect();
    manager.reorder_all(&mut orders.clone());
    
    assert_eq!(orders, vec![1000.0, 2000.0, 3000.0]);
}
```

#### API 测试

```rust
// tests/kanban_api_test.rs

#[tokio::test]
async fn test_create_board() {
    let client = create_test_client().await;
    let repository = BoardRepository::new(client);
    
    let board = repository.create_board(
        "测试看板",
        Some("描述"),
        None,
        Vec::new(),
    ).await;
    
    assert!(board.is_ok());
    assert_eq!(board.unwrap().name, "测试看板");
}
```

---

## 8. 风险缓解

### 8.1 技术风险

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|----------|
| Matrix SDK 变更 | 高 | 低 | 使用稳定的 API 版本 |
| 性能问题 | 中 | 中 | 虚拟化 + 增量更新 |
| 拖拽冲突 | 中 | 中 | 乐观更新 + 冲突解决 |
| 内存泄漏 | 中 | 低 | 使用 Rc<RefCell<>> 管理 |

### 8.2 进度风险

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|----------|
| 需求变更 | 中 | 中 | 敏捷开发，快速迭代 |
| 资源不足 | 高 | 低 | 预留 20% 缓冲时间 |
| 技术难点 | 中 | 中 | 提前调研，预估风险 |

### 8.3 风险监控

```markdown
## 每日站会检查清单

- [ ] 今日任务是否完成？
- [ ] 是否有阻塞问题？
- [ ] 风险是否需要升级？
- [ ] 明日计划是否清晰？

## 每周风险评估

- [ ] 更新风险矩阵
- [ ] 评估进度偏差
- [ ] 调整资源分配
- [ ] 必要时调整范围
```

---

## 9. 里程碑与时间线

### 9.1 总体时间线

```
Week 1          Week 2          Week 3          Week 4
  │               │               │               │
  ▼               ▼               ▼               ▼
┌───────────────────────────────────────────────────────┐
│  阶段一        │  阶段二        │  阶段三        │  阶段四       │
│  基础框架      │  看板管理      │  卡片管理      │  交互增强     │
│  (3天)         │  (4天)         │  (6天)         │  (5天)        │
├───────────────────────────────────────────────────────┤
│                                                       │
│  阶段五        │                                       │
│  协作同步      │                                       │
│  (4天)         │                                       │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 9.2 关键里程碑

| 里程碑 | 日期 | 交付物 |
|--------|------|--------|
| M1: 基础框架完成 | 第 3 天 | 数据模型、状态管理 |
| M2: 看板功能完成 | 第 7 天 | 看板 CRUD UI |
| M3: 卡片功能完成 | 第 13 天 | 卡片 CRUD UI |
| M4: 拖拽功能完成 | 第 18 天 | 完整拖拽系统 |
| M5: 协作功能完成 | 第 22 天 | 实时同步 |
| M6: 发布候选 | 第 24 天 | 完整应用 |

### 9.3 资源分配

| 阶段 | 开发 | 测试 | 设计 |
|------|------|------|------|
| 阶段一 | 3 人 | 0 人 | 0 人 |
| 阶段二 | 3 人 | 0.5 人 | 0.5 人 |
| 阶段三 | 3 人 | 1 人 | 0.5 人 |
| 阶段四 | 2 人 | 1 人 | 0 人 |
| 阶段五 | 2 人 | 1 人 | 0 人 |

### 9.4 质量门禁

| 检查点 | 标准 | 责任人 |
|--------|------|--------|
| 代码审查 | 所有 PR 必须经过审查 | Tech Lead |
| 测试覆盖 | > 80% 单元测试覆盖 | QA |
| 性能测试 | 拖拽帧率 > 50fps | QA |
| 安全审查 | 无高危漏洞 | Security |
| 用户验收 | 关键路径测试通过 | PM |

---

## 附录

### A. 依赖关系图

```
阶段一 ──┬──> 阶段二 ──┬──> 阶段三 ──┬──> 阶段四 ──┬──> 阶段五
         │            │            │            │
         │            │            │            │
         ▼            ▼            ▼            ▼
    T1.1.1       T2.4.1       T3.8.1       T4.14.1      T5.19.1
    T1.2.1       T2.5.1       T3.10.1      T4.16.1      T5.21.1
    T1.3.1       T2.6.1       T3.12.1      T4.17.1      T5.22.1
```

### B. 沟通机制

| 会议 | 频率 | 参与者 | 内容 |
|------|------|--------|------|
| 每日站会 | 每日 | 全体 | 进度、阻塞 |
| 周会 | 每周 | 全体 | 风险、计划 |
| 代码审查 | 按需 | 开发团队 | 代码质量 |
| 演示会 | 每周 | 全体 | 成果展示 |

### C. 文档清单

| 文档 | 阶段 | 状态 |
|------|------|------|
| 数据模型设计 | 阶段一 | ✅ 完成 |
| API 映射文档 | 阶段一 | ✅ 完成 |
| UI 组件设计 | 阶段一 | ✅ 完成 |
| 拖拽实现文档 | 阶段一 | ✅ 完成 |
| 测试计划 | 阶段三 | 待编写 |
| 用户手册 | 阶段五 | 待编写 |
| 部署指南 | 阶段五 | 待编写 |

---

> 文档版本: 1.0
> 最后更新: 2026-01-14
