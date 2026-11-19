# Coverage - Wave 107 (UI: `wipeCircle` follow, HUD sprite records, `daikonbou`)

Pass 14 of the `ui::hud` lane. After wave 101 the lane's 64 KiB bucket
`0x80010000` still had seven untranslated entries. This pass lands the six that
could be reconstructed honestly and leaves one documented skip, so the bucket is
now 252 translated of 253 entries.

## How to read this file

Every row maps one Ghidra function to one `pub unsafe fn fn_<addr>` body in
`src/ui/hud.rs`. `address` is the link-time address in `sys/main.dol`, `bytes`
is the size recorded in `work/ghidra/main-gb/export/functions.tsv`, and
`confidence` matches the `# Confidence` line of the body's doc block. The bodies
are appended under the `Wave 107` section marker at the end of the file.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 6 | 1,828 | 0 | 6 |

## What this pass covers

- **`wipeCircle` per-frame follow, `0x8001B620`.** The body zeroes the pane
  translation `+0x2C..+0x34`, then, behind the pending byte `+0x1C1`, either
  copies the stored `+0x1B4`/`+0x1B8` pair or reads the current player's
  `+0xAC`/`+0xB0` position, lifts it by the 4x7 ramp table at `0x803525E8`,
  transforms it through `0x800B33D0` and writes it back. It fills that ramp
  table once behind `SDA_BASE - 0x5A4C` from two SDA2 source blocks
  (`-0x6BE0`, 14 words; `-0x6B70`, 7 words, duplicated into the fourth row) and
  selects the row/column through `0x80354E60`/`0x80354E70`. Ghidra renders the
  table loads with the same `ldexpf`/`unaff_GQR0` noise wave 90 documented for
  [`fn_80176410`]; the scalar form of the load is reproduced and labelled.
- **Hit query and its slot body, `0x8001E6F0`/`0x8001E790`.** The first body
  asks `0x80075270` with the scene's `+0xAC` and `+0xB0 + SDA2-0x7E34`, and on
  a `1`/`2` answer forwards `[+0xAC, result, SDA2-0x7E30]` to virtual slot
  `+0xC8`. The second body is that slot: its signature `(f32, scene, float*)`
  matches the call site exactly, which is why both are recorded together. It
  builds the nine-float stack record (six copies of the scale, then the three
  source floats), calls `0x800929f0` with the record base and its `+0x18`
  vector, copies the vector into an eight-byte record for `0x80198c30`
  (id `0x282`) and plays it through `0x80078410`.
- **Twelve-slot release pass, `0x8001F8A0`.** The release counterpart of the
  insert/remove/probe bodies wave 96 translated at `0x8001F6C0`, `0x8001F740`
  and `0x8001F800`: it walks the same twelve slots at `+0x0C..+0x38` and calls
  `0x80020100` on each non-null entry. Ghidra prints the call with no operands;
  `0x80020100`'s own export takes one object pointer and the slot is the only
  live operand at the call site, so it is passed as `r3`.
- **`daikonbou` model builder, `0x8001FA70`.** Loads
  `g3d_daikonbou.brres` from the archive root at `SDA_BASE - 0x5648`, binds the
  model into `+0x544`, the two animation records into `+0x588`/`+0x5B8`, applies
  the scale at `SDA2-0x7E28` (or `SDA2-0x7E18` when bit 17 of `+0x04` is clear)
  and detaches the buffer at `+0x524`. The four `(double)` operands of
  `0x80169F80`, `0x80166EF0` and `0x80167B20` are single-precision `f1`/`f2`
  arguments, matching the existing `call_f64_u32` precedent in this file.
- **Sprite-record copy, `0x8001FD40`.** Copies the `f32` position triple at
  `+0xAC..+0xB4` and the three `u16` angles at `+0x100..+0x104`, offers them to
  virtual slot `+0x298`, normalises the angles in place with `0x80064AF0`,
  emits the position through `0x801C0BD0`, then writes the three angles into the
  sprite record at `+0x7C` in reverse order (`+0x104`, `+0x102`, `+0x100`) and
  rebinds `+0x544`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8001B620` | `fn_8001b620` | 684 | medium |
| `0x8001E6F0` | `fn_8001e6f0` | 160 | medium |
| `0x8001E790` | `fn_8001e790` | 188 | medium |
| `0x8001F8A0` | `fn_8001f8a0` | 84 | medium |
| `0x8001FA70` | `fn_8001fa70` | 468 | medium |
| `0x8001FD40` | `fn_8001fd40` | 244 | medium |

## Deliberate skips

- **`0x800129A0` (6,204 bytes).** The static initialiser of the course-select
  guide state table. It is 700 lines of `DAT_802FE4xx`/`DAT_8035xxxx` stores and
  `0x802DC780` registrations built from the `0x802FEB48..0x802FEFC8` data block,
  i.e. course-select state data rather than a HUD scene. Wave 101 deferred it
  for the same reason and this pass keeps that decision: transcribing the whole
  data block into `platform::write_u32` calls would add ~600 lines of literal
  data movement with no reconstructible control flow, and the risk of a
  transcription slip is high. It is the only untranslated entry left in
  `0x80010000`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/...` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/ui/hud.rs`.
- No address was already in `ledger/functions.csv`; the CSV validator checks this
  live against the merged ledger.
- `cargo check --message-format=short` reports no error and no warning for
  `src/ui/hud.rs`.
- `node tools/check-lane-csv.mjs wave107` prints `OK` for
  `ledger/agents/wave107-hud.csv`; problems it reports for other lanes' files
  are out of scope for this lane.
