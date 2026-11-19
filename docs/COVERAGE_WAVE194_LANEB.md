# Wave 194 lane B - `game::bases`

Ten bodies decoded from `d_basesNP`, all in the `0x8073`/`0x8074` band. Every
body was checked against the raw words; none are deferred.

| address | size | body |
| --- | --- | --- |
| `0x807382F0` | 256 | `daTorchiLight_c` collision-box init: halves `+0x464`/`+0x468` into `+0x164..+0x170`, rebuilds the records with `fn_80738580`, installs the `0x964` touch triple |
| `0x80738DB0` | 248 | child-list side test over `+0x4C0`, kind byte `+0x38C` in `{1,2}`, x compare against `+0xAC`, latches `+0x10D4` bits `0x40`/`0x20` |
| `0x8073AF20` | 256 | transform rebuild with `y += 8.0`, rotation install order `+0x102`, `+0x100`, `+0x104`, then `+0x4A4` virtual `+0x10` |
| `0x8073C730` | 220 | the same rebuild without the `8.0` lift and without the `+0x4A4` virtual |
| `0x8073F810` | 204 | scans `node01..node16` at `0x807CDF28` against `blob + 0x74C`, applies `entry - previous` to the `*out` transform |
| `0x80740A60` | 204 | identical scan for `0x807CDFF0` / `blob + 0x7E0` |
| `0x807433A0` | 224 | five unrolled `+0x30`-chain `vtable+0xC` predicate checks, then recursion |
| `0x807436C0` | 204 | `daTorchiLight_c` constructor tail: `fn_80743790`, `fn_80743920`, nine float stores, clip `0x80831548` |
| `0x80743A20` | 196 | flame counter `+0x5EC`, `900`-step U16 sample, `1/256` scale, `130.0` radius pane at `+0x5D4`, `+0x5D0` virtual `+0xC` |
| `0x80745C40` | 204 | `+0x3B8` interface type-filtered by `0x802F8FF0`, `0x8025C660`/`0x8025C470` scratch, `0x801C0C60` matrix, `0x8023CFA0` |

## Deferred

None.

## Decoding notes

* `0x807382F0` and `0x80738DB0` were left untranslated by wave 101 as
  "paired-single" and "`0x802DCD74` frame" bodies. Both were re-examined here:
  - `0x807382F0` contains **no** `psq_l`/`psq_st`; the `unaff_GQR0`/`ldexpf`
    guards in the export are the GQR0 compiler noise the brief calls out, and
    the body is a plain `lfs`/`fadds`/`fmuls` box builder.
  - `0x80738DB0`'s `0x802DCD74`/`0x802DCDC0` pair is `_savegpr_25`/
    `_restgpr_25`: `r3` survives, so the function returns `r30` as `u32`.
* `0x8073F810`'s `0x802DCD68` is likewise a `_savegpr` helper; `r3 = r25` is
  the original `this`, so `*(this + 4)` is the table block, not a helper
  result.
* The `0x80743A20` GQR3 `psq_l …,1,3` is the unsigned-16-bit sample path, so
  the "dequantise" is `((counter*900) & 0xFFFF) as u16 as f32` and the
  `CONCAT22`/`ldexpf` in the export is noise. `0x80237BD0` is a single
  precision scalar function (`f1 -> f1`), called through `call_f32_f32`.
* `0x807436C0`: Ghidra prints `_DAT_8042BB0C`/`_DAT_8042BB08`, but the raw
  `lis r3,0x8043`/`lis r5,0x8043` give `0x8043BB0C`/`0x8043BB08`; both are
  main-executable floats, so they stay `platform::read_f32` reads. The four
  `stfs` into the parameter-save area are never reloaded and are dead scratch.
* `0x80745C40` is byte-identical in shape to the already translated
  `fn_806caad0`, which supplied the `0x8025C660`/`0x8025C470`/`0x801C0C60`/
  `0x8023CFA0` argument order.
* `0x8073F810`/`0x80740A60` re-use the `0x801C0500(src, dst)` copy order
  established by `fn_806caad0`'s neighbours.
* No `src/game/bases.rs` edit was made; the bodies live in
  `work/wave194/laneB_block.rs`.
