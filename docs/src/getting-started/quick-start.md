# Quick Start

Create a Pkl configuration file:

```pkl
// config.pkl
host = "example.com"
port = 443
tls = true
```

Evaluate it to Nix:

```bash
pklx eval config.pkl
```

Output:

```nix
{ host = "example.com"; port = 443; tls = true; }
```

## As a NixOS module

```bash
pklx eval --module config.pkl
```

Output:

```nix
{ config, pkgs, ... }: {
{ host = "example.com"; port = 443; tls = true; }
}
```

## From Nix

```nix
# configuration.nix
{ pkgs, nix-pklx, ... }:
let
  inherit (nix-pklx.lib.${pkgs.system}) importPkl;
in {
  imports = [ (importPkl ./config.pkl) ];
}
```
