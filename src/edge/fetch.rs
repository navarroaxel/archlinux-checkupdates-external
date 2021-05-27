use crate::edge::model::{RepositoryMetadata};
use crate::yum::{fetch_yum_updates, YumUpdate};
use reqwest::{Error};

pub fn get_url(path: &str) -> String {
    format!("https://packages.microsoft.com/yumrepos/edge/{}", path)
}

pub async fn fetch_edge_repository_path() -> Result<String, Error> {
    let response = reqwest::get(get_url("repodata/repomd.xml"))
        .await?
        .text()
        .await?;

    let repository_metadata: RepositoryMetadata = serde_xml_rs::from_str(&response).unwrap();

    let repository = repository_metadata
        .repositories
        .iter()
        .find(|repo| repo.data_type == "other")
        .unwrap();

    Ok((repository.location.href).clone())
}

pub async fn fetch_edge_updates() -> Result<Vec<YumUpdate>,Error> {
    let path = fetch_edge_repository_path().await?;
    Ok(fetch_yum_updates(&get_url(&path)).await?)
}
