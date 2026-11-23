# Builds the Android app: libnsmbw.so (crates/android) for arm64,
# libadrenotools and its hooks (third_party/libadrenotools, for the GPU Driver
# option; APK assets), then the debug APK
# (android/app/build/outputs/apk/debug/app-debug.apk).
#   powershell -NoProfile -File android\build.ps1 [-Profile release|quick] [-NativeOnly] [-Install]
#                                                 [-ValidationLayer <libVkLayer_khronos_validation.so>]
# -ValidationLayer packs Khronos's Android validation layer (arm64, from the
# Vulkan-ValidationLayers releases' android-binaries) into the APK, for
# finding renderer bugs; the app uses it when its files hold a file named
# "validation". Without the option the APK has no layer.
# Needs the Rust target (rustup target add aarch64-linux-android), the
# Android SDK with an NDK (ANDROID_HOME / ANDROID_NDK_HOME, else
# %LOCALAPPDATA%\Android\Sdk and its newest ndk\*) and CMake (the SDK's
# cmake\*, which brings Ninja), a JDK 17 and Gradle 8
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

# libadrenotools.so, libhook_impl.so and libmain_hook.so: assets, unpacked by
# the app only once a GPU driver is picked (GpuDrivers.java).
$cmakeDir = Get-ChildItem "$env:ANDROID_HOME\cmake" -Directory -ErrorAction SilentlyContinue |
    Sort-Object { [version]$_.Name } | Select-Object -Last 1 -ExpandProperty FullName
if (-not $cmakeDir) { throw 'no CMake in the Android SDK (sdkmanager "cmake;3.22.1")' }
$hooksBuild = "$target\adrenotools"
$cmakeArgs = @('-G', 'Ninja', '-S', "$PSScriptRoot\third_party\libadrenotools", '-B', $hooksBuild,
    "-DCMAKE_MAKE_PROGRAM=$cmakeDir\bin\ninja.exe", "-DCMAKE_TOOLCHAIN_FILE=$ndk\build\cmake\android.toolchain.cmake",
    '-DANDROID_ABI=arm64-v8a', '-DANDROID_PLATFORM=android-28', '-DCMAKE_BUILD_TYPE=Release', '-DBUILD_SHARED_LIBS=ON')
& "$cmakeDir\bin\cmake.exe" @cmakeArgs | Out-Null
if ($LASTEXITCODE -ne 0) { throw "cmake (libadrenotools) failed ($LASTEXITCODE)" }
# Two jobs, as the cargo builds on this PC.
& "$cmakeDir\bin\cmake.exe" --build $hooksBuild --target adrenotools hook_impl main_hook -j 2
if ($LASTEXITCODE -ne 0) { throw "libadrenotools build failed ($LASTEXITCODE)" }
$hooks = "$PSScriptRoot\native\assets\gpu_hooks"
New-Item -ItemType Directory -Force $hooks | Out-Null
foreach ($name in 'libadrenotools.so', 'libhook_impl.so', 'libmain_hook.so') {
    $built = Get-ChildItem $hooksBuild -Recurse -Filter $name | Select-Object -First 1
    if (-not $built) { throw "libadrenotools build: no $name" }
    & "$bin\llvm-strip.exe" --strip-unneeded -o "$hooks\$name" $built.FullName
    if ($LASTEXITCODE -ne 0) { throw "llvm-strip $name failed ($LASTEXITCODE)" }
}
Write-Host "staged libadrenotools and its hooks in $hooks"
if ($NativeOnly) { exit 0 }

& powershell -NoProfile -File "$PSScriptRoot\gradle-limited.ps1" -p $PSScriptRoot --offline assembleDebug
if ($LASTEXITCODE -ne 0) { throw "gradle failed ($LASTEXITCODE)" }
$apk = "$PSScriptRoot\app\build\outputs\apk\debug\app-debug.apk"
Write-Host "APK: $apk"
if ($Install) {
    & adb install -r $apk
    if ($LASTEXITCODE -ne 0) { throw "adb install failed ($LASTEXITCODE)" }
}
