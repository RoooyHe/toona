use std::collections::HashMap;
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use crate::kanban::data::models::*;

/// 看板应用状态
#[derive(Debug, Clone, Default)]
pub struct KanbanAppState {
    /// 当前用户 ID
    pub current_user_id: Option<OwnedUserId>,

    /// 当前选中的看板
    pub current_board_id: Option<OwnedRoomId>,

    /// 看板数据缓存
    pub boards: HashMap<OwnedRoomId, KanbanBoard>,

    /// 列表数据缓存
    pub lists: HashMap<String, KanbanList>,

    /// 卡片数据缓存
    pub cards: HashMap<String, KanbanCard>,

    /// 加载状态
    pub loading: bool,

    /// 错误信息
    pub error: Option<String>,

    /// 过滤状态
    pub filter_state: Option<KanbanFilterState>,

    /// 排序状态
    pub sort_state: Option<KanbanSortState>,
}

impl KanbanAppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取当前看板
    pub fn current_board(&self) -> Option<&KanbanBoard> {
        self.current_board_id
            .as_ref()
            .and_then(|id| self.boards.get(id))
    }

    /// 获取当前看板的列表
    pub fn current_board_lists(&self) -> Vec<&KanbanList> {
        if let Some(board) = self.current_board() {
            board
                .list_ids
                .iter()
                .filter_map(|list_id| self.lists.get(list_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 获取列表中的卡片
    pub fn list_cards(&self, list_id: &str) -> Vec<&KanbanCard> {
        if let Some(list) = self.lists.get(list_id) {
            list.card_ids
                .iter()
                .filter_map(|card_id| self.cards.get(card_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 设置加载状态
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    /// 设置错误信息
    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    /// 添加或更新看板
    pub fn upsert_board(&mut self, board: KanbanBoard) {
        self.boards.insert(board.id.clone(), board);
    }

    /// 添加或更新列表
    pub fn upsert_list(&mut self, list: KanbanList) {
        self.lists.insert(list.id.clone(), list);
    }

    /// 添加或更新卡片
    pub fn upsert_card(&mut self, card: KanbanCard) {
        self.cards.insert(card.id.clone(), card);
    }

    /// 删除看板
    pub fn remove_board(&mut self, board_id: &OwnedRoomId) {
        self.boards.remove(board_id);
        if self.current_board_id.as_ref() == Some(board_id) {
            self.current_board_id = None;
        }
    }

    /// 删除列表
    pub fn remove_list(&mut self, list_id: &str) {
        self.lists.remove(list_id);
    }

    /// 删除卡片
    pub fn remove_card(&mut self, card_id: &str) {
        self.cards.remove(card_id);
    }
}
