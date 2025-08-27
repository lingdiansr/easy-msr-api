use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema, schema};

// 响应体
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ApiResp<T> {
    pub code: i32,
    #[serde(default)]
    pub msg: String,
    pub data: T,
}

// 歌曲
#[derive(Serialize, Deserialize, Debug, ToSchema)]
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
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AllSongsItem {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "albumCid")]
    pub album_id: String,
    pub artists: Vec<String>,
}
// 所有歌曲列表
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AllSongsData {
    pub list: Vec<AllSongsItem>,
    #[serde(rename = "autoplay")]
    pub auto_paly: String,
}
pub type AllSongsResp = ApiResp<AllSongsData>;

// 专辑
#[derive(Serialize, Deserialize, Debug, ToSchema)]
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
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AlbumDetailSongItem {
    #[serde(rename = "cid")]
    pub id: String,

    pub name: String,

    #[serde(rename = "artistes")] // 绷，还是错的
    pub artists: Vec<String>,
}
// 专辑详情
#[derive(Serialize, Deserialize, Debug, ToSchema)]
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
#[derive(Serialize, Deserialize, Debug, ToSchema)]
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

// 搜索结果中专辑列表单项
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SearchAlbumItem {
    #[serde(rename = "cid")]
    pub id: String,

    pub name: String,

    pub belong: String,

    #[serde(rename = "coverUrl")]
    pub cover_url: String,

    #[serde(rename = "artistes")] // 还是错的
    pub artists: Vec<String>,
}
// 搜索结果中专辑相关
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SearchAlbumData {
    pub list: Vec<SearchAlbumItem>,
    pub end: bool,
}
#[derive(Serialize, Deserialize, Debug, IntoParams)]
pub struct SearchAlbumQuery {
    pub keyword: String,
    #[serde(rename = "lastCid")]
    pub last_cid: Option<String>,
}
pub type SearchAlbumResp = ApiResp<SearchAlbumData>;
// 搜索结果中新闻列表单项
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewsItem {
    #[serde(rename = "cid")]
    pub id: String,

    pub title: String,

    pub cate: i32,
    #[schema(value_type = String, example = "2022-01-01")]
    pub date: String,
}
// 搜索结果中新闻相关
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewsData {
    pub list: Vec<NewsItem>,
    pub end: bool,
}
// 搜索结果
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SearchData {
    pub albums: SearchAlbumData,
    pub news: NewsData,
}
// 查询参数结构体
#[derive(Serialize, Deserialize, Debug, IntoParams)]
pub struct SearchQuery {
    pub keyword: String,
}
pub type SearchResp = ApiResp<SearchData>;
