# 调试Tags和Todos显示问题

## 当前状态

从你提供的metadata可以看到：
- ✅ **EndTime**: 已保存 (1772352600)
- ✅ **Tag "ui"**: 已保存在metadata中
- ❓ **Todos**: 看不到（因为todos用State Event存储，不在metadata中）

## 问题分析

### 问题1: Tag "ui" 已保存但不显示

**可能原因：**
1. UI组件没有正确读取tags数据
2. CardLoaded后没有触发UI重绘
3. TagSection组件有bug

**调试步骤：**

1. **检查日志中的tags数据：**
   查找这些日志：
   ```
   🎨 TagSection draw_walk: card_id=..., tags=[...]
   ```
   看看tags数组是否包含 "ui"

2. **检查CardLoaded日志：**
   ```
   CardLoaded: card_id='...', title='...'
   ```
   之后应该有：
   ```
   🔄 Forcing modal redraw for updated card ...
   ```

3. **检查内存中的卡片数据：**
   在 `CardLoaded` handler中添加日志：
   ```rust
   log!("📊 Card data: tags={:?}, end_time={:?}, todos_count={}", 
        card.tags, card.end_time, card.todos.len());
   ```

### 问题2: Todos没有保存

**可能原因：**
1. State Event保存失败
2. Todos加载失败
3. UI没有正确显示todos

**调试步骤：**

1. **检查保存日志：**
   查找：
   ```
   💾 Saving X todos for card ...
   ✓ Saved todos successfully
   ```

2. **检查加载日志：**
   查找：
   ```
   📖 Loading todos from room ...
   ```

3. **使用Matrix客户端验证：**
   - 用Element或其他Matrix客户端打开这个房间
   - 查看房间的State Events
   - 找 `m.kanban.card.todos` 事件
   - 看看内容是什么

## 快速测试方案

### 测试1: 验证保存

1. 打开卡片详情
2. 添加一个todo "测试1"
3. 查看日志，应该看到：
   ```
   📝 AddTodo: card_id='...', text='测试1'
   ✅ Added todo in memory immediately
   🔄 Forcing immediate modal redraw
   📝 MatrixRequest::SaveCardTodos received! card_id=..., todos_count=1
   📝 Task started: Saving 1 todos for card ...
   💾 Saving 1 todos for card ...
   ✓ Saved todos successfully
   ✅ Successfully saved todos for card ...
   ```

4. 如果看不到 "✓ Saved todos successfully"，说明保存失败

### 测试2: 验证加载

1. 重启应用
2. 打开同一张卡片
3. 查看日志，应该看到：
   ```
   📖 Loading todos from room ...
   ```
   然后是：
   ```
   📖 Found X todos
   ```
   或：
   ```
   📖 No todos found
   ```

### 测试3: 验证UI显示

1. 在TodoSection的draw_walk中添加日志（如果还没有）
2. 打开卡片详情
3. 查看日志：
   ```
   🎨 TodoSection draw_walk: todos_count=X
   ```

## 可能的修复

### 修复1: 确保CardLoaded更新正确

在 `src/app.rs` 的 `CardLoaded` handler中，确保：
```rust
state.upsert_card(card.clone());  // 这会更新内存中的卡片
```

### 修复2: 确保UI组件读取最新数据

UI组件应该从 `app_state.kanban_state.cards` 读取数据，而不是缓存旧数据。

### 修复3: 检查State Event权限

Matrix服务器可能不允许发送自定义State Event。检查日志中是否有权限错误。

## 使用Element验证

1. 用Element打开卡片房间
2. 点击房间设置 → Advanced → Room Information
3. 查看State Events
4. 找 `m.kanban.card.todos` 事件
5. 看看内容是否正确

如果Element中能看到todos，说明保存成功，问题在加载或显示。
如果Element中看不到，说明保存失败。

## 下一步

请提供以下信息：
1. 添加todo时的完整日志（从点击保存到保存完成）
2. 重启后打开卡片的完整日志
3. TagSection和TodoSection的draw_walk日志
4. 是否能在Element中看到State Event

这样我们就能定位问题所在。
