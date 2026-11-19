# Coverage - Wave 98 (`game::enemies`, the `0x80A00000` bucket)

## How to read this file

This page records the Wave 98 `game::enemies` lane: the complete `basabasa`
actor of the `0x80A00000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs` (section 79 of that file). Every row below is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave98-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. The
`basabasa` name comes from the model strings in the module data section
(`g3d/basabasa.brres`, `basabasa_wait`, `basabasa_fly`), not from a surviving
class symbol.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 5,892 | 3 | 21 |

Two rows are `low` confidence: the 48-byte argument-swapping forwarders
`0x80A0A360` and `0x80A0A390`, whose hidden-`r12` this-adjust pointer is not
modelled (the same limitation recorded for the `0x802DCBFC` copies in waves 56,
75, 80, 81, 82 and 97).

The bucket holds 212 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 120 were already in
`ledger/functions.csv` before this pass, 26 are claimed here and 66 remain.

## What this pass covers

One complete actor, in address order, `0x80A0A240..0x80A0C470`:

- **The shell.** The free helper `0x80A0A240` (identical in shape to
  [`fn_80b091f0`]), the two `0x802DCBFC` forwarders `0x80A0A360`/`0x80A0A390`,
  the `0x8A8`-byte constructor `0x80A0A3C0` with class vtable `0x80B348E8`, and
  the free helper `0x80A0B5A0`.
- **The three model binders.** `0x80A0A6C0` resets the `+0x524` block and runs
  the wait (`0x80A0A730`), fly (`0x80A0A8A0`) and eye (`0x80A0AA10`) binders.
  The wait model `basabasa`/`basabasa_wait` lands in `+0x540`, the fly model in
  `+0x544`, the eye model `basa_eye_light`/`eye_wait` in `+0x548`. The three
  bodies are field-for-field copies of one another; the only textual difference
  is that the fly binder's third lookup reads `+0x540` (the wait model), which
  is reproduced literally and called out in its doc block.
- **The placements.** `0x80A0ADC0` rebuilds the `+0x7C` matrix from
  `+0xAC..+0xB4`, rotates by `+0x102`, offsets by `8.0`, rotates by `+0x100`,
  offsets by `-8.0`, rotates by `+0x104`, and commits to the fly holder;
  `0x80A0B080` is the wait twin with the `4.0`/`-4.0` offsets. Both place the
  eye model at `7250.0` height when `+0x750` is set and `+0x390` is clear; the
  fly copy also has a branch that copies the fly model's joint-2 matrix when the
  actor carries state `0x80B6EC98`.
- **The small matrix readers.** `0x80A0B210`, `0x80A0B260` and `0x80A0B2B0`
  copy one holder joint matrix into the actor's `+0x770`/`+0x764` scratch.
- **The state steps.** `0x80A0B5E0` (fly clip attach), `0x80A0B690` (the
  `+0x414` hit record fed by `0x80096240` and the `0x8042C1B0` table),
  `0x80A0B770` (side classifier), `0x80A0B7F0` (player-slot latch),
  `0x80A0B910` (wait bind gated on `0x80B6ED58`), `0x80A0BA80` (fly frame
  handler with the `80.0`/`128.0` grab radius), `0x80A0BBE0` (unconditional
  wait bind plus `+0x360 = 1`), `0x80A0BEF0` (dive entry, sound `0x1F0`),
  `0x80A0BFE0` (dive frame handler), `0x80A0C160` (fly entry with the side
  tables at `0x80B2AC00`/`0x80B2AC08`), `0x80A0C2D0` (descent and side re-pick)
  and `0x80A0C470`.

### Identity evidence

- **No class name survives.** The constructor is referenced only from the class
  vtable at `0x80B348E8`, whose slots point at `0x80A0A530`, `0x80A0AB20`,
  `0x80A0AB70`, `0x80A0ACB0`, `0x80A0ACC0`, `0x80A0A3C0` and `0x80A0C8D0`; no
  `daEn*_c` string can be tied to them. The actor is described by size, vtable
  and field offsets only.
- **The strings are literal.** `g3d/basabasa.brres`, `basabasa`,
  `basabasa_wait`, `wait`, `basabasa_fly`, `fly`, `basa_eye_light`, `eye_wait`,
  `eye_fall` and `eye_fly` were read from `work/rels/elf/d_enemiesNP.elf` at
  `0x80B34864..0x80B348D8` and are cited by address in the source.
