//! # OpenAPI文档配置
//! 
//! 定义了OpenAPI文档的配置和生成逻辑。
//! 
//! 使用`utoipa`库生成OpenAPI 3.0规范文档，支持Swagger UI集成。
//! 仅在启用`swagger-ui` feature时才会被使用。

use crate::web::dto::*;
use crate::web::handler::*;
use utoipa::OpenApi;

/// API文档结构体
/// 
/// 使用`utoipa`的派生宏自动生成OpenAPI文档。
/// 包含了所有API端点、数据模型和标签信息。
#[derive(OpenApi)]
#[openapi(
    paths(
        get_song, 
        get_all_songs, 
        get_album, 
        get_album_detail,
        get_all_albums,
        get_all_news,
        get_news_detail,
        get_font,
        search,
        search_albums,
        search_news,
    ),
    components(schemas( 
        // 响应类型
        SongResp, 
        AllSongsResp, 
        AlbumResp, 
        AlbumDetailResp,
        SearchResp,
        SearchAlbumResp,
        SearchNewsResp,
        NewsDetailResp,
        FontResp,
        
        // 数据模型
        SongData, 
        AllSongsData, 
        AllSongsItem,
        AlbumData,
        AlbumDetailData,
        AlbumDetailSongItem,
        AllAlbumsItem,
        SearchData,
        SearchAlbumData,
        SearchAlbumItem,
        NewsData,
        NewsItem,
        NewsDetailData,
        FontData,
        FontItem,
    )),
    tags(
        (name = "search", description = "搜索相关接口"),
        (name = "songs", description = "歌曲相关接口"),
        (name = "albums", description = "专辑相关接口"),
        (name = "news", description = "新闻(动向)相关接口"),
        (name = "others", description = "其他接口")
    )
)]
struct ApiDoc;

/// 获取OpenAPI文档实例
/// 
/// 生成并返回配置好的OpenAPI文档实例，用于Swagger UI展示。
/// 
/// # 返回
/// 
/// 返回`utoipa::openapi::OpenApi`实例，包含完整的API文档信息
/// 
/// # 示例
/// 
/// ```rust
/// use msr_api_rs::web::docs::api_doc;
/// 
/// let openapi = api_doc();
/// // 可以将此实例用于Swagger UI配置
/// ```
pub fn api_doc() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
