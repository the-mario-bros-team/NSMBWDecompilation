# Coverage - Wave 107 (GX / render band, `0x80280000` bucket)

## How to read this file

This page covers the Wave 107 pass over the `0x80280000..0x8028FFFF` bucket of
the GX/render lane (`src/render/gx_band.rs`). Addresses are virtual addresses of
the unrelocated `main.dol` (image base `0x80004000`); byte counts come from
`work/ghidra/main-gb/export/functions.tsv`, never from prose. The
machine-readable mirror is `ledger/agents/wave107-gx-band.csv` (the same three
rows).

`status` is `translated` for all three rows; `confidence` follows each body's
`# Confidence` line (all `medium`, because every callee outside the lane is
reproduced opaquely).

Before this pass the bucket held 158 Ghidra entries, 145 of them already in
`ledger/functions.csv`. The thirteen that were left were re-read in full. Ten
have a live `unaff_GQR0` data path and are listed under "Deliberate skips"; the
other three print `unaff_GQR0` only in the prologue and epilogue, where
`psq_st`/`psq_l` save and restore `f22..f31` and every `ldexpf` result is
discarded. Those three are translated here: three bodies, 7,484 bytes.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/render/gx_band.rs` | `render::gx_band` | 3 | 7,484 | 0 | 3 |

## What this pass covers

- **Texture-grid cell packer** (`0x802821E0`, 2,988 bytes). The per-cell vertex
  builder behind `fn_80281d80`: it turns `object+0xBC`/`+0xC0..+0xD0` into four
  clamped corner weights, projects `(object+0xAC, object+0xB0) + cell[0x34,
  0x38]` either through the rounded-box helper `0x80282040` or directly, samples
  the two ramps `0x802837C0`/`0x802838A0` with the descriptor built from
  `object+0xF8` (`table = mode / 3`, two flag bits), and emits 12 fixed-point
  corner coordinates plus four `(u, v)` pairs, all scaled by `32768.0` and
  saturated through `0x802DCC4C`. The `0x18`-byte table at `+0x34..+0x3C` is
  indexed by the second grid coordinate.
- **Animated draw-state commit** (`0x802850F0`, 1,028 bytes). Takes the render
  manager lock at `0x8027B5F0()+0x354`, returns early when `object+0x114` is
  null, and otherwise fills the draw record: the two scales from `+0x04` and
  `+0x08 * +0x108`, the `0x3F`/`0x40`-biased step bytes `+0x10C..+0x111`
  divided by `63.0`/`127.0`, four corner weights from `+0xD4..+0xE0` gated by
  `+0xBC`, three floats from the bytes at `+0x10E..+0x110` plus one row of
  `0x80265F60`, one `0x18`-byte row per `+0xD8` entry copied through
  `0x802817D0`, and four floats from `0x80265FA0`. The single `f64` argument of
  `0x80268900` is `object+0x18`.
- **Keyframe range walk** (`0x8028CEE0`, 3,468 bytes). The emitter of *all* keys
  between two frames for the older `0x0C`-byte key layout (count at `+0x20`,
  entries at `+0x24`, key time in the first halfword). It resolves the frame
  twice (`index` and `index + 1`), then picks one of four walk arms: forward,
  backward, whole repeat groups, or ping-pong, each starting from a hand-written
  binary search, and calls `0x8028CD70` once per emitted entry. Its resolver
  differs from the already-translated `track_frame_index` in two places: the
  unreversed clamp and the out-of-range tail use the key count instead of
  `count - 1`, and the ping-pong arm reports whether the caller landed exactly
  on a repeat boundary.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802821E0` | `fn_802821e0` | 2988 | medium |
| `0x802850F0` | `fn_802850f0` | 1028 | medium |
| `0x8028CEE0` | `fn_8028cee0` | 3468 | medium |

## Deliberate skips

Ten entries of the bucket stay untranslated after a full read. They fall into
two groups.

