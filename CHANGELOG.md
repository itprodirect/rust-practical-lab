# Changelog

All notable changes to this repository are documented in this file.

The format is inspired by Keep a Changelog and follows semantic version tags for repository releases.

## [0.2.0] - 2026-03-05

### Added

- New interop crates:
  - `crates/ffi_demo` (C ABI exports + Python/Node examples)
  - `crates/wasm_demo` (wasm-bindgen exports + browser example)
- New reusable pattern crate:
  - `crates/power_blocks` with `typed_id`, `typestate_builder`, `retry`, `parallel_map`, and `zero_copy` modules
- Focused Phase 6 interop validation scripts:
  - `scripts/check_phase6.sh`
  - `scripts/check_phase6.ps1`
- Dedicated Phase 6 CI job in `.github/workflows/ci.yml`
- Root `LICENSE` and `docs/README.md`
- Session wrap-up notes in `docs/08-session-wrap-up-2026-02-17.md`

### Changed

- Hardened log parsing behavior in `crates/log_parser/src/lib.rs`:
  - robust whitespace token handling
  - clearer invalid-line behavior across allocating and streaming paths
- Expanded tests for parser edge cases, retry policy normalization, typed-id parsing bounds, and FFI/WASM Fibonacci saturation behavior
- Full documentation and onboarding overhaul across `README.md`, all phase docs, and `SECURITY.md`
- Added repository metadata to crate manifests for cleaner packaging/tooling output

### Fixed

- Correctly fail Phase 6 PowerShell checks on native command errors
- Reduced interop friction with a documented Node FFI default path (`koffi`) and advanced parity path (`ffi-napi` on Node 20)

## [0.1.0] - 2026-02-16

### Added

- Initial workspace bootstrap and CI scaffolding (phase 0)
- CLI/library split with tests (`hello_cli`, `hello_lib`) (phase 1)
- Typed error handling demo with `thiserror` (`error_demo`) (phase 2)
- Concurrency demo with channels and shared-state patterns (`concurrency_demo`) (phase 3)
- Performance parsing lab + criterion benchmark (`log_parser`, `parser_bench`) (phase 4)
- Security tooling integration (`cargo audit`, `cargo deny`, `SECURITY.md`) (phase 5)
