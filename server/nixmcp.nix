{
  config,
  lib,
  pkgs,
  ...
}: let
  inherit (lib) mkEnableOption mkOption types;
in {
  options.services.nixmcp = {
    enable = mkEnableOption "nixmcp";

    logLevel = mkOption {
      type = types.str;
      default = "INFO";
      description = "Logging level for nixmcp server";
    };

    logFile = mkOption {
      type = types.path;
      default = "/var/log/nixmcp/nixmcp-server.log";
      description = "Path to log file";
    };

    package = mkOption {
      type = types.package;
      default = pkgs.python3Packages.nixmcp;
      description = "The nixmcp package to use";
    };
  };

  config = lib.mkIf config.services.nixmcp.enable {
    programs.nix-ld = {
      enable = true;
      libraries = with pkgs; [
        stdenv.cc.cc.lib
        zlib
        python3
      ];
    };

    environment.systemPackages = with pkgs; [
      python3Packages.uv
    ];

    systemd.tmpfiles.rules = [
      "d /var/log/nixmcp 0777 nixmcp nixmcp -"
      "d /var/lib/nixmcp 0777 nixmcp nixmcp -"
      "d /var/lib/nixmcp/nixmcp 0777 nixmcp nixmcp -"
    ];

    systemd.services.nixmcp = {
      description = "NixMCP Server";
      after = ["network.target"];
      wantedBy = ["multi-user.target"];

      preStart = ''
        echo "fastmcp>=0.1.0" > /var/lib/nixmcp/requirements.txt
        ${pkgs.python3Packages.uv}/bin/uv pip install -r /var/lib/nixmcp/requirements.txt
      '';

      environment = {
        LOG_LEVEL = config.services.nixmcp.logLevel;
        LOG_FILE = config.services.nixmcp.logFile;
        PYTHONPATH = "/var/lib/nixmcp";
        UV_CACHE_DIR = "/var/lib/nixmcp/cache/uv";
        NIX_LD_LIBRARY_PATH = lib.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.zlib
          pkgs.python3
        ];
        NIX_LD = lib.fileContents "${pkgs.stdenv.cc}/nix-support/dynamic-linker";
      };

      serviceConfig = {
        Type = "simple";
        ExecStart = "${pkgs.python3Packages.uv}/bin/uvx nixmcp";
        Restart = "on-failure";
        RestartSec = "5s";
        RuntimeDirectory = "nixmcp";
        RuntimeDirectoryMode = "0777";
        StateDirectory = "nixmcp";
        StateDirectoryMode = "0777";
        CacheDirectory = "nixmcp";
        CacheDirectoryMode = "0777";
        User = "nixmcp";
        Group = "nixmcp";
        DynamicUser = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        ReadWritePaths = [
          "/var/log/nixmcp"
          "/var/lib/nixmcp"
        ];
        WorkingDirectory = "/var/lib/nixmcp";
      };
    };
  };
}
