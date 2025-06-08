use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Build {
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@fullNumber")]
    pub full_number: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "build")]
    pub builds: Vec<Build>,
}

#[derive(Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "channel", default)]
    pub channels: Vec<Channel>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "products")]
pub struct JetBrainsRepository {
    #[serde(rename = "product")]
    pub products: Vec<Product>,
}
