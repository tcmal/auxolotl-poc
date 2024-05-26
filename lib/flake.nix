{
  description = "Auxolotl system-agnostic libraries";
  inputs = { };
  outputs =
    { self }:
    {
      lib = import ./nix;
    };
}
