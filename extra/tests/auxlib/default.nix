{
  # The pkgs used for dependencies for the testing itself
  # Don't test properties of pkgs.lib, but rather the lib in the parent directory
  pkgs ? import <nixpkgs> { } // {
    lib = throw "pkgs.lib accessed, but the lib tests should use nixpkgs' lib path directly!";
  },
  libSrc,
  nix ? pkgs-nixVersions.stable,
  nixVersions ? [
    pkgs-nixVersions.minimum
    nix
    pkgs-nixVersions.unstable
  ],
  pkgs-nixVersions ? import ../nix-for-tests.nix { inherit pkgs; },
}:

let
  testWithNix = nix: import ./test-with-nix.nix { inherit libSrc nix pkgs; };
in
pkgs.symlinkJoin {
  name = "auxlib-tests";
  paths = map testWithNix nixVersions;
}
