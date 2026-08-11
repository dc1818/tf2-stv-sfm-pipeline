# TF2 STV to SFM — retail HLAE capture pipeline

This bundle parses a TF2 STV demo and records a selected tick range as an SFM-ready AdvancedFX GameRecord (`sfm_import.agr`). Version 0.2.0 uses the actual current retail TF2 client through HLAE. This is the animation-faithful route: TF2 itself plays the demo and computes the final player, weapon, overlay, pose-parameter, ragdoll, and bone state that HLAE records.

The earlier Source SDK 2013 ghost worker remains in the bundle only for diagnosis and research. It created a structurally valid `bones.ndjson`, but the SFM visual test showed T-pose/reference-pose players. Structural validity was not proof of animation fidelity.

## Safety boundary

HLAE is a process hook. The launcher always uses `-insecure`, `+sv_lan 1`, and a generated `playdemo` command. It never issues `connect`, `map`, matchmaking, or a public-server command. It refuses to start if `tf_win64.exe` is already running.

Do not join any server from the HLAE-launched TF2 window. Close it if the automated job does not close it.

## Downloads

Install these once:

- Current Steam **Team Fortress 2**. Launch it normally once, then close it.
- **HLAE 2.189.0 or newer**: <https://github.com/advancedfx/advancedfx/releases/latest>
- **AdvancedFX SFM import scripts**: the GUI downloads and installs the game-record rig automatically after you select the SFM folder. The upstream source is <https://github.com/advancedfx/afx-sfm-scripts>.
- **Rust stable** (for the supplied STV parser): <https://rustup.rs/>
- **Source Filmmaker** in Steam.

The main HLAE pipeline does not require Visual Studio, Source SDK Base 2013 Multiplayer, or the SDK source checkout.

The setup page checks the AdvancedFX game-record rig separately from the TF2 content and
`tf_fix` search path. It shows PASS only after
`game\platform\scripts\sfm\animset\advancedfx_import_gameRecord.py` exists and passes
content validation. If it is missing, the GUI installs it automatically.

## Fix missing current TF2 models

If TF2 itself displays the demo correctly but SFM shows error models, run:

```text
Mount_Live_TF2_Content_For_SFM.bat
```

This adds the installed TF2 `tf` directory to SFM's `game\usermod\gameinfo.txt`, so SFM can read the current TF2 VPKs and loose files. It creates a timestamped backup before editing and writes a BOM-free KeyValues file for older SFM parsers. Restart SFM completely afterward. To undo the change, run `Restore_SFM_Gameinfo_Backup.bat`.

This is preferable to copying individual VPK contents because it keeps SFM synchronized with the installed TF2 version. Valve documents `gameinfo.txt` as the file that defines Source search paths, and current TF2's own `gameinfo.txt` mounts its VPKs through those paths. [Valve GameInfo documentation](https://developer.valvesoftware.com/wiki/Gameinfo.txt), [current TF2 gameinfo](https://github.com/SteamDatabase/GameTracking-TF2/blob/master/tf/gameinfo.txt).

If SFM closes immediately after the mount, run `Restore_SFM_Gameinfo_Backup.bat` or `Emergency_Restore_SFM_Gameinfo.bat` before launching SFM again. The emergency BAT restores the newest timestamped backup beside the SFM file.

The mount fixes missing stock models, materials, particles, and sounds. It cannot make an AGR contain an entity that HLAE did not record. Engineer buildings are dynamic entities and may require a separate building track if they are absent from the AGR entirely.

## First setup

Use a freshly extracted complete bundle. The parser source file
`parser\src\bin\export_all.rs` must be present before building.

Extract this folder, open it, and run:

```text
Install_Or_Update.bat
Build_All.bat
```

`Install_Or_Update.bat` finds or remembers `HLAE.exe` and verifies that the x64 `AfxHookSource.dll` is present. `Build_All.bat` now builds only the Rust parser.

## Complete parser + animation project

Drag a `.dem` onto:

```text
Process_STV_To_SFM.bat
```

The program asks for a start demo tick and an end demo tick. TF2 is normally about 66.67 ticks per second:

| Length | Approximate ticks |
| ---: | ---: |
| 5 seconds | 333 |
| 10 seconds | 667 |
| 15 seconds | 1000 |
| 30 seconds | 2000 |

Start with 300–667 ticks because SFM can run out of memory on large multi-player AGR files. The launcher skips to a 330-tick pre-roll point, lets TF2 rebuild animation history, starts `mirv_agr` on the exact requested tick, stops on the end tick, and quits TF2.

The timestamped project folder beside the demo contains:

- `packets.ndjson`, `packet_index.ndjson`, and the parser JSON outputs;
- `animation_inputs.ndjson` and `worker_frames.tsv` from the original parser extension;
- `sfm_import.agr` recorded directly by HLAE from retail TF2;
- `project.json` linking the parsed data and capture range;
- `hlae_capture.json`, generated VDM/CFG files, and `tf2_console.log` for debugging.

The AGR is the animation data used by SFM. A separate `bones.ndjson` is intentionally not generated on this path because HLAE writes the client-computed transforms directly into AGR.

## Capture only

If the demo is already parsed, drag it onto:

```text
Record_Demo_With_HLAE.bat
```

This skips JSON parsing and creates a small capture folder containing `sfm_import.agr` and diagnostics.

## How automation replaces the video steps

The launcher performs the tutorial workflow without the Demo Editor UI:

1. launches `tf_win64.exe` with HLAE's x64 custom loader;
2. executes `mirv_agr enabled 1` before `playdemo`;
3. enables players, weapons, projectiles, camera, invisible entities, and ragdoll persistence;
4. creates a same-name `.vdm` with `SkipAhead` and tick-based `PlayCommands`;
5. starts AGR at 30 FPS on the start tick;
6. stops AGR, restores timing, and quits on the end tick;
7. verifies the `afxGameRecord` header before reporting success.

## Import into SFM

Read [SFM_IMPORT.md](SFM_IMPORT.md). In short: create a dummy camera animation set, right-click it, select **Rig → advancedfx_import_gameRecord**, and choose `sfm_import.agr`.

## Diagnostic SDK files

The old research route is explicitly named:

- `Install_SDK_Diagnostic_Worker.bat`
- `Build_SDK_Diagnostic_Worker.bat`
- `Process_STV_To_SFM_SDK_Diagnostic.bat`

Do not use its AGR for a final SFM shot unless its animation implementation is repaired and visually validated against HLAE.
