use bytes::Bytes;
use display_json::DisplayAsJsonPretty;
use reqwest::header::{HeaderMap, ACCEPT};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::PathBuf;
use url::Url;

use crate::{
    error::{BotifactoryError, Result},
    util::*,
    ChannelAPI,
};

#[derive(Serialize, Deserialize, Clone, DisplayAsJsonPretty)]
pub struct ReleaseResponse {
    pub id: i64,
    pub version: String,
    pub hash: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, DisplayAsJsonPretty)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseBody {
    pub release: ReleaseResponse,
}

pub struct NewRelease {
    pub version: String,
    pub path: PathBuf,
}

impl NewRelease {
    pub fn new(version: String, path: PathBuf) -> Self {
        NewRelease { version, path }
    }
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

    pub fn get_release_by_name_url(&self) -> Result<Url> {
        match &self.identifier {
            Identifier::Name(name) => {
                let mut url = self.channel.get_channel_url()?;
                url.path_segments_mut()
                    .map_err(|_| BotifactoryError::URLPathError)?
                    .push(name);
                Ok(url)
            }
            Identifier::Id(_) => Err(BotifactoryError::InvalidIdentifier),
        }
    }

    pub fn get_release_by_id_url(&self) -> Result<Url> {
        match self.identifier {
            Identifier::Id(id) => {
                let mut url = self.channel.base.url.clone();

                url.path_segments_mut()
                    .map_err(|_| BotifactoryError::URLPathError)?
                    .push("release")
                    .push(&id.to_string());
                Ok(url)
            }
            Identifier::Name(_) => Err(BotifactoryError::InvalidIdentifier),
        }
    }

    pub fn release_url(&self) -> Result<Url> {
        match self.identifier {
            Identifier::Id(_) => self.get_release_by_id_url(),
            Identifier::Name(_) => self.get_release_by_name_url(),
        }
    }

    pub async fn release_info(&self) -> Result<ReleaseBody> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse()?);

        Ok(client
            .get(self.release_url()?)
            .headers(headers)
            .send()
            .await?
            .json::<ReleaseBody>()
            .await?)
    }

    pub async fn release_binary(&self) -> Result<Bytes> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/octet-stream".parse()?);

        Ok(client
            .get(self.release_url()?)
            .headers(headers)
            .send()
            .await?
            .bytes()
            .await?)
    }
    pub async fn release_binary_path(&self, path: PathBuf) -> Result<()> {
        let response = reqwest::get(self.release_url()?).await?;

        let mut file = std::fs::File::create(path)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }

    pub async fn get_release_by_id(&self) -> Result<ReleaseBody> {
        release_by_url(self.get_release_by_id_url()?).await
    }
}

#[cfg(test)]
mod tests {
    use crate::Botifactory;

    use super::*;

    #[test]
    pub fn test_get_release_by_id_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let botifactory_api = Botifactory::new(base_url, "test-project");
        let channel = botifactory_api.channel(Identifier::Name("test".to_string()));
        let release = channel.release(Identifier::Id(1));
        let url = release
            .get_release_by_id_url()
            .expect("Expected a valid url");
        assert_eq!("https://botifactory.example.com/release/1", url.to_string());
    }

    #[test]
    pub fn test_get_release_by_name_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let botifactory_api = Botifactory::new(base_url, "test-project");
        let channel = botifactory_api.channel(Identifier::Name("test".to_string()));
        let release = channel.release(Identifier::Name("latest".to_string()));
        let url = release
            .get_release_by_name_url()
            .expect("Expected a valid url");
        assert_eq!(
            "https://botifactory.example.com/test-project/test/latest",
            url.to_string()
        );
    }
}
