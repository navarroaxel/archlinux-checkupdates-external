use crate::aur::AurPackage;
use crate::chrome::ChromeUpdate;

fn print_update(package: &str, version: String, new_version: &str) {
    if version != new_version {
        println!("{} {} -> {}", package, version, new_version);
    }
}

fn print_chrome_update(package: &AurPackage, update: &ChromeUpdate) {
    print_update(&package.name, package.get_package_version(), &update.version);
}

pub fn print_chrome_updates(products: Vec<Vec<&str>>, packages: Vec<AurPackage>, updates: Vec<ChromeUpdate>) {
    products.iter()
        .for_each(| product | print_chrome_update(
            packages.iter().find(| &p | p.name == product[0]).unwrap(),
            updates.iter().find(| &u | u.name == product[1]).unwrap()
        ));
}
