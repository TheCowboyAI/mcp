# System Patterns

## System Architecture
1. MCP Server Layer
   - NixOS MCP Server
   - ArXiv MCP Server
   - Extensible server framework

2. Protocol Layer
   - Model Context Protocol implementation
   - Standardized communication patterns
   - Resource access protocols

3. Integration Layer
   - Cursor IDE rules integration
   - Home Manager integration
   - Flake-based deployment

4. Inspector Layer
   - Nix Inspector MCP Server
     * System analysis capabilities
     * Flake inspection (local/remote)
     * Dependency graph generation
     * Configuration validation
     * Development environment analysis

## Technical Decisions
1. Using Nix Flakes
   - Reason: Reproducible builds and deployments
   - Impact: Consistent development environments
   - Trade-offs: Learning curve vs reliability

2. MCP Implementation
   - Following official MCP specification
   - Using Rust for core implementation
   - Supporting multiple resource types

3. IDE Integration
   - Cursor rules-based integration
   - Direct MCP server communication
   - Resource validation and verification

4. Nix Inspector Implementation
   - Using nix-topology for dependency visualization
   - Integrating nil for Nix language server features
   - Leveraging nix-direnv for environment management
   - Using std for standardized Nix patterns
   - Implementing niv for version management

## Design Patterns
1. Server Pattern
   - Independent server modules
   - Shared resource interfaces
   - Pluggable architecture

2. Resource Pattern
   - Standardized resource access
   - Cached responses
   - Versioned interfaces

3. Integration Pattern
   - Rule-based integration
   - Event-driven updates
   - Stateless operation 