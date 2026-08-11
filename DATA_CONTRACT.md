# Animation data contract

> Status in bundle 0.2.0: the parser formats below are still generated, but `worker_frames.tsv` and `bones.ndjson` belong to the deprecated SDK diagnostic experiment. The supported SFM animation file is now `sfm_import.agr`, captured directly from retail TF2 by HLAE for a selected demo-tick range. `project.json` format version 2 links that AGR to the parser outputs and records the capture ticks and safety settings.

The pipeline deliberately keeps three layers instead of pretending that parsed STV state and final animation are the same thing.

## `animation_inputs.ndjson`

Readable/debuggable resolved records. A `player_animation_input` record includes:

- timeline: logical frame, demo tick, server tick, and seconds;
- identity: entity index and serial number;
- visibility: STV PVS state;
- player state: class, team, alive/health, flags, and water level;
- movement: origin, velocity, and eye angles;
- server animation hints: source sequence, cycle, and playback rate;
- all five TF condition bitfields;
- active-weapon entity/serial/server class and inferred animation role; and
- selected stock player model.

`animation_event` records carry `CTEPlayerAnimEvent` event number, data, and fire delay. This file is useful for inspecting why the worker chose a pose, but it does not contain final bones.

## `worker_frames.tsv`

Versioned, fixed-column streaming contract (`TF2_BONE_WORKER_INPUT 2`). `F` lines carry the numeric state consumed by the SDK; `E` lines carry animation events. Tabs/newlines in string fields are sanitized. The C++ worker rejects a record with the wrong field count and reports a failure instead of shifting columns silently.

## `bones.ndjson`

Version 3 records:

| Record | Purpose |
|---|---|
| `metadata` | Matrix layout, quaternion layout, coordinate spaces, and format version |
| `timeline` | One logical worker time even when no entity is visible |
| `visibility` | Per-entity visibility used to hide/re-show SFM models |
| `skeleton` | Model, bone indices, names, parents, and flags |
| `frame` | Final animated entity transform and complete bone transform array |
| `complete` | Captured-frame count and failure count; required for success |

Each `frame` contains:

- `render_matrix`: row-major 3×4 entity-to-world matrix;
- per bone `w`: row-major 3×4 bone-to-world matrix;
- per bone `l`: row-major 3×4 bone-to-parent matrix (root is relative to the entity);
- per bone `p`: parent-local position; and
- per bone `q`: parent-local quaternion in `[x,y,z,w]` order.

Bones without `BONE_USED_BY_ANYTHING` receive an identity local transform, matching AdvancedFX's public AGR writer behavior. World matrices remain available in the NDJSON for direct analysis; AGR/SFM consumes the local matrices.

## `project.json`

The final combined manifest links the original parser files, resolved animation layer, worker contract, final bones, and SFM AGR. It also records the exact SDK commit and counts for decoded packets, logical frames, player samples, animation events, captured bone frames, and worker failures.
