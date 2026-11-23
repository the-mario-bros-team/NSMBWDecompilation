# Builds the Windows programs of a release into one folder, laid out as the
# zip and the setup program carry them:
#
#   nsmbw-launcher.exe       the launcher, for any x86-64 processor
#   nsmbw-port.exe           the game, for x86-64-v3 (AVX2, FMA, BMI2)
#   compat\nsmbw-port.exe    the game, for x86-64-v2 (SSE4.2, POPCNT): the
#                            "compat" build for processors without AVX2
#   LICENSE.txt
#
# README.txt is written for each release and added by hand.
#
#   powershell -File scripts\build-windows.ps1 -Out C:\nsmbw-data\release-v0.0.3\nsmbw-port-v0.0.3 -Target C:\nsmbw-data\worktrees\release-target
#
# Each build has its own target folder, as its RUSTFLAGS differ from
# .cargo/config.toml's (which is x86-64-v3 for everything): <Target> for the
# game, <Target>-compat for the compat build, <Target>-launcher for the
# launcher. -Cargo runs another cargo (a script ending in .ps1 is run with
# PowerShell), such as one that limits its CPU.
#
# Releases are built from a clean checkout with an empty
# crates\game-recomp\gen, so no recompiled game code is shipped: the script
# stops if gen holds anything, unless -Local says the build is for this PC
# only.
param(
    [Parameter(Mandatory = $true)][string]$Out,
    [string]$Target = '',
    [string]$Cargo = 'cargo',
    [switch]$Local
)
$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
if ($Target -eq '') { $Target = Join-Path $repo 'target' }
$gen = Join-Path $repo 'crates\game-recomp\gen'
if (-not $Local -and (Test-Path $gen) -and (Get-ChildItem $gen -Force | Select-Object -First 1)) {
    throw "$gen holds recompiled game code, which must never be shipped. Build releases from a clean checkout (or pass -Local for a build that stays on this PC)."
}

function Invoke-Build([string]$targetDir, [string]$rustflags, [string]$package) {
    $env:CARGO_TARGET_DIR = $targetDir
    # RUSTFLAGS replaces .cargo/config.toml's rustflags; empty keeps them.
    if ($rustflags -eq '') { Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue } else { $env:RUSTFLAGS = $rustflags }
    Push-Location $repo
    try {
        if ($Cargo.EndsWith('.ps1')) {
            & powershell -NoProfile -File $Cargo build --release -p $package
        } else {
            & $Cargo build --release -p $package
        }
        if ($LASTEXITCODE -ne 0) { throw "cargo build -p $package failed ($LASTEXITCODE)" }
    } finally {
        Pop-Location
    }
}

$saved = @{ CARGO_TARGET_DIR = $env:CARGO_TARGET_DIR; RUSTFLAGS = $env:RUSTFLAGS }
try {
    Invoke-Build $Target '' 'frontend'
    Invoke-Build "$Target-compat" '-C target-cpu=x86-64-v2' 'frontend'
    Invoke-Build "$Target-launcher" '-C target-cpu=x86-64' 'launcher'
} finally {
    foreach ($k in $saved.Keys) {
        if ($null -eq $saved[$k]) { Remove-Item "Env:$k" -ErrorAction SilentlyContinue } else { Set-Item "Env:$k" $saved[$k] }
    }
}

New-Item -ItemType Directory -Force (Join-Path $Out 'compat') | Out-Null
Copy-Item (Join-Path $Target 'release\nsmbw-port.exe') $Out
Copy-Item (Join-Path "$Target-compat" 'release\nsmbw-port.exe') (Join-Path $Out 'compat')
Copy-Item (Join-Path "$Target-launcher" 'release\nsmbw-launcher.exe') $Out
Copy-Item (Join-Path $repo 'LICENSE') (Join-Path $Out 'LICENSE.txt')
Get-ChildItem $Out -Recurse -File | ForEach-Object { "{0} ({1} KB)" -f $_.FullName, [int]($_.Length / 1KB) }
