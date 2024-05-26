# auxolotl/lib

<a href="https://forum.aux.computer/c/special-interest-groups/sig-core/14"><img src="https://img.shields.io/static/v1?label=Maintained%20By&message=SIG%20Core&style=for-the-badge&labelColor=222222&color=794AFF" /></a>

This repo contains system-agnostic library functions used by auxolotl repositories.

There are two main parts:

  - `stdlib` - mostly copied from Nixpkg's standard library. Meant to be forwards-compatible.
  - `auxlib` - library bits specifically for aux / auxpkgs. Not necessarily forwards-compatible.

## `extra/`

Due to limitations with flakes, both tests and documentation are stored in `extra/`.
This avoids the `lib` flake having to depend on other places for testing and documentation tools, which are basically dev dependencies.

This directory uses `npins`, as using flakes requires constantly running `nix flake update` to pick up local changes.

## Testing

Tests are stored in `extra/tests/<part>`. You can run them all with `nix-build -E '(import ./extra {}).checks.tests` (or `.stdlib`, `.auxlib`).

You should also check your formatting with `nix-build -E '(import ./extra {}).checks.formatting`.

## Documentation

Reference documentation for library functions is written above each function as a multi-line comment.
These comments are processed using [nixdoc](https://github.com/nix-community/nixdoc), although currently we aren't doing much with the output.
The nixdoc README describes the [comment format](https://github.com/nix-community/nixdoc#comment-format).

You can build the documentation with `nix-build -E '(import ./extra {}).packages.docs'`.
