use nix_inspector_mcp::{ServerBuilder, SystemAnalyzer};
use std::env;

#[tokio::test]
async fn test_server_setup() {
    // Get nix path from environment or use default
    let nix_path = env::var("NIX_BIN_PATH").unwrap_or_else(|_| "/nix/var/nix/profiles/default/bin/nix".to_string());
    let system_analyzer = SystemAnalyzer::with_nix_path(Some(nix_path))
        .expect("Failed to create system analyzer");
    
    let server = ServerBuilder::new()
        .name("test-server")
        .version("0.1.0")
        .add_provider("system", system_analyzer)
        .build()
        .expect("Failed to build server");
        
    assert_eq!(server.name(), "nix-inspector-mcp");
    assert_eq!(server.version(), "0.1.0");
} 