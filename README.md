# Dog Bot Template

一个用 Rust 编写的现代化 Discord Bot 开发模板，展示了最佳实践和现代架构设计。

## 📋 项目概述

Dog Bot Template 是一个功能完整的 Discord Bot 开发范例，采用现代化的 Rust 技术栈构建。该项目展示了如何使用 Serenity 和 Poise 框架创建一个高性能、可扩展的 Discord Bot。

### 主要特性

- 🚀 **现代化架构**: 基于 Serenity 和 Poise 框架，支持斜杠命令和传统前缀命令
- 🗃️ **数据库集成**: 使用 SQLite 和 Sea-ORM 进行数据持久化
- 📊 **系统监控**: 实时 CPU、内存、数据库统计信息
- 🌍 **国际化支持**: 内置中文本地化支持
- 🔗 **外部 API 集成**: Cookie 提交功能演示如何集成第三方服务
- ⚡ **高性能**: 使用 Jemalloc 内存分配器和 Tokio 异步运行时
- 🔧 **配置管理**: 支持 JSON 配置文件和环境变量

## 🛠️ 技术栈

| 组件 | 技术 | 版本 |
|------|------|------|
| **语言** | Rust | Edition 2024 |
| **Discord 框架** | Serenity | 0.12 |
| **命令框架** | Poise | 0.6 |
| **数据库** | SQLite | - |
| **ORM** | Sea-ORM | 1.0 |
| **异步运行时** | Tokio | 1.0 |
| **内存分配器** | Jemalloc | 0.6 |
| **日志系统** | Tracing | 0.1 |
| **配置管理** | Figment | 0.10 |
| **HTTP 客户端** | Reqwest | 0.12 |

## 📁 项目结构

```tree
dog-bot-example/
├── src/
│   ├── main.rs              # 应用程序入口点
│   ├── lib.rs               # 库入口和公共导出
│   ├── config.rs            # 配置管理和解析
│   ├── database.rs          # 数据库连接和初始化
│   ├── error.rs             # 统一错误处理
│   ├── commands/            # Discord 命令模块
│   │   ├── mod.rs           # 命令模块导出和框架配置
│   │   ├── cookie.rs        # Cookie 提交命令
│   │   └── system.rs        # 系统信息命令
│   ├── handlers/            # Discord 事件处理器
│   │   ├── mod.rs           # 事件处理器导出
│   │   └── ping.rs          # Ping/Pong 和基础消息处理
│   ├── repo/                # 数据访问层 (Repository 模式)
│   │   ├── mod.rs           # 数据访问层导出
│   │   ├── flush.rs         # 消息清理功能
│   │   └── messages.rs      # 消息管理
│   └── utils/               # 工具函数和辅助模块
│       ├── mod.rs           # 工具模块导出
│       └── children.rs      # 子进程管理工具
├── entities/                # 数据库实体定义 (Sea-ORM)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── entities/
│           └── mod.rs
├── migration/               # 数据库迁移脚本
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── lib.rs
│       ├── main.rs
│       └── m20220101_000001_create_table.rs
├── config.json             # 配置文件
├── Cargo.toml              # 项目依赖和工作区配置
└── README.md               # 项目文档
```

## 🚀 快速开始

### 环境要求

- **Rust**: 1.83+ (Edition 2024)
- **Discord 应用**: 需要创建 Discord 应用并获取 Bot Token

### 1. 克隆项目

```bash
git clone https://github.com/Xerxes-2/dog-bot-example.git
cd dog-bot-example
```

### 2. 配置 Discord 应用

