use crate::aur::{AurPackage, AurResponse};
use reqwest::Error;

pub async fn fetch_aur_packages(packages: Vec<&str>) -> Result<Vec<AurPackage>, Error> {
    let url = format!(
        "https://aur.archlinux.org/rpc/?v=5&type=info&arg[]={package}",
        package = packages.join("&arg[]=")
    );
    let response: AurResponse = reqwest::get(&url).await?.json().await?;
    Ok(response.results)
}
