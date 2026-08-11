# Animation fidelity report

## Confirmed failure in the SDK ghost experiment

The SDK worker successfully wrote a complete 37 GB `tf2-final-bones v3` file with no reported `SetupBones` failures. After conversion and SFM import, however, players appeared in T-pose/reference pose with no useful animation. This proves the former validator checked file structure and completion only; it did not prove that the reconstructed animation state matched retail TF2.

The likely missing state includes current retail animation changes, complete networked animation overlays, pose parameters, sequence transitions, weapon/econ ownership and activity overrides, bone-merged children, interpolation history, cosmetics, taunt scenes, and separate ragdoll state. A client-only SDK player with reduced STV state cannot reliably recreate all of that.

## Primary solution in 0.2.0

The supported pipeline now plays the local demo in current retail TF2 and records through HLAE `mirv_agr`. This captures the animation state after the real TF2 client has decoded the demo and evaluated its client animation system. It is the same underlying method demonstrated in the supplied video, automated with a generated VDM file.

The parsed JSON remains useful for event search, tick selection, metadata, camera planning, and non-animation analysis. It is not substituted for retail bone evaluation.

## Validation criteria

A capture is accepted only when:

- TF2 was launched through the x64 HLAE hook with `-insecure`;
- `mirv_agr enabled 1` ran before `playdemo`;
- tick-bounded start and stop commands ran;
- the AGR is nontrivial in size and begins with a valid `afxGameRecord` header; and
- visual inspection in SFM shows locomotion/aim/gesture animation rather than a reference pose.

The last criterion is intentionally visual. A binary file can be structurally valid while containing wrong transforms.

