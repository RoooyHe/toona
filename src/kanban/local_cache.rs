//! 本地缓存模块 - 用于应对服务器不持久化状态事件的bug
//!
//! 这个模块提供了一个本地缓存，用于存储 Space 和 Card 之间的关系。
//! 当服务器不能正确持久化 m.kanban.cards 或 m.space.child 事件时，
//! 我们使用这个缓存来确保卡片关系不会丢失。
//!
//! 缓存会自动保存到磁盘，重启后可以恢复。

use matrix_sdk::ruma::OwnedRoomId;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

/// 缓存数据结构（可序列化）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CacheData {
    /// Space ID -> Card IDs
    space_cards: HashMap<String, HashSet<String>>,
}

/// 全局缓存：Space ID -> Card IDs
static SPACE_CARDS_CACHE: Mutex<Option<CacheData>> = Mutex::new(None);

/// 获取缓存文件路径
fn get_cache_file_path() -> std::path::PathBuf {
    let app_data_dir = crate::app_data_dir();
    app_data_dir.join("kanban_cache.json")
}

/// 从磁盘加载缓存
fn load_cache_from_disk() -> CacheData {
    let cache_path = get_cache_file_path();

    if cache_path.exists() {
        match std::fs::read_to_string(&cache_path) {
            Ok(json_str) => match serde_json::from_str::<CacheData>(&json_str) {
                Ok(data) => {
                    makepad_widgets::log!(
                        "💾 Loaded kanban cache from disk: {} spaces",
                        data.space_cards.len()
                    );
                    return data;
                }
                Err(e) => {
                    makepad_widgets::error!("Failed to parse kanban cache: {:?}", e);
                }
            },
            Err(e) => {
                makepad_widgets::error!("Failed to read kanban cache file: {:?}", e);
            }
        }
    }

    CacheData::default()
}

/// 保存缓存到磁盘
fn save_cache_to_disk(data: &CacheData) {
    let cache_path = get_cache_file_path();

    match serde_json::to_string_pretty(data) {
        Ok(json_str) => match std::fs::write(&cache_path, json_str) {
            Ok(_) => {
                makepad_widgets::log!(
                    "💾 Saved kanban cache to disk: {} spaces",
                    data.space_cards.len()
                );
            }
            Err(e) => {
                makepad_widgets::error!("Failed to write kanban cache file: {:?}", e);
            }
        },
        Err(e) => {
            makepad_widgets::error!("Failed to serialize kanban cache: {:?}", e);
        }
    }
}

/// 初始化缓存（从磁盘加载）
fn ensure_cache_initialized() {
    let mut cache = SPACE_CARDS_CACHE.lock().unwrap();
    if cache.is_none() {
        *cache = Some(load_cache_from_disk());
    }
}

/// 添加卡片到 Space 的本地缓存
pub fn add_card_to_space_cache(space_id: OwnedRoomId, card_id: OwnedRoomId) {
    ensure_cache_initialized();
    let mut cache = SPACE_CARDS_CACHE.lock().unwrap();
    let cache_data = cache.as_mut().unwrap();

    let space_key = space_id.to_string();
    let card_key = card_id.to_string();

    cache_data
        .space_cards
        .entry(space_key)
        .or_insert_with(HashSet::new)
        .insert(card_key);

    // 立即保存到磁盘
    save_cache_to_disk(cache_data);
}

/// 从 Space 的本地缓存获取所有卡片 ID
pub fn get_cards_from_cache(space_id: &OwnedRoomId) -> Vec<OwnedRoomId> {
    ensure_cache_initialized();
    let cache = SPACE_CARDS_CACHE.lock().unwrap();
    let cache_data = cache.as_ref().unwrap();

    let space_key = space_id.to_string();

    cache_data
        .space_cards
        .get(&space_key)
        .map(|set| {
            set.iter()
                .filter_map(|s| OwnedRoomId::try_from(s.as_str()).ok())
                .collect()
        })
        .unwrap_or_default()
}

/// 从 Space 的本地缓存移除卡片
pub fn remove_card_from_cache(space_id: &OwnedRoomId, card_id: &OwnedRoomId) {
    ensure_cache_initialized();
    let mut cache = SPACE_CARDS_CACHE.lock().unwrap();
    let cache_data = cache.as_mut().unwrap();

    let space_key = space_id.to_string();
    let card_key = card_id.to_string();

    if let Some(cards) = cache_data.space_cards.get_mut(&space_key) {
        cards.remove(&card_key);
    }

    // 保存到磁盘
    save_cache_to_disk(cache_data);
}

/// 清空所有缓存
pub fn clear_cache() {
    let mut cache = SPACE_CARDS_CACHE.lock().unwrap();
    let cache_data = CacheData::default();
    *cache = Some(cache_data.clone());

    // 保存到磁盘
    save_cache_to_disk(&cache_data);
}
