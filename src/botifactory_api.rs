use botifactory_types::{ChannelBody, CreateChannel, CreateProject, ProjectBody};
use crate::{
    error::{BotifactoryError, Result},
    util::*,
    ChannelAPI,
};
use url::Url;

pub struct Botifactory {
    pub url: Url,
    pub project_name: String,
}

impl Botifactory {
    pub fn new(url: Url, project_name: &str) -> Botifactory {
        Botifactory {
            url,
            project_name: project_name.to_string(),
        }
    }

    pub fn new_project_url(&self) -> Result<Url> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| BotifactoryError::URLPathError)?
            .push("project")
            .push("new");
        Ok(url)
    }

    pub fn create_channel_url(&self) -> Result<Url> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| BotifactoryError::URLPathError)?
            .push(&self.project_name)
            .push("channel")
            .push("new");
        Ok(url)
    }

    pub async fn new_channel(self, channel_name: &str) -> Result<(ChannelBody, ChannelAPI)> {
        let request_body = CreateChannel::new(channel_name);
        let url = self.create_channel_url()?;

        let client = reqwest::Client::new();
        let create_response = client
            .post(url)
            .json(&request_body)
            .send()
            .await?
            .json::<ChannelBody>()
            .await?;

        let identifier = Identifier::Id(create_response.channel.id);
        Ok((create_response, ChannelAPI::new(self, identifier)))
    }

    pub async fn new_project(&self, project_name: &str) -> Result<(ProjectBody, Botifactory)> {
        let request_body = CreateProject::new(project_name);

        let client = reqwest::Client::new();

        let create_response = client
            .post(self.new_project_url()?)
            .json(&request_body)
            .send()
            .await?
            .json::<ProjectBody>()
            .await?;

        Ok((
            create_response,
            Botifactory::new(self.url.clone(), project_name),
        ))
    }

    pub fn get_project_url(&self) -> Result<Url> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| BotifactoryError::URLPathError)?
            .push("project")
            .push(&self.project_name);
        Ok(url)
    }

    pub async fn get_project(&self) -> Result<ProjectBody> {
        Ok(reqwest::get(self.get_project_url()?)
            .await?
            .json::<ProjectBody>()
            .await?)
    }

    pub fn channel(self, identifier: Identifier) -> ChannelAPI {
        ChannelAPI::new(self, identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new_project_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let new_project_url = Botifactory::new(base_url, "test-project")
            .new_project_url()
            .expect("Expected to get a valid new project url");
        assert_eq!(
            new_project_url.to_string(),
            "https://botifactory.example.com/project/new".to_string()
        )
    }

    #[test]
    pub fn create_channel_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let channel_url = Botifactory::new(base_url, "test-project")
            .create_channel_url()
            .expect("Expected to get a valid project url");
        assert_eq!(
            channel_url.to_string(),
            "https://botifactory.example.com/test-project/channel/new".to_string()
        )
    }

    #[test]
    pub fn test_project_url() {
        let base_url =
            Url::parse("https://botifactory.example.com").expect("Expected a valid test url");
        let project_url = Botifactory::new(base_url, "test-project")
            .get_project_url()
            .expect("Expected to get a valid project url");
        assert_eq!(
            project_url.to_string(),
            "https://botifactory.example.com/project/test-project".to_string()
        )
    }
}
