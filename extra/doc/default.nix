{
  auxlib,
  lib,
  runCommand,
  nixdoc,
  ...
}:
let
  inherit (lib) escapeShellArg concatMapStringsSep;
  sections = import ./sections.nix auxlib;
in
runCommand "auxolotl-stdlib-docs" { buildInputs = [ nixdoc ]; } ''
  mkdir $out
  function docgen {
    name=$1
    path=$2
    description=$3
    nixdoc -c "$name" -d "lib.$name: $description" -f "$path" > "$out/$name.md"
    echo "$name.md" >> "$out/index.md"
  }

  mkdir -p "$out"

  cat > "$out/index.md" << 'EOF'
  ```{=include=} sections auto-id-prefix=auto-generated
  EOF
  ${concatMapStringsSep "\n" (
    section:
    "docgen ${escapeShellArg section.name} ${section.path} ${escapeShellArg section.description};"
  ) sections}

  echo '```' >> "$out/index.md"
''
