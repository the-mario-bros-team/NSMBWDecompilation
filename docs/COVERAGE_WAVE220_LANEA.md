# Wave 220, lane A - `0x802C4FB0..0x802C83A0` (`resource::archive`)

Ten bodies from the `0x802C0000` sprite/layout band of
`src/resource/archive.rs`: the four-cell re-layout pass that stamps one freshly
created child pane, the 1020-byte interpolator that rebuilds a `0x60`-byte draw
record between two sources, two constructors, the constant `0` mode query, the
per-entry sprite uploader, the two-way insert dispatcher, the default-quadrant
float getter, the texture/scale binder, and a one-call forwarder.

All ten are `translated`; nothing is deferred.

Block file: `work/wave220/laneA_block.rs`
Ledger: `ledger/agents/wave220-laneA.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x802C4FB0` | 248 | `0x802C4FB0..0x802C50A8` (248) | [`fn_802c4fb0`] | high | create one child pane and stamp it |
| `0x802C52A0` | 1020 | `0x802C52A0..0x802C569C` (1020) | [`fn_802c52a0`] | high | lerp a `0x60`-byte draw record |
| `0x802C56D0` | 132 | `0x802C56D0..0x802C5754` (132) | [`fn_802c56d0`] | high | three-table object constructor |
| `0x802C6030` | 8 | `0x802C6030..0x802C6038` (8) | [`fn_802c6030`] | high | constant mode query (`0`) |
| `0x802C6080` | 72 | `0x802C6080..0x802C60C8` (72) | [`fn_802c6080`] | high | tint + four-float scale reset |
| `0x802C6660` | 132 | `0x802C6660..0x802C66E4` (132) | [`fn_802c6660`] | high | upload one entry per source record |
| `0x802C6D80` | 76 | `0x802C6D80..0x802C6DCC` (76) | [`fn_802c6d80`] | high | empty/populated insert dispatch |
| `0x802C6DF0` | 44 | `0x802C6DF0..0x802C6E1C` (44) | [`fn_802c6df0`] | high | read the default quadrant floats |
| `0x802C7B70` | 256 | `0x802C7B70..0x802C7C70` (256) | [`fn_802c7b70`] | medium | bind texture and three scales |
| `0x802C83A0` | 48 | `0x802C83A0..0x802C83D0` (48) | [`fn_802c83a0`] | high | one-call forwarder returning `this` |

Every `functions.tsv` size was re-checked against `work/dol/main.elf` from the
prologue down to the closing `bclr`. In this lane **all ten tsv values are
exact**: the `bclr` of each body sits at `start + size - 4`, from
`0x802C50A4` (`0x802C4FB0`) through `0x802C83CC` (`0x802C83A0`). Where the next
listed entry does not abut, the bytes behind the `bclr` are the alignment zero
words the tsv does not count: `+4` after `0x802C52A0` (next entry
`0x802C56D0`), `+4` after `0x802C6660`, and `+4` after `0x802C6D80`
(`0x802C6DD0`, the quadrant initialiser, is not in this lane).

## Byte weight

248 + 1020 + 132 + 8 + 72 + 132 + 76 + 44 + 256 + 48 = **2,036 bytes**.

## Evidence notes

### `0x802C4FB0` - what the flag writes and the shared block mean

`0x802D3700(param_4, param_5, 6)` is the pane factory the wave-219 lane A
section already documented (it also names `0x802C02C0` as the texture stamp).
The new pane gets `+0x18 |= 8` unconditionally, then
`+0x24`/`+0x28`/`+0x2A` from the three `r2` words at `-0x3E80`/`-0x3E7C`/
`-0x3E7A` - the same `0x8042F200`/`0x8042F204`/`0x8042F206` trio the earlier
lane reads, so its constants are reused rather than redefined.

`0x802D25F0(r13 - 0x4734)` is only used as a null test: on `0` the body sets
bits `1`/`2`/`4` of the same flag byte and copies the four bytes at
`r13 - 0x4734` into `+0x19..+0x1C`. That reads as "the shared tint block did
not exist yet, so install the defaults", hence `// inferred:` for the role, but
the flags, the four byte loads and the call argument are literal.

The tail is `bl 0x802D2F10`, and `0x802D2F10` is a bare `b 0x801C76C0`
(the image word is `4BEF47B0`, `AA = LK = 0`) followed by three alignment
words, so the flush is modelled as one call through the thunk address, which is
what the original executes.

### `0x802C52A0` - the record layout is source-order, the export is not

The export lists the ten floats as `local_60`, `local_5c`, `local_58`,
`local_54`, `local_50`, `local_4c`, `local_48`, `local_44`, `local_40`,
`local_3c` but assigns them from `param_3[6]`, `[7]`, `[8]`, `[9]`, `[0xC]`,
`[0xD]`, `[0xE]`, `[0xF]`, `[10]`, `[0xB]` in that written order. The
disassembly is what settles it: the stores run `stfs` to `0x20`/`0x24`/`0x28`/
`0x2C`/`0x30`/`0x34`/`0x38`/`0x3C` relative to `r1`, i.e. record offsets
`0x18..0x3C` in ascending order, and the target offsets in `param_4` are the
same values. So both the source and target float blocks are plain
`0x18 + 4*i` ranges and the Rust loop is an identity map.

The two `u8` blends are the only non-copy byte fields: `0x12` and `0x13` are
stored from `r29` at `r1 + 0x1A` and `r1 + 0x1B` (record offsets `0x12`/`0x13`)
after the `fctiwz` truncation.

The clamp direction is not symmetric, and the export shows it: the failing
`t >= 0` test branches to the *source* store (record keeps `param_3`), while the
failing `t <= 1` test executes `mr r29,r28` first, i.e. it takes
`param_4`'s byte. The float repeats do the same with `fmr f3,f4`. This is the
usual "below 0 stays, above 1 snaps" clamp and is written as
`t < lo { from } else if t > hi { to } else { blend }`.

