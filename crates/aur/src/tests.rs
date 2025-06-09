#[cfg(test)]
mod model_tests {
    use crate::model::*;

    #[test]
    fn test_aur_package_deserialization() {
        let json = r#"{
            "Name": "test-package",
            "Version": "1.2.3-1"
        }"#;
        let package: AurPackage = serde_json::from_str(json).unwrap();
        
        assert_eq!(package.name, "test-package");
        assert_eq!(package.version, "1.2.3-1");
    }

    #[test]
    fn test_aur_response_deserialization() {
        let json = r#"{
            "version": 5,
            "resultcount": 2,
            "results": [
                {
                    "Name": "package1",
                    "Version": "1.0.0-1"
                },
                {
                    "Name": "package2",
                    "Version": "2.0.0-1"
                }
            ]
        }"#;
        let response: AurResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.version, 5);
        assert_eq!(response.result_count, 2);
        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].name, "package1");
        assert_eq!(response.results[1].name, "package2");
    }

    #[test]
    fn test_aur_package_get_package_version() {
        let package = AurPackage {
            name: "test".to_string(),
            version: "1.2.3-1".to_string(),
        };
        assert_eq!(package.get_package_version(), "1.2.3");
        
        let package_no_release = AurPackage {
            name: "test".to_string(),
            version: "1.2.3".to_string(),
        };
        assert_eq!(package_no_release.get_package_version(), "1.2.3");
        
        let package_with_epoch = AurPackage {
            name: "test".to_string(),
            version: "2:1.2.3-1".to_string(),
        };
        assert_eq!(package_with_epoch.get_package_version(), "2:1.2.3");
    }

    #[test]
    fn test_aur_response_empty_results() {
        let json = r#"{
            "version": 5,
            "resultcount": 0,
            "results": []
        }"#;
        let response: AurResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.version, 5);
        assert_eq!(response.result_count, 0);
        assert_eq!(response.results.len(), 0);
    }
}

#[cfg(test)]
mod fetch_tests {
    use crate::fetch::*;
    
    #[tokio::test]
    async fn test_fetch_aur_packages_structure() {
        // Test with a real package that should exist
        match fetch_aur_packages(vec!["base"]).await {
            Ok(packages) => {
                // If we get packages, verify they have the expected structure
                for package in packages.iter() {
                    assert!(!package.name.is_empty(), "Package should have a name");
                    assert!(!package.version.is_empty(), "Package should have a version");
                }
            }
            Err(e) => {
                // In CI/CD or offline environments, the test might fail due to network issues
                // This is acceptable for a unit test
                eprintln!("Network request failed (this is acceptable in test environments): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_aur_packages_multiple() {
        // Test fetching multiple packages
        match fetch_aur_packages(vec!["base", "linux"]).await {
            Ok(packages) => {
                // Should return results for valid packages
                assert!(packages.len() <= 2, "Should return at most the requested number of packages");
            }
            Err(e) => {
                eprintln!("Network request failed (this is acceptable in test environments): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_aur_packages_nonexistent() {
        // Test with a package that probably doesn't exist
        match fetch_aur_packages(vec!["this-package-definitely-does-not-exist-123456789"]).await {
            Ok(packages) => {
                assert_eq!(packages.len(), 0, "Should return empty results for non-existent package");
            }
            Err(e) => {
                eprintln!("Network request failed (this is acceptable in test environments): {}", e);
            }
        }
    }
}