{ lib, options, ... }:
let
  discardPositions = lib.mapAttrs (k: v: v);
in
# unsafeGetAttrPos is unspecified best-effort behavior, so we only want to consider this test on an evaluator that satisfies some basic assumptions about this function.
assert builtins.unsafeGetAttrPos "a" { a = true; } != null;
assert
  builtins.unsafeGetAttrPos "a" (discardPositions {
    a = true;
  }) == null;
{
  imports = [
    {
      options.imported.line14 = lib.mkOption { type = lib.types.int; };

      # Simulates various patterns of generating modules such as
      # programs.firefox.nativeMessagingHosts.ff2mpv. We don't expect to get
      # line numbers for these, but we can fall back on knowing the file.
      options.generated = discardPositions { line19 = lib.mkOption { type = lib.types.int; }; };

      options.submoduleLine21.extraOptLine21 = lib.mkOption {
        default = 1;
        type = lib.types.int;
      };
    }
  ];

  options.nested.nestedLine28 = lib.mkOption { type = lib.types.int; };

  options.submoduleLine30 = lib.mkOption {
    default = { };
    type = lib.types.submoduleWith {
      modules = [
        (
          { options, ... }:
          {
            options.submodDeclLine37 = lib.mkOption { };
          }
        )
        { freeformType = with lib.types; lazyAttrsOf (uniq unspecified); }
      ];
    };
  };

  config = {
    submoduleLine30.submodDeclLine37 =
      (options.submoduleLine30.type.getSubOptions [ ]).submodDeclLine37.declarationPositions;
  };
}
