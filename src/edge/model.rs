use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "ver")]
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    #[serde(rename = "version")]
    pub versions: Vec<Version>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "otherdata")]
pub struct ChromeRepository {
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

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
