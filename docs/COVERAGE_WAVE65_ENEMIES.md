# Coverage - Wave 65 (enemies: `0x80AD2660..0x80AD5E70`)

## How to read this file

This page records the Wave 65 `game::enemies` lane: two runs in the
`0x80AD0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row corresponds to one real `pub unsafe fn` body in
that file, and the `evidence` column names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 30 | 3,256 | 21 | 5 | 4 |

## What this pass covers

The pass reads `work/ghidra/rel-d_enemiesNP/export/functions.tsv`. The bucket
holds 189 functions, 33 of which are already in `ledger/functions.csv`; none of
those addresses is repeated here. Two contiguous runs were taken:

- **The `obj_obake_door` run (`0x80AD2660..0x80AD2E10`, 1,292 bytes).** One
  small actor built by [`fn_80ad2a20`] (`0x5E0` bytes). The distinguishing
  evidence is the clip name `s_obj_obake_door` at `0x80B5B7C8`, looked up by
  [`fn_80ad2b80`]; the class therefore is the "obake door" (ghost door). The
  run contains the two deleting-destructor forwarders, two constant producers
  for the state descriptors `0x80353B24`/`0x80353AA4`, the five-link `+0x30`
  chain walker, the `:`-suffix name predicate, two 48-byte argument-swapping
  forwarders, the `1.0`/`0.0` timing helpers and the `0x28D` sound request.
- **The `0x4C30` container run (`0x80AD4DC0..0x80AD5E70`, 1,964 bytes).** Its
  deleting-destructor forwarder and two 48-byte forwarders, the constructor
  (class vtable `0x80B5C250`, four sub-object words at `+0x564..+0x574`, the
  `+0x5DC` holder, the `+0x6DC` model block and a `0x2290 x 2` container at
  `+0x6F8`), the three fixed-capacity element arrays (`9` slots at `+0x000`,
  `0x36` at `+0x4A4`, `4` at `+0x207C`, all of `0x84`-byte elements), the
  element constructor/destructor pair, the two rectangle writers, the
  `0x8007FA40` animator installer with its three callbacks, the corner update
  on `+0x5DC`, the spawn-word controller switch and the full destructor.

The two runs are separated by `0x80AD2E10..0x80AD3C10` (`0xE00` bytes) that
`functions.tsv` lists as no functions at all - inline state data and tables,
not code.

### Constants read from the image

`work/rels/elf/d_enemiesNP.elf` was read at the analysis address through its
program headers:

- `0x80B308C4 = 0.0` (f32), `0x80B308C8 = 1.0` (f32);
- `0x80B30958 = 0.0` (f32), `0x80B3095C = 0.5` (f32).

### Identity evidence

`0x80B5B7D8` (final vtable at `+0x60` of the `0x5E0` actor), `0x80B5C250` (the
`0x4C30` container) and the element sub-object pair `0x803295D8`/`0x80329548`
are literal in the exports. The container's class name is not printed anywhere
in this run, so it is described by size and offsets only.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AD2660` | `fn_80ad2660` | 88 | high |
| `0x80AD26C0` | `fn_80ad26c0` | 92 | high |
| `0x80AD2720` | `fn_80ad2720` | 12 | high |
| `0x80AD2730` | `fn_80ad2730` | 12 | high |
| `0x80AD2820` | `fn_80ad2820` | 224 | medium |
| `0x80AD2900` | `fn_80ad2900` | 136 | medium |
| `0x80AD29C0` | `fn_80ad29c0` | 48 | low |
| `0x80AD29F0` | `fn_80ad29f0` | 48 | low |
| `0x80AD2A20` | `fn_80ad2a20` | 156 | high |
| `0x80AD2B80` | `fn_80ad2b80` | 120 | high |
| `0x80AD2C80` | `fn_80ad2c80` | 64 | high |
| `0x80AD2CC0` | `fn_80ad2cc0` | 76 | high |
| `0x80AD2D60` | `fn_80ad2d60` | 80 | high |
| `0x80AD2E10` | `fn_80ad2e10` | 136 | high |
| `0x80AD4DC0` | `fn_80ad4dc0` | 88 | high |
| `0x80AD4EE0` | `fn_80ad4ee0` | 48 | low |
| `0x80AD4F10` | `fn_80ad4f10` | 48 | low |
| `0x80AD4F40` | `fn_80ad4f40` | 268 | high |
| `0x80AD5050` | `fn_80ad5050` | 136 | high |
| `0x80AD50E0` | `fn_80ad50e0` | 104 | medium |
| `0x80AD5150` | `fn_80ad5150` | 104 | high |
| `0x80AD51C0` | `fn_80ad51c0` | 152 | high |
| `0x80AD5480` | `fn_80ad5480` | 32 | high |
| `0x80AD54A0` | `fn_80ad54a0` | 196 | medium |
| `0x80AD5570` | `fn_80ad5570` | 44 | high |
| `0x80AD55A0` | `fn_80ad55a0` | 136 | high |
| `0x80AD5630` | `fn_80ad5630` | 96 | high |
| `0x80AD5690` | `fn_80ad5690` | 204 | high |
| `0x80AD5C40` | `fn_80ad5c40` | 96 | medium |
| `0x80AD5E70` | `fn_80ad5e70` | 212 | high |

## Deliberate skips

- **`0x80AD3C10` (208 bytes).** The matrix rebuild decodes two `GQR0` values
  through `ldexpf` and the export contains the `unaff_GQR0` pseudo-register,
  i.e. the decompiler lost the quantiser state. The body cannot be
  reconstructed without guessing the conversions, so it is skipped.
- **`0x80AD5760` (500 bytes).** The spawn initialiser reads the same
  `GQR0`-derived values and additionally builds two `CONCAT44(0x43300000, x)`
  int-to-double conversions. No honest body exists from this export.
- **`0x80AD5CD0` (316 bytes).** Same `CONCAT44`/`0x43300000` conversion, and
  the export calls `FUN_80ad54a0()` with **no** arguments even though
  `0x80AD54A0` takes the actor and two `double`s. Translating the call would
  mean inventing its operands.
- **`0x80AD3CE0`, `0x80AD3D50` and the rest of `0x80AD3C10..0x80AD48C0`.**
  Readable, but they belong to the next actor and were left for a later pass
  to keep this lane inside its 30-body budget. `0x80AD3D50` in particular is a
  688-byte state step whose calls to `0x80AD4220` and `0x80AD42B0` belong to
  that same untranslated block.
- **Forwarder identity.** All six 48-byte forwarders reproduce the literal
  `(second, second, first)` call to `0x802DCBFC`. The target's role is not
  established, so those six rows are `low`.
- **The two chain helpers.** `0x80AD2820` and `0x80AD2900` are byte-for-byte
  the same shapes as the earlier `0x80A05AC0`/`0x80A05BA0` pair; they are
  translated with those bodies and stay `medium`, because the predicate and
  the string-helper identities are inferred rather than printed.
- **Element receiver.** `0x80AD50E0`'s export prints `0x80169CD0()` with no
  argument. The element itself is used as the receiver (inferred from the
  matching `0x80169D20(element, -1)` in the destructor), so the row is
  `medium`.
- **Callback roles.** The four addresses `0x80AD5260`, `0x80AD52C0`,
  `0x80AD5320` and `0x80AD53C0`/`0x80AD53F0`/`0x80AD5420` are passed as
  callbacks/vtable slots but their bodies are outside this pass; they are kept
  as addresses.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 30 sizes sum to
  3,256 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave65` reports `OK` with
  `wave65-enemies.csv: 30 rows`.
