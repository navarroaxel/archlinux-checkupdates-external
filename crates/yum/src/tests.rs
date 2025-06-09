#[cfg(test)]
mod model_tests {
    use crate::model::*;

    #[test]
    fn test_repository_location_deserialization() {
        let xml = r#"<location href="repodata/other.xml.gz"/>"#;
        let location: RepositoryLocation = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(location.href, "repodata/other.xml.gz");
    }

    #[test]
    fn test_repository_data_deserialization() {
        let xml = r#"<data type="other">
            <location href="repodata/other.xml.gz"/>
        </data>"#;
        let data: RepositoryData = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(data.data_type, "other");
        assert_eq!(data.location.href, "repodata/other.xml.gz");
    }

    #[test]
    fn test_repository_metadata_deserialization() {
        let xml = r#"<repomd>
            <data type="primary">
                <location href="repodata/primary.xml.gz"/>
            </data>
            <data type="other">
                <location href="repodata/other.xml.gz"/>
            </data>
        </repomd>"#;
        let metadata: RepositoryMetadata = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(metadata.repositories.len(), 2);
        assert_eq!(metadata.repositories[0].data_type, "primary");
        assert_eq!(metadata.repositories[1].data_type, "other");
    }

    #[test]
    fn test_yum_version_deserialization() {
        let xml = r#"<version ver="1.2.3"/>"#;
        let version: YumVersion = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(version.version, "1.2.3");
    }

    #[test]
    fn test_yum_package_deserialization() {
        let xml = r#"<package name="test-package">
            <version ver="1.2.3"/>
            <version ver="1.2.2"/>
        </package>"#;
        let package: YumPackage = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(package.name, "test-package");
        assert_eq!(package.versions.len(), 2);
        assert_eq!(package.versions[0].version, "1.2.3");
        assert_eq!(package.versions[1].version, "1.2.2");
    }

    #[test]
    fn test_yum_repository_deserialization() {
        let xml = r#"<otherdata>
            <package name="package1">
                <version ver="1.0.0"/>
            </package>
            <package name="package2">
                <version ver="2.0.0"/>
            </package>
        </otherdata>"#;
        let repository: YumRepository = serde_xml_rs::from_str(xml).unwrap();
        
        assert_eq!(repository.packages.len(), 2);
        assert_eq!(repository.packages[0].name, "package1");
        assert_eq!(repository.packages[1].name, "package2");
    }

    #[test]
    fn test_yum_package_version() {
        let package = YumPackage {
            name: "test".to_string(),
            versions: vec![
                YumVersion { version: "1.2.3".to_string() },
                YumVersion { version: "1.2.2".to_string() },
            ],
        };
        
        assert_eq!(package.version(), "1.2.3");
    }

    #[test]
    fn test_yum_package_is_pre_release() {
        let stable_package = YumPackage {
            name: "test".to_string(),
            versions: vec![YumVersion { version: "1.2.3".to_string() }],
        };
        assert!(!stable_package.is_pre_release());
        
        let pre_release_package = YumPackage {
            name: "test".to_string(),
            versions: vec![YumVersion { version: "1.2.3~rc1".to_string() }],
        };
        assert!(pre_release_package.is_pre_release());
    }

    #[test]
    fn test_yum_package_semver() {
        // Normal version
        let package = YumPackage {
            name: "test".to_string(),
            versions: vec![YumVersion { version: "1.2.3".to_string() }],
        };
        assert_eq!(package.semver(), "1.2.3");
        
        // Chromium-style version (4+ components)
        let chromium_package = YumPackage {
            name: "chromium".to_string(),
            versions: vec![YumVersion { version: "120.0.6099.129".to_string() }],
        };
        assert_eq!(chromium_package.semver(), "120.6099.129");
    }

    #[test]
    fn test_yum_update_creation() {
        let update = YumUpdate {
            name: "test-package".to_string(),
            version: "1.2.3".to_string(),
        };
        
        assert_eq!(update.name, "test-package");
        assert_eq!(update.version, "1.2.3");
    }
}

#[cfg(test)]
mod fetch_tests {
    use crate::fetch::*;
    
    #[tokio::test]
    async fn test_fetch_yum_repository_path() {
        // This test would need a mock HTTP server in a real scenario
        // For now, we just verify the function doesn't panic with an invalid URL
        let result = fetch_yum_repository_path("http://invalid.url/repomd.xml").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fetch_yum_updates() {
        // This test would need a mock HTTP server in a real scenario
        // For now, we just verify the function doesn't panic with an invalid URL
        let result = fetch_yum_updates("http://invalid.url/other.xml.gz").await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod print_tests {
    use crate::print::*;
    use crate::model::YumUpdate;
    use aur::AurPackage;

    fn create_test_aur_package(name: &str, version: &str) -> AurPackage {
        AurPackage {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    fn create_test_yum_update(name: &str, version: &str) -> YumUpdate {
        YumUpdate {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    #[test]
    fn test_print_yum_updates() {
        let packages = vec![
            create_test_aur_package("google-chrome", "120.0.6099.129-1"),
            create_test_aur_package("microsoft-edge-stable", "120.0.2210.91-1"),
        ];
        
        let updates = vec![
            create_test_yum_update("google-chrome-stable", "120.0.6099.129"),
            create_test_yum_update("microsoft-edge-stable", "120.0.2210.91"),
        ];
        
        let products = vec![
            vec!["google-chrome", "google-chrome-stable"],
            vec!["microsoft-edge-stable", "microsoft-edge-stable"],
        ];
        
        // This test verifies the function executes without panicking
        let output = std::panic::catch_unwind(|| {
            print_yum_updates(products, packages, updates, false);
        });
        assert!(output.is_ok());
    }

    #[test]
    fn test_print_yum_updates_with_show_all() {
        let packages = vec![
            create_test_aur_package("teamviewer", "15.49.2-1"),
        ];
        
        let updates = vec![
            create_test_yum_update("teamviewer", "15.49.2"),
        ];
        
        let products = vec![
            vec!["teamviewer", "teamviewer"],
        ];
        
        // Test with show_all = true
        let output = std::panic::catch_unwind(|| {
            print_yum_updates(products, packages, updates, true);
        });
        assert!(output.is_ok());
    }
}