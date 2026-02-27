# 备选存储方案

## 当前状况
- ✅ Todos 保存成功（使用 `m.kanban.card.todos` State Event）
- ❌ Metadata 保存超时（使用同样的 State Event 但包含 metadata 字段）
- ✅ UI 立即更新（乐观更新）
- ❌ 重启后数据丢失（Matrix 保存失败）

## 方案对比

### 方案 1：继续调试当前方案（Combined State Event）
**原理：** 将 todos 和 metadata 存储在同一个 `m.kanban.card.todos` State Event 中

**优势：**
- 理论上应该能工作（todos 能保存）
- 数据结构简单

**问题：**
- 实际测试中 metadata 保存超时
- 可能是数据大小问题
- 可能是并发问题（同时读写）

**下一步：**
- 添加更详细的日志
- 测试只保存 metadata（不加载 todos）
- 减小数据大小

### 方案 2：分离存储（Separate State Events）
**原理：** Todos 和 metadata 使用不同的 State Event 类型

**实现：**
- Todos: `m.kanban.card.todos`
- Metadata: `m.kanban.card.info`（更短的名字）

**优势：**
- 避免并发冲突
- 数据更小
- 更清晰的分离

**问题：**
- 之前 `m.kanban.card.meta` 也超时了
- 可能服务器不支持多个自定义 State Events

### 方案 3：使用 Room Account Data
**原理：** 使用 Matrix 的 Account Data API 而不是 State Events

**实现：**
```rust
// 保存
client.account().set_account_data(
    format!("m.kanban.card.{}", room_id),
    metadata
).await?;

// 加载
client.account().account_data_raw(
    format!("m.kanban.card.{}", room_id)
).await?;
```

**优势：**
- 不需要 room 权限
- 每个用户独立存储
- 可能更快

**问题：**
- 数据不会跨设备同步（除非使用 room-level account data）
- 不适合多用户协作

### 方案 4：使用 Timeline Messages（推荐）
**原理：** 将 metadata 作为特殊的 Timeline Message 发送

**实现：**
```rust
// 发送特殊消息
room.send(
    RoomMessageEventContent::new(
        MessageType::Text(TextMessageEventContent::plain(
            format!("__METADATA__:{}", serde_json::to_string(&metadata)?)
        ))
    )
).await?;

// 加载时扫描 timeline 找最新的 metadata 消息
```

**优势：**
- ✅ Timeline messages 肯定能工作（聊天功能正常）
- ✅ 有历史记录（可以看到修改历史）
- ✅ 跨设备同步
- ✅ 多用户可见

**问题：**
- 会在 timeline 中显示（需要过滤）
- 加载时需要扫描 timeline
- 占用更多空间

### 方案 5：使用本地持久化 + 定期同步
**原理：** 主要存储在本地，定期尝试同步到 Matrix

**实现：**
- 立即保存到本地文件（`~/.toona/card_metadata.json`）
- 后台定期重试 Matrix 保存
- 启动时从本地加载，然后从 Matrix 更新

**优势：**
- ✅ 可靠（本地文件不会丢失）
- ✅ 快速（不等待网络）
- ✅ 离线可用

**问题：**
- 不能跨设备同步
- 不能多用户协作
- 需要实现同步逻辑

## 推荐方案

### 短期（立即修复）：方案 4 - Timeline Messages
使用 Timeline Messages 存储 metadata，因为：
1. 肯定能工作（聊天功能正常）
2. 实现简单
3. 支持多用户协作

### 长期（优化）：方案 5 - 本地持久化 + Timeline Messages
结合两者优势：
1. 本地文件作为主存储（快速、可靠）
2. Timeline Messages 作为同步机制（跨设备、多用户）
3. 启动时合并两者数据

## 下一步行动

请告诉我：
1. 你更倾向哪个方案？
2. 是否需要跨设备同步？
3. 是否需要多用户协作？

我可以立即实现方案 4（Timeline Messages），大约需要修改 3 个文件。
