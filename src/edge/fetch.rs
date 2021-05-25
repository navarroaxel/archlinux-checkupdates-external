use crate::chrome::ChromeUpdate;
use crate::edge::model::{ChromeRepository, RepositoryMetadata};
use itertools::Itertools;
use libflate::gzip::Decoder;
use reqwest::{Error, Response};
use std::io::Read;

async fn inflate_response(response: Response) -> Result<String, Error> {
    let body = response.bytes().await?;
    let mut decoder = Decoder::new(&body[..]).unwrap();
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    Ok(decoded.iter().map(|&c| c as char).collect::<String>())
}

pub fn get_url(path: &str) -> String {
    format!("https://packages.microsoft.com/yumrepos/edge/{}", path)
}

pub async fn fetch_edge_repository_url() -> Result<String, Error> {
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
    let location = &repository.location.href;
    Ok(location.clone())
}

pub async fn fetch_edge_updates() -> Result<Vec<ChromeUpdate>, Error> {
    let url = fetch_edge_repository_url().await?;
    let response = inflate_response(reqwest::get(get_url(&url)).await?).await?;
    let repository: ChromeRepository = serde_xml_rs::from_str(&response).unwrap();

    let updates = repository
        .packages
        .iter()
        .rev()
        .unique_by(|pkg| &pkg.name)
        .map(|pkg| ChromeUpdate {
            name: pkg.name.clone(),
            version: pkg.versions.first().unwrap().version.clone(),
        })
        .collect::<Vec<ChromeUpdate>>();

    Ok(updates)
}
