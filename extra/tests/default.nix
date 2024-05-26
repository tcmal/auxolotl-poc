{
  pkgs ? import <nixpkgs> { } // {
    lib = throw "pkgs.lib accessed, but the lib tests should use nixpkgs' lib path directly!";
  },
  libSrc ? ../..,
}:
pkgs.symlinkJoin {
  name = "nixpkgs-lib-tests";
  paths = [
    (import ./stdlib { inherit pkgs libSrc; })
    (import ./auxlib { inherit pkgs libSrc; })
  ];
}
