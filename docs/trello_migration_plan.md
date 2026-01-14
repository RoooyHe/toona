# Robrix → Trello 风格协同软件改造文档（Matrix 兼容）

本文档在不改变后端接口、不新增后端接口的前提下，将现有 Robrix（Matrix 客户端）改造成 Trello 风格协同软件。所有协同数据严格落在 Matrix 协议的现有能力中（房间、状态事件、消息事件、关系事件、权限等级、同步）。

## 约束与原则

- 不新增或改动后端接口，仅使用 Matrix 现有 API。
- 协同数据必须存储在 Matrix 房间事件内。
- 客户端承担数据结构定义、渲染和交互逻辑。
- 保持与现有架构一致的模块划分和 Makepad UI 风格。

## 改造合理性（Why）

- Matrix 的“房间 + 状态事件 + 时间线”天然支持协同数据结构。
- 房间状态事件适合保存看板结构（列、顺序、配置）。
- 消息事件适合保存卡片内容与变更记录。
- 关系事件可作为评论、附件、子任务等关联内容。
- 权限系统（power levels）可直接映射协作角色。
- 既有滑动同步与本地缓存可支持离线与实时协作。

## 目标形态（What）

### MVP（可演示）

- 工作区：使用 Matrix Space 表示协作空间。
- 看板：一个 Matrix Room 表示看板。
- 列：看板状态事件中保存列列表与顺序。
- 卡片：卡片以消息事件表示，并绑定列 ID。
- 基础交互：创建看板、创建列、添加卡片、卡片跨列移动。
- 权限：读取房间权限等级控制编辑能力。

### V1（团队可用）

- 卡片详情：描述、标签、截止日期、负责人。
- 评论：使用关系事件（reply）挂在卡片事件下。
- 活动记录：时间线回溯展示变更历史。
- 看板同步：列表/卡片排序更新可持久化。

### 可上线版本

- 角色系统：admin/editor/viewer 基于 power levels。
- 性能优化：长列表虚拟化、增量渲染。
- 离线与容错：使用本地持久化 + 滑动同步。
- UX 完整性：空状态、错误提示、重连提示。

### Trello 全量能力（后续）

- 清单、子任务、附件、模板、自动化规则。
- 多视图（日历、表格、时间线）。
- 高级搜索与筛选（标签、负责人、状态）。

## 数据映射（Matrix → Trello 语义）

- Workspace → Matrix Space
- Board → Matrix Room
- List/Column → Room State Event 里的 list entry
- Card → Room Message Event
- Comment → Relation (reply) to Card Event
- Attachment → Media message linked to Card Event
- Member/Role → Room membership + power levels

## 事件结构设计（Data Schema）

### 看板状态事件（示例）

```json
{
  "type": "com.robrix.board",
  "state_key": "main",
  "content": {
    "schema_version": 1,
    "lists": [
      { "id": "list_todo", "name": "Todo", "order": 0 },
      { "id": "list_doing", "name": "Doing", "order": 1 }
    ],
    "card_order": {
      "list_todo": ["card_001", "card_002"],
      "list_doing": ["card_003"]
    }
  }
}
```

### 卡片消息事件（示例）

```json
{
  "type": "m.room.message",
  "content": {
    "msgtype": "com.robrix.card",
    "body": "Implement login flow",
    "card_id": "card_001",
    "list_id": "list_todo",
    "description": "Support SSO login",
    "labels": ["auth"],
    "assignees": ["@alice:server"],
    "due": "2026-01-31"
  }
}
```

### 评论事件（示例）

```json
{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.text",
    "body": "I will pick this up today",
    "m.relates_to": {
      "rel_type": "m.in_reply_to",
      "event_id": "$card_event_id"
    }
  }
}
```

## 转换流程（How to Transform）

### 1. 识别 UI 模块

- 新增看板模块：例如 `src/board/`，遵循现有 UI 模块结构。
- 复用 `shared/` 组件，用于卡片/列表基础样式。

