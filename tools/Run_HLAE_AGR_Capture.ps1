param(
    [Parameter(Mandatory = $true)][string]$DemoPath,
    [string]$ProjectDirectory = '',
    [string]$BatchPlanPath = '',
    [long]$StartTick = -1,
    [long]$EndTick = -1,
    [string]$HlaePath = '',
    [string]$Tf2Root = '',
    [int]$PreRollTicks = 330,
    [int]$TimeoutMinutes = 120
)

$ErrorActionPreference = 'Stop'
$pipelineRoot = Split-Path -Parent $PSScriptRoot

function Resolve-FilePath([string]$Path, [string]$Description) {
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description was not found: $Path"
    }
    return (Resolve-Path -LiteralPath $Path).Path
}

function Resolve-HlaeExecutable([string]$Requested) {
    $candidates = New-Object System.Collections.Generic.List[string]
    if ($Requested) { $candidates.Add($Requested) }
    if ($env:HLAE_EXE) { $candidates.Add($env:HLAE_EXE) }
    $remembered = Join-Path $pipelineRoot 'HLAE_PATH.txt'
    if (Test-Path -LiteralPath $remembered -PathType Leaf) {
        $saved = (Get-Content -LiteralPath $remembered -Raw).Trim()
        if ($saved) { $candidates.Add($saved) }
    }
    $candidates.Add('C:\HLAE\HLAE.exe')
    if (${env:ProgramFiles(x86)}) { $candidates.Add((Join-Path ${env:ProgramFiles(x86)} 'HLAE\HLAE.exe')) }
    if ($env:ProgramFiles) { $candidates.Add((Join-Path $env:ProgramFiles 'HLAE\HLAE.exe')) }

    foreach ($candidate in $candidates) {
        if (-not $candidate) { continue }
        $test = $candidate
        if (Test-Path -LiteralPath $test -PathType Container) { $test = Join-Path $test 'HLAE.exe' }
        if (Test-Path -LiteralPath $test -PathType Leaf) {
            $resolved = (Resolve-Path -LiteralPath $test).Path
            [IO.File]::WriteAllText($remembered, $resolved + [Environment]::NewLine)
            return $resolved
        }
    }

    Write-Host ''
    Write-Host 'HLAE was not found.' -ForegroundColor Yellow
    Write-Host 'Install HLAE 2.189.0 or newer from:'
    Write-Host 'https://github.com/advancedfx/advancedfx/releases/latest'
    $entered = Read-Host 'Then paste the full path to HLAE.exe'
    if (-not $entered) { throw 'HLAE.exe is required for the animation-faithful capture.' }
    $resolvedEntered = Resolve-FilePath $entered 'HLAE.exe'
    [IO.File]::WriteAllText($remembered, $resolvedEntered + [Environment]::NewLine)
    return $resolvedEntered
}

