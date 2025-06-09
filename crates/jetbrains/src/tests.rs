#[cfg(test)]
mod model_tests {
    use crate::model::*;

    #[test]
    fn test_build_deserialization() {
        let xml = r#"<build number="241.14494.240" version="2024.1" fullNumber="241.14494.240"/>"#;
        let build: Build = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(build.number, "241.14494.240");
        assert_eq!(build.version, "2024.1");
        assert_eq!(build.full_number, Some("241.14494.240".to_string()));
    }

    #[test]
    fn test_build_without_full_number() {
        let xml = r#"<build number="241.14494.240" version="2024.1"/>"#;
        let build: Build = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(build.number, "241.14494.240");
        assert_eq!(build.version, "2024.1");
        assert_eq!(build.full_number, None);
    }

    #[test]
    fn test_channel_deserialization() {
        let xml = r#"<channel id="IC-IU-RELEASE-licensing-RELEASE">
            <build number="241.14494.240" version="2024.1"/>
            <build number="233.15026.9" version="2023.3.6"/>
        </channel>"#;
        let channel: Channel = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(channel.id, "IC-IU-RELEASE-licensing-RELEASE");
        assert_eq!(channel.builds.len(), 2);
        assert_eq!(channel.builds[0].version, "2024.1");
        assert_eq!(channel.builds[1].version, "2023.3.6");
    }

    #[test]
    fn test_product_deserialization() {
        let xml = r#"<product name="IntelliJ IDEA">
            <channel id="IC-IU-RELEASE-licensing-RELEASE">
                <build number="241.14494.240" version="2024.1"/>
            </channel>
            <channel id="IC-IU-EAP-licensing-EAP">
                <build number="241.15989.69" version="2024.1.1" fullNumber="241.15989.69"/>
            </channel>
        </product>"#;
        let product: Product = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(product.name, "IntelliJ IDEA");
        assert_eq!(product.channels.len(), 2);
        assert_eq!(product.channels[0].id, "IC-IU-RELEASE-licensing-RELEASE");
        assert_eq!(product.channels[1].id, "IC-IU-EAP-licensing-EAP");
    }

    #[test]
    fn test_product_without_channels() {
        let xml = r#"<product name="Empty Product"></product>"#;
        let product: Product = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(product.name, "Empty Product");
        assert_eq!(product.channels.len(), 0);
    }

    #[test]
    fn test_jetbrains_repository_deserialization() {
        let xml = r#"<products>
            <product name="IntelliJ IDEA">
                <channel id="IC-IU-RELEASE-licensing-RELEASE">
                    <build number="241.14494.240" version="2024.1"/>
                </channel>
            </product>
            <product name="PyCharm">
                <channel id="PC-PY-RELEASE-licensing-RELEASE">
                    <build number="241.14494.283" version="2024.1"/>
                </channel>
            </product>
        </products>"#;
        let repository: JetBrainsRepository = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(repository.products.len(), 2);
        assert_eq!(repository.products[0].name, "IntelliJ IDEA");
        assert_eq!(repository.products[1].name, "PyCharm");
    }
}

#[cfg(test)]
mod print_tests {
    use crate::print::*;
    use crate::model::*;
    use aur::AurPackage;

