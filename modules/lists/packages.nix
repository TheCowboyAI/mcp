{
  pkgs,
  naersk-lib,
  ...
}: {
  default = naersk-lib.buildPackage {
    pname = "nix-inspector-mcp";
    root = ../../.;

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = with pkgs; [
      openssl
    ];
  };
}
