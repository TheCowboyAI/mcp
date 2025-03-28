# Creating and Installing MCP Servers on NixOS Using Flakes

Model Context Protocol (MCP) servers provide AI assistants with accurate, up-to-date information about system resources. When implemented on NixOS using the declarative and reproducible Flakes system, these servers become powerful tools for enhancing AI capabilities while maintaining the deterministic nature of Nix. This report explores the available options for implementing MCP servers on NixOS using Flakes.

## This Repository is a already collection of MCP Servers for NixOS along with /rules for Cursor IDE.

## Understanding Model Context Protocol (MCP)

MCP is a structured approach that allows AI systems to communicate effectively with various data sources and tools. Developed by Anthropic and made open-source, it aims to enhance connections between data hubs and AI systems by overcoming traditional data integration limitations[8].

### Key Components of MCP

- **MCP host**: Programs like Claude Desktop, IDEs, or AI tools that access resources through MCP
- **MCP clients**: Protocol clients maintaining 1:1 connections with servers
- **MCP servers**: Lightweight programs exposing specific capabilities through the standardized protocol
- **Local/Remote resources**: Your computer's resources or internet-accessible resources that MCP servers can access[8]

### USAGE ###

In Composer,

```bash
"start a new server project"
```

Ask any questions about design and architecture. Add Notes and compile them into knowledge graphs and workflows.

Scrape web pages and research information, then compile your project.

### Development Workflow

This project follows a feature branch workflow:

- Developers work on personal branches named after themselves during active development sessions
- Changes are merged back to `main` when development is complete
- Pull requests are required for merging changes after official releases
- The `main` branch represents the stable, production-ready state of the project

For more details, see [Git Branching Strategy](docs/notes/decisions/2024-03-28-git-branching-strategy.md).

## Available MCP Server Implementations for NixOS

### NixMCP

NixMCP is a dedicated Model Context Protocol server that exposes NixOS packages, system options, Home Manager configuration options, and nix-darwin macOS configuration options to AI models. It provides AI models with up-to-date information, significantly reducing hallucinations and outdated information[2].

#### Quick Setup

To get started with NixMCP, you can add the following to your MCP configuration file:

```json
{
  "mcpServers": {
    "nixos": {
      "command": "uvx",
      "args": ["nixmcp"],
      "env": {
        "LOG_LEVEL": "INFO"
      }
    }
  }
}
```

This configuration enables your AI assistant to access correct information about NixOS instead of potentially hallucinating package names from outdated sources[2].

### Minimal MCP in Nix

For those seeking a simpler implementation, Minimal MCP in Nix offers a barebones MCP server implemented in Python, designed to work seamlessly with Nix package management[7].

#### Key Features

- Simple implementation of an MCP server in Python
- Nix flake integration for easy setup and management
- Provides a stub location for demo purposes[7]

#### How to Use

To use this project, you can initialize it using the provided Nix flake template and run the server with the specified command in your Claude configuration[7].

### Community-Developed MCP Servers

The NixOS community has been actively developing MCP servers tailored for specific use cases:

#### nixmcp (github.com/utens/nixmcp)

A simple MCP server specifically for NixOS packages and options, designed to help AI agents provide accurate options without inventing non-existent ones[11]. This project aims to refine AI interactions with NixOS configurations.

## Implementing MCP Servers Using Flakes

### Why Flakes for MCP?

Nix flakes are an excellent match for MCP implementations as they provide:

1. Reproducible builds and dependencies
2. Compatibility with both macOS and Linux
3. Atomic updates and rollbacks
4. Clear definition of inputs and outputs[11]

### Architecture Approaches

A promising approach to organizing MCP servers in NixOS involves:

- Developing a collection of `flake.mcpConfigurations` that represent various MCP server groupings
- Creating `flake.mcpModules` that correspond to individual servers
- Integrating with Home Manager for user-specific configurations[11]

### Basic Flake Structure

