# Coverage - Wave 104 (audio::seq)

## How to read this file

Every row is one `pub unsafe fn` added to `src/audio/seq.rs` by this pass. The
address is the original DOL virtual address, the byte count comes from
`work/ghidra/main-gb/export/functions.tsv`, and the evidence path is the Ghidra
C export the body was reconstructed from. `high` means every load, guard, store
and call is literal in that export; `medium` means at least one field role,
callee argument or save-stub half is inferred and marked as such in the
function's doc block; `low` means only the instruction sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/audio/seq.rs` | `audio::seq` | 13 | 5388 | 0 | 13 |

## What this pass covers

- **Control-object seek and drain, `0x80271E30..0x802720A0` (2 bodies).**
  `0x80271E30` recomputes the step length from the `+0xD4` scale and the
  `+0xEA`/`+0xEC` beat fields, advances `0x80271BB0` until the caller's target
  is consumed, and stores the leftover fraction at `+0xD8`. `0x802720A0`
  releases every track through `0x80273430`/`0x80273AE0` and walks the play
  clock down for at most `0x300` iterations. Both share the registration and
  track teardown extracted as `seq_control_teardown`.
- **Node-mix polar helpers, `0x80274630..0x80274B90` (4 bodies).** `0x80274B90`
  is the shared converter: it clamps a three-float direction, normalises x/z,
  and writes `atan2(x, -z)` plus the length ratio. The three callers turn that
  result into a ramp (`0x80274630`), a seven-segment fade (`0x80274A10`) and a
  ten-segment fade with a cosine mix (`0x802746D0`).
- **Archive-loader constructor, `0x80277050` (1 body).** Stores the archive
  handle at `+0x18`, clears the stream pointer at `+0x21C`, initialises the
  `+0x54` mutex and returns the block.
- **Sequence start-up and wait, `0x8027B360..0x8027B520` (2 bodies).**
  `0x8027B360` is the one-shot bring-up: sound driver, output mode, track table,
  start tick, three clock managers, DMA descriptor and thread start. `0x8027B520`
  is the companion wait for the manager word at `+0x50` and one time-base tick.
- **Element block teardown and ticks, `0x8027D160..0x8027EAF0` (4 bodies).**
  `0x8027D160` releases the eight fixed voice slots, both element tables, the
  `0x34`-byte slot array and the `+0x808` object. `0x8027E6A0` advances the
  row/column counters and re-sends voice positions, `0x8027E8A0` does the same
  for one column with a slot restart, and `0x8027EAF0` pops and fills a
  `0x34`-byte element record before registering it.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80271E30` | `fn_80271e30` | 612 | medium |
| `0x802720A0` | `fn_802720a0` | 660 | medium |
| `0x80274630` | `fn_80274630` | 148 | medium |
| `0x802746D0` | `fn_802746d0` | 828 | medium |
| `0x80274A10` | `fn_80274a10` | 384 | medium |
| `0x80274B90` | `fn_80274b90` | 284 | medium |
| `0x80277050` | `fn_80277050` | 60 | medium |
| `0x8027B360` | `fn_8027b360` | 428 | medium |
| `0x8027B520` | `fn_8027b520` | 116 | medium |
| `0x8027D160` | `fn_8027d160` | 456 | medium |
| `0x8027E6A0` | `fn_8027e6a0` | 508 | medium |
| `0x8027E8A0` | `fn_8027e8a0` | 348 | medium |
| `0x8027EAF0` | `fn_8027eaf0` | 556 | medium |

## Deliberate skips

- **`0x802741E0` (316 bytes) and `0x80274480` (432 bytes).** Both decode their
  source vector through `unaff_GQR0`, the paired-single quantisation format
  register. The format is not an ordinary argument, so the byte/half-word/float
  branches cannot be selected from the export. Reproducing only the float path
  would invent behaviour for every other value; no body is claimed.
- **`0x80273560` (1400 bytes) and `0x8027E080` (804 bytes).** Their exports read
  `in_f21..in_f31` and `in_ps21_1..in_ps31_1`, register pairs the caller never
  materialises as arguments. Only the outward call list is certain, so they are
  left for a pass with the assembly.
- **`0x80274320` (352 bytes).** Seven incoming `f25..f31` double pairs and their
  paired-single halves feed a stack array; the ABI does not expose them as
  parameters in the export. The two downstream fade calls are visible, but the
  seven inputs are not.
- **`0x80273DD0` (1032 bytes).** The body builds a `0x20`-byte local descriptor
  from fields whose roles the export does not print, passes it through
  `0x80272340`, then edits a voice object through six small setters. The
  descriptor layout and several callee argument lists cannot be recovered from
  the export without guessing.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/audio/seq.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/audio/seq.rs`; the tree still contains unrelated errors in other wave-104
  lanes, which are ignored by this lane.
- `node tools/check-lane-csv.mjs wave104` reports no problem for
  `wave104-seq.csv`.
