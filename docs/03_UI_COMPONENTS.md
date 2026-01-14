# UI 组件设计文档

> Toona 项目改造 - 看板 UI 组件详细设计

## 文档信息

- **版本**: 1.0
- **创建日期**: 2026-01-14
- **状态**: 设计稿

---

## 目录

1. [设计原则](#1-设计原则)
2. [组件架构](#2-组件架构)
3. [核心组件](#3-核心组件)
4. [业务组件](#4-业务组件)
5. [弹窗组件](#5-弹窗组件)
6. [样式设计](#6-样式设计)
7. [响应式设计](#7-响应式设计)
8. [组件交互](#8-组件交互)

---

## 1. 设计原则

### 1.1 设计目标

- **一致性**: 与现有 Toona 应用保持视觉和交互一致
- **可复用性**: 组件可独立使用，也可组合使用
- **可访问性**: 良好的键盘导航和屏幕阅读器支持
- **性能优化**: 避免不必要的重渲染
- **响应式**: 适配桌面和移动设备

### 1.2 设计规范

参考 Trello 和 Material Design 的设计规范：

| 属性 | 桌面端 | 移动端 |
|------|--------|--------|
| 看板宽度 | 自适应，最大 2400px | 100% |
| 列表宽度 | 272px (最小) | 280px |
| 卡片宽度 | 100% - 24px | 100% - 16px |
| 列表间距 | 12px | 8px |
| 卡片间距 | 8px | 4px |

### 1.3 色彩系统

```rust
// src/kanban_ui/styles/colors.rs

/// 看板应用色彩系统
pub struct KanbanColors {
    /// 主色
    pub primary: Color,
    
    /// 背景色
    pub background: Color,
    
    /// 表面色
    pub surface: Color,
    
    /// 卡片背景
    pub card_background: Color,
    
    /// 列表背景
    pub list_background: Color,
    
    /// 文字颜色
    pub text: Color,
    
    /// 次要文字
    pub text_secondary: Color,
    
    /// 边框颜色
    pub border: Color,
    
    /// 成功色
    pub success: Color,
    
    /// 警告色
    pub warning: Color,
    
    /// 错误色
    pub error: Color,
}

impl Default for KanbanColors {
    fn default() -> Self {
        Self {
            primary: color!("#0079BF"),
            background: color!("#F4F5F7"),
            surface: color!("#FFFFFF"),
            card_background: color!("#FFFFFF"),
            list_background: color!("#EBECF0"),
            text: color!("#172B4D"),
            text_secondary: color!("#5E6C84"),
            border: color!("#DFE1E6"),
            success: color!("#61BD4F"),
            warning: color!("#FF9F1A"),
            error: color!("#EB5A46"),
        }
    }
}

/// 标签颜色
pub struct LabelColors;

impl LabelColors {
    pub const GREEN: Color = color!("#61BD4F");
    pub const YELLOW: Color = color!("#F2D600");
    pub const ORANGE: Color = color!("#FF9F1A");
    pub const RED: Color = color!("#EB5A46");
    pub const PURPLE: Color = color!("#C377E0");
    pub const BLUE: Color = color!("#0079BF");
    pub const SKY: Color = color!("#00C2E0");
    pub const LIME: Color = color!("#51E898");
    pub const PINK: Color = color!("#FF78CB");
    pub const BLACK: Color = color!("#344563");
}
```

---

## 2. 组件架构

### 2.1 组件树

```
kanban_ui/
├── mod.rs
├── styles/
│   ├── mod.rs
│   ├── colors.rs
│   ├── typography.rs
│   └── shadows.rs
│
├── workspace/
│   ├── mod.rs
│   ├── kanban_workspace.rs          # 看板工作区容器
│   └── kanban_desktop_workspace.rs  # 桌面端工作区
│
├── board/
│   ├── mod.rs
│   ├── board_view.rs                # 看板主视图
│   ├── board_header.rs              # 看板头部
│   ├── board_toolbar.rs             # 看板工具栏
│   ├── board_menu.rs                # 看板菜单
│   ├── board_background.rs          # 看板背景
│   └── board_archived_items.rs      # 归档项目
│
├── list/
│   ├── mod.rs
│   ├── kanban_list.rs               # 列表组件
│   ├── list_header.rs               # 列表头部
│   ├── list_menu.rs                 # 列表菜单
│   ├── list_cards_container.rs      # 卡片容器
│   ├── add_card.rs                  # 添加卡片
│   └── quick_card_editor.rs         # 快速编辑卡片
│
├── card/
│   ├── mod.rs
│   ├── kanban_card.rs               # 卡片组件
│   ├── card_labels.rs               # 卡片标签
│   ├── card_members.rs              # 卡片成员
│   ├── card_badges.rs               # 卡片徽章
│   ├── card_cover.rs                # 卡片封面
│   └── card_drag_handle.rs          # 拖拽手柄
│
├── modal/
│   ├── mod.rs
│   ├── card_modal.rs                # 卡片详情弹窗
│   ├── card_title_editor.rs         # 卡片标题编辑
│   ├── card_description_editor.rs   # 卡片描述编辑
│   ├── card_labels_editor.rs        # 标签编辑
│   ├── card_members_editor.rs       # 成员编辑
│   ├── card_due_date_editor.rs      # 截止日期编辑
│   ├── card_attachments_editor.rs   # 附件编辑
│   ├── card_checklists_editor.rs    # 检查清单编辑
│   ├── card_activity.rs             # 活动记录
│   ├── card_comments.rs             # 评论列表
│   ├── add_list_modal.rs            # 添加列表弹窗
│   ├── copy_card_modal.rs           # 复制卡片弹窗
│   └── delete_confirm_modal.rs      # 删除确认弹窗
│
├── toolbar/
│   ├── mod.rs
│   ├── filter_bar.rs                # 筛选栏
│   ├── sort_bar.rs                  # 排序栏
│   └── action_bar.rs                # 操作栏
│
├── filter/
│   ├── mod.rs
│   ├── filter_menu.rs               # 筛选菜单
│   ├── member_filter.rs             # 成员筛选
│   ├── label_filter.rs              # 标签筛选
│   └── search_input.rs              # 搜索输入
│
├── sidebar/
│   ├── mod.rs
│   ├── boards_sidebar.rs            # 看板侧边栏
│   ├── board_item.rs                # 看板项
│   └── board_list_item.rs           # 看板列表项
│
├── drag_drop/
│   ├── mod.rs
│   ├── drag_drop_container.rs       # 拖拽容器
│   ├── drag_preview.rs              # 拖拽预览
│   └── drop_zone_indicator.rs       # 放置区域指示器
│
└── common/
    ├── mod.rs
    ├── avatar.rs                    # 头像组件
    ├── badge.rs                     # 徽章组件
    ├── button.rs                    # 按钮组件
    ├── input.rs                     # 输入框组件
    ├── menu.rs                      # 菜单组件
    ├── tooltip.rs                   # 提示组件
    └── empty_state.rs               # 空状态组件
```

### 2.2 组件依赖关系

```
┌─────────────────────────────────────────────────────────────────┐
│                        组件依赖关系                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  KanbanWorkspace                                                │
│  ├── KanbanSidebar                                              │
│  │   └── BoardListItem                                         │
│  └── KanbanBoardView                                            │
│      ├── BoardHeader                                            │
│      │   └── BoardMenu                                         │
│      ├── BoardToolbar                                          │
│      │   ├── FilterBar                                         │
│      │   │   └── FilterMenu                                    │
│      │   └── SortBar                                           │
│      └── ScrollContainer                                       │
│          └── ForEach<List>                                     │
│              └── KanbanList                                    │
│                  ├── ListHeader                                │
│                  │   └── ListMenu                              │
│                  ├── ScrollContainer                           │
│                  │   └── ForEach<Card>                         │
│                  │       └── KanbanCard                        │
│                  │           ├── CardLabels                    │
│                  │           ├── CardMembers                   │
│                  │           ├── CardBadges                    │
│                  │           ├── CardCover                     │
│                  │           └── CardDragHandle                │
│                  └── AddCard                                   │
│                      └── QuickCardEditor                       │
│                                                                 │
│  CardModal (Overlay)                                            │
│  ├── CardTitleEditor                                            │
│  ├── CardDescriptionEditor                                      │
│  ├── CardLabelsEditor                                           │
│  ├── CardMembersEditor                                          │
│  ├── CardDueDateEditor                                          │
│  ├── CardAttachmentsEditor                                      │
│  ├── CardChecklistsEditor                                       │
│  ├── CardComments                                               │
│  └── CardActivity                                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. 核心组件

### 3.1 看板工作区 (KanbanWorkspace)

```rust
// src/kanban_ui/workspace/kanban_workspace.rs

live_design! {
    kanban_workspace = {{KanbanWorkspace}} {
        flow: Right,
        width: Fill,
        height: Fill,
        
        /// 侧边栏
        sidebar = {
            width: 272,
            height: Fill,
            background_color: #FFFFFF,
            border_right: 1, #DFE1E6,
        }
        
        /// 主内容区
        main_content = {
            flow: Down,
            width: Fill,
            height: Fill,
            background_color: #F4F5F7,
        }
        
        /// 看板头部
        header = {
            height: 48,
            background_color: #FFFFFF,
            border_bottom: 1, #DFE1E6,
        }
        
        /// 工具栏
        toolbar = {
            height: 40,
            background_color: #F4F5F7,
        }
        
        /// 看板视图
        board_view = {
            flow: Down,
            width: Fill,
            height: Fill,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct KanbanWorkspace {
    /// 侧边栏组件
    #[live]
    sidebar: BoardsSidebar,
    
    /// 看板视图
    #[live]
    board_view: KanbanBoardView,
    
    /// 看板状态
    #[live]
    state: KanbanAppState,
    
    /// 是否显示侧边栏
    #[live]
    sidebar_visible: bool,
}

impl KanbanWorkspace {
    pub fn set_board(&mut self, board: &KanbanBoard) {
        self.state.board_state.current_board = Some(board.clone());
        self.board_view.set_board(board);
    }
    
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
        self.sidebar.set_visible(self.sidebar_visible);
    }
    
    pub fn set_on_board_select<F>(&mut self, callback: F)
    where
        F: FnMut(&RoomId) + 'static,
    {
        self.sidebar.set_on_board_select(callback);
    }
}
```

### 3.2 看板视图 (KanbanBoardView)

```rust
// src/kanban_ui/board/board_view.rs

live_design! {
    kanban_board_view = {{KanbanBoardView}} {
        flow: Down,
        width: Fill,
        height: Fill,
        
        /// 看板头部
        header = {
            height: 48,
            background_color: #FFFFFF,
            border_bottom: 1, #DFE1E6,
            flow: Right,
            align: {x: 0.0, y: 0.5},
            padding: 12,
            spacing: 8,
        }
        
        /// 工具栏
        toolbar = {
            height: 40,
            background_color: #F4F5F7,
            flow: Right,
            align: {x: 0.0, y: 0.5},
            padding: 8,
            spacing: 4,
        }
        
        /// 看板内容滚动区
        board_content = {
            flow: Right,
            width: Fill,
            height: Fill,
            scroll: {x: true, y: false},
            spacing: 12,
            padding: 12,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct KanbanBoardView {
    /// 看板头部
    #[live]
    header: BoardHeader,
    
    /// 工具栏
    #[live]
    toolbar: BoardToolbar,
    
    /// 看板内容滚动区
    #[live]
    board_content: FlowBox,
    
    /// 看板数据
    #[live]
    board: Option<KanbanBoard>,
    
    /// 列表数据
    #[live]
    lists: Vec<KanbanList>,
    
    /// 拖拽状态
    #[live]
    drag_drop_state: DragDropState,
}

impl KanbanBoardView {
    pub fn set_board(&mut self, board: &KanbanBoard) {
        self.board = Some(board.clone());
        self.header.set_board_name(&board.name);
    }
    
    pub fn set_lists(&mut self, lists: Vec<KanbanList>) {
        self.lists = lists;
        self.board_content = FlowBox {
            width: Fill,
            height: Fill,
            spacing: 12,
            ..Default::default()
        };
        
        for list in &self.lists {
            let list_component = KanbanList::new();
            list_component.set_list(list);
            self.board_content.add_child(list_component);
        }
        
        // 添加"添加列表"按钮
        let add_list_btn = self.create_add_list_button();
        self.board_content.add_child(add_list_btn);
    }
    
    fn create_add_list_button(&mut self) -> KanbanList {
        KanbanList::create_add_list()
    }
    
    /// 设置卡片移动回调
    pub fn set_on_card_move<F>(&mut self, callback: F)
    where
        F: FnMut(CardMoveOperation) + 'static,
    {
        for child in self.board_content.children_mut() {
            if let Some(list) = child.downcast_mut::<KanbanList>() {
                list.set_on_card_move(callback.clone());
            }
        }
    }
}
```

### 3.3 看板头部 (BoardHeader)

```rust
// src/kanban_ui/board/board_header.rs

live_design! {
    board_header = {{BoardHeader}} {
        flow: Right,
        width: Fill,
        height: 48,
        align: {x: 0.0, y: 0.5},
        spacing: 8,
        padding: 12,
        
        /// 看板标题
        title_label = {
            draw_text: {
                text_style: {
                    font_size: 16,
                    font_weight: Bold,
                },
                color: #172B4D,
            }
        }
        
        /// 面包屑导航
        breadcrumb = {
            draw_text: {
                text_style: {
                    font_size: 14,
                },
                color: #5E6C84,
            }
        }
        
        /// 成员头像列表
        member_avatars = {
            flow: Right,
            spacing: -8,
        }
        
        /// 菜单按钮
        menu_button = {
            width: 32,
            height: 32,
            border_radius: 3,
        }
        
        /// 筛选按钮
        filter_button = {
            width: 32,
            height: 32,
            border_radius: 3,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct BoardHeader {
    /// 看板标题
    #[live]
    title_label: Label,
    
    /// 面包屑
    #[live]
    breadcrumb: Label,
    
    /// 成员头像
    #[live]
    member_avatars: FlowBox,
    
    /// 菜单按钮
    #[live]
    menu_button: Button,
    
    /// 筛选按钮
    #[live]
    filter_button: Button,
    
    /// 看板数据
    board: Option<KanbanBoard>,
}

impl BoardHeader {
    pub fn set_board_name(&mut self, name: &str) {
        self.title_label.set_text(name);
    }
    
    pub fn set_breadcrumb(&mut self, items: &[&str]) {
        let path = items.join(" / ");
        self.breadcrumb.set_text(&path);
    }
    
    pub fn set_members(&mut self, members: &[BoardMember]) {
        self.member_avatars = FlowBox {
            width: Fit,
            height: 24,
            spacing: -8,
            ..Default::default()
        };
        
        for member in members.iter().take(5) {
            let avatar = Avatar::new();
            avatar.set_user(member);
            self.member_avatars.add_child(avatar);
        }
        
        // 如果成员超过5个，显示更多
        if members.len() > 5 {
            let more_label = Label::with_text(&format!("+{}", members.len() - 5));
            self.member_avatars.add_child(more_label);
        }
    }
}
```

### 3.4 看板工具栏 (BoardToolbar)

```rust
// src/kanban_ui/board/board_toolbar.rs

live_design! {
    board_toolbar = {{BoardToolbar}} {
        flow: Right,
        width: Fill,
        height: 40,
        align: {x: 0.0, y: 0.5},
        spacing: 4,
        padding: 8,
        
        /// 筛选按钮
        filter_button = {
            width: 32,
            height: 32,
            border_radius: 3,
        }
        
        /// 排序按钮
        sort_button = {
            width: 32,
            height: 32,
            border_radius: 3,
        }
        
        /// 搜索框
        search_input = {
            width: 180,
            height: 32,
            border_radius: 3,
        }
        
        /// 分隔符
        divider = {
            width: 1,
            height: 20,
            background_color: #DFE1E6,
        }
        
        /// 视图切换
        view_toggle = {
            width: Fit,
            height: 32,
            flow: Right,
            spacing: 0,
        }
        
        /// 操作按钮组
        action_buttons = {
            flow: Right,
            spacing: 4,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct BoardToolbar {
    /// 筛选按钮
    #[live]
    filter_button: Button,
    
    /// 排序按钮
    #[live]
    sort_button: Button,
    
    /// 搜索框
    #[live]
    search_input: SearchInput,
    
    /// 视图切换
    #[live]
    view_toggle: ViewToggle,
    
    /// 操作按钮组
    #[live]
    action_buttons: FlowBox,
    
    /// 筛选状态
    #[live]
    filter_state: KanbanFilterState,
}

impl BoardToolbar {
    pub fn set_on_filter<F>(&mut self, callback: F)
    where
        F: FnMut(KanbanFilterState) + 'static,
    {
        self.filter_button.set_on_click(move || {
            // 显示筛选菜单
        });
    }
    
    pub fn set_on_search<F>(&mut self, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        self.search_input.set_on_change(callback);
    }
    
    pub fn set_on_view_change<F>(&mut self, callback: F)
    where
        F: FnMut(KanbanViewMode) + 'static,
    {
        self.view_toggle.set_on_change(callback);
    }
}

/// 视图切换组件
#[derive(Debug, Clone, LiveHook, LiveRegister)]
pub struct ViewToggle {
    #[live]
    board_view: Button,
    #[live]
    list_view: Button,
    #[live]
    current_mode: KanbanViewMode,
}

impl ViewToggle {
    pub fn set_on_change<F>(&mut self, callback: F)
    where
        F: FnMut(KanbanViewMode) + 'static,
    {
        self.board_view.set_on_click(move || {
            callback(KanbanViewMode::Board);
        });
        self.list_view.set_on_click(move || {
            callback(KanbanViewMode::List);
        });
    }
}
```

---

## 4. 业务组件

### 4.1 列表组件 (KanbanList)

```rust
// src/kanban_ui/list/kanban_list.rs

live_design! {
    kanban_list = {{KanbanList}} {
        flow: Down,
        width: 272,
        min_width: 272,
        max_width: 272,
        background_color: #EBECF0,
        border_radius: 3,
        
        /// 列表头部
        header = {
            flow: Right,
            height: 32,
            align: {x: 0.0, y: 0.5},
            padding: 8,
            spacing: 4,
        }
        
        /// 卡片容器
        cards_container = {
            flow: Down,
            width: Fill,
            height: Fit,
            max_height: 800,
            padding: 4,
            spacing: 4,
        }
        
        /// 添加卡片区域
        add_card_area = {
            flow: Down,
            width: Fill,
            padding: 4,
            spacing: 4,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct KanbanList {
    /// 列表 ID
    pub id: String,
    
    /// 列表头部
    #[live]
    header: ListHeader,
    
    /// 卡片容器
    #[live]
    cards_container: ScrollContainer,
    
    /// 卡片列表
    cards: Vec<LivePtr>,
    
    /// 添加卡片区域
    #[live]
    add_card_area: AddCardArea,
    
    /// 列表数据
    list: Option<KanbanList>,
    
    /// 拖拽状态
    #[live]
    drag_drop_state: DragDropState,
    
    /// 卡片移动回调
    card_move_callback: Option<Box<dyn FnMut(CardMoveOperation)>>,
}

impl KanbanList {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            header: ListHeader::new(),
            cards_container: ScrollContainer::new(),
            cards: Vec::new(),
            add_card_area: AddCardArea::new(),
            list: None,
            drag_drop_state: DragDropState::default(),
            card_move_callback: None,
        }
    }
    
    pub fn set_list(&mut self, list: &KanbanListData) {
        self.id = list.id.clone();
        self.list = Some(list.clone());
        
        self.header.set_title(&list.name);
        self.header.set_color(list.color.as_deref());
        
        // 渲染卡片
        self.render_cards(&list.cards);
    }
    
    fn render_cards(&mut self, cards: &[KanbanCard]) {
        self.cards_container = ScrollContainer {
            flow: Down,
            width: Fill,
            height: Fit,
            max_height: 800,
            show_scroll_bars: false,
            ..Default::default()
        };
        
        for card in cards {
            let card_component = KanbanCard::new();
            card_component.set_card(card);
            self.cards_container.add_child(card_component);
        }
        
        self.cards = self.cards_container.children_mut()
            .map(|c| c.live_ptr())
            .collect();
    }
    
    pub fn set_on_card_move<F>(&mut self, callback: F)
    where
        F: FnMut(CardMoveOperation) + 'static,
    {
        self.card_move_callback = Some(Box::new(callback));
        
        // 转发给卡片组件
        for child in self.cards_container.children_mut() {
            if let Some(card) = child.downcast_mut::<KanbanCard>() {
                card.set_on_move(callback.clone());
            }
        }
    }
    
    /// 创建"添加列表"按钮
    pub fn create_add_list() -> Self {
        let mut list = Self::new();
        list.header.set_title("+ 添加列表");
        list.add_card_area.set_visible(false);
        list
    }
}

/// 列表头部
#[derive(Debug, Clone, LiveHook, LiveRegister)]
pub struct ListHeader {
    #[live]
    title: Label,
    #[live]
    menu_button: Button,
    #[live]
    card_count: Label,
}

impl ListHeader {
    pub fn new() -> Self {
        Self {
            title: Label::with_text("列表标题"),
            menu_button: Button::empty(),
            card_count: Label::with_text("0"),
        }
    }
    
    pub fn set_title(&mut self, title: &str) {
        self.title.set_text(title);
    }
    
    pub fn set_color(&mut self, color: Option<&str>) {
        if let Some(color) = color {
            self.title.set_text_color(color_from_hex(color));
        }
    }
}
```

### 4.2 卡片组件 (KanbanCard)

```rust
// src/kanban_ui/card/kanban_card.rs

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
        
        /// 封面图片 (可选)
        cover = {
            width: Fill,
            height: 0,
            visible: false,
        }
        
        /// 卡片内容
        content = {
            flow: Down,
            width: Fill,
            padding: 8,
            spacing: 4,
        }
        
        /// 标签行
        labels_row = {
            flow: Right,
            height: 0,
            visible: false,
            spacing: 4,
        }
        
        /// 标题
        title = {
            draw_text: {
                text_style: {
                    font_size: 14,
                },
                color: #172B4D,
            }
            wrap: Word,
        }
        
        /// 描述预览
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
        
        /// 徽章行
        badges_row = {
            flow: Right,
            height: 0,
            visible: false,
            spacing: 4,
        }
        
        /// 底部信息
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
    /// 卡片 ID
    pub id: String,
    
    /// 封面
    #[live]
    cover: ImageBox,
    
    /// 卡片内容
    #[live]
    content: FlowBox,
    
    /// 标签行
    #[live]
    labels_row: FlowBox,
    
    /// 标题
    #[live]
    title: Label,
    
    /// 描述预览
    #[live]
    description_preview: Label,
    
    /// 徽章行
    #[live]
    badges_row: FlowBox,
    
    /// 底部信息
    #[live]
    footer: FlowBox,
    
    /// 卡片数据
    card: Option<KanbanCard>,
    
    /// 点击回调
    click_callback: Option<Box<dyn FnMut()>>,
    
    /// 移动回调
    move_callback: Option<Box<dyn FnMut()>>,
}

impl KanbanCard {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            cover: ImageBox::empty(),
            content: FlowBox {
                flow: Down,
                width: Fill,
                padding: 8,
                spacing: 4,
                ..Default::default()
            },
            labels_row: FlowBox::new(),
            title: Label::with_text("卡片标题"),
            description_preview: Label::empty(),
            badges_row: FlowBox::new(),
            footer: FlowBox::new(),
            card: None,
            click_callback: None,
            move_callback: None,
        }
    }
    
    pub fn set_card(&mut self, card: &KanbanCardData) {
        self.id = card.id.to_string();
        self.card = Some(card.clone());
        
        // 设置标题
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
        
        // 设置底部信息
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
            let label = Label::new();
            label.set_text(label_id);
            label.set_background_color(LabelColors::BLUE);
            self.labels_row.add_child(label);
        }
        
        self.labels_row.set_visible(true);
    }
    
    fn render_badges(&mut self, card: &KanbanCard) {
        self.badges_row = FlowBox {
            flow: Right,
            height: Fit,
            spacing: 4,
            ..Default::default()
        };
        
        // 截止日期徽章
        if let Some(due_date) = &card.due_date {
            let badge = CardBadge::due_date(due_date);
            self.badges_row.add_child(badge);
        }
        
        // 附件徽章
        if card.attachment_count > 0 {
            let badge = CardBadge::attachments(card.attachment_count);
            self.badges_row.add_child(badge);
        }
        
        // 评论徽章
        if card.comment_count > 0 {
            let badge = CardBadge::comments(card.comment_count);
            self.badges_row.add_child(badge);
        }
        
        // 检查清单徽章
        if !card.checklists.is_empty() {
            let progress = calculate_checklist_progress(&card.checklists);
            let badge = CardBadge::checklist(progress);
            self.badges_row.add_child(badge);
        }
        
        self.badges_row.set_visible(true);
    }
    
    fn render_footer(&mut self, card: &KanbanCard) {
        self.footer = FlowBox {
            flow: Right,
            height: 24,
            align: {x: 1.0, y: 0.5},
            spacing: 4,
            ..Default::default()
        };
        
        // 成员头像
        if !card.member_ids.is_empty() {
            let members = card.member_ids.iter().take(3).collect::<Vec<_>>();
            let avatars = CardMembers::new(&members);
            self.footer.add_child(avatars);
        }
        
        // 加星标
        if card.is_starred {
            let star = CardBadge::starred();
            self.footer.add_child(star);
        }
    }
    
    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.click_callback = Some(Box::new(callback));
    }
    
    pub fn set_on_move<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.move_callback = Some(Box::new(callback));
    }
}

/// 计算检查清单进度
fn calculate_checklist_progress(checklists: &[CardChecklist]) -> (u32, u32) {
    let total = checklists.iter()
        .flat_map(|c| c.items.iter())
        .count() as u32;
    
    let completed = checklists.iter()
        .flat_map(|c| c.items.iter())
        .filter(|i| i.is_completed)
        .count() as u32;
    
    (completed, total)
}
```

---

## 5. 弹窗组件

### 5.1 卡片详情弹窗 (CardModal)

```rust
// src/kanban_ui/modal/card_modal.rs

live_design! {
    card_modal = {{CardModal}} {
        flow: Down,
        width: 768,
        max_width: 900,
        height: 600,
        max_height: 800,
        background_color: #F4F5F7,
        border_radius: 4,
        box_shadow: {
            color: #00000029,
            x: 0,
            y: 4,
            blur: 12,
            spread: 0,
        },
        
        /// 头部
        header = {
            flow: Right,
            width: Fill,
            height: 48,
            padding: 12,
            spacing: 8,
        }
        
        /// 内容区
        content = {
            flow: Right,
            width: Fill,
            height: Fill,
            padding: 16,
            spacing: 16,
        }
        
        /// 主内容区
        main_content = {
            flow: Down,
            width: Fill,
            height: Fill,
            spacing: 16,
        }
        
        /// 侧边栏
        sidebar = {
            flow: Down,
            width: 200,
            height: Fit,
            spacing: 16,
        }
    }
}

#[derive(Debug, Clone, LiveHook, LiveRegister)]
#[live_register_view(panic_recovery)]
pub struct CardModal {
    /// 弹窗背景
    #[live]
    overlay: ModalOverlay,
    
    /// 弹窗容器
    #[live]
    container: FlowBox,
    
    /// 头部
    #[live]
    header: CardModalHeader,
    
    /// 内容区
    #[live]
    content: FlowBox,
    
    /// 主内容区
    #[live]
    main_content: FlowBox,
    
    /// 侧边栏
    #[live]
    sidebar: FlowBox,
    
    /// 卡片数据
    card: Option<KanbanCard>,
    
    /// 编辑状态
    edit_mode: CardEditMode,
}

impl CardModal {
    pub fn new() -> Self {
        Self {
            overlay: ModalOverlay::new(),
            container: FlowBox::new(),
            header: CardModalHeader::new(),
            content: FlowBox::new(),
            main_content: FlowBox::new(),
            sidebar: FlowBox::new(),
            card: None,
            edit_mode: CardEditMode::View,
        }
    }
    
    pub fn set_card(&mut self, card: &KanbanCard) {
        self.card = Some(card.clone());
        
        self.header.set_title(&card.title);
        self.header.set_list_name("待办"); // 从列表获取
        
        // 渲染主内容区
        self.render_main_content(card);
        
        // 渲染侧边栏
        self.render_sidebar(card);
    }
    
    fn render_main_content(&mut self, card: &KanbanCard) {
        self.main_content = FlowBox {
            flow: Down,
            width: Fill,
            height: Fill,
            spacing: 16,
            ..Default::default()
        };
        
        // 描述编辑器
        let desc_editor = CardDescriptionEditor::new();
        desc_editor.set_description(card.description.as_deref());
        self.main_content.add_child(desc_editor);
        
        // 附件列表
        if !card.attachments.is_empty() {
            let attachments_editor = CardAttachmentsEditor::new();
            attachments_editor.set_attachments(&card.attachments);
            self.main_content.add_child(attachments_editor);
        }
        
        // 检查清单
        if !card.checklists.is_empty() {
            let checklists_editor = CardChecklistsEditor::new();
            checklists_editor.set_checklists(&card.checklists);
            self.main_content.add_child(checklists_editor);
        }
        
        // 评论
        let comments = CardComments::new();
        comments.set_comments(&card.activities);
        self.main_content.add_child(comments);
    }
    
    fn render_sidebar(&mut self, card: &KanbanCard) {
        self.sidebar = FlowBox {
            flow: Down,
            width: 200,
            height: Fit,
            spacing: 16,
            ..Default::default()
        };
        
        // 成员编辑
        let members_editor = CardMembersEditor::new();
        members_editor.set_members(&card.member_ids);
        self.sidebar.add_child(members_editor);
        
        // 标签编辑
        let labels_editor = CardLabelsEditor::new();
        labels_editor.set_labels(&card.label_ids);
        self.sidebar.add_child(labels_editor);
        
        // 截止日期编辑
        let due_date_editor = CardDueDateEditor::new();
        due_date_editor.set_due_date(card.due_date.as_ref());
        self.sidebar.add_child(due_date_editor);
        
        // 封面编辑
        let cover_editor = CardCoverEditor::new();
        cover_editor.set_cover(card.cover.as_ref());
        self.sidebar.add_child(cover_editor);
    }
    
    pub fn show(&mut self) {
        self.overlay.set_visible(true);
        self.container.set_visible(true);
    }
    
    pub fn hide(&mut self) {
        self.overlay.set_visible(false);
        self.container.set_visible(false);
    }
}

/// 卡片详情头部
#[derive(Debug, Clone, LiveHook, LiveRegister)]
pub struct CardModalHeader {
    #[live]
    icon: Label,
    #[live]
    title: Label,
    #[live]
    list_name: Label,
    #[live]
    close_button: Button,
}

impl CardModalHeader {
    pub fn new() -> Self {
        Self {
            icon: Label::with_text("📋"),
            title: Label::with_text("卡片标题"),
            list_name: Label::with_text("在 待办 中"),
            close_button: Button::empty(),
        }
    }
    
    pub fn set_title(&mut self, title: &str) {
        self.title.set_text(title);
    }
    
    pub fn set_list_name(&mut self, name: &str) {
        self.list_name.set_text(&format!("在 {} 中", name));
    }
}
```

---

## 6. 样式设计

### 6.1 主题系统

```rust
// src/kanban_ui/styles/theme.rs

/// 看板应用主题
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KanbanTheme {
    /// 颜色
    pub colors: KanbanColors,
    
    /// 字体
    pub typography: KanbanTypography,
    
    /// 阴影
    pub shadows: KanbanShadows,
    
    /// 圆角
    pub border_radius: BorderRadius,
    
    /// 间距
    pub spacing: Spacing,
    
    /// 过渡
    pub transitions: Transitions,
}

impl Default for KanbanTheme {
    fn default() -> Self {
        Self {
            colors: KanbanColors::default(),
            typography: KanbanTypography::default(),
            shadows: KanbanShadows::default(),
            border_radius: BorderRadius::default(),
            spacing: Spacing::default(),
            transitions: Transitions::default(),
        }
    }
}

/// 圆角规范
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BorderRadius {
    pub small: f64,     // 3px
    pub medium: f64,    // 4px
    pub large: f64,     // 6px
    pub xlarge: f64,    // 8px
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            small: 3.0,
            medium: 4.0,
            large: 6.0,
            xlarge: 8.0,
        }
    }
}

/// 间距规范
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spacing {
    pub xsmall: f64,    // 4px
    pub small: f64,     // 8px
    pub medium: f64,    // 12px
    pub large: f64,     // 16px
    pub xlarge: f64,    // 24px
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xsmall: 4.0,
            small: 8.0,
            medium: 12.0,
            large: 16.0,
            xlarge: 24.0,
        }
    }
}

/// 过渡动画
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transitions {
    pub fast: f64,      // 100ms
    pub normal: f64,    // 200ms
    pub slow: f64,      // 300ms
}

impl Default for Transitions {
    fn default() -> Self {
        Self {
            fast: 0.1,
            normal: 0.2,
            slow: 0.3,
        }
    }
}
```

### 6.2 组件样式

```rust
// src/kanban_ui/styles/components.rs

/// 卡片样式
pub fn card_styles() -> Styles {
    Styles::new()
        .style(
            "kanban_card",
            Style {
                width: Stretch(1.0),
                min_height: pixel(40.0),
                background_color: colors.surface,
                border_radius: pixels(border_radius.small),
                box_shadow: shadows.card,
                cursor: Cursor::Pointer,
                ..Default::default()
            },
        )
        .style(
            "kanban_card:hover",
            Style {
                background_color: colors.surface.darken(0.02),
                ..Default::default()
            },
        )
        .style(
            "kanban_card:active",
            Style {
                background_color: colors.surface.darken(0.04),
                ..Default::default()
            },
        )
}

/// 列表样式
pub fn list_styles() -> Styles {
    Styles::new()
        .style(
            "kanban_list",
            Style {
                width: pixel(272.0),
                min_width: pixel(272.0),
                max_width: pixel(272.0),
                background_color: colors.list_background,
                border_radius: pixels(border_radius.medium),
                ..Default::default()
            },
        )
        .style(
            "kanban_list:empty",
            Style {
                min_height: pixel(100.0),
                ..Default::default()
            },
        )
}

/// 标签样式
pub fn label_styles() -> Styles {
    Styles::new()
        .style(
            "label",
            Style {
                height: pixel(8.0),
                border_radius: pixels(border_radius.small / 2.0),
                ..Default::default()
            },
        )
        .style(
            "label.green",
            Style {
                background_color: colors.green,
                ..Default::default()
            },
        )
        .style(
            "label.yellow",
            Style {
                background_color: colors.yellow,
                ..Default::default()
            },
        )
        // ... 其他颜色
}
```

---

## 7. 响应式设计

### 7.1 断点定义

```rust
// src/kanban_ui/responsive.rs

/// 响应式断点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Breakpoint {
    /// 移动端 (< 768px)
    Mobile,
    /// 平板 (768px - 1024px)
    Tablet,
    /// 桌面端 (1024px - 1440px)
    Desktop,
    /// 大屏 (> 1440px)
    Wide,
}

impl Breakpoint {
    /// 根据宽度获取断点
    pub fn from_width(width: f64) -> Self {
        if width < 768.0 {
            Breakpoint::Mobile
        } else if width < 1024.0 {
            Breakpoint::Tablet
        } else if width < 1440.0 {
            Breakpoint::Desktop
        } else {
            Breakpoint::Wide
        }
    }
}

/// 响应式配置
#[derive(Debug, Clone)]
pub struct ResponsiveConfig {
    /// 当前断点
    pub breakpoint: Breakpoint,
    
    /// 列表宽度
    pub list_width: f64,
    
    /// 侧边栏宽度
    pub sidebar_width: f64,
    
    /// 是否显示侧边栏
    pub show_sidebar: bool,
    
    /// 卡片布局
    pub card_layout: CardLayout,
    
    /// 工具栏可见性
    pub show_toolbar: bool,
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self {
            breakpoint: Breakpoint::Desktop,
            list_width: 272.0,
            sidebar_width: 272.0,
            show_sidebar: true,
            card_layout: CardLayout::Vertical,
            show_toolbar: true,
        }
    }
}

impl ResponsiveConfig {
    /// 根据窗口宽度更新配置
    pub fn update(&mut self, window_width: f64) {
        self.breakpoint = Breakpoint::from_width(window_width);
        
        match self.breakpoint {
            Breakpoint::Mobile => {
                self.list_width = window_width - 16.0;
                self.sidebar_width = 0.0;
                self.show_sidebar = false;
                self.card_layout = CardLayout::Vertical;
                self.show_toolbar = false;
            }
            Breakpoint::Tablet => {
                self.list_width = 280.0;
                self.sidebar_width = 0.0;
                self.show_sidebar = false;
                self.card_layout = CardLayout::Vertical;
                self.show_toolbar = true;
            }
            Breakpoint::Desktop => {
                self.list_width = 272.0;
                self.sidebar_width = 272.0;
                self.show_sidebar = true;
                self.card_layout = CardLayout::Vertical;
                self.show_toolbar = true;
            }
            Breakpoint::Wide => {
                self.list_width = 272.0;
                self.sidebar_width = 300.0;
                self.show_sidebar = true;
                self.card_layout = CardLayout::Vertical;
                self.show_toolbar = true;
            }
        }
    }
}

/// 卡片布局模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardLayout {
    Vertical,    // 垂直列表
    Horizontal,  // 水平卡片
    Grid,        // 网格布局
}
```

### 7.2 响应式组件

```rust
// src/kanban_ui/responsive.rs

/// 响应式看板视图
#[derive(Debug, Clone, LiveHook, LiveRegister)]
pub struct ResponsiveKanbanView {
    /// 响应式配置
    #[live]
    config: ResponsiveConfig,
    
    /// 桌面端视图
    #[live]
    desktop_view: KanbanDesktopView,
    
    /// 移动端视图
    #[live]
    mobile_view: KanbanMobileView,
}

impl ResponsiveKanbanView {
    pub fn update_layout(&mut self, window_width: f64) {
        self.config.update(window_width);
        
        match self.config.breakpoint {
            Breakpoint::Mobile | Breakpoint::Tablet => {
                self.desktop_view.set_visible(false);
                self.mobile_view.set_visible(true);
            }
            Breakpoint::Desktop | Breakpoint::Wide => {
                self.desktop_view.set_visible(true);
                self.mobile_view.set_visible(false);
            }
        }
    }
}

/// 移动端看板视图
#[derive(Debug, Clone, LiveHook, LiveRegister)]
pub struct KanbanMobileView {
    #[live]
    stack: StackNavigation,
    #[live]
    board_list: BoardListScreen,
    #[live]
    board_detail: BoardDetailScreen,
}

impl KanbanMobileView {
    pub fn new() -> Self {
        Self {
            stack: StackNavigation::new(),
            board_list: BoardListScreen::new(),
            board_detail: BoardDetailScreen::new(),
        }
    }
}
```

---

## 8. 组件交互

### 8.1 事件流

```
┌─────────────────────────────────────────────────────────────────┐
│                        组件事件流                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  用户操作                                                        │
│      │                                                          │
│      ▼                                                          │
│  UI 组件捕获事件                                                 │
│      │                                                          │
│      ├─► 本地状态更新                                            │
│      │                                                          │
│      ├─► 回调函数通知                                            │
│      │                                                          │
│      ▼                                                          │
│  业务逻辑层 (KanbanService)                                      │
│      │                                                          │
│      ├─► 数据验证                                                │
│      │                                                          │
│      ├─► 乐观更新                                                │
│      │                                                          │
│      ▼                                                          │
│  API 层 (Repository)                                            │
│      │                                                          │
│      ├─► 构建请求                                                │
│      │                                                          │
│      ▼                                                          │
│  MatrixRequest                                                  │
│      │                                                          │
│      ▼                                                          │
│  Worker 线程处理                                                 │
│      │                                                          │
│      ├─► 调用 Matrix SDK                                        │
│      │                                                          │
│      ▼                                                          │
│  响应处理                                                        │
│      │                                                          │
│      ├─► 更新 UI                                                 │
│      │                                                          │
│      └─► 错误处理 (如需要回滚)                                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 状态同步

```rust
// 状态同步示例

impl KanbanList {
    /// 同步卡片状态
    pub fn sync_card_state(&mut self, card_id: &str, updates: CardUpdates) {
        // 1. 找到卡片
        for child in self.cards_container.children_mut() {
            if let Some(card) = child.downcast_mut::<KanbanCard>() {
                if card.id == card_id {
                    // 2. 应用更新
                    if let Some(title) = updates.title {
                        card.title.set_text(&title);
                    }
                    
                    if let Some(desc) = updates.description {
                        card.description_preview.set_text(&desc);
                    }
                    
                    // 3. 触发重新渲染
                    card.set_dirty(true);
                    break;
                }
            }
        }
    }
}
```

---

## 附录

### A. 组件清单

| 组件名称 | 文件路径 | 说明 |
|---------|----------|------|
| KanbanWorkspace | workspace/kanban_workspace.rs | 工作区容器 |
| KanbanBoardView | board/board_view.rs | 看板主视图 |
| BoardHeader | board/board_header.rs | 看板头部 |
| BoardToolbar | board/board_toolbar.rs | 看板工具栏 |
| KanbanList | list/kanban_list.rs | 列表组件 |
| KanbanCard | card/kanban_card.rs | 卡片组件 |
| CardModal | modal/card_modal.rs | 卡片详情弹窗 |

### B. 性能优化

- **虚拟列表**: 大量卡片时使用虚拟列表
- **懒加载**: 卡片按需加载
- **增量更新**: 只更新变化的组件
- **缓存优化**: 缓存组件实例

### C. 可访问性

- **键盘导航**: 支持 Tab 键导航
- **焦点管理**: 合理的焦点顺序
- **ARIA 标签**: 为屏幕阅读器提供标签
- **颜色对比**: 满足 WCAG 2.1 AA 标准

---

> 文档版本: 1.0
> 最后更新: 2026-01-14
