pub mod system;
pub mod flake;
pub mod environment;
pub mod validation;

use std::error::Error as StdError;

pub use system::*;

pub struct NixInspector {
    env_manager: environment::EnvironmentManager,
    validator: validation::ConfigValidator,
}

impl NixInspector {
    pub fn new() -> Result<Self, Box<dyn StdError>> {
        Ok(Self {
            env_manager: environment::EnvironmentManager::new()?,
            validator: validation::ConfigValidator::new()?,
        })
    }
} 