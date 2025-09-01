//! # Web路由模块
//! 
//! 提供Axum Web框架的路由定义和处理功能，包含完整的API端点和Swagger UI文档界面。
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
//! 
//! ### Swagger UI
//! - `GET /swagger-ui/` - Swagger UI文档界面
//! - `GET /api-docs/openapi.json` - OpenAPI规范文档

use crate::client::remote::RemoteApiClient;
use axum::Router;

pub mod docs;
pub mod handler;

/// 创建包含Swagger UI的完整API路由
/// 
/// 创建包含所有API端点的Axum路由，并添加Swagger UI文档界面。
/// 
/// # 参数
/// 
/// * `client` - 远程API客户端实例
/// 
/// # 返回
/// 
/// 返回配置好的Axum Router实例，包含所有API端点和Swagger UI
/// 
/// # 示例
/// 
/// ```rust
/// use msr_api::{client::remote::RemoteApiClient, web};
/// use std::net::Ipv4Addr;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
///     let app = web::routes(client);
///     
///     // 启动服务器
///     let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 8080)).await?;
///     println!("服务器运行在 http://localhost:8080");
///     println!("Swagger UI文档: http://localhost:8080/swagger-ui/");
///     axum::serve(listener, app).await?;
///     
///     Ok(())
/// }
/// ```
pub fn routes(client: RemoteApiClient) -> Router {
    use axum::routing::get;
    use handler::*;
    use utoipa_swagger_ui::SwaggerUi;

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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::api_doc()))
        .with_state(client)
}
