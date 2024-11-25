use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::{BotifactoryError, Result},
    identifier::Identifier,
    util::*,
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

    pub async fn get_release_by_id(&self) -> Result<ReleaseResponse> {
        release_by_url(self.get_release_by_id_url()?).await
    }

    pub fn create_release_url(&self) -> Result<Url> {
        todo!()
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
}
