{
  inputs = {
    auxlib.url = "path:../lib";

    core.url = "path:../core";
    python.url = "path:../python";
  };

  outputs =
    inp@{
      self,
      auxlib,
      core,
      python,
      ...
    }:
    let
      inherit (auxlib) lib;
      config = {
        system = "x86_64-linux";
      };
    in
    {
      lambdas = {
        core = core.lambdas;
        python = python.lambdas;
      };

      packages.${config.system} = import ./pkgs.nix inp;
    };
}
