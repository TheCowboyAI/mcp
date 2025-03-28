{
  config,
  lib,
  pkgs,
  ...
}: let
  inherit (lib) mkEnableOption mkOption types;
in {
  options.services.arxivmcp = {
    enable = mkEnableOption "arxivmcp";

    logLevel = mkOption {
      type = types.str;
      default = "INFO";
      description = "Logging level for arxivmcp server";
    };

    logFile = mkOption {
      type = types.path;
      default = "/var/log/arxivmcp/server.log";
      description = "Path to log file";
    };

    storagePath = mkOption {
      type = types.path;
      default = "/var/lib/arxivmcp/papers";
      description = "Path where papers will be stored";
    };

    package = mkOption {
      type = types.package;
      default = pkgs.python3Packages.arxivmcp;
      description = "The arxivmcp package to use";
    };
  };

  config = lib.mkIf config.services.arxivmcp.enable {
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
      "d /var/log/arxivmcp 0777 arxivmcp arxivmcp -"
      "d /var/lib/arxivmcp 0777 arxivmcp arxivmcp -"
      "d /var/lib/arxivmcp/papers 0777 arxivmcp arxivmcp -"
    ];

    systemd.services.arxivmcp = {
      description = "ArXiv MCP Server";
      after = ["network.target"];
      wantedBy = ["multi-user.target"];

      preStart = ''
        echo "arxivmcp>=0.1.0" > /var/lib/arxivmcp/requirements.txt
        ${pkgs.python3Packages.uv}/bin/uv pip install -r /var/lib/arxivmcp/requirements.txt
      '';

      environment = {
        LOG_LEVEL = config.services.arxivmcp.logLevel;
        LOG_FILE = config.services.arxivmcp.logFile;
        PYTHONPATH = "/var/lib/arxivmcp";
        UV_CACHE_DIR = "/var/lib/arxivmcp/cache/uv";
        NIX_LD_LIBRARY_PATH = lib.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.zlib
          pkgs.python3
        ];
        NIX_LD = lib.fileContents "${pkgs.stdenv.cc}/nix-support/dynamic-linker";
      };

      serviceConfig = {
        Type = "simple";
        ExecStart = "${pkgs.python3Packages.uv}/bin/uvx arxivmcp --storage-path ${config.services.arxivmcp.storagePath}";
        Restart = "on-failure";
        RestartSec = "5s";
        RuntimeDirectory = "arxivmcp";
        RuntimeDirectoryMode = "0777";
        StateDirectory = "arxivmcp";
        StateDirectoryMode = "0777";
        CacheDirectory = "arxivmcp";
        CacheDirectoryMode = "0777";
        User = "arxivmcp";
        Group = "arxivmcp";
        DynamicUser = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        ReadWritePaths = [
          "/var/log/arxivmcp"
          "/var/lib/arxivmcp"
        ];
        WorkingDirectory = "/var/lib/arxivmcp";
      };
    };
  };
}
