use aur::fetch_aur_packages;
use chrome::fetch_chrome_updates;
use edge::fetch_edge_updates;
use futures::join;
use jetbrains::{fetch_jetbrains_updates, print_jetbrains_updates};
use mongodb::fetch_mongodb_updates;
use reqwest::Error;
use yum::{print_yum_updates, YumUpdate};

async fn check_yum_updates(products: Vec<Vec<&str>>, updates: Vec<YumUpdate>) -> Result<(), Error> {
    let packages = fetch_aur_packages(products.iter().map(|p| p[0]).collect()).await?;
    print_yum_updates(products, packages, updates);
    Ok(())
}

async fn check_chrome_updates() -> Result<(), Error> {
    let products = vec![
        vec!["google-chrome", "google-chrome-stable"],
        vec!["google-chrome-beta", "google-chrome-beta"],
        vec!["google-chrome-dev", "google-chrome-unstable"],
    ];
    check_yum_updates(products, fetch_chrome_updates().await?).await?;
    Ok(())
}

async fn check_edge_updates() -> Result<(), Error> {
    let products = vec![
        vec!["microsoft-edge-beta-bin", "microsoft-edge-beta"],
        vec!["microsoft-edge-dev-bin", "microsoft-edge-dev"],
    ];
    check_yum_updates(products, fetch_edge_updates().await?).await?;
    Ok(())
}

async fn check_mongodb_updates() -> Result<(), Error> {
    let products = vec![
        vec!["mongodb-bin", "mongodb-org"],
        vec!["mongodb-tools-bin", "mongodb-database-tools"],
    ];
    check_yum_updates(products, fetch_mongodb_updates().await?).await?;
    Ok(())
}

async fn check_jetbrains_updates() -> Result<(), Error> {
    let jetbrains_products = vec![
        vec!["clion", "CLion", "CL-RELEASE-licensing-RELEASE"],
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
        fetch_aur_packages(jetbrains_products.iter().map(|p| p[0]).collect())
    );
    print_jetbrains_updates(jetbrains_products, packages.unwrap(), updates.unwrap());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (jetbrains_result, edge_result, chrome_result, mongodb_result) = join!(
        check_jetbrains_updates(),
        check_edge_updates(),
        check_chrome_updates(),
        check_mongodb_updates()
    );
    jetbrains_result.expect("Cannot fetch JetBrains updates!");
    edge_result.expect("Cannot fetch Microsoft Edge updates!");
    chrome_result.expect("Cannot fetch Google Chrome updates!");
    mongodb_result.expect("Cannot fetch MongoDB updates!");
    Ok(())
}
