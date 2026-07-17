# nix-pklx

Pkl ↔ Nix interop: evaluate Pkl files to native Nix expressions + Nix library.

**pklx** is a pure-Rust bridge between Apple's [Pkl configuration language](https://pkl-lang.org/)
and [Nix](https://nixos.org/). It evaluates `.pkl` files directly via
[`pklr`](https://github.com/jdx/pklr) (a pure-Rust Pkl evaluator) and
emits native Nix expressions — no JSON intermediate step.

## Features

- **Pkl → Nix**: evaluate `.pkl` files and produce Nix expression strings
- **Class identity preserved**: Pkl class names are carried through as
  `__pkl_class` metadata in the Nix output
- **NixOS modules**: emit NixOS module skeletons from Pkl configs (`--module`)
- **Nix library**: `importPkl` to consume Pkl files from Nix at build time,
  `toPkl` and `fromPkl` helpers for the reverse direction
- **Pure Rust**: no external Pkl binary required

## CLI Usage

```bash
# Evaluate a .pkl file to Nix
pklx eval config.pkl

# Wrap output in a NixOS module skeleton
pklx eval --module config.pkl

# Write to a file
pklx eval config.pkl -o config.nix

# Convert a Nix expression to Pkl syntax
pklx to-pkl '{"host": "example.com", "port": 443}'
```

## Nix Library Usage

```nix
{
  inputs.nix-pklx.url = "git+https://codeberg.org/caniko/nix-pklx.git";
  # ...
  outputs = { nix-pklx, ... }: {
    nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
      modules = [
        ({ pkgs, ... }: {
          imports = [
            (nix-pklx.lib.${pkgs.stdenv.hostPlatform.system}.importPkl ./config.pkl)
          ];
        })
      ];
    };
  };
}
```

## License

MIT OR Apache-2.0
