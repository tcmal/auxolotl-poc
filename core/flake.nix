{
  inputs = {
    auxlib.url = "path:../lib";
    nixpkgs.url = "github:nixos/nixpkgs/23.11";
  };

  outputs =
    {
      self,
      auxlib,
      nixpkgs,
      ...
    }:
    let
      inherit (auxlib) lib;
      config = {
        system = "x86_64-linux";
        # etc

        # Used in some places avoid writing a bunch of code that would distract from the actual POC
        removeMe = nixpkgs;
      };
    in
    {
      lambdas = import ./default.nix { inherit lib config; };
    };
}
