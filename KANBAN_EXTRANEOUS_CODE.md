# 看板与聊天室整合方案

## 目标

将 Matrix 聊天室功能整合到看板卡片详情视图中，类似 Trello 的：
- **评论 (Comments)** - 用户在卡片下的实时讨论
- **活动 (Activity)** - 卡片状态变更的历史记录

```
┌─────────────────────────────────────────────────────────┐
│  ┌─────────────────────────────────────────────────┐    │
│  │  📋 卡片标题                        [标签] [日期] │    │
│  ├─────────────────────────────────────────────────┤    │
│  │  描述                                            │    │
│  │  ───────────────────────────────────────────    │    │
│  │  ☑️ 待办清单                                     │    │
│  │    ☑ 完成的任务 1                                │    │
│  │    ☐ 待办任务 2                                  │    │
│  ├─────────────────────────────────────────────────┤    │
│  │  💬 评论 (Matrix 聊天室)                          │    │
│  │  ┌─────────────────────────────────────────┐    │    │
│  │  │ 👤 用户1: 这任务需要设计评审              │    │    │
│  │  │ 👤 用户2: 同意，我来做                    │    │    │
│  │  │ 👤 用户1: 好的，预计周五完成               │    │    │
│  │  └─────────────────────────────────────────┘    │    │
│  │  [输入框: 添加评论...]                    [发送] │    │
│  ├─────────────────────────────────────────────────┤    │
│  │  📜 活动记录                                    │    │
│  │  • 用户1 将状态改为"进行中"   10分钟前          │    │
│  │  • 用户2 添加了标签"紧急"      1小时前           │    │
│  │  • 用户1 创建了卡片             2小时前         │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

---

## 当前架构问题

### 问题 1：两套并行的 UI
```
当前：                    期望：
┌─────────┐             ┌─────────────┐
│ 看板界面 │             │  看板界面    │
└────┬────┘             └──────┬──────┘
     │                          │
     ▼                          ▼
┌─────────┐             ┌─────────────┐
│聊天室界面│   ──────→   │ 卡片详情 Modal│
│(独立Tab) │             │ ├─ 卡片信息   │
└─────────┘             │ ├─ 评论(聊天) │
                        │ └─ 活动记录   │
                        └─────────────┘
```

### 问题 2：room/ 模块独立存在
`src/room/` 是一个完整的聊天室功能模块，但应该只作为看板卡片的"评论"区域

---

## 重构方案

### 1. 保留 Matrix 核心 (必须)

```rust
// src/sliding_sync.rs - 保持不变，Matrix 协议核心
// src/space_service_sync.rs - 保持不变
// src/media_cache.rs - 保持不变
// src/avatar_cache.rs - 保持不变
```

### 2. 将 room/ 模块重构为"评论组件"

```
src/room/                              →  src/kanban/components/card_comments.rs
├── mod.rs                             →  (整合到 card_modal.rs)
├── reply_preview.rs                   →  评论回复预览
├── room_input_bar.rs                  →  评论输入框
├── room_display_filter.rs              →  评论过滤器
├── typing_notice.rs                   →  正在输入提示
└── basic_room_details.rs              →  (删除或整合)
```

### 3. 卡片详情 Modal 重构

```
当前结构：                              目标结构：
card_modal.rs                         card_modal.rs
├─ card_info_section (标题/描述/标签)   ├─ card_info_section
├─ todo_section (待办)                 ├─ todo_section
├─ tag_section (标签)                  ├─ tag_section
├─ endtime_section (截止时间)           ├─ endtime_section
├─ active_section (活动记录)            ├─ activity_section (优化)
└─ (无评论功能)                        └─ card_comments_section ⭐ 新增
```

### 4. 评论功能实现

**方案 A：在卡片 Modal 中嵌入 Timeline**

```rust
// src/kanban/components/card_comments.rs

use matrix_sdk::room::Room;
use matrix_sdk_ui::timeline::{Timeline, TimelineItem};

pub struct CardCommentsSection {
    pub room_id: OwnedRoomId,      // Matrix Room ID 作为评论后端
    pub timeline: Option<Timeline>,
}

impl CardCommentsSection {
    /// 加载卡片的评论 (Matrix Room timeline)
    pub fn load_comments(&mut self, room_id: OwnedRoomId) {
        // 使用 matrix_sdk_ui::Timeline 作为评论后端
    }

    /// 发送评论
    pub fn send_comment(&self, message: String) {
        // 调用 matrix_worker_task 发送消息到 Room
    }
}
```

**方案 B：利用现有 matrix_adapter**

看板卡片已经映射到 Matrix Room，可以直接利用 Room 的 timeline 作为评论：

```rust
// 在 kanban/matrix_adapter.rs 中扩展

