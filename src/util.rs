use crate::error::Result;
use botifactory_types::ReleaseBody;
use url::Url;

pub async fn release_by_url(url: Url) -> Result<ReleaseBody> {
    Ok(reqwest::get(url).await?.json::<ReleaseBody>().await?)
}

#[derive(Debug)]
pub enum Identifier {
    Name(String),
    Id(i64),
}
