use crate::client::remote::RemoteApiClient;
use crate::error::AppError;
use crate::web::dto::*;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

#[utoipa::path(
    get,
    path="/song/{cid}",
    params(
        ("cid"=i32,Path,description="歌曲cid")
    ),
    responses(
        (status=200,description="歌曲",body=SongResp)
    ),
    tag = "songs"
)]
pub async fn get_song(
    Path(cid): Path<i32>,
    State(client): State<RemoteApiClient>,
) -> Result<Json<SongResp>, AppError> {
    client.get_song(cid).await.map(Json)
}