impl KanbanCard {
    /// 获取评论 Room 的 Timeline
    pub fn comments_timeline(&self) -> Option<Timeline> {
        // self.id 就是 Matrix Room ID，可以直接获取 timeline
    }
}
```

### 5. 活动记录优化

当前 `active_section.rs` 显示的是看板操作历史 (状态变更、标签变更等)，保留但可以优化展示。

评论和活动是**不同的**：
- **评论** = 用户实时对话 (Matrix Room timeline)
- **活动** = 系统操作日志 (CardActivity 列表)

---

## 文件变更清单

### 需要修改

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/kanban/components/card_modal.rs` | 重构 | 添加评论组件区域 |
| `src/kanban/components/card_comments.rs` | 新增 | 评论 Section 组件 |
| `src/kanban/matrix_adapter.rs` | 扩展 | 添加评论 timeline 获取方法 |
| `src/home/room_screen.rs` | 可选删除 | 如果完全不需要独立聊天室 Tab |
| `src/home/main_desktop_ui.rs` | 重构 | 移除聊天室 Tab (可选) |
| `src/home/navigation_tab_bar.rs` | 重构 | 移除聊天室 Tab (可选) |

### 可以保留但简化的

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/room/` | 保留但重构 | 只保留评论相关功能，其他删除 |
| `src/home/rooms_list*.rs` | 可选 | 如果只需要看板入口 |

### 必须保留

| 文件 | 说明 |
|------|------|
| `src/sliding_sync.rs` | Matrix 协议核心 |
| `src/space_service_sync.rs` | Space 同步 |
| `src/kanban/` | 看板核心功能 |
| `src/persistence/matrix_state.rs` | Matrix 会话持久化 |

---

## 实现步骤

### Phase 1: 整合评论功能到卡片 Modal

1. 在 `card_modal.rs` 中添加 `card_comments_section`
2. 创建 `card_comments.rs` 组件
3. 实现 `load_comments()` - 获取 Matrix Room timeline
4. 实现 `send_comment()` - 发送消息到 Room

### Phase 2: 移除独立聊天室 Tab (可选)

1. 修改 `navigation_tab_bar.rs` - 只保留看板 Tab
2. 修改 `main_desktop_ui.rs` / `main_mobile_ui.rs` - 移除聊天室视图
3. 可选：完全删除 `room_screen.rs`

### Phase 3: 清理代码

1. 删除 `room/` 中不再需要的文件
2. 删除 `home/` 中聊天室相关文件
3. 更新 `src/lib.rs` 模块声明
4. 更新 Cargo.toml 清理依赖 (如有)

---

## 预期结果

```
最终界面：

┌─────────────────────────────────────────────────────────┐
│  🏠 看板    |  ⚙️ 设置                                    │ ← 只保留看板 Tab
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ 📋 待办   │  │ 🔄 进行中 │  │ ✅ 已完成 │               │
│  │ ┌──────┐ │  │ ┌──────┐ │  │ ┌──────┐ │               │
│  │ │卡片1 │ │  │ │卡片3 │ │  │ │卡片5 │ │               │
│  │ └──────┘ │  │ └──────┘ │  │ └──────┘ │               │
│  │ ┌──────┐ │  │          │  │          │               │
│  │ │卡片2 │ │  │          │  │          │               │
│  │ └──────┘ │  │          │  │          │               │
│  └──────────┘  └──────────┘  └──────────┘               │
│                                                         │
└─────────────────────────────────────────────────────────┘

点击卡片后：

┌─────────────────────────────────────────────────────────┐
│  ✕                      卡片详情                   [⋮]  │
├─────────────────────────────────────────────────────────┤
│  📌 任务：完成首页设计评审                               │
│                                                         │
│  描述：                                                 │
│  需要与UI团队确认设计稿，并完成开发评审文档               │
│                                                         │
│  标签：🔴 紧急  🔵 设计                                 │
│  截止：📅 2024-01-15                                   │
│                                                         │
│  ☑️ 待办：                                              │
│    ☑ 与UI确认设计稿                                     │
│    ☐ 完成开发评审文档                                    │
│    ☐ 提交PR                                            │
│                                                         │
│  💬 评论 (3条)                              [👀 预览]   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 👤 张三 · 2小时前                                │   │
│  │ 设计稿已更新，请查看                              │   │
│  ├─────────────────────────────────────────────────┤   │
│  │ 👤 李四 · 1小时前                                │   │
│  │ 好的，我这边开始评审                              │   │
│  ├─────────────────────────────────────────────────┤   │
│  │ 👤 王五 · 30分钟前                               │   │
│  │ @李四 评审结果如何？                              │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 添加评论...                                  [➤] │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  📜 活动                                               │
│  • 李四 将状态改为"进行中"            1小时前           │
│  • 张三 添加了标签"紧急"              2小时前           │
│  • 王五 创建了卡片                    3小时前           │
└─────────────────────────────────────────────────────────┘
```

---

## 总结

| 项目 | 说明 |
|------|------|
| **整合方式** | Matrix 聊天室 → 看板卡片的评论功能 |
| **技术实现** | 利用 Matrix Room Timeline 作为评论后端 |
| **主要变更** | `card_modal.rs` 添加评论 section |
| **可选变更** | 移除独立聊天室 Tab，只保留看板视图 |
| **保留模块** | Matrix 核心、看板核心、所有持久化和认证 |
