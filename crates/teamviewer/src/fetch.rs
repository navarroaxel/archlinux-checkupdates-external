use reqwest::Error;
use yum::{fetch_yum_updates, YumUpdate};

pub async fn fetch_teamviewer_updates() -> Result<Vec<YumUpdate>, Error> {
    Ok(fetch_yum_updates(
        "https://linux.teamviewer.com/yum/stable/main/binary-x86_64/repodata/other.xml.gz",
    )
    .await?)
}
