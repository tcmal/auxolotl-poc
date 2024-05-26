# This should contain the actual bootstrapping code, but for now we're cheating and using nixpkgs
{ lib, config, ... }:
let
  nixpkgs = import config.removeMe { inherit (config) system; };
in
{
  phase0 = {
    bootstrap-tools = _: throw "todo";
    # Specifying a dependency on `phase0.bootstrap-tools`
    stdenv =
      {
        phase0 ? {
          bootstrap-tools = { };
        },
        ...

      }:
      throw "todo";
  };

  phase1 = {
    stdenv =
      {
        phase0 ? {
          stdenv = { };
        },
        ...

      }:
      throw "todo";
  };

  # omitted: there are actually 5 phases of bootstrapping on linux, but we're not bothering with that right now.
  stdenv =
    {
      phase1 ? {
        stdenv = { };
      },
    }:
    nixpkgs.stdenv;
}
