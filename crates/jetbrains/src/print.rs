use crate::{JetBrainsRepository, Product};
use aur::AurPackage;

#[cfg(test)]
pub(crate) fn get_package_build(version: String) -> String {
    get_package_build_impl(version)
}

#[cfg(not(test))]
fn get_package_build(version: String) -> String {
    get_package_build_impl(version)
}

fn get_package_build_impl(version: String) -> String {
    let mut result = String::with_capacity(version.len());
    let mut splitter = version.rsplit('.');
    for i in 0..3 {
        if i != 0 {
            result.insert(0, '.');
        }
        result.insert_str(0, splitter.next().unwrap());
    }
    result
}

#[cfg(test)]
pub(crate) fn remove_package_build(version: String) -> String {
    remove_package_build_impl(version)
}

#[cfg(not(test))]
fn remove_package_build(version: String) -> String {
    remove_package_build_impl(version)
}

fn remove_package_build_impl(version: String) -> String {
    let mut splitter = version.split('b');
    splitter.next().unwrap().to_string()
}

#[cfg(test)]
pub(crate) fn remove_epoch(version: String) -> String {
    remove_epoch_impl(version)
}

#[cfg(not(test))]
fn remove_epoch(version: String) -> String {
    remove_epoch_impl(version)
}

fn remove_epoch_impl(version: String) -> String {
    let mut splitter = version.rsplit(':');
    splitter.next().unwrap().to_string()
}

#[cfg(test)]
pub(crate) fn print_update(package: &str, version: String, new_version: &str, show_all: bool) {
    print_update_impl(package, version, new_version, show_all)
}

#[cfg(not(test))]
fn print_update(package: &str, version: String, new_version: &str, show_all: bool) {
    print_update_impl(package, version, new_version, show_all)
}

fn print_update_impl(package: &str, version: String, new_version: &str, show_all: bool) {
    if show_all || version != new_version {
        println!("{} {} -> {}", package, version, new_version);
    }
}

#[cfg(test)]
pub(crate) fn get_package_pre_build(version: String) -> String {
    get_package_pre_build_impl(version)
}

#[cfg(not(test))]
fn get_package_pre_build(version: String) -> String {
    get_package_pre_build_impl(version)
}

fn get_package_pre_build_impl(version: String) -> String {
    let mut splitter = version.split("pre+");
    splitter.next();
    splitter.next().unwrap().to_string()
}

#[cfg(test)]
pub(crate) fn print_jetbrains_update(
    channel_name: &str,
    package: &AurPackage,
    product: &Product,
    is_eap: bool,
    show_all: bool,
) {
    print_jetbrains_update_impl(channel_name, package, product, is_eap, show_all)
}

#[cfg(not(test))]
fn print_jetbrains_update(
    channel_name: &str,
    package: &AurPackage,
    product: &Product,
    is_eap: bool,
    show_all: bool,
) {
    print_jetbrains_update_impl(channel_name, package, product, is_eap, show_all)
}

fn print_jetbrains_update_impl(
    channel_name: &str,
    package: &AurPackage,
    product: &Product,
    is_eap: bool,
    show_all: bool,
) {
    let channel = if is_eap {
        product.channels.first().unwrap()
    } else {
        product
            .channels
            .iter()
            .find(|&c| c.id == channel_name)
            .unwrap()
    };
    let build = channel.builds.first().unwrap();
    let mut version = remove_epoch(package.get_package_version());
    let new_version;
    if package.name == "rustrover-eap" {
        if version.contains("pre") {
            version = get_package_pre_build(version);
            new_version = build.full_number.as_ref().unwrap();
        } else {
            new_version = &build.version;
        }
    } else if is_eap {
        version = get_package_build(version);
        new_version = build.full_number.as_ref().unwrap();
    } else {
        version = remove_package_build(version);
        new_version = &build.version;
    }
    print_update(&package.name, version, new_version, show_all);
}

pub fn print_jetbrains_updates(
    products: Vec<Vec<&str>>,
    packages: Vec<AurPackage>,
    repository: JetBrainsRepository,
    show_all: bool,
) {
    products.iter().for_each(|product| {
        print_jetbrains_update(
            product[2],
            packages.iter().find(|&p| p.name == product[0]).unwrap(),
            repository
                .products
                .iter()
                .find(|&u| u.name == product[1])
                .unwrap(),
            product[0].ends_with("eap"),
            show_all,
        )
    });
}
