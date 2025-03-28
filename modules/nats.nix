{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.cim.nats;
  jsonFormat = pkgs.formats.json { };
in
{
  options.cim.nats = {
    enable = mkEnableOption "local nats server";

    settings = mkOption {
      type = jsonFormat.type;
      default = { };
      example = literalExpression ''
      '';
      description = ''
      '';
    };
  };

  config = mkIf cfg.enable {
    services.nats = {
      enable = true;
      settings = cfg.settings;
    };
  };
}
