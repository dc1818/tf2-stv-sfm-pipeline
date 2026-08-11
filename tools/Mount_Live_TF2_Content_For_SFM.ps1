param(
    [string]$Tf2Root = '',
    [string]$SfmRoot = ''
)

$ErrorActionPreference = 'Stop'

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
        $vdf = Join-Path $root 'steamapps\libraryfolders.vdf'
        if (Test-Path -LiteralPath $vdf -PathType Leaf) {
            foreach ($line in Get-Content -LiteralPath $vdf) {
                if ($line -match '^\s*"path"\s+"([^"]+)"') { $expanded.Add(($Matches[1] -replace '\\\\', '\')) }
            }
        }
    }
    return $expanded | Select-Object -Unique
}

function Find-Tf2Root([string]$Requested) {
    $candidates = New-Object System.Collections.Generic.List[string]
    if ($Requested) { $candidates.Add($Requested) }
    if ($env:TF2_ROOT) { $candidates.Add($env:TF2_ROOT) }
    foreach ($steam in Get-SteamRoots) { $candidates.Add((Join-Path $steam 'steamapps\common\Team Fortress 2')) }
    foreach ($candidate in $candidates) {
        if ($candidate -and (Test-Path -LiteralPath (Join-Path $candidate 'tf\gameinfo.txt') -PathType Leaf)) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }
    throw 'Team Fortress 2 was not found. Set TF2_ROOT or pass -Tf2Root.'
}

function Find-SfmRoot([string]$Requested) {
    $candidates = New-Object System.Collections.Generic.List[string]
    if ($Requested) { $candidates.Add($Requested) }
    if ($env:SFM_ROOT) { $candidates.Add($env:SFM_ROOT) }
    foreach ($steam in Get-SteamRoots) { $candidates.Add((Join-Path $steam 'steamapps\common\SourceFilmmaker')) }
    foreach ($candidate in $candidates) {
        if ($candidate -and (Test-Path -LiteralPath (Join-Path $candidate 'game\usermod\gameinfo.txt') -PathType Leaf)) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }
    throw 'Source Filmmaker was not found. Set SFM_ROOT or pass -SfmRoot.'
}

$tfRootResolved = Find-Tf2Root $Tf2Root
$sfmRootResolved = Find-SfmRoot $SfmRoot
$tfContent = (Resolve-Path -LiteralPath (Join-Path $tfRootResolved 'tf')).Path
$gameInfo = Join-Path $sfmRootResolved 'game\usermod\gameinfo.txt'
$text = Get-Content -LiteralPath $gameInfo -Raw

if ($text -match [regex]::Escape($tfContent)) {
    Write-Host 'TF2 live content is already mounted in SFM.' -ForegroundColor Green
    Write-Host "Mounted path: $tfContent"
    exit 0
}

$match = [regex]::Match($text, '(?m)^([ \t]*)Game\s+\|gameinfo_path\|\.\s*$')
if (-not $match.Success) { throw "Could not find the SFM SearchPaths insertion point in $gameInfo" }

$stamp = Get-Date -Format 'yyyyMMdd_HHmmss'
$backup = "$gameInfo.tf2sfm-backup-$stamp"
Copy-Item -LiteralPath $gameInfo -Destination $backup
$indent = $match.Groups[1].Value
$mountLine = $indent + 'Game "' + ($tfContent -replace '\\', '/') + '"' + [Environment]::NewLine
$newText = $text.Insert($match.Index, $mountLine)
# Windows PowerShell's Set-Content -Encoding UTF8 writes a BOM. Older SFM
# builds can reject that BOM while parsing gameinfo.txt, so write UTF-8
# explicitly without a BOM and preserve the original KeyValues format.
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
[IO.File]::WriteAllText($gameInfo, $newText, $utf8NoBom)

Write-Host 'SFM TF2 CONTENT MOUNT PASSED' -ForegroundColor Green
Write-Host "SFM gameinfo: $gameInfo"
Write-Host "Mounted TF2:  $tfContent"
Write-Host "Backup:       $backup"
Write-Host ''
Write-Host 'Restart SFM completely, then re-import the AGR.'
Write-Host 'This mounts current TF2 VPKs; it does not copy or modify TF2 files.'
