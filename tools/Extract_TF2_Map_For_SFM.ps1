param(
    [Parameter(Mandatory = $true)] [string] $MapPath
)

$ErrorActionPreference = 'Stop'
$map = Get-Item -LiteralPath $MapPath
$root = Split-Path -Parent $PSScriptRoot
$name = $map.Name
$lower = $name.ToLowerInvariant()

if ($lower.EndsWith('.bsp.bz2')) {
    $mapBaseName = $name.Substring(0, $name.Length - 8)
    $isBz2 = $true
} elseif ($lower.EndsWith('.bsp')) {
    $mapBaseName = [IO.Path]::GetFileNameWithoutExtension($name)
    $isBz2 = $false
} else {
    throw 'Drag a .bsp or .bsp.bz2 map file onto the BAT.'
}

$out = Join-Path $map.DirectoryName ($mapBaseName + '_sfm_ready')
$mapOut = Join-Path $out 'maps'
$temp = Join-Path $out '_temporary'
New-Item -ItemType Directory -Force -Path $mapOut, $temp | Out-Null

function Find-7Zip {
    $candidates = @()
    if ($env:ProgramFiles) { $candidates += (Join-Path $env:ProgramFiles '7-Zip\7z.exe') }
    if (${env:ProgramFiles(x86)}) { $candidates += (Join-Path ${env:ProgramFiles(x86)} '7-Zip\7z.exe') }
    foreach ($candidate in $candidates) {
        if (Test-Path -LiteralPath $candidate) { return $candidate }
    }
    $command = Get-Command '7z.exe' -ErrorAction SilentlyContinue
    if ($command) { return $command.Source }
    return $null
}

