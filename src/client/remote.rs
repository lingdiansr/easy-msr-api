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

    pub async fn get_song(&self, id: i32) -> Result<SongResp, AppError> {
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
}