### 2. 定义数据类型

- 在 `src/` 增加 Board/List/Card 数据结构。
- 数据结构仅作为 UI 与 Matrix 事件之间的转换层，不新增后端。

### 3. 读取与解析事件

- 读取 `com.robrix.board` 状态事件，构建列表结构。
- 遍历房间时间线，筛选 `msgtype = com.robrix.card`。
- 组合 list_id + card_id 渲染 UI。

### 4. 写入事件

- 创建列/重排序：更新状态事件。
- 创建卡片/更新卡片：发送消息事件。
- 评论：发送 reply 消息。

### 5. 同步与冲突处理

- 本地 optimistic UI，最终由 Matrix 同步确认。
- 排序冲突采用 last-write-wins。

## 示例代码（Rust + matrix-sdk）

### 解析看板状态事件

```rust
use matrix_sdk::ruma::events::AnySyncStateEvent;

fn parse_board_state(event: &AnySyncStateEvent) -> Option<BoardState> {
    if event.event_type() != "com.robrix.board" {
        return None;
    }
    let content = event.content().to_owned();
    serde_json::from_value(content).ok()
}
```

### 发送卡片事件

```rust
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use serde_json::json;

async fn send_card(room: &matrix_sdk::Room, card: Card) -> anyhow::Result<()> {
    let content = RoomMessageEventContent::new(json!({
        "msgtype": "com.robrix.card",
        "body": card.title,
        "card_id": card.id,
        "list_id": card.list_id,
        "description": card.description,
        "labels": card.labels,
        "assignees": card.assignees,
        "due": card.due
    }));
    room.send(content, None).await?;
    Ok(())
}
```

### 更新看板状态事件

```rust
use matrix_sdk::ruma::events::room::power_levels::RoomPowerLevelsEventContent;
use serde_json::json;

async fn update_board_state(room: &matrix_sdk::Room, board: BoardState) -> anyhow::Result<()> {
    let content = json!({
        "schema_version": 1,
        "lists": board.lists,
        "card_order": board.card_order
    });
    room.send_state_event("com.robrix.board", "main", content).await?;
    Ok(())
}
```

## 阶段功能与里程碑（When）

### MVP

- 看板结构事件定义完成
- 列/卡片 UI 可展示
- 卡片创建可发送事件
- 基础拖拽排序（可只前端）

### V1

- 卡片详情页/抽屉
- 评论系统（relations）
- 持久化排序
- 角色权限控制

### 可上线版本

- 离线可用 + 重连恢复
- 性能优化（虚拟化）
- 完整错误处理与 UX

### 后续全功能

- 清单、附件、自动化规则
- 多视图（时间线、日历）
- 搜索与过滤

## 风险与对策

- **事件体积过大**：保持 card_order 精简，仅保存 ID。
- **并发排序冲突**：last-write-wins + 提示冲突。
- **权限滥用**：强制校验 power levels。
- **性能压力**：分页渲染 + 本地缓存。

## UI 交互流程（Flow）

### 工作区与看板入口

- 进入应用后展示工作区（Matrix Space）列表。
- 点击工作区进入看板列表页（Space 下的 rooms）。
- 点击看板进入看板主视图。

### 看板主视图

- 顶部：看板标题、成员头像、过滤器入口。
- 主体：横向滚动列表列（List），每列纵向卡片（Card）。
- 右上：新增列表按钮、看板设置入口。

### 列操作流程

- 点击“新增列” → 输入列名 → 写入 `com.robrix.board` 状态事件。
- 拖拽列 → 更新列表顺序 → 写入状态事件。
- 删除列 → 从状态事件中移除列并移除卡片排序。

### 卡片操作流程

- 列底部“新增卡片” → 输入标题 → 发送 `com.robrix.card` 消息事件。
- 拖拽卡片跨列 → 更新 `card_order` 并写入状态事件。
- 点击卡片 → 打开详情抽屉/弹窗。

