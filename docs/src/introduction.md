# nix-pklx

**nix-pklx** (pronounced "pkl-ex") is a pure-Rust bridge between Apple's
[Pkl configuration language](https://pkl-lang.org/) and
[Nix](https://nixos.org/) expressions. It evaluates `.pkl` files directly and
emits native Nix expressions — no JSON intermediate step.

## Features

- **Pkl → Nix**: evaluate `.pkl` files and produce Nix expression strings
- **Class identity preserved**: Pkl class names are carried through as
  `__pkl_class` metadata in the Nix output
- **NixOS modules**: emit NixOS module skeletons from Pkl configs
- **Nix library**: `importPkl` to consume Pkl files directly from Nix,
  `toPkl` and `fromPkl` helpers for the reverse direction
- **Pure Rust**: built on [`pklr`](https://github.com/jdx/pklr), a pure-Rust
  Pkl evaluator — no external Pkl binary required

## Project Status

nix-pklx is in early development (v0.1.0). The core evaluation and
serialization paths are functional; the Nix library and advanced Pkl features
(converters, `output` blocks, package imports) are being filled in.
