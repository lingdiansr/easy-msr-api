use crate::error::AppError;
use crate::web::dto::{CreateUserRequest, User};
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

    pub async fn create_user(&self, req: &CreateUserRequest) -> Result<User, AppError> {
        let url = format!("{}/users", self.base);
        Ok(self
            .inner
            .post(&url)
            .json(req)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn get_user(&self, id: i32) -> Result<User, AppError> {
        let url = format!("{}/users/{}", self.base, id);
        Ok(self
            .inner
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
    pub async fn get_song(&self, id: &str) -> Result<SongResp, AppError> {
        let url = format!("{}/song/{}", self.base, id);
        Ok(self.inner.get(&url).send().await?.json().await?)
    }
}
