# Coverage - Wave 83 (enemies: the `0x80A00000` bucket descriptors)

## How to read this file

This page records the Wave 83 `game::enemies` lane: a pass over the
`0x80A00000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to one real
`pub unsafe fn fn_<addr>` body in that file, and the evidence column of the
machine-readable mirror (`ledger/agents/wave83-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants are cited by address and read through `platform::read_u32` /
`read_f32` / `read_u16` / `read_u8`, never transcribed as invented values.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 2,976 | 16 | 4 |

Eight rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80A05C60`, `0x80A05C90`, `0x80A08400`, `0x80A08430`, `0x80A0D2F0`,
`0x80A0D320`, `0x80A0F1C0` and `0x80A0F1F0`, whose hidden-`r12` this-adjust
pointer is not modelled (the same limitation already recorded for the
`0x802DCBFC` copies in waves 75, 80, 81 and 82).

The bucket holds 212 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 92 were already in
`ledger/functions.csv` before this pass, 28 are claimed here and 92 remain.

## What this pass covers

Three clusters, plus three leftovers inside an already-translated block.

**Cluster 1, `0x80A08120..0x80A086D0`, the `lift_taru` lift.** The run opens
with the freeing helper, the five-link `+0x30` chain walker and the `:`-suffix
name predicate, then the two 48-byte forwarders and the `0x5D8`-byte
constructor with class vtable `0x80B34320`. It closes with the dispose helper
that tears the object down through `0x800DAE00` instead of the ordinary
`0x8015F800`, and the binder that opens `g3d/lift_taru.brres`.

**Cluster 2, `0x80A0CF90..0x80A0D350`, a `0x618`-byte actor shell.** Two
freeing helpers, two constant producers returning the shared main.dol words
`0x80357BF4` and `0x80357BB4`, the five-link chain walker, the name predicate,
two forwarders, and the constructor whose class vtable is `0x80B34E08`. The
constructor builds the `+0x524` transform, the `+0x544` matrix holder, the
`+0x584` animation holder, and the owner-interface pair `0x803295D8` ->
`0x80329548` at `+0x5BC`.

**Cluster 3, `0x80A0EE70..0x80A0F220`, a `0x938`-byte actor shell.** A
single-producer copy of the same shell (only `0x80357BF4` this time) followed by
the constructor whose class vtable is `0x80B35238`. That constructor is the
largest body of the pass: it copies the spawn position `+0xAC..+0xB4` into
`+0x5C4..+0x5CC`, initialises the two `0x8008C200` sub-objects at `+0x608` and
`+0x6AC`, installs the `+0x7BC` holder and the `+0x7F4` transform object, and
calls the rig helper `0x800B6830` with the float at `0x8042C5C8`.

**Leftovers, `0x80A059D0`/`0x80A05C60`/`0x80A05C90`.** The wave-3 Buku block
(`0x80A05500..0x80A05CC0`) left three addresses between its own translated
entries: the `DieFall` extra-state-word producer and the two 48-byte
forwarders. They are included here so that the block has no holes.

### Identity evidence

- **`lift_taru` is read, not guessed.** The export names the strings
  `s_lift_taru_80b34310` (`lift_taru`) and `s_g3d_lift_taru_brres_80b342fc`
  (`g3d/lift_taru.brres`) inside `0x80A086D0`, and both are loaded by the
  binder itself.
- **The shared words are main.dol data.** `0x80357BF4` and `0x80357BB4` are
  also returned by main.dol accessors `0x800268B0`/`0x800268C0` and appear in
  `0x80A059D0`'s call site inside the already-translated
  `register_buku_states`. Their fields are `// unknown:`.
- **The two unnamed constructors load no string.** They are therefore
  described by allocation size, class vtable and the offsets their own code
  initialises; no class name is invented.
- **The `+0x30` walkers are one shape.** `0x80A0D150`, `0x80A08260` and
  `0x80A0F020` are byte-equivalent to the already-translated `fn_80a05ac0`,
  and the three name predicates are byte-equivalent to `fn_80a05ba0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A059D0` | `fn_80a059d0` | 12 | high |
| `0x80A05C60` | `fn_80a05c60` | 48 | low |
| `0x80A05C90` | `fn_80a05c90` | 48 | low |
| `0x80A08120` | `fn_80a08120` | 92 | high |
| `0x80A08260` | `fn_80a08260` | 224 | high |
| `0x80A08340` | `fn_80a08340` | 136 | medium |
| `0x80A08400` | `fn_80a08400` | 48 | low |
| `0x80A08430` | `fn_80a08430` | 48 | low |
| `0x80A08460` | `fn_80a08460` | 108 | high |
| `0x80A084D0` | `fn_80a084d0` | 88 | high |
| `0x80A086D0` | `fn_80a086d0` | 172 | high |
| `0x80A0CF90` | `fn_80a0cf90` | 88 | high |
| `0x80A0CFF0` | `fn_80a0cff0` | 92 | high |
| `0x80A0D050` | `fn_80a0d050` | 12 | high |
| `0x80A0D060` | `fn_80a0d060` | 12 | high |
| `0x80A0D150` | `fn_80a0d150` | 224 | high |
| `0x80A0D230` | `fn_80a0d230` | 136 | medium |
| `0x80A0D2F0` | `fn_80a0d2f0` | 48 | low |
| `0x80A0D320` | `fn_80a0d320` | 48 | low |
| `0x80A0D350` | `fn_80a0d350` | 184 | high |
| `0x80A0EE70` | `fn_80a0ee70` | 88 | high |
| `0x80A0EED0` | `fn_80a0eed0` | 92 | high |
| `0x80A0EF30` | `fn_80a0ef30` | 12 | high |
| `0x80A0F020` | `fn_80a0f020` | 224 | high |
| `0x80A0F100` | `fn_80a0f100` | 136 | medium |
| `0x80A0F1C0` | `fn_80a0f1c0` | 48 | low |
| `0x80A0F1F0` | `fn_80a0f1f0` | 48 | low |
| `0x80A0F220` | `fn_80a0f220` | 460 | medium |

## Deliberate skips

- **`0x80A0F220` is `medium`, not `high`.** Ghidra prints the frame-exit call
  `0x802DCDC0` outside the allocation guard, so the export reaches it with a
  possibly null register; the body preserves that order. The rig argument is
  printed as a `double` because the callee takes a promoted float, so the body
  re-reads the 32-bit float at `0x8042C5C8` and promotes it.
- **`0x80A086D0` passes three uninitialised words.** Only `local[0]` is written
  before the six-argument holder commit; the other three words are whatever the
  stack held. The sequence is literal and is recorded as such.
- **The rest of the bucket.** The remaining 92 bucket addresses are the large
  untranslated actor blocks above `0x80A09xxx` (`0x80A0A3C0`, `0x80A0ADC0`,
  `0x80A0B5A0..0x80A0B910`, `0x80A0BD20`, `0x80A0C160`, `0x80A0D5B0`,
  `0x80A0DCC0`, `0x80A0DF00`, `0x80A0E580` at 968 bytes, `0x80A0F610` at
  660 bytes) and the small `0x80A07C80..0x80A07C90` and `0x80A09xxx` pieces;
  they lie outside this lane's 28-body budget.
- **`0x80A01700`** appears in `ledger/functions.csv` but not in this
  export's `functions.tsv`; it is untouched by this pass.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  2,976 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv` (the bucket's 92 existing rows are untouched).
- `node tools/check-lane-csv.mjs wave83` reports 28 rows for this CSV and `OK`
  for the wave.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
