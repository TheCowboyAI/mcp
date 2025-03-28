{
  description = "A Model Context Protocol server for inspecting Nix systems and flakes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    # Systems supported
    systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

    # Helper function to create system-specific attributes
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);

    # Import our modules
    packages = import ./modules/lists/packages.nix;
  in {
    packages = forAllSystems (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = naersk.lib.${system};
      in
        packages {inherit pkgs naersk-lib;}
    );

    apps = forAllSystems (system: {
      default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/nix-inspector-mcp";
      };
    });
  };
}
