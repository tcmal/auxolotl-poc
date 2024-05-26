let
  inherit (import ./stdlib/fixed-points.nix { inherit lib; }) makeExtensible;
  lib = makeExtensible (self: (import ./stdlib) // (import ./auxlib self));
in
lib
