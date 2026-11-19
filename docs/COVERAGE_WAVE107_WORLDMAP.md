# Coverage - Wave 107 (`game::worldmap`, `0x80140000` bucket remainder)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 16 | 8,328 | 1 | 15 |

All sixteen rows are `translated`; none is a `stub` or `partial`.

## What this pass covers

The `0x80140000` bucket had 17 untranslated entries when wave 106 finished.
Sixteen are written here; the one deliberate skip is listed below. The pass
covers four clusters:

- **`0x8014A690`/`0x8014A6C0` - the `0x802DCBFC` dispatch pair.** Two
  byte-identical wrappers forward their second word to the unrecovered
  switch/vtable trampoline at `0x802DCBFC`. The export prints three operands
  but the trampoline reads only `r3`, so the first wrapper word is dropped
  exactly as the callee does.
- **`0x801406F0..0x80147700` - the WorldMap player's motion helpers.** This
  is the slip/warp actor's four-state countdown (`0x801406F0`), the
  `0x802F4EF8`/`0x802F4F18` pose blend (`0x80140FF0`), the two-record contact
  hand-off (`0x80141A60`), the `0x802F50A8`/`0x80325260` motion-table
  selector with its ground-span override (`0x80142030`), the four-group
  motion-table flag ladder (`0x80142660`), the dual ground probe (`0x801430E0`),
  the position/angle/transform submit with the 12-float pose matrix chain
  (`0x80145650`), the `0x80145980` x `0x800541C0` product (`0x801459B0`), the
  guarded `+0x400` vtable forward (`0x801468D0`), the `+0x27E8` slot retire
  pair (`0x801472F0`) and the nearest-player scan (`0x80147700`).
- **`0x8014AC10` - the warp-cannon flash step.** Reads the `+0x514` flash
  word through the signed-to-double idiom, maps it to `1.0`/`0.5`/`0.0`, and
  copies the owner's position back through `0x80139420`.
- **`0x8014CDD0`/`0x8014F630` - the scale and target-state helpers.** The
  first turns the `+0x10C` magnitude into the `+0xEC` scale; the second
  switches on the target's `+0x36D` mode and posts the `0x64`/`0x5C`/`0x68`
  effects before clearing the `+0x151C` state.

## Paired-single / GQR notes

- `GQR0` is zero for the whole image (see the wave 106 note in the module
  header), so every `if ((unaff_GQR0 & 0x3f00) != 0) { ldexpf(...); }` block is
  dead scaffolding. Two exports (`0x801430E0`, `0x801469D0`) additionally
  consume incoming `f30`/`f31` pairs; in `0x801430E0` both derived locals are
  dead and are not reproduced.
- `GQR3` is `0x00050005` (unsigned 16-bit, scale `0`), so the `0x801406F0`
  and `0x80142660` angle reads are plain `(u16)` loads scaled by `1/256`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801406F0` | `fn_801406f0` | 828 | medium |
| `0x80140FF0` | `fn_80140ff0` | 336 | medium |
| `0x80141A60` | `fn_80141a60` | 388 | medium |
| `0x80142030` | `fn_80142030` | 1,012 | medium |
| `0x80142660` | `fn_80142660` | 2,236 | medium |
| `0x801430E0` | `fn_801430e0` | 516 | medium |
| `0x80145650` | `fn_80145650` | 816 | medium |
| `0x801459B0` | `fn_801459b0` | 168 | medium |
| `0x801468D0` | `fn_801468d0` | 256 | medium |
| `0x801472F0` | `fn_801472f0` | 244 | medium |
| `0x80147700` | `fn_80147700` | 276 | medium |
| `0x8014A690` | `fn_8014a690` | 48 | medium |
| `0x8014A6C0` | `fn_8014a6c0` | 48 | medium |
| `0x8014AC10` | `fn_8014ac10` | 412 | medium |
| `0x8014CDD0` | `fn_8014cdd0` | 216 | high |
| `0x8014F630` | `fn_8014f630` | 528 | medium |

The sixteen sizes sum to 8,328 bytes.

## Deliberate skip

**`0x801469D0` (812 bytes).** Its export is a 47-branch action-entry state
machine whose argument block is entirely built out of the `0x802DCD6C` save
helper's preserved registers: `param_1`, the `extraout_f1` double and the four
incoming `psq_l` pairs are all aliases of the same entry registers, and the
`(int)uVar10 == 0` test later in the body reads the low word of that same
"return value". Reproducing the parameter split would mean guessing which
entry register is the height, which is the cancel flag and which is the
forwarding word, so the body is left out rather than invented.

## Notes for the coordinator

- **`0x80119240` has a `ledger/functions.csv` row (`fn_80119240`, 608 bytes)
  but no body in `src/game/worldmap.rs`.** This was already true before this
  pass started: the identifier does not occur anywhere in the file text apart
  from the header prose. It is outside this bucket, so it is not claimed here,
  but the row/body mismatch should be repaired by whoever owns the
  `0x80110000` bucket.
- The module's blank separator lines were normalised to none during this pass,
  and a handful of adjacent `//` comment lines were joined. All code, doc text
  and function bodies are otherwise unchanged, and `cargo check` is clean.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs` (972 public bodies in the file after this pass).
- No address was already in `ledger/functions.csv`; all sixteen were
  untranslated entries of the `0x80140000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave107` reports `OK` and
  `wave107-worldmap.csv: 16 rows`.
