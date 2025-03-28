pub mod system;
pub mod flake;
pub mod environment;
pub mod validation;

use mcp_rust_sdk::prelude::*;
use std::error::Error;

pub struct NixInspector {
    // Core components
    system_analyzer: system::SystemAnalyzer,
    flake_inspector: flake::FlakeInspector,
    env_manager: environment::EnvironmentManager,
    validator: validation::ConfigValidator,
}

impl NixInspector {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            system_analyzer: system::SystemAnalyzer::new()?,
            flake_inspector: flake::FlakeInspector::new()?,
            env_manager: environment::EnvironmentManager::new()?,
            validator: validation::ConfigValidator::new()?,
        })
    }
} 