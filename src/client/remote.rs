use crate::error::AppError;
use crate::web::dto::*;
use reqwest::Client;

#[derive(Clone)]
pub struct RemoteApiClient {
    inner: Client,
    base: String,
}

impl RemoteApiClient {
    pub fn new(base: String) -> Self {
        Self {
            inner: Client::new(),
            base,
        }
    }

    pub async fn get_song(&self, id: String) -> Result<SongResp, AppError> {
        let url = format!("{}/song/{}", self.base, id);
        Ok(self
            .inner
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn get_all_songs(&self) -> Result<AllSongsResp, AppError> {
        let url = format!("{}/songs", self.base);
        Ok(self
            .inner
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn get_album(&self, id: String) -> Result<AlbumResp, AppError> {
        let url = format!("{}/album/{}/data", self.base, id);
        Ok(self
            .inner
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
    pub async fn get_album_detail(&self, id: String) -> Result<AlbumDetailResp, AppError> {
        let url = format!("{}/album/{}/detail", self.base, id);
        Ok(self
            .inner
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
    pub async fn get_all_albums(&self) -> Result<ApiResp<Vec<AllAlbumsItem>>, AppError> {
        let url = format!("{}/albums", self.base);
        Ok(self
            .inner
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
    pub async fn search(&self, keyword: String) -> Result<SearchResp, AppError> {
        let url = format!("{}/search?keyword={}", self.base, keyword);
        Ok(self
            .inner
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
