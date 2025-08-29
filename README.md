# MSR API Rust 封装库

这是一个为MSR API提供Rust封装的库，支持直接API调用、Web服务集成，以及可选的Swagger UI文档。

## 功能特性

- **核心API封装**：完整的MSR API功能封装，可直接调用
- **可选Web路由**：提供Axum Web路由集成
- **可选Swagger UI**：通过feature控制是否启用Swagger文档界面
- **类型安全**：完整的DTO类型定义
- **灵活使用**：支持直接API调用和Web服务两种模式

## 使用方式

### 1. 作为库直接调用API（推荐）

这是最主要的使用方式，可以直接在其他库或应用中调用API：

```rust
use msr_api_rs::client::remote::RemoteApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建API客户端
    let client = RemoteApiClient::new("https://your-api-base.com".to_string());
    
    // 直接调用API方法 - 无需Web路由
    let song = client.get_song("123456".to_string()).await?;
    println!("歌曲名称: {}", song.data.name);
    
    let albums = client.get_all_albums().await?;
    println!("专辑数量: {}", albums.data.len());
    
    let search_result = client.search("关键词".to_string()).await?;
    println!("搜索结果: {} 个专辑, {} 条新闻", 
             search_result.data.albums.list.len(), 
             search_result.data.news.list.len());
    
    Ok(())
}
```

### 2. 作为Web服务使用

#### 基础Web路由（无Swagger UI）
```rust
use msr_api_rs::{client::remote::RemoteApiClient, web};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RemoteApiClient::new("https://your-api-base.com".to_string());
    let app = web::routes(client);
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 8080)).await?;
    println!("服务器运行在 http://localhost:8080");
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

#### 带Swagger UI的Web路由
```rust
use msr_api_rs::{client::remote::RemoteApiClient, web};

let client = RemoteApiClient::new("https://your-api-base.com".to_string());
let app = web::routes_with_swagger(client); // 需要启用swagger-ui feature
```

### 3. 作为独立服务运行

#### 默认模式（无Swagger UI）
```bash
cargo run --bin server
```

#### 启用Swagger UI模式
```bash
cargo run --features swagger-ui --bin server
```

## 可用的API方法

### 歌曲相关
- `client.get_song(id: String) -> Result<SongResp, AppError>`
- `client.get_all_songs() -> Result<AllSongsResp, AppError>`

### 专辑相关
- `client.get_album(id: String) -> Result<AlbumResp, AppError>`
- `client.get_album_detail(id: String) -> Result<AlbumDetailResp, AppError>`
- `client.get_all_albums() -> Result<ApiResp<Vec<AllAlbumsItem>>, AppError>`

### 新闻相关
- `client.get_all_news(last_cid: Option<String>) -> Result<SearchNewsResp, AppError>`
- `client.get_news_detail(id: String) -> Result<NewsDetailResp, AppError>`

### 搜索功能
- `client.search(keyword: String) -> Result<SearchResp, AppError>`
- `client.search_albums(keyword: String, last_cid: Option<String>) -> Result<SearchAlbumResp, AppError>`
- `client.search_news(keyword: String, last_cid: Option<String>) -> Result<SearchNewsResp, AppError>`

### 其他
- `client.get_font() -> Result<FontResp, AppError>`

## Cargo Features

- **default**: 无额外功能，仅包含核心API封装
- **swagger-ui**: 启用Swagger UI界面支持（用于Web服务）

## 项目结构

```
src/
├── client/
│   └── remote.rs   # 核心API封装，可直接调用
├── web/            # Web路由层（可选）
│   ├── handler.rs  # 请求处理器
│   ├── dto.rs      # 数据传输对象
│   └── docs.rs     # OpenAPI文档
├── config.rs       # 配置管理
├── error.rs        # 错误处理
└── lib.rs          # 库入口
```

## 使用场景对比

| 使用方式 | 适用场景 | 代码示例 |
|----------|----------|----------|
| **直接API调用** | 其他Rust库、CLI工具、后台服务 | `client.get_song("123")` |
| **Web路由** | RESTful API服务、微服务 | `web::routes(client)` |
| **带Swagger的Web** | 开发调试、API文档展示 | `web::routes_with_swagger(client)` |

## 环境变量

创建`.env`文件：
```bash
SERVER_PORT=8080
REMOTE_BASE=https://api.example.com
```

## 快速开始

1. **添加到Cargo.toml**：
```toml
[dependencies]
msr-api-rs = { path = "path/to/msr-api-rs" }
```

2. **直接API调用**：
```rust
use msr_api_rs::client::remote::RemoteApiClient;

let client = RemoteApiClient::new("https://api.example.com".to_string());
let song = client.get_song("123456".to_string()).await?;
```

3. **Web服务**（可选）：
```rust
use msr_api_rs::{client::remote::RemoteApiClient, web};
let client = RemoteApiClient::new("https://api.example.com".to_string());
let app = web::routes(client);
