{
  pkgs ? import <nixpkgs> { } // {
    lib = throw "pkgs.lib accessed, but the lib tests should use nixpkgs' lib path directly!";
  },
  nix ? pkgs.nixVersions.stable,
  libPath ? ../../../../nix/stdlib,
  # Random seed
  seed ? null,
}:
pkgs.runCommand "lib-path-tests"
  {
    nativeBuildInputs =
      [ nix ]
      ++ (with pkgs; [
        jq
        bc
      ]);
  }
  ''
    # Needed to make Nix evaluation work
    export TEST_ROOT=$(pwd)/test-tmp
    export NIX_BUILD_HOOK=
    export NIX_CONF_DIR=$TEST_ROOT/etc
    export NIX_LOCALSTATE_DIR=$TEST_ROOT/var
    export NIX_LOG_DIR=$TEST_ROOT/var/log/nix
    export NIX_STATE_DIR=$TEST_ROOT/var/nix
    export NIX_STORE_DIR=$TEST_ROOT/store
    export PAGER=cat

    echo ${libPath}
    cp -r ${libPath} lib
    export TEST_LIB=$PWD/lib

    echo "Running unit tests lib/tests/stdlib/path/unit.nix"
    nix-instantiate --eval --show-trace \
      --argstr libpath "$TEST_LIB" \
      lib/tests/stdlib/path/unit.nix

    echo "Running property tests lib/tests/stdlib/path/prop.sh"
    bash lib/tests/stdlib/path/prop.sh ${toString seed}

    touch $out
  ''
