use aur::fetch_aur_packages;
use chrome::fetch_chrome_updates;
use clap::Parser;
use edge::fetch_edge_updates;
use futures::join;
use jetbrains::{fetch_jetbrains_updates, print_jetbrains_updates};
use mongodb::fetch_mongodb_updates;
use reqwest::Error;
use teamviewer::fetch_teamviewer_updates;
use yum::{print_yum_updates, YumUpdate};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Check updates from JetBrains products
    #[arg(long = "jb")]
    jetbrains: bool,

    /// Check updates from MongoDB products
    #[arg(long = "mongo")]
    mongodb: bool,

    /// Check updates from Google Chrome products
    #[arg(long = "chrome")]
    chrome: bool,

    /// Show all packages even if versions match
    #[arg(long = "all", short = 'a')]
    show_all: bool,
}

async fn check_yum_updates(
    products: Vec<Vec<&str>>,
    updates: Vec<YumUpdate>,
    show_all: bool,
) -> Result<(), Error> {
    let packages = fetch_aur_packages(products.iter().map(|p| p[0]).collect()).await?;
    print_yum_updates(products, packages, updates, show_all);
    Ok(())
}

async fn check_chrome_updates(show_all: bool) -> Result<(), Error> {
    let products = vec![
        vec!["google-chrome", "google-chrome-stable"],
        vec!["google-chrome-beta", "google-chrome-beta"],
        vec!["google-chrome-canary", "google-chrome-canary"],
        vec!["google-chrome-dev", "google-chrome-unstable"],
    ];
    check_yum_updates(products, fetch_chrome_updates().await?, show_all).await?;
    Ok(())
}

async fn check_edge_updates(show_all: bool) -> Result<(), Error> {
    let products = vec![
        vec!["microsoft-edge-beta-bin", "microsoft-edge-beta"],
        vec!["microsoft-edge-dev-bin", "microsoft-edge-dev"],
        vec!["microsoft-edge-stable-bin", "microsoft-edge-stable"],
    ];
    check_yum_updates(products, fetch_edge_updates().await?, show_all).await?;
    Ok(())
}

async fn check_mongodb_updates(show_all: bool) -> Result<(), Error> {
    let products = vec![
        vec!["mongodb-bin", "mongodb-org"],
        vec!["mongodb-tools-bin", "mongodb-database-tools"],
        vec!["mongosh-bin", "mongodb-mongosh"],
    ];
    check_yum_updates(products, fetch_mongodb_updates().await?, show_all).await?;
    Ok(())
}

async fn check_teamviewer_updates(show_all: bool) -> Result<(), Error> {
    let products = vec![vec!["teamviewer", "teamviewer"]];
    check_yum_updates(products, fetch_teamviewer_updates().await?, show_all).await?;
    Ok(())
}

async fn check_jetbrains_updates(show_all: bool) -> Result<(), Error> {
    let jetbrains_products = vec![
        vec!["aqua", "Aqua", "QA-RELEASE-licensing-RELEASE"],
        vec!["aqua-eap", "Aqua", "QA-EAP-licensing-EAP"],
        vec!["clion", "CLion", "CL-RELEASE-licensing-RELEASE"],
        vec!["datagrip", "DataGrip", "DB-RELEASE-licensing-RELEASE"],
        vec!["goland", "GoLand", "GO-RELEASE-licensing-RELEASE"],
        vec!["goland-eap", "GoLand", "GO-EAP-licensing-EAP"],
        vec![
            "intellij-idea-community-edition-jre",
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
        vec!["pycharm-eap", "PyCharm", "PC-PY-EAP-licensing-EAP"],
        vec![
            "pycharm-professional",
            "PyCharm",
            "PC-PY-RELEASE-licensing-RELEASE",
        ],
        vec!["rider", "Rider", "RD-RELEASE-licensing-RELEASE"],
        vec!["rubymine", "RubyMine", "RM-RELEASE-licensing-RELEASE"],
        vec!["rubymine-eap", "RubyMine", "RM-EAP-licensing-EAP"],
        vec!["rustrover", "RustRover", "RR-RELEASE-licensing-RELEASE"],
        vec!["rustrover-eap", "RustRover", "RR-EAP-licensing-EAP"],
        vec!["webstorm", "WebStorm", "WS-RELEASE-licensing-RELEASE"],
        vec!["webstorm-eap", "WebStorm", "WS-EAP-licensing-EAP"],
    ];
    let (updates, packages) = join!(
        fetch_jetbrains_updates(),
        fetch_aur_packages(jetbrains_products.iter().map(|p| p[0]).collect())
    );
    print_jetbrains_updates(
        jetbrains_products,
        packages.unwrap(),
        updates.unwrap(),
        show_all,
    );
    Ok(())
}

type UpdateFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Error>> + Send>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    if args.jetbrains || args.mongodb || args.chrome {
        // Run only the selected products concurrently
        let mut futures: Vec<(&str, UpdateFuture)> = Vec::new();

        macro_rules! add_if_enabled {
            ($flag:expr, $name:expr, $check_fn:expr) => {
                if $flag {
                    futures.push(($name, Box::pin($check_fn(args.show_all))));
                }
            };
        }

        add_if_enabled!(args.jetbrains, "JetBrains", check_jetbrains_updates);
        add_if_enabled!(args.mongodb, "MongoDB", check_mongodb_updates);
        add_if_enabled!(args.chrome, "Google Chrome", check_chrome_updates);

        let results = futures::future::join_all(
            futures
                .into_iter()
                .map(|(name, future)| async move { (name, future.await) }),
        )
        .await;
        for (name, result) in results {
            result.unwrap_or_else(|_| panic!("Cannot fetch updates for {}!", name));
        }
    } else {
        // Check all products if no specific flag is provided
        let (jetbrains_result, chrome_result, edge_result, mongodb_result, teamviewer_result) = join!(
            check_jetbrains_updates(args.show_all),
            check_chrome_updates(args.show_all),
            check_edge_updates(args.show_all),
            check_mongodb_updates(args.show_all),
            check_teamviewer_updates(args.show_all)
        );
        jetbrains_result.expect("Cannot fetch JetBrains updates!");
        edge_result.expect("Cannot fetch Microsoft Edge updates!");
        chrome_result.expect("Cannot fetch Google Chrome updates!");
        mongodb_result.expect("Cannot fetch MongoDB updates!");
        teamviewer_result.expect("Cannot fetch TeamViewer updates!");
    }
    Ok(())
}
