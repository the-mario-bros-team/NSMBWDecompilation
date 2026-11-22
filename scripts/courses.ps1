# P7 smoke test: start every course on the disc (Stage/WW-LL.arc) through
# --course, hold right and hop for about 15 seconds of game time, and record
# whether it started, whether player 1 appeared and moved, any guest error,
# and Vulkan validation errors. Run from the repository root after
# `cargo build --release -p frontend`:
#   powershell -NoProfile -File scripts\courses.ps1 -Out D:\courses [-Only 01-01,03-24]
# Writes <Out>\results.csv, one log and two screenshots per course. Courses
# start from a state saved at the World 1 map (made once, from boot), so a
# fresh save in file 1 is needed as for scripts/1-1.press.
param(
    [Parameter(Mandatory)][string]$Out,
    [string[]]$Only,
    [string]$Exe = 'target\release\nsmbw-port.exe'
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$game = ((Get-Content port.toml) -match "^game_dir" -replace "^game_dir\s*=\s*'(.*)'.*", '$1')
$state = Join-Path $Out 'map.state'
if (-not (Test-Path $state)) {
    Write-Host 'saving the World 1 map state...'
    & $Exe --unpaced --no-audio --press-file scripts\to-map.press --save-state "6090:$state" --frames 6091 | Out-Null
}
# Enter the course at 6100 (to-map.press), then hold right and run, hopping.
$moves = @('6250:down+1:900') + (0..21 | ForEach-Object { "$(6260 + 40 * $_):2:20" })
$courses = Get-ChildItem (Join-Path $game 'files\Stage') -Filter '??-??.arc' | ForEach-Object { $_.BaseName }
if ($Only) { $Only = $Only -split ','; $courses = $courses | Where-Object { $Only -contains $_ } }
$rows = foreach ($c in $courses) {
    $log = Join-Path $Out "$c.log"
    $a = @('--unpaced', '--no-audio', '--load-state', $state, '--press-file', 'scripts\to-map.press', '--course', $c, '--watch-player', '--frames', '1110',
        '--shot', "6500:$(Join-Path $Out "$c-a.png")", '--shot', "7100:$(Join-Path $Out "$c-b.png")")
    foreach ($m in $moves) { $a += @('--press', $m) }
    # The port prints to stderr too, which `&` would turn into errors.
    # Below normal priority, so the PC stays responsive during the run.
    $t = Measure-Command {
        $p = Start-Process -FilePath $Exe -ArgumentList $a -NoNewWindow -PassThru -RedirectStandardOutput $log -RedirectStandardError "$log.err"
        try { $p.PriorityClass = 'BelowNormal' } catch {}
        $p.WaitForExit()
    }
    Get-Content "$log.err" | Add-Content $log
    Remove-Item "$log.err"
    $text = Get-Content $log
    $xs = @($text | Select-String '^player at field (\d+): x (-?[\d.]+)' | ForEach-Object { [double]$_.Matches[0].Groups[2].Value })
    $err = ($text | Select-String '^error: (.*)' | Select-Object -First 1 | ForEach-Object { $_.Matches[0].Groups[1].Value })
    $val = ($text | Select-String 'validation errors: (\d+)' | Select-Object -Last 1 | ForEach-Object { [int]$_.Matches[0].Groups[1].Value })
    $row = [pscustomobject]@{
        course     = $c
        started    = [bool]($text | Select-String '^course: ')
        player     = $xs.Count -gt 0
        moved      = if ($xs.Count) { [math]::Round(($xs | Measure-Object -Maximum).Maximum - $xs[0], 1) } else { 0 }
        validation = $val
        error      = $err
        seconds    = [math]::Round($t.TotalSeconds)
    }
    Write-Host ("{0}  started={1} player={2} moved={3} validation={4} {5}" -f $row.course, $row.started, $row.player, $row.moved, $row.validation, $row.error)
    $row
}
$rows | Export-Csv -NoTypeInformation (Join-Path $Out 'results.csv')
$bad = @($rows | Where-Object { -not $_.player -or $_.error -or $_.validation })
Write-Host "$($rows.Count) courses, $($bad.Count) with problems"
exit [int]($bad.Count -gt 0)
