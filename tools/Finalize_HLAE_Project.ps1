param(
    [Parameter(Mandatory = $true)][string]$ProjectDirectory,
    [Parameter(Mandatory = $true)][string]$SourceDemo,
    [string]$ParserDirectory = ''
)

$ErrorActionPreference = 'Stop'
$project = (Resolve-Path -LiteralPath $ProjectDirectory).Path
$capturePath = Join-Path $project 'hlae_capture.json'
$agrPath = Join-Path $project 'sfm_import.agr'
foreach ($required in @($capturePath, $agrPath)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) { throw "Required output is missing: $required" }
}
$capture = Get-Content -LiteralPath $capturePath -Raw | ConvertFrom-Json
$parserRoot = if ($ParserDirectory) { (Resolve-Path -LiteralPath $ParserDirectory).Path } else { $project }
$manifestPath = Join-Path $parserRoot 'manifest.json'
$parserManifest = if (Test-Path -LiteralPath $manifestPath -PathType Leaf) { Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json } else { $null }

$combined = [ordered]@{
    format = 'tf2-stv-sfm-project'
    format_version = 2
    status = 'complete'
    source_demo = (Resolve-Path -LiteralPath $SourceDemo).Path
    created_utc = [DateTime]::UtcNow.ToString('o')
    animation_fidelity = 'retail-client-authoritative'
    pipeline = [ordered]@{
        parser = if ($parserManifest) { 'tf-demo-parser export_all' } else { 'not run for capture-only job' }
        animation_capture = 'Retail TF2 x64 demo playback + HLAE AfxHookSource mirv_agr'
        capture_fps = 30
        insecure = $true
        issued_connect_command = $false
        sdk_ghost_worker_used = $false
    }
    capture = $capture
    counts = if ($parserManifest) { [ordered]@{
        decoded_packets = [int64]$parserManifest.packet_count
        logical_animation_frames = [int64]$parserManifest.animation_export.logical_frames
        resolved_player_samples = [int64]$parserManifest.animation_export.player_samples
        animation_events = [int64]$parserManifest.animation_export.animation_events
    }} else { $null }
    files = [ordered]@{
        parser_data_directory = if ($parserManifest -and $ParserDirectory) { $parserRoot } else { $null }
        parser_manifest = if ($parserManifest) { (Join-Path $parserRoot 'manifest.json') } else { $null }
        packets = if ($parserManifest) { (Join-Path $parserRoot 'packets.ndjson') } else { $null }
        animation_inputs = if ($parserManifest) { (Join-Path $parserRoot 'animation_inputs.ndjson') } else { $null }
        sfm_game_record = 'sfm_import.agr'
        capture_settings = 'hlae_capture.json'
        capture_console = 'tf2_console.log'
        generated_vdm = 'hlae_capture.vdm'
        generated_cfg = 'hlae_bootstrap.cfg'
    }
    note = 'The earlier SDK ghost bones path is retained only as a diagnostic experiment because visual testing showed reference-pose/T-pose output. This project uses the actual retail TF2 client animation result.'
}
$combined | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath (Join-Path $project 'project.json') -Encoding UTF8
Write-Host "Validated and wrote $(Join-Path $project 'project.json')"
