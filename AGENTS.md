# AGENTS.md

## Purpose

This repository is `rust-practical-lab`: a practical Rust workspace of runnable micro-labs and copyable building blocks. Keep changes tight, reality-based, and easy to resume.

## Read Order

1. `README.md`
2. `AGENTS.md`
3. `docs/repo-brief.md`
4. `docs/heartbeat.md`
5. `logs/2026-03-27-session.md` or the newest file in `logs/`
6. `docs/decisions.md` when the task involves identity, scope, or tradeoffs

## Working Rules

- Preserve the phase-based workspace structure and existing crate behavior unless the task explicitly requires code changes.
- Prefer small, practical updates over broad refactors or framework churn.
- Keep docs aligned with actual repo contents and validation commands.
- Treat `scripts/check_phase6.sh` and `scripts/check_phase6.ps1` as focused interop checks, not generic smoke tests.
- Do not touch the untracked `.claude/` directory unless explicitly asked.

## Fast Validation

- `cargo fmt --all -- --check`
- `cargo test --workspace`
- `cargo run -p hello_cli -- --name "world"`

Use the smallest grounded subset that matches the change.
