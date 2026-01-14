# iFlow Context Configuration for Robrix

## 项目概述

**Robrix** 是一个基于 Rust 和 Makepad UI 框架构建的多平台 Matrix 聊天客户端，使用 Project Robius 应用开发框架。

- **编程语言**: Rust (2024 edition)
- **UI 框架**: Makepad
- **主要依赖**: matrix-sdk, matrix-sdk-ui, robius-*, makepad-widgets
- **构建系统**: Cargo
- **目标平台**: macOS, Windows, Linux, Android, iOS

## 构建与运行命令

### 基础命令

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 运行应用
cargo run

# 带功能特性运行
cargo run --features tsp

# 检查编译（不生成二进制）
cargo check
```

### 测试命令

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test utils

# 运行单个测试
cargo test test_human_readable_list_empty

# 带输出运行测试
cargo test -- --nocapture

# 运行所有目标与特性的测试
cargo test --all-targets --all-features
```

### 代码质量

```bash
# 运行 clippy 检查
cargo clippy
cargo clippy --all-targets --all-features

# 格式化代码
cargo fmt

# 检查格式（不应用更改）
cargo fmt -- --check
```

### 自定义构建配置

```bash
# 优化的开发构建（带完整调试信息和断言）
cargo build --profile debug-opt

# 薄 LTO 的发布构建
cargo build --profile release-lto

# 完整优化的分发构建
cargo build --profile distribution
```

## 代码风格规范

### 导入组织

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
| Live Design ID | `snake_case` (组件), `PascalCase` (组件) | `button`, `MyWidget` |

### 错误处理

- 使用 `Result<T, Error>` 处理可失败操作
- 应用层错误使用 `anyhow::Result<T>`
- 使用 `.context()` 链式添加上下文
- 复杂错误实现 `std::error::Error` 和 `Display`

### Live Design (Makepad DSL)

```rust
live_design! {
    use link::theme::*;
    use crate::shared::styles::*;
    
    MyWidget = <View> {
        width: Fill,
        height: Fit,
        draw_bg: { color: (COLOR_SECONDARY) }
        content = <View> {
            flow: Down,
            padding: 10.0,
            spacing: 5.0
        }
    }
}
```

## rustfmt 配置

```toml
edition = "2021"
max_width = 100
tab_spaces = 4
hard_tabs = false
fn_single_line = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

## 项目结构

```
src/
├── lib.rs              # 库入口，模块导出
├── app.rs              # 应用入口点，状态管理
├── main.rs             # 程序入口
├── utils.rs            # 共享工具函数
├── sliding_sync.rs     # Matrix Sliding Sync 协议
├── avatar_cache.rs     # 头像缓存
├── media_cache.rs      # 媒体文件缓存
├── location.rs         # 位置功能
├── persistence/        # 状态序列化与存储
├── home/               # 主界面：房间列表、导航
├── room/               # 独立房间视图与消息处理
├── shared/             # 可复用 UI 组件
├── login/              # 认证与登录界面
├── settings/           # 应用设置与偏好
├── profile/            # 用户信息与侧边栏
├── verification/       # E2EE 验证流程
├── tsp/                # TSP 钱包集成 [特性: tsp]
└── tsp_dummy/          # TSP 占位模块 [默认禁用]
```

## 特性标志

| 特性 | 描述 |
|------|------|
| `default` | 空 |
| `tsp` | 启用 TSP 钱包实验支持 |
| `hide_windows_console` | 隐藏 Windows 命令行控制台 |
| `log_room_list_diffs` | 记录所有 RoomListService diffs |
| `log_timeline_diffs` | 记录所有 timeline diffs |
| `log_space_service_diffs` | 记录 SpaceService diffs |

## 关键依赖模式

### Matrix SDK 集成

- 使用 `matrix-sdk` 进行核心 Matrix 操作
- 使用 `matrix-sdk-ui` 管理 timeline
- 异步 Matrix 操作需要正确的 runtime 设置
- 使用 sliding sync 高效更新 timeline

### Makepad 模式

- 使用 `live_design!` 宏定义 UI
- 实现 `LiveHook` 处理组件生命周期
- 使用 `WidgetRef` 引用组件
- 通过事件系统处理交互

### 状态管理

- 使用 `AppState` 管理全局应用状态
- 使用 `OnceLock` 实现单例
- 为持久化状态实现正确序列化
- 使用异步通道进行组件间通信

## 开发最佳实践

1. **提交前必须运行**: `cargo check` 和 `cargo clippy`
2. **Matrix 操作需谨慎测试**: 涉及网络 I/O
3. **新 UI 组件遵循现有模式**: 不要重复造轮子
4. **注意 async/await 边界**: UI 与 Matrix 操作之间
5. **遵守特性标志**: 新功能需考虑特性开关
6. **复用 `utils.rs` 工具函数**: 避免逻辑重复

## 常见陷阱

- Makepad 的 live design 语法有特定要求
- Matrix SDK 操作是异步的，需要正确的 runtime 设置
- 图片处理需要注意格式和缓存
- 跨平台兼容性需要仔细的资源管理
- 特性标志影响编译的代码，需测试所有相关配置
