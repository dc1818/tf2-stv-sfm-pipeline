# TF2 STV to SFM Pipeline

Windows tools for turning a Team Fortress 2 SourceTV demo into SFM-ready clips.

The pipeline parses the demo once, captures final client animation through retail TF2 and HLAE, and writes one importable `.agr` file per selected clip. It also includes setup tools for bringing current TF2 content into Source Filmmaker.

## Safety

The retail capture worker launches TF2 with `-insecure`, LAN mode, and `playdemo`. It does not issue remote server-connect or matchmaking commands.
