# Package definitions for nix-inspector-mcp
{pkgs, ...}: {
  default = pkgs.rustPlatform.buildRustPackage {
    pname = "nix-inspector-mcp";
    version = "0.1.0";
    src = ../../.;

    cargoLock = {
      lockFile = ../../Cargo.lock;
      outputHashes = {
        # Add dependency hashes here if needed
      };
    };

    nativeBuildInputs = with pkgs; [
      pkg-config
      rustPlatform.cargoSetupHook
    ];

    buildInputs = with pkgs; [
      openssl
      nix
      graphviz
    ];

    # Add runtime dependencies
    propagatedBuildInputs = with pkgs; [
      nix
      graphviz
    ];

    # Set environment variables for build
    NIX_BIN_PATH = "${pkgs.nix}/bin/nix";

    meta = with pkgs.lib; {
      description = "A Model Context Protocol server for inspecting Nix systems and flakes";
      homepage = "https://github.com/thecowboyai/mcp";
      license = licenses.mit;
      maintainers = with maintainers; [
        /*
        TODO: Add maintainers
        */
      ];
      platforms = platforms.all;
    };
  };
}
