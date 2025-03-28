{
  pkgs,
  toolchain,
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

  # Rust development tools
  bacon
  openssl.dev
  openssl.out
  openssl
  gnupg

  # Additional Rust tools
  cargo-edit
  cargo-expand
  cargo-udeps
  cargo-whatfeatures
  cargo-generate
  cargo-make
]
