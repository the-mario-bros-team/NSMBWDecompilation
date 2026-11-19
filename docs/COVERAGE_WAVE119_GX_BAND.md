# Coverage - Wave 119 (GX / render band, `0x802A0000` bucket)

## How to read this file

This page covers the Wave 119 pass over the last open entries of the
`0x802A0000..0x802AFFFF` bucket of the GX/render lane
(`src/render/gx_band.rs`). Addresses are virtual addresses of the unrelocated
`main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave119-gx-band.csv` (two rows).

The bucket held four untranslated entries when this pass started
(`0x802A3160`, `0x802A3A20`, `0x802A71D0`, `0x802A90C0`). Two are landed here;
the other two are documented under "Deliberate skips" with the evidence that is
still missing for each. The lane as a whole now holds 338 translated entries.

## The finding that re-opened this bucket

Wave 114 skipped all four entries on two grounds: "the export keeps only one of
the three FIFO component writes" and "the lane split is not proved". Both are
export artefacts, not missing information. Re-reading `work/dol/main.elf`
instruction by instruction gives the full per-lane dataflow for the two entries
landed here:

- **The Ghidra `psq_l`/`psq_st` displacement field was re-derived.** The
  encoding is `opcode | Fs | rA | W(1) | I(3) | DQ(12)` with the displacement
  equal to `DQ` (its two low bits are always zero), so `psq_st f31,0x138(r1)`
  follows the `stfd f31,0x130(r1)` of the same save pair. Every
  `psq_l`/`psq_st` operand in both bodies carries `I = 0`, i.e. GQR0, which
  `0x801AB150` pins to plain float; the ladders in the exports are therefore
  dead register spills.
- **The paired-single lanes are a plain 3-float vector blend.** In
  `0x802A3A20` the pairs at `+0x00`, `+0x0C` and `+0x18` are the three points
  `P0`, `P1`, `P2`; the `ps_muls0`/`ps_add` chain computes
  `(w1*P1 + w0*P2) + P0` and the three `stfs` that follow in the raw stream are
  the three position components. Ghidra's C kept only the third component
  (its `+8`/`+0x14`/`+0x20` reads are the `z` lane) and collapsed the other two
  into dead stack spills, which is why the export shows one FIFO write per
  vertex instead of three.
- **`0x802A3160` writes two vertices, not one.** The raw stream ends with three
  `stfs` for the first transformed point (`0x802A352C`), the optional
  `(1.0, f2)` texture pair, three `stfs` for the second point (`0x802A355C`)
  and the optional `(0.0, f2)` pair. The export printed only the first of the
  six position floats.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 2 | 2,080 | 0 | 2 |

## What this pass covers

- **The range-node strip emitter `0x802A3160`** (1,092 bytes, `medium`). Called
  once per node by [`fn_802a2de0`] through the local `draw_range_node`
  trampoline with `(f1, f2, scene, ctx, a, node, callback, vector, b, c)`. A
  null `node` is replaced by the first `u32[desc+0x3C]` list entry whose
  `+0x0C` type word is not 1, walked with the `u16[desc+0x42]` stride. The
  callback supplies a three-float reference; `0x801C1530` crosses it with
  `node+0xCC` (or `ctx+0xC8` when bit 3 of `a` is set) and the result is
  normalised against `FLT_EPSILON` with the `ctx+0xBC..+0xC4` fallback. A
  second, unconditional cross with the reference gives the third axis, which is
  normalised without a fallback and stored to `node+0xCC..+0xD4`. The three
  axes plus `vector` form the 3x4 record matrix; the `+0x9A..+0x9C` byte
  counters advance `node+0x40..+0x48` by `0.024543693` and the transform at
  `node+0xC8` adds its `+0x60/+0x64/+0x68` and provides `+0x58`. A
  `0x802961E0` sample of the `y` phase builds the 3x4 side matrix
  (`cos*cos`, `sin*cos` slots, `1.0`/`0.0` constants at `0x8042EFEC` and
  `0x8042F000`), `0x801C0540` composes the two and `0x801C1160` transforms the
  two caller points. The six position floats and the optional `(1.0, f2)` /
  `(0.0, f2)` pairs go to the FIFO; the caller already issued the `GXBegin`
  whose count covers this vertex pair.
- **The pair blender `0x802A3A20`** (988 bytes, `medium`). Called with
  `(scene, previous, current, param, count, ramp)` by [`fn_802a3e00`],
  [`fn_802a40c0`] and [`fn_802a4400`]. It begins `GX_TRIANGLESTRIP` with
  `2 * count + 2` vertices, then runs `count + 1` iterations. Each iteration
  reads the `{w0, w1}` pair at `ramp + 8*i` and emits two vertices, one per
  0x28-byte record: `(w1 * P1 + w0 * P2) + P0` with `P0/P1/P2` at `+0x00`,
  `+0x0C` and `+0x18`. Bit 0 of `param` adds the texture pair
  `(i / count, f32[+0x24])`, where `i / count` is the `fres` reciprocal plus
  one Newton step (`0x802A3A84..0x802A3A90`); bit 4 emits `previous` before
  `current`, and its absence emits them the other way round, which is the only
  difference between the two raw loops. The shared helper
  `emit_blended_vertex` holds the blend so the transcript has one vertex body.

