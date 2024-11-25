use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::{BotifactoryError, Result},
    identifier::{self, Identifier},
    Botifactory, ReleaseAPI,
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
}
