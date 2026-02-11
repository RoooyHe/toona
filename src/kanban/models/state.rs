use super::dto::{SpaceDto, CardDetailDto, TagDto, TodoDto, ActiveDto};
use makepad_widgets::*;
use std::sync::mpsc::Receiver;

// 应用状态 - 从 app.rs 移动过来
pub struct State {
    pub spaces_data: Vec<SpaceDto>,
    pub space_signal: SignalToUI,
    pub space_rx: Option<Receiver<Vec<SpaceDto>>>,
    pub create_space_signal: SignalToUI,
    pub create_space_rx: Option<Receiver<bool>>,

    // 卡片 CRUD 相关
    pub card_signal: SignalToUI,
    pub card_rx: Option<Receiver<bool>>,
    pub current_space_id: Option<i64>,
    pub current_card_id: Option<i64>,
    pub card_modal_visible: bool,
    pub card_title: String,
    pub card_description: String,
    pub is_editing_card: bool,

    // 按钮点击状态
    pub pending_add_card_space_id: Option<i64>,
    pub pending_edit_card_id: Option<i64>,
    pub pending_delete_card_id: Option<i64>,
    pub pending_detail_card_id: Option<i64>,  // 新增：待查看详情的卡片ID

    // 卡片详情模态框
    pub card_detail_modal_visible: bool,
    pub card_detail_data: Option<CardDetailDto>,
    pub card_detail_signal: SignalToUI,
    pub card_detail_rx: Option<Receiver<CardDetailDto>>,

    // 内联编辑状态
    pub editing_space_id: Option<i64>,
    pub editing_card_id: Option<i64>,
    pub space_update_signal: SignalToUI,
    pub space_update_rx: Option<Receiver<bool>>,
    pub card_update_signal: SignalToUI,
    pub card_update_rx: Option<Receiver<bool>>,

    // 待处理的更新
    pub pending_space_update: Option<(i64, String)>,
    pub pending_card_update: Option<(i64, String)>,

    // 新卡片输入框状态
    pub new_card_inputs: std::collections::HashMap<i64, String>, // space_id -> input_text
    pub pending_create_card: Option<(i64, String)>,              // space_id, title

    // 卡片原始文本缓存（用于对比是否有变化）
    pub card_original_texts: std::collections::HashMap<i64, String>, // card_id -> original_text

    // 标签管理
    pub all_tags: Vec<TagDto>,
    pub tags_signal: SignalToUI,
    pub tags_rx: Option<Receiver<Vec<TagDto>>>,
    pub selected_tag_id: Option<i64>,
    pub pending_add_tag_to_card: Option<(i64, i64)>, // (card_id, tag_id)
    pub card_tags_update_signal: SignalToUI,
    pub card_tags_update_rx: Option<Receiver<bool>>,
    
    // 新增标签
    pub new_tag_input: String,
    pub show_new_tag_input: bool,
    pub create_tag_signal: SignalToUI,
    pub create_tag_rx: Option<Receiver<bool>>,
    pub pending_create_tag: Option<String>,

    // Todo管理 - 使用 Vec 支持 PortalList
    pub current_todos: Vec<TodoDto>,
    pub create_todo_signal: SignalToUI,
    pub create_todo_rx: Option<Receiver<bool>>,
    pub update_todo_signal: SignalToUI,
    pub update_todo_rx: Option<Receiver<bool>>,
    pub delete_todo_signal: SignalToUI,
    pub delete_todo_rx: Option<Receiver<bool>>,
    pub new_todo_input: String,
    pub show_new_todo_input: bool,
    pub pending_create_todo: Option<String>,
    pub pending_toggle_todo: Option<(i64, bool)>, // (todo_id, completed)
    pub pending_delete_todo: Option<i64>,

    // Active管理 - 使用 Vec 支持 PortalList
    pub current_actives: Vec<ActiveDto>,
    pub create_active_signal: SignalToUI,
    pub create_active_rx: Option<Receiver<bool>>,
    pub delete_active_signal: SignalToUI,
    pub delete_active_rx: Option<Receiver<bool>>,
    pub new_active_input: String,
    pub show_new_active_input: bool,
    pub pending_create_active: Option<String>,
    pub pending_delete_active: Option<i64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            spaces_data: Vec::new(),
            space_signal: SignalToUI::default(),
            space_rx: None,
            create_space_signal: SignalToUI::default(),
            create_space_rx: None,

            // 卡片 CRUD 相关
            card_signal: SignalToUI::default(),
            card_rx: None,
            current_space_id: None,
            current_card_id: None,
            card_modal_visible: false,
            card_title: String::new(),
            card_description: String::new(),
            is_editing_card: false,

            // 按钮点击状态
            pending_add_card_space_id: None,
            pending_edit_card_id: None,
            pending_delete_card_id: None,
            pending_detail_card_id: None,

            // 卡片详情模态框
            card_detail_modal_visible: false,
            card_detail_data: None,
            card_detail_signal: SignalToUI::default(),
            card_detail_rx: None,

            // 内联编辑状态
            editing_space_id: None,
            editing_card_id: None,
            space_update_signal: SignalToUI::default(),
            space_update_rx: None,
            card_update_signal: SignalToUI::default(),
            card_update_rx: None,

            // 待处理的更新
            pending_space_update: None,
            pending_card_update: None,

            // 新卡片输入框状态
            new_card_inputs: std::collections::HashMap::new(),
            pending_create_card: None,

            // 卡片原始文本缓存
            card_original_texts: std::collections::HashMap::new(),

            // 标签管理
            all_tags: Vec::new(),
            tags_signal: SignalToUI::default(),
            tags_rx: None,
            selected_tag_id: None,
            pending_add_tag_to_card: None,
            card_tags_update_signal: SignalToUI::default(),
            card_tags_update_rx: None,
            
            // 新增标签
            new_tag_input: String::new(),
            show_new_tag_input: false,
            create_tag_signal: SignalToUI::default(),
            create_tag_rx: None,
            pending_create_tag: None,

            // Todo管理
            current_todos: Vec::new(),
            create_todo_signal: SignalToUI::default(),
            create_todo_rx: None,
            update_todo_signal: SignalToUI::default(),
            update_todo_rx: None,
            delete_todo_signal: SignalToUI::default(),
            delete_todo_rx: None,
            new_todo_input: String::new(),
            show_new_todo_input: false,
            pending_create_todo: None,
            pending_toggle_todo: None,
            pending_delete_todo: None,

            // Active管理
            current_actives: Vec::new(),
            create_active_signal: SignalToUI::default(),
            create_active_rx: None,
            delete_active_signal: SignalToUI::default(),
            delete_active_rx: None,
            new_active_input: String::new(),
            show_new_active_input: false,
            pending_create_active: None,
            pending_delete_active: None,
        }
    }
}
