pub mod client;
pub mod config;
pub mod error;
pub mod web;



#[cfg(not(feature = "swagger-ui"))]
use crate::{client::remote::RemoteApiClient, web::dto::*, error::AppError};
// 默认的MSR API客户端，使用官方API地址
#[cfg(not(feature = "swagger-ui"))]
pub struct MSRApiClient {
    inner: RemoteApiClient,
}

#[cfg(not(feature = "swagger-ui"))]
impl MSRApiClient {
    /// 创建默认的MSR API客户端，使用官方API地址
    pub fn new() -> Self {
        Self {
            inner: RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string()),
        }
    }

    /// 使用自定义API地址
    pub fn with_base(base: String) -> Self {
        Self {
            inner: RemoteApiClient::new(base),
        }
    }

    // 代理所有API方法
    pub async fn get_song(&self, id: String) -> Result<SongResp, AppError> {
        self.inner.get_song(id).await
    }

    pub async fn get_all_songs(&self) -> Result<AllSongsResp, AppError> {
        self.inner.get_all_songs().await
    }

    pub async fn get_album(&self, id: String) -> Result<AlbumResp, AppError> {
        self.inner.get_album(id).await
    }

    pub async fn get_album_detail(&self, id: String) -> Result<AlbumDetailResp, AppError> {
        self.inner.get_album_detail(id).await
    }

    pub async fn get_all_albums(&self) -> Result<ApiResp<Vec<AllAlbumsItem>>, AppError> {
        self.inner.get_all_albums().await
    }

    pub async fn get_all_news(&self, last_cid: Option<String>) -> Result<SearchNewsResp, AppError> {
        self.inner.get_all_news(last_cid).await
    }

    pub async fn get_news_detail(&self, id: String) -> Result<NewsDetailResp, AppError> {
        self.inner.get_news_detail(id).await
    }

    pub async fn get_font(&self) -> Result<FontResp, AppError> {
        self.inner.get_font().await
    }

    pub async fn search(&self, keyword: String) -> Result<SearchResp, AppError> {
        self.inner.search(keyword).await
    }

    pub async fn search_albums(
        &self,
        keyword: String,
        last_cid: Option<String>,
    ) -> Result<SearchAlbumResp, AppError> {
        self.inner.search_albums(keyword, last_cid).await
    }

    pub async fn search_news(
        &self,
        keyword: String,
        last_cid: Option<String>,
    ) -> Result<SearchNewsResp, AppError> {
        self.inner.search_news(keyword, last_cid).await
    }
}
