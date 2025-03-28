{ pkgs, buildInputs, packages, env, ... }:
let
  generateShellHook = attrs: builtins.concatStringsSep "\n" (map (key: "export ${key}='${attrs.${key}}'") (builtins.attrNames attrs));
in
pkgs.mkShell {
  name = "cim-devShell";

  buildInputs = buildInputs;

  packages = packages;

  shellHook = ''
    ${generateShellHook env}
  '';

}

