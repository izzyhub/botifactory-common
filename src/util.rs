use crate::{error::Result, ReleaseResponse};
use url::Url;

pub async fn release_by_url(url: Url) -> Result<ReleaseResponse> {
    Ok(reqwest::get(url).await?.json::<ReleaseResponse>().await?)
}
