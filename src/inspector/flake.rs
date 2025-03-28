use std::path::PathBuf;

pub struct FlakeInspector {
    flake_path: PathBuf,
}

impl FlakeInspector {
    pub fn new(flake_path: PathBuf) -> Self {
        Self { flake_path }
    }
} 