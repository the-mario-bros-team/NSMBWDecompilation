# Coverage - Wave 109 (resource::bres)

## How to read this file

This pass edits only `src/resource/bres.rs`, and adds exactly two bodies to the
end of it. Every row below is one literal reconstruction of a Ghidra body: the
doc block above the Rust body names the original address, the byte count from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence file. Offsets are
relative to the object named in the row.

The `0x80230000` bucket is shared with several subsystems. The Ghidra inventory
lists **251** addresses in it; waves 100 and 106 brought it to **246**
translated. **Wave 109 takes two of the five that remain** (`0x80235AE0` and
`0x80238930`) and leaves three, each with its reason under *Deliberate skips*.
The bucket now has **248 of 251** translated, and the file as a whole holds
**258** translated bodies.

- **high** - every load, store, branch and call is visible in the export and no
  argument had to be recovered.
- **medium** - a field role or callee identity had to be reconstructed from a
  sibling, a value had to be recovered from a prologue stub, or a return value
  is inferred from two call sites.
- **low** - only the instruction sequence is certain; three lost argument lists,
  a control-flow restructure across `goto`s or a table layout had to be
  modelled.

Inside the Rust file, `// observed:` marks facts read directly from the Ghidra
C, `// inferred:` marks a role reconstructed from neighbours, and `// unknown:`
marks an unresolved field.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/resource/bres.rs` | `resource::bres` | 2 | 896 | 0 | 1 |

One of the two is `low`: the script parser.

## What this pass covers

- **Script-symbol parser** (`0x80235AE0`, 716 bytes): the body behind
  `fn_80235dc0`'s `SCRIPT_PARSE` call. It scans the console script for a line
  beginning with `.`, skips the three-line section header, then walks the
  entry lines of the current block record. For each line it reads token 1 as a
  byte length and token 2 as a byte offset, tests
  `block.start + offset <= address < block.start + offset + length`, and on a
  match copies token 5 (the symbol name) into the caller's buffer until a
  newline, a NUL or `capacity - 1` characters.
- **Beam-matrix wrapper** (`0x80238930`, 180 bytes): converts the caller's
  angle to radians as `(2*pi/256) * (256/360) * (0.5 * angle)`, takes the
  Metrowerks `tan` at `0x802E8590`, scales the result, and calls the matrix
  builder `0x802389F0` with `(s, -s, -(s*ratio), s*ratio, scale)` plus the two
  pointer words it receives in `r3`/`r4`.

## Two decoding facts that unlocked this pass

1. **`0x802DCD64` is `_savegpr0_23`, not a call.** The parser export opens with
   `uVar8 = FUN_802dcd64();` and then treats `(int)uVar8` as a record pointer
   and `(int)(uVar8 >> 32)` as the script address. That is the incoming
   `r3:r4` pair with `r4` in the low word; the same swap the wave-106 anchor
   twins needed. Its call site `0x80235DC0` passes exactly those two words
   (`0x80000000`/the buffered command, and `record[1]`).
2. **The two argument lists Ghidra dropped belong to `0x80235A10`.** Both
   `FUN_80235a10()` calls sit directly below an `FUN_80235940(cursor, n, 0x20)`
   whose result is otherwise unused, and `0x80235DC0` shows the same
   token-then-parse pairing. The recovered arguments are the token addresses.
   The copy loop the export printed after its `return` is reachable from the
   `goto` at `0x80235D20`, so it is a normal loop, not dead code.

The three constant words `0x80238930` multiplies were read from the shipped
image (`work/dol/main.elf`, section at `0x80006780`): `0x8042E4E8` is
`0x3CC90FDB` (`2*pi/256`), `0x8042E4EC` is `0x3F360B61` (`256/360`) and
`0x8042E4E0` is `0x3F000000` (`0.5`). The last one is the address the math band
now calls `SDA2_MATH_HALF`; the bytes are a half, and the product of the three
is exactly one degree in radians.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80235AE0` | `fn_80235ae0` | 716 | low |
| `0x80238930` | `fn_80238930` | 180 | medium |

## Deliberate skips

Three of the five addresses wave 100 left stay untranslated after this pass.
Every one was re-read in the export before being skipped again.

- **`0x802382B0` (360 bytes):** the 3x4 Euler rotation builder. Its angle
  reducer subtracts `r2 - 0x4BA8` (`0x8042E4D8`, `f32` 65536) from `|angle|`
  until the angle is smaller — `// inferred:` a full turn in 16-bit fixed point
  - and every table read is branched four ways on the *live* GQR word: `psq_l`
  type 0 reads floats, types 4/6 read signed bytes, types 5/7 read signed
  halfwords, while the table index is taken from the raw bits of a value the
  export keeps in one reused four-byte scratch word. The quantization type is
  an immediate of the instruction, not a runtime value, so choosing a branch is
  a guess about the encoding; the decompiler also reuses
  `dVar13`/`dVar16`/`dVar17`/`dVar21` for both the reduced angle and the table
  slopes, so a transcription would not describe one consistent program.
  `MTX34_SCALE` is still the address call.
- **`0x802389F0` (1 752 bytes):** the matrix builder `0x80238930` calls. The
  export is 2 906 lines of code with 126 "removing unreachable block" markers,
  six paired-single input pairs and four 12-float scratch matrices;
  its own parameters are the `undefined8` pointer pair plus five doubles. It is
  a faithful transcription problem far beyond this pass's size, so it stays an
  address call; `fn_80238930` documents the argument shape it needs.
- **`0x802390D0` (688 bytes):** the closest-approach/segment kernel. 803 lines
  of code (840 with its 34 unreachable-block warnings) of scalar emulation with
  more than thirty scratch slots reused across `psq_st`/`psq_l` lane widths, and
  its vector reads split across a record boundary (`param_1 + 3` and
  `param_1[4]` are the fourth lane of one vector and the first lane of the
  next). As with `0x802382B0`, the GQR branches are
  instruction immediates the export renders as runtime tests, so each load
  would be a guess about which lane the scratch slot holds.

## Verification

- Every address and byte size matches
  `work/ghidra/main-gb/export/functions.tsv` (re-read from the TSV, not copied
  from prose): 716 and 180.
- Every evidence file exists at the cited repo-relative path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/resource/bres.rs`; the file now holds 258 such bodies.
- No address in `ledger/agents/wave109-bres.csv` was already in
  `ledger/functions.csv` (both were absent before this pass).
- `cargo check --message-format=short` reports no error and no warning for
  `src/resource/bres.rs`.
- `node tools/check-lane-csv.mjs wave109` reports `OK` for this lane.

`cargo check` proves the Rust is well-formed and type-correct; it does not run
the target code. The pointer arithmetic models the Wii's 32-bit big-endian
PowerPC and must only execute on a target-compatible runtime with the original
memory image mapped.