1. **Live GQR-quantised emitters** (7 entries): `0x80287C10` (1,908 bytes),
   `0x80288E30` (572), `0x8028E890` (1,172), `0x8028ED30` (1,436),
   `0x8028F2D0` (1,412), `0x8028F860` (1,600), `0x8028FEA0` (2,024).
   In these the `ldexpf` arms are not dead: `0x80287C10` uses
   `lVar6 = ldexpf(...)` and `dVar9 = ldexpf(...)` to re-quantise
   `param_4+0xAC..+0xC0` before `0x801C1160` consumes the packed `local_2C8`
   record; `0x80288E30` scales `local_64`/`local_5C` twice with
   `ldexpf(-(bVar3 & 0x3f))` and stores the result back through the type
   switches on `GQR0 & 7`; the five `0x8028E8xx..0x8028FExx` entries use
   `dVar16 = (double)ldexpf(bVar11 & 0x3f)` and
   `dVar15 = (double)ldexpf(...)` as live multipliers. In every case the stored
   byte width is decided by the GQR type (`4`/`5`/`6`/`7`) and the stored value
   by its scale, neither of which the export records.
2. **Per-key blend samplers** (3 entries): `0x80289410` (4,792 bytes),
   `0x8028A6F0` (4,012), `0x8028B6C0` (4,280). The GQR blocks in these are dead,
   but each body holds three or four copies of a per-channel blend ladder in
   which a key's value is either read inline at `key+0x30` (plain key, mode byte
   `+0x26 == 0`) or pulled from a shared row
   `record+0x20 + *key_u16 * stride*2 + record[0x0C]` (dithered key), and the
   dither seed is a five-term integer hash whose bytes are then chained with a
   cumulative XOR before the `0x343FD`/`0x269EC3` LCG steps. Ghidra renders
   that seed as `CONCAT12`/`CONCAT13` over a `local_78` whose first byte is read
   before it is written, and it reads the row index through
   `*(ushort *)float_ptr`, i.e. the low half of a float. The byte order and the
   row arithmetic therefore cannot be proved from the export alone; guessing
   them would change every blended byte. `0x8028A6F0` is the callee of the
   already-translated `fn_802888e0`, which keeps calling it by address through
   `FN_8028A6F0`.

## Verification

- Every address and byte size matches `functions.tsv` (3 rows, 7,484 bytes:
  2,988 + 1,028 + 3,468).
- Every evidence file exists at the cited path under
  `work/ghidra/main-gb/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/render/gx_band.rs` (the module now holds 315 translated entries).
- No address was already in `ledger/functions.csv`; all three were among the
  thirteen still untranslated in this bucket before the pass.
- `node tools/check-lane-csv.mjs wave107` reports `OK` and lists
  `wave107-gx-band.csv: 3 rows`.
- `cargo check --message-format=short` reports no error and no warning in
  `src/render/gx_band.rs`.
- The `sdata2` constants used by this pass were read from
  `work/dol/main.elf` (`r2 = 0x80433080`):
  `0x8042EB78` = 1.0, `0x8042EB7C` = 0.0, `0x8042EB80` = 63.0,
  `0x8042EB84` = 0.015625, `0x8042EB88` = 127.0, `0x8042EB90`/`0x8042EB98` =
  2^52 as `f64`, `0x8042EB0C` = 0.0, `0x8042EB08` = 1.0, `0x8042EB30` = -1.0,
  `0x8042EB28` = 2.0, `0x8042EB34` = -3.0, `0x8042EB18` = 0.5,
  `0x8042EB38` = 32768.0.
- `0x43300000` is stored where Ghidra prints `local_d4[3] = 176.0`; the two are
  the same word, which confirms the `2^52` int-to-double idiom used by
  `fn_802850f0` and `track_group_frame`.
- The `f64` argument of `0x80283760` was verified from its own export
  (`double FUN_80283760(double)`), so `fn_802821e0` calls it through an
  `extern "C" fn(f64) -> f64` pointer.