The two entries are a matched pair: `0x802A3160` is the single-node strip
emitter used by the range drivers, while `0x802A3A20` is the two-record strip
emitter the ring drivers use.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802A3160` | `fn_802a3160` | 1092 | medium |
| `0x802A3A20` | `fn_802a3a20` | 988 | medium |

## Deliberate skips

Two entries of the bucket stay untranslated. Both were re-read from
`functions.tsv`, from their export and from `work/dol/main.elf` before being
left open, and the wave-114 reasons are corrected below. Neither is blocked by
the GQR quantiser: all 197 `psq_l`/`psq_st` operands in `0x802A71D0` and all 26
in `0x802A90C0` carry `I = 0`, so the ladders collapse to plain float.

| address | bytes | insns | `psq_l`/`psq_st` | indirect calls | why it stays open |
|---|---:|---:|---:|---:|---|
| `0x802A71D0` | 2,748 | 687 | 197 | 0 | Three-record bilinear blend whose two loops each write ten FIFO floats per iteration (20 `stfs` to `0xCC008000` in total). The weights are derived from the incoming `f1` (`f1`, `0.5*f1`, `0.5 + w`, `-f1`), but no in-lane caller passes that float: the translated `draw_ring_triple` reaches it through `call8` with word arguments only. A faithful body needs the lane-by-lane transcription of both loops *and* an argument model for `f1` that no caller in this band pins, so it stays open rather than guessing. |
| `0x802A90C0` | 1,396 | 349 | 26 | 6 `bcctr` | Circled in the wave-114 note as an "uninitialised ring index"; that is wrong. The raw stream initialises the index at `0x802A9264..0x802A9278` (`0` with `+1`, or `u16[desc+0x50]` with `-1`) before the first iterator call, and the slot arithmetic is a `mulhw` mod-3 (`0x802A93E0..0x802A943C`, `0x802A94A4..0x802A94E8`). What remains is volume and risk, not a defect: five 0x28-byte stack records indexed by `i*40`, ten-word record copies that overlap the source and destination stack blocks, an offset vector (`ctx+0xA4 + node+0xAC`) added into the copied record, and two mirrored edge paths around four `0x802A71D0` calls. Since `0x802A71D0` is itself still open, the callee contract would also be unverified on both sides. |

### Wave-114 record

`docs/COVERAGE_WAVE114_GX_BAND.md` described `0x802A3160` as having "one
`_DAT_cc008000` write at the tail" and `0x802A3A20` as a "software-pipelined
loop" whose "lane split is not proved". Both statements describe the Ghidra
export, not the binary: the raw streams contain three `stfs` per vertex and
the lane split is a plain vector blend (see "The finding that re-opened this
bucket"). The `psq_l ...,1,0` loads in `0x802A3A20` do leave the second lane
live from the previous instruction, but every live value is lane 0 of a fresh
load, so no pipeline state crosses an iteration.

## Verification

- Every address and byte size matches `functions.tsv` (2 rows, 2,080 bytes;
  the next entries start at `0x802A35B0` and `0x802A3E00`).
- Every evidence file exists at the cited path:
  `work/ghidra/main-gb/export/802a3160_FUN_802a3160.c` and
  `work/ghidra/main-gb/export/802a3a20_FUN_802a3a20.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 338 translated entries).
- Neither address was in `ledger/functions.csv` before the pass (the whole
  `0x802A0000` bucket was re-read: the only open entries were the four listed
  above).
- `node tools/check-lane-csv.mjs wave119` reports `OK`; the gx-band lane CSV
  has two rows.
- `cargo check --message-format=short` reports no error from
  `src/render/gx_band.rs`; the three `unused_assignments` warnings at lines
  23321/23368/23466 belong to the wave-107 body `fn_8028cee0` and predate this
  pass.
- The `psq` inventory was produced by decoding the `W`/`I` fields of every
  `psq_l`/`psq_st` in `0x802A3160..0x802A35A8` and
  `0x802A3A20..0x802A3DFC`; all operands carry `I = 0`, and every one outside
  the save prologue is a stack spill or a record slot.
- Constants were read from `work/dol/main.elf` with `r2 = 0x80433080`:
  `0x8042EFEC` = `1.0f`, `0x8042EFF8` = `FLT_EPSILON` (`1.1920929e-7`),
  `0x8042EFFC` = `0.024543693`, `0x8042F000` = `0.0f`,
  `0x8042F008` = `2^52` as a double, `0x8042F038` = `0.5f`,
  `0x8042F03C` = `-1.0f`.
