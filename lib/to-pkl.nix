{ lib }:

# Convert a Nix value to a Pkl expression string.
# This is a pure Nix function — no external dependencies.
#
# Types:
#   null    → null
#   bool    → true / false
#   int     → 42
#   float   → 3.14
#   string  → "foo"
#   list    → new Listing { elem1; elem2; }
#   attrs   → new { key = value; }
let
  inherit (builtins) isNull isBool isInt isFloat isString isList isAttrs
    toString typeOf;

  go = value:
    if isNull value then "null"
    else if isBool value then if value then "true" else "false"
    else if isInt value then toString value
    else if isFloat value then
      let s = toString value;
      in if builtins.match ".*[.eE].*" s != null then s else "${s}.0"
    else if isString value then ''"${value}"''
    else if isList value then "new {\n${builtins.concatStringsSep "\n" (map go value)}\n}"
    else if isAttrs value then
      let
        entries = lib.mapAttrsToList (k: v: "  ${k} = ${go v}") value;
      in "new {\n${builtins.concatStringsSep "\n" entries}\n}"
    else "null";
in
go
