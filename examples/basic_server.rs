use nix_inspector_mcp::{ServerBuilder, SystemAnalyzer};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create system analyzer
    let system_analyzer = SystemAnalyzer::new()?;
    info!("System analyzer initialized");
    
    // Create and start MCP server
    let server = ServerBuilder::new()
        .name("example-server")
        .version("0.1.0")
        .add_provider("system", system_analyzer)
        .build()?;
        
    info!("Server starting...");
    server.run().await?;
    
    Ok(())
} 