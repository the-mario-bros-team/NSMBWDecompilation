# Coverage - Wave 90 (runtime::dvd, GX matrix/vector maths and channel-state emitters in the `0x801C0000` page)

## How to read this file

The `runtime::dvd` bucket is the whole `0x801C0000` 64 KiB page of `main.dol`.
Only the middle of that page is disc code (`0x801CA640`-`0x801D6780`); the rest
is SDK code that happens to share the page. `src/runtime/dvd.rs` is the only
writable file assigned to the bucket, so those routines are reconstructed there
too, and every function doc says plainly which routine it is and which fields it
touches. This pass adds no disc code: the remaining untranslated addresses in
the bucket are all in the GX/matrix/texture bands below `0x801CA640`.

**GQR0 note.** Most of the vector routines below were compiled with paired
singles (`psq_l`/`psq_st`). Ghidra renders every such access as a switch over
the quantisation register GQR0 (arms `4`/`6` are 8-bit, `5`/`7` are 16-bit, the
remaining arms move plain `f32` pairs). GQR0 cannot be read from Rust and this
title's callers hand the routines plain `f32` arrays, so the bodies reconstruct
the `f32` arm and their doc blocks say so. The quantised arms are not
reproduced; those routines carry `medium` confidence for that reason.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 29 | 3032 | 8 | 21 |

## What this pass covers

- Matrix library, `0x801C04D0`-`0x801C1650` (16 bodies, 1660 bytes). The 3x4
  float matrix core (`fn_801c04d0` identity, `fn_801c0500` copy), the look-at
  basis builder `fn_801c0da0` (two normalises, two crosses, three negated eye
  dots) and four projection-style builders, `fn_801c0f20` (perspective terms),
  `fn_801c10d0` (orthographic terms), `fn_801c11c0`/`fn_801c1350` (the 4x4
  light variants with the `r2-0x4F24`/`-0x4F28`/`-0x4F2C`/`-0x4F30`
  placeholders). Around them sit the 3-vector helpers: add `fn_801c13f0`, sub
  `fn_801c1420`, scale `fn_801c1450`, normalise `fn_801c1470` (reciprocal
  square root plus one Newton refinement using the SDA2 floats at `r2-0x4F14`
  and `r2-0x4F18`), squared length `fn_801c14c0`, dot `fn_801c1510`, cross
  `fn_801c1530`, the two-point normal `fn_801c1570` (threshold `r2-0x4F10`) and
  squared distance `fn_801c1650`. The square root goes through the SDK helper
  `0x802E8620`, because `f32::sqrt` is unavailable in this `no_std` crate.
- Texture-object helpers, `0x801C6610`-`0x801C7840` (4 bodies, 464 bytes). The
  filter-scale selector `fn_801c6610` (range checks against `r2-0x4EC0`/
  `-0x4EA8`, three modes writing `+0x1C`/`+0x20`/`+0x24`), the packed-word
  writer `fn_801c7130` (`leading_zeros(count-1)` selects bit 4, then the byte
  table at `r13-0x6088` supplies bits 5-7), the descriptor splitter
  `fn_801c7180` and the `+0x564`/`+0x584` to `+0x108`/`+0x128` mapper
  `fn_801c7840`, which ends by emitting the register word and clearing
  `state + 2`.
