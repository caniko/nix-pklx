{ pkgs, pklx }:

# Evaluate a .pkl file and return a Nix value by importing the generated .nix file.
file: import (pkgs.runCommandLocal "pklx-import" {
  nativeBuildInputs = [ pklx ];
  passAsFile = [ "src" ];
  src = builtins.readFile file;
} ''
  pklx eval "$src" > "$out"
'')
