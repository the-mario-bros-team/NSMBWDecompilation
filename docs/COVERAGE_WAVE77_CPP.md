# Coverage - Wave 77 (runtime::cpp)

## How to read this file

Each row is one `pub unsafe fn fn_<address>` body appended to
`src/runtime/cpp.rs` by this pass. The `bytes` column is the exact size from
`work/ghidra/main-gb/export/functions.tsv`; the body's `# Original` block cites
the same file. Confidence is the same value as the body doc block.

Roles that the Ghidra export does not prove are marked `// inferred:` in the
bodies. Field offsets are relative to the first argument unless stated
otherwise.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/cpp.rs` | `runtime::cpp` | 26 | 2848 | 18 | 8 |

## What this pass covers

Address range: `0x802D0810..0x802D7170`.

- **SDA addressing.** `0x802D0810` and `0x802D0820` are two-instruction `addi`
  getters: they return an SDA2 address (`0x8042F52C`, `0x8042FC38`) rather than
  the value stored there. `0x802D25F0`/`0x802D2600` read bit 2 and bit 3 of one
  SDA latch word (`0x80429B80`, `r13 - 0x5B00`).
- **The `+0x40..+0x84` member block.** `0x802D0E50` is the copy constructor and
  `0x802D1140` the copy assignment for the same 9-word-pair block. They differ
  in exactly one store -- the vtable word at `+0x38` -- which is the evidence
  for telling them apart. `0x802D1820` walks the `+0x3C` chain looking for the
  dirty bit, `0x802D17C0` rebuilds the transform through `0x802D12E0` and
  publishes the swapped SDA scratch pair at `+0x68`/`+0x6C`, and `0x802D1D70` is
  the smallest vtable installation in the band.
- **The 32-byte small-block allocator.** `0x802D35C0` is the allocation half of
  the scheme whose `free` side already existed in this file. It rounds the
  request up to a 32-byte multiple, maintains the `r13 - 0x4718` / `-0x4714`
  byte counters and the `r13 - 0x4728` / `-0x4724` free and retired lists, and
  splits or retires the chosen block. All four SDA cells are resolved to
  absolute addresses in named constants.
- **View teardown and the fourth submit predicate.** `0x802D3E00` closes the
  `+0x4C4` view by state (`+0x4CC` = 1, 2 or 3/4 selects a different child),
  and `0x802D3E90` is the fourth copy of the Wave 65 submit predicate with the
  `0x80277A90` ready check inlined. `0x802D38A0` releases the three children at
  `+0x374`, `+0x198`, `+0x0C`; `0x802D3940` dispatches to vtable `+0x10`/`+0x14`;
  `0x802D3920` stores the `+0x4D0` slot while the object is idle.
- **The three-slot effect table (`0x802D4C00..0x802D50A0`).** One self-contained
  sub-system: a table of three 0x10-byte slots, each holding a loader handle
  (`+0x0C`), a slot id (`+8`) and a chain of records. `0x802D4C00` releases a
  slot, `0x802D4C70` binds a loaded resource, `0x802D4CD0` dispatches to one of
  eight per-mode builders, `0x802D4DA0` shuts a slot down, `0x802D4F00` /
  `0x802D4F60` build and destroy the table through the existing array helpers
  `0x802DC9A0` / `0x802DCA98`, `0x802D4FD0` binds up to three resources, and
  `0x802D50A0` returns the table address `0x80424AF0`. The loader key is the
  ASCII tag `FX A` (`0x46582041`) bumped per slot -- the only string evidence in
  the family.
- **`0x802D7170`** busy-waits for a millisecond count against the tick reader
  `0x801B5FA0`, using `0x800000F8 / 4000` as ticks per millisecond.
- **New constants.** `SDA2_BASE`, `SDA_FLOAT_QUAD`, `BLOCK_BYTES_LEFT`,
  `BLOCK_BYTES_LOW`, `BLOCK_FREE_HEAD`, `BLOCK_RETIRED_HEAD`,
  `SDA_TRANSFORM_SCRATCH`, `SDA_FEATURE_LATCH`, `EFFECT_SLOT_TABLE`,
  `EFFECT_SLOT_STRIDE`, `EFFECT_SLOT_COUNT` and `EFFECT_SLOT_CONSTRUCTOR`. No
  existing constant was renamed or reused.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x802D0810` | `fn_802d0810` | 8 | high |
