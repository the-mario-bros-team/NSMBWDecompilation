# Coverage - Wave 93 (`game::enemies`, the tail of the `0x80A40000` bucket)

## How to read this file

This page records the Wave 93 `game::enemies` lane: 30 functions of the
`0x80A40000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below is one real `pub unsafe fn fn_<addr>`
body in that file, and the `evidence` column of the machine-readable mirror
(`ledger/agents/wave93-enemies.csv`) names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. The two
float thresholds used by `fn_80a4f250` were read from
`work/rels/elf/d_enemiesNP.elf` at the cited synthetic addresses with a small
ELF program-header reader, never guessed from the C text.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 2,952 | 7 | 15 | 8 |

The bucket holds 189 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 81 were already in
`ledger/functions.csv` before this pass, in the wave-7 `daEnCoin_c` block
(`0x80A41DC0..0x80A43F40`), the wave-59 run (`0x80A447D0..0x80A46CC0`), the
wave-77 runs (`0x80A401D0..0x80A40740` and `0x80A4A370..0x80A4CBA0`) and a
handful of named helpers such as `register_crasher_states` at `0x80A49C60` and
`0x80A41180`. 30 are claimed here and 78 remain.

## What this pass covers

The `0x80A40000` bucket from `0x80A4D590` to `0x80A4FC30`. The run is built
around four further copies of the five-entry state-descriptor skeleton that
wave 77 opened at `0x80A4C7E0..0x80A4CBA0`, so one field map carries across
almost every body.

**Run 1, `0x80A4D590..0x80A4D8E0`, the first copy.**

- `0x80A4D590` is the deleting destructor pair member: `0x8015F800(object, 0)`
  and, for a positive mode, `0x802B9280`. `0x80A4D5F0` returns the shared
  base-class descriptor `0x80358084`.
- `0x80A4D6E0` is the `+0x30` owner-chain walker: five unrolled steps, each
  invoking the `+0xC` slot of the owner found at `object + 0x30`, then a
  recursion on the sixth owner. `0x80A4D7C0` is the `:`-suffix name predicate
  built from `0x802E1C40` (find `0x3A`) and `0x802E1AB4` (compare).
- `0x80A4D880`/`0x80A4D8B0` are the two 48-byte `0x802DCBFC` forwarders, and
  `0x80A4D8E0` allocates and constructs the `0x760`-byte actor:
  `0x801628C0(0x760)`, base constructor `0x800281C0`, class vtable
  `0x80B43744` at `+0x60`.

**Run 2, `0x80A4E0B0..0x80A4E400`, the second copy.**

- Byte-identical to run 1 with one difference: the constructor installs class
  vtable `0x80B43AFC`.

**Run 3, `0x80A4EA00..0x80A4EF80`, a predicate plus the third copy.**

- `0x80A4EA00` is 40 bytes: `0x800962A0(this, this + 0xAC)` inverted with
  `^ 1`. The helper's contract is not printed, so only the inversion is
  asserted.
- The copy then repeats the destructor, the constant producer, the chain
  walker, the name predicate and the two forwarders; its constructor
  `0x80A4EF80` installs class vtable `0x80B43EB4`.

**Run 4, `0x80A4F250..0x80A4FC30`, two helpers and the fourth copy.**

- `0x80A4F250` calls `0x80064740(actor, &record)`; a zero answer returns `1`,
  otherwise the first record word's magnitude is compared with `80.0f`
  (`0x80B2CB90`) and the second with `0.0f` (`0x80B2CB78`), which selects
  `3`/`2` when far and `1`/`0` when near.
- `0x80A4F4B0` fills an eight-float record from `0x80152480` and rewrites
  `+0xAC`/`+0xB0` as `record[3] - (stored - current)` and
  `record[7] - (stored - current)` against the `+0x5E8`/`+0x5EC` reference.
- The copy repeats the five skeleton entries and closes the bucket view with
  `0x80A4FC30`, the `0x6E8`-byte constructor. It is the same body as
  `0x80A50780` (wave 78) - `0x801628C0(0x6E8)`, `0x80094E80`, the intermediate
  vtable `0x80303F48`, the `+0x524` render block (`0x80069020`), the cleared
  `+0x540`, the `+0x544` owner (`0x80169CD0`), the `+0x584` animation holder
  (`0x80168D10`, vtable replaced by `0x80B330E0`), the `+0x600` block
  (`0x8008C200`) and the `+0x6D8` sub-object (`0x80304228` with its `+0x6DC`
  back-pointer) - except that its class vtable is `0x80B44264`.

### Identity evidence

- **No class name is claimed.** `0x80358084` also comes back from `0x80A50490`
  and `0x80AC0350`, so the constant producer names no enemy class. The four
  class vtables (`0x80B43744`, `0x80B43AFC`, `0x80B43EB4`, `0x80B44264`) are
  cited by address only; nothing in the export prints a `StateID` or a class
  string for this run.
- **The skeleton identity is structural.** The five bodies of each copy match
  the wave-77 copy at `0x80A4C7E0..0x80A4CBA0` instruction for instruction,
  including the `+0x30` walk depth and the `48`-byte forwarder size.
- **The two float thresholds were dumped, not assumed.**
  `0x80B2CB90` holds `0x42A00000` (`80.0f`) and `0x80B2CB78` holds
  `0x00000000` (`0.0f`) in `work/rels/elf/d_enemiesNP.elf`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A4D590` | `fn_80a4d590` | 92 | high |
