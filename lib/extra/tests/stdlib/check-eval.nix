# Throws an error if any of our lib tests fail.
lib:
let
  tests = [
    "misc"
    "systems"
  ];
  all = builtins.concatLists (map (f: import (./. + "/${f}.nix") lib) tests);
in
if all == [ ] then null else throw (builtins.toJSON all)
