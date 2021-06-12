use itertools::Itertools;
use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct YumVersion {
    #[serde(rename = "ver")]
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct YumPackage {
    pub name: String,
    #[serde(rename = "version")]
    pub versions: Vec<YumVersion>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "otherdata")]
pub struct YumRepository {
    #[serde(rename = "package")]
    pub packages: Vec<YumPackage>,
}

#[derive(Debug)]
pub struct YumUpdate {
    pub name: String,
    pub version: String,
}

impl YumPackage {
    pub fn version(&self) -> String {
        self.versions.first().unwrap().version.clone()
    }

    pub fn semver(&self) -> String {
        let version = self.version();
        let mut groups = version.split(".").collect_vec();
        if groups.len() <= 3 {
            return version;
        }

        // remove 2nd group for chromium versions, it's always 0.
        groups.remove(1);
        let result = groups.join(".").clone();
        result
    }
}
