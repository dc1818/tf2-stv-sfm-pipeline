param(
    [Parameter(Mandatory = $true)][string]$DemoPath,
    [string]$ProjectDirectory = '',
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
$StartTick = Read-CaptureTick 'Start demo tick' $StartTick 'use demoui; 0 starts at the beginning'
$EndTick = Read-CaptureTick 'End demo tick' $EndTick ('10 seconds is about 667 ticks; suggested end: ' + ($StartTick + 667))
if ($EndTick -le $StartTick) { throw 'The end tick must be greater than the start tick.' }
if (($EndTick - $StartTick) -gt 4000) {
    Write-Host 'WARNING: This is a long SFM capture. Start with 300-667 ticks to avoid SFM memory crashes.' -ForegroundColor Yellow
}

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

$job = 'tf2sfm_' + (Get-Date -Format 'yyyyMMdd_HHmmss') + '_' + $PID
$demoLeaf = $job + '.dem'
$vdmLeaf = $job + '.vdm'
$cfgLeaf = $job + '.cfg'
$agrLeaf = $job + '.agr'
$tempDemo = Join-Path $tfGame $demoLeaf
$tempVdm = Join-Path $tfGame $vdmLeaf
$tempCfg = Join-Path $tfCfg $cfgLeaf
$tempAgr = Join-Path $tfRoot $agrLeaf
$alternateAgr = Join-Path $tfGame $agrLeaf
$agrCandidates = @(
    $tempAgr,
    $alternateAgr,
    (Join-Path $pipelineRoot $agrLeaf),
    (Join-Path (Get-Location).Path $agrLeaf),
    (Join-Path (Split-Path -Parent $hlaeExe) $agrLeaf)
) | Select-Object -Unique
$finalAgr = Join-Path $project 'sfm_import.agr'
$auditVdm = Join-Path $project 'hlae_capture.vdm'
$auditCfg = Join-Path $project 'hlae_bootstrap.cfg'
$loaderOut = Join-Path $project 'hlae_loader_stdout.log'
$loaderErr = Join-Path $project 'hlae_loader_stderr.log'

$recordTick = [Math]::Max(1L, $StartTick)
$skipTick = [Math]::Max(1L, $StartTick - [Math]::Max(0, $PreRollTicks))
$actions = New-Object System.Collections.Generic.List[string]
$actionNumber = 1
if ($skipTick -gt 1) {
    $actions.Add("`t`"$actionNumber`"`r`n`t{`r`n`t`tfactory `"SkipAhead`"`r`n`t`tname `"TF2SFM preroll`"`r`n`t`tstarttick `"1`"`r`n`t`tskiptotick `"$skipTick`"`r`n`t}")
    $actionNumber++
}
$startCommands = "echo TF2SFM_CAPTURE_START; host_timescale 1; host_framerate 30; mirv_agr start $agrLeaf"
$stopCommands = 'echo TF2SFM_CAPTURE_STOP; mirv_agr stop; host_timescale 1; host_framerate 0; quit'
$actions.Add("`t`"$actionNumber`"`r`n`t{`r`n`t`tfactory `"PlayCommands`"`r`n`t`tname `"TF2SFM start AGR`"`r`n`t`tstarttick `"$recordTick`"`r`n`t`tcommands `"$startCommands`"`r`n`t}")
$actionNumber++
$actions.Add("`t`"$actionNumber`"`r`n`t{`r`n`t`tfactory `"PlayCommands`"`r`n`t`tname `"TF2SFM stop AGR`"`r`n`t`tstarttick `"$EndTick`"`r`n`t`tcommands `"$stopCommands`"`r`n`t}")
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
Write-Host '=== TF2 retail/HLAE animation capture ===' -ForegroundColor Cyan
Write-Host "Demo:       $demo"
Write-Host "Ticks:      $StartTick through $EndTick ($($EndTick - $StartTick) ticks, about $([Math]::Round(($EndTick - $StartTick) / 66.6667, 2)) seconds)"
Write-Host 'AGR rate:   30 fps (required by the AdvancedFX workflow)'
Write-Host "Output:     $finalAgr"
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
        $candidate = $agrCandidates | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1
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

$producedAgr = $agrCandidates | Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } | Select-Object -First 1
if (-not $producedAgr) {
    if (Test-Path -LiteralPath $consoleLog) { Copy-Item -LiteralPath $consoleLog -Destination (Join-Path $project 'tf2_console.log') -Force }
    throw 'HLAE did not create an AGR. Search tf2_console.log for TF2SFM_, mirv_agr, or Unknown command.'
}
Wait-ForExclusiveRead $producedAgr 60
if ((Get-Item -LiteralPath $producedAgr).Length -lt 32) { throw 'The AGR exists but is too small to contain animation frames.' }

$header = New-Object byte[] 18
$stream = [IO.File]::OpenRead($producedAgr)
try { $read = $stream.Read($header, 0, $header.Length) } finally { $stream.Dispose() }
if ($read -ne 18 -or [Text.Encoding]::ASCII.GetString($header, 0, 13) -ne 'afxGameRecord' -or $header[13] -ne 0) {
    throw 'The output does not have an afxGameRecord header.'
}
$agrVersion = [BitConverter]::ToInt32($header, 14)
Move-Item -LiteralPath $producedAgr -Destination $finalAgr -Force
if (Test-Path -LiteralPath $consoleLog) { Copy-Item -LiteralPath $consoleLog -Destination (Join-Path $project 'tf2_console.log') -Force }

$settings = [ordered]@{
    format = 'tf2-hlae-agr-capture'
    format_version = 1
    source_demo = $demo
    start_demo_tick = $StartTick
    end_demo_tick = $EndTick
    duration_ticks = $EndTick - $StartTick
    estimated_duration_seconds = ($EndTick - $StartTick) / 66.6667
    pre_roll_ticks = $PreRollTicks
    capture_fps = 30
    capture_method = 'Retail TF2 x64 demo playback with HLAE AfxHookSource mirv_agr'
    hlae_executable = $hlaeExe
    hook_dll = $hookDll
    tf2_executable = $tfExe
    insecure = $true
    remote_connect_command_issued = $false
    agr_file = 'sfm_import.agr'
    agr_version = $agrVersion
    agr_size_bytes = (Get-Item -LiteralPath $finalAgr).Length
    created_utc = [DateTime]::UtcNow.ToString('o')
}
$settings | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $project 'hlae_capture.json') -Encoding UTF8

Remove-Item -LiteralPath $tempDemo, $tempVdm, $tempCfg -Force -ErrorAction SilentlyContinue

Write-Host ''
Write-Host 'HLAE AGR CAPTURE PASSED' -ForegroundColor Green
Write-Host "File:        $finalAgr"
Write-Host "Size:        $((Get-Item -LiteralPath $finalAgr).Length) bytes"
Write-Host "AGR version: $agrVersion"
Write-Host "Ticks:       $StartTick-$EndTick"
Write-Host 'This file contains the retail TF2 client animation result; it does not use the SDK ghost bones.'
