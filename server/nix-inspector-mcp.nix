{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.nix-inspector-mcp;
in {
  options.services.nix-inspector-mcp = {
    enable = mkEnableOption "Nix Inspector MCP Server";

    package = mkOption {
      type = types.package;
      default = pkgs.nix-inspector-mcp;
      description = "The Nix Inspector MCP server package to use.";
    };

    logLevel = mkOption {
      type = types.enum ["DEBUG" "INFO" "WARNING" "ERROR"];
      default = "INFO";
      description = "Log level for the Nix Inspector MCP server.";
    };

    logFile = mkOption {
      type = types.str;
      default = "/var/log/nix-inspector-mcp/server.log";
      description = "Path to the log file.";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.nix-inspector-mcp = {
      description = "Nix Inspector MCP Server";
      after = ["network.target" "nix-daemon.service"];
      wantedBy = ["multi-user.target"];

      environment = {
        LOG_LEVEL = cfg.logLevel;
        LOG_FILE = cfg.logFile;
        NIX_PATH = "/nix/var/nix/profiles/per-user/root/channels";
        RUST_LOG = "debug";
      };

      serviceConfig = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/nix-inspector-mcp";
        Restart = "on-failure";
        RestartSec = "5s";
        RuntimeDirectory = "nix-inspector-mcp";
        StateDirectory = "nix-inspector-mcp";
        CacheDirectory = "nix-inspector-mcp";
        User = "nix-inspector-mcp";
        Group = "nix-inspector-mcp";
        DynamicUser = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        ReadWritePaths = [
          "/var/log/nix-inspector-mcp"
          "/var/lib/nix-inspector-mcp"
          "/nix/var/nix/daemon-socket"
        ];
        SupplementaryGroups = ["nix-users"];
      };
    };
  };
}
