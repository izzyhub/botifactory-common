use reqwest::multipart;
use url::Url;

use crate::{
    error::{BotifactoryError, Result},
    util::*,
    Botifactory, NewRelease, ReleaseAPI,
};
use botifactory_types::{ChannelBody, ReleaseBody};

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
                let mut url = self.base.url.clone();
                url.path_segments_mut()
                    .map_err(|_| BotifactoryError::URLPathError)?
                    .push(&self.base.project_name)
                    .push(name);
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
    pub async fn get_channel(&self) -> Result<ChannelBody> {
        Ok(reqwest::get(self.get_channel_url()?)
            .await?
            .json::<ChannelBody>()
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

    pub async fn get_latest_release(&self) -> Result<ReleaseBody> {
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

    pub async fn new_release(self, new_release: NewRelease) -> Result<(ReleaseBody, ReleaseAPI)> {
        let url = self.new_release_url()?;

        let form = multipart::Form::new()
            .part("version", multipart::Part::text(new_release.version))
            .file("binary", new_release.path)
            .await?;

        let client = reqwest::Client::new();
        let create_response = client
            .post(url)
            .multipart(form)
            .send()
            .await?
            .json::<ReleaseBody>()
            .await?;
        let identifier = Identifier::Id(create_response.release.id);
        Ok((create_response, ReleaseAPI::new(self, identifier)))
    }

    pub async fn get_previous_release(&self) -> Result<ReleaseBody> {
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
