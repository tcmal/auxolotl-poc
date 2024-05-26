# * Runs all library tests with a particular version of Nix.
{
  pkgs,
  libSrc,
  # Only ever use this nix; see comment at top
  nix,
}:
pkgs.runCommand "auxlib-tests-nix-${nix.version}"
  {
    buildInputs = [
      # TODO: Tests!
    ];
    strictDeps = true;
  }
  ''
    mkdir $out
    echo success > $out/${nix.version}
  ''
