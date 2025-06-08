use crate::YumUpdate;
use aur::AurPackage;

fn print_update(package: &str, version: String, new_version: &str, show_all: bool) {
    if show_all || version != new_version {
        println!("{} {} -> {}", package, version, new_version);
    }
}

fn print_yum_update(package: &AurPackage, update: &YumUpdate, show_all: bool) {
    print_update(
        &package.name,
        package.get_package_version(),
        &update.version,
        show_all,
    );
}

pub fn print_yum_updates(
    products: Vec<Vec<&str>>,
    packages: Vec<AurPackage>,
    updates: Vec<YumUpdate>,
    show_all: bool,
) {
    products.iter().for_each(|product| {
        print_yum_update(
            packages.iter().find(|&p| p.name == product[0]).unwrap(),
            updates.iter().find(|&u| u.name == product[1]).unwrap(),
            show_all,
        )
    });
}
