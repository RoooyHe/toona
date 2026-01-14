# 拖拽排序实现文档

> Toona 项目改造 - 看板拖拽排序系统详细设计

## 文档信息

- **版本**: 1.0
- **创建日期**: 2026-01-14
- **状态**: 设计稿

---

## 目录

1. [概述](#1-概述)
2. [架构设计](#2-架构设计)
3. [排序算法](#3-排序算法)
4. [拖拽处理器](#4-拖拽处理器)
5. [放置区域](#5-放置区域)
6. [视觉反馈](#6-视觉反馈)
7. [状态管理](#7-状态管理)
8. [性能优化](#8-性能优化)

---

## 1. 概述

### 1.1 设计目标

拖拽排序是看板应用的核心交互功能，需要实现：

- **卡片拖拽**: 在列表内和跨列表移动卡片
- **列表拖拽**: 在看板内调整列表顺序
- **实时预览**: 拖拽时显示放置位置预览
- **排序算法**: 高效的排序索引计算
- **性能优化**: 流畅的拖拽体验

### 1.2 技术挑战

| 挑战 | 解决方案 |
|------|----------|
| 排序索引频繁更新 | Lexorank 算法 |
| 跨列表移动 | 应用层维护 list_id |
| 实时预览 | 虚拟拖拽预览 |
| 大列表性能 | 虚拟化 + 增量更新 |
| 触摸设备 | 触摸事件支持 |

---

## 2. 架构设计

### 2.1 拖拽系统架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      拖拽系统架构                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    拖拽管理器                             │   │
│  │  (DragDropManager)                                       │   │
│  │  ├── 管理拖拽状态                                        │   │
│  │  ├── 协调拖拽事件                                        │   │
│  │  └── 触发状态更新                                        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                            │                                    │
│            ┌───────────────┼───────────────┐                   │
│            ▼               ▼               ▼                   │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐  │
│  │   拖拽源        │ │   拖拽处理器    │ │   放置区域      │  │
│  │ (DragSource)    │ │(DragHandler)    │ │ (DropZone)      │  │
│  │                 │ │                 │ │                 │  │
│  │ - KanbanCard    │ │ - 开始拖拽      │ │ - 检测放置      │  │
│  │ - KanbanList    │ │ - 移动拖拽      │ │ - 计算位置      │  │
│  │                 │ │ - 结束拖拽      │ │ - 显示预览      │  │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘  │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    排序管理器                             │   │
│  │  (OrderManager)                                         │   │
│  │  ├── Lexorank 算法实现                                   │   │
│  │  ├── 位置计算                                            │   │
│  │  └── 批量排序                                            │   │
│  └─────────────────────────────────────────────────────────┘   │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    API 层                                │   │
│  │  - CardRequest::MoveCard                                │   │
│  │  - CardRequest::BatchMoveCards                          │   │
│  │  - ListRequest::MoveList                                │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 数据流

```
用户开始拖拽卡片
      │
      ▼
拖拽管理器捕获事件
      │
      ├──► 设置拖拽状态 (is_dragging = true)
      │
      ├──► 创建拖拽预览
      │
      └──► 注册全局移动事件监听器
      
用户移动鼠标/触摸
      │
      ├──► 拖拽处理器更新预览位置
      │
      ├──► 放置区域检测目标
      │
      └──► 计算新位置 (使用 OrderManager)
           │
           ├──► 找到相邻元素
           │
           └──► 计算排序索引
                │
                └──► 更新放置预览 UI
      
用户释放鼠标/触摸
      │
      ├──► 拖拽处理器检测释放
      │
      ├──► 如果有有效放置目标
      │     │
      │     ├──► 构建移动操作
      │     │
      │     ├──► 调用 API
      │     │
      │     └──► 更新 UI 状态
      │
      └──► 清理拖拽状态
           │
           ├──► 移除事件监听器
           │
           └──► 重置拖拽状态
```

---

## 3. 排序算法

### 3.1 Lexorank 算法介绍

Lexorank 是一种用于拖拽排序的算法，通过字符串比较而非数值计算来避免排序索引溢出问题。

#### 算法特点

- **字符串排序**: 使用字符串而非数值作为排序键
- **无限扩展**: 理论上支持无限次插入
- **局部更新**: 只更新受影响元素的排序键
- **简单实现**: 易于理解和维护

#### 基本原理

```
排序键格式: <prefix><rank>

示例:
  列表中有 3 个元素:
    "a0" - 第一个元素
    "a1" - 第二个元素
    "a2" - 第三个元素
  
  在 "a0" 和 "a1" 之间插入:
    计算中间值: "a0" 和 "a1" 的中间值
    结果: "a05" (字典序在 "a0" 和 "a1" 之间)
```

### 3.2 Lexorank 实现

```rust
// src/kanban/drag_drop/order_manager.rs

use std::cmp::Ordering;

/// 排序键管理器 (Lexorank 算法)
pub struct LexorankManager;

impl LexorankManager {
    /// 排序键前缀
    const PREFIX: &str = "a";
    
    /// 中间字符
    const MIDDLE: &str = "m";
    
    /// 新元素前缀
    const NEW_PREFIX: &str = "z";
    
    /// 计算两个排序键的中间值
    pub fn calculate_middle_rank(&self, before: &str, after: &str) -> String {
        let before_len = before.len();
        let after_len = after.len();
        
        // 找到公共前缀长度
        let common_prefix_len = self.common_prefix_length(before, after);
        
        // 如果两个键相同，返回错误
        if before == after {
            return self.insert_between_same(before);
        }
        
        // 找到第一个不同的字符位置
        let diff_index = common_prefix_len;
        
        // 获取需要比较的部分
        let before_suffix = &before[common_prefix_len..];
        let after_suffix = &after[common_prefix_len..];
        
        // 计算中间值
        self.calculate_middle_value(
            before,
            after,
            before_suffix,
            after_suffix,
            diff_index,
        )
    }
    
    /// 计算中间值
    fn calculate_middle_value(
        &self,
        before: &str,
        after: &str,
        before_suffix: &str,
        after_suffix: &str,
        diff_index: usize,
    ) -> String {
        // 获取第一个不同的字符
        let before_char = before_suffix.chars().next().unwrap_or('0') as u8;
        let after_char = after_suffix.chars().next().unwrap_or('z') as u8;
        
        // 计算中间字符
        let middle_char = (before_char as f64 + after_char as f64) / 2.0;
        
        // 如果中间字符不是整数，需要扩展
        if middle_char.fract() != 0.0 {
            return self.expand_and_insert(before, after, diff_index);
        }
        
        let middle_char = middle_char as char;
        
        // 构建新的排序键
        let mut result = String::new();
        result.push_str(&before[..diff_index]);
        result.push(middle_char);
        
        // 如果原键更长，添加后续部分
        if after_suffix.len() > 1 {
            result.push_str(&after_suffix[1..]);
        }
        
        result
    }
    
    /// 扩展并插入 (当两个字符之间没有空间时)
    fn expand_and_insert(&self, before: &str, after: &str, index: usize) -> String {
        // 在两个键之间插入一个字符
        let mut result = String::new();
        result.push_str(&before[..index]);
        result.push_str(Self::MIDDLE);
        result.push_str(&after[index..]);
        result
    }
    
    /// 在相同键之间插入
    fn insert_between_same(&self, key: &str) -> String {
        // 在键后面添加一个字符
        format!("{}a", key)
    }
    
    /// 生成新元素的排序键 (比所有现有元素都大)
    pub fn generate_new_rank(&self, existing_ranks: &[&str]) -> String {
        if existing_ranks.is_empty() {
            return format!("{}0", Self::PREFIX);
        }
        
        // 找到最大的排序键
        let max_rank = existing_ranks
            .iter()
            .max()
            .unwrap();
        
        // 在最大键后追加字符
        format!("{}z", max_rank)
    }
    
    /// 计算在指定位置插入的排序键
    pub fn rank_for_insertion(
        &self,
        position: f64,
        prev_rank: Option<&str>,
        next_rank: Option<&str>,
        existing_ranks: &[f64],
    ) -> String {
        match (prev_rank, next_rank) {
            // 插入到开头
            (None, Some(next)) => {
                let min_rank = existing_ranks
                    .iter()
                    .min()
                    .unwrap_or(&0.0);
                // 在最小值前插入，使用更小的值
                format!("{}a", Self::PREFIX)
            }
            
            // 插入到结尾
            (Some(prev), None) => {
                // 使用 prev + 1000 作为新排序键
                let prev_value = self.rank_to_value(prev);
                self.value_to_rank(prev_value + 1000.0)
            }
            
            // 插入中间
            (Some(prev), Some(next)) => {
                self.calculate_middle_rank(prev, next)
            }
            
            // 空列表
            (None, None) => {
                format!("{}0", Self::PREFIX)
            }
        }
    }
    
    /// 排序键转数值 (用于比较)
    pub fn rank_to_value(&self, rank: &str) -> f64 {
        let prefix_len = rank.len().min(2);
        let suffix = &rank[prefix_len..];
        
        // 将字母数字字符串转换为数值
        let mut value = 0.0;
        let base = 26.0; // 26个字母
        
        for (i, c) in suffix.chars().enumerate() {
            let digit = if c.is_ascii_alphabetic() {
                (c as u8 - b'a' + 10) as f64
            } else {
                (c as u8 - b'0') as f64
            };
            value = value * base + digit;
            value /= base.powi(i as i32 + 1);
        }
        
        value
    }
    
    /// 数值转排序键
    pub fn value_to_rank(&self, value: f64) -> String {
        format!("{}{}", Self::PREFIX, (value * 10000.0) as u32)
    }
    
    /// 找到两个键的公共前缀长度
    fn common_prefix_length(&self, a: &str, b: &str) -> usize {
        let mut len = 0;
        for (ca, cb) in a.chars().zip(b.chars()) {
            if ca == cb {
                len += 1;
            } else {
                break;
            }
        }
        len
    }
    
    /// 批量重新排序
    pub fn reorder_all(&self, items: &mut [impl Sortable]) {
        items.sort_by(|a, b| {
            a.rank().cmp(&b.rank()).then_with(|| {
                a.id().cmp(&b.id())
            })
        });
        
        // 为每个元素分配新的排序键
        for (i, item) in items.iter_mut().enumerate() {
            let new_rank = format!("{}{}", Self::PREFIX, i as f64 * 100.0);
            item.set_rank(&new_rank);
        }
    }
}

/// 可排序 trait
pub trait Sortable {
    fn id(&self) -> &str;
    fn rank(&self) -> &str;
    fn set_rank(&mut self, rank: &str);
}
```

### 3.2 简化版排序管理器

```rust
// src/kanban/drag_drop/order_manager.rs

use std::f64;

/// 简化版排序管理器 (使用浮点数)
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
            // 插入到开头
            (None, Some(after)) => {
                if after > Self::ORDER_INTERVAL {
                    after - Self::ORDER_INTERVAL
                } else {
                    self.reorder_and_insert(None, Some(after))
                }
            }
            
            // 插入到结尾
            (Some(before), None) => {
                before + Self::ORDER_INTERVAL
            }
            
            // 插入中间
            (Some(before), Some(after)) => {
                let middle = (before + after) / 2.0;
                
                // 如果中间还有空间，直接使用
                if middle != before && middle != after {
                    middle
                } else {
                    // 空间不足，需要重新排序
                    self.reorder_and_insert(Some(before), Some(after))
                }
            }
            
            // 空列表
            (None, None) => Self::INITIAL_ORDER,
        }
    }
    
    /// 重新排序并插入
    fn reorder_and_insert(
        &self,
        before_order: Option<f64>,
        after_order: Option<f64>,
    ) -> f64 {
        // 返回一个合理的中间值
        match (before_order, after_order) {
            (Some(before), Some(after)) => (before + after) / 2.0,
            (Some(before), None) => before + Self::ORDER_INTERVAL,
            (None, Some(after)) => after - Self::ORDER_INTERVAL,
            (None, None) => Self::INITIAL_ORDER,
        }
    }
    
    /// 批量重新排序
    pub fn reorder_all(&self, orders: &mut [f64]) {
        orders.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        for (i, order) in orders.iter_mut().enumerate() {
            *order = Self::INITIAL_ORDER + i as f64 * Self::ORDER_INTERVAL;
        }
    }
    
    /// 计算批量移动后的位置
    pub fn calculate_batch_positions(
        &self,
        target_list_orders: &[f64],
        insert_count: usize,
    ) -> Vec<f64> {
        if target_list_orders.is_empty() {
            return (0..insert_count)
                .map(|i| Self::INITIAL_ORDER + i as f64 * Self::ORDER_INTERVAL)
                .collect();
        }
        
        // 在目标列表中分配新位置
        let mut positions = Vec::with_capacity(insert_count);
        let interval = Self::ORDER_INTERVAL / (insert_count + 1) as f64;
        
        for i in 0..insert_count {
            let base = target_list_orders[0];
            positions.push(base + (i as f64 + 1.0) * interval);
        }
        
        positions
    }
}
```

---

## 4. 拖拽处理器

### 4.1 拖拽状态

```rust
// src/kanban/drag_drop/drag_drop_state.rs

use serde::{Deserialize, Serialize};
use super::*;

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
    pub source: Option<DragSource>,
    
    /// 拖拽预览
    #[live]
    pub preview: Option<DragPreview>,
    
    /// 放置目标
    #[live]
    pub drop_target: Option<DropTarget>,
    
    /// 拖拽预览位置
    #[live]
    pub preview_position: (f64, f64),
}

impl Default for DragDropState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_type: DragType::None,
            source: None,
            preview: None,
            drop_target: None,
            preview_position: (0.0, 0.0),
        }
    }
}

/// 拖拽类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, LiveHook)]
pub enum DragType {
    None,
    Card,
    List,
}

/// 拖拽源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragSource {
    /// 看板 ID
    pub board_id: OwnedRoomId,
    
    /// 列表 ID
    pub list_id: String,
    
    /// 卡片 ID (拖拽卡片时)
    pub card_id: Option<String>,
    
    /// 列表 ID (拖拽列表时)
    pub list_id_dragged: Option<String>,
    
    /// 原始排序值
    pub order_index: f64,
    
    /// 拖拽起始位置
    pub start_position: (f64, f64),
}

/// 放置目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropTarget {
    /// 看板 ID
    pub board_id: OwnedRoomId,
    
    /// 列表 ID
    pub list_id: String,
    
    /// 卡片 ID (放置到卡片上时)
    pub card_id: Option<String>,
    
    /// 放置位置
    pub position: DropPosition,
    
    /// 新排序值
    pub new_order_index: f64,
}

/// 放置位置
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DropPosition {
    Before,   // 放置在目标之前
    After,    // 放置在目标之后
    Into,     // 放置在目标内部
    Empty,    // 放置在空列表
}
```

### 4.2 拖拽预览

```rust
// src/kanban/drag_drop/drag_preview.rs

use makepad_widgets::*;

/// 拖拽预览组件
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct DragPreview {
    /// 预览内容
    #[live]
    content: FlowBox,
    
    /// 预览位置
    #[live]
    position: (f64, f64),
    
    /// 预览透明度
    #[live]
    opacity: f64,
    
    /// 预览缩放
    #[live]
    scale: f64,
    
    /// 偏移量
    #[live]
    offset: (f64, f64),
    
    /// 源卡片数据
    source_card: Option<KanbanCard>,
    
    /// 阴影效果
    #[live]
    shadow: Shadow,
}

impl DragPreview {
    pub fn new() -> Self {
        Self {
            content: FlowBox {
                width: 272,
                height: Fit,
                background_color: #FFFFFF,
                border_radius: 3,
                ..Default::default()
            },
            position: (0.0, 0.0),
            opacity: 0.9,
            scale: 1.0,
            offset: (0.0, -20.0),
            source_card: None,
            shadow: Shadow {
                color: #00000033,
                x: 0,
                y: 4,
                blur: 8,
                spread: 0,
            },
        }
    }
    
    /// 从卡片创建预览
    pub fn from_card(&mut self, card: &KanbanCard) {
        self.source_card = Some(card.clone());
        
        // 复制卡片内容作为预览
        self.content = self.create_card_preview(card);
        
        // 设置预览样式
        self.opacity = 0.9;
        self.scale = 1.0;
    }
    
    /// 从列表创建预览
    pub fn from_list(&mut self, list: &KanbanList) {
        // 复制列表内容作为预览
        self.content = self.create_list_preview(list);
        
        // 设置预览样式
        self.opacity = 0.9;
        self.scale = 1.0;
    }
    
    fn create_card_preview(&self, card: &KanbanCard) -> FlowBox {
        FlowBox {
            width: 272,
            height: Fit,
            background_color: #FFFFFF,
            border_radius: 3,
            box_shadow: self.shadow.clone(),
            flow: Down,
            padding: 8,
            spacing: 4,
            ..Default::default()
        }
    }
    
    fn create_list_preview(&self, list: &KanbanList) -> FlowBox {
        FlowBox {
            width: 272,
            height: 100,
            background_color: #EBECF0,
            border_radius: 3,
            box_shadow: self.shadow.clone(),
            flow: Down,
            padding: 8,
            spacing: 4,
            ..Default::default()
        }
    }
    
    /// 更新预览位置
    pub fn update_position(&mut self, x: f64, y: f64) {
        self.position = (x + self.offset.0, y + self.offset.1);
    }
    
    /// 设置透明度
    pub fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
    
    /// 设置缩放
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale.clamp(0.8, 1.1);
    }
    
    /// 设置偏移
    pub fn set_offset(&mut self, x: f64, y: f64) {
        self.offset = (x, y);
    }
}
```

### 4.3 拖拽处理器

```rust
// src/kanban/drag_drop/drag_handler.rs

use super::*;

/// 拖拽处理器
pub struct DragHandler {
    /// 排序管理器
    order_manager: SimpleOrderManager,
    
    /// 当前拖拽状态
    state: DragDropState,
    
    /// 拖拽开始回调
    on_drag_start: Option<Box<dyn FnMut(&DragSource)>>,
    
    /// 拖拽移动回调
    on_drag_move: Option<Box<dyn FnMut(&DragSource, &DropTarget)>>,
    
    /// 拖拽结束回调
    on_drag_end: Option<Box<dyn FnMut(&DragSource, Option<&DropTarget>)>>,
    
    /// 拖拽取消回调
    on_drag_cancel: Option<Box<dyn FnMut(&DragSource)>>,
}

impl DragHandler {
    pub fn new() -> Self {
        Self {
            order_manager: SimpleOrderManager,
            state: DragDropState::default(),
            on_drag_start: None,
            on_drag_move: None,
            on_drag_end: None,
            on_drag_cancel: None,
        }
    }
    
    /// 开始拖拽卡片
    pub fn start_drag_card(
        &mut self,
        board_id: &RoomId,
        list_id: &str,
        card_id: &str,
        order_index: f64,
        position: (f64, f64),
    ) {
        let source = DragSource {
            board_id: board_id.to_owned(),
            list_id: list_id.to_string(),
            card_id: Some(card_id.to_string()),
            list_id_dragged: None,
            order_index,
            start_position: position,
        };
        
        self.state.is_dragging = true;
        self.state.drag_type = DragType::Card;
        self.state.source = Some(source.clone());
        
        // 触发拖拽开始回调
        if let Some(ref mut callback) = self.on_drag_start {
            callback(&source);
        }
    }
    
    /// 开始拖拽列表
    pub fn start_drag_list(
        &mut self,
        board_id: &RoomId,
        list_id: &str,
        order_index: f64,
        position: (f64, f64),
    ) {
        let source = DragSource {
            board_id: board_id.to_owned(),
            list_id: list_id.to_string(),
            card_id: None,
            list_id_dragged: Some(list_id.to_string()),
            order_index,
            start_position: position,
        };
        
        self.state.is_dragging = true;
        self.state.drag_type = DragType::List;
        self.state.source = Some(source.clone());
        
        if let Some(ref mut callback) = self.on_drag_start {
            callback(&source);
        }
    }
    
    /// 处理拖拽移动
    pub fn handle_drag_move(&mut self, position: (f64, f64)) {
        if !self.state.is_dragging {
            return;
        }
        
        // 更新预览位置
        self.state.preview_position = position;
        
        // 检测放置目标
        if let Some(ref source) = self.state.source {
            let target = self.detect_drop_target(source, position);
            self.state.drop_target = target;
            
            // 触发拖拽移动回调
            if let (Some(ref mut callback), Some(ref target)) = 
                (self.on_drag_move, &self.state.drop_target)
            {
                callback(source, target);
            }
        }
    }
    
    /// 处理拖拽释放
    pub fn handle_drag_end(&mut self) -> Option<DropTarget> {
        if !self.state.is_dragging {
            return None;
        }
        
        let target = self.state.drop_target.clone();
        let source = self.state.source.clone();
        
        // 触发拖拽结束回调
        if let (Some(ref mut callback), Some(ref src)) = 
            (self.on_drag_end, &source)
        {
            callback(src, target.as_ref());
        }
        
        // 重置状态
        self.reset_state();
        
        target
    }
    
    /// 取消拖拽
    pub fn cancel_drag(&mut self) {
        if let (Some(ref mut callback), Some(ref source)) = 
            (self.on_drag_cancel, &self.state.source)
        {
            callback(source);
        }
        
        self.reset_state();
    }
    
    /// 检测放置目标
    fn detect_drop_target(
        &self,
        source: &DragSource,
        position: (f64, f64),
    ) -> Option<DropTarget> {
        // 这里需要访问 UI 组件的布局信息
        // 实际实现中需要从 UI 层获取
        
        // 伪代码：
        // for each list in visible_lists:
        //     if position is within list.bounds:
        //         for each card in list.cards:
        //             if position is within card.bounds:
        //                 return DropTarget {
        //                     list_id: list.id,
        //                     card_id: Some(card.id),
        //                     position: if position.y < card.center.y { Before } else { After },
        //                     new_order_index: calculate_order(card, position),
        //                 }
        //         return DropTarget {
        //             list_id: list.id,
        //             card_id: None,
        //             position: Empty,
        //             new_order_index: list.cards.last().order_index + 1000,
        //         }
        
        None // 实际返回需要从 UI 层获取
    }
    
    /// 重置状态
    fn reset_state(&mut self) {
        self.state = DragDropState::default();
    }
    
    /// 设置回调
    pub fn set_on_drag_start<F>(&mut self, callback: F)
    where
        F: FnMut(&DragSource) + 'static,
    {
        self.on_drag_start = Some(Box::new(callback));
    }
    
    pub fn set_on_drag_move<F>(&mut self, callback: F)
    where
        F: FnMut(&DragSource, &DropTarget) + 'static,
    {
        self.on_drag_move = Some(Box::new(callback));
    }
    
    pub fn set_on_drag_end<F>(&mut self, callback: F)
    where
        F: FnMut(&DragSource, Option<&DropTarget>) + 'static,
    {
        self.on_drag_end = Some(Box::new(callback));
    }
    
    pub fn set_on_drag_cancel<F>(&mut self, callback: F)
    where
        F: FnMut(&DragSource) + 'static,
    {
        self.on_drag_cancel = Some(Box::new(callback));
    }
}
```

---

## 5. 放置区域

### 5.1 放置区域指示器

```rust
// src/kanban/drag_drop/drop_zone.rs

use makepad_widgets::*;

/// 放置区域指示器
#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct DropZoneIndicator {
    /// 指示器类型
    #[live]
    indicator_type: DropIndicatorType,
    
    /// 指示器位置
    #[live]
    position: DropIndicatorPosition,
    
    /// 可见性
    #[live]
    visible: bool,
    
    /// 颜色
    #[live]
    color: Color,
    
    /// 高度
    #[live]
    height: f64,
}

impl DropZoneIndicator {
    pub fn new() -> Self {
        Self {
            indicator_type: DropIndicatorType::Line,
            position: DropIndicatorPosition::None,
            visible: false,
            color: color!("#0079BF"),
            height: 2.0,
        }
    }
    
    /// 显示卡片放置指示器
    pub fn show_card_indicator(
        &mut self,
        position: DropIndicatorPosition,
        card_bounds: (f64, f64, f64, f64),
    ) {
        self.indicator_type = DropIndicatorType::Line;
        self.position = position;
        self.visible = true;
        
        match position {
            DropIndicatorPosition::Before => {
                self.height = 2.0;
            }
            DropIndicatorPosition::After => {
                self.height = 2.0;
            }
            _ => {}
        }
    }
    
    /// 显示列表放置指示器
    pub fn show_list_indicator(&mut self, list_bounds: (f64, f64, f64, f64)) {
        self.indicator_type = DropIndicatorType::Highlight;
        self.position = DropIndicatorPosition::Into;
        self.visible = true;
    }
    
    /// 隐藏指示器
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    /// 设置高亮颜色
    pub fn set_highlight_color(&mut self, is_valid: bool) {
        if is_valid {
            self.color = color!("#0079BF"); // 蓝色 - 有效
        } else {
            self.color = color!("#EB5A46"); // 红色 - 无效
        }
    }
}

/// 指示器类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, LiveHook)]
pub enum DropIndicatorType {
    None,
    Line,       // 线条指示器
    Highlight,  // 高亮指示器
    Placeholder, // 占位指示器
}

/// 指示器位置
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, LiveHook)]
pub enum DropIndicatorPosition {
    None,
    Before,     // 在目标之前
    After,      // 在目标之后
    Into,       // 在目标内部
    Empty,      // 空列表
}
```

### 5.2 放置区域检测

```rust
// src/kanban/drag_drop/drop_zone_detector.rs

use super::*;

/// 放置区域检测器
pub struct DropZoneDetector;

impl DropZoneDetector {
    /// 检测卡片放置位置
    pub fn detect_card_drop_zone(
        &self,
        pointer_pos: (f64, f64),
        cards: &[CardBounds],
    ) -> Option<CardDropZone> {
        for card in cards {
            let bounds = card.bounds;
            
            // 检查是否在卡片区域内
            if self.is_point_in_rect(pointer_pos, bounds) {
                let center_y = bounds.1 + bounds.3 / 2.0;
                
                let position = if pointer_pos.1 < center_y {
                    DropIndicatorPosition::Before
                } else {
                    DropIndicatorPosition::After
                };
                
                let new_order = self.calculate_card_order(
                    card.order_index,
                    position,
                    cards,
                );
                
                return Some(CardDropZone {
                    list_id: card.list_id.clone(),
                    card_id: Some(card.id.clone()),
                    position,
                    new_order_index: new_order,
                });
            }
        }
        
        // 检查是否在列表空白区域
        if let Some(empty_zone) = self.detect_empty_zone(pointer_pos, cards) {
            return Some(empty_zone);
        }
        
        None
    }
    
    /// 检测空列表放置区域
    fn detect_empty_zone(
        &self,
        pointer_pos: (f64, f64),
        cards: &[CardBounds],
    ) -> Option<CardDropZone> {
        if cards.is_empty() {
            Some(CardDropZone {
                list_id: cards.first()?.list_id.clone(),
                card_id: None,
                position: DropIndicatorPosition::Empty,
                new_order_index: 1000.0,
            })
        } else {
            None
        }
    }
    
    /// 计算卡片排序值
    fn calculate_card_order(
        &self,
        card_order: f64,
        position: DropIndicatorPosition,
        all_cards: &[CardBounds],
    ) -> f64 {
        let order_manager = SimpleOrderManager;
        
        let prev_order = all_cards
            .iter()
            .filter(|c| c.order_index < card_order)
            .max_by(|a, b| a.order_index.partial_cmp(&b.order_index).unwrap())
            .map(|c| c.order_index);
        
        let next_order = all_cards
            .iter()
            .filter(|c| c.order_index > card_order)
            .min_by(|a, b| a.order_index.partial_cmp(&b.order_index).unwrap())
            .map(|c| c.order_index);
        
        match position {
            DropIndicatorPosition::Before => {
                order_manager.calculate_new_position(prev_order, Some(card_order))
            }
            DropIndicatorPosition::After => {
                order_manager.calculate_new_position(Some(card_order), next_order)
            }
            _ => card_order,
        }
    }
    
    /// 检测列表放置位置
    pub fn detect_list_drop_zone(
        &self,
        pointer_pos: (f64, f64),
        lists: &[ListBounds],
    ) -> Option<ListDropZone> {
        for list in lists {
            let bounds = list.bounds;
            
            if self.is_point_in_rect(pointer_pos, bounds) {
                let center_x = bounds.0 + bounds.2 / 2.0;
                
                let position = if pointer_pos.0 < center_x {
                    DropIndicatorPosition::Before
                } else {
                    DropIndicatorPosition::After
                };
                
                return Some(ListDropZone {
                    list_id: list.id.clone(),
                    position,
                    new_order_index: list.order_index,
                });
            }
        }
        
        None
    }
    
    /// 检查点是否在矩形内
    fn is_point_in_rect(&self, point: (f64, f64), rect: (f64, f64, f64, f64)) -> bool {
        point.0 >= rect.0 && point.0 <= rect.0 + rect.2 &&
            point.1 >= rect.1 && point.1 <= rect.1 + rect.3
    }
}

/// 卡片边界信息
#[derive(Debug, Clone)]
pub struct CardBounds {
    pub id: String,
    pub list_id: String,
    pub order_index: f64,
    pub bounds: (f64, f64, f64, f64), // x, y, width, height
}

/// 卡片放置区域
#[derive(Debug, Clone)]
pub struct CardDropZone {
    pub list_id: String,
    pub card_id: Option<String>,
    pub position: DropIndicatorPosition,
    pub new_order_index: f64,
}

/// 列表边界信息
#[derive(Debug, Clone)]
pub struct ListBounds {
    pub id: String,
    pub order_index: f64,
    pub bounds: (f64, f64, f64, f64),
}

/// 列表放置区域
#[derive(Debug, Clone)]
pub struct ListDropZone {
    pub list_id: String,
    pub position: DropIndicatorPosition,
    pub new_order_index: f64,
}
```

---

## 6. 视觉反馈

### 6.1 拖拽样式

```rust
// src/kanban/drag_drop/styles.rs

/// 拖拽相关样式
pub fn drag_drop_styles() -> Styles {
    Styles::new()
        // 拖拽卡片样式
        .style(
            "kanban_card.dragging",
            Style {
                opacity: 0.5,
                transform: Transform::scale(1.02),
                box_shadow: BoxShadow {
                    color: Color::from_rgb(0, 0, 0, 0.2),
                    x: 0,
                    y: 8,
                    blur: 16,
                    spread: 0,
                },
                ..Default::default()
            },
        )
        // 拖拽预览样式
        .style(
            "drag_preview",
            Style {
                opacity: 0.9,
                box_shadow: BoxShadow {
                    color: Color::from_rgb(0, 0, 0, 0.25),
                    x: 0,
                    y: 4,
                    blur: 12,
                    spread: 0,
                },
                ..Default::default()
            },
        )
        // 放置指示器样式
        .style(
            "drop_indicator",
            Style {
                background_color: color!("#0079BF"),
                height: pixel(2.0),
                width: Stretch(1.0),
                ..Default::default()
            },
        )
        // 有效放置区域样式
        .style(
            "drop_zone.valid",
            Style {
                background_color: Color::from_rgba(0, 121, 191, 0.1),
                border_color: color!("#0079BF"),
                border_width: pixel(2.0),
                ..Default::default()
            },
        )
        // 无效放置区域样式
        .style(
            "drop_zone.invalid",
            Style {
                background_color: Color::from_rgba(235, 90, 70, 0.1),
                border_color: color!("#EB5A46"),
                border_width: pixel(2.0),
                ..Default::default()
            },
        )
        // 列表拖拽样式
        .style(
            "kanban_list.dragging",
            Style {
                opacity: 0.7,
                transform: Transform::rotate(2.0),
                box_shadow: BoxShadow {
                    color: Color::from_rgb(0, 0, 0, 0.2),
                    x: 0,
                    y: 4,
                    blur: 12,
                    spread: 0,
                },
                ..Default::default()
            },
        )
        // 列表占位符样式
        .style(
            "list_placeholder",
            Style {
                background_color: Color::from_rgba(0, 0, 0, 0.05),
                border_style: BorderStyle::dashed,
                border_color: color!("#DFE1E6"),
                border_width: pixel(2.0),
                border_radius: pixel(3.0),
                ..Default::default()
            },
        )
}
```

### 6.2 动画效果

```rust
// src/kanban/drag_drop/animations.rs

/// 拖拽动画配置
pub struct DragDropAnimations {
    /// 拖拽开始动画
    pub drag_start_duration: f64,
    pub drag_start_easing: Easing,
    
    /// 拖拽移动动画
    pub drag_move_duration: f64,
    pub drag_move_easing: Easing,
    
    /// 放置动画
    pub drop_duration: f64,
    pub drop_easing: Easing,
    
    /// 取消动画
    pub cancel_duration: f64,
    pub cancel_easing: Easing,
    
    /// 列表移动动画
    pub list_move_duration: f64,
}

impl Default for DragDropAnimations {
    fn default() -> Self {
        Self {
            drag_start_duration: 0.15,
            drag_start_easing: Easing::CubicOut,
            drag_move_duration: 0.1,
            drag_move_easing: Easing::Linear,
            drop_duration: 0.3,
            drop_easing: Easing::Spring,
            cancel_duration: 0.25,
            cancel_easing: Easing::CubicOut,
            list_move_duration: 0.2,
        }
    }
}

/// 动画执行器
pub struct AnimationExecutor;

impl AnimationExecutor {
    /// 执行卡片放置动画
    pub fn animate_card_drop(
        &self,
        card: &mut KanbanCard,
        from_position: (f64, f64),
        to_position: (f64, f64),
        animations: &DragDropAnimations,
    ) {
        // 使用 CSS 动画或 Makepad 动画系统
        // 伪代码：
        // card.style.transition = format!("transform {} {}", 
        //     animations.drop_duration, animations.drop_easing);
        // card.style.transform = format!("translate(0, 0)");
    }
    
    /// 执行列表移动动画
    pub fn animate_list_move(
        &self,
        list: &mut KanbanList,
        from_index: usize,
        to_index: usize,
        animations: &DragDropAnimations,
    ) {
        // 列表重新排序动画
        // 使用 FLIP (First, Last, Invert, Play) 技术
    }
}
```

---

## 7. 状态管理

### 7.1 拖拽状态与 UI 同步

```rust
// src/kanban/drag_drop/state_sync.rs

/// 拖拽状态同步器
pub struct DragDropStateSync {
    /// UI 状态引用
    ui_state: Rc<KanbanUIState>,
    
    /// 拖拽状态
    drag_drop_state: Rc<DragDropState>,
    
    /// 乐观更新队列
    optimistic_queue: Vec<OptimisticUpdate>,
}

impl DragDropStateSync {
    pub fn new(
        ui_state: Rc<KanbanUIState>,
        drag_drop_state: Rc<DragDropState>,
    ) -> Self {
        Self {
            ui_state,
            drag_drop_state,
            optimistic_queue: Vec::new(),
        }
    }
    
    /// 应用乐观更新
    pub fn apply_optimistic_update(&mut self, update: OptimisticUpdate) {
        // 1. 保存更新到队列
        self.optimistic_queue.push(update.clone());
        
        // 2. 立即更新 UI
        self.apply_update_to_ui(&update);
    }
    
    /// 回滚乐观更新
    pub fn rollback_optimistic_update(&mut self, update_id: &str) {
        // 1. 找到更新
        if let Some(index) = self.optimistic_queue
            .iter()
            .position(|u| u.id == update_id)
        {
            let update = self.optimistic_queue.remove(index);
            
            // 2. 反向更新 UI
            self.rollback_update_from_ui(&update);
        }
    }
    
    /// 应用更新到 UI
    fn apply_update_to_ui(&self, update: &OptimisticUpdate) {
        match update {
            OptimisticUpdate::MoveCard { from_list, to_list, card_id, new_order } => {
                // 从原列表移除
                self.ui_state.list_state.lists
                    .iter_mut()
                    .find(|l| l.id == from_list)
                    .map(|list| {
                        list.cards.retain(|c| c.id != *card_id);
                    });
                
                // 添加到新列表
                self.ui_state.list_state.lists
                    .iter_mut()
                    .find(|l| l.id == to_list)
                    .map(|list| {
                        // 插入到正确位置
                        let card = self.find_card_by_id(card_id);
                        if let Some(card) = card {
                            list.cards.push(card);
                            list.cards.sort_by(|a, b| a.order_index.partial_cmp(&b.order_index).unwrap());
                        }
                    });
            }
            
            OptimisticUpdate::MoveList { from_index, to_index, list_id } => {
                // 移动列表位置
                self.ui_state.list_state.lists
                    .reorder(|lists| {
                        // 重新排序
                    });
            }
        }
    }
    
    /// 从 UI 回滚更新
    fn rollback_update_from_ui(&self, update: &OptimisticUpdate) {
        match update {
            OptimisticUpdate::MoveCard { from_list, to_list, card_id, old_order, .. } => {
                // 恢复原列表
                self.ui_state.list_state.lists
                    .iter_mut()
                    .find(|l| l.id == from_list)
                    .map(|list| {
                        let card = self.find_card_by_id(card_id);
                        if let Some(card) = card {
                            list.cards.push(card);
                            list.cards.retain(|c| c.order_index != *old_order);
                        }
                    });
                
                // 从新列表移除
                self.ui_state.list_state.lists
                    .iter_mut()
                    .find(|l| l.id == to_list)
                    .map(|list| {
                        list.cards.retain(|c| c.id != *card_id);
                    });
            }
            
            _ => {}
        }
    }
    
    fn find_card_by_id(&self, card_id: &str) -> Option<KanbanCard> {
        self.ui_state.list_state.lists
            .iter()
            .flat_map(|l| l.cards.iter())
            .find(|c| c.id == card_id)
            .cloned()
    }
}

/// 乐观更新
#[derive(Debug, Clone)]
pub enum OptimisticUpdate {
    MoveCard {
        id: String,
        card_id: String,
        from_list: String,
        to_list: String,
        old_order: f64,
        new_order: f64,
    },
    MoveList {
        id: String,
        list_id: String,
        from_index: usize,
        to_index: usize,
    },
    BatchMove {
        id: String,
        moves: Vec<CardMoveOperation>,
    },
}
```

---

## 8. 性能优化

### 8.1 虚拟化

```rust
// src/kanban/drag_drop/virtualization.rs

/// 虚拟化拖拽管理器
pub struct VirtualizedDragDrop {
    /// 视口信息
    viewport: ViewportInfo,
    
    /// 可见区域索引
    visible_range: (usize, usize),
    
    /// 虚拟列表容器
    virtual_list: VirtualList<KanbanCard>,
    
    /// 拖拽项在虚拟列表中的位置
    drag_item_virtual_index: usize,
}

impl VirtualizedDragDrop {
    /// 根据视口计算可见范围
    pub fn update_visible_range(
        &mut self,
        viewport_height: f64,
        card_height: f64,
        total_cards: usize,
        scroll_offset: f64,
    ) {
        let start = (scroll_offset / card_height).floor() as usize;
        let visible_count = (viewport_height / card_height).ceil() as usize;
        let end = (start + visible_count + 2).min(total_cards); // +2 缓冲
        
        self.visible_range = (start, end);
    }
    
    /// 获取可见的卡片索引
    pub fn get_visible_indices(&self) -> (usize, usize) {
        self.visible_range
    }
    
    /// 检查拖拽项是否可见
    pub fn is_drag_item_visible(&self) -> bool {
        let (start, end) = self.visible_range;
        self.drag_item_virtual_index >= start && self.drag_item_virtual_index < end
    }
}

/// 虚拟列表
pub struct VirtualList<T> {
    /// 全部数据
    items: Vec<T>,
    
    /// 视口高度
    viewport_height: f64,
    
    /// 元素高度
    item_height: f64,
    
    /// 滚动偏移
    scroll_offset: f64,
    
    /// 缓存的渲染项
    cached_items: Vec<VirtualItem<T>>,
}

impl<T> VirtualList<T> {
    /// 渲染可见项
    pub fn render_visible_items(&self) -> Vec<&VirtualItem<T>> {
        let (start, end) = self.calculate_visible_range();
        self.cached_items[start..end].iter().collect()
    }
    
    /// 计算可见范围
    fn calculate_visible_range(&self) -> (usize, usize) {
        let start = (self.scroll_offset / self.item_height).floor() as usize;
        let visible_count = (self.viewport_height / self.item_height).ceil() as usize;
        let end = (start + visible_count + 2).min(self.items.len());
        
        (start, end)
    }
}

/// 虚拟项
pub struct VirtualItem<T> {
    /// 原始数据
    data: T,
    
    /// 渲染位置
    y_position: f64,
    
    /// 是否可见
    is_visible: bool,
}
```

### 8.2 批量操作

```rust
// src/kanban/drag_drop/batch_operations.rs

/// 批量操作优化
pub struct BatchOperationOptimizer {
    /// 待处理的移动操作
    pending_moves: Vec<CardMove>,
    
    /// 批量处理阈值
    batch_threshold: usize,
    
    /// 批量处理计时器
    batch_timer: Option<tokio::time::Delay>,
}

impl BatchOperationOptimizer {
    pub fn new(batch_threshold: usize) -> Self {
        Self {
            pending_moves: Vec::new(),
            batch_threshold,
            batch_timer: None,
        }
    }
    
    /// 添加移动操作
    pub fn add_move(&mut self, move_op: CardMove) {
        // 检查是否已有相同列表的操作
        if let Some(existing) = self.pending_moves
            .iter_mut()
            .find(|m| m.to_list_id == move_op.to_list_id)
        {
            // 合并到同一列表的操作
            // 重新计算排序值
            self.recalculate_orders(move_op.to_list_id.clone());
        } else {
            self.pending_moves.push(move_op);
        }
        
        // 检查是否达到批量阈值
        if self.pending_moves.len() >= self.batch_threshold {
            self.flush_batch();
        }
    }
    
    /// 刷新批量操作
    pub fn flush_batch(&mut self) {
        if self.pending_moves.is_empty() {
            return;
        }
        
        // 批量发送 API 请求
        let moves = std::mem::replace(&mut self.pending_moves, Vec::new());
        
        // 调用批量移动 API
        // submit_async_request(CardRequest::BatchMoveCards { moves });
    }
    
    /// 重新计算排序值
    fn recalculate_orders(&mut self, list_id: String) {
        let orders: Vec<_> = self.pending_moves
            .iter()
            .filter(|m| m.to_list_id == list_id)
            .enumerate()
            .map(|(i, _)| 1000.0 + i as f64 * 100.0)
            .collect();
        
        for (i, order) in orders.iter().enumerate() {
            if let Some(m) = self.pending_moves
                .iter_mut()
                .filter(|m| m.to_list_id == list_id)
                .nth(i)
            {
                m.new_position = *order;
            }
        }
    }
}
```

---

## 附录

### A. 拖拽事件处理流程

```
1. 鼠标按下 (mousedown/touchstart)
   └── 识别拖拽源
       └── 创建拖拽预览
       └── 设置拖拽状态
       └── 注册全局移动/释放事件

2. 鼠标移动 (mousemove/touchmove)
   └── 更新预览位置
   └── 检测放置区域
   └── 计算新位置
   └── 更新视觉反馈

3. 鼠标释放 (mouseup/touchend)
   └── 移除全局事件
   └── 执行放置操作
   └── 清理拖拽状态

4. 取消 (mouseleave/esc key)
   └── 移除全局事件
   └── 返回原位置
   └── 清理拖拽状态
```

### B. 性能指标

| 操作 | 目标性能 | 说明 |
|------|----------|------|
| 拖拽开始 | < 16ms | 创建预览的时间 |
| 拖拽移动 | < 8ms | 每帧更新预览 |
| 放置检测 | < 4ms | 检测放置区域 |
| 排序计算 | < 1ms | 计算新排序值 |
| UI 更新 | < 16ms | 应用拖拽效果 |

### C. 兼容性

| 功能 | Chrome | Firefox | Safari | Edge |
|------|--------|---------|--------|------|
| 拖拽 API | ✅ | ✅ | ✅ | ✅ |
| 触摸事件 | ✅ | ✅ | ✅ | ✅ |
| CSS 变换 | ✅ | ✅ | ✅ | ✅ |
| 虚拟滚动 | ✅ | ✅ | ✅ | ✅ |

---

> 文档版本: 1.0
> 最后更新: 2026-01-14
