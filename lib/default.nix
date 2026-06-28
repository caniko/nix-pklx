{ lib }:
let
  inherit (lib) stringToCharacters;
in
{
  # Evaluate a .pkl file and import it as a Nix value.
  # Usage: { pkgs, ... }: { imports = [ (lib.pklx.importPkl ./config.pkl) ]; };
  importPkl = file: import (builtins.toFile "pklx-import" ''
    let
      pklx = builtins.fetchurl {
        url = "https://example.org/pklx";  # placeholder — must be overridden
        sha256 = "0000000000000000000000000000000000000000000000000000";
      };
    in
    builtins.fromJSON (builtins.readFile (builtins.toString file))
  '');

  # Convert a Nix value to a Pkl expression string.
  # FIXME: This is a stub — full recursive conversion pending.
  # toPkl = value:
  #   if builtins.isNull value then "null"
  #   else if builtins.isBool value then if value then "true" else "false"
  #   else if builtins.isInt value then toString value
  #   else if builtins.isFloat value then toString value
  #   else if builtins.isString value then ''"${value}"''
  #   else if builtins.isList value then "new { ${builtins.concatStringsSep " " (map toPkl value)} }"
  #   else if builtins.isAttrs value then "new { ${...} }"
  #   else "null";
}
