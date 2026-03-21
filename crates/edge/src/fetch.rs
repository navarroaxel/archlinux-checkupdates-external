use reqwest::Error;
use yum::{YumUpdate, fetch_yum_repository_path, fetch_yum_updates};

pub fn get_url(path: &str) -> String {
    format!("https://packages.microsoft.com/yumrepos/edge/{}", path)
}

pub async fn fetch_edge_updates() -> Result<Vec<YumUpdate>, Error> {
    let path = fetch_yum_repository_path(&get_url("repodata/repomd.xml")).await?;
    Ok(fetch_yum_updates(&get_url(&path)).await?)
}