- GX channel state and FIFO packers, `0x801C95E0`-`0x801C9C40` (9 bodies, 908
  bytes). `fn_801c95e0` flushes `+0x5FC` through `fn_801c52f0` and then emits
  one caller word; `fn_801c9660` transforms a point through a 3x4 matrix and
  applies two scale/offset blocks; `fn_801c9840`/`fn_801c98b0`/`fn_801c9900`
  store and reload the twelve-word colour block at `+0x528`-`+0x540` and set
  bit 27 of `+0x5FC`; `fn_801c9990`/`fn_801c9a20` pack the `0xB000`/`0x8000`
  texture command words and `fn_801c9a60` sets the low six bits of `+0x80`;
  `fn_801c9c40` copies the six floats at `+0x544`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801C04D0` | `fn_801c04d0` | 44 | medium |
| `0x801C0500` | `fn_801c0500` | 52 | medium |
| `0x801C0DA0` | `fn_801c0da0` | 372 | medium |
| `0x801C0F20` | `fn_801c0f20` | 164 | medium |
| `0x801C10D0` | `fn_801c10d0` | 136 | medium |
| `0x801C11C0` | `fn_801c11c0` | 156 | medium |
| `0x801C1350` | `fn_801c1350` | 152 | medium |
| `0x801C13F0` | `fn_801c13f0` | 36 | medium |
| `0x801C1420` | `fn_801c1420` | 36 | medium |
| `0x801C1450` | `fn_801c1450` | 28 | medium |
| `0x801C1470` | `fn_801c1470` | 68 | medium |
| `0x801C14C0` | `fn_801c14c0` | 68 | medium |
| `0x801C1510` | `fn_801c1510` | 32 | medium |
| `0x801C1530` | `fn_801c1530` | 60 | medium |
| `0x801C1570` | `fn_801c1570` | 216 | medium |
| `0x801C1650` | `fn_801c1650` | 40 | medium |
| `0x801C6610` | `fn_801c6610` | 196 | medium |
| `0x801C7130` | `fn_801c7130` | 40 | high |
| `0x801C7180` | `fn_801c7180` | 84 | high |
| `0x801C7840` | `fn_801c7840` | 144 | high |
| `0x801C95E0` | `fn_801c95e0` | 116 | high |
| `0x801C9660` | `fn_801c9660` | 392 | medium |
| `0x801C9840` | `fn_801c9840` | 100 | high |
| `0x801C98B0` | `fn_801c98b0` | 76 | medium |
| `0x801C9900` | `fn_801c9900` | 64 | medium |
| `0x801C9990` | `fn_801c9990` | 44 | high |
| `0x801C9A20` | `fn_801c9a20` | 52 | high |
| `0x801C9A60` | `fn_801c9a60` | 32 | high |
| `0x801C9C40` | `fn_801c9c40` | 32 | medium |

## Deliberate skips

- `0x801C9470` (180 bytes) and `0x801C9530` (176 bytes): both consume a
  64-bit return value from `0x802DCD74`/`0x802DCD6C` and split it into two
  words before storing it. `crate::platform` has no 64-bit-return call helper,
  so the two halves cannot be reconstructed honestly here.
- `0x801C97F0` and `0x801C99C0` (68 and 88 bytes): their final store is a
  paired-single pair emitted through `0xCC008004` with Ghidra merging a 16-bit
  and a 32-bit store at `+2`/`+4`. The low half of the emitted pair is not
  recoverable from the export, which is the same defect Wave 80 recorded for
  `0x801C5570`/`0x801C56C0`.
- `0x801C9940` (80 bytes) and `0x801C9A80` (180 bytes): both begin with a
  16-bit FIFO store and then merge the three-float payload into
  `0xCC008000`/`0xCC008004`. The first command word is a `param_2 << 2`
  nine-bit field whose residual high bits come from the write-only FIFO, so the
  exact emitted word is not established.
- `0x801C0FD0` and `0x801C1260` (248 and 236 bytes): the two projection builders
  that call `0x802E8590` with an `f64` argument. Only the one-argument `f64`
  call shape is used elsewhere in the tree, and the exact role of the argument
  (tangent of half the field of view) is inferred, so no body is claimed.
- `0x801C1680` (460 bytes): quaternion extraction; its entry point returns two
  pointers packed in a 64-bit register pair through `0x802DCD6C`, the same
  limitation as `0x801C9470`.
- `0x801C6FA0` (72 bytes): reads its real value from a ninth stack argument
  (offset `+8`) while the intervening register arguments are unused and unnamed
  in the export, so the signature cannot be written without inventing seven
  parameters.
- `0x801C0020`, `0x801C0CF0` and the other untouched addresses in the page are
  outside these clusters and were not attempted.
- No address already present in `ledger/functions.csv` was re-claimed. After
  this pass the `0x801C0000` bucket holds 66 untranslated addresses (95 before
  the pass).

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (29 rows, 3032 bytes total, 8 high and 21 medium).
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c` path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave90` prints `OK`.
- `src/runtime/dvd.rs` was checked in isolation with `rustc --emit=metadata`
  (the shared `cargo check` currently stops on another lane's
  `src/game/worldmap.rs` parse errors); it reports no error, only the existing
  `unused_mut`/dead-code warnings.
