pub mod inspector;

pub use inspector::system::SystemAnalyzer;

/// Re-export common types used in the public API
pub use mcp_rust_sdk::{Server, ServerBuilder}; 