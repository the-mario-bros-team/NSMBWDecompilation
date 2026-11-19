# Coverage - Wave 117 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 117 pass over the two remaining complete exports of
the `0x80280000..0x8028FFFF` bucket in the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave117-gx-band.csv` (two rows).

The bucket held 158 entries, four of them untranslated, when this pass started.
Two are landed here; the other two are the selectors whose export drops control
flow and are documented under "Deliberate skips" with the concrete state of
their evidence. The lane as a whole now has 335 translated entries and ten open
entries (`0x8028F860`, `0x8028FEA0`, the four `0x8029xxxx` entries of
`docs/COVERAGE_WAVE115_GX_BAND.md` and the four `0x802Axxxx` entries listed in
the module header).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 2 | 8,804 | 0 | 2 |

## What this pass covers

- **The byte keyframe sampler `0x80289410`** (4,792 bytes, `medium`). The byte
  member of the keyframe-sampler family: one payload byte per channel, ramp
  records made of `(value, delta)` byte pairs, and a blend that runs entirely in
  16.16 fixed point. It shares the five-arm frame resolver, the key table and
  the ramp hash with the wave-116 body [`fn_8028b6c0`], so it reuses
  [`track_frame_position`] and [`key_hash`].
- **The float keyframe sampler `0x8028A6F0`** (4,012 bytes, `medium`). The float
  twin without the dither stage: the same `(value, delta)` float pairs, but no
  per-record dither byte, no sample negation, and a ramp record index stride of
  `8 * channels` with no `+4` term, so the wave-116 derivation does not
  transfer unchanged.
- Three new shared helpers:
  - [`band_ramp_record`] - the `record = desc + 0x20 + key * stride + base` /
    wrap-into-`u16[wrap]` selector that both bodies (and the old float body)
    inline once per key;
  - [`band_ramp_byte`] - `value + trunc(delta * (i16)(state >> 16) / 32768)`,
    clamped to `0..=255`;
  - [`band_key_blend_byte`] - the fixed-point twin of [`band_key_blend`].

### Descriptor and payload layout (both bodies)

`desc+0x03` is the channel bitmask; the popcount table at `0x8034F3D8` gives the
number of stored channels and `0x8034F4D8` the highest channel bit (both tables
were re-read from `work/dol/main.elf`: `0,1,1,2,1,2,2,3,...` and
`0,1,2,2,4,4,4,4,8,...`). `+0x04` flags (`0x20` explicit step, `0x40`
ping-pong, `0x80` reversed, `0x04` "do not advance the ramp noise"), `+0x05`
step, `+0x06` track salt, `+0x08` key count, `+0x0C` base offset, `+0x10`
wrap-base offset, `+0x20` key count reused as the search bound, `+0x24` key
table.

| | `0x80289410` | `0x8028A6F0` |
|---|---|---|
| entry stride | `(channels + 13) & ~1` | `4 * channels + 0x0C` |
| payload at entry | one `u8` per channel at `+0x0C` | one `f32` per channel at `+0x0C` |
| ramp index stride | `2 * channels` | `8 * channels` |
| first pair | `record + 4` | `record + 4` |
| sample | `value + trunc(delta * (i16)(state>>16) / 32768)`, clamp `0..255` | `value + delta * (f32)(state>>16)` |
| dither byte / negation | none | none (unlike `0x8028B6C0`) |
| LCG step | once per channel slot | once per **selected** channel |
| blend | fixed-point 16.16, byte domain | float, [`band_key_blend`] |

### Details worth recording

- **The `0xAB` tag.** Both bodies start with `lbz r0,0(r3); cmpli r0,0xAB`:
  when `desc[0] == 0xAB` they call the plain sampler instead (`0x8028DC70` for
  the byte track, `0x8028DFC0` for the float track) and return. The meaning of
  the tag is `unknown:`; only the dispatch is reproduced.
- **Fixed-point `t` in the byte body.** The blend position is
  `(u32)(65536.0 * (position - lower)) / (upper - lower)` (`fmuls`, `fctiwz`,
  `divwu`), and every curve arm consumes that `u32` directly. The linear arm is
  `from + ((t * (to - from)) >> 16)` with unsigned wrap and a final `& 0xFF`;
  the cubic arm clamps to `0..=255` while the smoothstep arm truncates with
  `fctiwz` and stores the low byte, exactly as the raw stream does.
- **LCG placement.** `0x8028A6F0` advances the record LCG inside the
  `mask & bit` branch (one step per selected channel, checked at
  `0x8028AD00`), while `0x80289410` advances it at the merge point of every
  channel slot (`0x80289A2C`). The byte body's "both keys" arm advances the two
  states after their own samples, which is equivalent to one step per slot.
- **Curve bytes.** In every blend arm the curve byte comes from the **first**
  key of the segment (`entry + 4`), matching [`fn_8028b6c0`]; the mid key's
  payload pointer is `entry + 0x0C` and only advances for selected channels.
- **`0x80289410`'s tail.** `functions.tsv` reports 4,792 bytes, i.e. the body
  ends at `0x8028A6C8`, but the raw stream continues into the epilogue at
  `0x8028A6D0..0x8028A6F0` and the next entry starts at `0x8028A6F0`. Ghidra's
  warning "Removing unreachable block (ram,0x8028A6D4)" names exactly that
  block, so the epilogue was read from `work/dol/main.elf`,
  not from the export.
- **Constants** (read from `work/dol/main.elf` with `r2 = 0x80433080`):
  `0x8042EBE8` = `0.0`, `0x8042EBF0` = `2^52`, `0x8042EBF8` =
  `0x4330000080000000`, `0x8042EC00` = `1.0`, `0x8042EC04` = `FLT_EPSILON`,
  `0x8042EC08` = `65536.0`, `0x8042EC0C` = `3.0`, `0x8042EC10` = `2.0`,
  `0x8042EC14` = `1.5`, `0x8042EC18` = `-2.0`, `0x8042EC1C` = `255.0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80289410` | `fn_80289410` | 4,792 | medium |
| `0x8028A6F0` | `fn_8028a6f0` | 4,012 | medium |

## Deliberate skips

Two entries of the bucket stay untranslated. Both were re-read from
`functions.tsv` and from their export before being left open; the defect is in
the export, not in the amount of budget.

| address | bytes | export lines | why |
|---|---:|---:|---|
| `0x8028F860` | 1,600 | 1,458 | Selector `4` of the shape dispatcher. The export references ten entry registers it never defines (`in_f27..in_f31`, `in_ps27_1..in_ps31_1`; 35 textual references) and drops 53 blocks, so the quadratic's root-pair acceptance and the tangent-frame tail cannot be read without inventing the frame layout. |
| `0x8028FEA0` | 2,024 | 1,737 | Selector `5`. Same defect, larger: 63 dropped blocks and the same ten undefined entry registers, on top of a per-plane root solve whose smallest-`t` bookkeeping lives entirely in the dropped arms. |

Neither body can be written from the shipped export without inventing arithmetic
or register values, so both are left open rather than guessed.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 8,804 bytes:
  4,792 + 4,012).
- Every evidence file exists at the cited path:
  `work/ghidra/main-gb/export/80289410_FUN_80289410.c` and
  `work/ghidra/main-gb/export/8028a6f0_FUN_8028a6f0.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs`; the module now holds 335 translated entries and the
  new section also adds the helpers `band_ramp_record`, `band_ramp_byte` and
  `band_key_blend_byte`.
- Neither address was in `ledger/functions.csv` before the pass (re-read for the
  whole band: the only open `0x80280000` entries were these two plus
  `0x8028F860`/`0x8028FEA0`).
- `node tools/check-lane-csv.mjs wave117` reports `OK` (the lane CSV has 2 rows).
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings at lines
  23297/23344/23442 belong to the wave-107 body `fn_8028cee0` and predate this
  pass.
- The instruction-level claims (the `0xAB` dispatch, the `2*channels` /
  `8*channels` ramp strides, the `srawi 15` + `addze` division, the `divwu`
  fixed-point `t`, the `fctiwz` smoothstep truncation, the cubic clamp and the
  LCG placement inside or outside the mask branch) were checked against the raw
  big-endian words in `work/dol/main.elf`, not only against the export.
