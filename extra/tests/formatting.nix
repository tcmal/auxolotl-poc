{
  libSrc,
  runCommand,
  nixfmt,
}:
runCommand "aux-lib-formatting" { buildInputs = [ nixfmt ]; } ''
  find ${libSrc} -iname '*.nix' -type f -print0 | xargs -0 -i nixfmt -c {}
  mkdir $out
  touch $out/formatted
''
