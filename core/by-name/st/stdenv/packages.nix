# This should contain the actual bootstrapping code, but for now we're cheating and using nixpkgs
{ lib, config, ... }:
let
  nixpkgs = import config.removeMe { inherit (config) system; };
in
{
  stdenv = _: nixpkgs.stdenv;
  fetchurl = _: nixpkgs.fetchurl;
  gcc = _: nixpkgs.gcc;
}
