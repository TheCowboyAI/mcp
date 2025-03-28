use async_trait::async_trait;
use futures::{Stream, stream};
use log::{error, debug};
use mcp_rust_sdk::error::{Error as McpError, ErrorCode};
use mcp_rust_sdk::transport::{Message, Transport as McpTransport};
use mcp_rust_sdk::{Request, Response, Notification, protocol::RequestId};
use serde_json::Value;
use std::io::{self, BufRead, Write};
use std::pin::Pin;
use tokio::sync::broadcast;
use thiserror::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;

lazy_static! {
    static ref INITIALIZED: AtomicBool = AtomicBool::new(false);
}

/// Error types that can occur during stdio transport operations
#[derive(Error, Debug)]
pub enum StdioError {
    /// Represents an IO error that occurred during stdio operations
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    /// Represents an error that occurred while broadcasting a message
    #[error("Failed to broadcast message: {0}")]
    Broadcast(#[from] broadcast::error::SendError<Message>),
    /// Represents a JSON parsing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<StdioError> for McpError {
    fn from(err: StdioError) -> Self {
        McpError::protocol(ErrorCode::InternalError, err.to_string())
    }
}

/// StdioTransport implements the Transport trait for standard input/output communication
/// 
/// This transport allows MCP messages to be sent and received through stdin/stdout,
/// making it useful for command-line applications and testing.
/// 
/// # Example
/// ```no_run
/// use nix_inspector_mcp::StdioTransport;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let transport = StdioTransport::new()?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct StdioTransport {
    /// Channel sender for broadcasting messages to all receivers
    tx: broadcast::Sender<Message>,
}

impl StdioTransport {
    /// Creates a new StdioTransport instance
    pub fn new() -> Result<Self, StdioError> {
        let (tx, _) = broadcast::channel(100);
        let transport = Self { tx: tx.clone() };
        
        // Spawn a task to handle stdin
        tokio::spawn(Self::handle_stdio(tx));
        
        Ok(transport)
    }

    /// Handles reading from stdin and broadcasting messages
    async fn handle_stdio(tx: broadcast::Sender<Message>) {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin.lock());
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    debug!("Received line: {}", line);
                    match serde_json::from_str::<serde_json::Value>(&line) {
                        Ok(value) => {
                            if let Some(method) = value.get("method") {
                                if method.as_str() == Some("initialize") {
                                    INITIALIZED.store(true, Ordering::SeqCst);
                                }
                                
                                // Check if this is a request or notification
                                if value.get("id").is_some() {
                                    // This is a request
                                    if let Ok(request) = serde_json::from_value::<Request>(value.clone()) {
                                        debug!("Broadcasting request: {:?}", request);
                                        if let Err(e) = tx.send(Message::Request(request)) {
                                            error!("Failed to broadcast request: {}", e);
                                        }
                                    }
                                } else {
                                    // This is a notification
                                    if let Ok(notification) = serde_json::from_value::<Notification>(value) {
                                        debug!("Broadcasting notification: {:?}", notification);
                                        if let Err(e) = tx.send(Message::Notification(notification)) {
                                            error!("Failed to broadcast notification: {}", e);
                                        }
                                    }
                                }
                            } else if value.get("result").is_some() || value.get("error").is_some() {
                                // This is a response
                                if let Ok(response) = serde_json::from_value::<Response>(value) {
                                    debug!("Broadcasting response: {:?}", response);
                                    if let Err(e) = tx.send(Message::Response(response)) {
                                        error!("Failed to broadcast response: {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => error!("Failed to parse JSON: {}", e),
                    }
                }
                Err(e) => {
                    error!("Failed to read from stdin: {}", e);
                    break;
                }
            }
        }
    }

    pub fn is_initialized() -> bool {
        INITIALIZED.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl McpTransport for StdioTransport {
    /// Sends a message through stdout
    /// 
    /// # Arguments
    /// * `message` - The message to send
    /// 
    /// # Returns
    /// * `Result<(), McpError>` - Ok if the message was sent successfully, or an error if it failed
    async fn send(&self, message: Message) -> Result<(), McpError> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        
        let json = match &message {
            Message::Request(req) => serde_json::to_string(&req)?,
            Message::Response(resp) => serde_json::to_string(&resp)?,
            Message::Notification(notif) => serde_json::to_string(&notif)?,
        };
        
        debug!("Sending message: {}", json);
        writeln!(handle, "{}", json)?;
        handle.flush()?;
        Ok(())
    }

    /// Creates a stream that receives messages from stdin
    /// 
    /// # Returns
    /// * `Pin<Box<dyn Stream<Item = Result<Message, McpError>> + Send + 'static>>` - A stream of messages or errors
    fn receive(&self) -> Pin<Box<dyn Stream<Item = Result<Message, McpError>> + Send + 'static>> {
        let rx = self.tx.subscribe();
        Box::pin(stream::unfold(rx, move |mut rx| async move {
            match rx.recv().await {
                Ok(msg) => Some((Ok(msg), rx)),
                Err(e) => Some((Err(McpError::protocol(ErrorCode::ParseError, e.to_string())), rx)),
            }
        }))
    }

    /// Closes the transport
    /// 
    /// This is a no-op for StdioTransport as the broadcast channel is automatically closed when dropped
    /// 
    /// # Returns
    /// * `Result<(), McpError>` - Always returns Ok
    async fn close(&self) -> Result<(), McpError> {
        // The broadcast channel will be closed when StdioTransport is dropped
        Ok(())
    }
}

/// Provides a default implementation that creates a new StdioTransport
/// 
/// # Panics
/// This implementation will panic if StdioTransport creation fails
impl Default for StdioTransport {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_create_stdio_transport() {
        let transport = StdioTransport::new();
        assert!(transport.is_ok());
    }

    #[test]
    async fn test_send_message() {
        let transport = StdioTransport::new().unwrap();
        let request = Request::new(
            "test".to_string(),
            Some(Value::String("test message".to_string())),
            RequestId::String("test".to_string())
        );
        let message = Message::Request(request);
        let result = transport.send(message).await;
        assert!(result.is_ok());
    }
} 