# Package definitions for nix-inspector-mcp
{pkgs, ...}: let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = pkgs.rust-bin.stable."1.85.1".default;
    rustc = pkgs.rust-bin.stable."1.85.1".default;
  };
in {
  default = rustPlatform.buildRustPackage {
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
    ];

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
