use mcp_rust_sdk::{Server, ServerBuilder};
use tokio;
use log::{info, error};

mod inspector;
use inspector::system::SystemAnalyzer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    info!("Starting nix-inspector-mcp server");

    // Create system analyzer
    let system_analyzer = SystemAnalyzer::new()?;

    // Create MCP server
    let server = ServerBuilder::new()
        .name("nix-inspector-mcp")
        .version("0.1.0")
        .add_provider("system", system_analyzer)
        .build()?;

    info!("Server initialized, starting...");

    // Start server
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
        return Err(e.into());
    }

    Ok(())
} 