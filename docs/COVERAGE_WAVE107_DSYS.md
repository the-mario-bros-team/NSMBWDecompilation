# Coverage - Wave 107 (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite and notification helpers around it in the main `DOL`
(bucket `0x800E0000`, 189 functions in total).

Each row below is one `pub unsafe fn fn_<address>` in `src/engine/dsys.rs`. The
doc block above that body cites the Ghidra export it came from, the byte count
from `work/ghidra/main-gb/export/functions.tsv`, and a confidence level:

- `high` means every guard, store and call argument is literal in the export;
- `medium` means a field role, a callee identity or a register-pair recovery was
  inferred and is marked `// inferred:` or `// unknown:` in the body;
- `low` means the instruction sequence is certain but one value or argument list
  is not recoverable from the export.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 4 | 2496 | 2 | 2 |

The file already held 142 bodies (18 `dSys_c` start-up, 30 Wave 75, 28 Wave 89,
26 Wave 93, 25 Wave 97 and 15 Wave 103); this pass adds 4 more, for 146 bodies
total. The bucket still has 3 untranslated entries, all listed under
**Deliberately skipped** below.

## What this pass covers

- **The glyph emitters of the text-stream dispatcher, `0x800E5820..0x800E6680`
  (4 bodies).** These are the four `case` arms of the unlisted dispatcher at
  `0x800E6310`, which forwards `(context, stream)` from `0x800E6290`. All four
  build the same 0x64-byte glyph record on the stack, hand it to the
  `0x8022D3C0`/`0x8022D5F0`/`0x802307E0` triplet and advance the stream cursor.
  - `0x800E5820` copies the object, parks the record's `+0x48` receiver into a
    virtual step at vtable `+0x10`, and branches on the cursor's high byte: `2`
    runs the three library colour setups, anything else submits the record
    as-is.
  - `0x800E5AD0` is the vertical twin: it squeezes the record's `+0x24`/`+0x28`
    scale pair to `0.6 *` the swapped values, measures the run on both sides of
    the cursor, and applies the `+0x40` window as two corrections (`-span * 0.5`
    on `+0x2C`, `0.75 * (tail + head)` on `+0x30`).
  - `0x800E5DD0` is not a record emitter but the "fit the cell to the display"
    path: `0x801C9CD0` fills four corners, the display record's `u16` fields
    `+4`/`+6` override two of them after the `0x802DCC4C` widening, and four
    saturated conversions become the `0x801C9C60` quad arguments.
  - `0x800E6680` keeps the longer of its two run measures, adds the per-record
    step (or the zero constant when the holder's `+0x10` bit 0 is set) and
    writes both the object's position and the caller's four-float block.

### Evidence that unblocked the pass

1. **The head `psq_l` blocks were never loads.** Wave 103 left five entries out
   because the exports open with paired-single work whose source address is
   missing. The instruction stream shows the pattern is `stfd f31 / psq_st
   f31` at entry and `psq_l f31 / lfd f31` at exit: the pairs are the
   non-volatile register saves of `f21..f31`, and the decompiled
   `(float)in_fNN` blocks write a fixed stack slot that nothing reads. They are
   dead stores in exactly the sense of the `ldexpf` saves recorded in Wave 97,
   so they are omitted with a note in each body.
2. **`0x802DCC4C` takes one double.** `0x800E5DD0` prints the same callee with
   one, two, three and six arguments. `work/dol/main.elf` at `0x800E5F30` shows
   only `f1` being set (`lfd f1,72(r1)`, `fsubs f1,f1,f2`, `fmuls f1,f1,f0`)
   before the `bl 0x802dcc4c`; the six-double site is the decompiler listing the
   live `f2..f6` registers of a prototype-less call. The body therefore uses one
   `call_f64_ret_u32` per site.
3. **The emitters are not called directly.** `0x800E5820`, `0x800E5AD0` and
   `0x800E6680` have no `bl` call site at all; they are the arms of the unlisted
   152-byte switch at `0x800E6310` (`cmpwi r4, 0xB/2/1/0xF`, reachable only
   through a function pointer), which is why `functions.tsv` has no entry for
   the switch and why Wave 103 could not see the argument shape. The switch
   forwards the stream in `r5` after a `mr r4,r5`, so the real signature is
   `(context, stream)`.
4. **The record copy is flat.** `0x800E5820`, `0x800E5AD0` and `0x800E6680` copy
   `+0x00..+0x63` from the object to the stack with a constant offset of zero,
   verified against the store offsets at `0x800E5850..0x800E5920`,
   `0x800E5B68..0x800E5C60` and `0x800E66C4..0x800E67C8`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800E5820` | `fn_800e5820` | 584 | medium |
| `0x800E5AD0` | `fn_800e5ad0` | 764 | medium |
| `0x800E5DD0` | `fn_800e5dd0` | 596 | high |
| `0x800E6680` | `fn_800e6680` | 552 | high |

## Field notes

- The glyph record is 25 words. `0x800E5AD0` and `0x800E6680` overwrite `+0x24`
  and `+0x28` with `0.6 *` the swapped scale pair (the factor is the word at
  `0x8042CD14`), and both park `FLT_MAX` (`0x8042CD10`) in the record's `+0x4C`
  slot so the two run measures cannot read a stale length.
- `+0x40` of the record is a `u16` angle/width word guarded by two bytes at
  `+0x42`/`+0x43`; the copy is byte-exact, so the body copies the whole 0x64
  bytes instead of unpacking them.
- `0x800E5DD0`'s two per-cell scan values are the `u16` fields `+4`/`+6` of the
  *dereferenced* display object (`**(r13 - 0x5200)`), the same pair
  `fn_800e28a0` reads; the `0.5` factor at `0x8042CD18` is applied before the
  saturating conversion, so the two values reach `0x802DCC4C` already halved.
- The `x ^ 0x8000_0000` + `2^52 + 2^31` sequence at `0x8042CCF8` is the
  CodeWarrior *signed* widening. `0x800E5820` writes it as the equivalent
  `step as i32 as f64` cast, per the Wave 103 convention.

## Deliberately skipped

3 bucket entries remain. Each was re-read this pass against both the export and
the instruction stream before being left out.

- **`0x800E9130` (1036 bytes).** The body is a road/ribbon generator: seven
  `psq_st` pairs are the usual dead float saves, and the two angle seeds are
  resolved (the `u16` pair `0x4000`/`0xC000` at `0x8042CD94`, dequantised by
  `psq_l ...,I=3` and scaled by the `1/256` word at `0x8042CD98`, i.e. the
  quarter-turn value `0.25` in the sine helper's units). It is skipped because
  the export's tail does not survive a check against the binary: it writes eight
  colour bytes at `+0xD1CC`/`+0xD21C` with `stb`, but the function contains no
  `stb` and no store to those offsets at all (`work/dol/main.elf`,
  `0x800E9130..0x800E9538`). Transcribing the tail would be transcribing
  Ghidra's reconstruction, not the code.
- **`0x800E0540` (1756 bytes).** A dynamic `0x3FE0`-byte frame
  (`subfic r11,r11,-16352`) that gathers a header plus three pairs of
  sub-records into one buffer and hands it to `0x800CF080`. The record bounds
  are only implied by the collapsed pointer arithmetic in the export (210- and
  52-iteration copies plus ~40 scalar stores per record), so the per-record
  length cannot be stated exactly; a byte-for-byte body would be speculation.
- **`0x800EC3B0` (1836 bytes).** Eleven `psq_st` pairs open it (dead, as above),
  and the rest is a per-particle update. The blocker is the colour assembly: two
  4-byte colour words are built as six `fctiwz`/`stfd`/`lwz` truncations plus
  six byte adds and four `stb` merges into unaligned stack slots, and the
  export's `CONCAT12`/`CONCAT13` chain does not determine which byte lands where.
  The position/`frsqrte` half is recoverable, the colour half is not, and a
  partial body would be worse than none.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (`800e5820` 584, `800e5ad0` 764, `800e5dd0` 596, `800e6680` 552).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/engine/dsys.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/dsys.rs` (the only remaining errors belong to other lanes'
  in-flight files).
- `node tools/check-lane-csv.mjs wave107` reports `OK` with 4 rows for this lane.
