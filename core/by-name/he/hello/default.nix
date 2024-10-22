{ stdenv, ... }:
stdenv.mkDerivation rec {
  pname = "hello";
  version = "2.12.1";

  src = builtins.fetchurl {
    url = "mirror://gnu/hello/hello-${version}.tar.gz";
    sha256 = "sha256-jZkUKv2SV28wsM18tCqNxoCZmLxdYH2Idh9RLibH2yA=";
  };
}
