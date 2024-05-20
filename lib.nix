# This file contains additions to the existing library for the POC
# These would probably be added to `auxolotl/lib` under the `auxlib` directory
lib:
lib
// {
  /**
    Build an attribute set from the directory tree `baseDirectory`.

    The top-level package pkgs.some-package may be declared by setting up this file structure:

    base-directory
    ├── so
    ┊  ├── some-package
       ┊  └── default.nix

    Where some-package is the package name and so is the lowercased 2-letter prefix of the package name.
    This will result in the attribute set `{some-package = import ./so/some-package;}`

    Alternatively, if `so/some-package/packages.nix` exists, it will be called with `args `and its return value will be merged with the existing package list.
  */
  useByName =
    baseDirectory: args:
    let
      inherit (builtins) readDir;
      inherit (lib.attrsets) mapAttrs mapAttrsToList mergeAttrsList;
      namesForShard =
        shard: type:
        if type != "directory" then
          { }
        else
          mapAttrs (name: _: baseDirectory + "/${shard}/${name}/default.nix") (
            lib.filterAttrs (
              name: _: !(lib.hasAttr "packages.nix" (readDir (baseDirectory + "/${shard}/${name}")))
            ) (readDir (baseDirectory + "/${shard}"))
          );

      namesForShardGroups =
        shard: type:
        if type != "directory" then
          { }
        else
          mapAttrs (name: _: baseDirectory + "/${shard}/${name}") (
            lib.filterAttrs (
              name: _: lib.hasAttr "packages.nix" (readDir (baseDirectory + "/${shard}/${name}"))
            ) (readDir (baseDirectory + "/${shard}"))
          );

      packageFiles = mergeAttrsList (mapAttrsToList namesForShard (readDir baseDirectory));
      packageGroupFiles = lib.attrValues (
        mergeAttrsList (mapAttrsToList namesForShardGroups (readDir baseDirectory))
      );

      res =
        (mapAttrs (name: file: import file) packageFiles)
        // (lib.foldl (acc: path: (import "${path}/packages.nix" args) // acc) { } packageGroupFiles);
    in
    res;
}
