# Coverage - Wave 56 (`d_enemiesNP`, enemies lane)

## How to read this file

This page records the Wave 56 enemies lane: three runs inside the
`0x80AB0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below corresponds to a real `pub unsafe fn`
body in that file; the `evidence` column of the lane CSV names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 3,464 | 17 | 12 |

## What this pass covers

The pass is three runs, all read from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Addresses already in
`ledger/functions.csv` (including the Kuribo table at `0x80AB4A50`, the actor
allocator at `0x80AB5140` and the handlers at `0x80AB5E50..0x80AB6430`) were
skipped.

- **State-helper run, `0x80AB0180..0x80AB0360`.** Two animation-blend starters
  on the shared holder at `+0x584` (`1.0` and `0.0`), two position effects
  (`0x294` and `0x292`), the controller predicate behind the `+0x394` chain,
  and the six-holder destructor of the same actor. The `1.0`/`0.0` constants
  were verified byte-for-byte in the REL image (`0x80B2FA4C`/`0x80B2FA44`).
- **`lift_taru` run, `0x80AB50E0..0x80AB60A0`.** The two deleting-destructor
  forwarders, the `g3d/lift_taru.brres` model binder, the position/rotation
  matrix rebuild, the hit predicate that calls `fn_80ab5e50`, the yaw/sound
  step, the tilt/timer step, the signed angle clamp, the spawn-word facing
  selector, the shared animation/speed update used by the already-translated
  Kuribo handlers, and the idle reset. The resource names survive in the
  export at `0x80B55698`/`0x80B55684`.
- **Next-actor skeleton, `0x80AB6A50..0x80AB7730`.** A state normaliser, two
  `0x802DCBFC` forwarders, the `0xBE8`-byte constructor (four holder
  sub-objects and two vtables), its two-argument destructor forwarder, the
  arming helper for `+0x5F8`, the full destructor, two more normaliser copies,
  two BSS descriptor-address producers and the five-link `+0x30` chain walker.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AB0180` | `fn_80ab0180` | 64 | high |
| `0x80AB01C0` | `fn_80ab01c0` | 76 | high |
| `0x80AB0270` | `fn_80ab0270` | 80 | high |
| `0x80AB02C0` | `fn_80ab02c0` | 80 | high |
| `0x80AB0310` | `fn_80ab0310` | 80 | medium |
| `0x80AB0360` | `fn_80ab0360` | 164 | high |
| `0x80AB50E0` | `fn_80ab50e0` | 48 | medium |
| `0x80AB5110` | `fn_80ab5110` | 48 | medium |
| `0x80AB5350` | `fn_80ab5350` | 172 | high |
| `0x80AB54B0` | `fn_80ab54b0` | 232 | high |
| `0x80AB56D0` | `fn_80ab56d0` | 60 | medium |
| `0x80AB5B40` | `fn_80ab5b40` | 132 | medium |
| `0x80AB5BD0` | `fn_80ab5bd0` | 152 | medium |
| `0x80AB5C70` | `fn_80ab5c70` | 80 | high |
| `0x80AB5CC0` | `fn_80ab5cc0` | 104 | high |
| `0x80AB5D30` | `fn_80ab5d30` | 280 | medium |
| `0x80AB6020` | `fn_80ab6020` | 112 | medium |
| `0x80AB6A50` | `fn_80ab6a50` | 88 | high |
| `0x80AB6B70` | `fn_80ab6b70` | 48 | medium |
| `0x80AB6BA0` | `fn_80ab6ba0` | 48 | medium |
| `0x80AB6BD0` | `fn_80ab6bd0` | 384 | high |
| `0x80AB6DF0` | `fn_80ab6df0` | 56 | medium |
| `0x80AB6F30` | `fn_80ab6f30` | 76 | high |
| `0x80AB7210` | `fn_80ab7210` | 372 | high |
| `0x80AB7570` | `fn_80ab7570` | 88 | high |
| `0x80AB75D0` | `fn_80ab75d0` | 92 | high |
| `0x80AB7630` | `fn_80ab7630` | 12 | high |
| `0x80AB7640` | `fn_80ab7640` | 12 | high |
| `0x80AB7730` | `fn_80ab7730` | 224 | medium |

## Deliberate skips

- **`0x80AB5710` (1,064 bytes).** The export contains `unaff_GQR0` conversion
  artifacts, duplicated `ldexpf` guards and two "removing unreachable block"
  warnings. The pair-speed flow is visible, but the float maths cannot be
  reconstructed honestly, so it is left untranslated. [`fn_80ab56d0`] keeps
  the neighbouring call but not this body.
- **`0x80AB0570` (664 bytes).** A large spawn initialiser with many table
  writes and three calls into the same actor. It is readable but was left out
  to keep this lane reviewable; it is the natural next entry in the
  `0x80AB0000` bucket.
- **`0x80AB1830` (92 bytes).** The export ends in `halt_baddata()` after a
  bad-instruction warning, so there is no trustworthy body to translate.
- **Rest of the `0x80AB0000` bucket.** 145 of the 188 functions in this bucket
  remain untranslated outside these runs.
- **Hidden `r12` this-adjust operands.** The five `0x802DCBFC` forwarders
  (`0x80AB50E0`, `0x80AB5110`, `0x80AB6B70`, `0x80AB6BA0`, `0x80AB6DF0`) are
  marked `translated`, but their hidden register operand cannot be modelled;
  each doc block says so and the bodies carry a `// partial:` note.
- **Unnamed BSS tables.** [`fn_80ab7630`] and [`fn_80ab7640`] return
  `0x80B6E994` and `0x80B6E954`; the export does not name their contents, so
  the docs record the addresses only.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  3,464 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave56` prints `OK` with 29 rows for
  `wave56-enemies.csv`.
