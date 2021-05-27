use crate::yum::model::{YumRepository, YumUpdate};
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

pub async fn fetch_yum_updates(url: &str) -> Result<Vec<YumUpdate>, Error> {
    let response = inflate_response(reqwest::get(url).await?).await?;
    let repository: YumRepository = serde_xml_rs::from_str(&response).unwrap();

    let updates = repository
        .packages
        .iter()
        .rev()
        .unique_by(|pkg| &pkg.name)
        .map(|pkg| YumUpdate {
            name: pkg.name.clone(),
            version: pkg.versions.first().unwrap().version.clone(),
        })
        .collect::<Vec<YumUpdate>>();

    Ok(updates)
}
