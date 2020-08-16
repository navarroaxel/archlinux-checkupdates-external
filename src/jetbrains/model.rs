use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Build {
    pub number: String,
    pub version: String,
    #[serde(rename = "fullNumber")]
    pub full_number: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "build")]
    pub builds: Vec<Build>,
}

#[derive(Deserialize, Debug)]
pub struct Product {
    pub name: String,
    #[serde(rename = "channel")]
    pub channels: Vec<Channel>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "products")]
pub struct JetBrainsRepository {
    #[serde(rename = "product")]
    pub products: Vec<Product>,
}
