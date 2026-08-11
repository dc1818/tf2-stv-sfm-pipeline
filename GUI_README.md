# TF2 STV to SFM GUI

Run `Build_GUI_And_Parser.bat` once. After that, open only
`TF2_STV_SFM_GUI.exe`.

## First-run setup gate

The application opens on **Setup** and does not expose the demo page until
all checks pass:

- a valid `HLAE.exe` has been selected;
- the Team Fortress 2 folder containing `tf_win64.exe` has been selected;
- the SourceFilmmaker folder containing `game\usermod\gameinfo.txt` has been selected;
- current TF2 models, materials, particles, and sounds exist in SFM's local
  `game\tf_fix` directory;
- the local `tf_fix` search path is enabled in SFM.

If the content is already complete, the app does not extract it again. If it
is incomplete, the GUI repairs it using the selected TF2 folder and does not
mount that live folder into SFM.
No setup action button is shown while paths or content are incomplete: once the
three selected paths are valid, the GUI automatically repairs missing content
and enables `tf_fix`. **Continue** appears only after every check is PASS.
The setup progress bar fills as each VPK archive is extracted, copied, and
verified; the setup log names the archive currently being handled.
When setup passes, the app automatically switches to the only visible page,
**Demo clips**.

Use **Change setup paths...** on the Demo clips page whenever HLAE, TF2, or
SFM moves. It reopens the setup path selectors and checks the newly selected
installation before returning to Demo clips; it only extracts TF2 content if
the selected SFM installation lacks it.

## Batch clips

On **2. Demo clips**, browse for or drag in a `.dem`. Select the **Output
location** where you want the batch saved. Every run creates a new timestamped
batch folder there, so existing batches are not overwritten. Enter a start tick and
click **+ Add clip**. The GUI automatically creates an end tick 667 ticks
later: a fixed 10-second range. Repeat for each moment you want to capture.

**Create SFM clip project(s)** parses the demo one time into
`parsed_demo_data`, then plays every queued range through retail TF2 and HLAE.
Every clip has its own folder under `clips` and its own `sfm_import.agr` for
SFM import. The progress bar fills as parser and clip stages complete.

The TF2 worker remains demo-only: `-insecure`, LAN mode, and `playdemo`; it
never generates a server-connect or matchmaking command.

The TF2 worker uses a 960×540 windowed client because HLAE AGR needs the actual
retail TF2 client animation/render loop. It is not a fully headless server
process. The GUI stops following the worker as soon as its TF2 process exits or
crashes.

The HLAE worker is the exact known-working worker from v0.5.11. It rejects a
missing, undersized, or invalid-header AGR. Console output is batched and capped
by the GUI so a noisy client cannot flood and lock the window. Cancel terminates
the active wrapper together with its HLAE and TF2 child-process tree.
