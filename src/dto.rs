//! # 数据传输对象 (DTO)
//!
//! 定义了API请求和响应的数据结构。
//!
//! 这些结构体用于序列化和反序列化JSON数据，并提供了OpenAPI文档支持。
//! 所有结构体都实现了`Serialize`、`Deserialize`和`ToSchema` trait。

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema, schema};

/// 统一的API响应格式
///
/// 所有API响应都使用这个统一的格式包装。
/// 包含状态码、消息和实际数据。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct ApiResp<T> {
    /// 响应状态码
    ///
    /// 0表示成功，其他值表示错误
    #[schema(value_type = i32, example = 0)]
    pub code: i32,

    /// 响应消息
    ///
    /// 成功时为空字符串，错误时包含错误描述
    #[schema(value_type = String, example = "")]
    pub msg: String,

    /// 响应数据
    ///
    /// 实际的业务数据，类型由泛型参数T决定
    pub data: T,
}

impl<T> ApiResp<T> {
    /// 创建成功的响应
    ///
    /// # 参数
    ///
    /// * `data` - 要返回的业务数据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use easy_msr_api::web::dto::ApiResp;
    ///
    /// let resp = ApiResp::success("Hello, World!");
    /// assert_eq!(resp.code, 0);
    /// ```
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data,
        }
    }

    /// 创建错误响应
    ///
    /// # 参数
    ///
    /// * `msg` - 错误消息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use easy_msr_api::web::dto::ApiResp;
    ///
    /// let resp: ApiResp<String> = ApiResp::error("参数错误".to_string());
    /// assert_eq!(resp.code, -1);
    /// ```
    pub fn error(msg: String) -> Self
    where
        T: Default,
    {
        Self {
            code: -1,
            msg,
            data: T::default(),
        }
    }
}

/// 歌曲数据
///
/// 包含歌曲的完整信息，包括音频文件URL、歌词URL等。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct SongData {
    /// 歌曲唯一标识符（cid）
    #[serde(rename = "cid")]
    #[schema(value_type = String, example = "953953")]
    pub id: String,

    /// 歌曲名称
    #[schema(value_type = String, example = "Little Wish")]
    pub name: String,

    /// 所属专辑cid
    #[serde(rename = "albumCid")]
    #[schema(value_type = String, example = "3888")]
    pub album_id: String,

    /// 音频文件URL
    #[serde(rename = "sourceUrl")]
    #[schema(value_type = String, example = "https://res01.hycdn.cn/xxx/xxx.wav")]
    pub source_url: Option<String>,

    /// 歌词文件URL
    #[serde(rename = "lyricUrl")]
    #[schema(value_type = String, example = "https://web.hycdn.cn/siren/lyric/xxx/xxx.lrc")]
    pub lyric_url: Option<String>,

    /// MV视频URL
    #[serde(rename = "mvUrl")]
    pub mv_url: Option<String>,

    /// MV封面URL
    #[serde(rename = "mvCoverUrl")]
    pub mv_cover_url: Option<String>,

    /// 艺术家列表
    pub artists: Vec<String>,
}

/// 歌曲响应类型
pub type SongResp = ApiResp<SongData>;

/// 所有歌曲列表单项
///
/// 简化版的歌曲信息，用于列表展示。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct AllSongsItem {
    /// 歌曲唯一标识符（cid）
    #[serde(rename = "cid")]
    #[schema(value_type = String, example = "953953")]
    pub id: String,

    /// 歌曲名称
    #[schema(value_type = String, example = "Little Wish")]
    pub name: String,

    /// 所属专辑cid
    #[serde(rename = "albumCid")]
    #[schema(value_type = String, example = "3888")]
    pub album_id: String,

    /// 艺术家列表
    pub artists: Vec<String>,
}

/// 所有歌曲数据
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct AllSongsData {
    /// 歌曲列表
    pub list: Vec<AllSongsItem>,

    /// 自动播放的歌曲cid
    #[serde(rename = "autoplay")]
    #[schema(value_type = String, example = "048794")]
    pub auto_paly: String,
}

/// 所有歌曲响应类型
pub type AllSongsResp = ApiResp<AllSongsData>;

