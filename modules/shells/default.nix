{pkgs, ...}: {
  default = pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      # Rust toolchain from oxalica overlay
      (rust-bin.stable."1.85.1".default.override {
        extensions = ["rust-src"];
        targets = ["x86_64-unknown-linux-gnu"];
      })

      # Build dependencies
      pkg-config
      openssl

      # Development tools
      rustfmt
      clippy

      # Direnv integration
      direnv
      nix-direnv
    ];

    buildInputs = with pkgs; [
      openssl
    ];

    # Environment variables
    shellHook = ''
      echo "Nix development shell for nix-inspector-mcp"

      # Set up direnv
      eval "$(direnv hook $SHELL)"

      # Add local cargo bin to PATH
      export PATH="$PWD/target/debug:$PATH"

      # Set RUST_SRC_PATH for rust-analyzer
      export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/library"

      # Set OPENSSL_DIR for openssl-sys
      export OPENSSL_DIR="${pkgs.openssl.dev}"
      export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
      export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
    '';
  };
}
