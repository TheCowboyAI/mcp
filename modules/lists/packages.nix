{ pkgs, ... }:
with pkgs;
[
  just
  nix
  bash
  zsh
  starship
  helix
  xdg-utils
  socat

  # Nix
  nix-index
  nixpkgs-fmt
  nixd
  alejandra
  direnv

  # debugger is a MODULE, don't add that here.
  # it adds about 10 minutes to the build.

  # Rust
  rust-bin.stable.latest.default
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

  # wasm
  wasmtime
  wasmserve
  wasm-tools
  wasm-pack
  alsa-lib
  mesa
  libdrm

  wayland
  wl-clipboard
  spice-vdagent
  wayland-proxy-virtwl
  waypipe
  sway
  kitty
]
