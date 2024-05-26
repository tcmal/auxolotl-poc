{ core, python, ... }:
let
  lvl0 = { } // {
    phase0.bootstrap-tools = core.phase0.bootstrap-tools { };
  };

  lvl1 = lvl0 // {
    phase0.stdenv = core.phase0.stdenv { phase0.bootstrap-tools = lvl6.phase0.bootstrap-tools; };
  };

  lvl2 = lvl1 // {
    phase1.stdenv = core.phase1.stdenv { phase0.stdenv = lvl6.phase0.stdenv; };
  };

  lvl3 = lvl2 // {
    stdenv = core.stdenv { phase1.stdenv = lvl6.phase1.stdenv; };
  };

  lvl4 = lvl3 // {
    hello = core.hello { stdenv = lvl6.stdenv; };
    python3 = python.python3 { stdenv = lvl6.stdenv; };
  };

  lvl5 = lvl4 // {
    nix = core.nix {
      python3 = lvl6.python3;
      stdenv = lvl6.stdenv;
    };
  };

  lvl6 = lvl5 // {
    python3Packages.python-nix = python.python3Packages.python-nix {
      nix = lvl6.nix;
      python3 = lvl6.python3;
    };
  };
in
lvl6
