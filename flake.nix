{
  description = "A Model Context Protocol server for inspecting Nix systems and flakes";

  # Input sources for this flake
  inputs = {
    # Pinned to 24.11 for stability
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";

    # Rust build support
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    # Systems supported by this flake
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    # Helper function to create system-specific attributes
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);

    # Import our modular components
    packages = import ./modules/lists/packages.nix;
  in {
    # Binary packages for each supported system
    packages = forAllSystems (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = naersk.lib.${system};
      in
        packages {inherit pkgs naersk-lib;}
    );

    # Apps provide a way to run the package
    apps = forAllSystems (system: {
      default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/nix-inspector-mcp";
      };
    });

    # Overlay for adding our packages to nixpkgs
    overlays.default = final: prev: {
      nix-inspector-mcp = self.packages.${prev.system}.default;
    };
  };
}
