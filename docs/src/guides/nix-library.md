# Nix Library

nix-pklx provides a per-system Nix library exposed as `nix-pklx.lib.<system>`.

## `importPkl`

Evaluate a `.pkl` file and import it as a Nix value at build time:

```nix
{ pkgs, nix-pklx, ... }:
let
  pklxLib = nix-pklx.lib.${pkgs.stdenv.hostPlatform.system};
  hosts = pklxLib.importPkl ./hosts.pkl;
in {
  services.my-service.hosts = hosts;
}
```

The `.pkl` file is evaluated in a derivation (`pkgs.runCommandLocal`) using the `pklx` binary, producing a Nix expression that is then `import`ed.

## `toPkl`

Convert a Nix value to a Pkl expression string (stub — full support pending):

```nix
{ pkgs, nix-pklx, ... }:
let
  pklxLib = nix-pklx.lib.${pkgs.stdenv.hostPlatform.system};
  pkl = pklxLib.toPkl { foo = "bar"; count = 42; };
in {
  # pkl is a string containing Pkl syntax
}
```

## `fromPkl`

Helpers for consuming Pkl values in Nix:

```nix
{ nix-pklx, ... }:
let
  pklxLib = nix-pklx.lib.x86_64-linux;
in {
  # Parse duration strings: "5days 3hours" → seconds
  # Parse data size strings: "10mb" → bytes
  result = pklxLib.fromPkl.parseDuration "5days 3hours";
}
```
