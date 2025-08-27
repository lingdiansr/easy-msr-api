use crate::client::remote::RemoteApiClient;
use crate::error::AppError;
use crate::web::dto::*;
use axum::{
    Json,
    extract::{Path, Query, State},
};

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
)->Result<Json<FontResp>,AppError>{
    client.get_font().await.map(Json)
}
