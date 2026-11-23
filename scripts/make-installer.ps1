# Builds the Windows installer (scripts/installer.iss) from a release folder
# holding nsmbw-launcher.exe, nsmbw-port.exe, compat\nsmbw-port.exe (the
# build for processors without AVX2), LICENSE.txt and README.txt, as the
# release zip does; scripts\build-windows.ps1 builds the programs into it.
# The setup program is written next to the folder:
# nsmbw-port-v<version>-windows-x64-setup.exe. Needs Inno Setup 6.
#
#   powershell -File scripts\make-installer.ps1 -Release C:\nsmbw-data\release-v0.0.2\nsmbw-port-v0.0.2
#
# The version is read from crates\port-update\Cargo.toml (the updater's)
# unless -Version is given. As for the zip, the folder must come from a
# clean checkout with an empty gen\: the setup program carries the same files.
param(
    [Parameter(Mandatory = $true)][string]$Release,
    [string]$Version = ''
)
$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
$Release = (Resolve-Path $Release).Path
foreach ($f in 'nsmbw-launcher.exe', 'nsmbw-port.exe', 'compat\nsmbw-port.exe', 'LICENSE.txt', 'README.txt') {
    if (-not (Test-Path (Join-Path $Release $f))) { throw "$Release has no $f" }
}
if ($Version -eq '') {
    $line = Select-String -Path (Join-Path $repo 'crates\port-update\Cargo.toml') -Pattern '^version\s*=\s*"([^"]+)"' | Select-Object -First 1
    $Version = $line.Matches[0].Groups[1].Value
}
$iscc = @("${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe", "$env:ProgramFiles\Inno Setup 6\ISCC.exe", "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe") | Where-Object { Test-Path $_ } | Select-Object -First 1
if (-not $iscc) { throw 'Inno Setup 6 (ISCC.exe) is not installed: https://jrsoftware.org/isdl.php' }
$out = Split-Path -Parent $Release
& $iscc /Q "/DAppVersion=$Version" "/DSource=$Release" "/O$out" (Join-Path $PSScriptRoot 'installer.iss')
if ($LASTEXITCODE -ne 0) { throw "ISCC failed ($LASTEXITCODE)" }
$setup = Join-Path $out "nsmbw-port-v$Version-windows-x64-setup.exe"
"$setup ($([int]((Get-Item $setup).Length / 1KB)) KB)"
