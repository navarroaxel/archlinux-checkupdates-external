use crate::model::{RepositoryMetadata, YumPackage, YumRepository, YumUpdate};
use itertools::Itertools;
use libflate::gzip::Decoder;
use reqwest::{Error, Response};
use semver::Version;
use std::cmp::Ordering;
use std::io::Read;

async fn inflate_response(response: Response) -> Result<String, Error> {
    let body = response.bytes().await?;
    let mut decoder = Decoder::new(&body[..]).unwrap();
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    Ok(decoded.iter().map(|&c| c as char).collect::<String>())
}

pub async fn fetch_yum_repository_path(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?.text().await?;

    let repository_metadata: RepositoryMetadata = serde_xml_rs::from_str(&response).unwrap();

    let repository = repository_metadata
        .repositories
        .iter()
        .find(|repo| repo.data_type == "other")
        .unwrap();

    Ok((repository.location.href).clone())
}

fn compare_versions(a: &YumPackage, b: &YumPackage) -> Ordering {
    Version::parse(&a.semver())
        .unwrap()
        .cmp(&Version::parse(&b.semver()).unwrap())
}

pub async fn fetch_yum_updates(url: &str) -> Result<Vec<YumUpdate>, Error> {
    let response = inflate_response(reqwest::get(url).await?).await?;
    let repository: YumRepository = serde_xml_rs::from_str(&response).unwrap();

    let updates = repository
        .packages
        .iter()
        .sorted_by(|a, b| compare_versions(b, a))
        .unique_by(|pkg| &pkg.name)
        .map(|pkg| YumUpdate {
            name: pkg.name.clone(),
            version: pkg.versions.first().unwrap().version.clone(),
        })
        .collect::<Vec<YumUpdate>>();

    Ok(updates)
}
