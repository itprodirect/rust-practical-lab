# Repo Brief

## Name

`rust-practical-lab`

## What It Is

A Rust workspace for learning by running real code: small labs, focused docs, and reusable modules that can be copied into other projects.

## Who It Serves

- Developers building practical Rust fluency
- Maintainers who want a compact workspace of proven examples
- Coding agents resuming work without re-discovering repo intent

## Current Practical Value

- Runnable examples for CLI structure, error handling, concurrency, parsing, FFI, and WASM
- A reusable `power_blocks` crate with copyable patterns
- Lightweight validation and CI conventions that match the repo's teaching goals

## Shape Of The Repo

- `crates/hello_cli`, `crates/hello_lib`: first runnable lab and testable core logic
- `crates/error_demo`, `crates/concurrency_demo`, `crates/log_parser`: focused learning labs
- `crates/ffi_demo`, `crates/wasm_demo`: practical interop paths
- `crates/power_blocks`: reusable patterns intended for transplant into real projects
- `docs/`: phase docs and operational notes
- `scripts/`: focused validation helpers, especially for Phase 6

## What Not To Touch In A Memory Pass

- Crate names, phase numbering, and workspace layout
- Existing teaching flow across `docs/00-...` through `docs/09-...`
- Dependency surface or CI shape unless required for a repo identity fix
- Generated/build output under `target/`

## Out Of Scope For This Pass

- Broad code rewrites
- New frameworks or tooling layers
- Reframing the repo as anything other than a practical Rust lab workspace
