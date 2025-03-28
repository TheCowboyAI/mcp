pub mod inspector;
pub mod transport;

// Re-export StdioTransport for public use
pub use transport::stdio::StdioTransport;

pub use inspector::*;
pub use transport::*;

use std::sync::Arc;
use mcp_rust_sdk::server::Server as McpServer;
use mcp_rust_sdk::server::ServerHandler;

/// A builder for creating MCP servers with custom configurations
pub struct ServerBuilder {
    name: String,
    version: String,
    providers: Vec<(String, Arc<dyn ServerHandler>)>,
}

impl ServerBuilder {
    /// Creates a new ServerBuilder instance
    pub fn new() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            providers: Vec::new(),
        }
    }

    /// Sets the server name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the server version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Adds a provider to the server
    pub fn add_provider(mut self, name: impl Into<String>, provider: impl ServerHandler + 'static) -> Self {
        self.providers.push((name.into(), Arc::new(provider)));
        self
    }

    /// Builds the server with the configured options
    pub fn build(self) -> Result<Server, Box<dyn std::error::Error>> {
        // For now, we'll use stdio transport by default
        let transport = StdioTransport::new()?;
        
        // Create the server with the first provider
        if let Some((_, provider)) = self.providers.into_iter().next() {
            Ok(Server {
                inner: McpServer::new(Arc::new(transport) as Arc<dyn McpTransport>, provider),
            })
        } else {
            Err("No providers added to server".into())
        }
    }
}

/// A wrapper around the MCP server that provides a simpler interface
pub struct Server {
    inner: McpServer,
}

impl Server {
    /// Returns the name of the server
    pub fn name(&self) -> &str {
        "nix-inspector-mcp"
    }

    /// Returns the version of the server
    pub fn version(&self) -> &str {
        "0.2.0"
    }

    /// Starts the server and runs until completion
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.start().await.map_err(Into::into)
    }
}

// Re-export only the Server trait since that's what we implement
pub use mcp_rust_sdk::server::Server as McpServerTrait; 