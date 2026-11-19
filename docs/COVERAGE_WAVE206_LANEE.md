# Coverage - Wave 206, lane E (`ui::hud`, `0x80033D40..0x80034A90`)

Ten functions of `src/ui/hud.rs` translated from `sys/main.dol` in wave 206,
lane E. All ten have a faithful Rust body in `work/wave206/laneE_block.rs`
(appended to `src/ui/hud.rs` on merge); nothing in this lane is deferred.

## Scope

| address | size | name | confidence | role |
|---|---|---|---|---|
| `0x80033D40` | 76 | `fn_80033d40` | high | post positional sound `0x1FE` at the actor position |
| `0x80033F10` | 336 | `fn_80033f10` | high | hit reaction: `Wm_en_hit`, sound `0x17B`, fill the `+0x414` record |
| `0x80034060` | 140 | `fn_80034060` | high | install a three-word state descriptor at `+0x74C` and restart the state word |
| `0x80034140` | 276 | `fn_80034140` | high | two-state `+0x74C` update: arm, then the `+0x76C` sine pulse |
| `0x80034260` | 132 | `fn_80034260` | high | bind a clip to `+0x584`, blend it, restore the scale |
| `0x800342F0` | 144 | `fn_800342f0` | high | bind a clip to `+0x5C0`, virtual `+0x18`, two weights |
| `0x80034380` | 204 | `fn_80034380` | high | hit side from `0x80064A30`, name query, `+0x102` angle limit |
| `0x800344C0` | 92 | `fn_800344c0` | high | forward `{+0x73C, +0x734, +0x738}` to owner virtual `+0x408` |
| `0x80034840` | 104 | `fn_80034840` | high | play the `damage` pair, zero `+0xE8..+0xF0`/`+0x102`, set `+0x390` |
| `0x80034A90` | 104 | `fn_80034a90` | high | byte-identical twin of `0x80034840` |

Total: 1,608 bytes, 10/10 translated, 0 deferred.

## Size checks

Every `functions.tsv` size was verified against the prologue and the terminal
`blr`; all ten match, so this lane needs no prologue-artefact correction for the
`# Original` lines. The ten extents are 76 (`0x80033D40..0x80033D8C`, one
alignment word after the `blr`), 336 (`..0x8003405C`), 140 (`..0x800340E8`), 276
(`..0x80034250`), 132 (`..0x800342E0`), 144 (`..0x8003437C`), 204
(`..0x80034448`), 92 (`..0x80034518`), 104 (`..0x800348A4`) and 104
(`..0x80034AF4`).

Note: the wave-206 lane JSON that seeded this wave carries `136` for
`0x80034140` and `16` for `0x80034260`. Both are stale/prologue artefacts - the
disassembly shows the real extents 276 and 132, which is what `functions.tsv`
reports and what the `# Original` lines use.

## What the cluster is

The run sits directly after the wave-205 lane bodies (`..0x8002EF5C`) and uses
the same conventions as the neighbouring `dE_*` hit-reaction bodies already in
this module (`fn_80024720`, `fn_8002be60`, `fn_8002fa50`, `fn_80030140`):

- **Hit record `+0x414`.** `0x80033F10` fills the same fourteen-word record as
  the wave-204 `fn_80024720`: `+0x414` is a side-selected colour word from
  `0x8042C1B0` (loaded `lwzx`, stored `stfs`, so it is a bit-reinterpreted
  `f32`), `+0x418..+0x420` are the SDA2 floats `3.9f` / `-4.0f` / `-0.24375f`
  (`r2 - 0x6DAC`, `-0x6DA8`, `-0x6DB0`, already named `F32_SDA2_6DAC` and
  friends), `+0x424..+0x42C` are the three words of the `0x802EE6A8`
  descriptor (`0x80357BF4`, `-1`, `-1`), `+0x430` is the side, `+0x431` the
  `0x802EE6B5` byte (`0xFF`) and `+0x432` is `1`.
- **Side predicate.** The shipped sequence is `fsubs` (`+0xAC - +0xB8`),
  `fcmpo` against SDA2 `0.0f`, `mfcr`, `rlwinm` (extract `LT`), `cntlzw`,
  `rlwinm`; the result is `1` when the difference is *not* less than `0.0f`,
  including the unordered case. Reconstructed as `!(a < b)` so the NaN
  behaviour survives.