### 卡片详情交互

- 标题、描述：内联编辑 → 发更新事件或新消息事件。
- 标签：选择/新增 → 更新卡片内容。
- 负责人：选择 Matrix 成员 → 更新卡片内容。
- 截止日期：选择日期 → 更新卡片内容。
- 评论：输入后发送 reply 关系事件。
- 活动记录：按时间线展示卡片相关事件。

### 权限与只读

- 根据 power levels 判断可编辑/只读状态。
- 无权限用户隐藏编辑入口，保留查看。

## 页面原型说明（UI 草图级描述）

### 1. 工作区页（Workspace）

- 左侧：工作区列表（Space）。
- 右侧：所选工作区的看板列表。
- 操作：创建新看板、加入看板。

### 2. 看板页（Board）

- 顶栏：看板名、成员头像、筛选按钮、设置按钮。
- 主区：列表横向滚动，卡片纵向堆叠。
- 底栏：快速添加卡片入口。

### 3. 卡片详情页（Drawer/Modal）

- 左侧主区：标题、描述、清单、附件。
- 右侧侧栏：负责人、标签、日期、操作按钮。
- 底部：评论列表 + 输入框。

### 4. 成员与权限页

- 成员列表展示 Matrix 用户 ID 与角色。
- 支持修改角色（受 power levels 约束）。

### 5. 过滤与搜索

- 顶栏筛选：标签、负责人、截止日期。
- 搜索框：本地索引搜索卡片内容。

## Makepad 组件结构清单（建议）

### 视图层级（顶层到细节）

- `WorkspaceView`
  - `WorkspaceSidebar`
  - `BoardListView`
    - `BoardListItem`
- `BoardView`
  - `BoardTopBar`
    - `BoardTitle`
    - `BoardMembers`
    - `BoardFiltersButton`
    - `BoardSettingsButton`
  - `BoardListsScroller`
    - `BoardListColumn`
      - `ListHeader`
      - `CardList`
        - `CardItem`
      - `ListFooterAddCard`
  - `BoardAddListButton`
- `CardDetailDrawer`
  - `CardDetailHeader`
  - `CardDetailBody`
    - `CardDescriptionEditor`
    - `CardChecklistView`
    - `CardAttachmentList`
  - `CardDetailSidebar`
    - `AssigneePicker`
    - `LabelPicker`
    - `DueDatePicker`
  - `CardCommentList`
    - `CardCommentItem`
  - `CardCommentComposer`

### 推荐组件文件（示例）

- `src/board/mod.rs`
- `src/board/board_view.rs`
- `src/board/board_list_column.rs`
- `src/board/card_item.rs`
- `src/board/card_detail_drawer.rs`
- `src/board/board_top_bar.rs`
- `src/board/board_list_view.rs`
- `src/board/board_models.rs`

### 与现有模块对接点（home/、shared/）

- `home/`
  - 复用主页导航/布局骨架，将 `BoardView` 作为主内容区。
  - `home/` 中的 room/space 列表可映射为工作区与看板列表入口。
- `shared/`
  - 复用按钮、头像、标签、输入框、滚动容器等通用组件。
  - 统一颜色与字体，直接引用 `shared/styles.rs` 中的样式常量。
  - 复用头像加载与缓存逻辑，显示成员与负责人头像。

### Live Design DSL 结构建议

- `BoardView` 使用横向 `View` + `ScrollView`。
- `BoardListColumn` 内部使用 `flow: Down`。
- `CardItem` 使用可点击 `View` + `Label`。
- `CardDetailDrawer` 可用 `Overlay` + `View`。

## 输出物（Deliverables）

- Board/List/Card 数据结构定义
- 新的看板 UI 模块
- Matrix 事件读写逻辑
- 评论与活动记录视图
- MVP → V1 → 上线版本里程碑规划
- UI 交互流程与页面原型说明
- Makepad 组件结构清单
