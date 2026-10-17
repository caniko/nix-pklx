# nix-pklx

<!-- simit:badges:start -->

[![CI](https://img.shields.io/badge/CI-drift-2088ff)](.forgejo/workflows/ci.yaml) [![Nix](https://img.shields.io/badge/Nix-managed-5277c3)](flake.nix) [![docs](https://img.shields.io/badge/docs-enabled-6f42c1)](docs) [![crates.io](https://img.shields.io/badge/crates.io-ready-f46623)](https://crates.io/crates/pklx)

<!-- simit:badges:end -->

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
  inputs.nix-pklx.url = "git+https://github.com/caniko/nix-pklx.git";
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

## Migrating to 0.2

Pklx 0.2 re-exports Pklr 5.1 and keeps Rust 1.88 as its minimum version.
The async evaluation functions are unchanged; synchronous Pklr work runs on
Tokio's blocking pool. Local imports, HTTP rewrites, package ZIPs, class
metadata, data-only output, and typed deserialization remain supported.

For custom HTTP configuration, replace `EvalOptions.client` and
`pklx::pklr::reqwest` with `EvalOptions.agent` and `pklx::pklr::ureq`:

```rust
let options = pklx::pklr::EvalOptions {
    agent: Some(pklx::pklr::ureq::Agent::new_with_defaults()),
    ..Default::default()
};
```

Code constructing raw Pklr values must also migrate to Pklr's shared types:
use `.into()` for strings and lists, and `pklx::pklr::value::ObjectMap` for
object members. Use `ObjectSource::type_name()` to read class metadata.

## License

MIT OR Apache-2.0
