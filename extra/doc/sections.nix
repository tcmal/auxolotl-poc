auxlib:
let
  libPath = "${auxlib}/nix";
in
[
  {
    name = "asserts";
    description = "assertion functions";
    path = "${libPath}/stdlib/asserts.nix";
  }
  {
    name = "attrsets";
    description = "attribute set functions";
    path = "${libPath}/stdlib/attrsets.nix";
  }
  {
    name = "strings";
    description = "string manipulation functions";
    path = "${libPath}/stdlib/strings.nix";
  }
  {
    name = "versions";
    description = "version string functions";
    path = "${libPath}/stdlib/versions.nix";
  }
  {
    name = "trivial";
    description = "miscellaneous functions";
    path = "${libPath}/stdlib/trivial.nix";
  }
  {
    name = "fixed-points";
    description = "explicit recursion functions";
    path = "${libPath}/stdlib/fixed-points.nix";
  }
  {
    name = "lists";
    description = "list manipulation functions";
    path = "${libPath}/stdlib/lists.nix";
  }
  {
    name = "debug";
    description = "debugging functions";
    path = "${libPath}/stdlib/debug.nix";
  }
  {
    name = "options";
    description = "NixOS / nixpkgs option handling";
    path = "${libPath}/stdlib/options.nix";
  }
  {
    name = "path";
    description = "path functions";
    path = "${libPath}/stdlib/path/default.nix";
  }
  {
    name = "filesystem";
    description = "filesystem functions";
    path = "${libPath}/stdlib/filesystem.nix";
  }
  {
    name = "fileset";
    description = "file set functions";
    path = "${libPath}/stdlib/fileset/default.nix";
  }
  {
    name = "sources";
    description = "source filtering functions";
    path = "${libPath}/stdlib/sources.nix";
  }
  {
    name = "cli";
    description = "command-line serialization functions";
    path = "${libPath}/stdlib/cli.nix";
  }
  {
    name = "gvariant";
    description = "GVariant formatted string serialization functions";
    path = "${libPath}/stdlib/gvariant.nix";
  }
  {
    name = "customisation";
    description = "Functions to customise (derivation-related) functions, derivatons, or attribute sets";
    path = "${libPath}/stdlib/customisation.nix";
  }
  {
    name = "meta";
    description = "functions for derivation metadata";
    path = "${libPath}/stdlib/meta.nix";
  }
  {
    name = "derivations";
    description = "miscellaneous derivation-specific functions";
    path = "${libPath}/stdlib/derivations.nix";
  }
]
