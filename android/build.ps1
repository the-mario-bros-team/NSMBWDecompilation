# Builds the Android app: libnsmbw.so (crates/android) for arm64, then the
# debug APK (android/app/build/outputs/apk/debug/app-debug.apk).
#   powershell -NoProfile -File android\build.ps1 [-Profile release|quick] [-NativeOnly] [-Install]
#                                                 [-ValidationLayer <libVkLayer_khronos_validation.so>]
# -ValidationLayer packs Khronos's Android validation layer (arm64, from the
# Vulkan-ValidationLayers releases' android-binaries) into the APK, for
# finding renderer bugs; the app uses it when its files hold a file named
# "validation". Without the option the APK has no layer.
# Needs the Rust target (rustup target add aarch64-linux-android), the
# Android SDK with an NDK (ANDROID_HOME / ANDROID_NDK_HOME, else
# %LOCALAPPDATA%\Android\Sdk and its newest ndk\*), a JDK 17 and Gradle 8
# (see gradle-limited.ps1). On this PC cargo and Gradle run through the
# build-limit scripts (D:\nsmbw-p6). Set CARGO_TARGET_DIR to build
# somewhere other than target\.
param(
    [ValidateSet('release', 'quick')] [string] $Profile = 'release',
    [switch] $NativeOnly,
    [switch] $Install,
    [string] $ValidationLayer
)
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot

if (-not $env:ANDROID_HOME) { $env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk" }
$ndk = $env:ANDROID_NDK_HOME
if (-not $ndk) {
    $ndk = Get-ChildItem "$env:ANDROID_HOME\ndk" -Directory -ErrorAction SilentlyContinue |
        Sort-Object { [version]$_.Name } | Select-Object -Last 1 -ExpandProperty FullName
}
if (-not $ndk -or -not (Test-Path $ndk)) { throw 'no Android NDK found: set ANDROID_NDK_HOME' }
$bin = "$ndk\toolchains\llvm\prebuilt\windows-x86_64\bin"
# Android 9 (API 28) and up.
$clang = "$bin\aarch64-linux-android28-clang.cmd"
$env:CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER = $clang
$env:CC_aarch64_linux_android = $clang
$env:CXX_aarch64_linux_android = "$bin\aarch64-linux-android28-clang++.cmd"
$env:AR_aarch64_linux_android = "$bin\llvm-ar.exe"

$limited = 'D:\nsmbw-p6\cargo-limited.ps1'
$cargoArgs = @('build', '--profile', $Profile, '-p', 'nsmbw-android', '--target', 'aarch64-linux-android')
Push-Location $root
try {
    if (Test-Path $limited) {
        & powershell -NoProfile -File $limited @cargoArgs
    } else {
        & cargo @cargoArgs
    }
    if ($LASTEXITCODE -ne 0) { throw "cargo failed ($LASTEXITCODE)" }
} finally {
    Pop-Location
}

$target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "$root\target" }
$so = "$target\aarch64-linux-android\$Profile\libnsmbw.so"
$jni = "$PSScriptRoot\native\jniLibs\arm64-v8a"
New-Item -ItemType Directory -Force $jni | Out-Null
Copy-Item $so "$jni\libnsmbw.so" -Force
Write-Host "staged $so"
$layer = "$jni\libVkLayer_khronos_validation.so"
if ($ValidationLayer) {
    Copy-Item $ValidationLayer $layer -Force
    Write-Host "with the validation layer $ValidationLayer"
} elseif (Test-Path $layer) {
    Remove-Item $layer -Force
}
if ($NativeOnly) { exit 0 }

& powershell -NoProfile -File "$PSScriptRoot\gradle-limited.ps1" -p $PSScriptRoot --offline assembleDebug
if ($LASTEXITCODE -ne 0) { throw "gradle failed ($LASTEXITCODE)" }
$apk = "$PSScriptRoot\app\build\outputs\apk\debug\app-debug.apk"
Write-Host "APK: $apk"
if ($Install) {
    & adb install -r $apk
    if ($LASTEXITCODE -ne 0) { throw "adb install failed ($LASTEXITCODE)" }
}
