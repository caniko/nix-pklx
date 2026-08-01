# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-08-01

### Changed

- Update `pklr` to 1.2.0 and explicitly enable its parser, evaluator, native IO,
  HTTP, package ZIP, and miette diagnostic features.
- Raise the MSRV to Rust 1.88 to match `pklr` 1.2.0.

### Added

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
