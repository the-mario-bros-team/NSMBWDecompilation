# Coverage - Wave 116 (`runtime::os`, `0x80250000` bucket)

## How to read this file

This page records the final pass of the `0x80250000` bucket. The bucket holds
163 inventory entries; 162 were already merged when this pass started and one
was open: `0x8025FE30`. It is reconstructed here, so the bucket is complete.

- **Status** - the row is `translated`: every guard, vtable call, record read,
  weighted accumulation, byte conversion and `dcbst` flush of the
  `0x8025FE30..0x802605C4` instruction stream has a counterpart in
  `src/runtime/os.rs`.
- **Confidence** - `medium`. The five-argument shape, all record offsets, the
  three flag gates, the `psq_l`/`psq_st` element types and the two-argument
  flush are literal in the instruction stream. The roles of the records (morph
  or skin sources), of the `0xE8`/`0x138`/`0x178` caller fields and of the
  `sda2(-0x48B0)` blend word are `// inferred:` from the blend arithmetic.
- **Evidence** - the C file under `work/ghidra/main-gb/export/`; the byte size
  was re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.
  The export loses the argument list, so the calling convention and the two
  GQR lanes were read from `work/dol/main.elf` at `0x8025FE30` and
  `0x8025EEE0`..`0x8025EF28`.
- Addresses are virtual addresses of the unrelocated `main.dol` (image base
  `0x80004000`), not file offsets.

The machine-readable companion is `ledger/agents/wave116-os.csv` (same row).
This wave edits only `src/runtime/os.rs` plus those two files;
`ledger/functions.csv` is untouched and is merged by the coordinator.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 1 | 1,940 | 0 | 1 |

Status split: 1 `translated`, 0 `partial`, 0 `stub`; 0 of the 1 are `low`.

## What this pass covers

- **The export is degenerate, the instruction stream is not.** Wave 107 left
  `0x8025FE30` open because Ghidra prints an unregistered `extraout_r4` as the
  root argument, calls `FUN_8023b460()` with no arguments and reads three
  undeclared `f31`/`ps31` pairs. The bytes settle all three:
  `0x802DCD54` is the save entry of the MSL register-save block at
  `0x802DCD40` (`stw r19,-0x34(r11)` ... `stw r31,-0x04(r11)`), the four
  `or rN,rM,rM` instructions after it copy `r4`..`r7` into `r19`..`r22`, and
  the "incoming" floats are the callee-save spill of `f31` at
  `0x8025FE40`/`0x8025FE44`. The real signature is the five integer words
  `(slot, tree, meshes, mesh_table_1, mesh_table_2)`.
- **The call site fixes every argument.** The only direct caller is
  `0x8025DF8C`..`0x8025DFB0`: `slot` is the address of the stack word holding
  `*(object + 0xE8)`, `tree` is `*(object + 0x138)`, `meshes` is
  `*(object + 0x178)`, and the two tables are `*(object + 0x17C)` and
  `*(object + 0x180)`. `0x8023B460(slot)` returns the instance count through
  the root's `+0x18` offset, and the vtable slot `+0x38` returns the
  per-instance record.
- **The record layout is pinned by literal offsets.** `{u32 flags, u32 count,
  u32 channel0, u32 channel1, u32 channel2, f32 base_weight}` with sixteen-byte
  entries `{u32 channel0, u32 channel1, u32 channel2, f32 weight}` starting at
  `+0x18`. Each channel record stores its payload offset at `+0x08`, its table
  index at `+0x10` (the second and third channels) and its vertex count at
  `+0x1E`. A source is included when its weight differs from the shared
  `sda2(-0x48B0)` word; the base channel record is itself the first source,
  fetched through `0x8023E700`/`0x8023E7E0`/`0x8023E8C0`.
- **The two GQR lanes are proven, not guessed.** Index 0 stays at the zero
  written by `0x801AB150`, i.e. `f32` pairs: the first two passes read twelve
  bytes per vertex at `+0x00` (two floats) and `+0x08` (one float). Index 2 is
  GQR2, which `0x8025EEE8` programs to `0x00040004` (unsigned 8-bit load and
  store, scale 1.0): the third pass reads and writes four bytes at offsets 0
  and 2. The earlier waves' "GQR arms are unreachable" note does not apply to
  this address.
- **The flush has two arguments.** Ghidra prints `FUN_801ac4a0(ptr)`, but the
  callee is the `dcbst` range loop `FUN_801ac4a0(address, length)`. The length
  is left in `r4` by `mulli r4,r3,12` (first two passes) and
  `rlwinm r4,r3,2,0,29` (third pass), and the blend loops never touch `r4`; the
  body passes `count * 12` or `count * 4` as the second argument.
- **The record area is 0x218 bytes, not eight.** Ghidra declares the vtable
  method's second argument as `auStack_260[8]`, but the same instruction block
  clears 0x1F0 bytes at `r1+0x368` in sixteen-byte steps, three words at a
  time. That is the tail of the record area passed at `r1+0x340`: the method
  writes the 0x28-byte header and first entry, and entries 2..32 get their
  three pointer words cleared while the weight word at `+0x0C` is left alone.
  The Rust body allocates the full 0x218 bytes and reproduces that clear loop.
- **The three `(source, weight)` lists are 33 pairs each.** Their bases are
  `r1+0x028`, `r1+0x130` and `r1+0x238`, i.e. `0x108` bytes apart, which is 33
  eight-byte pairs and matches 32 record entries plus the base source.
- **The zero-source case is guarded in the host body.** If every weight equals
  the shared blend word the original builds an empty list and still reads the
  first (uninitialised) stack slot; the Rust helpers return without writing
  instead of dereferencing the zero-filled slot. This is the only place the
  body deviates from the letter of the instruction stream, and it is recorded
  as `// observed:` in the helper.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8025FE30` | `fn_8025fe30` | 1,940 | medium |

## Deliberate skips

None. This closes the bucket at 163 of 163.

The recorded 1,940-byte extent ends just before the `addi r1,r1,1440` at
`0x802605C4`; its last included instruction is the `mtlr` at `0x802605C0`. The
ordinary `addi r1,r1,1440` / `blr` pair that follows is the function's
two-instruction epilogue and lies outside the size Ghidra recorded. The Rust
body has no stack frame, so it is represented by the function's return and not
by a separate translation step.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`
  (1 of 1; re-read for this pass, not copied from prose).
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<address>` body in
  `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` finishes without errors; it reports no
  warning for `src/runtime/os.rs`, and the remaining warnings belong to other
  lanes (`src/runtime/bt.rs`, `src/runtime/cpp.rs`, `src/render/gx_band.rs`,
  `src/runtime/input.rs`, `src/ui/menu.rs`).
- `node tools/check-lane-csv.mjs wave116` prints `OK`.
