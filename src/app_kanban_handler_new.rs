// 新的简化 Kanban action 处理器
// 这个文件包含重构后的 handle_kanban_action 方法

use crate::{
    kanban::{KanbanActions, KanbanAppState, KanbanCard, KanbanList},
    sliding_sync::{get_client, submit_async_request, MatrixRequest},
};
use makepad_widgets::*;
use matrix_sdk::ruma::OwnedRoomId;

pub fn handle_kanban_action_v2(
    cx: &mut Cx,
    state: &mut KanbanAppState,
    action: KanbanActions,
    ui: &mut WidgetRef,
) {
    match action {
        KanbanActions::LoadLists => {
            // 加载所有 kanban Space（列表）
            if get_client().is_some() {
                submit_async_request(MatrixRequest::LoadKanbanLists);
                state.loading = true;
            }
        }

        KanbanActions::ListLoaded(list) => {
            // 列表已加载
            log!("ListLoaded: space_id='{}', name='{}'", list.id, list.name);
            state.upsert_list(list);
            ui.redraw(cx);
        }

        KanbanActions::CardLoaded(card) => {
            // 卡片已加载
            log!("CardLoaded: card_id='{}', title='{}'", card.id, card.title);
            
            // 添加卡片到 state
            let space_id = card.space_id.clone();
            state.upsert_card(card.clone());
            
            // 添加卡片 ID 到列表的 card_ids
            if let Some(list) = state.lists.get_mut(&space_id) {
                if !list.card_ids.contains(&card.id) {
                    list.card_ids.push(card.id);
                }
            }
            
            ui.redraw(cx);
        }

        KanbanActions::CreateList { name } => {
            // 创建新列表（Space）
            if get_client().is_some() {
                submit_async_request(MatrixRequest::CreateKanbanList { name });
                state.loading = true;
            }
        }

        KanbanActions::CreateCard { space_id, title } => {
            // 在列表中创建新卡片
            if get_client().is_some() {
                submit_async_request(MatrixRequest::CreateKanbanCard { space_id, title });
                state.loading = true;
            }
        }

        KanbanActions::MoveCard {
            card_id,
            target_space_id,
            position,
        } => {
            // 移动卡片到不同列表
            if let Some(card) = state.cards.get_mut(&card_id) {
                let old_space_id = card.space_id.clone();
                card.space_id = target_space_id.clone();
                card.position = position;
                
                // 从旧列表移除
                if let Some(old_list) = state.lists.get_mut(&old_space_id) {
                    old_list.card_ids.retain(|id| id != &card_id);
                }
                
                // 添加到新列表
                if let Some(new_list) = state.lists.get_mut(&target_space_id) {
                    if !new_list.card_ids.contains(&card_id) {
                        new_list.card_ids.push(card_id);
                    }
                }
            }
        }

        KanbanActions::UpdateCardTitle { card_id, title } => {
            // 更新卡片标题
            if let Some(card) = state.cards.get_mut(&card_id) {
                card.title = title;
            }
        }

        KanbanActions::UpdateCardDescription {
            card_id,
            description,
        } => {
            // 更新卡片描述
            if let Some(card) = state.cards.get_mut(&card_id) {
                card.description = description;
            }
        }

        KanbanActions::DeleteCard { card_id } => {
            // 删除卡片
            if let Some(card) = state.cards.remove(&card_id) {
                // 从列表中移除卡片 ID
                if let Some(list) = state.lists.get_mut(&card.space_id) {
                    list.card_ids.retain(|id| id != &card_id);
                }
            }
        }

        KanbanActions::Loading(loading) => {
            state.loading = loading;
        }

        KanbanActions::Error(message) => {
            state.error = Some(message);
            state.loading = false;
        }
    }
}
