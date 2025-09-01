# MSR API Rust 封装库

这是一个为MSR API提供Rust封装的库，支持直接API调用以及可选的Swagger UI文档。

## 功能特性

- **核心API封装**：完整的MSR API功能封装，可直接调用
- **可选Swagger UI**：通过feature控制是否启用Swagger文档界面
- **类型安全**：完整的DTO类型定义

## 使用方式

### 1. 作为库直接调用API（推荐）

这是最主要的使用方式，可以直接在其他库或应用中调用API：

```rust
use easy_msr_api::client::remote::RemoteApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建API客户端
    let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
    
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

### 2. 作为Swagger UI服务使用

#### 带Swagger UI的Web路由
```rust
use easy_msr_api::{client::remote::RemoteApiClient, web};

let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
let app = web::routes_with_swagger(client); // 需要启用swagger-ui feature
```

### 3. 作为独立服务运行

#### 启用Swagger UI模式
```bash
cargo run --features web --bin server
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
- **web**: 启用Swagger UI界面支持（用于Web服务）

## 项目结构

```
easy-msr-api
├── Cargo.lock
├── Cargo.toml
├── examples                        # 示例项目
│   ├── easy-msr-api-hello-lib           # 作为lib使用
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── easy-msr-api-hello-swagger-ui    # 作为swagger-ui使用
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── README.md
└── src
    ├── bin                         # 启动swagger-ui
    │   └── server.rs
    ├── client                      # api封装
    │   └── remote.rs
    ├── client.rs
    ├── config.rs                   # 配置管理
    ├── dto.rs                      # 相应、查询结构体
    ├── error.rs                    # 错误处理
    ├── lib.rs              
    ├── web                         # web处理
    │   ├── docs.rs                 # OpenAPI文档
    │   └── handler.rs              # 请求处理器
    └── web.rs
```


## 环境变量

创建`.env`文件：
```bash
SERVER_PORT=8080
REMOTE_BASE=https://monster-siren.hypergryph.com/api
```

## 快速开始

1. **添加到Cargo.toml**：
```toml
[dependencies]
easy-msr-api = "0.1.0"
```

2. **直接API调用**：
```rust
use easy_msr_api::client::remote::RemoteApiClient;

let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
let song = client.get_song("123456".to_string()).await?;
```

3. **Web服务**（可选）：
```rust
use easy_msr_api::{client::remote::RemoteApiClient, web};
let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
let app = web::routes(client);
```
