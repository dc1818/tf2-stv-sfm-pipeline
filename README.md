# TF2 STV to SFM Pipeline

Windows tools for turning a Team Fortress 2 SourceTV demo into SFM-ready clips.

The pipeline parses the demo once, captures final client animation through retail
TF2 and HLAE, and writes one importable `.agr` file per selected clip. It also
includes setup tools for bringing current TF2 content into Source Filmmaker.

## Start here

1. Run `Build_GUI_And_Parser.bat`.
2. Open `TF2_STV_SFM_GUI.exe`.
3. Complete the HLAE, TF2, and SFM setup checks.
4. Add one or more non-overlapping ten-second clips and create the SFM projects.

See [GUI_README.md](GUI_README.md) for the guided workflow and
[SFM_IMPORT.md](SFM_IMPORT.md) for importing the resulting AGR files.

## Safety

The retail capture worker launches TF2 with `-insecure`, LAN mode, and
`playdemo`. It does not issue remote server-connect or matchmaking commands.

## Repository layout

- `parser/` — bundled STV parser and export binaries source.
- `tools/` — HLAE, SFM-content, validation, and project-finalization scripts.
- `gui/` — Windows Forms setup and clip-capture application.
- `patches/` — deprecated Source SDK diagnostic worker patches.
