{ core, python, ... }:
let
  lvl0 = { } // {
    stdenv = core.stdenv { inherit (lvl2); };
    gcc = core.gcc { inherit (lvl2); };
    fetchurl = core.fetchurl { inherit (lvl2); };
  };

  lvl1 = lvl0 // {
    hello = core.hello { inherit (lvl2) fetchurl stdenv; };
  };

  lvl2 = lvl1 // {
    goodbye = python.goodbye { inherit (lvl2) hello fetchurl stdenv; };
  };
in
lvl2
