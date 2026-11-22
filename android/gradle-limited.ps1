# Runs Gradle within this PC's build rules, like D:\nsmbw-p6\cargo-limited.ps1:
# two workers, below-normal priority, and only one build at a time (the same
# lock folder). Where the lock folder's parent does not exist, it just runs.
#   powershell -NoProfile -File android\gradle-limited.ps1 -p android assembleDebug
# Gradle is found from GRADLE (a gradle.bat), else the newest 8.x in the
# Gradle wrapper cache; the JDK from JAVA_HOME, else a JDK 17 found under
# %USERPROFILE%\AndroidBuild.
$lock = 'D:\nsmbw-p6\build.lock'
(Get-Process -Id $PID).PriorityClass = 'BelowNormal'

$gradle = $env:GRADLE
if (-not $gradle) {
    $gradle = Get-ChildItem "$env:USERPROFILE\.gradle\wrapper\dists\gradle-8.*-bin\*\gradle-8.*\bin\gradle.bat" -ErrorAction SilentlyContinue |
        Sort-Object { [version]($_.Directory.Parent.Name -replace '^gradle-', '') } | Select-Object -Last 1 -ExpandProperty FullName
}
if (-not $gradle) { Write-Error 'no Gradle 8 found: set GRADLE to a gradle.bat'; exit 2 }
if (-not $env:JAVA_HOME) {
    $jdk = Get-ChildItem "$env:USERPROFILE\AndroidBuild\jdk-17*" -Directory -ErrorAction SilentlyContinue | Select-Object -Last 1
    if ($jdk) { $env:JAVA_HOME = $jdk.FullName }
}
if (-not $env:ANDROID_HOME) { $env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk" }

$locked = $false
if (Test-Path (Split-Path $lock)) {
    $announced = $false
    while ($true) {
        try {
            New-Item -ItemType Directory -Path $lock -ErrorAction Stop | Out-Null
            Set-Content -Path "$lock\pid" -Value $PID
            $locked = $true
            break
        } catch {
            $owner = Get-Content "$lock\pid" -ErrorAction SilentlyContinue
            if ($owner -and -not (Get-Process -Id ([int]$owner) -ErrorAction SilentlyContinue)) {
                Remove-Item $lock -Recurse -Force -ErrorAction SilentlyContinue
                continue
            }
            if (-not $announced) { Write-Host "waiting for another build (pid $owner) to finish..."; $announced = $true }
            Start-Sleep -Seconds 5
        }
    }
}
try {
    & $gradle --max-workers=2 --no-daemon @args
    $code = $LASTEXITCODE
} finally {
    if ($locked) { Remove-Item $lock -Recurse -Force -ErrorAction SilentlyContinue }
}
exit $code
