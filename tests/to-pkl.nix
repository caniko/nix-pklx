{lib}: let
  renderer = import ../lib/pkl-renderer.nix {inherit lib;};
  control = builtins.fromJSON ''"\u0001\b\f"'';
  expected = {
    "hyphen-key" = "quotes: \"; slash: \\; interpolation: \\(name)";
    "class" = "keyword";
    "when" = "reserved keyword";
    "_" = "blank identifier";
    "123key" = "leading digit";
    "with space" = "quoted identifier";
    unicode = "Mitrović";
    controls = "\n\r\t${control}";
    literalEscapes = "\\b\\f\\u0001";
    emptyList = [];
    emptyObject = {};
    nested = [null true false 42 1.25 {"keyword" = "value";}];
  };
  # Deployment-shaped data must also load through the native Rust evaluator.
  nativeExpected = {
    projectsRoot = "/data/can/canix/projects";
    projectStateRoot = "/data/can/ProjectState";
    preflight.alwaysCheck = ["atlas" "thething"];
    nested._field2 = {
      enable = true;
      optional = null;
    };
  };
  rejected = value: !(builtins.tryEval (builtins.deepSeq (renderer.value value) true)).success;
in
  assert rejected (x: x);
  assert rejected {"bad`key" = 1;}; {
    inherit expected;
    document = renderer.fields expected;
    inherit nativeExpected;
    nativeDocument = renderer.fields nativeExpected;
  }
