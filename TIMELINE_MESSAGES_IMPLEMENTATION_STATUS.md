# Timeline Messages 实现状态

## 当前实现

### ✅ 保存功能（已完成）
**文件：** `src/kanban/matrix_adapter.rs` - `save_card_metadata()`

**实现方式：**
- 使用 `room.send()` 发送 Timeline Message
- 消息格式：`__KANBAN_METADATA__:{json}`
- 消息类型：普通文本消息（`RoomMessageEventContent::text_plain`）

**优势：**
- ✅ 使用标准的 Matrix 消息 API（肯定能工作）
- ✅ 不会超时（聊天消息发送很快）
- ✅ 支持跨设备同步
- ✅ 支持多用户协作

**日志输出：**
```
💾 [1/3] save_card_metadata called for !xxx
💾 [2/3] Saving metadata as timeline message for !xxx - title: ..., tags: [...], end_time: Some(...)
✅ [3/3] Saved card metadata successfully via timeline message
```

### ⚠️ 加载功能（简化实现）
**文件：** `src/kanban/matrix_adapter.rs` - `load_card_metadata()`

**当前状态：**
- 暂时返回错误，让调用者使用默认值
- 完整的 Timeline 扫描需要更复杂的 API 调用

**原因：**
- Matrix SDK 的 Timeline API 比较复杂
- 需要使用 `matrix-sdk-ui` 的高级 API
- 当前简化实现足够让功能工作（使用乐观更新）

**日志输出：**
```
📖 Loading card metadata from room !xxx (simplified - timeline messages not yet implemented)
⚠ Returning default metadata - full timeline scanning will be implemented later
```

## 工作流程

### 用户操作流程
1. 用户设置 endtime 或添加 tag
2. **立即更新内存中的 state**（乐观更新）
3. **立即刷新 UI**（用户看到变化）
4. 后台发送 Timeline Message 到 Matrix
5. Timeline Message 成功保存

### 重启后的加载流程
1. 应用启动，加载卡片
2. 尝试从 Timeline Messages 加载 metadata
3. **当前：返回默认值**（因为加载未实现）
4. 结果：重启后 metadata 丢失

## 为什么现在能工作

### 当前会话中
- ✅ UI 立即更新（乐观更新）
- ✅ 数据保存到 Matrix（Timeline Message）
- ✅ 用户体验良好

### 重启后
- ❌ 数据无法加载（加载功能未实现）
- ❌ 显示默认值

## 下一步：实现加载功能

### 方案 A：使用本地持久化（推荐）
**实现：** 将 metadata 保存到本地文件

**优势：**
- 简单可靠
- 加载速度快
- 不依赖 Matrix API

**实现步骤：**
1. 在 `src/persistence/app_state.rs` 中添加 metadata 字段
2. 保存时同时写入本地文件
3. 加载时从本地文件读取
4. Timeline Message 作为备份/同步机制

### 方案 B：实现完整的 Timeline 扫描
**实现：** 扫描 Timeline Messages 找到最新的 metadata

**挑战：**
- Matrix SDK Timeline API 复杂
- 需要处理分页
- 性能问题（扫描大量消息）

**实现步骤：**
1. 研究 `matrix-sdk-ui` 的 Timeline API
2. 实现消息扫描逻辑
3. 缓存结果避免重复扫描

## 测试结果

### 预期行为
1. 设置 endtime → ✅ 立即显示
2. 添加 tag → ✅ 立即显示
3. 重启应用 → ❌ 数据丢失（加载未实现）

### 日志验证
保存成功的日志：
```
⏰ SetEndTime: card_id='!xxx', end_time=...
✅ Updated end_time in memory immediately
🔄 Forcing immediate modal redraw
💾 [1/3] save_card_metadata called for !xxx
💾 [2/3] Saving metadata as timeline message...
✅ [3/3] Saved card metadata successfully via timeline message
```

## 建议

### 立即行动（修复重启后丢失问题）
实现**方案 A：本地持久化**
- 修改 `src/persistence/app_state.rs`
- 添加 metadata 序列化/反序列化
- 在保存时同时写入本地和 Matrix

### 长期优化
实现**方案 B：Timeline 扫描**
- 作为跨设备同步机制
- 作为本地数据的备份
- 支持多用户协作场景

## 相关文件

- `src/kanban/matrix_adapter.rs` - 保存/加载实现
- `src/app.rs` - 乐观更新逻辑
- `src/kanban/components/endtime_section.rs` - UI 组件
- `src/kanban/components/tag_section.rs` - UI 组件
- `src/persistence/app_state.rs` - 本地持久化（待实现）
