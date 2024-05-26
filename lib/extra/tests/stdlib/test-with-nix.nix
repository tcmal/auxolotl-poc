# * Runs all library tests with a particular version of Nix.
{
  pkgs,
  lib,
  libSrc,
  # Only ever use this nix; see comment at top
  nix,
}:
pkgs.runCommand "stdlib-tests-nix-${nix.version}"
  {
    buildInputs = [
      (import ./check-eval.nix lib)
      # (import ./path { inherit pkgs libPath nix; })
    ];
    nativeBuildInputs = [
      nix
      pkgs.gitMinimal
    ] ++ lib.optional pkgs.stdenv.isLinux pkgs.inotify-tools;
    strictDeps = true;
  }
  ''
    datadir="${nix}/share"
    export TEST_ROOT=$(pwd)/test-tmp
    export HOME=$(mktemp -d)
    export NIX_BUILD_HOOK=
    export NIX_CONF_DIR=$TEST_ROOT/etc
    export NIX_LOCALSTATE_DIR=$TEST_ROOT/var
    export NIX_LOG_DIR=$TEST_ROOT/var/log/nix
    export NIX_STATE_DIR=$TEST_ROOT/var/nix
    export NIX_STORE_DIR=$TEST_ROOT/store
    export PAGER=cat
    cacheDir=$TEST_ROOT/binary-cache

    nix-store --init

    cp -r ${libSrc} lib
    echo "Running lib/extra/tests/stdlib/modules.sh"
    bash lib/extra/tests/stdlib/modules.sh

    echo "Checking lib.version"
    nix-instantiate lib -A version --eval || {
      echo "lib.version does not evaluate when lib is isolated from the rest of the nixpkgs tree"
      exit 1
    }

    echo "Running lib/extra/tests/stdlib/filesystem.sh"
    TEST_LIB=$PWD/lib bash lib/extra/tests/stdlib/filesystem.sh

    echo "Running lib/extra/tests/stdlib/sources.sh"
    TEST_LIB=$PWD/lib bash lib/extra/tests/stdlib/sources.sh

    echo "Running lib/tests/stdlib/fileset.sh"
    TEST_LIB=$PWD/lib bash lib/extra/tests/stdlib/fileset.sh

    mkdir $out
    echo success > $out/${nix.version}
  ''
