{
  stdenv,
  fetchurl,
  # Pretend this depends on `hello` (from `core/`) for whatever reason
  hello,
  ...
}:
stdenv.mkDerivation rec {
  pname = "goodbye";
  version = "2.12.1";

  src = fetchurl {
    url = "mirror://gnu/hello/hello-${version}.tar.gz";
    sha256 = "sha256-jZkUKv2SV28wsM18tCqNxoCZmLxdYH2Idh9RLibH2yA=";
  };
}
