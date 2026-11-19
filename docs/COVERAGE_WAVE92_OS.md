# Coverage - Wave 92 (`runtime::os`, `0x80240000` bucket)

## How to read this file

This page records wave 92 of the `main.dol` reconstruction for the
`runtime::os` lane: a pass over the `0x80240000` bucket, which holds 137
inventory entries. 51 of them were already merged when the lane started (the
Wave 9/67/83 passes plus the `runtime::bt` neighbours), 86 were untranslated,
and 26 of those are reconstructed here.

- **Status** - `translated` means every guard, store and call printed in the
  Ghidra export has a counterpart in `src/runtime/os.rs`. All 26 rows are
  `translated`; none is `partial` and none is `stub`.
- **Confidence** - strength of the evidence, not of the Rust. `high` means the
  flow, the offsets and the callee argument shapes are explicit in the export.
  `medium` means the flow is explicit but at least one callee argument list or
  field role is not recoverable and is marked `// unknown:` / `// inferred:` in
  the body. `low` means only the instruction sequence is certain, which happens
  once here: `0x80249D60` reads three of its operands from untyped registers.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; it is the
  source of truth for every row. Byte sizes were re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.
- `f32`/`f64` arguments reach the orientation kernels through the floating-point
  registers. The bodies model those calls with host-side `extern "C"`
  trampolines ([`wave92_kernel_f64x3_ptr`]), the same convention Wave 91 and
  `src/audio/ax.rs` use.

The machine-readable companion is `ledger/agents/wave92-os.csv` (same rows).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 26 | 4,624 | 13 | 12 |

Status split: 26 `translated`, 0 `partial`, 0 `stub`. One `low`.

## What this pass covers

- **Cluster A - the three component stores, `0x802418F0..0x80241DE0`
  (3 bodies).** These sit between the already translated `fn_80241A00` and
  `fn_80241BE0` and complete the component block's write side.
  `0x802418F0` combines flag bits `0x20`/`0x40` into the four ways a 3x4
  destination can be filled (matrix copy through `0x801C0500`, matrix copy plus
  three neutral row tails, clear plus secondary triplet, or clear only).
  `0x80241AA0` is the "set the whole stream" writer: the uniform path sets one
  of three flag patterns and nine neutral words, the general path scales the
  inputs by `sda2(-0x4AD8)`, runs the `0x802382B0` matrix kernel, restores the
  three row-start words and copies the re-read inputs to `+0x10..+0x18`.
  `0x80241DE0` clamps a `double` against `sda2(-0x4AB8)` and the `u16` at
  `record+0x34` (through the `0x43300000` bias at `sda2(-0x4AB0)`), reports the
  object word at `+0x14` masked with `3`, and resolves the companion through
  `0x8023F140` unless that word is negative.
- **Cluster B - record and FIFO helpers, `0x80249470..0x80249DD0` (8 bodies).**
  `0x80249470` and `0x80249720` are the two probes over the halfword array at
  `+0x14` (vtable slot `+0x14` plus four; `& 0xC000` with the leading-zero
  trick). `0x80249A70` and `0x80249B40` are the two destructor/notify forwards
  (vtable `0x8034E6E8` and event `0x10001`). `0x80249CB0` is the one-word GX
  FIFO writer. `0x80249D30` and `0x80249D60` build two more FIFO words.
  `0x80249DD0` is the texture-scale command builder: the exponent loop over the
  six `|m|` magnitudes, the two bounds `sda2(-0x4A1C)`/`sda2(-0x4A18)`, the
  shrink loop at `sda2(-0x4A14)` and the final word
  `(command+8)<<24 | exponent<<18 | mantissa0 | mantissa1<<11`.
- **Cluster C - render-state blocks, `0x8024A070..0x8024A3D0` (5 bodies).**
  Two fixed FIFO pushes (`0x8024A070` literal, `0x8024A0D0` packed from four
  record words), the twelve-word stack record with the `sda2(-0x4A0C)` fill at
  slots 3/7/11 (`0x8024A150`), the per-channel callback install pair
  (`0x8024A1D0`, mode `0`/`1`/`2`) and the eight-step table loader behind it
  (`0x8024A3D0`).
- **Cluster D - region initialiser and channel latch, `0x8024B610`,
  `0x8024B6B0` (2 bodies).** The first clears `0x20` entries of four strided
  arrays (`+0x34`/`+0x634`/`+0xE34`/`+0x1434`); the second latches the
  `0x8023D100` id and raises the dirty byte only on a change.
- **Cluster E - the four state-flush wrappers, `0x8024BEC0..0x8024C8D0`
  (4 bodies).** Three are byte-identical apart from their tail callee
  (`0x8023CB00`, `0x8023CB30`, `0x8023D5B0`) and all share the same packed
  state word, built from the five bytes at `0x803DEF40`; the fourth
  (`0x8024C8D0`) flushes through `0x8023DDD0` first and then re-pushes the five
  bytes through `0x80249CB0`.
- **Cluster F - record tables and the reset switch, `0x8024D140..0x8024D5E0`
  (4 bodies).** Two record accessors (the `0x24`-stride table at `0x803DF174`
  versus the `0x30`-stride copy into `0x803E44C0`; the `0x30`-byte slot compare
  at `0x803DF988`), the twenty-field render-state block copier at
  `0x8024D4F0`, and the ten-bit reset switch at `0x8024D5E0`.

## What the pass adds beyond the raw bodies

