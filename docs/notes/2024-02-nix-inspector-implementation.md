# Nix Inspector MCP Implementation Notes

## Context
The nix-inspector-mcp server provides comprehensive system and flake analysis capabilities for NixOS environments through the Model Context Protocol.

## Details

### Core Capabilities

1. System Analysis
   ```rust
   // Example request
   {
     "method": "get_system_info",
     "params": {}
   }
   
   // Example response
   {
     "nixos_version": "24.11",
     "system_flake": "/nix/store/...-nixos-system",
     "current_system": "x86_64-linux",
     "nix_version": "2.25.0",
     "store_path": "/nix/store"
   }
   ```

2. Flake Analysis
   ```rust
   // Example request
   {
     "method": "analyze_dev_flake",
     "params": {}
   }
   
   // Example response
   {
     "path": "/path/to/current/flake",
     "locked_inputs": [
       "nixpkgs",
       "nix-topology",
       "nil"
     ],
     "outputs": [
       "packages.x86_64-linux.default",
       "devShells.x86_64-linux.default"
     ],
     "dev_shells": [
       "devShell.x86_64-linux"
     ]
   }
   ```

3. Flake Graph Generation
   ```rust
   // Example request
   {
     "method": "generate_flake_graph",
     "params": {
       "format": "svg"
     }
   }
   
   // Example response
   {
     "dot_graph": "digraph { ... }",
     "svg_output": "<svg>...</svg>",
     "nodes": ["nixpkgs", "home-manager"],
     "edges": [
       ["root", "nixpkgs"],
       ["root", "home-manager"]
     ]
   }
   ```

### Integration Points

1. Nix Commands
   - `nix eval` - System information queries
   - `nix flake` - Flake analysis and metadata
   - `nix show` - Output inspection
   - `nix develop` - Development environment analysis
   - `nix-topology` - Flake dependency visualization

2. External Tools
   - nix-topology: Dependency graph generation
   - nil: Nix language server features
   - nix-direnv: Environment detection
   - std: Pattern validation

### Error Handling

```rust
pub enum SystemAnalyzerError {
    NixCommandError(String),  // Nix command execution failures
    ParseError(String),       // JSON parsing errors
}
```

## Usage Examples

1. Basic System Information
   ```bash
   curl -X POST http://localhost:8080/api \
     -H "Content-Type: application/json" \
     -d '{"method": "get_system_info", "params": {}}'
   ```

2. Development Flake Analysis
   ```bash
   curl -X POST http://localhost:8080/api \
     -H "Content-Type: application/json" \
     -d '{"method": "analyze_dev_flake", "params": {}}'
   ```

## Configuration

NixOS module configuration:
```nix
{
  services.nix-inspector-mcp = {
    enable = true;
    logLevel = "INFO";
    logFile = "/var/log/nix-inspector-mcp/server.log";
  };
}
```

## Security Considerations

1. File System Access
   - Limited to specific paths:
     * `/nix/store`
     * `/nix/var/nix/daemon-socket`
     * `/var/log/nix-inspector-mcp`
     * `/var/lib/nix-inspector-mcp`

2. User Permissions
   - Runs as dedicated system user
   - Part of `nix-users` group
   - Uses systemd sandboxing

## Testing Strategy

1. Unit Tests
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[tokio::test]
       async fn test_system_info() {
           let analyzer = SystemAnalyzer::new().unwrap();
           let info = analyzer.get_system_info().await.unwrap();
           assert!(!info.nixos_version.is_empty());
       }
   }
   ```

2. Integration Tests
   - System analysis validation
   - Flake inspection verification
   - Error handling scenarios
   - Performance benchmarks

## Next Steps

1. Tool Integration
   - [ ] Implement nix-topology graph generation
   - [ ] Add nil language server support
   - [ ] Integrate nix-direnv detection

2. Feature Expansion
   - [ ] Add system resource monitoring
   - [ ] Implement configuration validation
   - [ ] Add package dependency analysis

3. Documentation
   - [ ] Create API reference
   - [ ] Write usage guides
   - [ ] Add example configurations

## References
- [MCP Specification](https://spec.modelcontextprotocol.io/specification/)
- [Nix Manual](https://nixos.org/manual/nix/stable/)
- [nix-topology](https://github.com/oddlama/nix-topology)
- [nil](https://github.com/oxalica/nil)
- [nix-direnv](https://github.com/nix-community/nix-direnv) 