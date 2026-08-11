param(
    [Parameter(Mandatory = $true)] [string] $Path,
    [int] $TimeoutSeconds = 60
)

$ErrorActionPreference = 'Stop'

if ([IO.Directory]::Exists($Path)) {
    $bonesPath = Join-Path $Path 'bones.ndjson'
} else {
    $bonesPath = $Path
}

$deadline = (Get-Date).AddSeconds([Math]::Max(0, $TimeoutSeconds))
$complete = $null
$lastReadError = $null

do {
    if ([IO.File]::Exists($bonesPath)) {
        try {
            $fileInfo = Get-Item -LiteralPath $bonesPath
            if ($fileInfo.Length -gt 0) {
                $lastLine = Get-Content -LiteralPath $bonesPath -Tail 1
                if ($lastLine) {
                    $candidate = $lastLine | ConvertFrom-Json
                    if ($candidate.type -eq 'complete') {
                        $complete = $candidate
                        break
                    }
                }
            }
        }
        catch {
            $lastReadError = $_.Exception.Message
        }
    }

    if ((Get-Date) -ge $deadline) { break }
    Start-Sleep -Milliseconds 500
} while ($true)

if (-not [IO.File]::Exists($bonesPath)) {
    Write-Error "Missing bones file: $bonesPath"
}

$fileInfo = Get-Item -LiteralPath $bonesPath
if ($null -eq $complete) {
    if ($lastReadError) { Write-Host "Last read error: $lastReadError" }
    Write-Error "bones.ndjson exists but has no complete footer: $bonesPath"
}

$firstLine = Get-Content -LiteralPath $bonesPath -TotalCount 1
$metadata = $firstLine | ConvertFrom-Json

if ($metadata.type -ne 'metadata' -or $metadata.format -ne 'tf2-final-bones') {
    Write-Error 'The first record is not tf2-final-bones metadata.'
}
if ([int64] $complete.captured_player_frames -le 0) {
    Write-Error 'The complete record reports no captured player frames.'
}
if ([int64] $complete.failures -ne 0) {
    Write-Error "The complete record reports $($complete.failures) failures."
}

Write-Host ''
Write-Host 'BONE EXPORT PASSED'
Write-Host "File:          $bonesPath"
Write-Host "Size:          $($fileInfo.Length) bytes"
Write-Host "Format:        $($metadata.format) v$($metadata.format_version)"
Write-Host "Player frames: $($complete.captured_player_frames)"
Write-Host "Failures:      $($complete.failures)"
exit 0
