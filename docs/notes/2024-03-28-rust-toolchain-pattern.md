## Context
Establishing the correct pattern for managing Rust toolchain in Nix configurations.

## Details
The correct pattern for managing Rust toolchain in Nix configurations is to use `rust-bin.fromRustupToolchainFile` to read directly from rust-toolchain.toml. This approach:

1. **Single Source of Truth**
   - Uses rust-toolchain.toml as the definitive source
   - Avoids duplicating toolchain configuration
   - Maintains consistency with standard Rust development practices

2. **Implementation Pattern**
   ```nix
   let
     system = "x86_64-linux";
     overlays = [(import rust-overlay)];
     pkgs = import nixpkgs { inherit system overlays; };
     toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
   in {
     devShells.${system}.default = pkgs.mkShell {
       buildInputs = [
         toolchain
       ] ++ toolchain.packages;  # This adds all specified components
     };
   }
   ```

3. **Key Components**
   - Use `rust-overlay` from oxalica
   - Read toolchain configuration from rust-toolchain.toml
   - Access components via toolchain.packages
   - Pass toolchain to buildInputs and packages lists

4. **Benefits**
   - Automatic component management
   - Consistent with rustup behavior
   - Easier maintenance
   - Better integration with Rust ecosystem

## Decisions
1. Always use rust-toolchain.toml as source of truth
2. Use fromRustupToolchainFile instead of manual configuration
3. Include toolchain.packages in development shell
4. Pass toolchain to both buildInputs and packages

## Next Steps
1. Update any existing configurations to use this pattern
2. Document pattern in development guides
3. Add validation to ensure pattern is followed

## References
- rust-toolchain.toml
- flake.nix
- modules/lists/buildInputs.nix
- modules/lists/packages.nix 