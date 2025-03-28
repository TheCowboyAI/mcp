{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    src = ./.;
    pkg = nixpkgs.lib.importTOML ./Cargo.toml;
    pkgName = pkg.package.name;
    buildInputs = import ./modules/lists/buildInputs.nix {
      inherit pkgs toolchain;
    };
    env = import ./modules/sets/env.nix {inherit pkgs buildInputs;};
    shellpackages = import ./modules/lists/packages.nix {
      inherit pkgs toolchain;
    };
    devshell = import ./modules/shells/devshell.nix {
      inherit pkgs buildInputs env;
      packages = shellpackages ++ toolchain.packages;
    };
    configurationModule = import ./modules/configuration.nix {
      inherit pkgs buildInputs;
      packages = shellpackages;
    };
    buildPkg = import ./modules/buildPackage.nix {
      inherit
        pkgs
        pkg
        buildInputs
        src
        env
        toolchain
        ;
    };
  in {
    packages.${system} = {
      ${pkgName} = buildPkg;
      default = self.packages.${system}.${pkgName};
    };

    apps.${system} = {
      ${pkgName} = flake-utils.lib.mkApp {
        drv = self.packages.${system}.${pkgName};
      };
      default = self.apps.${system}.${pkgName};
    };

    devShells.${system}.default = devshell;

    # nixosConfigurations."${pkgName}-container" = nixpkgs.lib.nixosSystem {
    #   inherit system;
    #   modules = [
    #     configurationModule
    #     {
    #       environment.systemPackages = [self.packages.${system}.${pkgName}];
    #       nixpkgs.overlays = [(import rust-overlay)];
    #     }
    #   ];
    # };
  };
}