| `0x80A4D5F0` | `fn_80a4d5f0` | 12 | high |
| `0x80A4D6E0` | `fn_80a4d6e0` | 224 | medium |
| `0x80A4D7C0` | `fn_80a4d7c0` | 136 | medium |
| `0x80A4D880` | `fn_80a4d880` | 48 | low |
| `0x80A4D8B0` | `fn_80a4d8b0` | 48 | low |
| `0x80A4D8E0` | `fn_80a4d8e0` | 76 | medium |
| `0x80A4E0B0` | `fn_80a4e0b0` | 92 | high |
| `0x80A4E110` | `fn_80a4e110` | 12 | high |
| `0x80A4E200` | `fn_80a4e200` | 224 | medium |
| `0x80A4E2E0` | `fn_80a4e2e0` | 136 | medium |
| `0x80A4E3A0` | `fn_80a4e3a0` | 48 | low |
| `0x80A4E3D0` | `fn_80a4e3d0` | 48 | low |
| `0x80A4E400` | `fn_80a4e400` | 76 | medium |
| `0x80A4EA00` | `fn_80a4ea00` | 40 | medium |
| `0x80A4EC30` | `fn_80a4ec30` | 92 | high |
| `0x80A4ED80` | `fn_80a4ed80` | 224 | medium |
| `0x80A4EE60` | `fn_80a4ee60` | 136 | medium |
| `0x80A4EF20` | `fn_80a4ef20` | 48 | low |
| `0x80A4EF50` | `fn_80a4ef50` | 48 | low |
| `0x80A4EF80` | `fn_80a4ef80` | 76 | medium |
| `0x80A4F250` | `fn_80a4f250` | 148 | medium |
| `0x80A4F4B0` | `fn_80a4f4b0` | 140 | medium |
| `0x80A4F8E0` | `fn_80a4f8e0` | 92 | high |
| `0x80A4F940` | `fn_80a4f940` | 12 | high |
| `0x80A4FA30` | `fn_80a4fa30` | 224 | medium |
| `0x80A4FB10` | `fn_80a4fb10` | 136 | medium |
| `0x80A4FBD0` | `fn_80a4fbd0` | 48 | low |
| `0x80A4FC00` | `fn_80a4fc00` | 48 | low |
| `0x80A4FC30` | `fn_80a4fc30` | 168 | medium |

## Deliberate skips

- **`0x80A4EC90` (12 bytes).** The fourth copy of the `0x80358084` constant
  producer. It is byte-identical to [`fn_80a4d5f0`], [`fn_80a4e110`] and
  [`fn_80a4f940`], which are all claimed, so the lane stops at 30 bodies rather
  than adding a fourth identical producer; a later pass can claim it with no
  new evidence.
- **Eleven `medium` bodies are `medium` because a role or a callee contract is
  inferred, not because an instruction is guessed.** The four chain walkers
  (`fn_80a4d6e0`, `fn_80a4e200`, `fn_80a4ed80`, `fn_80a4fa30`) and the four name
  predicates (`fn_80a4d7c0`, `fn_80a4e2e0`, `fn_80a4ee60`, `fn_80a4fb10`) are
  marked `medium` because the `+0xC`/`+0x20` slot arguments and the two
  `main.dol` callee names are inferred from the live `r3`; the three `0x760`
  constructors and the `0x6E8` constructor are `medium` because `0x800281C0`
  and `0x80094E80` receive the new actor only through `r3`; `fn_80a4ea00`,
  `fn_80a4f250` and `fn_80a4f4b0` are `medium` because `0x800962A0`,
  `0x80064740` and `0x80152480` are addressed but not named.
- **Eight `low` bodies** are the 48-byte `0x802DCBFC` forwarders
  (`fn_80a4d880`, `fn_80a4d8b0`, `fn_80a4e3a0`, `fn_80a4e3d0`, `fn_80a4ef20`,
  `fn_80a4ef50`, `fn_80a4fbd0`, `fn_80a4fc00`). The argument reshuffle is
  literal but neither the target's role nor the hidden `r12` this-adjust
  pointer is established; every body carries the same `// partial:` note.
- **The rest of the bucket stays untranslated: 78 functions.** After this pass
  the largest untouched runs are `0x80A40290..0x80A40FA0` (10, including the
  3,124-byte `0x80A41180`, which is claimed and therefore splits the run),
  `0x80A41E20..0x80A42150` (5), `0x80A429C0..0x80A446B0` (14),
  `0x80A47B90..0x80A47EB0` (5), `0x80A48520..0x80A498C0` (17) and
  `0x80A4B010..0x80A4BD70` (13), plus the singles `0x80A45DB0`, `0x80A46200`,
  `0x80A46BF0`, `0x80A4A8D0`, `0x80A4AC00` and `0x80A4EC90`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  2,952 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 30 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave93` prints
  `wave93-enemies.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The workspace-wide errors during this pass are all in
  `src/runtime/os.rs`, which belongs to another Wave 93 lane; a scratch copy of
  the crate with that file's `u32`/`usize` mismatches cast locally compiles
  with zero errors, and its only warnings are in `runtime::os`, `runtime::bt`,
  `runtime::cpp` and `runtime::input`.
