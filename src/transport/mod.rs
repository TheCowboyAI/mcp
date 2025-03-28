use async_trait::async_trait;
use futures::Stream;
use mcp_rust_sdk::error::Error as McpError;
use mcp_rust_sdk::transport::Message;
use std::pin::Pin;

mod stdio;
pub use stdio::StdioTransport;

/// Transport trait that defines the interface for MCP message transport
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a message through the transport
    async fn send(&self, message: Message) -> Result<(), McpError>;

    /// Receive messages as a stream
    async fn receive(&self) -> Pin<Box<dyn Stream<Item = Result<Message, McpError>> + Send + 'static>>;

    /// Close the transport
    async fn close(&self) -> Result<(), McpError>;
}

// Re-export the Transport trait from mcp_rust_sdk for convenience
pub use mcp_rust_sdk::transport::Transport as McpTransport; 