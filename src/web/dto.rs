use serde::{Deserialize, Serialize};
use utoipa::schema;

/// 响应体
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct ApiResp<T> {
    pub code: i32,
    #[serde(default)]
    pub msg: String,
    pub data: T,
}

/// 歌曲
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
pub type SongResp = ApiResp<SongData>;

// 所有歌曲列表单项
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct AllSongsItem {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "albumCid")]
    pub album_id: String,
    pub artists: Vec<String>,
}
// 所有歌曲列表
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct AllSongsData{
    pub list: Vec<AllSongsItem>,
    #[serde(rename = "autoplay")]
    pub auto_paly: String,
}
pub type AllSongsResp = ApiResp<AllSongsData>;


// 专辑
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct AlbumData {
    #[serde(rename = "cid")]
    pub id: String,

    pub name: String,

    pub intro: String,

    pub belong: String,

    #[serde(rename = "coverUrl")]
    pub cover_url: String,

    #[serde(rename = "coverDeUrl")]
    pub cover_de_url: String,

    #[serde(rename = "artistes")] // 怎么还拼错单词了
    pub artists: Vec<String>,
}
pub type AlbumResp = ApiResp<AlbumData>;

// 专辑详情中歌曲列表单项
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct AlbumDetailSongItem {
    #[serde(rename = "cid")]
    pub id: String,

    pub name: String,

    #[serde(rename = "artistes")] // 绷，还是错的
    pub artists: Vec<String>,
}
// 专辑详情
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct AlbumDetailData {
    #[serde(rename = "cid")]
    pub id: String,

    pub name: String,

    pub intro: String,

    pub belong: String,

    #[serde(rename = "coverUrl")]
    pub cover_url: String,

    #[serde(rename = "coverDeUrl")]
    pub cover_de_url: String,

    pub songs: Vec<AlbumDetailSongItem>,
}
pub type AlbumDetailResp = ApiResp<AlbumDetailData>;


// 所有专辑列表单项
#[derive(Serialize, Deserialize, Debug, utoipa::ToSchema)]
pub struct AllAlbumsItem {
    #[serde(rename = "cid")]
    pub id: String,
    
    pub name: String,

    #[serde(rename = "coverUrl")]
    pub cover_url: String,

    #[serde(rename = "artistes")] // 还是错的
    pub artists: Vec<String>,
}

// 所有专辑列表
// data字段下面直接是数组了，没法用类型别名
// pub type AllAlbumsResp = ApiResp<Vec<AllAlbumsItem>>;

