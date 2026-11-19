# Coverage - Wave 87 (enemies: the `0x80A30000` bucket opening)

## How to read this file

This page records the Wave 87 `game::enemies` lane: one run in the
`0x80A30000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below is one real `pub unsafe fn fn_<addr>`
body in that file, and the `evidence` column of the machine-readable mirror
(`ledger/agents/wave87-enemies.csv`) names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants, tables and strings are cited by address and read through
`platform::read_u32` / `read_f32` / `read_u16` / `read_u8`, never transcribed
as invented values. Where a constant's meaning matters for the translation, its
value was read out of the REL image and recorded in the constant's doc comment
(`houdai_ball`, `obj_door_shiroboss`, `obj_chikuwa_block`, `0.05`, `-0.1875`,
the `{4, 252}` halfword table and the rest).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 25 | 3,640 | 14 | 11 |

The bucket holds 149 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 32 were already in
`ledger/functions.csv` before this pass (waves 55 through 86), 25 are claimed
here and 92 remain.

## What this pass covers

Two runs. They are adjacent in the bucket but belong to different holder
families, so they are documented separately.

**Cluster 1, `0x80A30CB0..0x80A313C0`, four small `+0x524`/`+0x540`/`+0x544`
holder helpers.** These are the only functions in the bucket before the door
block, and all four key on the same three-word holder chain, so they are taken
together:

- `0x80A30CB0` runs vtable slot `+0x14` of the `+0x544` holder and returns `1`;
- `0x80A30CE0` compares the owner word `+0x538` with the scene handle from
  `0x8016A5D0` and releases the holder through slot `+0x10` when it changed -
  the same body as the already-translated `0x80A3CA30`;
- `0x80A31190` builds the `+0x7C` matrix from the position `+0xAC..+0xB4` and
  the three halfword angles at `+0x100..+0x104` through `0x80064AF0` and
  `0x801C0BD0`, feeds the angles back through
  `0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0`, and applies the matrix and the
  `+0xDC` offset to the holder;
- `0x80A313C0` is the `houdai_ball` binder: `0x20`-byte record, model holder
  `+0x524`, resource `+0x540`, animation holder `+0x544`. The class that owns
  it is not printed anywhere in the run, so it is named by its model only.

**Cluster 2, `0x80A31800..0x80A329B0`, the `obj_door_shiroboss` door block.**
The class is identified by the two strings the block passes to the model and
clip lookups: `obj_door_shiroboss` at `0x80B3CE74` (passed by `0x80A31AE0`;
the constructor `0x80A31980` installs the class vtable `0x80B3CE88` 20 bytes
behind it) and `obj_chikuwa_block` at `0x80B3D168`. The run is the actor's
whole surface:

- `0x80A31800` is the state-descriptor normaliser (`0x8015F800`, then
  `0x802B9280` for a positive mode) and `0x80A31920`/`0x80A31950` are its two
  `0x802DCBFC` deleting-destructor forwarders;
- `0x80A31980` is the allocating constructor: `0x5E0` bytes, holder chain
  `+0x524`/`+0x544`/`+0x584`, the `0x80303658` base vtable replaced by the
  class vtable `0x80B3CE88`, and `+0x584 = 0x80B330E0`;
- `0x80A31AE0` binds `obj_door_shiroboss` and applies two `0.0` blends;
  `0x80A31C40` moves the clip to rate `1.0` and sets `+0x5B8 = 3`;
  `0x80A31C80` stops both layers at `0.0`;
- `0x80A31D60`/`0x80A31DB0` request sounds `0x291` and `0x28F` at the actor
  position through `0x80198C30`;
- `0x80A31E00` resolves the object at `+0x394` through slot `+0x28` and
  compares its type through slot `+0x14` against `0x80353090`;
- `0x80A31E50` is the full destructor: three holders, the base object, and the
  allocation release for a positive flag;
- `0x80A31F80` is the spawn initialiser: it builds the child prop, offsets the
  position by `+8.0`/`-16.0`, installs the seven-word `0x8007FB10` collision
  record at `+0x5B4`, clears the four `+0x6BC` colour bytes to `0xFF`, seeds
  the `0x80B2BF08..0x80B2BF2C` geometry, searches a class-`0x18E` actor at the
  spawn point and dispatches `0x80B70110`;
- `0x80A32320` builds the `obj_chikuwa_block` child prop: `0x23`-byte record,
  `0x80166830`/`0x80166BD0` binders, the holder's `+0x18` slot and the
  `1.0`/`0.0` clip blends;
- `0x80A32470` is the per-slot colour callback: a class byte of `1` or `2`
  copies the animation slot byte into `+0x6BC + slot` and re-arms `+0x6B4`;
- `0x80A324E0` is the four-colour guard: it arms `+0x6B4` while any colour is
  `-1`, counts it down, and on expiry calls `0x80A326F0`, the stage helper
  `0x80077860` at the `-8.0`/`+16.0` offset and `0x80162510`;
- `0x80A32660` searches the running actor list for a class-`0x18E` object
  whose `+0xAC`/`+0xB0` match a target point and caches the found word at
  `+0x5B0`; `0x80A326F0` writes the `+0x124` byte on that cached object;
- `0x80A32750` flips the `+0x6A4` palette index from the `0x8042A200` input
  word and reads the `0x80B2BF38` `{4, 252}` halfword table;
- `0x80A327C0` is the main per-frame handler: `0x80A324E0`, the two base
  updates, the `+0x6AC` countdown, the palette flip, the `0x40`-phase
  `-0.1875` speed and the `0x80B70150` dispatch;
- `0x80A328D0` is the landing handler: on contact flag `2` it resets the
  `+0x5B4` record, zeroes `+0xDC..+0xE4`, restores the `+0x69C` height and
  dispatches `0x80B70190`;
- `0x80A329B0` advances the `+0x6B8` timer by `0.05`, clamps it at `1.0`,
  emits the landing helper `0x80077860`, re-arms `0x80A326F0` and mirrors the
  timer into `+0xDC..+0xE4`.

### Identity evidence

- **`obj_door_shiroboss` is read, not guessed.** `0x80A31AE0` passes
  `0x80B3CE74` to `0x8023A0B0` with the model holder `+0x540`, and the class
  vtable installed by the constructor at `0x80A31980` is `0x80B3CE88`, 20
  bytes behind that string.
- **`obj_chikuwa_block` is the child prop.** `0x80A32320` passes both
  `0x80B3D168` (clip name) and `0x80B3D14C` (`g3d/obj_chikuwa_block.brres`)
  to `0x800DF180` and then binds the two records to `+0x584`.
- **`houdai_ball` is a different holder family.** `0x80A313C0` uses the same
  `+0x524`/`+0x540`/`+0x544` triple but neither `+0x584` nor any `+0x6Bx`
  field; it is kept as a separate cluster.
- **The spawn geometry is a real table, not invented.** The values at
  `0x80B2BF08..0x80B2BF40` were read from the REL image: `0.0`, `8.0`,
  `16.0`, `16.0`, `8.0`, `16.0`, `1.0`, `-8.0`, `0.0`, `-2.0`, then the
  `{4, 252}` halfword table, `-0.1875` and `0.05`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A30CB0` | `fn_80a30cb0` | 48 | medium |
