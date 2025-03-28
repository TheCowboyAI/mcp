# Nix Inspector MCP API Reference

## Overview
The Nix Inspector MCP server provides a Model Context Protocol interface for analyzing NixOS systems and development flakes.

## Endpoints

### 1. System Information
Get information about the running NixOS system.

**Method:** `get_system_info`
**Parameters:** None

**Response:**
```json
{
  "nixos_version": "string",
  "system_flake": "string?",
  "current_system": "string",
  "nix_version": "string",
  "store_path": "string"
}
```

### 2. Development Flake Analysis
Analyze a development flake in the current directory.

**Method:** `analyze_dev_flake`
**Parameters:** None

**Response:**
```json
{
  "path": "string",
  "locked_inputs": ["string"],
  "outputs": ["string"],
  "dev_shells": ["string"]
}
```

### 3. Flake Graph Generation
Generate a visual representation of the current flake's dependency structure.

**Method:** `generate_flake_graph`
**Parameters:** 
```json
{
  "format": "string?"  // Optional: "svg" to include SVG output
}
```

**Response:**
```json
{
  "dot_graph": "string",      // DOT format graph
  "svg_output": "string?",    // Optional SVG rendering
  "nodes": ["string"],        // List of graph nodes
  "edges": [                  // List of edges
    ["string", "string"]      // [from, to] pairs
  ]
}
```

**Example:**
```bash
# Generate DOT graph
curl -X POST http://localhost:8080/api \
  -H "Content-Type: application/json" \
  -d '{
    "method": "generate_flake_graph",
    "params": {}
  }'

# Generate with SVG
curl -X POST http://localhost:8080/api \
  -H "Content-Type: application/json" \
  -d '{
    "method": "generate_flake_graph",
    "params": {"format": "svg"}
  }'
```

## Error Responses

```json
{
  "error": {
    "code": "number",
    "message": "string",
    "data": "any?"
  }
}
```

### Error Codes
- `1`: NixCommandError - Failed to execute Nix command
- `2`: ParseError - Failed to parse command output
- `3`: MethodNotFound - Invalid method requested

## Usage Examples

### Curl Examples
```bash
# Get system information
curl -X POST http://localhost:8080/api \
  -H "Content-Type: application/json" \
  -d '{
    "method": "get_system_info",
    "params": {}
  }'

# Analyze development flake
curl -X POST http://localhost:8080/api \
  -H "Content-Type: application/json" \
  -d '{
    "method": "analyze_dev_flake",
    "params": {}
  }'
```

### Python Example
```python
import requests

def get_system_info():
    response = requests.post(
        "http://localhost:8080/api",
        json={
            "method": "get_system_info",
            "params": {}
        }
    )
    return response.json()
```

### Rust Example
```rust
use mcp_rust_sdk::Client;

async fn get_system_info() -> Result<SystemInfo, Error> {
    let client = Client::new("http://localhost:8080")?;
    let response = client.request("get_system_info", None).await?;
    Ok(serde_json::from_value(response)?)
}
```

## Security Considerations
- Server runs with limited permissions
- Access restricted to specific paths
- Uses systemd sandboxing
- Requires nix-users group membership

## Rate Limiting
- Default: 100 requests per minute
- Configurable through NixOS module

## See Also
- [Project Documentation](../README.md)
- [Implementation Notes](../notes/2024-02-nix-inspector-implementation.md)
- [MCP Specification](https://spec.modelcontextprotocol.io/specification/) 