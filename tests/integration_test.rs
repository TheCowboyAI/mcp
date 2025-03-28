use nix_inspector_mcp::{ServerBuilder, SystemAnalyzer};
use std::env;

#[tokio::test]
async fn test_server_setup() {
    let system_analyzer = SystemAnalyzer::new().unwrap();
    let server = ServerBuilder::new()
        .name("nix-inspector-mcp")
        .version("0.2.0")
        .add_provider("system", system_analyzer)
        .build()
        .unwrap();

    assert_eq!(server.name(), "nix-inspector-mcp");
    assert_eq!(server.version(), "0.2.0");
}