- **Every constant was dumped, not assumed.** `0x80B2AC00` is `1.0`, `-1.0`;
  `0x80B2AC08` is the halfword pair `0x1560`, `-0x1560`; `0x80B2AC2C..0x80B2AC50`
  is `32.0`, `24.0`, `1.0`, `0.0`, `8.0`, `-8.0`, `7250.0`, `16.0`, `4.0`,
  `-4.0`; `0x80B2AC64` is `2.0`; `0x80B2AC6C..0x80B2AC84` is
  `0.0`, `-4.0`, `-0.1875`, `0x80357BF4`, `-1`, `-1`, `0xFF0000`;
  `0x80B2AC88..0x80B2ACA0` is `80.0`, `128.0`, `-2.0`, `40.0`, `56.0`,
  `-0.08`, `0.03125`.
- **The state words are BSS.** `0x80B6EC58`, `0x80B6EC98`, `0x80B6ECD8`,
  `0x80B6ED18` and `0x80B6ED58` lie in the module BSS segment
  (`0x80B6E500`..`0x80B799E0`, `p_filesz = 0`), so they are entered by address
  and their contents are not claimed.

### Deliberate skips

- **`0x80A0B8A0` (108 bytes) and `0x80A0BD20` (460 bytes).** The target search
  at `0x80A0B8A0` exports as a two-argument recursive call to itself even though
  its prototype takes one argument; neither the loop nor the chaining call to
  `0x80162D50(0x83, previous)` can be read honestly from that text.
  `0x80A0BD20` is its only caller inside the run and would need the
  untranslated signature, so both were left for a pass that can read the
  disassembly.
- **Six vtable-only bodies.** `0x80A0A530`, `0x80A0AB20`, `0x80A0AB70`,
  `0x80A0ACB0`, `0x80A0ACC0` and `0x80A0C8D0` are referenced from the class
  vtable but have no row in `functions.tsv`, so no byte count exists for them
  and the lane rule (size from `functions.tsv`) cannot be satisfied.
- **The rest of the bucket: 66 functions.** The named next runs are
  `0x80A00200`/`0x80A00240`, `0x80A00D00` (424 bytes), `0x80A03FF0` (708
  bytes), `0x80A05000` (616 bytes), the untranslated block
  `0x80A08CB0..0x80A09880`, `0x80A0D5B0..0x80A0E580` (including the 968-byte
  `0x80A0E580`) and `0x80A0F490..0x80A0FFD0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A0A240` | `fn_80a0a240` | 88 | high |
| `0x80A0A360` | `fn_80a0a360` | 48 | low |
| `0x80A0A390` | `fn_80a0a390` | 48 | low |
| `0x80A0A3C0` | `fn_80a0a3c0` | 360 | high |
| `0x80A0A6C0` | `fn_80a0a6c0` | 100 | medium |
| `0x80A0A730` | `fn_80a0a730` | 368 | medium |
| `0x80A0A8A0` | `fn_80a0a8a0` | 368 | medium |
| `0x80A0AA10` | `fn_80a0aa10` | 260 | medium |
| `0x80A0ADC0` | `fn_80a0adc0` | 548 | medium |
| `0x80A0B080` | `fn_80a0b080` | 400 | medium |
| `0x80A0B210` | `fn_80a0b210` | 68 | medium |
| `0x80A0B260` | `fn_80a0b260` | 68 | medium |
| `0x80A0B2B0` | `fn_80a0b2b0` | 68 | medium |
| `0x80A0B5A0` | `fn_80a0b5a0` | 64 | medium |
| `0x80A0B5E0` | `fn_80a0b5e0` | 164 | medium |
| `0x80A0B690` | `fn_80a0b690` | 212 | medium |
| `0x80A0B770` | `fn_80a0b770` | 128 | medium |
| `0x80A0B7F0` | `fn_80a0b7f0` | 164 | medium |
| `0x80A0B910` | `fn_80a0b910` | 340 | medium |
| `0x80A0BA80` | `fn_80a0ba80` | 352 | medium |
| `0x80A0BBE0` | `fn_80a0bbe0` | 296 | medium |
| `0x80A0BEF0` | `fn_80a0bef0` | 224 | medium |
| `0x80A0BFE0` | `fn_80a0bfe0` | 384 | medium |
| `0x80A0C160` | `fn_80a0c160` | 344 | medium |
| `0x80A0C2D0` | `fn_80a0c2d0` | 356 | medium |
| `0x80A0C470` | `fn_80a0c470` | 72 | medium |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 26 sizes sum to
  5,892 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 26 was
  already in `ledger/functions.csv` (the bucket's 120 existing rows are
  untouched).
- `node tools/check-lane-csv.mjs wave98` reports `OK`; this lane's CSV has 26
  rows, 7 fields each.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace build itself is currently stopped by
  errors in `src/runtime/os.rs`, `src/runtime/dvd.rs`, `src/render/gx_band.rs`
  and `src/game/bases.rs` (other Wave 98 lanes), which do not name this file.
