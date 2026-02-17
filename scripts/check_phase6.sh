#!/usr/bin/env bash
# Focused regression checks for Phase 6 (FFI + WASM).
set -euo pipefail

# Keep temp files in workspace to reduce host permission friction.
tmp_dir="$(pwd)/.tmp/phase6"
mkdir -p "$tmp_dir"
export TMPDIR="$tmp_dir"
export TEMP="$tmp_dir"
export TMP="$tmp_dir"

echo "== Phase 6: Rust tests (ffi_demo + wasm_demo) =="
cargo test -p ffi_demo -p wasm_demo

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "wasm-pack not found in PATH."
  echo "Install it with: cargo install wasm-pack"
  exit 1
fi

echo ""
echo "== Phase 6: Build wasm package =="
(
  cd crates/wasm_demo
  wasm-pack build --target web
)

if [[ ! -f crates/wasm_demo/pkg/wasm_demo_bg.wasm ]]; then
  echo "Expected wasm output not found: crates/wasm_demo/pkg/wasm_demo_bg.wasm"
  exit 1
fi

echo ""
echo "Phase 6 checks passed."
