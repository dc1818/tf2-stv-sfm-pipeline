param(
    [switch]$IncludeSound,
    [string]$Tf2Root = '',
    [string]$SfmRoot = ''
)

$ErrorActionPreference = 'Stop'

$steamRoot = Join-Path ${env:ProgramFiles(x86)} 'Steam'
if (-not $Tf2Root) { $Tf2Root = Join-Path $steamRoot 'steamapps\common\Team Fortress 2' }
if (-not $SfmRoot) { $SfmRoot = Join-Path $steamRoot 'steamapps\common\SourceFilmmaker' }
$tf2Root = $Tf2Root.TrimEnd('\', '/')
$sfmRoot = $SfmRoot.TrimEnd('\', '/')

if (-not (Test-Path -LiteralPath (Join-Path $tf2Root 'tf\tf2_misc_dir.vpk') -PathType Leaf)) {
    throw "TF2 was not found at $tf2Root"
}
if (-not (Test-Path -LiteralPath (Join-Path $sfmRoot 'game\usermod\gameinfo.txt') -PathType Leaf)) {
    throw "SFM was not found at $sfmRoot"
}

$outputRoot = Join-Path $sfmRoot 'game\tf_fix'
New-Item -ItemType Directory -Force -Path $outputRoot | Out-Null

$gameInfoText = @'
"GameInfo"
{
    "game" "TF2 current content for SFM"
    "FileSystem"
    {
        "SteamAppId" "1840"
        "SearchPaths"
        {
            "Game" "|gameinfo_path|."
        }
    }
}
'@
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
[IO.File]::WriteAllText((Join-Path $outputRoot 'gameinfo.txt'), $gameInfoText, $utf8NoBom)

$wantedRoots = @('models', 'materials', 'particles')
if ($IncludeSound) {
    $wantedRoots += 'sound'
}

function Read-NullTerminatedString {
    param([IO.BinaryReader]$Reader)

    $bytes = New-Object System.Collections.Generic.List[byte]
    while ($true) {
        $value = $Reader.ReadByte()
        if ($value -eq 0) {
            break
        }
        $bytes.Add($value)
    }
    return [Text.Encoding]::ASCII.GetString($bytes.ToArray())
}

function Expand-SelectedVpkContent {
    param([string]$DirectoryVpk)

    $directoryStream = [IO.File]::OpenRead($DirectoryVpk)
    $reader = New-Object IO.BinaryReader($directoryStream)

    try {
        $signature = $reader.ReadUInt32()
        if ($signature -ne 0x55AA1234) {
            throw "Not a Valve VPK file: $DirectoryVpk"
        }

        $version = $reader.ReadUInt32()
        $treeSize = $reader.ReadUInt32()
        $headerSize = 12

        if ($version -eq 2) {
            $null = $reader.ReadUInt32()
            $null = $reader.ReadUInt32()
            $null = $reader.ReadUInt32()
            $null = $reader.ReadUInt32()
            $headerSize = 28
        }
        elseif ($version -ne 1) {
            throw "Unsupported VPK version ${version}: $DirectoryVpk"
        }

        $treeEnd = [int64]$headerSize + [int64]$treeSize
        $archiveBase = [IO.Path]::GetFileNameWithoutExtension($DirectoryVpk) -replace '_dir$', ''
        $archiveFolder = Split-Path -Parent $DirectoryVpk

        while ($directoryStream.Position -lt $treeEnd) {
            $extension = Read-NullTerminatedString -Reader $reader
            if ($extension.Length -eq 0) {
                break
            }

            while ($true) {
                $folder = Read-NullTerminatedString -Reader $reader
                if ($folder.Length -eq 0) {
                    break
                }

                while ($true) {
                    $fileName = Read-NullTerminatedString -Reader $reader
                    if ($fileName.Length -eq 0) {
                        break
                    }

                    $null = $reader.ReadUInt32()
                    $preloadLength = $reader.ReadUInt16()
                    $archiveIndex = $reader.ReadUInt16()
                    $entryOffset = $reader.ReadUInt32()
                    $entryLength = $reader.ReadUInt32()
                    $terminator = $reader.ReadUInt16()
                    if ($terminator -ne 0xFFFF) {
                        throw "Invalid VPK directory entry in $DirectoryVpk"
                    }
                    $preloadData = $reader.ReadBytes($preloadLength)

                    $relativePath = ''
                    if ($folder -ne ' ') {
                        $relativePath = $folder + '/'
                    }
                    $relativePath += $fileName
                    if ($extension -ne ' ') {
                        $relativePath += '.' + $extension
                    }

                    $topFolder = $relativePath.Split('/')[0].ToLowerInvariant()
                    if ($wantedRoots -notcontains $topFolder) {
                        continue
                    }

                    $windowsRelativePath = $relativePath -replace '/', '\'
                    $destination = Join-Path $outputRoot $windowsRelativePath
                    $destinationFolder = Split-Path -Parent $destination
                    New-Item -ItemType Directory -Force -Path $destinationFolder | Out-Null

                    $outputStream = [IO.File]::Create($destination)
                    try {
                        if ($preloadData.Length -gt 0) {
                            $outputStream.Write($preloadData, 0, $preloadData.Length)
                        }

                        if ($entryLength -gt 0) {
                            if ($archiveIndex -eq 0x7FFF) {
                                $dataFile = $DirectoryVpk
                                $dataOffset = [int64]$headerSize + [int64]$treeSize + [int64]$entryOffset
                            }
                            else {
                                $archiveName = $archiveBase + '_' + $archiveIndex.ToString('000') + '.vpk'
                                $dataFile = Join-Path $archiveFolder $archiveName
                                $dataOffset = [int64]$entryOffset
                            }

                            $dataStream = [IO.File]::OpenRead($dataFile)
                            try {
                                $dataStream.Position = $dataOffset
                                $buffer = New-Object byte[] 1048576
                                $remaining = [int64]$entryLength
                                while ($remaining -gt 0) {
                                    $requested = [Math]::Min([int64]$buffer.Length, $remaining)
                                    $read = $dataStream.Read($buffer, 0, [int]$requested)
                                    if ($read -le 0) {
                                        throw "Unexpected end of VPK data file: $dataFile"
                                    }
                                    $outputStream.Write($buffer, 0, $read)
                                    $remaining -= $read
                                }
                            }
                            finally {
                                $dataStream.Dispose()
                            }
                        }
                    }
                    finally {
                        $outputStream.Dispose()
                    }
                }
            }
        }
    }
    finally {
        $reader.Dispose()
        $directoryStream.Dispose()
    }
}

$archives = @('tf2_misc_dir.vpk', 'tf2_textures_dir.vpk')
if ($IncludeSound) {
    $archives += @('tf2_sound_misc_dir.vpk', 'tf2_sound_vo_english_dir.vpk')
}

Write-Host 'Extracting current TF2 assets. This can take several minutes.'
$archiveNumber = 0
$archiveCount = $archives.Count
Write-Host 'TF2SFM_PROGRESS: 2'
foreach ($archiveName in $archives) {
    $archiveNumber++
    $beforePercent = [Math]::Floor(5 + (85 * ($archiveNumber - 1) / $archiveCount))
    Write-Host ("TF2SFM_PROGRESS: {0}" -f $beforePercent)
    Write-Host ("TF2SFM_STAGE: extracting archive {0} of {1}" -f $archiveNumber, $archiveCount)
    $archivePath = Join-Path $tf2Root (Join-Path 'tf' $archiveName)
    if (-not (Test-Path -LiteralPath $archivePath -PathType Leaf)) {
        throw "Required TF2 archive was not found: $archivePath"
    }
    Write-Host "Extracting $archiveName ..."
    Expand-SelectedVpkContent -DirectoryVpk $archivePath
    $afterPercent = [Math]::Floor(5 + (85 * $archiveNumber / $archiveCount))
    Write-Host ("TF2SFM_PROGRESS: {0}" -f $afterPercent)
}

Write-Host 'TF2SFM_STAGE: copying TF2 item definitions and verifying output'
$itemsSource = Join-Path $tf2Root 'tf\scripts\items\items_game.txt'
if (Test-Path -LiteralPath $itemsSource -PathType Leaf) {
    $itemsFolder = Join-Path $outputRoot 'scripts\items'
    New-Item -ItemType Directory -Force -Path $itemsFolder | Out-Null
    Copy-Item -LiteralPath $itemsSource -Destination (Join-Path $itemsFolder 'items_game.txt') -Force
}

$buildableFolder = Join-Path $outputRoot 'models\buildables'
$modelCount = 0
if (Test-Path -LiteralPath $buildableFolder -PathType Container) {
    $modelCount = @(Get-ChildItem -LiteralPath $buildableFolder -Recurse -File -Filter '*.mdl').Count
}
if ($modelCount -eq 0) {
    throw 'No buildable MDL files were extracted. The operation is not valid.'
}

Write-Host ''
Write-Host 'TF2 CONTENT EXTRACTION PASSED' -ForegroundColor Green
Write-Host 'TF2SFM_PROGRESS: 100'
Write-Host "Output: $outputRoot"
Write-Host "Buildable MDL files: $modelCount"
Write-Host 'Run Enable_TF_Fix_In_SFM.bat, then restart SFM completely.'
