param([string]$SfmRoot = '')
$ErrorActionPreference = 'Stop'
if ($SfmRoot) { $sfm = $SfmRoot.TrimEnd('\', '/') }
else {
    $steam = ${env:ProgramFiles(x86)} + '\Steam'
    if (-not (Test-Path $steam)) { $steam = (Get-ItemProperty 'HKCU:\Software\Valve\Steam' -Name SteamPath -ErrorAction Stop).SteamPath }
    $sfm = Join-Path $steam 'steamapps\common\SourceFilmmaker'
}
$gameInfo = Join-Path $sfm 'game\usermod\gameinfo.txt'
$fix = Join-Path $sfm 'game\tf_fix'
if (-not (Test-Path $fix)) { throw "tf_fix was not found at $fix. Run Extract_Current_TF2_Content_For_SFM.bat first." }
if (-not (Test-Path $gameInfo)) { throw "SFM gameinfo was not found at $gameInfo" }
$text = [IO.File]::ReadAllText($gameInfo)
if ($text -match '(?im)^\s*"?Game"?\s+"?tf_fix"?\s*$') { Write-Host 'tf_fix is already enabled.'; exit 0 }
$target = [regex]::Match($text, '(?im)^([ \t]*)"?Game"?\s+"?tf"?\s*$')
if (-not $target.Success) { throw 'Could not find the normal tf search-path line. Restore SFM gameinfo, then try again.' }
$stamp = Get-Date -Format 'yyyyMMdd_HHmmss'
$backup = "$gameInfo.tf_fix-backup-$stamp"
Copy-Item -LiteralPath $gameInfo -Destination $backup
$line = $target.Groups[1].Value + 'Game "tf_fix"' + [Environment]::NewLine
$updated = $text.Insert($target.Index, $line)
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
[IO.File]::WriteAllText($gameInfo, $updated, $utf8NoBom)
Write-Host 'tf_fix is now enabled in SFM.' -ForegroundColor Green
Write-Host "Backup: $backup"
Write-Host 'Close both Source Filmmaker and the SDK launcher, then start SFM again.'
