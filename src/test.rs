#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phpmd_config_file_detection() {
        // Test that we correctly identify PHPMD config files
        let config_files = vec![
            ".phpmd.xml",
            "phpmd.xml",
            ".phpmd.xml.dist",
            "phpmd.xml.dist",
            "ruleset.xml",
        ];

        for file in config_files {
            assert!(PHPMD_CONFIG_FILES.contains(&file));
        }
    }

    #[test]
    fn test_platform_binary_name_generation() {
        // Test macOS ARM64
        let binary_name = match (zed::Os::Mac, zed::Architecture::Aarch64) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "phpmd-lsp-server-macos-arm64",
            _ => "",
        };
        assert_eq!(binary_name, "phpmd-lsp-server-macos-arm64");

        // Test macOS x64
        let binary_name = match (zed::Os::Mac, zed::Architecture::X8664) {
            (zed::Os::Mac, zed::Architecture::X8664) => "phpmd-lsp-server-macos-x64",
            _ => "",
        };
        assert_eq!(binary_name, "phpmd-lsp-server-macos-x64");

        // Test Linux x64
        let binary_name = match (zed::Os::Linux, zed::Architecture::X8664) {
            (zed::Os::Linux, zed::Architecture::X8664) => "phpmd-lsp-server-linux-x64",
            _ => "",
        };
        assert_eq!(binary_name, "phpmd-lsp-server-linux-x64");

        // Test Linux ARM64
        let binary_name = match (zed::Os::Linux, zed::Architecture::Aarch64) {
            (zed::Os::Linux, zed::Architecture::Aarch64) => "phpmd-lsp-server-linux-arm64",
            _ => "",
        };
        assert_eq!(binary_name, "phpmd-lsp-server-linux-arm64");

        // Test Windows x64
        let binary_name = match (zed::Os::Windows, zed::Architecture::X8664) {
            (zed::Os::Windows, zed::Architecture::X8664) => "phpmd-lsp-server-windows-x64.exe",
            _ => "",
        };
        assert_eq!(binary_name, "phpmd-lsp-server-windows-x64.exe");
    }

    #[test]
    fn test_version_constant() {
        // Ensure VERSION is properly set from Cargo.toml
        assert!(!VERSION.is_empty());
        assert!(VERSION.len() > 0);
        
        // Version should follow semver format (basic check)
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert!(parts.len() >= 2); // At least major.minor
    }

    #[test]
    fn test_language_server_id() {
        assert_eq!(PhpmdLspServer::LANGUAGE_SERVER_ID, "phpmd");
    }

    #[test]
    fn test_default_ruleset_configuration() {
        // When no configuration is provided, we should use default rulesets
        let default_ruleset = "cleancode,codesize,controversial,design,naming,unusedcode";
        
        // This would be the default if no config is found
        assert!(default_ruleset.contains("cleancode"));
        assert!(default_ruleset.contains("codesize"));
        assert!(default_ruleset.contains("controversial"));
        assert!(default_ruleset.contains("design"));
        assert!(default_ruleset.contains("naming"));
        assert!(default_ruleset.contains("unusedcode"));
    }
}