use crate::chrome::model::{ChromeRepository, ChromeUpdate};
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

pub async fn fetch_chrome_updates() -> Result<Vec<ChromeUpdate>, Error> {
    let response = inflate_response(
        reqwest::get("https://dl.google.com/linux/chrome/rpm/stable/x86_64/repodata/other.xml.gz").await?
    ).await?;
    let repository: ChromeRepository = serde_xml_rs::from_str(&response).unwrap();

    let updates = repository.packages.iter()
        .map(| pkg| ChromeUpdate {
            name: pkg.name.clone(),
            version: pkg.versions.first().unwrap().version.clone()
        })
        .collect::<Vec<ChromeUpdate>>();

    Ok(updates)
}
