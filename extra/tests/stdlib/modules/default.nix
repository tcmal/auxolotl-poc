{
  lib ? import ../../../../nix/stdlib,
  modules ? [ ],
}:
{
  inherit
    (lib.evalModules {
      inherit modules;
      specialArgs.modulesPath = ./.;
    })
    config
    options
    ;
}