- **State record `+0x74C`.** `0x80034060` installs `{word0, word1, word2}` at
  `+0x74C`/`+0x750`/`+0x754`, guarded by `0x802DCB90` (a three-word "any
  non-zero" predicate) and followed by `0x802DCBFC`, which takes the record
  address in `r12` - the same convention as every other `0x802DCBFC` call in
  this module, so only `r3` is passed. `0x80034140` then drives the record:
  state `0` arms it (`+0x758 = 1`, `+0x768 = 2`, `+0x76C = 0`, three `0.0f`),
  state `1` advances `+0x76C` by `0x2000` and stores
  `0.5 * (weight * (1.0 + sin(angle/256)))` into all three scale copies. When
  the angle wraps to `0` the count is decremented and, on dropping below `1`,
  the `0x80304DF8` descriptor `{0, -1, 0x80034120}` is installed. Ghidra's
  `ldexpf`/GQR noise there is the `extsh` + `psq_l ... ,1,3` integer-to-float
  conversion of the stacked `s16`; truncated back to 16 bits that value is the
  *old* angle, which is what the body uses.
- **Clip bind pairs.** `0x80034260` is the f64 twin of the wave-205
  `fn_80030140` (`0x8023A0B0` + `0x801651F0` + `0x80169F80`) plus an
  `0x801680E0(1.0f, +0x584)` scale restore; the prologue's `stfd`/`psq_st`
  proves the blend is 64-bit. `0x800342f0` is the `+0x5C0` counterpart
  (`0x8023A200` + `0x80166BD0` + virtual `+0x18` + `0x80166EF0` /
  `0x80166F10`). In both, the object is `r3`, the clip name is the forwarded
  `r4` and the flag is `r5`; Ghidra shifts these because it folds `r3`/`r4`
  into a 64-bit `param_1`.
- **Side lookup.** `0x80034380` calls `0x80064A30` with the *other* object's
  `+0xAC + +0xD0` first and this object's second. The "non-zero" guard is
  `neg` / `or` / `rlwinm. r0,r0,0,31,31` (sign bit of `-x | x`). The `+0x102`
  limit comes from `0x8042C1A4` (`{8192, -8192, 1024, -1024}`) indexed by
  `+0x348`.
- **Owner forwarding.** `0x800344c0` resolves `+0x744` through `0x8005F900`
  and calls virtual `+0x408` with the unusual word order `+0x73C, +0x734,
  +0x738`.
- **Clip entry points.** `0x80034840` and `0x80034A90` are word-for-word
  identical: the clip name is `SDA_BASE - 0x7DC8`, i.e. the string `"damage"`
  at `0x804278B8`, the blend is an `lfs` of `0.0f`, and the tail zeroes
  `+0xE8..+0xF0` and `+0x102`, then sets `+0x390 = 1`.
- **Omitted stores.** `0x80033F10`'s eight trailing stores
  (`0x80034028..0x80034050`) only re-copy live values into its own dead stack
  slots; the same is true of the two dead position copies at `0x10(r1)` /
  `0x14(r1)`. They are omitted and noted in the doc block.

## New named constants

`FN_800954D0_E`, `FN_802DCB90_E`, `FN_80166F10_E`, `F32_SDA2_7B74_E`,
`F32_SDA2_7B70_E`, `F32_SDA2_7B6C_E`, `F32_SDA2_7B68_E`, `F32_SDA2_7B64_E`,
`F32_SDA2_7B60_E`, `DAT_802EE6A8_E`, `DAT_80304DF8_E`, `S_DAMAGE_E`,
`DAT_8035362C_E` and `DAT_803535EC_E`. Every pre-existing constant
(`G_SDA_5924_F`, `FN_8006A4B0_F`, `FN_80197F00_F`, `FN_802DCBFC`,
`FN_80237BD0_A`, `FN_8023A0B0`, `FN_8023A200`, `FN_801651F0`, `FN_80169F80_F`,
`FN_801680E0`, `FN_80166BD0`, `FN_80166EF0`, `FN_80064A30`, `FN_8005F900`,
`TABLE_HIT_COLOURS`, `TABLE_ANGLE_LIMITS`, `F32_SDA2_6DAC`, `F32_SDA2_6DA8`,
`F32_SDA2_6DB0`, `SDA_BASE`, `SDA2_BASE`) is reused unchanged, as are the
host-side helpers `call_f32_1u`, `call_f32_2u`, `call_2f_2u`,
`call_f32_f32_a`, `w205f_call_f64_u32x2`, `p32` and `vtable_method`.

## Verification

The block was appended to a scratch copy of `src/` and `Cargo.toml`/
`Cargo.lock` in `%TEMP%\opencode\w206E`; `cargo check --lib` exits 0
(`CARGO_EXIT=0`). The only diagnostics are the 23 pre-existing warnings (the
function-item-to-integer casts in `src/ui/menu.rs` and the existing warning
sites); nothing comes from this lane.