1. **`0x80249DD0` closes the Wave 83 loop.** `fn_8024B020` documented that its
   three float triples are handed to `0x80249DD0`, "each of which writes the GX
   FIFO per its own export". The body now shows exactly what is written: the
   command byte is `id + 8`, the exponent occupies bits 22..23 and the two
   matrix components `+0x08`/`+0x18` become the two 11-bit mantissas. That is
   the BP texture-size word, which is why the three triples are a texture
   matrix and not a position.
2. **The render-state block is now readable end to end.** `0x803DEF40..0x44` is
   a five-byte description that the flush wrappers pack into one FIFO word,
   `0x803DEF48` is a two-bit push state plus a "valid" bit, and `0x803DEF4C`
   is the auxiliary word the reset switch clears. `0x8024D5E0` names every
   other global of the same block, so the reset switch doubles as the field map.
3. **The 12-word record layout is pinned.** `0x8024A150` inserts the same
   `sda2(-0x4A0C)` word at slots 3, 7 and 11 of a nine-word payload, which
   matches the `0x30`-byte records the `0x8024B0xx` list wrappers use.
4. **`0x8024D4F0` proves the `0x803E43E8` block size.** Twenty fields are
   copied individually from `0x802DCD58`; two destination halfwords
   (`0x803E43FA`, `0x803E4422`) are deliberately skipped, which is only visible
   field by field and would have been lost in a `memcpy`-style summary.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802418F0` | `fn_802418f0` | 164 | high |
| `0x80241AA0` | `fn_80241aa0` | 320 | medium |
| `0x80241DE0` | `fn_80241de0` | 208 | medium |
| `0x80249470` | `fn_80249470` | 48 | medium |
| `0x80249720` | `fn_80249720` | 28 | high |
| `0x80249A70` | `fn_80249a70` | 76 | high |
| `0x80249B40` | `fn_80249b40` | 88 | medium |
| `0x80249CB0` | `fn_80249cb0` | 128 | high |
| `0x80249D30` | `fn_80249d30` | 48 | medium |
| `0x80249D60` | `fn_80249d60` | 112 | low |
| `0x80249DD0` | `fn_80249dd0` | 668 | medium |
| `0x8024A070` | `fn_8024a070` | 92 | high |
| `0x8024A0D0` | `fn_8024a0d0` | 124 | high |
| `0x8024A150` | `fn_8024a150` | 128 | high |
| `0x8024A1D0` | `fn_8024a1d0` | 248 | medium |
| `0x8024A3D0` | `fn_8024a3d0` | 352 | medium |
| `0x8024B610` | `fn_8024b610` | 152 | medium |
| `0x8024B6B0` | `fn_8024b6b0` | 112 | medium |
| `0x8024BEC0` | `fn_8024bec0` | 168 | high |
| `0x8024BF70` | `fn_8024bf70` | 168 | high |
| `0x8024C020` | `fn_8024c020` | 180 | high |
| `0x8024C8D0` | `fn_8024c8d0` | 144 | medium |
| `0x8024D140` | `fn_8024d140` | 136 | high |
| `0x8024D220` | `fn_8024d220` | 156 | medium |
| `0x8024D4F0` | `fn_8024d4f0` | 212 | high |
| `0x8024D5E0` | `fn_8024d5e0` | 364 | high |

## Deliberate skips

The remaining 60 untranslated addresses in the bucket were left alone for one of
three reasons.

1. **The `0x802DCDxx` two-register return is not provable.** `0x80241C70`,
   `0x80248E00`, `0x80243000`, `0x802430C0`, `0x8024A610`, `0x8024A6E0` and
   `0x8024ABD0` (among others) read one `undefined8` from
   `0x802DCD68`/`0x802DCD70`/`0x802DCD74` and then use `>> 32` as one pointer
   and the low word as another. The export never shows which half the callee
   means as which, so no body was written. These belong with the allocator
   itself, which is still untranslated.
2. **The gather-register kernels are not recoverable.** `0x8024EA30` (192
   bytes) and `0x8024EAF0` (216 bytes) are the two quantised matrix-inverse
   kernels: their bodies read an untyped `GQR0` and branch on its type field
   (`4`/`5`/`6`/`7`) and scale field before every single element load. The
   decompiled C is dominated by `ldexpf` pseudo-calls whose argument is the
   register field, and the store paths are emitted twice. Reconstructing them
   would mean inventing the gather-register setup, so they were skipped.
3. **Large state machines with unrecovered helper tables.** The 2,660-byte
   `0x802405B0`, the 1,124-byte `0x80241690`, and the 1,000-byte `0x80241EB0`
   are the front of the bucket and each dispatch through a table that is not
   yet documented. Like the seven large bodies Wave 91 deferred, they are better
   handled together with their tables than padded with guesses.

An additional group was skipped because the address is already translated:
`0x80249C60` and `0x80249C90` (the flag mask pair), `0x8024D390` (the
`0x44`-stride table accessor) and `0x80243300`/`0x80247A50` are all in
`ledger/functions.csv` under `runtime::bt`. They are **not** duplicated here;
`0x80249C60` keeps being called by address from the Wave 83 cell-table clears
([`fn_80244ad0`]) and from [`fn_80242b50`] above, exactly as those entries
already did.

Every skipped address stays untranslated in `ledger/functions.csv` and can be
revisited together with the `0x802DCDxx` allocator family.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv` (checked before writing;
  the five `runtime::bt` neighbours listed above were excluded for that reason).
- `cargo check --message-format=short` reports no error or warning for
  `src/runtime/os.rs`; the remaining build errors belong to other lanes'
  `src/game/player.rs`, `src/game/worldmap.rs` and `src/game/bases.rs`.
- `node tools/check-lane-csv.mjs wave92` prints `OK` with
  `wave92-os.csv: 26 rows`.
