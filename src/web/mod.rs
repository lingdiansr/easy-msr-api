//! # Web路由模块
//! 
//! 提供Axum Web框架的路由定义和处理功能。
//! 
//! 该模块定义了所有可用的HTTP端点，并提供了两种路由配置：
//! - 基础路由（不包含Swagger UI）
//! - 带Swagger UI的路由（需要启用swagger-ui feature）
//! 
//! ## 路由端点
//! 
//! ### 歌曲相关
//! - `GET /song/{cid}` - 获取歌曲详情
//! - `GET /songs` - 获取所有歌曲列表
//! 
//! ### 专辑相关
//! - `GET /album/{cid}/data` - 获取专辑信息
//! - `GET /album/{cid}/detail` - 获取专辑详情（含歌曲列表）
//! - `GET /albums` - 获取所有专辑列表
//! 
//! ### 新闻相关
//! - `GET /news` - 获取所有新闻列表
//! - `GET /news/{cid}` - 获取新闻详情
//! 
//! ### 搜索功能
//! - `GET /search` - 综合搜索（专辑和新闻）
//! - `GET /search/album` - 搜索专辑
//! - `GET /search/news` - 搜索新闻
//! 
//! ### 其他
//! - `GET /fontset` - 获取字体配置

use crate::client::remote::RemoteApiClient;
use axum::Router;

pub mod docs;
pub mod dto;
pub mod handler;

/// 基础API路由，不包含Swagger UI
/// 
/// 创建包含所有API端点的Axum路由，但不包含Swagger UI文档界面。
/// 
/// # 参数
/// 
/// * `client` - 远程API客户端实例
/// 
/// # 返回
/// 
/// 返回配置好的Axum Router实例
/// 
/// # 示例
/// 
/// ```rust
/// use msr_api_rs::{client::remote::RemoteApiClient, web};
/// use std::net::Ipv4Addr;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = RemoteApiClient::new("https://api.example.com".to_string());
///     let app = web::routes(client);
///     
///     let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 8080)).await?;
///     axum::serve(listener, app).await?;
///     
///     Ok(())
/// }
/// ```
pub fn routes(client: RemoteApiClient) -> Router {
    use axum::routing::get;
    use handler::*;

    Router::new()
        .route("/song/{cid}", get(get_song))
        .route("/songs", get(get_all_songs))
        .route("/album/{cid}/data", get(get_album))
        .route("/album/{cid}/detail", get(get_album_detail))
        .route("/albums", get(get_all_albums))
        .route("/news", get(get_all_news))
        .route("/news/{cid}", get(get_news_detail))
        .route("/search", get(search))
        .route("/search/album", get(search_albums))
        .route("/search/news", get(search_news))
        .route("/fontset", get(get_font))
        .with_state(client)
}

/// 带Swagger UI的API路由
/// 
/// 在基础路由的基础上添加Swagger UI文档界面。
/// 需要启用`swagger-ui` feature才能使用。
/// 
/// # 参数
/// 
/// * `client` - 远程API客户端实例
/// 
/// # 返回
/// 
/// 返回包含Swagger UI的Axum Router实例
/// 
/// # 示例
/// 
/// ```rust
/// use msr_api_rs::{client::remote::RemoteApiClient, web};
/// 
/// let client = RemoteApiClient::new("https://api.example.com".to_string());
/// let app = web::routes_with_swagger(client);
/// // Swagger UI将在 http://localhost:8080/swagger-ui 可用
/// ```
#[cfg(feature = "swagger-ui")]
pub fn routes_with_swagger(client: RemoteApiClient) -> Router {
    use utoipa_swagger_ui::SwaggerUi;
    
    routes(client).merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::api_doc()))
}
