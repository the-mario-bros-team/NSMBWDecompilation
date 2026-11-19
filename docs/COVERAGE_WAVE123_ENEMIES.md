# Coverage - Wave 123 (`game::enemies`, the `0x80A20000` bucket remainder)

## How to read this file

This page records the Wave 123 `game::enemies` lane: 28 functions spread over
three runs of the `0x80A20000` bucket of REL module 3, `d_enemiesNP`, appended
to `src/game/enemies.rs` as section 104 of that file. Every row of the machine
readable mirror (`ledger/agents/wave123-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 6,224 | 20 | 8 |

The bucket holds 171 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 117 were already in
`ledger/functions.csv`, leaving 54. This pass claims 28 of them, all in the
three runs below, and leaves 26 untouched.

Two rows are `partial` (the two `0x802DCBFC` state-destroying-destructor
trampolines, whose hidden `r12` this-adjust pointer `platform::call3` cannot
model - the same limitation recorded for the copies in waves 56, 72, 75, 80,
81, 82, 97, 98, 99, 101 and 122). Eight rows are `medium` for a named reason,
none for an unread instruction.

## What this pass covers

- **The `daEnBomhei_c` tail (`0x80A296E0..0x80A2A180`, 10 bodies).** Wave 92
  built the state skeleton and wave 105 the actor body; this pass fills the two
  runs those waves left. `0x80A296E0` and `0x80A298A0` are the two `Walk`
  entries: they attach the `walk` clip to `+0x548`/`+0x588`, blend at the
  `0x80B2B9A4` (`2.0`) / `0x80B2B93C` (`0.0`) weights, branch on spawn-word
  bit 1 for the side byte (`0x80096240(actor, actor+0xAC)` or `~bit0`) and seed
  `+0xE8..+0xF0` from the `0x80B2BA80`/`0x80B2BA90` (bits 0..1) or
  `0x80B2BAB0` (bit 4) / `0x80B2BAA0` (side and bit 3) tables.
  `0x80A29AF0` is the third entry (three visible blend weights and the
  `0x80B2BAB8` `±1.25` pair). `0x80A299B0` and `0x80A29BF0` are byte-identical
  two-phase frame steps that leave mode 1 through the six-word probe
  `0x80B2B888`/`0x80B2B87C`/`0x80B2B898` and then enter `Turn`
  (`0x80B6FD28`) or `Walk` (`0x80B6FB28`). `0x80A29830` counts `+0x5F4` down
  and enters `Walk`; `0x80A29D30` resets the `+0x5C0` clip position;
  `0x80A29D90` is the `Turn` ramp (`0x8015F4F0` step `0x400`).
  `0x80A2A020` is the `Explode` entry (both `Wm_en_*` effects, sound `0x1CC`,
  the `+0x164..+0x184` blast record and the `+0xB0 += 8.0` lift);
  `0x80A2A180` seeds the three halfword angles from `0x800B2EE0(9)`.
- **The next actor's support objects (`0x80A2CDE0..0x80A2D2A0`, 9 bodies).**
  Three teardown shapes (`0x80A2CDE0` with the `+0x0C` member, `0x80A2CEA0`
  through `0x802BE7C0`, `0x80A2CF80` through `0x8015F800`), the `0x76`-byte
  descriptor constructor `0x80A2CE50` (`0x80B3BD54` at `+0x00`), the class-word
  producer `0x80A2CFE0` (`0x8082C47C`), the five-step `+0x30` chain walker
  `0x80A2D0D0`, the `:`-suffix name predicate `0x80A2D1B0`, and the two
  `0x802DCBFC` trampolines `0x80A2D270`/`0x80A2D2A0`.
- **The `bubble` actor (`0x80A2DFE0..0x80A2F170`, 9 bodies).** `0x80A2DFE0`
  is the 940-byte binder: it loads `g3d/bubble.brres`, builds the `bubble`
  matrix/attribute pair at `+0x54C`/`+0x610`, the `bubble_fog` pair at
  `+0x58C`/`+0x63C`, the `bubble_fog2` pair at `+0x5CC`/`+0x668`, and eight
  `bubble_tail` pairs stepping `0x88` from `+0x704`/`+0x744`. `0x80A2E450`
  rebuilds the three segment matrices and patches `+0xB44..+0xB4C` with two
  `24.0` z drops; `0x80A2E710` writes one packed colour byte per node.
  `0x80A2E7E0`/`0x80A2E8F0` refresh and release the eight tail slots, and
  `0x80A2EDE0` releases the five main holders. `0x80A2EE70` is the
  `+0xB64..+0xB7A` fade ramp, `0x80A2EF70` the main per-frame step and
  `0x80A2F170` the `-1.0`/`1.0` speed selector.
- **Constants.** The tables were read from `work/rels/elf/d_enemiesNP.elf` and
  are documented once in the section header: the bombhei speed pairs,
  `±1.25`/`±1.5`/`±1.3` side pairs, `-0.1875`, `2.25`/`4.5`, the `1.0`/`2.0`
  blends, the `0.0`/`8.0`/`18.0` blast seeds, and the bubble `1.0`, `24.0`,
  `1.2`, `0.06`, `6500.0`, `20.0`, `-1.0` and `-999999.0` words.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A296E0` | `fn_80a296e0` | 228 | high |
| `0x80A29830` | `fn_80a29830` | 112 | high |
| `0x80A298A0` | `fn_80a298a0` | 228 | medium |
| `0x80A299B0` | `fn_80a299b0` | 312 | high |
| `0x80A29AF0` | `fn_80a29af0` | 220 | high |
| `0x80A29BF0` | `fn_80a29bf0` | 312 | high |
| `0x80A29D30` | `fn_80a29d30` | 68 | high |
| `0x80A29D90` | `fn_80a29d90` | 364 | high |
| `0x80A2A020` | `fn_80a2a020` | 288 | medium |
| `0x80A2A180` | `fn_80a2a180` | 108 | high |
| `0x80A2CDE0` | `fn_80a2cde0` | 108 | high |
| `0x80A2CE50` | `fn_80a2ce50` | 76 | high |
| `0x80A2CEA0` | `fn_80a2cea0` | 88 | high |
| `0x80A2CF80` | `fn_80a2cf80` | 92 | high |
| `0x80A2CFE0` | `fn_80a2cfe0` | 12 | high |
| `0x80A2D0D0` | `fn_80a2d0d0` | 224 | medium |
| `0x80A2D1B0` | `fn_80a2d1b0` | 136 | high |
| `0x80A2D270` | `fn_80a2d270` | 48 | medium (partial) |
| `0x80A2D2A0` | `fn_80a2d2a0` | 48 | medium (partial) |
| `0x80A2DFE0` | `fn_80a2dfe0` | 940 | high |
| `0x80A2E450` | `fn_80a2e450` | 704 | high |
| `0x80A2E710` | `fn_80a2e710` | 208 | medium |
| `0x80A2E7E0` | `fn_80a2e7e0` | 268 | high |
| `0x80A2E8F0` | `fn_80a2e8f0` | 116 | high |
| `0x80A2EDE0` | `fn_80a2ede0` | 140 | high |
| `0x80A2EE70` | `fn_80a2ee70` | 168 | high |
| `0x80A2EF70` | `fn_80a2ef70` | 512 | medium |
| `0x80A2F170` | `fn_80a2f170` | 96 | medium |

## Deliberate skips

- **The other 26 addresses of the bucket were not attempted.** In ascending
  order they are: `0x80A20810` (228), `0x80A20900` (520), `0x80A22820` (700),
  `0x80A22B20` (44), `0x80A24AD0` (312), `0x80A25F00` (48),
  `0x80A281C0` (32), `0x80A2B490` (48), `0x80A2BC50` (456),
  `0x80A2C2A0` (76), `0x80A2D460` (20), `0x80A2D660` (132),
  `0x80A2E970` (240), `0x80A2EA60` (224),
  `0x80A2EB40` (188), `0x80A2EC00` (472), `0x80A2F670` (88),
  `0x80A2F6D0` (92), `0x80A2F730` (12), `0x80A2F740` (12),
  `0x80A2F830` (224), `0x80A2F910` (136), `0x80A2F9D0` (48),
  `0x80A2FA00` (48), `0x80A2FA30` (308) and `0x80A2FD40` (340). Nothing was
  written for them, so no ledger row claims them.
- **`0x80A281C0`, `0x80A2B490` and `0x80A2C2A0` stay skipped** for the reasons
  already recorded in the wave-92 and wave-105 docs: the first is a truncated
  `halt_baddata` stub, and the other two are `2^52` bias artefacts whose
  integer operand the decompiler folded away.
- **`0x80A2BC50`, `0x80A2D460` and `0x80A2D660` stay skipped** as wave 92
  recorded: the first carries `unaff_GQR0` pseudo-registers and a removed
  unreachable block, the second is truncated, and the third calls the second.
- **`0x80A2E970`, `0x80A2EA60`, `0x80A2EB40` and `0x80A2EC00` are not claimed
  here.** All four are members of the `bubble` per-frame block, but their
  exports mix `unaff_GQR0` conversion artefacts (and `0x80A2EC00` also reads
  `in_f31`) with stores whose source values the decompiler no longer types.
  [`fn_80a2ef70`] and [`fn_80a2e450`] keep them as address-only callees.
- **The `0x80A2F670..0x80A2FD40` descriptor skeleton was left for a later
  pass.** Its thirteen functions are a sixth copy of the five-entry state
  skeleton plus two sizeable helpers (`0x80A2FA30`, `0x80A2FD40`); they were
  not needed to close this lane and are documented here so the next pass can
  claim them without re-deriving the layout.
- **No address already in `ledger/functions.csv` was touched.** In particular
  the `daEnBomhei_c` bodies referenced by this pass ([`fn_80a282c0`],
  [`fn_80a284d0`], [`fn_80a285d0`], [`fn_80a263c0`] and the wave-105 handlers)
  keep their existing rows.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  6,224 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave123` prints
  `wave123-enemies.csv: 28 rows` and `OK`.
- `cargo check --message-format=short` emits no diagnostic naming
  `src/game/enemies.rs`.
- The module header was updated: cluster item 104, the cluster count (one
  hundred and three to one hundred and four), the function totals (2,783 to
  2,811 public translations, 2,712 to 2,740 `fn_<addr>` bodies), the "start
  here" list and the records list.
