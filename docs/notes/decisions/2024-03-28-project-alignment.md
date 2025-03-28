# Project Rules Alignment Decision

## Context
After reviewing all project rules (@code-policy.mdc, @mcp.mdc, @nix-flakes.mdc, @nixmcp.mdc, @nixos.mdc, @rust.mdc, @project-memory.mdc), we need to align our project structure and implementation with these requirements.

## Details
Current misalignments and required changes:

1. Project Memory Structure
   - Missing core files:
     - projectBrief.md
     - projectContext.md
     - domainContext.md
     - systemPatterns.md
     - techContext.md
   - Have progress.md but needs review
   - Need proper notes structure in docs/notes/

2. Rust Implementation
   - Currently using naersk (forbidden)
   - Need to switch to buildRustPackage
   - Need oxalica overlay
   - Missing rust-toolchain.toml (v1.85.1)
   - Need proper project structure:
     - src/main.rs
     - src/lib.rs
     - tests/
     - benches/
     - examples/

3. Nix Configuration
   - Not using template from thecowboyai/flake-templates
   - Need to implement devshell.nix
   - Need to document NixMCP integration
   - Need to reorganize modules structure

4. Code Quality Requirements
   - Need acceptance tests (gherkin)
   - Need to implement event-driven architecture
   - Need subject-based messaging
   - Need functional/reactive programming
   - Need proper error handling/logging

## Decisions
1. Create all missing core memory files first
2. Switch build system from naersk to buildRustPackage
3. Implement proper Rust project structure
4. Update Nix configuration to match requirements
5. Implement required architectural patterns

## Next Steps
- [ ] Create core memory files
- [ ] Switch to buildRustPackage with oxalica overlay
- [ ] Create rust-toolchain.toml
- [ ] Set up proper Rust project structure
- [ ] Implement testing framework
- [ ] Update Nix configuration
- [ ] Document MCP integration

## References
- All project .mdc files
- MCP specification
- Rust documentation
- NixOS documentation 