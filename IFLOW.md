# iFlow CLI Configuration for toona

## 项目概述

**toona** 是一个多平台 Matrix 聊天客户端，使用 Rust 和 Makepad UI 框架构建。

- **语言**: Rust (2024 edition)
- **UI 框架**: Makepad
- **构建系统**: Cargo
- **目标平台**: macOS, Windows, Linux, Android, iOS

## iFlow 内存配置

### 记忆要点

- **不要生成任何形式的文档在项目中**

## 构建命令速查

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 运行应用
cargo run

# 带功能特性运行（TSP）
cargo run --features tsp

# 代码检查
cargo check

# 运行 clippy
cargo clippy --all-targets --all-features

# 代码格式化
cargo fmt
```

## 代码风格要点

### 导入顺序

```rust
// 标准库 → 外部 crate → 本地模块
use std::{borrow::Cow, ops::{Deref, DerefMut}};
use serde::{Deserialize, Serialize};
use makepad_widgets::*;
use matrix_sdk::ruma::{OwnedRoomId, RoomId};
use crate::{avatar_cache::*, home::*};
```

### 命名约定

| 类型 | 规范 | 示例 |
|------|------|------|
| 模块 | `snake_case` | `utils.rs`, `app_state.rs` |
| 类型/结构体 | `PascalCase` | `RoomNameId`, `AppState` |
| 函数/方法 | `snake_case` | `human_readable_list` |
| 常量 | `SCREAMING_SNAKE_CASE` | `COLOR_PRIMARY` |

## 项目结构

```
src/
├── lib.rs              # 库入口
├── app.rs              # 应用入口，状态管理
├── main.rs             # 程序入口
├── utils.rs            # 共享工具函数
├── sliding_sync.rs     # Matrix Sliding Sync
├── avatar_cache.rs     # 头像缓存
├── media_cache.rs      # 媒体缓存
├── location.rs         # 位置功能
├── persistence/        # 状态持久化
├── home/               # 主界面组件
├── room/               # 房间视图
├── shared/             # 可复用 UI 组件
├── login/              # 登录认证
├── settings/           # 应用设置
├── profile/            # 用户信息
├── verification/       # E2EE 验证
├── logout/             # 登出流程
├── tsp/                # TSP 钱包集成 [特性: tsp]
└── tsp_dummy/          # 占位模块
```

## 特性标志

| 特性 | 描述 |
|------|------|
| `tsp` | 启用 TSP 钱包实验支持 |
| `hide_windows_console` | 隐藏 Windows 控制台 |
| `log_room_list_diffs` | 记录 RoomList diffs |
| `log_timeline_diffs` | 记录 timeline diffs |
| `log_space_service_diffs` | 记录 SpaceService diffs |

## 开发注意事项

1. **提交前检查**: `cargo check` 和 `cargo clippy`
2. **Matrix 操作**: 涉及网络 I/O，需谨慎测试
3. **新 UI 组件**: 遵循现有模式，使用 `live_design!` 宏
4. **async/await**: 注意 UI 与 Matrix 操作间的边界
5. **特性标志**: 新功能需考虑特性开关
6. **工具函数**: 复用 `utils.rs` 避免重复