The `0x43300000`/`xoris 0x8000` pairs are the compiler's `i32 -> f64`
conversions of the two bytes (bias `2^52` at `r2 - 0x3D18`, value confirmed as
`4503599627370496` in the image), not real double arithmetic: the blends
themselves are `fsubs`/`fmuls`/`fadds`, i.e. single precision. The Rust does
the guards in `f64` and the arithmetic in `f32`, as in the wave-219 lane A note.

The final call is `lwz r12,4(r3)` / `lwz r12,0x6C(r12)` / `bcctr` with
`r4 = r1 + 8`, so the record goes to `[owner+0x04]`'s slot `+0x6C` as
`(owner, record)`.

### `0x802C56D0` / `0x802C6080` - the state resets

`0x802C56D0` calls the wave-219 lane F base constructor `0x802C3210` first, then
installs three tables derived from one `lis/addi` pair: `0x803502A8` at `+0x04`,
`0x803502B4` (`+0x0C`) at `+0x10` and `0x803502C4` (`+0x1C`) at `+0x14`. The
word at `+0x24` is the image word at `0x8042F390`, which is `0x000000FF`; it is
read at run time, exactly as the original does.

`0x802C6080` copies the four bytes at `r2 - 0x3E3C` (`0x8042F244` =
`0xFFFFFFFF`, the wave-219 `SDA2_OPAQUE_A`) into `+0x04..+0x07`. The export
types the second source as a word, but `0x802C6084` is `lfs f0,-0x3CD8(r2)` and
the stores are `stfs`, so the four `+0x08`/`+0x0C`/`+0x10`/`+0x14` fields are
floats and the value at `0x8042F3A8` is `0.0f`.

### `0x802C6660` / `0x802C6D80` / `0x802C6DF0`

`0x802C6660` compares `cmp cr0,r30,r0` and branches with `bc 12,0`, i.e. it
leaves the loop when `index >= count`; the entry stride is the literal
`addi r31,r31,32`. The fetched word is spilled to `r1 + 8` and that address is
what `0x802C60D0` receives; it queries that one word only, so forwarding the
original pointer in `0x802C6D80` is equivalent to the export's stack copy.

`0x802C6D80` selects on `*this == 0` and the two callees are the wave-220 lane E
bodies `0x802C6E80` (populated path) and `0x802C6EF0` (empty path).

`0x802C6DF0` is four `lfs`/`stfs` pairs from `r13 - 0x47C8` and `r13 - 0x47C0`,
two floats each. Those globals live in the small-data band and are written by
`0x802C6DD0` (the body immediately before, not in this lane) from `f1..f4`, so
the getter is a plain four-float read; no constant is embedded.

### `0x802C7B70` - payload offset, then three signed scales

The payload pointer is `desc + (desc[0x1C] ? desc[0x1C] : 0x20)`
(`li r0,32` / `lwz r5,0x1C(r4)` / `cmpi` / `mr r0,r5` / `add r4,r4,r0`).

The last argument of `0x801C6D90` is `(desc[0x10]) & 1`: the export writes it
as `-(uint)byte >> 31`, and the disassembly is `lbz r7,0x10(r31)` / `neg` /
`or` / `rlwinm r10,r0,1,31,31`.

The three scales are `int8(desc[0x16]) * 0.125f`, `int8(desc[0x17]) * 0.125f`
and `int16(desc[0x1A]) / 100.0f`. The signed conversions are visible as `extsb`
(XO 954) on the two bytes and `lha` on the halfword; `0x8042F3EC` is `0.125f`
and `0x8042F3F0` is `100.0f` in the image. The bias at `r2 - 0x3C88`
(`0x8042F3F8`) is `2^52`, so the `0x4330`/`xoris` pairs are `i32 -> f64`
conversions again, and each scale is really computed and passed as single
precision promoted to `f64` - the Rust writes `x as f32 * scale` and promotes
at the call.

The second call passes `(f1, f2, f3, draw, desc[0x14], desc[0x15], desc[0x12],
desc[0x11], desc[0x13])`. The confidence is `medium` only because what those
five bytes select is not visible in either the export or the callee's
signature; the offsets, widths, constants and both call shapes are literal.

### New helpers and constants

The block adds two small call helpers (`vcall_slot_6c_220a`, plus the inline
`extern "C"` types for `0x801C6D90`'s eight arguments and `0x801C6FF0`'s three
doubles and six words, because `platform` stops at `call7`) and the two blend
helpers `blend_u8_220a`/`blend_f32_220a`.

Every new constant is `_220A`-suffixed, because the plain `_A` suffix is
already taken by the wave-219 lane A additions in the same file. Existing
definitions are reused instead of redefined: `FN_802D3700_A`,
`FN_802C02C0_A`, `FN_802C3210_E`, `FN_801C6D90`, `FN_801C6FF0`,
`SDA2_ZERO_A`, `SDA2_ONE_A`, `SDA2_OPAQUE_A`, `SDA2_SPRITE_COLOUR_A`,
`SDA2_SPRITE_SIZE_A`, `SDA2_SPRITE_TAIL_A`, `SDA_BASE`, `SDA2_BASE` and the
`platform::read_*`/`write_*`/`call0..call5` family.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to
`%TEMP%\opencode\wave220laneA`, the block was appended to the copy's
`src/resource/archive.rs`, and `cargo check --lib` finished with **exit code 0**
(`Finished dev profile ... in 17.94s`). The 26 warnings are the pre-existing
`ui::menu`/`runtime::bt` function-item casts and parentheses; the appended
block produces none.

## Deferrals

None.
