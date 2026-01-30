{
  description = "AC Cup Server - Assetto Corsa Content Update Protocol Server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ { flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

      imports = [
        inputs.nci.flakeModule
      ];

      perSystem = { config, pkgs, system, ... }: 
        let
          projectName = "ac-cup-server";
          crateOutputs = config.nci.outputs.${projectName};
        in
        {
          nci.projects.${projectName}.path = ./.;
          nci.crates.${projectName} = { };

          packages = {
            default = crateOutputs.packages.release;
            ac-cup-server = crateOutputs.packages.release;
          };

          devShells.default = crateOutputs.devShell.overrideAttrs (old: {
            packages = (old.packages or [ ]) ++ (with pkgs; [
              rust-analyzer
              cargo-watch
              cargo-edit
            ]);
          });

          apps.default = {
            type = "app";
            program = "${crateOutputs.packages.release}/bin/ac-cup-server";
          };
        };

      flake = {
        nixosModules.default = { config, lib, pkgs, ... }:
          with lib;
          let
            cfg = config.services.ac-cup-server;
          in
          {
            options.services.ac-cup-server = {
              enable = mkEnableOption "AC Cup Server";

              package = mkOption {
                type = types.package;
                default = inputs.self.packages.${pkgs.system}.default;
                defaultText = literalExpression "inputs.self.packages.\${pkgs.system}.default";
                description = "The ac-cup-server package to use.";
              };

              host = mkOption {
                type = types.str;
                default = "0.0.0.0";
                example = "127.0.0.1";
                description = "IP address to bind to.";
              };

              port = mkOption {
                type = types.port;
                default = 3000;
                description = "Port to listen on.";
              };

              storagePath = mkOption {
                type = types.str;
                default = "/var/lib/ac-cup-server/storage.json";
                description = "Path to the storage JSON file.";
              };

              environmentFile = mkOption {
                type = types.nullOr types.path;
                default = null;
                description = "Environment file to load additional configuration from.";
              };

              logLevel = mkOption {
                type = types.str;
                default = "info";
                example = "debug";
                description = "Log level for the service (trace, debug, info, warn, error).";
              };
            };

            config = mkIf cfg.enable {
              systemd.services.ac-cup-server = {
                description = "AC Cup Server";
                wantedBy = [ "multi-user.target" ];
                after = [ "network.target" ];

                serviceConfig = {
                  Type = "simple";
                  DynamicUser = true;
                  StateDirectory = "ac-cup-server";
                  ExecStart = "${cfg.package}/bin/ac-cup-server";
                  Restart = "on-failure";
                  RestartSec = "5s";

                  # Hardening
                  NoNewPrivileges = true;
                  PrivateTmp = true;
                  ProtectSystem = "strict";
                  ProtectHome = true;
                  ProtectKernelTunables = true;
                  ProtectKernelModules = true;
                  ProtectControlGroups = true;
                  RestrictAddressFamilies = [ "AF_INET" "AF_INET6" ];
                  RestrictNamespaces = true;
                  LockPersonality = true;
                  RestrictRealtime = true;
                  RestrictSUIDSGID = true;
                  PrivateDevices = true;
                  ProtectClock = true;
                };

                environment = {
                  HOST = cfg.host;
                  PORT = toString cfg.port;
                  STORAGE_PATH = cfg.storagePath;
                  RUST_LOG = "ac_cup_server=${cfg.logLevel},tower_http=${cfg.logLevel},axum=${cfg.logLevel}";
                };

                inherit (cfg) environmentFile;
              };
            };
          };
      };
    };
}




