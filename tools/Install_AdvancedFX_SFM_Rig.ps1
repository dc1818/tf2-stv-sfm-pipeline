param(
    [Parameter(Mandatory = $true)]
    [string]$SfmRoot,
    [switch]$Force
)

$ErrorActionPreference = 'Stop'

$sfm = $SfmRoot.TrimEnd('\', '/')
$gameInfo = Join-Path $sfm 'game\usermod\gameinfo.txt'
$animsetDir = Join-Path $sfm 'game\platform\scripts\sfm\animset'
$target = Join-Path $animsetDir 'advancedfx_import_gameRecord.py'
$downloadUrl = 'https://raw.githubusercontent.com/advancedfx/afx-sfm-scripts/main/advancedfx_import_gameRecord.py'

if (-not (Test-Path -LiteralPath $gameInfo -PathType Leaf)) {
    throw "SFM was not found at $sfm. Expected game\usermod\gameinfo.txt."
}

function Test-RigFile([string]$path) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { return $false }
    $content = [IO.File]::ReadAllText($path)
    return $content.Contains('ImportGameRecord') -and $content.Contains('afxGameRecord') -and $content.Contains('sfm.CreateAnimationSet')
}

if ((-not $Force) -and (Test-RigFile $target)) {
    Write-Host 'TF2SFM_RIG: AdvancedFX game-record rig is already installed.' -ForegroundColor Green
    Write-Host "TF2SFM_RIG_PATH: $target"
    exit 0
}

New-Item -ItemType Directory -Force -Path $animsetDir | Out-Null
$temp = Join-Path ([IO.Path]::GetTempPath()) ('tf2sfm_advancedfx_' + [guid]::NewGuid().ToString('N') + '.py')
try {
    Write-Host 'TF2SFM_STAGE: downloading AdvancedFX SFM game-record rig'
    Invoke-WebRequest -UseBasicParsing -Uri $downloadUrl -OutFile $temp
    if (-not (Test-RigFile $temp)) {
        throw 'The downloaded AdvancedFX script failed validation.'
    }
    Copy-Item -LiteralPath $temp -Destination $target -Force
}
finally {
    if (Test-Path -LiteralPath $temp) { Remove-Item -LiteralPath $temp -Force -ErrorAction SilentlyContinue }
}

if (-not (Test-RigFile $target)) {
    throw "AdvancedFX rig installation did not validate: $target"
}

Write-Host 'TF2SFM_RIG: AdvancedFX game-record rig installation passed.' -ForegroundColor Green
Write-Host "TF2SFM_RIG_PATH: $target"
Write-Host 'Restart SFM completely before using Rig -> advancedfx_import_gameRecord.'