| `0x802D0820` | `fn_802d0820` | 8 | high |
| `0x802D0E50` | `fn_802d0e50` | 228 | medium |
| `0x802D1140` | `fn_802d1140` | 208 | medium |
| `0x802D1280` | `fn_802d1280` | 44 | high |
| `0x802D12B0` | `fn_802d12b0` | 36 | high |
| `0x802D17C0` | `fn_802d17c0` | 92 | medium |
| `0x802D1820` | `fn_802d1820` | 44 | high |
| `0x802D1D70` | `fn_802d1d70` | 24 | high |
| `0x802D25F0` | `fn_802d25f0` | 12 | high |
| `0x802D2600` | `fn_802d2600` | 12 | high |
| `0x802D35C0` | `fn_802d35c0` | 304 | medium |
| `0x802D38A0` | `fn_802d38a0` | 116 | high |
| `0x802D3920` | `fn_802d3920` | 32 | high |
| `0x802D3940` | `fn_802d3940` | 100 | medium |
| `0x802D3E00` | `fn_802d3e00` | 140 | high |
| `0x802D3E90` | `fn_802d3e90` | 204 | high |
| `0x802D4C00` | `fn_802d4c00` | 108 | medium |
| `0x802D4C70` | `fn_802d4c70` | 84 | high |
| `0x802D4CD0` | `fn_802d4cd0` | 200 | high |
| `0x802D4DA0` | `fn_802d4da0` | 352 | medium |
| `0x802D4F00` | `fn_802d4f00` | 92 | medium |
| `0x802D4F60` | `fn_802d4f60` | 100 | high |
| `0x802D4FD0` | `fn_802d4fd0` | 168 | high |
| `0x802D50A0` | `fn_802d50a0` | 12 | high |
| `0x802D7170` | `fn_802d7170` | 120 | high |

## Deliberate skips

- `0x802D5080`, `0x802D5090` - the two `fn_802d4cd0(slot, mode)` wrappers. The
  export prints the call with only two of the callee's three arguments, so the
  third word cannot be recovered; filling in `0` would be a guess.
- `0x802D32F0` - its body opens with `FUN_802c0240()` printed with no
  arguments, so the object the following byte stores belong to is unrecoverable
  from this evidence (same register-argument loss as the Wave 65 skips).
- `0x802D3700` - calls `FUN_802d3590()` with no arguments to obtain the object it
  then fills in; `0x802D3590`'s own body in this file is still a stub, so the
  record layout cannot be established from these two exports together.
- `0x802D0D90`, `0x802D1010`, `0x802D1850`, `0x802D6F20`, `0x802D7070` - the
  `GQR0` / `ldexpf` / `CONCAT44` artefact family documented in
  `docs/QUALITY.md` section 5.3; no honest floating-point reconstruction.
- `0x802D0AE0`, `0x802D12E0`, `0x802D1E20`, `0x802D2310`, `0x802D2610`,
  `0x802D2730`, `0x802D2850`, `0x802D2990`, `0x802D39B0`, `0x802D3B50`,
  `0x802D3CE0`, `0x802D42D0`, `0x802D4830`, `0x802D48C0`, `0x802D71F0`,
  `0x802D7410`, `0x802D74C0`, `0x802D7570`, `0x802D7680`, `0x802D7860`,
  `0x802D79A0`, `0x802D9570`, `0x802DADD0`, `0x802DB6B0..0x802DB970` - not
  attempted this pass; no body was written and no ledger row was created.
- `0x802DCCCC..0x802DCDC0`, `0x802DCD40..0x802DCDBC` - the `__savegprlr` /
  `__restgprlr` register-save family, 4-byte alternate entry points into one
  `blr`. Previously excluded for the same reason.
- `0x802DCDD8..0x802DD210` - the 64-bit divide/modulo/shift family whose exports
  expand into `CARRY4` / `CONCAT44` chains. Previously excluded.
- `0x802D2F10`, `0x802D3260`, `0x802DEBB4`, `0x802DB0E0` - 4-byte `b` thunks and
  one empty body; not callable high-level routines.
- `0x802DFF64` and above - the `%a`/`%f` formatter, still blocked on the
  paired-single artefacts.

## Pre-existing bodies without a ledger row

Two addresses in this range already had complete bodies in `src/runtime/cpp.rs`
from an earlier pass, but were never given a row in `ledger/functions.csv`:

- `0x802DCC4C` - saturated `double` to `u32` conversion, body at line 804.
- `0x802DF42C` - Metrowerks `__fpclassify`, body at line 2296.

Both are real translations that already cite their evidence and byte sizes, so
adding rows for them is a coordinator-side decision. This lane did not duplicate
or claim them.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/cpp.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error in `src/runtime/cpp.rs`.
- `node tools/check-lane-csv.mjs wave77` reports `wave77-cpp.csv: 26 rows` and
  no problem naming this CSV.
