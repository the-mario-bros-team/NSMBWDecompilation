# Full project gate: compile, readability, ledger integrity, generated artifacts.
# Usage: .\scripts\check.ps1
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

function Assert-Exit([string]$step) {
  if ($LASTEXITCODE -ne 0) { throw "$step failed (exit $LASTEXITCODE)" }
}

Write-Host '== cargo check =='
cargo check
Assert-Exit 'cargo check'

Write-Host '== doc-lint =='
node tools/doc-lint.mjs src

# No truncated or clobbered source files (a whole-file write once left
# src/ui/lyt.rs at 0 bytes).
node tools/check-source-integrity.mjs
Assert-Exit 'source-integrity'
Assert-Exit 'doc-lint'

Write-Host '== merge ledger =='
node tools/merge-ledger.mjs | Out-Null
Assert-Exit 'merge-ledger'

Write-Host '== verify ledger =='
# Enforces the tsv-inventory invariant too: every ledger address must be a
# table entry in some functions.tsv and rows must never outnumber the inventory.
node tools/verify-ledger.mjs --enforce-bodies
Assert-Exit 'verify-ledger'

# One real body per address: duplicate fn_<addr> names are allowed only as
# one-line 'canonical:' delegations or as differing cfg(target_arch) pairs.
node tools/check-dupe-bodies.mjs
Assert-Exit 'dupe-bodies'

# A documented `lis page` / `addi reg,reg,imm` pair must decode to the declared
# constant; catches the recurring `addi`-as-`ori` sign-extension error.
node tools/check-page-offsets.mjs
Assert-Exit 'page-offsets'

# Address-based guard: no modelled `_savegpr_*`/`_restgpr_*` helper calls may
# survive anywhere in src/. Scans by constant value and numeric literal (both
# cases, with or without `_`), attributes by brace span, and lists dead
# band-valued declarations separately (dead ones are not fatal).
Write-Host '== helper calls =='
node tools/check-helper-calls.mjs
Assert-Exit 'helper-calls'

Write-Host '== regenerate index =='
node tools/index-functions.mjs
Assert-Exit 'index-functions'

Write-Host '== regenerate coverage =='
node tools/coverage-doc.mjs
Assert-Exit 'coverage-doc'

Write-Host '== regenerate progress =='
node tools/progress.mjs | Out-Null
Assert-Exit 'progress'

Write-Host '== regenerate gap report =='
node tools/gap-report.mjs . | Out-Null
Assert-Exit 'gap-report'

Write-Host 'OK'
