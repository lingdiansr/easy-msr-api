use crate::error::AppError;
use crate::web::dto::*;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

#[derive(Clone)]
pub struct RemoteApiClient {
    inner: Client,
    base: String,
}

impl RemoteApiClient {
    /// 创建默认配置的客户端
    pub fn new(base: String) -> Self {
        Self::with_config(base, Duration::from_secs(30))
    }

    /// 使用自定义超时时间创建客户端
    pub fn with_config(base: String, timeout: Duration) -> Self {
        let client = ClientBuilder::new()
            .timeout(timeout)
            .user_agent(format!("{}/{}",env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
            .build()
            .expect("Failed to build HTTP client");
        
        Self {
            inner: client,
            base: base.trim_end_matches('/').to_string(),
        }
    }

    /// 统一的请求发送方法，减少代码重复
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

    pub async fn get_song(&self, id: String) -> Result<SongResp, AppError> {
        self.send_get_request(&format!("song/{}", id), &[]).await
    }

    pub async fn get_all_songs(&self) -> Result<AllSongsResp, AppError> {
        self.send_get_request("songs", &[]).await
    }

    pub async fn get_album(&self, id: String) -> Result<AlbumResp, AppError> {
        self.send_get_request(&format!("album/{}/data", id), &[]).await
    }

    pub async fn get_album_detail(&self, id: String) -> Result<AlbumDetailResp, AppError> {
        self.send_get_request(&format!("album/{}/detail", id), &[]).await
    }

    pub async fn get_all_albums(&self) -> Result<ApiResp<Vec<AllAlbumsItem>>, AppError> {
        self.send_get_request("albums", &[]).await
    }

    pub async fn get_all_news(&self, last_cid: Option<String>) -> Result<SearchNewsResp, AppError> {
        let mut query = Vec::new();
        if let Some(cid) = &last_cid {
            query.push(("lastCid", cid.as_str()));
        }
        self.send_get_request("news", &query).await
    }

    pub async fn get_news_detail(&self, id: String) -> Result<NewsDetailResp, AppError> {
        self.send_get_request(&format!("news/{}", id), &[]).await
    }

    pub async fn get_font(&self) -> Result<FontResp, AppError> {
        self.send_get_request("fontset", &[]).await
    }

    pub async fn search(&self, keyword: String) -> Result<SearchResp, AppError> {
        self.send_get_request("search", &[("keyword", &keyword)]).await
    }

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
