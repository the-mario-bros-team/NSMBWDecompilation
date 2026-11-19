# Wave 190, lane B coverage (`game::bases`)

Lane band `0x80776C80..0x80779E40`, all inside the `daWmSandPillar_c` method
cluster of REL `d_basesNP`. Ten of ten requested bodies translated; none
deferred.

## Bodies

| Address | Bytes | Function | Summary |
| --- | --- | --- | --- |
| `0x80776C80` | 136 | `fn_80776c80` | Binds the `+0x1A4` holder to animation part `this + part*0x38 + 0x1E4` and sets the three speeds. |
| `0x80777BA0` | 140 | `fn_80777ba0` | Spawn init: loads the model, snapshots the spawn point, rebuilds the matrix, mirrors the position, applies the per-character y table. |
| `0x80777D20` | 176 | `fn_80777d20` | Rebuilds the transform from position/angles and binds it to the `+0x1AC` holder. |
| `0x80778100` | 164 | `fn_80778100` | Rising-state init: countdown `100`, height `0.5`, `cobSandPillar` clip at `1.0`, clears `+0x4EC`. |
| `0x80778200` | 156 | `fn_80778200` | Same clip re-bind without the `+0x4F0` countdown. |
| `0x807787E0` | 144 | `fn_807787e0` | Integrates the height, clamps to `0.5` and posts `StateID_BottomWait`. |
| `0x80778880` | 148 | `fn_80778880` | Re-binds the clip and sets the top height `2.0`. |
| `0x807789A0` | 176 | `fn_807789a0` | Builds `ef_cobSandPillar` and starts `Wm_cs_sandpillar01/02` on `+0x260`/`+0x388`. |
| `0x80779770` | 176 | `fn_80779770` | `fn_80777d20` with the holder at `+0x1A4`. |
| `0x80779E40` | 176 | `fn_80779e40` | `fn_80777d20` with the holder at `+0x1AC` (duplicate body). |

## Evidence and checks

- Every body was re-read from the raw words with
  `ppc159.js work/rels/elf/d_basesNP.elf <addr>`; Ghidra's truncated exports
  were used only as an index.
- `0x80776C80` has no `r4`/`r5` integer parameters: the three `fmr` moves at
  `0x80776CA0..0x80776CA8` show `f1`/`f2`/`f3` are the real arguments and
  Ghidra's `param_4`/`param_5` are its lost float registers.
- `0x80777D20`, `0x80779770` and `0x80779E40` share one shape: stack slots
  `0x8/0xC/0x10` receive the halfwords from `+0x100/+0x102/+0x104` and are the
  three pointer arguments of `0x8016EF50`; the three floats at
  `0x1C/0x20/0x24` are the lost arguments of `0x801C0BD0`.
- `0x807787E0` folds to `height <= limit` (`fcmpo` + `cror` + `bc 4,2` at
  `0x80778828..0x80778830`); the `unaff_GQR0` head/tail in the export is the
  documented GQR0 spill noise. The `+0x4B0` state call is the load-and-call
  form because that object's vtable pointer lives at offset 0.
- Constants read from the ELF: `0x807CF51C = -320.0`, `0x807CF528 = 0.5`,
  `0x807CF534 = 2.0`, `0x807CF564 = 100` (u32), `0x807CF57C = 350.0`,
  `0x807CF580 = 1.0`.
- Strings read from the ELF: `0x80817830 "cobSandPillar"`,
  `0x80817848 "ef_cobSandPillar"`, `0x8081785C "Wm_cs_sandpillar01"`,
  `0x80817870 "Wm_cs_sandpillar02"`.
- State descriptor `0x808329A0` is the cell registered under
  `daWmSandPillar_c::StateID_BottomWait` by the translated
  `register_state_desc` block in `bases.rs`.
- `fn_80777e10` and `fn_80777f60` already have Rust bodies in `bases.rs`, so
  they are called directly instead of through the ABI helpers.

## Deferred

None.

## Files

- `work/wave190/laneB_block.rs`
- `ledger/agents/wave190-laneb.csv`
- `docs/COVERAGE_WAVE190_LANEB.md`
