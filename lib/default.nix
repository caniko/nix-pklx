{
  lib,
  pkgs,
  pklx,
}: {
  # Evaluate a .pkl file and import it as a Nix value.
  # Usage: importPkl ./config.pkl
  importPkl = import ./eval.nix {inherit lib pkgs pklx;};

  # Convert a Nix value to a Pkl expression string.
  toPkl = import ./to-pkl.nix {inherit lib;};
  toPklFields = (import ./pkl-renderer.nix {inherit lib;}).fields;
  pklString = (import ./pkl-renderer.nix {inherit lib;}).string;

  # Helpers for consuming Pkl values in Nix.
  fromPkl = import ./from-pkl.nix {inherit lib;};
}
