# Coverage - Wave 115 (`runtime::os`, `0x80240000` bucket)

## How to read this file

This page records the final pass of the `0x80240000` bucket. The bucket holds
137 inventory entries; 136 were already merged when this pass started and one
was open: `0x8024E180`. It is reconstructed here, so the bucket is complete.

- **Status** - the row is `translated`: every guard, call, store, arithmetic
  expression and GX FIFO write in the `0x8024E180..0x8024EA20` instruction
  stream has a counterpart in `src/runtime/os.rs`.
- **Confidence** - `medium`. The control flow, the seven-argument calling shape
  (eight call sites), the two power evaluations, the three matrices and the
  nine-byte CP-register write are literal. The roles of the pose-record
  exponents at `+0x0C`/`+0x10`, the `0x8023D460` four-byte quad and the
  `0x8023D8B0` twelve-byte matrix are `// inferred:` from the way the body
  consumes them.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; the byte size
  was re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
  The Ghidra C omits the GX command bytes, so the byte-level packet was read
  from `work/dol/main.elf` at `0x8024E858`..`0x8024E878`.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave115-os.csv` (same row).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 1 | 2,212 | 0 | 1 |

Status split: 1 `translated`, 0 `partial`, 0 `stub`; 0 of the 1 are `low`.

## What this pass covers

- **`0x8024E180` is the per-drawable GX packet emitter, not a vertex
  integrator.** The eight call sites already translated in Wave 97 pass seven
  integer words; the function resolves the drawable pair through `0x802DCD44`
  (high half = context, low half = drawable), asks `0x8024DCE0` to install the
  render state, and then chooses between two paths.
- **The animation path.** When `0x8023E370` returns a record, the body copies
  the constant table `0x802F8EC0` (`0, 2, 1, 3`), reads the two exponents at
  record `+0x0C`/`+0x10`, samples one pose per frame with `0x8023D240`, and
  evaluates `0x802E861C` three times: twice with the record exponents and once
  with the literal `4.0` from `sda2(-0x49D0)`. The three `0x8023D460` calls
  fill three four-byte quads; the first two supply four interpolated
  components, the first three with `1 - sample` / `sample` and the fourth with
  `1 - pow(sample, exponent_a)` / `pow(sample, exponent_a)`.
  `0x8023DCB0` fills the two four-word output arrays
  and `0x8023D8B0` builds one twelve-byte matrix per present vertex. The
  texture word is `[255 * sample, sample^4, low half of the third quad]`.
- **The nine-byte GX write.** Each surviving triangle vertex writes `0x10`
  (load CP register), the 32-bit register number `0x100C` or `0x100D`, and the
  four assembled bytes to `0xCC008000`. Ghidra collapsed this to one
  `_DAT_cc008000 = local_21c` store; the three preceding `stb`/`sth`
  instructions are present at `0x8024E858`..`0x8024E878` and are reproduced.
  The register selector is bit 0 of the aliased word at `+0x80 + 8 * vertex`.
- **The literal frame aliasing.** The original uses one `0x260`-byte frame.
  The `+0x90`/`+0xA0` output arrays are read with a two-word stride, so the
  third iteration reaches the `+0xB0` float scratch; the `+0x78` blend array is
  read with a four-byte stride, so the third entry is the constant table at
  `+0x80`. The Rust body keeps a `[u32; 0x98]` frame and the literal offsets
  rather than replacing the aliases with disjoint typed arrays, because doing
  so would change the bytes the emitter sends.
- **The no-animation path.** `0x8024CC10` installs the state, up to four
  indexed records from `param_6 + 0x2C`/`+0x30`/`+0x34` are handed to
  `0x8023E6C0`/`0x8023E7A0`/`0x8023E880` (the last two with channel `0x0B` and
  `0x0C`), and `0x8024CDC0` closes the packet.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8024E180` | `fn_8024e180` | 2,212 | medium |

## Deliberate skips

None. This closes the bucket at 137 of 137.

The earlier waves rejected this address for three reasons that are now
resolved and recorded rather than skipped:

1. **The eight `psq_st`/`ldexpf` blocks are a callee-save prologue, not eight
   float arguments.** `f23`..`f31`/`ps23`..`ps31` are the non-volatile paired
   registers; the eight call sites pass seven integer words. The Rust
   signature is the real one.
2. **Ghidra's `local_1e8` is two four-byte entries, not a three-entry array.**
   The emitter deliberately reads the third entry from the constant table at
   `+0x80`; the C's `i*4` stride is literal and the overlap is the function's
   actual behaviour.
3. **Ghidra's CONCAT chain lost a byte.** `CONCAT31(local_21c._0_3_, bVar16)`
   overwrites the third computed byte, while the instruction stream stores
   three separate bytes at `+0x44`/`+0x45`/`+0x46` and the fourth at `+0x47`.
   The body writes the four bytes explicitly and the packet matches the
   `stb`/`stw` sequence.

The only remaining uncertainty is the meaning of the two pose exponents and
the two four-byte quads; they are marked `// inferred:` in the body.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (1 of 1; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors and free of warnings
  for `src/runtime/os.rs`; the remaining warnings belong to other lanes.
- `node tools/check-lane-csv.mjs wave115` prints `OK`.
