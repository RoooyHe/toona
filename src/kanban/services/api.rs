use crate::kanban::models::dto::*;
use std::sync::mpsc::Sender;
use makepad_widgets::SignalToUI;

// API 服务层 - 从 app.rs 移动过来
pub struct ApiService;

impl ApiService {
    const BASE_URL: &'static str = "http://localhost:8911/api/v1";
    const USER_ID: &'static str = "1";

    // 获取空间列表
    pub fn fetch_spaces(tx: Sender<Vec<SpaceDto>>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let url = format!("{}/space/byUserId/{}", Self::BASE_URL, Self::USER_ID);
            let request = reqwest::blocking::get(&url);
            
            match request {
                Ok(response) => match response.json::<Vec<SpaceDto>>() {
                    Ok(spaces) => {
                        let _ = tx.send(spaces);
                        signal.set();
                    }
                    Err(e) => {
                        println!("JSON 解析失败: {}", e);
                    }
                },
                Err(e) => {
                    println!("API 请求失败: {}", e);
                }
            }
        });
    }

    // 获取卡片详情
    pub fn fetch_card_detail(card_id: i64, tx: Sender<CardDetailDto>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let url = format!("{}/card/{}/detail", Self::BASE_URL, card_id);
            println!("API: 请求卡片详情 URL: {}", url);
            
            let request = reqwest::blocking::get(&url);
            
            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 卡片详情响应状态: {}", status);
                    
                    if status.is_success() {
                        match response.json::<CardDetailDto>() {
                            Ok(card_detail) => {
                                println!("API: 成功获取卡片详情: {:?}", card_detail);
                                let _ = tx.send(card_detail);
                                signal.set();
                            }
                            Err(e) => {
                                println!("API: 卡片详情 JSON 解析失败: {}", e);
                            }
                        }
                    } else {
                        println!("API: 获取卡片详情失败，状态码: {}", status);
                    }
                }
                Err(e) => {
                    println!("API: 卡片详情请求失败: {}", e);
                }
            }
        });
    }

    // 创建空间
    pub fn create_space(title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let new_space = CreateSpaceRequest {
                title,
                user_id: Self::USER_ID.to_string(),
                canceled: Some(false),
                sort: Some(1),
                color: Some("#FFFFFF".to_string()),
                sort_by: Some("created_at".to_string()),
            };

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/space", Self::BASE_URL);
            let request = client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&new_space)
                .send();

            match request {
                Ok(response) => {
                    let success = response.status().is_success();
                    let _ = tx.send(success);
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 更新空间标题
    pub fn update_space_title(space_id: i64, new_title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let update_space = UpdateSpaceRequest { 
                title: new_title,
                user_id: Self::USER_ID.to_string(),
            };

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/space/{}", Self::BASE_URL, space_id);
            let request = client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(&update_space)
                .send();

            match request {
                Ok(response) => {
                    let success = response.status().is_success();
                    let _ = tx.send(success);
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 创建卡片
    pub fn create_card(space_id: i64, title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let new_card = CreateCardRequest {
                title: title.clone(),
                description: None,
                status: Some(false),
                space: SpaceReference { id: space_id },
            };

            println!("API: 发送创建卡片请求，数据: {:?}", new_card);

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/card", Self::BASE_URL);
            let request = client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&new_card)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 创建卡片响应状态: {}", status);
                    
                    if status.is_success() {
                        if let Ok(response_text) = response.text() {
                            println!("API: 创建卡片响应内容: {}", response_text);
                        }
                        let _ = tx.send(true);
                    } else {
                        println!("API: 创建卡片失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 创建卡片请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 更新卡片标题
    pub fn update_card_title(card_id: i64, new_title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let update_card = UpdateCardRequest {
                title: new_title,
                description: None,
                status: Some(false),
            };

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/card/{}", Self::BASE_URL, card_id);
            let request = client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(&update_card)
                .send();

            match request {
                Ok(response) => {
                    let success = response.status().is_success();
                    let _ = tx.send(success);
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 更新卡片描述
    pub fn update_card_description(
        card_id: i64, 
        title: String,  // 保持原标题
        new_description: String, 
        status: Option<bool>,  // 保持原状态
        tx: Sender<bool>, 
        signal: SignalToUI
    ) {
        std::thread::spawn(move || {
            let update_card = UpdateCardRequest {
                title,  // 使用原标题
                description: Some(new_description),
                status,  // 使用原状态
            };

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/card/{}", Self::BASE_URL, card_id);
            println!("API: 更新卡片描述 URL: {}", url);
            
            let request = client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(&update_card)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 更新卡片描述响应状态: {}", status);
                    let success = status.is_success();
                    let _ = tx.send(success);
                }
                Err(e) => {
                    println!("API: 更新卡片描述失败: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 删除卡片
    pub fn delete_card(card_id: i64, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let url = format!("{}/card/{}", Self::BASE_URL, card_id);
            let request = client.delete(&url).send();

            match request {
                Ok(response) => {
                    let success = response.status().is_success();
                    let _ = tx.send(success);
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 获取全部标签
    pub fn fetch_all_tags(tx: Sender<Vec<TagDto>>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let url = format!("{}/tag", Self::BASE_URL);
            println!("API: 请求全部标签 URL: {}", url);
            
            let request = reqwest::blocking::get(&url);
            
            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 全部标签响应状态: {}", status);
                    
                    if status.is_success() {
                        match response.json::<Vec<TagDto>>() {
                            Ok(tags) => {
                                println!("API: 成功获取 {} 个标签", tags.len());
                                let _ = tx.send(tags);
                                signal.set();
                            }
                            Err(e) => {
                                println!("API: 标签 JSON 解析失败: {}", e);
                            }
                        }
                    } else {
                        println!("API: 获取标签失败，状态码: {}", status);
                    }
                }
                Err(e) => {
                    println!("API: 标签请求失败: {}", e);
                }
            }
        });
    }

    // 为卡片添加标签
    pub fn update_card_tags(card_id: i64, title: String, description: Option<String>, status: Option<bool>, tags: Vec<TagDto>, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let update_request = UpdateCardTagsRequest {
                title,
                description,
                status,
                tags,
            };

            println!("API: 更新卡片标签，卡片ID: {}, 标签数量: {}", card_id, update_request.tags.len());

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/card/{}", Self::BASE_URL, card_id);
            let request = client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(&update_request)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 更新卡片标签响应状态: {}", status);
                    
                    if status.is_success() {
                        let _ = tx.send(true);
                    } else {
                        println!("API: 更新卡片标签失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 更新卡片标签请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 创建新标签
    pub fn create_tag(title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            // 生成随机颜色
            let colors = vec![
                "#9FE7B4FF", "#61BD4FFF", "#FF6B6BFF", "#4ECDC4FF", 
                "#45B7D1FF", "#FFA07AFF", "#FFD93DFF", "#6BCF7FFF",
                "#FF8A80FF", "#81C784FF", "#64B5F6FF", "#FFB74DFF"
            ];
            let color = colors[title.len() % colors.len()].to_string();

            let create_request = CreateTagRequest {
                title: title.clone(),
                color,
            };

            println!("API: 创建标签，标题: '{}'", title);

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/tag", Self::BASE_URL);
            let request = client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&create_request)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 创建标签响应状态: {}", status);
                    
                    if status.is_success() {
                        let _ = tx.send(true);
                    } else {
                        println!("API: 创建标签失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 创建标签请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 创建新Todo
    pub fn create_todo(card_id: i64, title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let create_request = CreateTodoRequest {
                title: title.clone(),
                user_id: Self::USER_ID.to_string(),
                parent_id: None,
                end_time: None,
                card: CardReference { id: card_id },
            };

            println!("API: 创建Todo，标题: '{}', 卡片ID: {}", title, card_id);

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/todo", Self::BASE_URL);
            let request = client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&create_request)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 创建Todo响应状态: {}", status);
                    
                    if status.is_success() {
                        let _ = tx.send(true);
                    } else {
                        println!("API: 创建Todo失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 创建Todo请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 更新Todo状态
    pub fn update_todo(todo_id: i64, title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let update_request = UpdateTodoRequest {
                title,
                user_id: Self::USER_ID.to_string(),
                parent_id: None,
                end_time: None,
            };

            println!("API: 更新Todo，ID: {}", todo_id);

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/todo/{}", Self::BASE_URL, todo_id);
            let request = client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(&update_request)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 更新Todo响应状态: {}", status);
                    
                    if status.is_success() {
                        let _ = tx.send(true);
                    } else {
                        println!("API: 更新Todo失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 更新Todo请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 删除Todo
    pub fn delete_todo(todo_id: i64, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            println!("API: 删除Todo，ID: {}", todo_id);

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/todo/{}", Self::BASE_URL, todo_id);
            let request = client.delete(&url).send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 删除Todo响应状态: {}", status);
                    
                    if status.is_success() {
                        let _ = tx.send(true);
                    } else {
                        println!("API: 删除Todo失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 删除Todo请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 创建新Active
    pub fn create_active(card_id: i64, title: String, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            let create_request = CreateActiveRequest {
                title: title.clone(),
                user_id: Self::USER_ID.to_string(),
                start_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string()),
                card: CardReference { id: card_id },
            };

            println!("API: 创建Active，标题: '{}', 卡片ID: {}", title, card_id);

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/active", Self::BASE_URL);
            let request = client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&create_request)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 创建Active响应状态: {}", status);
                    
                    if status.is_success() {
                        let _ = tx.send(true);
                    } else {
                        println!("API: 创建Active失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 创建Active请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    // 删除Active
    pub fn delete_active(active_id: i64, tx: Sender<bool>, signal: SignalToUI) {
        std::thread::spawn(move || {
            println!("API: 删除Active，ID: {}", active_id);

            let client = reqwest::blocking::Client::new();
            let url = format!("{}/active/{}", Self::BASE_URL, active_id);
            let request = client.delete(&url).send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("API: 删除Active响应状态: {}", status);
                    
                    if status.is_success() {
                        let _ = tx.send(true);
                    } else {
                        println!("API: 删除Active失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("API: 删除Active请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }
}