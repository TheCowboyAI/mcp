# Package definitions for nix-inspector-mcp
{
  pkgs,
  naersk-lib,
  ...
}: {
  default = naersk-lib.buildPackage {
    pname = "nix-inspector-mcp";
    version = "0.1.0";
    root = ../../.;

    nativeBuildInputs = with pkgs; [
      pkg-config
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
