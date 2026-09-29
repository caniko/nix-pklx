{lib}: let
  renderer = import ../lib/pkl-renderer.nix {inherit lib;};
  control = builtins.fromJSON ''"\u0001\b\f"'';
  expected = {
    "hyphen-key" = "quotes: \"; slash: \\; interpolation: \\(name)";
    unicode = "Mitrović";
    controls = "\n\r\t${control}";
    literalEscapes = "\\b\\f\\u0001";
    emptyList = [];
    emptyObject = {};
    nested = [null true false 42 1.25 {"keyword" = "value";}];
  };
  rejected = value: !(builtins.tryEval (builtins.deepSeq (renderer.value value) true)).success;
in
  assert rejected (x: x);
  assert rejected {"bad`key" = 1;}; {
    inherit expected;
    document = renderer.fields expected;
  }
