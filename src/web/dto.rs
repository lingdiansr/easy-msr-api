use serde::{Deserialize, Serialize};
use utoipa::schema;
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
}
/// 最外层
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct SongResp {
    pub code: i32,
    #[serde(default)]
    pub msg: String,
    pub data: SongData,
}

/// data 字段
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct SongData {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "albumCid")]
    pub album_id: String,

    #[serde(rename = "sourceUrl")]
    #[schema(value_type = String, example = "https://res01.hycdn.cn/xxx/xxx.wav")]
    pub source_url: Option<String>,

    #[serde(rename = "lyricUrl")]
    pub lyric_url: Option<String>,

    #[serde(rename = "mvUrl")]
    pub mv_url: Option<String>,

    #[serde(rename = "mvCoverUrl")]
    pub mv_cover_url: Option<String>,

    pub artists: Vec<String>,
}