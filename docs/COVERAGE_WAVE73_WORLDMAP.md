# Coverage - Wave 73 (WorldMap action-state and trick remainder)

## How to read this file

Every row below is one `pub unsafe fn` body in `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. The evidence column points at the
Ghidra C export that the body was transcribed from; byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`. Field offsets in the summaries are
literal offsets from the object pointer named in the body's `# Arguments`
block. The WorldMap player actor's status words live at `+0x1014`, the
controller object at `+0x2A64` and the `+0x60` vtable is the shared actor
interface, exactly as in the wave 36/60 passes.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 28 | 4512 | 20 | 8 |

## What this pass covers

- `0x8013C3F0..0x8013CF70` - the spin/trick state machine. The group shares
  the `+0x14A4` state word, the trick counter `+0x17C4`, the heading delta
  `+0x17CE` and its direction `+0x17D0`, plus the `+0x102` heading halfword.
  [`fn_8013c520`] is the seed routine, [`fn_8013c660`] / [`fn_8013c7c0`] /
  [`fn_8013c5f0`] are the three exit variants, [`fn_8013ca80`] dispatches on
  `+0x14A0`, [`fn_8013cb90`] is the full cancel, [`fn_8013cc90`] is the
  per-frame delta update, [`fn_8013ce00`] synchronises the animation record at
  `+0x388` and [`fn_8013cf70`] selects its state word. [`fn_8013c3f0`]
  recomputes the forward speed `+0x11C` and the turn value `+0x110` from the
  facing probe and the `0x8042B840` sign table.
- `0x8013F950..0x8013FD70` - the approach/landing ladder on the
  `+0x1458`/`+0x145C` counters. [`fn_8013f950`] starts the first state,
  [`fn_8013f9a0`] posts `0x803251CC`, [`fn_8013f9e0`] arms a three-word
  target, [`fn_8013fac0`] tests the `+0x1418` holder's name, [`fn_8013fb40`]
  emits the `85.0f` controller sound, [`fn_8013fbb0`] clears the two status
  bits, [`fn_8013fbf0`] eases X then Y through the `0x8015F480` thunk and
  [`fn_8013fd70`] copies the target words to another actor.
- `0x80130BC0`, `0x80130C10`, `0x80130DA0` - the per-frame dispatcher for the
  `+0x1090 == 5` variant, its `+0x2060`/`+0x2061` timer bytes and the
  `+0x1464` child-name test.
- `0x80134100`, `0x80134160`, `0x801380D0`, `0x80139180`, `0x801393D0`,
  `0x80139A10`, `0x8013A150` - seven small helpers that use the same `+0x60`
  vtable and status bits: mode reset, mirrored heading ease, event-state
  reset, one-shot event `0x80102260(3,0,1,2)`, the `+0x2A6C` height store,
  the 10-frame timer seed, and the mutual-link comparator.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80130BC0` | `fn_80130bc0` | 80 | high |
| `0x80130C10` | `fn_80130c10` | 196 | medium |
| `0x80130DA0` | `fn_80130da0` | 412 | medium |
| `0x80134100` | `fn_80134100` | 88 | high |
| `0x80134160` | `fn_80134160` | 72 | high |
| `0x801380D0` | `fn_801380d0` | 108 | high |
| `0x80139180` | `fn_80139180` | 100 | high |
| `0x801393D0` | `fn_801393d0` | 72 | medium |
| `0x80139A10` | `fn_80139a10` | 124 | high |
| `0x8013A150` | `fn_8013a150` | 48 | high |
| `0x8013C3F0` | `fn_8013c3f0` | 304 | medium |
| `0x8013C520` | `fn_8013c520` | 196 | high |
| `0x8013C5F0` | `fn_8013c5f0` | 104 | high |
| `0x8013C660` | `fn_8013c660` | 88 | high |
| `0x8013C7C0` | `fn_8013c7c0` | 140 | high |
| `0x8013CA80` | `fn_8013ca80` | 260 | high |
| `0x8013CB90` | `fn_8013cb90` | 248 | high |
| `0x8013CC90` | `fn_8013cc90` | 368 | high |
| `0x8013CE00` | `fn_8013ce00` | 364 | medium |
| `0x8013CF70` | `fn_8013cf70` | 156 | high |
| `0x8013F950` | `fn_8013f950` | 80 | high |
| `0x8013F9A0` | `fn_8013f9a0` | 64 | high |
| `0x8013F9E0` | `fn_8013f9e0` | 180 | medium |
| `0x8013FAC0` | `fn_8013fac0` | 124 | medium |
| `0x8013FB40` | `fn_8013fb40` | 108 | high |
| `0x8013FBB0` | `fn_8013fbb0` | 60 | high |
| `0x8013FBF0` | `fn_8013fbf0` | 228 | medium |
| `0x8013FD70` | `fn_8013fd70` | 140 | high |

## Deliberate skips

- `0x80130CE0`, `0x8013A880` and `0x8013E850` were skipped because their
  exports carry `unaff_GQR0` / `ldexpf` reconstruction blocks, so the
  paired-single scale factors are not recoverable from the C. `0x8013A880`
  additionally has a removed unreachable block at `0x8013A908`. Waves 60, 64
  and 67 skipped the same record class.
- `0x8013B0B0` and `0x8013BB90` remain untranslated for the same GQR reason;
  they were already skipped by wave 36 and are outside this lane's pick.
- `0x80133E10`, `0x80133EC0`, `0x801344E0`, `0x801345C0` and `0x80134700` were
  skipped: Ghidra prints "Could not recover jumptable ... Treating indirect
  jump as call" directly above their tail call, so the recorded call is a
  misrendered jump-table branch, not a verified call.
- `0x8013A180` was not selected: it is reconstructible but dense (eight `r2`
  float pairs, a `CONCAT44` conversion and three engine geometry calls), and
  it does not belong to any of the three clusters this pass took.
- `0x8013F570` was skipped because its `4503601774854154.0` operand is a
  Ghidra integer-to-double conversion artifact, not a recoverable constant.
- `0x8013CE00` keeps one inferred stack contract: slot `+0x54` of the
  controller is printed without a prototype and receives a 52-byte buffer; the
  Rust body therefore calls it through an explicit `extern "C"` pointer and
  documents the buffer as opaque. The rest of the body is literal.
- The remaining untranslated entries of the `0x80130000` bucket are the
  `0x80130F40..0x80132DF0` movement/record group, the `0x80137320..0x80138F40`
  map-object group and the `0x80139950..0x8013A880` link group; the last one
  already has address constants used by earlier bodies, so translating it
  needs a reference cleanup rather than a new lane pick.


## Verification

- Every address and byte size matches `functions.tsv`; the 28 sizes sum to
  4,512 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv` (all 28 were untranslated
  entries of the `0x80130000` bucket at the start of the pass).
- `cargo check --message-format=short` produces no error or warning that names
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave73` reports `OK` for this lane's CSV
  (28 rows, unique address).
