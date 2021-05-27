use serde::Deserialize;

#[derive(Debug)]
pub struct ChromeUpdate {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct RepositoryLocation {
    #[serde()]
    pub href: String,
}

#[derive(Deserialize, Debug)]
pub struct RepositoryData {
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "location")]
    pub location: RepositoryLocation,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "repomd")]
pub struct RepositoryMetadata {
    #[serde(rename = "data")]
    pub repositories: Vec<RepositoryData>,
}
