{
  callPackage,
  rustToolchain,
}:
{
  default = callPackage ./language { inherit rustToolchain; };
}
