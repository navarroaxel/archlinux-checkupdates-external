use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AurPackage {
    pub name: String,
    pub version: String
}

#[derive(Deserialize, Debug)]
pub struct AurResponse {
    pub version: u32,
    #[serde(rename="resultcount")]
    pub result_count: u32,
    pub results: Vec<AurPackage>
}

impl AurPackage {
    pub fn get_package_version(&self) -> String {
        let mut splitter = self.version.split("-");
        splitter.next().unwrap().to_string()
    }
}
