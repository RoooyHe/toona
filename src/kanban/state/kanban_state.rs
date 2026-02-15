use std::collections::HashMap;
use matrix_sdk::ruma::OwnedRoomId;

/// 简化的看板列表（对应 Matrix Space）
#[derive(Debug, Clone)]
pub struct KanbanList {
    /// 列表 ID（Space ID）
    pub id: OwnedRoomId,
    
    /// 列表名称
    pub name: String,
    
    /// 卡片 ID 列表
    pub card_ids: Vec<OwnedRoomId>,
    
    /// 排序位置
    pub position: f64,
}

/// 简化的看板卡片（对应 Matrix Room）
#[derive(Debug, Clone)]
pub struct KanbanCard {
    /// 卡片 ID（Room ID）
    pub id: OwnedRoomId,
    
    /// 卡片标题
    pub title: String,
    
    /// 卡片描述
    pub description: Option<String>,
    
    /// 所属列表 ID（Space ID）
    pub space_id: OwnedRoomId,
    
    /// 排序位置
    pub position: f64,
}

/// 看板应用状态
#[derive(Debug, Clone, Default)]
pub struct KanbanAppState {
    /// 列表数据缓存（Space ID -> KanbanList）
    pub lists: HashMap<OwnedRoomId, KanbanList>,

    /// 卡片数据缓存（Room ID -> KanbanCard）
    pub cards: HashMap<OwnedRoomId, KanbanCard>,

    /// 加载状态
    pub loading: bool,

    /// 错误信息
    pub error: Option<String>,
}

impl KanbanAppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取所有列表（按位置排序）
    pub fn all_lists(&self) -> Vec<&KanbanList> {
        let mut lists: Vec<&KanbanList> = self.lists.values().collect();
        lists.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        lists
    }

    /// 获取列表中的卡片
    pub fn list_cards(&self, space_id: &OwnedRoomId) -> Vec<&KanbanCard> {
        if let Some(list) = self.lists.get(space_id) {
            list.card_ids
                .iter()
                .filter_map(|card_id| self.cards.get(card_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 添加或更新列表
    pub fn upsert_list(&mut self, list: KanbanList) {
        self.lists.insert(list.id.clone(), list);
    }

    /// 添加或更新卡片
    pub fn upsert_card(&mut self, card: KanbanCard) {
        self.cards.insert(card.id.clone(), card);
    }

    /// 设置加载状态
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    /// 设置错误信息
    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }
}
