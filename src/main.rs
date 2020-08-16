mod aur;
mod chrome;
mod jetbrains;

use chrome::{fetch_chrome_updates, print_chrome_updates};
use jetbrains::{fetch_jetbrains_updates, print_jetbrains_updates};

use aur::fetch_aur_packages;
use futures::join;
use reqwest::Error;

async fn check_chrome_updates() -> Result<(), Error> {
    let products = vec![
        vec!["google-chrome", "google-chrome-stable"],
        vec!["google-chrome-beta", "google-chrome-beta"],
        vec!["google-chrome-dev", "google-chrome-unstable"],
    ];
    let (updates, packages) = join!(
        fetch_chrome_updates(),
        fetch_aur_packages(products.iter().map(|p| p[0].clone()).collect())
    );
    print_chrome_updates(products, packages.unwrap(), updates.unwrap());
    Ok(())
}

async fn check_jetbrains_updates() -> Result<(), Error> {
    let jetbrains_products = vec![
        vec!["datagrip", "DataGrip", "DB-RELEASE-licensing-RELEASE"],
        vec!["goland", "GoLand", "GO-RELEASE-licensing-RELEASE"],
        vec!["goland-eap", "GoLand", "GO-EAP-licensing-EAP"],
        vec![
            "intellij-idea-ce",
            "IntelliJ IDEA",
            "IC-IU-RELEASE-licensing-RELEASE",
        ],
        vec![
            "intellij-idea-ce-eap",
            "IntelliJ IDEA",
            "IC-IU-EAP-licensing-EAP",
        ],
        vec![
            "intellij-idea-ultimate-edition",
            "IntelliJ IDEA",
            "IC-IU-RELEASE-licensing-RELEASE",
        ],
        vec![
            "intellij-idea-ue-eap",
            "IntelliJ IDEA",
            "IC-IU-EAP-licensing-EAP",
        ],
        vec!["phpstorm", "PhpStorm", "PS-RELEASE-licensing-RELEASE"],
        vec!["phpstorm-eap", "PhpStorm", "PS-EAP-licensing-EAP"],
        vec![
            "pycharm-community-eap",
            "PyCharm",
            "PC-PY-EAP-licensing-EAP",
        ],
        vec!["pycharm-edu", "PyCharm Edu", "PE-RELEASE-licensing-RELEASE"],
        vec![
            "pycharm-professional",
            "PyCharm",
            "PC-PY-RELEASE-licensing-RELEASE",
        ],
        vec!["rider", "Rider", "RD-RELEASE-licensing-RELEASE"],
        vec!["rubymine", "RubyMine", "RM-RELEASE-licensing-RELEASE"],
        vec!["rubymine-eap", "RubyMine", "RM-EAP-licensing-EAP"],
        vec!["webstorm", "WebStorm", "WS-RELEASE-licensing-RELEASE"],
        vec!["webstorm-eap", "WebStorm", "WS-EAP-licensing-EAP"],
    ];
    let (updates, packages) = join!(
        fetch_jetbrains_updates(),
        fetch_aur_packages(jetbrains_products.iter().map(|p| p[0].clone()).collect())
    );
    print_jetbrains_updates(jetbrains_products, packages.unwrap(), updates.unwrap());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (jetbrains_result, chrome_result) =
        join!(check_jetbrains_updates(), check_chrome_updates());
    jetbrains_result.expect("Cannot fetch JetBrains updates!");
    chrome_result.expect("Cannot fetch Google Chrome updates!");
    Ok(())
}