function Get-SteamRoots {
    $roots = New-Object System.Collections.Generic.List[string]
    if (${env:ProgramFiles(x86)}) { $roots.Add((Join-Path ${env:ProgramFiles(x86)} 'Steam')) }
    if ($env:ProgramFiles) { $roots.Add((Join-Path $env:ProgramFiles 'Steam')) }
    try {
        $steamReg = (Get-ItemProperty -Path 'HKCU:\Software\Valve\Steam' -Name SteamPath -ErrorAction Stop).SteamPath
        if ($steamReg) { $roots.Add(($steamReg -replace '/', '\')) }
    } catch { }

    $expanded = New-Object System.Collections.Generic.List[string]
    foreach ($root in $roots) {
        if (-not $root -or -not (Test-Path -LiteralPath $root -PathType Container)) { continue }
        $expanded.Add($root)
        $libraries = Join-Path $root 'steamapps\libraryfolders.vdf'
        if (Test-Path -LiteralPath $libraries -PathType Leaf) {
            foreach ($line in Get-Content -LiteralPath $libraries) {
                if ($line -match '^\s*"path"\s+"([^"]+)"') {
                    $expanded.Add(($Matches[1] -replace '\\\\', '\'))
                }
            }
        }
    }
    return $expanded | Select-Object -Unique
}

function Resolve-Tf2Root([string]$Requested) {
    $candidates = New-Object System.Collections.Generic.List[string]
    if ($Requested) { $candidates.Add($Requested) }
    if ($env:TF2_ROOT) { $candidates.Add($env:TF2_ROOT) }
    foreach ($steam in Get-SteamRoots) {
        $candidates.Add((Join-Path $steam 'steamapps\common\Team Fortress 2'))
    }
    foreach ($candidate in $candidates) {
        if ($candidate -and (Test-Path -LiteralPath (Join-Path $candidate 'tf_win64.exe') -PathType Leaf)) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }
    throw 'Team Fortress 2\tf_win64.exe was not found. Set TF2_ROOT to the Team Fortress 2 folder.'
}

function Resolve-X64Hook([string]$HlaeExe) {
    $root = Split-Path -Parent $HlaeExe
    $hooks = @(Get-ChildItem -LiteralPath $root -Filter 'AfxHookSource.dll' -File -Recurse -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -match '[\\/]x64[\\/]' })
    if ($hooks.Count -eq 0) {
        throw "The x64 AfxHookSource.dll was not found under $root. Install HLAE 2.189.0 or newer."
    }
    return $hooks[0].FullName
}

function Read-CaptureTick([string]$Label, [long]$Current, [string]$Hint) {
    if ($Current -ge 0) { return $Current }
    while ($true) {
        $raw = Read-Host "$Label ($Hint)"
        $value = 0L
        if ([long]::TryParse($raw, [ref]$value) -and $value -ge 0) { return $value }
        Write-Host 'Enter a whole-number demo tick, such as 18600.' -ForegroundColor Yellow
    }
}

function Show-NewConsoleLog([string]$Path) {
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) { return }
    $length = (Get-Item -LiteralPath $Path).Length
    if ($length -lt $script:consoleOffset) { $script:consoleOffset = 0L }
    if ($length -eq $script:consoleOffset) { return }
    $stream = [IO.File]::Open($Path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::ReadWrite)
    try {
        [void]$stream.Seek($script:consoleOffset, [IO.SeekOrigin]::Begin)
        $remaining = [int]($length - $script:consoleOffset)
        $bytes = New-Object byte[] $remaining
        $read = $stream.Read($bytes, 0, $remaining)
        if ($read -gt 0) {
            $text = [Text.Encoding]::Default.GetString($bytes, 0, $read)
            Write-Host -NoNewline $text
            $script:consoleOffset += $read
        }
    } finally {
        $stream.Dispose()
    }
}

function Wait-ForExclusiveRead([string]$Path, [int]$Seconds) {
    $deadline = [DateTime]::UtcNow.AddSeconds($Seconds)
    while ([DateTime]::UtcNow -lt $deadline) {
        try {
            $stream = [IO.File]::Open($Path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::None)
            $stream.Dispose()
            return
        } catch {
            Start-Sleep -Milliseconds 500
        }
    }
    throw "Timed out waiting for the AGR file to close: $Path"
}

$demo = Resolve-FilePath $DemoPath 'Demo'
if ([IO.Path]::GetExtension($demo) -ine '.dem') { throw 'The input must be a .dem file.' }
$hlaeExe = Resolve-HlaeExecutable $HlaePath
$hookDll = Resolve-X64Hook $hlaeExe
$tfRoot = Resolve-Tf2Root $Tf2Root
$tfExe = Join-Path $tfRoot 'tf_win64.exe'
$tfGame = Join-Path $tfRoot 'tf'
$tfCfg = Join-Path $tfGame 'cfg'
$consoleLog = Join-Path $tfGame 'console.log'

$runningTf = @(Get-Process -Name 'tf_win64' -ErrorAction SilentlyContinue)
if ($runningTf.Count -gt 0) {
    throw 'TF2 is already running. Close it first so the launcher cannot attach to the wrong process.'
}

if (-not $ProjectDirectory) {
    $stamp = Get-Date -Format 'yyyyMMdd_HHmmss'
    $ProjectDirectory = Join-Path (Split-Path -Parent $demo) (([IO.Path]::GetFileNameWithoutExtension($demo)) + "_hlae_sfm_$stamp")
}
if (-not (Test-Path -LiteralPath $ProjectDirectory -PathType Container)) {
    [void](New-Item -ItemType Directory -Path $ProjectDirectory)
}
$project = (Resolve-Path -LiteralPath $ProjectDirectory).Path

$captureRows = New-Object System.Collections.Generic.List[object]
if ($BatchPlanPath) {
    $resolvedPlan = Resolve-FilePath $BatchPlanPath 'Batch capture plan'
    $planRows = @(Import-Csv -LiteralPath $resolvedPlan -Delimiter "`t")
    if ($planRows.Count -eq 0) { throw 'The batch capture plan contains no clips.' }
    foreach ($row in $planRows) {
        $index = 0
        $start = 0L
        $end = 0L
        if (-not [int]::TryParse($row.clip_index, [ref]$index) -or $index -lt 1) { throw 'The batch plan has an invalid clip_index.' }
        if (-not [long]::TryParse($row.start_tick, [ref]$start) -or $start -lt 0) { throw "Clip $index has an invalid start_tick." }
        if (-not [long]::TryParse($row.end_tick, [ref]$end) -or $end -le $start) { throw "Clip $index has an invalid end_tick." }
        if (-not $row.clip_directory) { throw "Clip $index has no output directory." }
        if (-not (Test-Path -LiteralPath $row.clip_directory -PathType Container)) {
            [void](New-Item -ItemType Directory -Path $row.clip_directory -Force)
        }
        $captureRows.Add([pscustomobject]@{
            Index = $index
            StartTick = $start
            EndTick = $end
            ClipDirectory = (Resolve-Path -LiteralPath $row.clip_directory).Path
        })
    }
    $captureRows = @($captureRows | Sort-Object StartTick, EndTick)
} else {
    $StartTick = Read-CaptureTick 'Start demo tick' $StartTick 'use demoui; 0 starts at the beginning'
    $EndTick = Read-CaptureTick 'End demo tick' $EndTick ('10 seconds is about 667 ticks; suggested end: ' + ($StartTick + 667))
    if ($EndTick -le $StartTick) { throw 'The end tick must be greater than the start tick.' }
    $captureRows.Add([pscustomobject]@{ Index = 1; StartTick = $StartTick; EndTick = $EndTick; ClipDirectory = $project })
    $captureRows = @($captureRows)
}

for ($i = 0; $i -lt $captureRows.Count; $i++) {
    $capture = $captureRows[$i]
    if (($capture.EndTick - $capture.StartTick) -gt 4000) {
        Write-Host "WARNING: Clip $($capture.Index) is long. Start with 300-667 ticks to avoid SFM memory crashes." -ForegroundColor Yellow
    }
    if ($i -gt 0 -and $capture.StartTick -le $captureRows[$i - 1].EndTick) {
        throw "Clip $($capture.Index) overlaps or touches clip $($captureRows[$i - 1].Index). AGR ranges must not overlap."
    }
}

$job = 'tf2sfm_' + (Get-Date -Format 'yyyyMMdd_HHmmss') + '_' + $PID
$demoLeaf = $job + '.dem'
$vdmLeaf = $job + '.vdm'
$cfgLeaf = $job + '.cfg'
$tempDemo = Join-Path $tfGame $demoLeaf
$tempVdm = Join-Path $tfGame $vdmLeaf
$tempCfg = Join-Path $tfCfg $cfgLeaf
$auditVdm = Join-Path $project 'hlae_capture.vdm'
$auditCfg = Join-Path $project 'hlae_bootstrap.cfg'
$loaderOut = Join-Path $project 'hlae_loader_stdout.log'
$loaderErr = Join-Path $project 'hlae_loader_stderr.log'

$captures = New-Object System.Collections.Generic.List[object]
foreach ($row in $captureRows) {
    $agrLeaf = $job + ('_clip_{0:000}.agr' -f $row.Index)
    $candidates = @(
        (Join-Path $tfRoot $agrLeaf),
        (Join-Path $tfGame $agrLeaf),
        (Join-Path $pipelineRoot $agrLeaf),
        (Join-Path (Get-Location).Path $agrLeaf),
        (Join-Path (Split-Path -Parent $hlaeExe) $agrLeaf)
    ) | Select-Object -Unique
    $captures.Add([pscustomobject]@{
        Index = $row.Index
        StartTick = $row.StartTick
        EndTick = $row.EndTick
        ClipDirectory = $row.ClipDirectory
        AgrLeaf = $agrLeaf
        AgrCandidates = $candidates
        FinalAgr = Join-Path $row.ClipDirectory 'sfm_import.agr'
    })
}
$allAgrCandidates = @($captures | ForEach-Object { $_.AgrCandidates } | Select-Object -Unique)

$actions = New-Object System.Collections.Generic.List[string]
$actionNumber = 1
$firstCapture = $captures[0]
$skipTick = [Math]::Max(1L, $firstCapture.StartTick - [Math]::Max(0, $PreRollTicks))
if ($skipTick -gt 1) {
    $actions.Add("`t`"$actionNumber`"`r`n`t{`r`n`t`tfactory `"SkipAhead`"`r`n`t`tname `"TF2SFM preroll`"`r`n`t`tstarttick `"1`"`r`n`t`tskiptotick `"$skipTick`"`r`n`t}")
    $actionNumber++
}
for ($i = 0; $i -lt $captures.Count; $i++) {
    $capture = $captures[$i]
    $recordTick = [Math]::Max(1L, $capture.StartTick)
    $startCommands = "echo TF2SFM_BATCH_CLIP_START_$($capture.Index); host_timescale 1; host_framerate 30; mirv_agr start $($capture.AgrLeaf)"
    $actions.Add("`t`"$actionNumber`"`r`n`t{`r`n`t`tfactory `"PlayCommands`"`r`n`t`tname `"TF2SFM start AGR clip $($capture.Index)`"`r`n`t`tstarttick `"$recordTick`"`r`n`t`tcommands `"$startCommands`"`r`n`t}")
    $actionNumber++
    $stopCommands = "echo TF2SFM_BATCH_CLIP_STOP_$($capture.Index); mirv_agr stop; host_timescale 1; host_framerate 0"
    if ($i -eq $captures.Count - 1) { $stopCommands += '; quit' }
    $actions.Add("`t`"$actionNumber`"`r`n`t{`r`n`t`tfactory `"PlayCommands`"`r`n`t`tname `"TF2SFM stop AGR clip $($capture.Index)`"`r`n`t`tstarttick `"$($capture.EndTick)`"`r`n`t`tcommands `"$stopCommands`"`r`n`t}")
    $actionNumber++
    if ($i -lt $captures.Count - 1) {
        $nextCapture = $captures[$i + 1]
        $skipActionTick = $capture.EndTick + 1
        $nextPreRollTick = [Math]::Max($skipActionTick, $nextCapture.StartTick - [Math]::Max(0, $PreRollTicks))
        if ($nextPreRollTick -gt $skipActionTick) {
            $actions.Add("`t`"$actionNumber`"`r`n`t{`r`n`t`tfactory `"SkipAhead`"`r`n`t`tname `"TF2SFM seek to clip $($nextCapture.Index)`"`r`n`t`tstarttick `"$skipActionTick`"`r`n`t`tskiptotick `"$nextPreRollTick`"`r`n`t}")
            $actionNumber++
        }
    }
}
$vdmText = "demoactions`r`n{`r`n" + ($actions -join "`r`n") + "`r`n}`r`n"

$cfgText = @"
// Generated by TF2 STV to SFM. Loaded before playdemo.
echo TF2SFM_BOOTSTRAP_BEGIN
sv_lan 1
con_enable 1
developer 1
engine_no_focus_sleep 0
cl_ragdoll_forcefade 0
cl_ragdoll_fade_time 999999
cl_ragdoll_physics_enable 1
ragdoll_sleepaftertime 999999
mirv_agr enabled 1
mirv_agr debug 1
mirv_agr recordCamera 1
mirv_agr recordPlayers 1
mirv_agr recordWeapons 1
mirv_agr recordProjectiles 1
mirv_agr recordInvisible 1
mirv_agr recordViewmodel 0
echo TF2SFM_BOOTSTRAP_PLAYDEMO
playdemo $job
"@

[IO.File]::WriteAllText($auditVdm, $vdmText, [Text.Encoding]::ASCII)
[IO.File]::WriteAllText($auditCfg, $cfgText, [Text.Encoding]::ASCII)
Copy-Item -LiteralPath $demo -Destination $tempDemo
Copy-Item -LiteralPath $auditVdm -Destination $tempVdm
Copy-Item -LiteralPath $auditCfg -Destination $tempCfg

if (Test-Path -LiteralPath $consoleLog -PathType Leaf) {
    Copy-Item -LiteralPath $consoleLog -Destination (Join-Path $project 'tf2_console_before_capture.log')
    Move-Item -LiteralPath $consoleLog -Destination (Join-Path $project ('tf2_console_previous_' + $job + '.log'))
}

Write-Host ''
Write-Host '=== TF2 retail/HLAE multi-clip animation capture ===' -ForegroundColor Cyan
Write-Host "Demo:       $demo"
Write-Host "Clips:      $($captures.Count) in one TF2 process"
foreach ($capture in $captures) { Write-Host "  Clip $($capture.Index): $($capture.StartTick)-$($capture.EndTick) -> $($capture.FinalAgr)" }
Write-Host 'AGR rate:   30 fps (required by the AdvancedFX workflow)'
Write-Host "TF2:        $tfExe"
Write-Host "HLAE:       $hlaeExe"
Write-Host "Hook:       $hookDll"
Write-Host ''
Write-Host 'SAFETY: -insecure, sv_lan 1, and playdemo only. No connect or matchmaking command is issued.' -ForegroundColor Yellow
Write-Host 'Do not use this HLAE-launched TF2 window to join any server.' -ForegroundColor Yellow
Write-Host ''
Write-Host 'Launching HLAE. Live TF2 console output follows when console.log appears:'
Write-Host '-----------------------------------------------------------------------'

$gameCmdLine = "-steam -insecure +sv_lan 1 -window -w 960 -h 540 -console -condebug -novid -noborder +exec $cfgLeaf"
$hlaeArguments = '-customLoader -noGui -autoStart -hookDllPath "{0}" -programPath "{1}" -cmdLine "{2}"' -f $hookDll, $tfExe, $gameCmdLine
$launchTime = [DateTime]::Now
$hlaeProcess = Start-Process -FilePath $hlaeExe -ArgumentList $hlaeArguments -PassThru -RedirectStandardOutput $loaderOut -RedirectStandardError $loaderErr

$tfProcess = $null
$launchDeadline = [DateTime]::UtcNow.AddSeconds(45)
while ([DateTime]::UtcNow -lt $launchDeadline -and -not $tfProcess) {
    $tfProcess = Get-Process -Name 'tf_win64' -ErrorAction SilentlyContinue |
        Where-Object { $_.StartTime -ge $launchTime.AddSeconds(-2) } |
        Sort-Object StartTime -Descending |
        Select-Object -First 1
    if (-not $tfProcess) {
        if ($hlaeProcess.HasExited -and $hlaeProcess.ExitCode -ne 0) {
            throw "HLAE custom loader exited with code $($hlaeProcess.ExitCode). See $loaderErr"
        }
        Start-Sleep -Milliseconds 500
    }
}
if (-not $tfProcess) { throw "TF2 did not start within 45 seconds. See $loaderErr" }

Write-Host "TF2 process ID: $($tfProcess.Id)"
$script:consoleOffset = 0L
$deadline = [DateTime]::UtcNow.AddMinutes($TimeoutMinutes)
$lastProgress = [DateTime]::MinValue
while (-not $tfProcess.HasExited) {
    Show-NewConsoleLog $consoleLog
    if ([DateTime]::UtcNow -gt $deadline) {
        throw "Capture exceeded $TimeoutMinutes minutes. TF2 was left open for inspection."
    }
    if ([DateTime]::UtcNow.Subtract($lastProgress).TotalSeconds -ge 10) {
        $candidate = $allAgrCandidates | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1
        if ($candidate) {
            Write-Host ("`n[AGR progress] {0:N0} bytes" -f (Get-Item -LiteralPath $candidate).Length) -ForegroundColor DarkCyan
        } else {
            Write-Host "`n[waiting] TF2 is loading/seeking; AGR recording has not started yet." -ForegroundColor DarkCyan
        }
        $lastProgress = [DateTime]::UtcNow
    }
    Start-Sleep -Milliseconds 750
    $tfProcess.Refresh()
}
Show-NewConsoleLog $consoleLog
Write-Host "`nTF2 exited with code $($tfProcess.ExitCode)."

if (Test-Path -LiteralPath $consoleLog) { Copy-Item -LiteralPath $consoleLog -Destination (Join-Path $project 'tf2_console.log') -Force }

foreach ($capture in $captures) {
    $producedAgr = $capture.AgrCandidates | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1
    if (-not $producedAgr) {
        throw "HLAE did not create AGR clip $($capture.Index). Search tf2_console.log for TF2SFM_BATCH_CLIP_$($capture.Index), mirv_agr, or Unknown command."
    }
    Wait-ForExclusiveRead $producedAgr 60
    if ((Get-Item -LiteralPath $producedAgr).Length -lt 32) { throw "AGR clip $($capture.Index) is too small to contain animation frames." }
    $header = New-Object byte[] 18
    $stream = [IO.File]::OpenRead($producedAgr)
    try { $read = $stream.Read($header, 0, $header.Length) } finally { $stream.Dispose() }
    if ($read -ne 18 -or [Text.Encoding]::ASCII.GetString($header, 0, 13) -ne 'afxGameRecord' -or $header[13] -ne 0) {
        throw "AGR clip $($capture.Index) does not have an afxGameRecord header."
    }
    $agrVersion = [BitConverter]::ToInt32($header, 14)
    Move-Item -LiteralPath $producedAgr -Destination $capture.FinalAgr -Force
    Copy-Item -LiteralPath $auditVdm -Destination (Join-Path $capture.ClipDirectory 'hlae_capture.vdm') -Force
    Copy-Item -LiteralPath $auditCfg -Destination (Join-Path $capture.ClipDirectory 'hlae_bootstrap.cfg') -Force
    if (Test-Path -LiteralPath $consoleLog) { Copy-Item -LiteralPath $consoleLog -Destination (Join-Path $capture.ClipDirectory 'tf2_console.log') -Force }
    $settings = [ordered]@{
        format = 'tf2-hlae-agr-capture'
        format_version = 2
        source_demo = $demo
        start_demo_tick = $capture.StartTick
        end_demo_tick = $capture.EndTick
        duration_ticks = $capture.EndTick - $capture.StartTick
        estimated_duration_seconds = ($capture.EndTick - $capture.StartTick) / 66.6667
        pre_roll_ticks = $PreRollTicks
        capture_fps = 30
        capture_method = 'Retail TF2 x64 demo playback with HLAE AfxHookSource mirv_agr'
        batch_capture = ($captures.Count -gt 1)
        batch_clip_index = $capture.Index
        batch_clip_count = $captures.Count
        single_tf2_process = $true
        hlae_executable = $hlaeExe
        hook_dll = $hookDll
        tf2_executable = $tfExe
        insecure = $true
        remote_connect_command_issued = $false
        agr_file = 'sfm_import.agr'
        agr_version = $agrVersion
        agr_size_bytes = (Get-Item -LiteralPath $capture.FinalAgr).Length
        created_utc = [DateTime]::UtcNow.ToString('o')
    }
    $settings | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $capture.ClipDirectory 'hlae_capture.json') -Encoding UTF8
    Write-Host "Validated AGR clip $($capture.Index): $($capture.FinalAgr)" -ForegroundColor Green
}

Remove-Item -LiteralPath $tempDemo, $tempVdm, $tempCfg -Force -ErrorAction SilentlyContinue

Write-Host ''
Write-Host 'HLAE MULTI-CLIP AGR CAPTURE PASSED' -ForegroundColor Green
Write-Host "Files:       $($captures.Count) separate sfm_import.agr files"
Write-Host 'TF2 launches: 1'
Write-Host 'This file contains the retail TF2 client animation result; it does not use the SDK ghost bones.'
