//! # 请求处理器
//! 
//! 定义了所有HTTP端点的请求处理函数。
//! 
//! 每个处理函数都对应一个特定的API端点，负责接收请求、调用远程API并返回响应。
//! 所有处理函数都使用`utoipa`进行OpenAPI文档注解。

use crate::client::remote::RemoteApiClient;
use crate::error::AppError;
use crate::dto::*;
use axum::{
    Json,
    extract::{Path, Query, State},
};

/// 获取歌曲详情
/// 
/// 根据歌曲cid获取歌曲的详细信息，包括音频文件URL、歌词URL等。
#[utoipa::path(
    get,
    path="/song/{cid}",
    params(
        ("cid"=String,Path,description="歌曲cid")
    ),
    responses(
        (status=200,description="歌曲",body=ApiResp<SongData>)
    ),
    tag = "songs"
)]
pub async fn get_song(
    Path(cid): Path<String>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<SongResp>, AppError> {
    client.get_song(cid).await.map(Json)
}

/// 获取所有歌曲列表
/// 
/// 获取所有歌曲的基本信息列表，不包含音频文件URL等详细信息。
#[utoipa::path(
    get,
    path="/songs",
    responses(
        (status=200,description="所有歌曲列表",body=ApiResp<AllSongsData>)
    ),
    tag = "songs"
)]
pub async fn get_all_songs(
    State(client): State<RemoteApiClient>,
) -> Result<Json<AllSongsResp>, AppError> {
    client.get_all_songs().await.map(Json)
}

/// 获取专辑信息
/// 
/// 根据专辑cid获取专辑的基本信息。
#[utoipa::path(
    get,
    path="/album/{cid}/data",
    params(
        ("cid"=String,Path,description="专辑cid")
    ),
    responses(
        (status=200,description="专辑",body=ApiResp<AlbumData>)
    ),
    tag = "albums"
)]
pub async fn get_album(
    Path(cid): Path<String>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<AlbumResp>, AppError> {
    client.get_album(cid).await.map(Json)
}

/// 获取专辑详情
/// 
/// 根据专辑cid获取专辑的详细信息，包括专辑中的所有歌曲列表。
#[utoipa::path(
    get,
    path="/album/{cid}/detail",
    params(
        ("cid"=String,Path,description="专辑cid")
    ),
    responses(
        (status=200,description="专辑详情",body=ApiResp<AlbumDetailData>)
    ),
    tag = "albums"
)]
pub async fn get_album_detail(
    Path(cid): Path<String>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<AlbumDetailResp>, AppError> {
    client.get_album_detail(cid).await.map(Json)
}

/// 获取所有专辑列表
/// 
/// 获取所有专辑的基本信息列表。
#[utoipa::path(
    get,
    path="/albums",
    responses(
        (status=200,description="所有专辑列表",body=ApiResp<Vec<AllAlbumsItem>>)
    ),
    tag = "albums"
)]
pub async fn get_all_albums(
    State(client): State<RemoteApiClient>,
) -> Result<Json<ApiResp<Vec<AllAlbumsItem>>>, AppError> {
    client.get_all_albums().await.map(Json)
}

/// 综合搜索
/// 
/// 同时搜索专辑和新闻，返回综合搜索结果。
#[utoipa::path(
    get,
    path="/search",
    params(
        ("keyword" = String, Query, description = "搜索关键词")
    ),
    responses(
        (status=200,description="搜索结果",body=ApiResp<SearchData>)
    ),
    tag = "search"
)]
pub async fn search(
    Query(q): Query<SearchQuery>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<SearchResp>, AppError> {
    client.search(q.keyword).await.map(Json)
}

/// 搜索专辑
/// 
/// 根据关键词搜索专辑，支持分页。
#[utoipa::path(
    get,
    path="/search/album",
    params(
        ("keyword"=String,Query,description="搜索专辑关键词"),
        ("lastCid"=Option<String>,Query,description="从该项之后加载")
    ),
    responses(
        (status=200,description="搜索专辑结果",body=ApiResp<SearchAlbumData>)
    ),
    tags=["search","albums"],
)]
pub async fn search_albums(
    Query(q): Query<SearchAlbumQuery>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<SearchAlbumResp>, AppError> {
    client.search_albums(q.keyword, q.last_cid).await.map(Json)
}

/// 搜索新闻
/// 
/// 根据关键词搜索新闻，支持分页。
#[utoipa::path(
    get,
    path="/search/news",
    params(
        ("keyword" = String, Query, description = "新闻关键词"),
        ("lastCid" = Option<String>, Query, description = "从该项之后加载")
    ),
    responses(
        (status=200,description="新闻列表",body=ApiResp<NewsData>)
    ),
    tags =[ "news","search"],
)]
pub async fn search_news(
    Query(q): Query<NewsQuery>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<SearchNewsResp>, AppError> {
    client.search_news(q.keyword, q.last_cid).await.map(Json)
}

/// 获取所有新闻列表
/// 
/// 获取所有新闻的基本信息列表，支持分页。
#[utoipa::path(
    get,
    path="/news",
    params(
        ("lastCid" = Option<String>, Query, description = "从该项之后加载")
    ),
    responses(
        (status=200,description="新闻列表",body=ApiResp<NewsData>)
    ),
    tags =["news"],
)]
pub async fn get_all_news(
    Query(q): Query<AllNewsQuery>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<SearchNewsResp>, AppError> {
    client.get_all_news(q.last_cid).await.map(Json)
}

/// 获取新闻详情
/// 
/// 根据新闻cid获取新闻的详细内容。
#[utoipa::path(
    get,
    path="/news/{cid}",
    params(
        ("cid"=String,Path,description="新闻cid")
    ),
    responses(
        (status=200,description="新闻详情",body=ApiResp<NewsDetailData>)
    ),
    tag = "news"
)]
pub async fn get_news_detail(
    Path(cid): Path<String>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<NewsDetailResp>, AppError> {   
    client.get_news_detail(cid).await.map(Json)
}

/// 获取字体配置
/// 
/// 获取字体文件的配置信息，包括不同格式的字体文件URL。
#[utoipa::path(
    get,
    path="/fontset",
    responses(
        (status=200,description="字体文件",body=ApiResp<FontData>)
    ),
    tag = "others"
)]
pub async fn get_font(
    State(client): State<RemoteApiClient>
) -> Result<Json<FontResp>, AppError> {
    client.get_font().await.map(Json)
}
