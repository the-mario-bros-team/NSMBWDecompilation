# Coverage - Wave 110 (engine::dsys)

## How to read this file

`engine::dsys` reconstructs the `dSys_c` engine start-up together with the
display, text, sprite and ribbon helpers around it in the main `DOL` (bucket
`0x800E0000`, 189 functions in total). This pass closes the last three
untranslated entries in that bucket.

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
| `src/engine/dsys.rs` | `engine::dsys` | 3 | 4628 | 1 | 2 |

The file already held 146 bodies (18 `dSys_c` start-up, 30 Wave 75, 28 Wave 89,
26 Wave 93, 25 Wave 97, 15 Wave 103 and 4 Wave 107); this pass adds 3 more, for
149 bodies total. No `0x800E0000` bucket entry remains untranslated.

## What this pass covers

- **The three remaining ribbon/road entries, `0x800E0540..0x800EC3B0`.**
  They are one small cluster: a record marshal, a rail generator and a
  per-emitter update.

### 1. The Wave 107 skip of `0x800E9130` was a misattribution

Wave 107 skipped `0x800E9130` because the export was said to write eight colour
bytes at `+0xD1CC`/`+0xD21C` with `stb`. The export at
`work/ghidra/main-gb/export/800e9130_FUN_800e9130.c` contains no `stb` at all,
and `work/dol/main.elf` over `0x800E9130..0x800E9538` contains no primary
opcode 38 (`stb`). The eight colour bytes belong to `0x800EC3B0`, whose export
and instruction stream both contain the four `stb` pairs at
`0x800EC9B0..0x800EC9F4`. The Wave 107 note therefore described the wrong
function.

With that cleared, `0x800E9130` is a normal rail generator: it reads nine floats
from the lane object at `+0x61C..+0x654`, seeds a matrix from the `u16` pair
`0x4000`/`0xC000` at `0x8042CD94`, concatenates a cross-section rotation, walks
the `+0x24` node chain (`+0x54` next), writes `+0x34..+0x48` and calls
`0x800EA7D0` per node.

### 2. `0x800E0540` is a straight record marshal

The dynamic frame is `subfic ...,-16352` = `0x3FE0` bytes. The first two copy
loops (`0x69` eight-byte moves, from `source + 0x20` and `source + 0x36C`)
produce one 0x6A0-byte header. Each `0x800E0380`/`0x800E03B0` result is then
copied into a slot that advances by `0x980`; the scalar stores plus the
`0xD2`/`8`/`0x34`-iteration loops cover every byte of the slot, so the copies
are full `0x980`-byte blocks. The destination bases are `0x6A0 + i * 0x980` and
`0x2320 + i * 0x980`, i.e. three A records followed by three B records.

### 3. `0x800EC3B0`'s colour chain is two explicit four-byte words

The Wave 107 blocker was the colour assembly. The byte stores are explicit: the
`CONCAT12`/`CONCAT13` chains build the word stored at `+0xD1CC` as
`[scale*(b12-255)-1, b10+scale*(b13-b10), 0xFF, 0xFF]` and the word at
`+0xD21C` as `[b20+scale*(b18-b20), 0, 0xFF, 0xFF]` in the first half of the
ramp, with the white/`b15` variants in the second half. The body transcribes
those byte expressions directly and writes each word with `write_u32`, which
stores big-endian bytes exactly like the four `stb` instructions.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800E0540` | `fn_800e0540` | 1756 | high |
| `0x800E9130` | `fn_800e9130` | 1036 | medium |
| `0x800EC3B0` | `fn_800ec3b0` | 1836 | medium |

## Field notes

- `0x800E9130` keeps the `u16` seed `0x3A00` (`0x3A00 / 256 * 1/256 =
  0.2265625`) separate from the orientation pair `0x4000`/`0xC000`; only the
  latter goes through `0x8016ED10`. The `psq_l ...,I=3` dequantisation and the
  explicit `1/256` at `0x8042CD98` are modelled as two steps so neither is
  hidden in a magic constant.
- `0x800E9130`'s two `0x80237BD0`/`0x80237C40` calls are the sin/cos pair; the
  export does not name them, so the body marks the identity `// inferred:`.
  Swapping them only mirrors the cross-section.
- `0x800EC3B0`'s ramp divisor is `48.0`, the phase is
  `(index + *(r13 - 0x5480)) % 5`, and the tick byte at `+0xD1B8` adds four
  phases per unit. The mode byte at `+0xD308` selects the green ramp.
- `0x800E0540`'s record layout is not named: the body copies whole slots rather
  than pretending to know every field. The header and slot sizes are literal.

## Deliberately skipped

None. All three remaining bucket entries were reconstructed.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (`800e0540` 1756, `800e9130` 1036, `800ec3b0` 1836).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/engine/dsys.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/engine/dsys.rs` (the remaining errors belong to other lanes' in-flight
  files).
- `node tools/check-lane-csv.mjs wave110` reports `OK` with 3 rows for this
  lane.
