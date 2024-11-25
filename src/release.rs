use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::{BotifactoryError, Result},
    identifier::{self, Identifier},
    ChannelAPI,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ReleaseResponse {
    pub id: i64,
    pub version: String,
    pub hash: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
}

async fn release_by_url(url: Url) -> Result<ReleaseResponse> {
    Ok(reqwest::get(url).await?.json::<ReleaseResponse>().await?)
}

pub struct ReleaseAPI {
    channel: ChannelAPI,
    identifier: Identifier,
}

impl ReleaseAPI {
    pub fn new(channel: ChannelAPI, identifier: Identifier) -> Self {
        ReleaseAPI {
            channel,
            identifier,
        }
    }

    pub fn latest_release_url(&self) -> Result<Url> {
        todo!()
    }

    pub async fn get_latest_release(&self) -> Result<ReleaseResponse> {
        release_by_url(self.latest_release_url()?).await
    }

    pub fn previous_release_url(&self) -> Result<Url> {
        todo!()
    }

    pub async fn get_previous_release(&self) -> Result<ReleaseResponse> {
        release_by_url(self.previous_release_url()?).await
    }

    pub fn get_release_by_id_url(&self) -> Result<Url> {
        todo!()
    }

    pub async fn get_release_by_id(&self) -> Result<ReleaseResponse> {
        release_by_url(self.get_release_by_id_url()?).await
    }

    pub fn create_release_url(&self) -> Result<Url> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_latest_release_url() {
        todo!()
    }

    #[test]
    pub fn test_get_previous_release_url() {
        todo!()
    }

    #[test]
    pub fn test_get_release_by_id_url() {
        todo!()
    }

    #[test]
    pub fn test_new_release_url() {
        todo!()
    }
}
