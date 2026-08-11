# Import the HLAE GameRecord into Source Filmmaker

Use the `sfm_import.agr` created by `Process_STV_To_SFM.bat` or `Record_Demo_With_HLAE.bat`. This file is already a short 30 FPS retail-TF2 capture; do not run it through the old `bones.ndjson` clip converter.

1. Download <https://github.com/advancedfx/afx-sfm-scripts>.
2. Copy its `.py` files to `SourceFilmmaker\game\platform\scripts\sfm\animset`.
3. Restart SFM after installing or changing scripts.
4. Create a new session and shot at 30 FPS, then load the correct map.
5. In the Animation Set Editor, create an animation set for a new camera.
6. Right-click that dummy camera and choose **Rig → advancedfx_import_gameRecord**.
7. Select the project folder's `sfm_import.agr` and wait for the import to finish.
8. The importer creates an `afxCam` camera and the recorded entity animation sets. Switch the viewport camera if desired.

If SFM closes during import, capture a shorter tick range. Five to ten seconds (about 333–667 TF2 ticks) is a practical first test. SFM is a 32-bit application, so there is no launch option that gives it an unlimited address space.

If players are still T-posed, verify that you imported the new HLAE `sfm_import.agr`, not an AGR converted from the old SDK `bones.ndjson`. Open `project.json`: it must say `animation_fidelity` is `retail-client-authoritative` and `sdk_ghost_worker_used` is `false`.

The capture terminal must also show `HLAE AGR CAPTURE PASSED`. In the project `tf2_console.log`, search for:

```text
TF2SFM_BOOTSTRAP_BEGIN
TF2SFM_CAPTURE_START
TF2SFM_CAPTURE_STOP
```

An `Unknown command "mirv_agr"` line means HLAE did not hook TF2 correctly or the wrong/non-x64 `AfxHookSource.dll` was used.

