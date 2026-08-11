param(
    [Parameter(Mandatory = $true)][string]$ProjectDirectory,
    [Parameter(Mandatory = $true)][string]$SourceDemo
)

$ErrorActionPreference = 'Stop'
$project = (Resolve-Path -LiteralPath $ProjectDirectory).Path
$manifestPath = Join-Path $project 'manifest.json'
$bonesPath = Join-Path $project 'bones.ndjson'
$agrPath = Join-Path $project 'sfm_import.agr'

foreach ($required in @($manifestPath, $bonesPath, $agrPath)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Required output is missing: $required"
    }
}

$parserManifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
$complete = Get-Content -LiteralPath $bonesPath -Tail 1 | ConvertFrom-Json
if ($complete.type -ne 'complete' -or [int64]$complete.failures -ne 0) {
    throw 'bones.ndjson does not end in a successful complete record.'
}

$combined = [ordered]@{
    format = 'tf2-stv-sfm-project'
    format_version = 1
    status = 'complete'
    source_demo = (Resolve-Path -LiteralPath $SourceDemo).Path
    created_utc = [DateTime]::UtcNow.ToString('o')
    pipeline = [ordered]@{
        parser = 'tf-demo-parser export_all with resolved animation input extension'
        animation_worker = 'Source SDK 2013 TF client animation state + SetupBones'
        sdk_commit = '22288b919617be6c8ca3cefd7cca979cbb39a88c'
        bones_format = 3
        sfm_transport = 'AdvancedFX afxGameRecord v6'
        sfm_fps = 30
    }
    counts = [ordered]@{
        decoded_packets = [int64]$parserManifest.packet_count
        logical_animation_frames = [int64]$parserManifest.animation_export.logical_frames
        resolved_player_samples = [int64]$parserManifest.animation_export.player_samples
        animation_events = [int64]$parserManifest.animation_export.animation_events
        captured_player_frames = [int64]$complete.captured_player_frames
        worker_failures = [int64]$complete.failures
    }
    files = [ordered]@{
        parser_manifest = 'manifest.json'
        header = 'header.json'
        packets = 'packets.ndjson'
        packet_index = 'packet_index.ndjson'
        animation_inputs = 'animation_inputs.ndjson'
        worker_input = 'worker_frames.tsv'
        final_bones = 'bones.ndjson'
        sfm_game_record = 'sfm_import.agr'
    }
    fidelity_report = 'See FIDELITY_REPORT.md in the pipeline bundle; proof-of-concept limitations remain for concrete econ weapons, attachments, ragdolls, special taunts, map IK, and retail-only animation changes.'
}

$combined | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $project 'project.json') -Encoding UTF8
Write-Host "Validated and wrote $(Join-Path $project 'project.json')"

