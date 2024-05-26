# Nixpkgs lib

This directory contains the implementation of our standard utility library, migrated from Nixpkgs' `lib`.

## Overview

The evaluation entry point for `lib` is [`default.nix`](default.nix).
This file evaluates to an attribute set containing two separate kinds of attributes:
- Sub-libraries:
  Attribute sets grouping together similar functionality.
  Each sub-library is defined in a separate file usually matching its attribute name.

  Example: `lib.lists` is a sub-library containing list-related functionality such as `lib.lists.take` and `lib.lists.imap0`.
  These are defined in the file [`lists.nix`](lists.nix).

- Aliases:
  Attributes that point to an attribute of the same name in some sub-library.

  Example: `lib.take` is an alias for `lib.lists.take`.

Most files in this directory are definitions of sub-libraries, but there are a few others:
- [`minver.nix`](minver.nix): A string of the minimum version of Nix that is required to evaluate Nixpkgs.
- [`systems`](systems): The `lib.systems` sub-library, structured into a directory instead of a file due to its complexity
- [`path`](path): The `lib.path` sub-library, which also includes a document describing the design goals of `lib.path`
- All other files in this directory are sub-libraries

### Module system

The [module system](https://nixos.org/manual/nixpkgs/#module-system) spans multiple sub-libraries:
- [`modules.nix`](modules.nix): `lib.modules` for the core functions and anything not relating to option definitions
- [`options.nix`](options.nix): `lib.options` for anything relating to option definitions
- [`types.nix`](types.nix): `lib.types` for module system types

## Reference documentation

Reference documentation for library functions is written above each function as a multi-line comment.
These comments are processed using [nixdoc](https://github.com/nix-community/nixdoc).
The nixdoc README describes the [comment format](https://github.com/nix-community/nixdoc#comment-format).

See [doc/README.md](../../doc/README.md) for how to build the manual.
