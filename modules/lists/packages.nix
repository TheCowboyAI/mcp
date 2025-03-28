{
  pkgs,
  rustVersion ? pkgs.rust-bin.stable."1.85.1".default,
  ...
}:
with pkgs; [
  just
  nix
  bash
  zsh
  starship
  xdg-utils

  # Nix
  nix-index
  nixpkgs-fmt
  nixd
  alejandra
  direnv

  # debugger is a MODULE, don't add that here.
  # it adds about 10 minutes to the build.

  # Rust
  rustVersion
  bacon
  openssl.dev
  openssl.out
  openssl
  gnupg

  # cargo
  cargo
  cargo-edit
  cargo-expand
  cargo-udeps
  cargo-whatfeatures
  cargo-generate
  cargo-make
  cargo-edit
]
