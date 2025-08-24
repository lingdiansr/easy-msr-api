use crate::client::remote::RemoteApiClient;
use crate::error::AppError;
use crate::web::dto::*;
use axum::{
    Json,
    extract::{Path, State},
};

#[utoipa::path(
    get,
    path="/song/{cid}",
    params(
        ("cid"=String,Path,description="歌曲cid")
    ),
    responses(
        (status=200,description="歌曲",body=SongResp)
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
        (status=200,description="所有歌曲列表",body=AllSongsResp)
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
        (status=200,description="专辑",body=AlbumResp)
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
        (status=200,description="专辑详情",body=AlbumDetailResp)
    ),
    tag = "albums"
)]
pub async fn get_album_detail(
    Path(cid): Path<String>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<AlbumDetailResp>, AppError> {
    client.get_album_detail(cid).await.map(Json)
}