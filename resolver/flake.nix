{
  description = "Build a cargo project without extra checks";

  inputs = {
    auxlib.url = "path:../lib";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      auxlib,
      nixpkgs,
      crane,
      ...
    }:
    let
      inherit (auxlib) lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
      pkgsForSystem = system: import nixpkgs { inherit system; };
      mkWrapped =
        system:
        let
          pkgs = pkgsForSystem system;
          craneLib = crane.mkLib pkgs;

          resolver = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            strictDeps = true;
          };
        in
        pkgs.runCommand "resolver-wrapped" { } ''
          . ${pkgs.makeWrapper}/nix-support/setup-hook
          makeWrapper ${resolver}/bin/resolver $out/bin/resolver \
            --prefix PATH : ${pkgs.lix}/bin \
            --prefix PATH : ${pkgs.graphviz-nox}/bin;
        '';
    in
    {
      packages = forAllSystems (system: {
        default = mkWrapped system;
      });
      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${mkWrapped system}/bin/resolver";
        };
      });
    };
}
