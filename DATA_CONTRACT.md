# TF2 STV to SFM data contract

The supported pipeline has two outputs for each selected clip:

- parsed STV data from `export_all.exe`; and
- an `sfm_import.agr` captured directly from retail TF2 through HLAE.

## Parsed demo data

`header.json` contains the demo header. `packets.ndjson` contains one decoded top-level
demo packet per line, and `packet_index.ndjson` records each packet's order, tick, type,
and bit-range location in the original demo stream.

`animation_inputs.ndjson` is a resolved, analysis-oriented stream of player state and
player animation events. It is useful for finding ticks, inspecting conditions, and
building metadata. It is not used to synthesize SFM bones.

`manifest.json` records the parser version, source demo path, packet count, parser
incomplete flag, animation-input counts, and the names of the files above.

## SFM animation capture

Each queued ten-second range produces a separate `sfm_import.agr`. Retail TF2 evaluates
the demo and HLAE writes the final game-record data at 30 FPS. `hlae_capture.json`
records the capture range and settings. `tf2_console.log`, `hlae_capture.vdm`, and
`hlae_bootstrap.cfg` are retained for troubleshooting.

`project.json` joins the parser output with the matching AGR and declares
`animation_fidelity: retail-client-authoritative`.
