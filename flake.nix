{
  inputs = {
    auxlib.url = "github:auxolotl/lib";
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
      lib = import ./lib.nix auxlib.lib;
      config = {
        system = "x86_64-linux";
        # etc

        # Used in some places avoid writing a bunch of code that would distract from the actual POC
        removeMe = nixpkgs;
      };
      core = import ./core { inherit lib config; };
      extra = import ./extra { inherit lib config; };
    in
    {
      core = {
        # An attribute set of <package name>: <lambda>
        # This is analysed to build our registry, which calls the lambdas and makes them actual derivations.
        lambdas = core;
      };

      # In practice, these would be across different repositories / flakes: We ignore this for now.
      extra = {
        lambdas = extra;
      };

      # The actual package set
      packages.${config.system} = import ./gen.nix { inherit core extra; };
    };
}