| `0x80A30CE0` | `fn_80a30ce0` | 80 | medium |
| `0x80A31190` | `fn_80a31190` | 220 | high |
| `0x80A313C0` | `fn_80a313c0` | 172 | high |
| `0x80A31800` | `fn_80a31800` | 88 | high |
| `0x80A31920` | `fn_80a31920` | 48 | medium |
| `0x80A31950` | `fn_80a31950` | 48 | medium |
| `0x80A31980` | `fn_80a31980` | 156 | high |
| `0x80A31AE0` | `fn_80a31ae0` | 120 | high |
| `0x80A31C40` | `fn_80a31c40` | 64 | high |
| `0x80A31C80` | `fn_80a31c80` | 76 | high |
| `0x80A31D60` | `fn_80a31d60` | 80 | high |
| `0x80A31DB0` | `fn_80a31db0` | 80 | high |
| `0x80A31E00` | `fn_80a31e00` | 80 | medium |
| `0x80A31E50` | `fn_80a31e50` | 136 | medium |
| `0x80A31F80` | `fn_80a31f80` | 436 | medium |
| `0x80A32320` | `fn_80a32320` | 328 | high |
| `0x80A32470` | `fn_80a32470` | 104 | medium |
| `0x80A324E0` | `fn_80a324e0` | 380 | medium |
| `0x80A32660` | `fn_80a32660` | 136 | medium |
| `0x80A326F0` | `fn_80a326f0` | 84 | high |
| `0x80A32750` | `fn_80a32750` | 52 | medium |
| `0x80A327C0` | `fn_80a327c0` | 240 | high |
| `0x80A328D0` | `fn_80a328d0` | 184 | high |
| `0x80A329B0` | `fn_80a329b0` | 200 | high |

## Deliberate skips

- **The rest of the bucket stays untranslated.** 92 of the 149 functions in
  `0x80A30000` remain, all of them from `0x80A32D90` onwards
  (`0x80A32D90..0x80A3FFF0`).
  The large exports in that tail - `0x80A37830` (1,148 bytes),
  `0x80A3A4A0` (2,620), `0x80A3DEA0` (2,940) and `0x80A3FFF0` (480) - are
  constructors/initialisers that need their own pass; none of the 25 bodies
  here calls into them.
- **`0x80A313C0`'s owning class stays unnamed.** The binder is complete and
  literal, but no string, state table or vtable in the surrounding export
  names its class; it is described by its `houdai_ball` model instead.
- **The `0x80A314C0..0x80A317F0` range has no exported function**, so there
  is nothing to translate between cluster 1 and the door block.
- **`fn_80a30cb0` and `fn_80a30ce0` are `medium`, not `high`.** Ghidra prints
  the holder slot calls without the receiver register; the bodies pass the
  `+0x544` holder, following the already-translated `0x80A3CA30` sibling.
- **`fn_80a31e00` is `medium`, not `high`.** The `+0x28` slot call is printed
  without arguments; the `+0x394` object is supplied as its receiver, and the
  `(-v | v) >> 31` return is reproduced as `v != 0`.
- **`fn_80a32660` is `medium`, not `high`.** Ghidra renders the two position
  loads as `(float)puVar1[0x2B]`/`[0x2C]` (integer-to-float casts), which
  cannot be the original comparison; the body uses the raw `f32` words at
  `+0xAC`/`+0xB0`, the offsets the `class 0x18E` objects use elsewhere.
- **`fn_80a31f80` is `medium`, not `high`.** Its seven-word `0x8007FB10`
  record is reconstructed field by field from the stack slots
  (`-8.0`, `16.0`, `8.0`, `8.0`, the `0x80A32470` callback, `0`, `0`); the
  export loads `local_20` (`16.0`) once and never reads it again.
- **`fn_80a324e0` and `fn_80a329b0` are `medium`/`high` respectively.** Both
  call `0x80077860`, whose effect is not established; only its argument shape
  (stage object, two fixed-point coordinates, `0`, `7`) is literal.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 25 sizes sum to
  3,640 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave87` reports 25 rows for this CSV and
  `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
