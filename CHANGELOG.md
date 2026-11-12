# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3] - 2026-11-12

### Fixed

- Release archives: the GitHub release now builds and publishes the signed
  cargo-binstall archives (the previous tag's release workflow failed).
- Cross-target archives are built with `doCheck = false`, so the aarch64 musl
  test binary is never run on the x86_64 build host. This needs the `harbor-rs`
  input at `82206bd` or newer.

## [0.2.2] - 2026-11-11

### Added

- `fromPkl.importSidecar` and `fromPkl.normalize` import Pkl-rendered Nix
  sidecars without `__pkl_class` markers or null-valued attributes.
- Release checksums (`SHA256SUMS.txt`) are now minisign-signed, and release
  archives carry Sigstore bundles and SLSA attestations. Verify a download
  with `minisign -V -m SHA256SUMS.txt -x SHA256SUMS.txt.minisig -p keys/minisign.pub`
  before checking it against `sha256sum -c`.

### Changed

- Build the `pklx` binary unconditionally instead of behind
  `required-features = ["cli"]`, so release archives always contain it. The
  `cli` feature stays as a no-op alias for existing
  `--features cli`/`--no-default-features` consumers.

## [0.2.1] - 2026-10-19

### Fixed

- Require Pklr 5.2.0 so imported class constraints retain their defining
  constants and type aliases even when the caller reuses those names. Keep
  invalid-value rejection and Rust 1.88 compatibility.

## [0.2.0] - 2026-10-17

### Changed

- Upgrade the public `pklr` re-export to 5.1.2, retaining Rust 1.88, native
  evaluation, HTTP imports, package ZIPs, and miette diagnostics. This is an
  API-breaking release: HTTP options now accept a `ureq::Agent`, and Pklr
  strings, object keys, and collections use its shared value types.
- Run synchronous Pklr evaluation on Tokio's blocking pool while retaining
  Pklx's asynchronous evaluation functions.
- Resolve package archives through ZIP 9.0.4 and TLS through Rustls 0.23.46.

### Added

- Bounded loopback regressions for HTTP rewrites and package ZIP imports,
  plus local import, typed value, class metadata, and offline package checks.

## Earlier development

### Changed

- Render ordinary ASCII property names without backticks so deployment
  manifests also load in native Rust Pkl consumers. Keywords and non-regular
  names retain their lossless quoted representation.
- Update `pklr` to 1.2.0 and explicitly enable its parser, evaluator, native IO,
  HTTP, package ZIP, and miette diagnostic features.
- Raise the MSRV to Rust 1.88 to match `pklr` 1.2.0.

### Added

- Pure Nix `toPklFields` and `pklString` helpers for schema-backed manifests.
  The shared renderer preserves Unicode, escapes control characters and literal
  backslashes, emits explicit lists, and rejects unsupported Nix types.
- `PklValueDeserializer` — serde `Deserializer` impl over `pklr::Value` for direct
  Rust-to-Rust deserialization without JSON string intermediates
- `from_pkl_value`, `eval_to_typed`, `eval_source_to_typed` — deserialize Pkl
  values directly into any `T: DeserializeOwned` via the new deserializer
- `pkl_string_literal` — Pkl string escaper without `serde_json` dependency
- `eval_to_value`, `eval_source_to_value` — evaluate Pkl to raw `pklr::Value`
- `pklx eval --expr` — evaluate inline Pkl expression strings
- `pklx eval --http-rewrite` / `--http-proxy` — HTTP rewrite and proxy config
  for Pkl evaluator
- `pub use pklr` — re-export `pklr` crate so consumers can access
  `pklx::pklr::EvalOptions` and `pklx::pklr::reqwest`

### Changed

- Bump `pklr` from stempler git fork (`#dda90eb`) to crates.io `^1.1.2` (upstreams
  the `type_name` fix from pklr#115)
- `serde` promoted from dev-dependency to regular dependency (required for
  `PklValueDeserializer`)
- `eval_pkl`, `eval_pkl_source`, `analyze_pkl_imports` — moved from `cli.rs`
  to `eval.rs` module (available without `cli` feature gate)
