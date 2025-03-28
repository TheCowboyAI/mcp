use std::sync::Arc;
use env_logger;
use log::info;
use mcp_rust_sdk::server::{Server, ServerHandler};
use mcp_rust_sdk::transport::Transport;
use nix_inspector_mcp::{StdioTransport, inspector::system::SystemAnalyzer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    info!("Starting nix-inspector-mcp server");

    // Create system analyzer
    let system_analyzer = SystemAnalyzer::new()?;
    info!("System analyzer initialized");

    // Create stdio transport
    let transport = StdioTransport::new()?;
    info!("Using stdio transport");

    // Create and start server
    let server = Server::new(
        Arc::new(transport) as Arc<dyn Transport>,
        Arc::new(system_analyzer) as Arc<dyn ServerHandler>
    );
    info!("Server starting...");
    server.start().await?;

    Ok(())
} 