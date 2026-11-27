# The Rust-body gate (docs/RUST_PLAN.md): the seeded 1-1 replay, interpreted and headless (9700 fields), with
# the switched Rust bodies running in lockstep, compared hash by hash with scripts/rust-gate-baseline.txt.
#
#   scripts/rust-gate.ps1 -Game <extracted SMNE01 rev 1 folder> -Seed <save folder> [-Set 0x800643F0,...]
#                         [-Mode 1|accounted] [-Budget 256] [-Out <runs folder>] [-NoBuild]
#
# -Seed is a folder holding wiimj2d.sav and banner.bin. Make it once: run the replay below with no seed
# and no NSMBW_RUST_* variables, then copy saves\nand\title\00010000\534d4e45\data from its folder. The
# save comes from your own game, so it never goes in the repo.
# Passing means: all 98 hashes equal, 13,194,609,027 instructions, and no failed body. Each body's own
# counts are printed: a body with validated=0 was never called, so the run proves nothing about it.
param(
    [Parameter(Mandatory)][string]$Game,
    [Parameter(Mandatory)][string]$Seed,
    [string]$Set = '', [string]$Mode = '1', [int]$Budget = 256,
    [string]$Out = (Join-Path $env:TEMP 'nsmbw-rust-gate'), [switch]$NoBuild
)
$ErrorActionPreference = 'Continue'
$repo = Split-Path $PSScriptRoot -Parent
if (-not $NoBuild) {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { 'cargo is not on PATH (set RUSTUP_HOME, CARGO_HOME and PATH for your Rust install first)'; exit 1 }
    # 4 jobs at below-normal priority: the PC may be in use.
    $p = Start-Process cargo -ArgumentList 'build', '--profile', 'quick', '-p', 'frontend', '-j', '4' -WorkingDirectory $repo -NoNewWindow -PassThru -RedirectStandardError "$Out-build.err" -RedirectStandardOutput "$Out-build.out"
    $null = $p.Handle  # without this, ExitCode reads empty after WaitForExit
    try { $p.PriorityClass = 'BelowNormal' } catch {}
    $p.WaitForExit()
    if ($p.ExitCode -ne 0) { 'BUILD FAILED'; Select-String -Path "$Out-build.err" -Pattern '^error' -Context 0, 8 | Select-Object -First 6 | ForEach-Object { "$_" }; exit 1 }
}
$run = Join-Path $Out (Get-Date -Format 'MMdd-HHmmss')
$save = Join-Path $run 'saves\nand\title\00010000\534d4e45\data'
New-Item -ItemType Directory -Force $save | Out-Null
Copy-Item (Join-Path $Seed '*') $save
$env:NSMBW_RUST_MEMCPY = '1'
$env:NSMBW_RUST_BODIES = $Mode
$env:NSMBW_RUST_BODIES_VALIDATE = "$Budget"
$env:NSMBW_RUST_BODIES_SET = $Set
$p = Start-Process (Join-Path $repo 'target\quick\nsmbw-port.exe') -ArgumentList '--game', $Game, '--interpreter', '--headless', '--unpaced', '--no-audio', '--press-file', (Join-Path $repo 'scripts\1-1.press'), '--frames', '9700', '--hash-every', '100' -WorkingDirectory $run -NoNewWindow -PassThru -RedirectStandardOutput "$run\run.log" -RedirectStandardError "$run\run.err"
$null = $p.Handle
try { $p.PriorityClass = 'BelowNormal' } catch {}
$p.WaitForExit()
$want = @(Select-String -Path (Join-Path $PSScriptRoot 'rust-gate-baseline.txt') -Pattern 'hash at field (\d+): (\w+)' | ForEach-Object { $_.Matches[0].Groups[2].Value })
$got = @(Select-String -Path "$run\run.log" -Pattern 'hash at field (\d+): (\w+)' | ForEach-Object { $_.Matches[0].Groups[2].Value })
$same = 0; $first = -1
for ($i = 0; $i -lt $want.Count; $i++) { if ($i -lt $got.Count -and $got[$i] -eq $want[$i]) { $same++ } elseif ($first -lt 0) { $first = $i } }
$ins = Select-String -Path "$run\run.log" -Pattern 'guest: (\d+) instructions' | ForEach-Object { $_.Matches[0].Groups[1].Value }
$bodies = (Select-String -Path "$run\run.log" -Pattern '^native bodies:' | Select-Object -Last 1).Line
$pass = ($same -eq $want.Count -and $got.Count -eq $want.Count -and $ins -eq '13194609027' -and $bodies -match 'failed=0')
'{0}: hashes {1}/{2}{3}, instructions {4}, {5}' -f $(if ($pass) { 'PASS' } else { 'FAIL' }), $same, $want.Count, $(if ($first -ge 0) { " (first difference at field $($first * 100))" } else { '' }), $ins, $bodies
Select-String -Path "$run\run.log" -Pattern '^native body ' | ForEach-Object { "  $($_.Line)" }
Select-String -Path "$run\run.err" -Pattern 'mismatch' | Select-Object -First 3 | ForEach-Object { "  $($_.Line)" }
"run: $run"
if (-not $pass) { exit 1 }
