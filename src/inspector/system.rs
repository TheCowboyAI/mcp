use async_trait::async_trait;
use mcp_rust_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::process::Command;
use thiserror::Error;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum SystemAnalyzerError {
    #[error("Failed to execute nix command: {0}")]
    NixCommandError(String),
    #[error("Failed to parse nix output: {0}")]
    ParseError(String),
    #[error("Failed to generate graph: {0}")]
    GraphError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    nixos_version: String,
    system_flake: Option<String>,
    current_system: String,
    nix_version: String,
    store_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlakeInfo {
    path: String,
    locked_inputs: Vec<String>,
    outputs: Vec<String>,
    dev_shells: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlakeGraph {
    dot_graph: String,
    svg_output: Option<String>,
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

pub struct SystemAnalyzer {
    nix_cmd: String,
}

impl SystemAnalyzer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            nix_cmd: which::which("nix")?.to_string_lossy().to_string(),
        })
    }

    /// Get information about the running NixOS system
    pub async fn get_system_info(&self) -> Result<SystemInfo, SystemAnalyzerError> {
        // Get NixOS version
        let nixos_version = self.run_nix_command(&["eval", "nixosVersion"])?;
        
        // Get current system
        let current_system = self.run_nix_command(&["eval", "--raw", "system"])?;
        
        // Get Nix version
        let nix_version = self.run_nix_command(&["--version"])?;
        
        // Get store path
        let store_path = self.run_nix_command(&["eval", "--raw", "storeDir"])?;

        // Try to get system flake if available
        let system_flake = self.get_system_flake().ok();

        Ok(SystemInfo {
            nixos_version,
            system_flake,
            current_system,
            nix_version,
            store_path,
        })
    }

    /// Analyze a development flake in the current directory
    pub async fn analyze_dev_flake(&self) -> Result<FlakeInfo, SystemAnalyzerError> {
        // Check if we're in a flake
        if !std::path::Path::new("flake.nix").exists() {
            return Err(SystemAnalyzerError::NixCommandError(
                "No flake.nix found in current directory".to_string(),
            ));
        }

        // Get locked inputs
        let locked_inputs = self.run_nix_command(&["flake", "metadata", "--json"])?;
        let inputs: Vec<String> = serde_json::from_str(&locked_inputs)
            .map_err(|e| SystemAnalyzerError::ParseError(e.to_string()))?;

        // Get outputs
        let outputs = self.run_nix_command(&["flake", "show", "--json"])?;
        let output_list: Vec<String> = serde_json::from_str(&outputs)
            .map_err(|e| SystemAnalyzerError::ParseError(e.to_string()))?;

        // Get development shells
        let dev_shells = self.get_dev_shells()?;

        Ok(FlakeInfo {
            path: std::env::current_dir()?.to_string_lossy().to_string(),
            locked_inputs: inputs,
            outputs: output_list,
            dev_shells,
        })
    }

    /// Generate a graph of the current flake's dependency structure
    pub async fn generate_flake_graph(&self, output_format: Option<&str>) -> Result<FlakeGraph, SystemAnalyzerError> {
        // Ensure we're in a flake directory
        if !std::path::Path::new("flake.nix").exists() {
            return Err(SystemAnalyzerError::NixCommandError(
                "No flake.nix found in current directory".to_string(),
            ));
        }

        // Generate DOT graph using nix-topology
        let dot_graph = self.run_nix_command(&[
            "run",
            "github:oddlama/nix-topology",
            "--",
            "generate",
            ".",
            "--format=dot"
        ])?;

        // Parse nodes and edges from DOT output
        let (nodes, edges) = self.parse_dot_graph(&dot_graph)?;

        // Generate SVG if requested
        let svg_output = if output_format == Some("svg") {
            Some(self.dot_to_svg(&dot_graph)?)
        } else {
            None
        };

        Ok(FlakeGraph {
            dot_graph,
            svg_output,
            nodes,
            edges,
        })
    }

    // Helper methods
    fn run_nix_command(&self, args: &[&str]) -> Result<String, SystemAnalyzerError> {
        let output = Command::new(&self.nix_cmd)
            .args(args)
            .output()
            .map_err(|e| SystemAnalyzerError::NixCommandError(e.to_string()))?;

        if !output.status.success() {
            return Err(SystemAnalyzerError::NixCommandError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_system_flake(&self) -> Result<String, SystemAnalyzerError> {
        self.run_nix_command(&["eval", "--raw", "nixosConfig.system.build.toplevel.drvPath"])
    }

    fn get_dev_shells(&self) -> Result<Vec<String>, SystemAnalyzerError> {
        let shells = self.run_nix_command(&["flake", "show", "--json", "--allow-import-from-derivation"])?;
        let shell_data: serde_json::Value = serde_json::from_str(&shells)
            .map_err(|e| SystemAnalyzerError::ParseError(e.to_string()))?;
        
        // Extract devShell outputs
        let mut dev_shells = Vec::new();
        if let Some(obj) = shell_data.as_object() {
            for (key, value) in obj {
                if key.starts_with("devShell.") {
                    dev_shells.push(key.clone());
                }
            }
        }

        Ok(dev_shells)
    }

    // Helper method to parse DOT graph
    fn parse_dot_graph(&self, dot_graph: &str) -> Result<(Vec<String>, Vec<(String, String)>), SystemAnalyzerError> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for line in dot_graph.lines() {
            let line = line.trim();
            if line.ends_with(";") {
                if line.contains("->") {
                    // Parse edge
                    let parts: Vec<&str> = line.split("->").collect();
                    if parts.len() == 2 {
                        let from = parts[0].trim().to_string();
                        let to = parts[1].split(";").next().unwrap_or("").trim().to_string();
                        edges.push((from, to));
                    }
                } else if !line.starts_with("{") && !line.starts_with("}") {
                    // Parse node
                    let node = line.split(";").next().unwrap_or("").trim().to_string();
                    if !node.is_empty() {
                        nodes.push(node);
                    }
                }
            }
        }

        Ok((nodes, edges))
    }

    // Helper method to convert DOT to SVG
    fn dot_to_svg(&self, dot_graph: &str) -> Result<String, SystemAnalyzerError> {
        let mut child = std::process::Command::new("dot")
            .arg("-Tsvg")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| SystemAnalyzerError::GraphError(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(dot_graph.as_bytes())
                .map_err(|e| SystemAnalyzerError::GraphError(e.to_string()))?;
        }

        let output = child.wait_with_output()
            .map_err(|e| SystemAnalyzerError::GraphError(e.to_string()))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[async_trait]
impl ResourceProvider for SystemAnalyzer {
    async fn handle_request(&self, request: &Request) -> Result<Response, Error> {
        match request.method.as_str() {
            "get_system_info" => {
                let info = self.get_system_info().await?;
                Ok(Response::new(serde_json::to_value(info)?))
            }
            "analyze_dev_flake" => {
                let info = self.analyze_dev_flake().await?;
                Ok(Response::new(serde_json::to_value(info)?))
            }
            "generate_flake_graph" => {
                let params: Option<HashMap<String, String>> = request.params.clone()
                    .map(|v| serde_json::from_value(v))
                    .transpose()?;
                
                let output_format = params
                    .and_then(|p| p.get("format"))
                    .map(String::as_str);

                let graph = self.generate_flake_graph(output_format).await?;
                Ok(Response::new(serde_json::to_value(graph)?))
            }
            _ => Err(Error::MethodNotFound),
        }
    }
} 