use reqwest::Error;
use yum::{fetch_yum_repository_path, fetch_yum_updates, YumUpdate};

pub fn get_url(path: &str) -> String {
    format!(
        "https://repo.mongodb.org/yum/redhat/8/mongodb-org/7.0/x86_64/{}",
        path
    )
}

pub async fn fetch_mongodb_updates() -> Result<Vec<YumUpdate>, Error> {
    let path = fetch_yum_repository_path(&get_url("repodata/repomd.xml")).await?;
    Ok(fetch_yum_updates(&get_url(&path)).await?)
}