1. 前往 [Discord Developer Portal](https://discord.com/developers/applications)
2. 创建新应用程序
3. 在 "Bot" 页面创建 Bot 并获取 Token
4. 在 "OAuth2" > "URL Generator" 中选择 `bot` 和 `applications.commands` 权限
5. 使用生成的 URL 邀请 Bot 到服务器

### 3. 配置文件设置

复制配置文件模板并根据需要修改：

```bash
cp config.json.example config.json
```

编辑 `config.json`：

```json
{
  "token": "YOUR_BOT_TOKEN_HERE",
  "timeOffset": 28800
}
```

### 4. 数据库设置

#### 安装 SeaORM CLI 工具

```bash
cargo install sea-orm-cli
```

#### 创建数据库文件

```bash
# 创建 SQLite 数据库文件
touch sqlite.db
```

#### 配置环境变量

创建 `.env` 文件并指定数据库路径：

```env
# .env 文件内容
DATABASE_URL=sqlite://sqlite.db
```

#### 配置数据库迁移

编辑 `migration/src/m20220101_000001_create_table.rs` 文件，替换模板代码为实际的建表逻辑：

```rust
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 pending_flushes 表
        manager
            .create_table(
                Table::create()
                    .table(PendingFlushes::Table)
                    .if_not_exists()
                    .col(pk_auto(PendingFlushes::Id))
                    .col(big_integer(PendingFlushes::MessageId))
                    .col(big_integer(PendingFlushes::NotificationId))
                    .col(big_integer(PendingFlushes::ChannelId))
                    .col(big_integer(PendingFlushes::ToiletId))
                    .col(big_integer(PendingFlushes::AuthorId))
                    .col(big_integer(PendingFlushes::FlusherId))
                    .col(big_integer(PendingFlushes::ThresholdCount))
                    .col(timestamp_with_time_zone(PendingFlushes::CreatedAt))
                    .col(text_null(PendingFlushes::Reason))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PendingFlushes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PendingFlushes {
    Table,
    Id,
    MessageId,
    NotificationId,
    ChannelId,
    ToiletId,
    AuthorId,
    FlusherId,
    ThresholdCount,
    CreatedAt,
    Reason,
}
```

#### 执行数据库迁移和生成实体

```bash
# 执行数据库迁移
sea-orm-cli migrate up

# 生成实体文件
sea-orm-cli generate entity -o entities/src/entities
```

#### 更新数据库结构

如果需要修改数据库结构：

```bash
# 生成新的迁移文件
sea-orm-cli generate migration add_new_column

# 编辑新生成的迁移文件，然后重复执行迁移和生成实体
sea-orm-cli migrate up
sea-orm-cli generate entity -o entities/src/entities
```

### 5. 运行 Bot

```bash
# 开发模式
cargo run

# 生产模式
cargo run --release
```

### 6. 注册斜杠命令

Bot 启动后，使用以下命令注册斜杠命令：

```
@your_bot_name register
```

## ⚙️ 配置说明

### 配置文件结构

| 字段 | 类型 | 说明 |
|------|------|------|
| `token` | String | Discord Bot Token |
| `timeOffset` | Number | log 时区偏移量 (秒) |

### 环境变量

除了配置文件，也支持环境变量配置：

```bash
export DOG_BOT_TOKEN="your_bot_token"
```

### 命令行参数

```bash
# 指定配置文件路径
cargo run -- --config custom-config.json

# 指定数据库路径
cargo run -- --db custom-database.db
```

## 🔧 功能模块详解

### 系统监控命令

`/system_info` 命令提供详细的系统性能统计：

- **系统信息**: OS 版本、内核版本、Rust 版本
- **硬件监控**: CPU 使用率、内存使用情况
- **Bot 统计**: 内存使用、数据库大小、WebSocket 延迟
- **运行时信息**: Tokio 任务队列、活跃任务数、工作线程数
- **缓存统计**: 缓存的用户数、服务器数、频道数

### Cookie 提交功能

`/submit_cookie` 命令演示了如何集成外部 API：

- 支持中文本地化 (`/提交曲奇`)
- Bearer Token 认证
- 错误处理和用户反馈
- 异步 HTTP 请求

### 消息管理系统

通过 Repository 模式实现的消息管理：

- **消息清理**: 定时清理过期消息记录
- **数据持久化**: 消息元数据存储到 SQLite
- **批量操作**: 支持批量插入和删除

### 事件处理系统

- **Ping/Pong**: 延迟测试和连接状态检查
- **缓存就绪**: 服务器连接状态监控
- **消息处理**: 传统前缀命令支持

## 📚 开发指南

### 添加新命令

1. 在 `src/commands/` 目录下创建新的命令文件
2. 使用 `#[command]` 宏定义命令
3. 在 `src/commands/mod.rs` 中注册命令

```rust
use poise::command;
use super::Context;
use crate::error::BotError;

#[command(slash_command, description = "示例命令")]
pub async fn example_command(ctx: Context<'_>) -> Result<(), BotError> {
    ctx.say("Hello, World!").await?;
    Ok(())
}
```

### 扩展事件处理器

1. 在 `src/handlers/` 目录下创建新的处理器
2. 实现 `EventHandler` trait
3. 在 `main.rs` 中注册处理器

```rust
use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct CustomHandler;

#[async_trait]
impl EventHandler for CustomHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        // 处理消息事件
    }
}
```

### 数据库开发流程

#### 1. 创建新的数据库迁移

```bash
# 生成新的迁移文件
sea-orm-cli generate migration create_users_table
```

#### 2. 编辑迁移文件

在 `migration/src/` 目录下编辑新生成的迁移文件：

```rust
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(big_integer(Users::UserId))
                    .col(string(Users::Username))
                    .col(timestamp_with_time_zone(Users::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    UserId,
    Username,
    CreatedAt,
}
```

#### 3. 执行迁移和生成实体

```bash
# 执行数据库迁移
sea-orm-cli migrate up

# 生成实体文件
sea-orm-cli generate entity -o entities/src/entities
```

#### 4. 实现 Repository 模式

在 `src/repo/` 目录下创建数据访问层：

```rust
use entities::users::*;
use sea_orm::{DbErr, Set, prelude::*};
use serenity::all::*;

use crate::{database::BotDatabase, error::BotError};

pub type UserInfo = Model;

pub struct UserRepo<'a>(&'a BotDatabase);

impl BotDatabase {
    pub fn users(&self) -> UserRepo<'_> {
        UserRepo(self)
    }
}

impl UserRepo<'_> {
    pub async fn create_user(
        self,
        user_id: UserId,
        username: String,
    ) -> Result<UserInfo, BotError> {
        let user = ActiveModel {
            user_id: Set(user_id.get() as i64),
            username: Set(username),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        Ok(user.insert(self.0.inner()).await?)
    }

    pub async fn get_user(
        self,
        user_id: UserId,
    ) -> Result<Option<UserInfo>, BotError> {
        Ok(Entity::find()
            .filter(Column::UserId.eq(user_id.get() as i64))
            .one(self.0.inner())
            .await?)
    }
}
```

### 错误处理最佳实践

使用 `snafu` 库进行结构化错误处理：

```rust
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum BotError {
    #[snafu(display("Discord API error: {source}"))]
    Discord { source: serenity::Error },
    
    #[snafu(display("Database error: {source}"))]
    Database { source: sea_orm::DbErr },
}
```

## 🚀 部署指南

### 构建发布版本

```bash
# 优化构建
cargo build --release

# 生成的二进制文件位于 target/release/dog-bot-template
```

### 生产环境配置

1. 使用环境变量存储敏感信息
2. 配置日志级别：`RUST_LOG=info`
3. 启用 Jemalloc 统计：`MALLOC_STATS_PRINT=1`

### 系统服务配置

创建 systemd 服务文件 `/etc/systemd/system/dog-bot.service`：

```ini
[Unit]
Description=Dog Bot Discord Bot
After=network.target

[Service]
Type=simple
User=botuser
WorkingDirectory=/opt/dog-bot
ExecStart=/opt/dog-bot/target/release/dog-bot-template
Environment=RUST_LOG=info
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

### Docker 部署 (可选)

```dockerfile
FROM rust:1.83-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dog-bot-template /usr/local/bin/
COPY config.json /app/

WORKDIR /app
CMD ["dog-bot-template"]
```

## 🔍 常见问题 & 故障排除

### 常见错误

**Q: Bot 无法启动，显示 "Invalid token"**
A: 检查配置文件中的 `token` 字段，确保使用正确的 Bot Token

**Q: 斜杠命令不显示**
A: 确保 Bot 有 `applications.commands` 权限，并运行 `/register` 命令

**Q: 数据库连接失败**
A: 检查 SQLite 文件权限，确保 Bot 有读写权限

### 性能优化

1. **内存优化**: 调整缓存设置中的 `max_messages` 参数
2. **网络优化**: 配置适当的超时时间和重试策略
3. **数据库优化**: 定期运行 `VACUUM` 命令压缩数据库

### 调试技巧

1. **启用详细日志**:

   ```bash
   RUST_LOG=debug cargo run
   ```

2. **使用 tracing 进行问题追踪**:

   ```rust
   use tracing::{info, warn, error};
   
   info!("Processing command: {}", command_name);
   ```

3. **监控系统资源**:
   使用 `/system_info` 命令监控 Bot 性能

## 🤝 贡献指南

### 代码风格

- 使用 `rustfmt` 格式化代码
- 遵循 Rust 标准命名约定
- 添加适当的文档注释

### 提交规范

使用 Conventional Commits 格式：

```
feat: 添加新的斜杠命令
fix: 修复内存泄漏问题
docs: 更新 README 文档
```

### 测试要求

```bash
# 运行测试
cargo test

# 检查代码格式
cargo fmt --check

# 运行 Clippy 检查
cargo clippy
```

## 📄 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 🙏 致谢

感谢以下开源项目的支持：

- [Serenity](https://github.com/serenity-rs/serenity) - Discord API 库
- [Poise](https://github.com/serenity-rs/poise) - 命令框架
- [Sea-ORM](https://github.com/SeaQL/sea-orm) - 异步 ORM
- [Tokio](https://github.com/tokio-rs/tokio) - 异步运行时

---

📝 **注意**: 这是一个开发模板项目，请根据实际需求进行定制和扩展。如有问题，请提交 Issue 或 Pull Request。
