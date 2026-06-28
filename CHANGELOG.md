# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Fix clippy `redundant_closure` lint in `eval_pkl`
- Fix Pages CI: copy nix build output to a regular directory instead of relying on the `result` symlink

### Added

- `pklx eval` — evaluate a `.pkl` file and emit a Nix expression
- `pklx eval --module` — wrap output in a NixOS module skeleton
- `pklx to-pkl` — convert a Nix-like expression to Pkl syntax
- `pklx analyze` — list transitive local file imports of a `.pkl` file
- `pkl_value_to_nix` — serialize a `pklr::Value` to a Nix expression string
- Nix library: `importPkl`, `toPkl`, `fromPkl`
- Nix flake with crane build, checks, and rs-harbor dev shells
- Preserves Pkl class identity via `__pkl_class` attribute (relies on pklr#115)
