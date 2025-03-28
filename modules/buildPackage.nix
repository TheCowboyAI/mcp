{
  pkgs,
  pkg,
  buildInputs,
  src,
  env,
  toolchain,
}: let
  generateShellHook = attrs:
    builtins.concatStringsSep "\n" (
      map (key: "export ${key}='${attrs.${key}}'") (builtins.attrNames attrs)
    );
in
  pkgs.rustPlatform.buildRustPackage {
    buildType = "debug";

    pname = pkg.package.name;
    version = pkg.package.version;
    src = src;
    cargoLock.lockFile = "${src}/Cargo.lock";
    buildInputs = buildInputs ++ [toolchain pkgs.nix];
    nativeBuildInputs = [toolchain pkgs.nix];

    cargoHash = "";

    buildPhase = ''
      ${generateShellHook env}
      export NIX_BIN_PATH=${pkgs.nix}/bin/nix
      cargo build
    '';

    checkPhase = ''
      export NIX_BIN_PATH=${pkgs.nix}/bin/nix
      cargo test
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp "target/debug/${pkg.package.name}" $out/bin/
    '';

    meta = with pkgs.lib; {
      description = pkg.package.description;
      license = licenses.mit;
      maintainers = with maintainers; pkg.package.authors;
    };
  }
