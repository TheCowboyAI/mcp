use std::sync::Arc;
use env_logger;
use log::info;
use nix_inspector_mcp::{ServerBuilder, inspector::system::SystemAnalyzer};
use nix_inspector_mcp::transport::stdio::StdioTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    info!("Starting nix-inspector-mcp server");

    // Create system analyzer
    let system_analyzer = SystemAnalyzer::new()?;
    info!("System analyzer initialized");

    // Create and start server using builder
    let server = ServerBuilder::new()
        .name("nix-inspector-mcp")
        .version("0.2.0")
        .add_provider("system", system_analyzer)
        .build()?;

    info!("Server starting...");
    server.run().await?;

    Ok(())
} 