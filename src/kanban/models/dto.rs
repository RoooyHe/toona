use serde::{Deserialize, Serialize};

// DTO 定义 - 从 space.rs 移动过来
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TagDto {
    pub id: i64,
    pub title: String,
    pub color: Option<String>,
}

// Todo 实体
#[derive(Clone, Deserialize, Debug)]
pub struct TodoDto {
    pub id: i64,
    pub title: String,
    pub completed: Option<bool>,
}

// Active 实体
#[derive(Clone, Deserialize, Debug)]
pub struct ActiveDto {
    pub id: i64,
    pub title: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
}

// 卡片详情 DTO（包含 todos 和 active）
#[derive(Clone, Deserialize, Debug)]
pub struct CardDetailDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub tags: Vec<TagDto>,
    pub todos: Vec<TodoDto>,
    pub active: Vec<ActiveDto>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CardDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub tags: Vec<TagDto>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpaceDto {
    pub id: i64,
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub canceled: Option<bool>,
    pub sort: Option<i32>,
    pub color: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    pub cards: Vec<CardDto>,
}

// API 请求结构
#[derive(Serialize, Debug)]
pub struct CreateSpaceRequest {
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub canceled: Option<bool>,
    pub sort: Option<i32>,
    pub color: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct CreateCardRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    pub space: SpaceReference,
}

#[derive(Serialize, Debug)]
pub struct SpaceReference {
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct UpdateCardRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct UpdateSpaceRequest {
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Serialize, Debug)]
pub struct UpdateCardTagsRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    pub tags: Vec<TagDto>,
}

#[derive(Serialize, Debug)]
pub struct CreateTagRequest {
    pub title: String,
    pub color: String,
}

#[derive(Serialize, Debug)]
pub struct CreateTodoRequest {
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub card: CardReference,
}

#[derive(Serialize, Debug)]
pub struct CardReference {
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct UpdateTodoRequest {
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct CreateActiveRequest {
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    pub card: CardReference,
}