/// 专辑数据
///
/// 包含专辑的基本信息。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct AlbumData {
    /// 专辑唯一标识符（cid）
    #[serde(rename = "cid")]
    #[schema(value_type = String, example = "3888")]
    pub id: String,

    /// 专辑名称
    #[schema(value_type = String, example = "Little Wish")]
    pub name: String,

    /// 专辑简介
    #[schema(value_type = String, example = "一触即碎的肥皂泡，也要托起小小愿望，飞越风雨，飞向太阳，绽放她的幻彩流光。")]
    pub intro: String,

    /// 所属分类
    #[schema(value_type = String, example = "arknights")]
    pub belong: String,

    /// 封面图片URL
    #[serde(rename = "coverUrl")]
    #[schema(value_type = String, example = "https://web.hycdn.cn/siren/pic/xxx/xxx.jpg")]
    pub cover_url: String,

    /// 详情页封面URL
    #[serde(rename = "coverDeUrl")]
    #[schema(value_type = String, example = "https://web.hycdn.cn/siren/pic/xxx/xxx.jpg")]
    pub cover_de_url: String,

    /// 艺术家列表（注意：API中拼写为"artistes"）
    #[serde(rename = "artistes")]
    pub artists: Vec<String>,
}

/// 专辑响应类型
pub type AlbumResp = ApiResp<AlbumData>;

/// 专辑详情中的歌曲项
///
/// 专辑详情中包含的简化歌曲信息。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct AlbumDetailSongItem {
    /// 歌曲唯一标识符（cid）
    #[serde(rename = "cid")]
    #[schema(value_type = String, example = "953953")]
    pub id: String,

    /// 歌曲名称
    #[schema(value_type = String, example = "Little Wish")]
    pub name: String,

    /// 艺术家列表
    #[serde(rename = "artistes")]
    pub artists: Vec<String>,
}

/// 专辑详情数据
///
/// 包含专辑的完整信息和歌曲列表。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct AlbumDetailData {
    /// 专辑唯一标识符（cid）
    #[serde(rename = "cid")]
    #[schema(value_type = String, example = "953953")]
    pub id: String,

    /// 专辑名称
    #[schema(value_type = String, example = "Little Wish")]
    pub name: String,

    /// 专辑简介
    #[schema(value_type = String, example = "一触即碎的肥皂泡，也要托起小小愿望，飞越风雨，飞向太阳，绽放她的幻彩流光。")]
    pub intro: String,

    /// 所属分类
    #[schema(value_type = String, example = "arknights")]
    pub belong: String,

    /// 封面图片URL
    #[serde(rename = "coverUrl")]
    #[schema(value_type = String, example = "https://web.hycdn.cn/siren/pic/xxx/xxx.jpg")]
    pub cover_url: String,

    /// 详情页封面URL
    #[serde(rename = "coverDeUrl")]
    #[schema(value_type = String, example = "https://web.hycdn.cn/siren/pic/xxx/xxx.jpg")]
    pub cover_de_url: String,

    /// 专辑中的歌曲列表
    pub songs: Vec<AlbumDetailSongItem>,
}

/// 专辑详情响应类型
pub type AlbumDetailResp = ApiResp<AlbumDetailData>;

/// 所有专辑列表单项
///
/// 简化版的专辑信息，用于列表展示。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct AllAlbumsItem {
    /// 专辑唯一标识符（cid）
    #[serde(rename = "cid")]
    #[schema(value_type = String, example = "3888")]
    pub id: String,

    /// 专辑名称
    #[schema(value_type = String, example = "Little Wish")]
    pub name: String,

    /// 封面图片URL
    #[serde(rename = "coverUrl")]
    #[schema(value_type = String, example = "https://web.hycdn.cn/siren/pic/xxx/xxx.jpg")]
    pub cover_url: String,

    /// 艺术家列表
    #[serde(rename = "artistes")]
    pub artists: Vec<String>,
}

