$ErrorActionPreference = "Stop"

function Invoke-Native {
    param(
        [Parameter(Mandatory = $true)] [string] $Command,
        [Parameter(Mandatory = $true)] [string[]] $Arguments
    )

    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed with exit code ${LASTEXITCODE}: $Command $($Arguments -join ' ')"
    }
}

# Keep temp files in workspace to reduce host permission friction.
$tmpDir = Join-Path (Get-Location) ".tmp/phase6"
New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null
$env:TMP = $tmpDir
$env:TEMP = $tmpDir

Write-Host "== Phase 6: Rust tests (ffi_demo + wasm_demo) =="
Invoke-Native -Command "cargo" -Arguments @("test", "-p", "ffi_demo", "-p", "wasm_demo")

if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "wasm-pack not found in PATH."
    Write-Host "Install it with: cargo install wasm-pack"
    exit 1
}

Write-Host ""
Write-Host "== Phase 6: Build wasm package =="
Push-Location "crates/wasm_demo"
try {
    Invoke-Native -Command "wasm-pack" -Arguments @("build", "--target", "web")
}
finally {
    Pop-Location
}

if (-not (Test-Path "crates/wasm_demo/pkg/wasm_demo_bg.wasm")) {
    Write-Host "Expected wasm output not found: crates/wasm_demo/pkg/wasm_demo_bg.wasm"
    exit 1
}

Write-Host ""
Write-Host "Phase 6 checks passed."
