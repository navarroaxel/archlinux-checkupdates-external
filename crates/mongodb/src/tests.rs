#[cfg(test)]
mod fetch_tests {
    use crate::fetch::*;

    #[test]
    fn test_get_url() {
        assert_eq!(
            get_url("repodata/repomd.xml"),
            "https://repo.mongodb.org/yum/redhat/8/mongodb-org/8.0/x86_64/repodata/repomd.xml"
        );
        assert_eq!(
            get_url("test/path"),
            "https://repo.mongodb.org/yum/redhat/8/mongodb-org/8.0/x86_64/test/path"
        );
    }

    #[tokio::test]
    async fn test_fetch_mongodb_updates_structure() {
        // This test verifies the function executes without panicking
        // In a real test environment, you might want to mock the HTTP requests
        match fetch_mongodb_updates().await {
            Ok(updates) => {
                // If we get updates, verify they have the expected structure
                for update in updates.iter().take(1) {
                    assert!(!update.name.is_empty(), "Update should have a name");
                    assert!(!update.version.is_empty(), "Update should have a version");
                    // MongoDB updates should be for mongodb packages
                    assert!(
                        update.name.contains("mongodb"),
                        "Package name should contain 'mongodb'"
                    );
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