/// 搜索结果中的专辑项
///
/// 搜索结果中的专辑信息。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct SearchAlbumItem {
    /// 专辑唯一标识符（cid）
    #[serde(rename = "cid")]
    #[schema(value_type = String, example = "3888")]
    pub id: String,

    /// 专辑名称
    #[schema(value_type = String, example = "Little Wish")]
    pub name: String,

    /// 所属分类
    #[schema(value_type = String, example = "arknights")]
    pub belong: String,

    /// 封面图片URL
    #[serde(rename = "coverUrl")]
    #[schema(value_type = String, example = "https://web.hycdn.cn/siren/pic/xxx/xxx.jpg")]
    pub cover_url: String,

    /// 艺术家列表
    #[serde(rename = "artistes")]
    pub artists: Vec<String>,
}

/// 搜索结果中的专辑数据
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct SearchAlbumData {
    /// 专辑列表
    pub list: Vec<SearchAlbumItem>,

    /// 是否已到达列表末尾
    pub end: bool,
}

/// 搜索专辑查询参数
#[derive(Serialize, Deserialize, Debug, IntoParams)]
pub struct SearchAlbumQuery {
    /// 搜索关键词
    pub keyword: String,

    /// 分页参数，从指定cid之后开始获取
    #[serde(rename = "lastCid")]
    pub last_cid: Option<String>,
}

/// 搜索专辑响应类型
pub type SearchAlbumResp = ApiResp<SearchAlbumData>;

/// 新闻项
///
/// 简化版的新闻信息，用于列表展示。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct NewsItem {
    /// 新闻唯一标识符（cid）
    #[serde(rename = "cid")]
    pub id: String,

    /// 新闻标题
    pub title: String,

    /// 分类ID
    pub cate: i32,

    /// 发布日期
    #[schema(value_type = String, example = "2022-01-01")]
    pub date: String,
}

/// 新闻数据
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct NewsData {
    /// 新闻列表
    pub list: Vec<NewsItem>,

    /// 是否已到达列表末尾
    pub end: bool,
}

/// 搜索新闻查询参数
#[derive(Serialize, Deserialize, Debug, IntoParams)]
pub struct NewsQuery {
    /// 搜索关键词
    pub keyword: String,

    /// 分页参数，从指定cid之后开始获取
    #[serde(rename = "lastCid")]
    pub last_cid: Option<String>,
}

/// 获取所有新闻查询参数
#[derive(Serialize, Deserialize, Debug, IntoParams)]
pub struct AllNewsQuery {
    /// 分页参数，从指定cid之后开始获取
    #[serde(rename = "lastCid")]
    pub last_cid: Option<String>,
}

/// 搜索新闻响应类型
pub type SearchNewsResp = ApiResp<NewsData>;

/// 搜索查询参数
#[derive(Serialize, Deserialize, Debug, IntoParams)]
pub struct SearchQuery {
    /// 搜索关键词
    pub keyword: String,
}

/// 搜索结果数据
///
/// 包含专辑和新闻的搜索结果。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct SearchData {
    /// 专辑搜索结果
    pub albums: SearchAlbumData,

    /// 新闻搜索结果
    pub news: NewsData,
}

/// 综合搜索响应类型
pub type SearchResp = ApiResp<SearchData>;

/// 新闻详情数据
///
/// 包含新闻的完整内容。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct NewsDetailData {
    /// 新闻唯一标识符（cid）
    #[serde(rename = "cid")]
    pub id: String,

    /// 新闻标题
    pub title: String,

    /// 分类ID
    pub cate: i32,

    /// 作者
    pub author: String,

    /// 新闻内容
    pub content: String,

    /// 发布日期
    pub date: String,
}

/// 新闻详情响应类型
pub type NewsDetailResp = ApiResp<NewsDetailData>;

/// 字体文件项
///
/// 包含不同格式的字体文件URL。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct FontItem {
    /// TrueType字体文件URL
    pub tt: String,

    /// Embedded OpenType字体文件URL
    pub eot: String,

    /// SVG字体文件URL
    pub svg: String,

    /// Web Open Font Format字体文件URL
    pub woff: String,
}

/// 字体数据
///
/// 包含所有可用的字体配置。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, ToSchema)]
pub struct FontData {
    /// 常规无衬线字体
    #[serde(rename = "Sans-Regular")]
    pub sans_regular: FontItem,

    /// 粗体无衬线字体
    #[serde(rename = "Sans-Bold")]
    pub sans_bold: FontItem,
}

/// 字体响应类型
pub type FontResp = ApiResp<FontData>;
