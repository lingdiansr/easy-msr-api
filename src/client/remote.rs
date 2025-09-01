//! # 远程API客户端
//! 
//! 提供与MSR API进行HTTP通信的客户端实现。
//! 
//! 该模块包含`RemoteApiClient`结构体，它封装了HTTP客户端并提供了
//! 所有MSR API的调用方法。

use crate::error::AppError;
use crate::dto::*;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

/// 远程API客户端
/// 
/// 封装了HTTP客户端和API基础URL，提供所有MSR API的调用方法。
/// 支持自定义超时时间和基础URL配置。
#[derive(Clone)]
pub struct RemoteApiClient {
    inner: Client,
    base: String,
}

impl RemoteApiClient {
    /// 创建默认配置的客户端
    /// 
    /// 使用30秒的超时时间和默认的用户代理字符串。
    /// 
    /// # 参数
    /// 
    /// * `base` - API的基础URL地址
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use msr_api_rs::client::remote::RemoteApiClient;
    /// 
    /// let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
    /// ```
    pub fn new(base: String) -> Self {
        Self::with_config(base, Duration::from_secs(30))
    }

    /// 使用自定义超时时间创建客户端
    /// 
    /// # 参数
    /// 
    /// * `base` - API的基础URL地址
    /// * `timeout` - HTTP请求超时时间
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use std::time::Duration;
    /// use msr_api_rs::client::remote::RemoteApiClient;
    /// 
    /// let client = RemoteApiClient::with_config(
    ///     "https://monster-siren.hypergryph.com/api".to_string(),
    ///     Duration::from_secs(60)
    /// );
    /// ```
    pub fn with_config(base: String, timeout: Duration) -> Self {
        let client = ClientBuilder::new()
            .timeout(timeout)
            .user_agent(format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
            .build()
            .expect("Failed to build HTTP client");
        
        Self {
            inner: client,
            base: base.trim_end_matches('/').to_string(),
        }
    }

    /// 统一的请求发送方法，减少代码重复
    /// 
    /// 内部使用的辅助方法，用于发送GET请求并解析响应。
    /// 
    /// # 类型参数
    /// 
    /// * `T` - 响应数据的类型，必须实现`DeserializeOwned`
    /// 
    /// # 参数
    /// 
    /// * `path` - API路径（相对于基础URL）
    /// * `query` - 查询参数列表
    /// 
    /// # 返回
    /// 
    /// 返回解析后的响应数据或错误
    async fn send_get_request<T>(&self, path: &str, query: &[(&str, &str)]) -> Result<T, AppError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.base, path.trim_start_matches('/'));
        
        let response = self.inner
            .get(&url)
            .query(query)
            .send()
            .await?
            .error_for_status()?;
            
        Ok(response.json().await?)
    }

    /// 获取指定ID的歌曲详情
    /// 
    /// # 参数
    /// 
    /// * `id` - 歌曲的唯一标识符（cid）
    /// 
    /// # 返回
    /// 
    /// 返回包含歌曲详细信息的响应
    pub async fn get_song(&self, id: String) -> Result<SongResp, AppError> {
        self.send_get_request(&format!("song/{}", id), &[]).await
    }

    /// 获取所有歌曲列表
    /// 
    /// # 返回
    /// 
    /// 返回包含所有歌曲基本信息的列表
    pub async fn get_all_songs(&self) -> Result<AllSongsResp, AppError> {
        self.send_get_request("songs", &[]).await
    }

    /// 获取指定ID的专辑信息
    /// 
    /// # 参数
    /// 
    /// * `id` - 专辑的唯一标识符（cid）
    /// 
    /// # 返回
    /// 
    /// 返回包含专辑详细信息的响应
    pub async fn get_album(&self, id: String) -> Result<AlbumResp, AppError> {
        self.send_get_request(&format!("album/{}/data", id), &[]).await
    }

    /// 获取指定ID的专辑详情（包含歌曲列表）
    /// 
    /// # 参数
    /// 
    /// * `id` - 专辑的唯一标识符（cid）
    /// 
    /// # 返回
    /// 
    /// 返回包含专辑详情和歌曲列表的响应
    pub async fn get_album_detail(&self, id: String) -> Result<AlbumDetailResp, AppError> {
        self.send_get_request(&format!("album/{}/detail", id), &[]).await
    }

    /// 获取所有专辑列表
    /// 
    /// # 返回
    /// 
    /// 返回包含所有专辑基本信息的列表
    pub async fn get_all_albums(&self) -> Result<ApiResp<Vec<AllAlbumsItem>>, AppError> {
        self.send_get_request("albums", &[]).await
    }

    /// 获取所有新闻列表
    /// 
    /// # 参数
    /// 
    /// * `last_cid` - 可选参数，用于分页，从指定cid之后开始获取
    /// 
    /// # 返回
    /// 
    /// 返回包含新闻列表的响应
    pub async fn get_all_news(&self, last_cid: Option<String>) -> Result<SearchNewsResp, AppError> {
        let mut query = Vec::new();
        if let Some(cid) = &last_cid {
            query.push(("lastCid", cid.as_str()));
        }
        self.send_get_request("news", &query).await
    }

    /// 获取指定ID的新闻详情
    /// 
    /// # 参数
    /// 
    /// * `id` - 新闻的唯一标识符（cid）
    /// 
    /// # 返回
    /// 
    /// 返回包含新闻详细内容的响应
    pub async fn get_news_detail(&self, id: String) -> Result<NewsDetailResp, AppError> {
        self.send_get_request(&format!("news/{}", id), &[]).await
    }

    /// 获取字体配置信息
    /// 
    /// # 返回
    /// 
    /// 返回包含字体文件URL配置的响应
    pub async fn get_font(&self) -> Result<FontResp, AppError> {
        self.send_get_request("fontset", &[]).await
    }

    /// 综合搜索（同时搜索专辑和新闻）
    /// 
    /// # 参数
    /// 
    /// * `keyword` - 搜索关键词
    /// 
    /// # 返回
    /// 
    /// 返回包含专辑和新闻搜索结果的响应
    pub async fn search(&self, keyword: String) -> Result<SearchResp, AppError> {
        self.send_get_request("search", &[("keyword", &keyword)]).await
    }

    /// 搜索专辑
    /// 
    /// # 参数
    /// 
    /// * `keyword` - 搜索关键词
    /// * `last_cid` - 可选参数，用于分页
    /// 
    /// # 返回
    /// 
    /// 返回包含专辑搜索结果的响应
    pub async fn search_albums(
        &self,
        keyword: String,
        last_cid: Option<String>,
    ) -> Result<SearchAlbumResp, AppError> {
        let mut query = vec![("keyword", keyword.as_str())];
        if let Some(cid) = &last_cid {
            query.push(("lastCid", cid.as_str()));
        }
        self.send_get_request("search/album", &query).await
    }

    /// 搜索新闻
    /// 
    /// # 参数
    /// 
    /// * `keyword` - 搜索关键词
    /// * `last_cid` - 可选参数，用于分页
    /// 
    /// # 返回
    /// 
    /// 返回包含新闻搜索结果的响应
    pub async fn search_news(
        &self,
        keyword: String,
        last_cid: Option<String>,
    ) -> Result<SearchNewsResp, AppError> {
        let mut query = vec![("keyword", keyword.as_str())];
        if let Some(cid) = &last_cid {
            query.push(("lastCid", cid.as_str()));
        }
        self.send_get_request("search/news", &query).await
    }
}
