use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::{BotifactoryError, Result},
    identifier::Identifier,
    util::*,
    Botifactory, ReleaseAPI, ReleaseResponse,
};

#[derive(Serialize, Deserialize)]
pub struct ChannelJson {
    pub id: i64,
    pub name: String,
    pub project_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateChannel {
    channel_name: String,
}

impl CreateChannel {
    pub fn new(channel_name: &str) -> Self {
        CreateChannel {
            channel_name: channel_name.to_string(),
        }
    }
}

pub struct ChannelAPI {
    pub base: Botifactory,
    identifier: Identifier,
}

impl ChannelAPI {
    pub fn new(base: Botifactory, identifier: Identifier) -> Self {
        ChannelAPI { base, identifier }
    }

    pub fn get_channel_url(&self) -> Result<Url> {
        match &self.identifier {
            Identifier::Name(name) => {
                let mut url = self.base.get_project_url()?;
                url.path_segments_mut()
                    .map_err(|_| BotifactoryError::URLPathError)?
                    .push(&name);
                Ok(url)
            }
            Identifier::Id(id) => {
                let mut url = self.base.url.clone();
                url.path_segments_mut()
                    .map_err(|_| BotifactoryError::URLPathError)?
                    .push("channel")
                    .push(&id.to_string());
                Ok(url)
            }
        }
    }
    pub async fn get_channel(&self) -> Result<ChannelJson> {
        Ok(reqwest::get(self.get_channel_url()?)
            .await?
            .json::<ChannelJson>()
            .await?)
    }

    pub fn release(self, identifier: Identifier) -> ReleaseAPI {
        ReleaseAPI::new(self, identifier)
    }

    pub fn latest_release_url(&self) -> Result<Url> {
        let mut url = self.get_channel_url()?;
        url.path_segments_mut()
            .map_err(|_| BotifactoryError::URLPathError)?
            .push("latest");
        Ok(url)
    }

    pub async fn get_latest_release(&self) -> Result<ReleaseResponse> {
        release_by_url(self.latest_release_url()?).await
    }

    pub fn previous_release_url(&self) -> Result<Url> {
        let mut url = self.get_channel_url()?;
        url.path_segments_mut()
            .map_err(|_| BotifactoryError::URLPathError)?
            .push("previous");
        Ok(url)
    }

    pub fn new_release_url(&self) -> Result<Url> {
        let mut url = self.get_channel_url()?;
        url.path_segments_mut()
            .map_err(|_| BotifactoryError::URLPathError)?
            .push("new");
        Ok(url)
    }

    pub async fn get_previous_release(&self) -> Result<ReleaseResponse> {
        release_by_url(self.previous_release_url()?).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_channel_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let channel_url = Botifactory::new(base_url, "test-project")
            .channel(Identifier::Name("stable".to_string()))
            .get_channel_url()
            .expect("Expected to successfully get a channel url");

        assert_eq!(
            channel_url.to_string(),
            "https://botifactory.example.com/test-project/stable".to_string()
        )
    }

    #[test]
    pub fn test_get_latest_release_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let botifactory_api = Botifactory::new(base_url, "test-project");
        let channel = botifactory_api.channel(Identifier::Name("test".to_string()));
        let latest_release_url = channel
            .latest_release_url()
            .expect("expected to get a valid url");
        assert_eq!(
            "https://botifactory.example.com/test-project/test/latest",
            latest_release_url.to_string()
        );
    }

    #[test]
    pub fn test_get_previous_release_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let botifactory_api = Botifactory::new(base_url, "test-project");
        let channel = botifactory_api.channel(Identifier::Name("test".to_string()));
        let latest_release_url = channel
            .previous_release_url()
            .expect("expected to get a valid url");
        assert_eq!(
            "https://botifactory.example.com/test-project/test/previous",
            latest_release_url.to_string()
        );
    }

    #[test]
    pub fn test_new_release_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let botifactory_api = Botifactory::new(base_url, "test-project");
        let channel = botifactory_api.channel(Identifier::Name("test".to_string()));
        let new_release_url = channel.new_release_url().expect("expected a valid url");
        assert_eq!(
            "https://botifactory.example.com/test-project/test/new",
            new_release_url.to_string()
        );
    }
}
