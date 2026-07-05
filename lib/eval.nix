{
  lib,
  pkgs,
  pklx,
}:

# Evaluate a .pkl file and return a Nix value by importing the generated .nix
# file. Copy the containing directory, not just the file contents, so local
# import("sibling.pkl") and import("dir/file.pkl") references remain valid.
file: let
  srcDir = builtins.dirOf file;
  fileName = builtins.baseNameOf file;
  source = builtins.path {
    path = srcDir;
    name = "pklx-import-source";
  };
in
  import (pkgs.runCommandLocal "pklx-import-${lib.removeSuffix ".pkl" fileName}" {
    nativeBuildInputs = [pklx];
  } ''
    pklx eval "${source}/${fileName}" > "$out"
  '')