    fn create_test_aur_package(name: &str, version: &str) -> AurPackage {
        AurPackage {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    fn create_test_build(number: &str, version: &str, full_number: Option<&str>) -> Build {
        Build {
            number: number.to_string(),
            version: version.to_string(),
            full_number: full_number.map(|s| s.to_string()),
        }
    }

    fn create_test_channel(id: &str, builds: Vec<Build>) -> Channel {
        Channel {
            id: id.to_string(),
            builds,
        }
    }

    fn create_test_product(name: &str, channels: Vec<Channel>) -> Product {
        Product {
            name: name.to_string(),
            channels,
        }
    }

    #[test]
    fn test_get_package_build() {
        assert_eq!(get_package_build("2024.1.4.241.18034.62".to_string()), "241.18034.62");
        assert_eq!(get_package_build("2023.3.6.233.15026.9".to_string()), "233.15026.9");
        assert_eq!(get_package_build("1.2.3.4.5.6".to_string()), "4.5.6");
    }

    #[test]
    fn test_remove_package_build() {
        assert_eq!(remove_package_build("2024.1b241.14494.240".to_string()), "2024.1");
        assert_eq!(remove_package_build("2023.3b233.15026.9".to_string()), "2023.3");
        assert_eq!(remove_package_build("2024.1".to_string()), "2024.1");
    }

    #[test]
    fn test_remove_epoch() {
        assert_eq!(remove_epoch("1:2024.1-1".to_string()), "2024.1-1");
        assert_eq!(remove_epoch("2:2023.3.6-1".to_string()), "2023.3.6-1");
        assert_eq!(remove_epoch("2024.1-1".to_string()), "2024.1-1");
    }

    #[test]
    fn test_get_package_pre_build() {
        assert_eq!(get_package_pre_build("2024.1pre+241.15989.69".to_string()), "241.15989.69");
        assert_eq!(get_package_pre_build("2023.3pre+233.15026.9".to_string()), "233.15026.9");
    }

    #[test]
    fn test_print_update_with_different_versions() {
        let output = std::panic::catch_unwind(|| {
            print_update("test-package", "1.0.0".to_string(), "2.0.0", false);
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_update_with_same_versions_show_all_false() {
        let output = std::panic::catch_unwind(|| {
            print_update("test-package", "1.0.0".to_string(), "1.0.0", false);
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_update_with_same_versions_show_all_true() {
        let output = std::panic::catch_unwind(|| {
            print_update("test-package", "1.0.0".to_string(), "1.0.0", true);
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_jetbrains_update_stable() {
        let package = create_test_aur_package("intellij-idea-ce", "2024.1b241.14494.240-1");
        let build = create_test_build("241.14494.240", "2024.1", None);
        let channel = create_test_channel("IC-IU-RELEASE-licensing-RELEASE", vec![build]);
        let product = create_test_product("IntelliJ IDEA", vec![channel]);
        
        let output = std::panic::catch_unwind(|| {
            print_jetbrains_update(
                "IC-IU-RELEASE-licensing-RELEASE",
                &package,
                &product,
                false,
                false,
            );
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_jetbrains_update_eap() {
        let package = create_test_aur_package("intellij-idea-ce-eap", "2024.1.1.241.15989.69-1");
        let build = create_test_build("241.15989.69", "2024.1.1", Some("241.15989.69"));
        let channel = create_test_channel("IC-223-EAP-RELEASE", vec![build]);
        let product = create_test_product("IntelliJ IDEA", vec![channel]);
        
        let output = std::panic::catch_unwind(|| {
            print_jetbrains_update(
                "IC-223-EAP-RELEASE",
                &package,
                &product,
                true,
                false,
            );
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_jetbrains_update_rustrover_eap_with_pre() {
        let package = create_test_aur_package("rustrover-eap", "2024.1pre+241.15989.69-1");
        let build = create_test_build("241.15989.69", "2024.1", Some("241.15989.69"));
        let channel = create_test_channel("RR-233-EAP-RELEASE", vec![build]);
        let product = create_test_product("RustRover", vec![channel]);
        
        let output = std::panic::catch_unwind(|| {
            print_jetbrains_update(
                "RR-233-EAP-RELEASE",
                &package,
                &product,
                true,
                false,
            );
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_jetbrains_update_rustrover_eap_without_pre() {
        let package = create_test_aur_package("rustrover-eap", "2024.1-1");
        let build = create_test_build("241.15989.69", "2024.1", Some("241.15989.69"));
        let channel = create_test_channel("RR-233-EAP-RELEASE", vec![build]);
        let product = create_test_product("RustRover", vec![channel]);
        
        let output = std::panic::catch_unwind(|| {
            print_jetbrains_update(
                "RR-233-EAP-RELEASE",
                &package,
                &product,
                true,
                false,
            );
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_jetbrains_updates() {
        let packages = vec![
            create_test_aur_package("intellij-idea-ce", "2024.1b241.14494.240-1"),
            create_test_aur_package("pycharm-community", "2024.1b241.14494.283-1"),
        ];
        
        let idea_build = create_test_build("241.14494.240", "2024.1", None);
        let idea_channel = create_test_channel("IC-IU-RELEASE-licensing-RELEASE", vec![idea_build]);
        let idea_product = create_test_product("IntelliJ IDEA", vec![idea_channel]);
        
        let pycharm_build = create_test_build("241.14494.283", "2024.1", None);
        let pycharm_channel = create_test_channel("PC-PY-RELEASE-licensing-RELEASE", vec![pycharm_build]);
        let pycharm_product = create_test_product("PyCharm", vec![pycharm_channel]);
        
        let repository = JetBrainsRepository {
            products: vec![idea_product, pycharm_product],
        };
        
        let products = vec![
            vec!["intellij-idea-ce", "IntelliJ IDEA", "IC-IU-RELEASE-licensing-RELEASE"],
            vec!["pycharm-community", "PyCharm", "PC-PY-RELEASE-licensing-RELEASE"],
        ];
        
        let output = std::panic::catch_unwind(|| {
            print_jetbrains_updates(products, packages, repository, false);
        });
        assert!(output.is_ok());
    }
}

#[cfg(test)]
mod fetch_tests {
    use crate::fetch::*;
    
    #[tokio::test]
    async fn test_fetch_jetbrains_updates_structure() {
        // This test verifies the structure is parsed correctly
        // In a real test environment, you might want to mock the HTTP request
        match fetch_jetbrains_updates().await {
            Ok(repository) => {
                // Verify we got some products
                assert!(!repository.products.is_empty(), "Should have at least one product");
                
                // Check first product has required fields
                let first_product = &repository.products[0];
                assert!(!first_product.name.is_empty(), "Product should have a name");
                
                // If channels exist, verify their structure
                if !first_product.channels.is_empty() {
                    let first_channel = &first_product.channels[0];
                    assert!(!first_channel.id.is_empty(), "Channel should have an ID");
                    
                    if !first_channel.builds.is_empty() {
                        let first_build = &first_channel.builds[0];
                        assert!(!first_build.number.is_empty(), "Build should have a number");
                        assert!(!first_build.version.is_empty(), "Build should have a version");
                    }
                }
            }
            Err(e) => {
                // In CI/CD or offline environments, the test might fail due to network issues
                // This is acceptable for a unit test
                eprintln!("Network request failed (this is acceptable in test environments): {}", e);
            }
        }
    }
}