A basic flake for an MCP server might look similar to this NixOS flake structure:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    # Additional inputs as needed
  };

  outputs = { self, nixpkgs }: {
    # MCP server configurations
    mcpConfigurations = {
      # Server configurations
    };
    
    # Package definitions
    packages.x86_64-linux.default = # MCP server package
  };
}
```

## Debugging MCP Servers on NixOS

When running MCP servers on NixOS, you might encounter PATH-related issues, particularly when using Nix-managed tools with Claude's MCP server[4].

### Common Issues

The `ENOENT` error typically indicates that an executable couldn't be found. This often happens because the shell's `PATH` differs from Claude's, and Nix-managed executables aren't available in the MCP server environment[4].

### Solution

Modify the MCP server configuration to extend the `PATH` environment variable:

```json
"filesystem": {
  "command": "/bin/sh",
  "args": [
    "-c",
    "PATH=/run/current-system/sw/bin:$PATH exec npx -y @modelcontextprotocol/server-filesystem /Users/b/Desktop /Users/b/Documents /Users/b/Downloads"
  ]
}
```

This solution allows Claude to work seamlessly with Nix-managed development environments while maintaining the isolation and reproducibility that Nix provides[4].

## Integration with Home Manager

For a complete MCP setup, integrating with Home Manager allows for user-specific configurations. A fork of CodeMCP that includes a flake and a Home Manager module is available for those interested in this approach[11].

## Conclusion

Implementing MCP servers on NixOS using Flakes offers a powerful combination of AI capability enhancement and system reproducibility. With multiple implementation options ranging from comprehensive solutions like NixMCP to minimal implementations, developers can choose the approach that best fits their needs.

The growing community interest in this area promises further improvements in the integration between AI systems and NixOS, with potential for more specialized MCP servers and better integration with existing tools.

For those looking to get started, the GitHub repositories mentioned (utens/nixmcp, minimal-mcp-in-nix) provide excellent starting points, while the Flake documentation in the NixOS wiki offers guidance on the general structure and management of Flakes.

Citations:
[1] https://www.youtube.com/watch?v=Fph7SMldxpI
[2] https://github.com/utensils/nixmcp
[3] https://www.youtube.com/watch?v=CA8V2hEIxCc
[4] https://newschematic.org/blog/debugging-claude-mcp-nix/
[5] https://discourse.nixos.org/t/flakes-for-noobs/39368
[6] https://github.com/mccartykim/minimal-mcp-in-nix
[7] https://mcp.so/server/minimal-mcp-in-nix
[8] https://memo.d.foundation/playground/ai/model-context-protocol/
[9] https://github.com/nix-community/mineflake
[10] https://nixos.wiki/wiki/Flakes
[11] https://www.reddit.com/r/NixOS/comments/1jjv62s/nixos_mcp/
[12] https://discourse.nixos.org/t/proper-way-to-build-a-remote-system-with-flakes/17661
[13] https://discourse.nixos.org/t/one-flake-to-rule-them-all/24039
[14] https://chengeric.com/homelab/
[15] https://nixos-and-flakes.thiscute.world/nixos-with-flakes/modularize-the-configuration
[16] https://www.reddit.com/r/mcp/comments/1jkwiu5/nixmcp_model_context_protocol_for_nixos_resources/
[17] https://model-context-protocol.com/servers/nix-mcp-server-flake
[18] https://www.reddit.com/r/admincraft/comments/1i0f621/host_minecraft_server_declaratively_with_nixos/
[19] https://jvns.ca/blog/2023/11/11/notes-on-nix-flakes/
[20] https://nixos.org/nixpkgs/manual/
[21] https://github.com/storopoli/flakes
[22] https://discourse.nixos.org/t/howto-setting-up-a-nixos-minecraft-server-using-the-newest-version-of-minecraft/3445
[23] https://discourse.nixos.org/t/nix-from-first-principles-flake-edition/23109
[24] https://flakehub.com/flake/lessuselesss/nix-mcp-servers
[25] https://github.com/mkaito/nixos-modded-minecraft-servers/blob/master/flake.nix
[26] https://mikelev.in/futureproof/python-mcp-server-example/

[25] https://github.com/mkaito/nixos-modded-minecraft-servers/blob/master/flake.nix
[26] https://mikelev.in/futureproof/python-mcp-server-example/



