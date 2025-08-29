# MSR API Rust 封装库

这是一个为MSR API提供Rust封装的库，支持作为独立服务运行或集成到其他项目中。

## 功能特性

- **核心API封装**：完整的MSR API功能封装
- **可选Swagger UI**：通过feature控制是否启用Swagger文档界面
- **Axum集成**：基于Axum的Web路由
- **类型安全**：完整的DTO类型定义

## 使用方式

### 作为库使用

#### 基础API路由（无Swagger UI）
```rust
use msr_api_rs::{client::remote::RemoteApiClient, web};

let client = RemoteApiClient::new("https://api.example.com");
let app = web::routes(client);
```

#### 带Swagger UI的API路由
```rust
use msr_api_rs::{client::remote::RemoteApiClient, web};

let client = RemoteApiClient::new("https://api.example.com");
let app = web::routes_with_swagger(client); // 需要启用swagger-ui feature
```

### 作为独立服务运行

#### 默认模式（无Swagger UI）
```bash
cargo run
# 或
cargo run --bin server
```

#### 启用Swagger UI模式
```bash
cargo run --features swagger-ui --bin server
```

服务启动后：
- API服务：http://localhost:8080
- Swagger UI：http://localhost:8080/swagger-ui （仅启用swagger-ui feature时可用）

## Cargo Features

- **default**: 无额外功能
- **swagger-ui**: 启用Swagger UI界面支持

## API端点

### 歌曲相关
- `GET /song/{cid}` - 获取歌曲详情
- `GET /songs` - 获取所有歌曲列表

### 专辑相关
- `GET /album/{cid}/data` - 获取专辑信息
- `GET /album/{cid}/detail` - 获取专辑详情
- `GET /albums` - 获取所有专辑列表

### 新闻相关
- `GET /news` - 获取新闻列表
- `GET /news/{cid}` - 获取新闻详情

### 搜索功能
- `GET /search?keyword={keyword}` - 综合搜索
- `GET /search/album?keyword={keyword}` - 搜索专辑
- `GET /search/news?keyword={keyword}` - 搜索新闻

### 其他
- `GET /fontset` - 获取字体配置

## 开发说明

### 项目结构
```
src/
├── client/          # API客户端封装
├── web/            # Web路由和DTO
│   ├── handler.rs  # 请求处理器
│   ├── dto.rs      # 数据传输对象
│   └── docs.rs     # OpenAPI文档
├── config.rs       # 配置管理
├── error.rs        # 错误处理
└── lib.rs          # 库入口
```

### 添加新功能
1. 在`client/remote.rs`中添加API方法
2. 在`web/dto.rs`中添加对应的DTO类型
3. 在`web/handler.rs`中添加处理器函数
4. 在`web/mod.rs`中添加路由
5. 在`web/docs.rs`中更新OpenAPI文档

## 环境变量

创建`.env`文件：
```bash
SERVER_PORT=8080
REMOTE_BASE=https://api.example.com
