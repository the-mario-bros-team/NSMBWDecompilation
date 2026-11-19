# Coverage - Wave 52 (audio::seq, RSAR vtable forwarders and table resolvers)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export the body was reconstructed from. `high` means the loads, guards and
stores are literal in that export; `medium` means at least one forwarded
argument or field role is inferred and marked as such in the function's doc
block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 30 | 3064 | 9 | 21 |

Bytes, High and Medium counts cover only the 30 bodies added in this pass.

## What this pass covers

- **RSAR vtable forwarders, `0x80275750..0x80275840` (15 bodies).** Each entry
  loads the wrapper's archive pointer from `+0x04` into `r3` and tail-branches
  into one resolver. The branch leaves `r4`..`r7` untouched, so the wrapper
  signatures mirror the callee signatures. `0x80275820` is the clean
  one-argument case; the rest forward an id, an id plus an output block, or two
  ids plus an output block.
- **RSAR table resolvers, `0x80276100..0x80276BE0` (15 bodies).** These are the
  functions the forwarders and the wave-45 load-by-id paths call. Every one
  reads a byte/offset pair out of the archive info block at `this + 0x28`,
  resolves it through the shared table helper `0x80283A60`, bounds-checks the
  caller's id against the table's first word, and then decodes the entry.
  Versions before `0x101` name entries with a base-relative offset; newer
  versions use the byte/offset pair again. The type-1/2/3 decoders additionally
  go through `0x80276F50`, which is left for a later pass and called by address.
- The cluster completes the archive front end begun in wave 45: the same
  `RSAR` magic and header, now followed through the entry-kind decoders
  (`0x80275EE0`), the six-word and five-word record readers
  (`0x80276660`/`0x80276770`), the seven-word info header (`0x802768B0`) and the
  `+0x30` pointer-table resolver (`0x802769D0`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80275750` | `fn_80275750` | 16 | medium |
| `0x80275760` | `fn_80275760` | 8 | medium |
| `0x80275770` | `fn_80275770` | 8 | medium |
| `0x80275780` | `fn_80275780` | 8 | medium |
| `0x80275790` | `fn_80275790` | 8 | medium |
| `0x802757A0` | `fn_802757a0` | 8 | medium |
| `0x802757B0` | `fn_802757b0` | 8 | medium |
| `0x802757C0` | `fn_802757c0` | 8 | medium |
| `0x802757E0` | `fn_802757e0` | 8 | medium |
| `0x802757F0` | `fn_802757f0` | 8 | medium |
| `0x80275800` | `fn_80275800` | 8 | medium |
| `0x80275810` | `fn_80275810` | 8 | medium |
| `0x80275820` | `fn_80275820` | 8 | high |
| `0x80275830` | `fn_80275830` | 8 | medium |
| `0x80275840` | `fn_80275840` | 8 | medium |
| `0x80276100` | `fn_80276100` | 268 | medium |
| `0x80276210` | `fn_80276210` | 228 | medium |
| `0x80276300` | `fn_80276300` | 260 | medium |
| `0x80276410` | `fn_80276410` | 212 | medium |
| `0x802764F0` | `fn_802764f0` | 172 | high |
| `0x802765A0` | `fn_802765a0` | 184 | high |
| `0x80276660` | `fn_80276660` | 272 | medium |
| `0x80276770` | `fn_80276770` | 312 | high |
| `0x802768B0` | `fn_802768b0` | 132 | high |
| `0x80276940` | `fn_80276940` | 64 | high |
| `0x80276980` | `fn_80276980` | 68 | high |
| `0x802769D0` | `fn_802769d0` | 260 | medium |
| `0x80276AE0` | `fn_80276ae0` | 184 | high |
| `0x80276BA0` | `fn_80276ba0` | 64 | high |
| `0x80276BE0` | `fn_80276be0` | 256 | medium |

## Deliberate skips

- `0x80276CE0`, `0x80276E00` and `0x80276F50` are the remaining three entries of
  the `0x80276100..0x80276F50` resolver run. They are not required by the 15
  decoders landed here (`0x80276CE0` and `0x80276E00` are reached only through
  the wave-45 forwarders and `0x80276F50` is called by address), and stopping at
  30 bodies keeps this lane inside the requested 25-30 range. All three are
  ordinary, tractable functions for a later pass.
- `0x802700F0`, `0x80270100`, `0x80270110`, `0x80270310`, `0x80270920`,
  `0x80270AD0` and `0x80270C20` sit before the `RSEQ` validator. Their object is
  the four-slot JAS voice band at `0x803FC9C8` (four `0xA8`-byte slots, an
  `unaff_r13` initialisation sentinel) rather than the sequence-control object;
  `0x80270920` also calls `0x80270B70`, which Ghidra did not split into its own
  export. They belong to an AX/JAS voice pass, not this sequence lane.
- `0x80271E30` and `0x802720A0` are the two tempo/play-clock functions between
  the wave-38 player bodies. Both are dominated by `unaff_GQR0` paired-single
  loads and `ldexpf` conversions; reconstructing the fixed-point scaling without
  the PowerPC paired-single instructions would be a guess.
- `0x80273560`, `0x80273DD0`, `0x80274B90`, `0x80274CE0`, `0x80275850` and the
  `0x802741E0..0x80274A10` band remain from wave 45's skip list: large
  dispatchers or paired-single audio-rate arithmetic. `0x80274CE0` additionally
  calls the unexported `0x802741E0`/`0x80274320`/`0x80274480`/`0x80274630`
  helpers.
- The `0x8027Axxx` and later blocks in the `0x8027` bucket belong to other
  subsystems and were not touched.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/audio/seq.rs`.
- `node tools/check-lane-csv.mjs wave52` reports `OK` for this CSV.