function Get-SteamRoots {
    $roots = New-Object System.Collections.Generic.List[string]
    try {
        $steamPath = (Get-ItemProperty 'HKCU:\Software\Valve\Steam' -ErrorAction Stop).SteamPath
        if ($steamPath) { $roots.Add($steamPath) }
    } catch {}
    if (${env:ProgramFiles(x86)}) { $roots.Add((Join-Path ${env:ProgramFiles(x86)} 'Steam')) }
    if ($env:ProgramFiles) { $roots.Add((Join-Path $env:ProgramFiles 'Steam')) }

    foreach ($rootPath in @($roots)) {
        $libraryFile = Join-Path $rootPath 'steamapps\libraryfolders.vdf'
        if (Test-Path -LiteralPath $libraryFile) {
            foreach ($line in Get-Content -LiteralPath $libraryFile) {
                if ($line -match '^\s*"path"\s*"([^"]+)"') {
                    $roots.Add(($Matches[1] -replace '\\\\', '\'))
                }
            }
        }
    }
    return $roots | Select-Object -Unique
}

function Find-BspZip {
    foreach ($steamRoot in Get-SteamRoots) {
        $tf2Root = Join-Path $steamRoot 'steamapps\common\Team Fortress 2'
        $candidate = Join-Path $tf2Root 'bin\bspzip.exe'
        if (Test-Path -LiteralPath $candidate) {
            return [pscustomobject]@{ Exe = $candidate; Tf2Root = $tf2Root }
        }
    }
    $bundled = Join-Path $root 'tools\bspzip.exe'
    if (Test-Path -LiteralPath $bundled) {
        return [pscustomobject]@{ Exe = $bundled; Tf2Root = $null }
    }
    return $null
}

function Invoke-LoggedNative {
    param([string] $Executable, [string[]] $Arguments, [string] $LogPath)
    $oldPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        & $Executable @Arguments 2>&1 | Tee-Object -FilePath $LogPath | Out-Host
        return $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $oldPreference
    }
}

function Remove-PackedNavigation {
    param(
        [string] $BspPath,
        [string] $BspZipExe,
        [string] $OutputDirectory
    )

    $pakZip = Join-Path $OutputDirectory 'packed_content_without_nav.zip'
    $extractLog = Join-Path $OutputDirectory 'bspzip_pak_extract.log'
    Remove-Item -LiteralPath $pakZip -Force -ErrorAction SilentlyContinue
    $extractResult = Invoke-LoggedNative $BspZipExe @('-extract', $BspPath, $pakZip) $extractLog
    if ($extractResult -ne 0 -or -not (Test-Path -LiteralPath $pakZip)) {
        throw ('Could not extract the BSP pakfile. See ' + $extractLog)
    }

    Add-Type -AssemblyName System.IO.Compression
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $archive = [IO.Compression.ZipFile]::Open($pakZip, [IO.Compression.ZipArchiveMode]::Update)
    try {
        $navEntries = @($archive.Entries | Where-Object { $_.FullName -match '(?i)(^|/)[^/]+\.nav$' })
        foreach ($entry in $navEntries) {
            Write-Host ('Removing incompatible packed navigation: ' + $entry.FullName)
            $entry.Delete()
        }
    } finally {
        $archive.Dispose()
    }

    if ($navEntries.Count -eq 0) {
        Write-Warning 'No packed .nav entry was found. The crash may instead come from a loose .nav beside the SFM map.'
        return 0
    }

    # Source BSP header: ident + version, followed by 64 sixteen-byte lump
    # records. Pakfile is lump 40. Append the modified ZIP and repoint only
    # that lump, leaving every geometry/lighting/entity lump unchanged.
    $pakLumpHeader = 8 + (40 * 16)
    $bspStream = [IO.File]::Open($BspPath, [IO.FileMode]::Open, [IO.FileAccess]::ReadWrite, [IO.FileShare]::None)
    try {
        $reader = New-Object IO.BinaryReader($bspStream, [Text.Encoding]::ASCII, $true)
        $ident = [Text.Encoding]::ASCII.GetString($reader.ReadBytes(4))
        $version = $reader.ReadInt32()
        if ($ident -ne 'VBSP') { throw 'The output file does not have a valid VBSP header.' }
        if ($version -lt 19 -or $version -gt 21) { throw ('Unexpected BSP version ' + $version + '; refusing to modify its lump table.') }

        $bspStream.Position = $pakLumpHeader
        $oldOffset = $reader.ReadInt32()
        $oldLength = $reader.ReadInt32()
        if ($oldOffset -lt 0 -or $oldLength -le 0 -or ([int64]$oldOffset + $oldLength) -gt $bspStream.Length) {
            throw 'The BSP pakfile lump has invalid bounds; refusing to modify it.'
        }
        $reader.Dispose()

        $bspStream.Position = $bspStream.Length
        while (($bspStream.Position % 4) -ne 0) { $bspStream.WriteByte(0) }
        $newOffset = $bspStream.Position
        $pakStream = [IO.File]::OpenRead($pakZip)
        try { $pakStream.CopyTo($bspStream) } finally { $pakStream.Dispose() }
        $newLength = $bspStream.Position - $newOffset
        if ($newOffset -gt [int]::MaxValue -or $newLength -gt [int]::MaxValue) {
            throw 'The modified BSP pakfile exceeds Source 1 offset limits.'
        }

        $writer = New-Object IO.BinaryWriter($bspStream, [Text.Encoding]::ASCII, $true)
        $bspStream.Position = $pakLumpHeader
        $writer.Write([int]$newOffset)
        $writer.Write([int]$newLength)
        $writer.Flush()
        $writer.Dispose()
    } finally {
        $bspStream.Dispose()
    }
    return $navEntries.Count
}

Write-Host '=== Prepare a TF2 map for Source Filmmaker ==='
Write-Host ('Input:  ' + $map.FullName)
Write-Host ('Output: ' + $out)

if ($isBz2) {
    $sevenZip = Find-7Zip
    if (-not $sevenZip) { throw 'Install 7-Zip to remove the outer .bz2 download wrapper.' }
    Write-Host 'Step 1/4: Removing the outer .bz2 download wrapper...'
    $bz2Log = Join-Path $out '7zip_bz2.log'
    $bz2Result = Invoke-LoggedNative $sevenZip @('e', '-y', ("-o$temp"), $map.FullName) $bz2Log
    if ($bz2Result -ne 0) { throw ('7-Zip failed. See ' + $bz2Log) }
    $bsp = Get-ChildItem -LiteralPath $temp -Filter '*.bsp' | Select-Object -First 1
    if (-not $bsp) { throw 'The .bz2 wrapper did not contain a BSP.' }
} else {
    Write-Host 'Step 1/4: Input is already a BSP.'
    $bsp = $map
}

$readyBsp = Join-Path $mapOut ($mapBaseName + '.bsp')
Copy-Item -LiteralPath $bsp.FullName -Destination $readyBsp -Force
Write-Host ('Step 2/4: Made a safe working copy: ' + $readyBsp)

$tool = Find-BspZip
if (-not $tool) {
    throw 'Team Fortress 2\bin\bspzip.exe was not found. Verify TF2 in Steam, or place TF2''s bspzip.exe in this bundle''s tools folder.'
}

Write-Host ('Step 3/4: Decompressing BSP lumps with: ' + $tool.Exe)
Write-Host 'Command: bspzip.exe -repack (without -compress)'
$repackLog = Join-Path $out 'bspzip_repack.log'
$oldVProject = $env:VProject
if ($tool.Tf2Root) { $env:VProject = Join-Path $tool.Tf2Root 'tf' }
Push-Location (Split-Path -Parent $tool.Exe)
try {
    $result = Invoke-LoggedNative $tool.Exe @('-repack', $readyBsp) $repackLog
    if ($result -ne 0) { throw ('bspzip -repack failed with exit code ' + $result + '. See ' + $repackLog) }
    if (-not (Test-Path -LiteralPath $readyBsp) -or (Get-Item -LiteralPath $readyBsp).Length -eq 0) {
        throw 'bspzip reported success but the output BSP is missing or empty.'
    }

    Write-Host 'Step 4/4: Removing packed TF2 navigation data that old SFM cannot read...'
    $removedNavCount = Remove-PackedNavigation $readyBsp $tool.Exe $out
} finally {
    Pop-Location
    $env:VProject = $oldVProject
}
if ($removedNavCount -gt 0) {
    Write-Host ('Removed ' + $removedNavCount + ' packed .nav file(s).')
}

Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue
$sizeMB = [Math]::Round((Get-Item -LiteralPath $readyBsp).Length / 1MB, 2)
Write-Host ''
Write-Host 'SFM MAP PREPARATION PASSED'
Write-Host ('Ready map: ' + $readyBsp)
Write-Host ('Size:      ' + $sizeMB + ' MB')
Write-Host ''
Write-Host 'Copy this BSP to:'
Write-Host '  ...\Steam\steamapps\common\SourceFilmmaker\game\usermod\maps'
Write-Host ('Also remove any loose ' + $mapBaseName + '.nav from that SFM maps folder.')
Write-Host 'Restart SFM, then load koth_product_final from Create Session.'
