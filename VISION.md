# nix-pklx vision

`nix-pklx` (pklx) is a pure-Rust bridge between Apple's Pkl configuration
language and Nix. It evaluates `.pkl` files directly through `pklr` and emits
native Nix expressions, so a fleet can keep Pkl as the authored source of truth
without hand-maintained JSON or Nix copies of the same data.

## Problem

Pkl is good at typed, inheritable configuration; Nix needs evaluated values at
build time. The common bridge — render Pkl to JSON and import it — loses type
identity, needs a second language toolchain, and turns every consumer into a
bespoke converter.

## Principles

- **No JSON intermediate step.** `pklx eval` produces Nix expression text
  directly, and the Rust API exposes typed deserialization without a JSON
  string round trip.
- **Class identity is preserved.** Pkl class names travel through the output as
  `__pkl_class` metadata instead of being flattened away.
- **Pure Rust.** Evaluation uses `pklr`; no external Pkl binary is required.
  `pklr` capability features are listed explicitly so an upstream default
  change cannot silently remove one.
- **Lossless rendering.** One shared renderer serves deployment manifests:
  Unicode, control characters, and literal backslashes survive, ordinary ASCII
  property names stay unquoted so native Rust Pkl consumers can load them, and
  keywords or irregular names keep their quoted form.
- **Inputs are locked and checked.** The flake pins its inputs, and checks fail
  on retired forge hosts.
- **Both directions are supported.** `importPkl` evaluates Pkl from Nix,
  `toPkl`/`to-pkl` renders Nix values as Pkl, and `toPklFields`/`pklString`
  provide schema-backed manifest helpers.

## Scope

The `pklx` CLI (`eval`, including `--expr` and `--module`, and `to-pkl`), the
library crate (evaluation, serializer, deserializer, import analysis), the Nix
library (`importPkl`, `toPkl`, `fromPkl`, renderer), an mdBook under `docs/`, and
fixture/golden tests.

## Non-goals

- The project describes itself as early development at v0.1.0: converters,
  `output` blocks, and package imports are still being filled in.

## Direction

The unreleased changelog records native-compatible property names, the shared
lossless renderer, `toPklFields`/`pklString`, direct `pklr::Value`
deserialization, and `pklx eval --expr`, with `pklr` 1.2.0 and an MSRV of
Rust 1.88.

## Place in the fleet

`canix/flake.nix` pins `nix-pklx` and several canix inputs follow it, including
the CLI flake. canix's Pkl ownership design documents Pkl as the authored
format, and `home/modules/development/canix_cli.nix` wires the toolchain around
it.
