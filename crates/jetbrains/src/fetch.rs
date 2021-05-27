use crate::JetBrainsRepository;
use reqwest::Error;

pub async fn fetch_jetbrains_updates() -> Result<JetBrainsRepository, Error> {
    let res = reqwest::get("https://www.jetbrains.com/updates/updates.xml")
        .await?
        .text()
        .await?;
    let repository: JetBrainsRepository = serde_xml_rs::from_str(&res).unwrap();
    Ok(repository)
}
