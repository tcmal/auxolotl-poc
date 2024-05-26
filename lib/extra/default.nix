{
  nixpkgs ? (import ./npins).nixpkgs,
}:
let
  libSrc = ../.;
  lib = import libSrc;
  pkgs = import nixpkgs {
    overlays = [
      # update nixfmt, as nixpkgs is pretty out of date
      (
        final: prev:
        prev
        // {
          nixfmt = prev.nixfmt.overrideAttrs {
            src = final.fetchFromGitHub {
              owner = "nixos";
              repo = "nixfmt";
              rev = "3bcb63c13e7aaf0b8e14081cf0c14c44f62e840a";
              sha256 = "sha256-8QWhKKqpHSlHuvFW8wdWTbG0pJ6XtuxZQ3HhR4uPbRA=";
            };
          };
        }
      )
    ];
  };
in
{
  devShell = pkgs.mkShellNoCC { packages = [ pkgs.nixfmt ]; };

  packages = {
    docs = pkgs.callPackage ./doc { inherit libSrc; };
  };

  checks =
    let
      auxlib = import ./tests/auxlib { inherit pkgs libSrc; };
      stdlib = import ./tests/stdlib { inherit pkgs libSrc; };
      tests = pkgs.symlinkJoin {
        name = "auxlib-tests";
        paths = [
          auxlib
          stdlib
        ];
      };
      formatting = pkgs.callPackage ./tests/formatting.nix { inherit libSrc; };
    in
    {
      all = pkgs.symlinkJoin {
        name = "auxlib-checks";
        paths = [
          tests
          formatting
        ];
      };

      inherit
        auxlib
        stdlib
        tests
        formatting
        ;
    };
}
