# Wave 195 lane D coverage (`game::bases`)

Ten functions in the `0x806DEFD0..0x80733B00` band were decoded from the raw
PowerPC words of `work/rels/elf/d_basesNP.elf` and checked against the Ghidra
exports. All ten are translated; none are deferred.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Deferred |
|---|---|---:|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 10 | 2828 | 9 | 1 | 0 |

Source of the bodies: `work/wave195/laneD_block.rs`; per-function ledger:
`ledger/agents/wave195-laned.csv`.

## Translated

| Address | Size | Body |
| --- | ---: | --- |
| `0x806DEFD0` | 292 | Re-binds the `lift_spin_kinoko` pane stack (`+0x394`/`+0x474`/`+0x4D4`/`+0x534`/`+0x594`), rebuilds the `+0x5F4` box and re-seeds the `+0x62C..+0x634` offsets. |
| `0x806E2E40` | 272 | Steers the `lift_suisha` plank tilt from the `fn_806e2950` probe angle: `+12`/`-12` steps or the `-angle` reflection clamped at `+/-0x48`. |
| `0x806E8340` | 288 | Picks the rider launch speed from the `0x807CBDE8` table for pane states `1`/`2` (with the `+0x45D` half-scale) and `3` (`+8` word `0x19C`/`0x19D`). |
| `0x806ED340` | 288 | Snap-steps each actor in the `+0x1024..+0x1388` ring until its `+0x80` sample is more than 32 units from the previous one (up to 16 steps). |
| `0x806F1210` | 276 | Resets a tilt controller: zeroes `+0xE8..+0xF0`, sets `+0x728 = 0x10`, stores the two `entry/16.0` rates at `+0x6F4`/`+0x6F8`, flag `0xED00`, then `fn_806f1110`. |
| `0x806F5640` | 292 | Creates the `W7_shiroboss_bg_M` background pane (`g3d/W7_shiroboss_bg_M.brres`), verifies `+0x548` against `0x802F8FF0`, posts `0x30001` and releases the pane blocks. |
| `0x80705600` | 260 | Rebuilds a 48-byte launch matrix from the holder at `+4`: copies the target matrix in, post-multiplies one or two rotations (`+0x3FC`/`+0x404` for mode 1, `+0x400` for mode 2) and copies it back. |
| `0x80707F40` | 272 | Returns whether the horizontal position is exactly zero (`0x80070BA0` result `== 0`), or `1` when `+0x3D4` and `+0x3DC` are both clear. |
| `0x807103D0` | 296 | While `+0x694 < 29`, snaps `+0xAC` through `fn_8070f9c0` against the `0x8082F760` grid and writes `+0xB0 = 0x8082F764 + 16*record[1]`. |
| `0x80733B00` | 292 | Fire-cannon burst: builds the `12.0`/`20.0` offset position from the `0x807CD820`/`0x807CD830`/`0x807CD840` tables, spawns effect `0x15B`, sound `0x2A2` and `Wm_en_burst_ss`, then `fn_80733ec0`. |

## Signature notes

- `fn_806defd0` returns `u32` (`li r3,1` in the epilogue); Ghidra types it
  `undefined4`. `fn_80707f40` returns `u32` through the `cntlzw`/`rlwinm` pair
  and is tested as a boolean by `fn_80708070`.
- `fn_806e8340` takes `r3 = this`, `r4 = other`; `other` is dereferenced at
  `+0x38C` before the `param_2 != 0` test that Ghidra keeps, so the check is
  unreachable in practice and the Rust body keeps it only for literalness.
- `fn_80705600` takes `r3 = this`, `r4 = mode`, `r5 = matrix-pointer pointer`;
  Ghidra prints the matrix pointer as `undefined4 *param_3`.
- `fn_806f1210` reads `+0x73C` unchecked as the index `2 + variant` into the
  six-float stack table; the Rust body uses `get_unchecked`, matching the
  original (the caller must supply `0`/`1`).

## Decoding notes

- `0x80707F40`: the whole `+0x3D4`/`+0x3D8`/`+0x3DC`/`+0x3E0` displacement
  arithmetic writes only stack slots that the raw body never reloads (the final
  loads are `lfs f1,0xAC(r3)`/`lfs f2,0xB0(r3)`), so it is omitted as dead.
- `0x80705600`: the `addic. r4,r1,200` guard is the classic
  "`&local != NULL`" idiom; the export loses the destination argument of the
  first `0x801C0500` call. The raw register pair proves `0x801C0500` is called
  as `(target, local)` first and `(local, target)` last, i.e. the SDK
  `PSMTXCopy(src, dst)` order, while `0x801C0540` is `PSMTXConcat(a, b, ab)`.
- `0x806E2E40`: the `sth`/`local_18[0]` stores are dead; only the first probe
  element is consumed. The reflection test uses the shifted angle `iVar2`, not
  Ghidra's `local_18[0] + 0x4048`.
- `0x806F1210`: the `0x43300000`/`0x80000010` pair minus `0x807CC0C0`
  (`2^52 + 2^31`) folds to `16.0`, so the two stored rates are `entry / 16.0`.
- `0x806ED340`: the `GQR0`/`ldexpf` guards in the export are compiler noise;
  only the `x`/`y` components of the `+0x80` sample are ever compared.

## Deferred

None.

## Caveats

- `fn_807103d0` is `medium`: `0x8082F760`/`0x8082F764`/`0x8082F774` lie outside
  `d_basesNP` (main.dol), so they are read at run time instead of being folded.
  All in-REL constants (`16.0` at `0x807CCA84`, `48` at `0x808063A8`, `29` at
  `0x80805EA0`) are literal.
