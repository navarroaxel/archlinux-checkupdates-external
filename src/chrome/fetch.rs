use crate::yum::{fetch_yum_updates, YumUpdate};
use reqwest::Error;

pub async fn fetch_chrome_updates() -> Result<Vec<YumUpdate>, Error> {
    Ok(fetch_yum_updates(
        "https://dl.google.com/linux/chrome/rpm/stable/x86_64/repodata/other.xml.gz",
    )
    .await?)
}
