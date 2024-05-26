{
  inputs = {
    auxlib.url = "path:../lib";
  };

  outputs =
    { self, auxlib, ... }:
    let
      inherit (auxlib) lib;
      config = {
        system = "x86_64-linux";
        # etc
      };
    in
    {
      lambdas = import ./default.nix { inherit lib config; };
    };
}
