use nix_inspector_mcp::{ServerBuilder, SystemAnalyzer};

#[tokio::test]
async fn test_server_setup() {
    let system_analyzer = SystemAnalyzer::new().expect("Failed to create system analyzer");
    
    let server = ServerBuilder::new()
        .name("test-server")
        .version("0.1.0")
        .add_provider("system", system_analyzer)
        .build()
        .expect("Failed to build server");
        
    assert_eq!(server.name(), "nix-inspector-mcp");
    assert_eq!(server.version(), "0.1.0");
} 