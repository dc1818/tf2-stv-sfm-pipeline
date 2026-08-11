param(
    [Parameter(Mandatory = $true)] [string] $Executable,
    [Parameter(Mandatory = $true)] [string] $GameDirectory,
    [Parameter(Mandatory = $true)] [string] $InputPath,
    [Parameter(Mandatory = $true)] [string] $OutputPath,
    [Parameter(Mandatory = $true)] [string] $ConsoleLog
)

$ErrorActionPreference = 'Stop'

Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;

public static class TF2WorkerWindow
{
    [DllImport("user32.dll")]
    public static extern bool ShowWindowAsync(IntPtr hWnd, int nCmdShow);
}
'@

function Quote-NativeArgument([string] $Value) {
    return '"' + $Value.Replace('"', '\"') + '"'
}

$nativeArguments = @(
    '-game', (Quote-NativeArgument $GameDirectory),
    '-steam',
    '-insecure',
    '-ip', '127.0.0.1',
    '-nomaster',
    '-nohltv',
    '-textmode',
    '-noshaderapi',
    '-windowed',
    '-noborder',
    '-w', '640',
    '-h', '480',
    '-novid',
    '-nosound',
    '-nojoy',
    '-nosteamcontroller',
    '-console',
    '-condebug',
    '-dev',
    '-tf_bone_worker_input', (Quote-NativeArgument $InputPath),
    '-tf_bone_worker_output', (Quote-NativeArgument $OutputPath),
    '+developer', '1',
    '+sv_lan', '1',
    '+map', 'tf_bone_worker'
) -join ' '

Write-Host ''
Write-Host '=== TF2 STV animation worker ==='
Write-Host 'Mode: local listen worker; no demo playback and no remote connect command'
Write-Host 'Network guard: VAC disabled (-insecure), loopback only (127.0.0.1), sv_lan 1, no master listing'
Write-Host 'Display guard: 640x480 windowed mode; Source is minimized so this terminal remains visible'
Write-Host "Input:  $InputPath"
Write-Host "Output: $OutputPath"
Write-Host "Log:    $ConsoleLog"
Write-Host 'Starting Source SDK Base 2013 Multiplayer...'
Write-Host ''

$process = Start-Process -FilePath $Executable -ArgumentList $nativeArguments -WorkingDirectory (Split-Path -Parent $Executable) -WindowStyle Minimized -PassThru
Write-Host "Source process ID: $($process.Id)"
Write-Host 'Waiting for local-map sign-on. Live Source console follows:'
Write-Host '---------------------------------------------------------'

$reader = $null
$workerStarted = $false
$startTime = Get-Date
$nextStatus = $startTime.AddSeconds(10)
$lastMinimizedWindow = [IntPtr]::Zero
$windowMinimizeUnavailable = $false

try {
    while (-not $process.HasExited) {
        $process.Refresh()
        if (-not $windowMinimizeUnavailable) {
            try {
                $windowHandle = $process.MainWindowHandle
                if ($windowHandle -is [IntPtr] -and $windowHandle -ne [IntPtr]::Zero -and $windowHandle -ne $lastMinimizedWindow) {
                    [void] [TF2WorkerWindow]::ShowWindowAsync($windowHandle, 6)
                    $lastMinimizedWindow = $windowHandle
                }
            }
            catch {
                $windowMinimizeUnavailable = $true
                Write-Warning 'Could not minimize the Source window automatically; live worker monitoring will continue.'
            }
        }

        if ($null -eq $reader -and [IO.File]::Exists($ConsoleLog)) {
            $stream = New-Object IO.FileStream($ConsoleLog, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::ReadWrite)
            $reader = New-Object IO.StreamReader($stream)
        }

        if ($null -ne $reader) {
            while ($true) {
                $line = $reader.ReadLine()
                if ($null -eq $line) { break }
                Write-Host $line
                if ($line -like '*TF bone worker: client sign-on complete*') {
                    $workerStarted = $true
                }
            }
        }

        $now = Get-Date
        if ($now -ge $nextStatus) {
            $size = if ([IO.File]::Exists($OutputPath)) { (Get-Item -LiteralPath $OutputPath).Length } else { 0 }
            if ($workerStarted) {
                Write-Host "[launcher] Worker is running; bones.ndjson size: $size bytes"
            } else {
                Write-Host '[launcher] Source is still initializing the local worker map...'
            }
            $nextStatus = $now.AddSeconds(10)
        }

        if (-not $workerStarted -and ((Get-Date) - $startTime).TotalSeconds -gt 180) {
            Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
            Write-Error 'The client did not start the bone worker within 180 seconds. The local Source process was stopped.'
        }

        Start-Sleep -Milliseconds 250
    }

    if ($null -ne $reader) {
        while ($true) {
            $line = $reader.ReadLine()
            if ($null -eq $line) { break }
            Write-Host $line
        }
    }
}
finally {
    if ($null -ne $reader) { $reader.Dispose() }
}

$process.WaitForExit()
Write-Host '---------------------------------------------------------'
Write-Host "Source exited with code $($process.ExitCode)."

# Source can terminate before Windows makes the final closed output handle
# visible to this process. Retry instead of reporting a false missing file.
$outputDeadline = (Get-Date).AddSeconds(60)
$nextOutputWaitMessage = Get-Date
while (-not [IO.File]::Exists($OutputPath) -and (Get-Date) -lt $outputDeadline) {
    if ((Get-Date) -ge $nextOutputWaitMessage) {
        Write-Host '[launcher] Waiting for Windows to publish the completed bones.ndjson file...'
        $nextOutputWaitMessage = (Get-Date).AddSeconds(5)
    }
    Start-Sleep -Milliseconds 500
}

if ([IO.File]::Exists($OutputPath)) {
    Write-Host "bones.ndjson final size: $((Get-Item -LiteralPath $OutputPath).Length) bytes"
} else {
    Write-Error "bones.ndjson was not visible after 60 seconds: $OutputPath"
}

exit 0
