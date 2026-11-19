# Coverage - Wave 88 (enemies: the `0x80AF0000` bucket tail)

## How to read this file

This page records the Wave 88 `game::enemies` lane: one run in the
`0x80AF0000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row below is one real `pub unsafe fn fn_<addr>`
body in that file, and the `evidence` column of the machine-readable mirror
(`ledger/agents/wave88-enemies.csv`) names the Ghidra C export each body was
reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Data
constants, tables and strings are cited by address and read through
`platform::read_u32` / `read_f32` / `read_u16` / `platform::read_u8`, never
transcribed as invented values. Where a constant's meaning matters for the
translation, its value was read out of the REL image and recorded in the
constant's doc comment (`teresa`, `fog`, `Wm_en_teresatail`, `walk`, the
`4.0`/`24.0` offsets, the `1.2`/`1.15` record scales, `0.1875`, `64.0`, `8.0`
and the rest).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 5,564 | 16 | 13 |

The bucket holds 208 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 92 were already in
`ledger/functions.csv` before this pass, 29 are claimed here and 87 remain.

## What this pass covers

Two runs that earlier passes left untouched. They are adjacent in the bucket
but belong to different classes, so they are documented separately.

**Cluster 1, `0x80AF6190..0x80AF7330`, the `Wm_en_teresa` world-map ghost.**
The class is named by the strings its binders pass to the resource and clip
lookups: `teresa` at `0x80B6404C`, `fog` at `0x80B64054` and
`Wm_en_teresatail` at `0x80B64058` (bound by [`fn_80af6ed0`] and
[`fn_80af7330`]). The run is the actor's whole visible surface:

- `0x80AF6190` is the state-descriptor normaliser (`0x8015F800`, then
  `0x802B9280` for a positive count) and `0x80AF6840` releases the `+0x544`
  and `+0x584` holders through slot `+0x14`;
- `0x80AF6ED0` binds the model `g3d/teresa.brres`, the nine per-variant clips
  from the pointer tables `0x80B63FB0`/`0x80B64014` (index `+0x7E4 % 9`), the
  second animation layer `fog` at `+0x584` and the material layer at `+0x790`,
  all at rate `1.0`;
- `0x80AF6B50` and `0x80AF9170` fill the same fifteen-word effect record
  (`{0, x, y - 4.0, z, 1.2 x 3, 0.0 x 8}` and
  `{0, x, y - 4.0, z, 1.15 x 3, 0.0 x 8}` respectively) and arm it with
  `0x802DCA98` for `0x3C` frames;
- `0x80AF6CD0` rebuilds both holder transforms: `+0x544` from
  `+0xAC..+0xB4` and the three halfword angles at `+0x100..+0x104`, then
  `+0x584` from the first holder's slot-3 output lowered by `24.0`;
- `0x80AF7110` polls the `+0x7F0` latch through `0x80097B30`;
- `0x80AF7180` converts a direction to a side-aware aim angle
  (`atan2` then `+-0x4000`), clamps it to `+-0xE38` and accumulates it into
  `+0x100` through `0x8015F4F0`; `0x80AF7240` drives it from the `0x80064740`
  probe and flips the `+0x348` side byte;
- `0x80AF7330` copies the marked world position (`+0x7E4` index) out of
  `0x80732AD0`, binds the `Wm_en_teresatail` clip through the `+0x664` slot
  `+0xA0`, runs the turn step and releases the `+0x544` holder.

**Cluster 2, `0x80AF8660..0x80AF9EF0`, the `daEnSakasaTogezo_c` surface and
the following `0x800`-byte sibling.** This is the five-entry
state-descriptor skeleton plus the actor handlers it fronts:

- `0x80AF8660`/`0x80AF86C0` are two normaliser copies, and
  `0x80AF9B40`/`0x80AF9BA0` are the pair that
  [`register_sakasa_togezo_states`] actually names as the callbacks of the
  three `daEnSakasaTogezo_c` records;
- `0x80AF8800`/`0x80AF9CF0` are the two five-link `+0x30` chain walkers;
  `0x80AF88E0`/`0x80AF9DD0` are the `:`-suffix name predicates; the four
  48-byte thunks `0x80AF89A0`/`0x80AF89D0`/`0x80AF9E90`/`0x80AF9EC0` forward
  to `0x802DCBFC`;
- `0x80AF8A00` is the allocating constructor of the `0x9D8`-byte
  `daEnSakasaTogezo_c` (vtable `0x80B647FC`, embedded `+0x7DC`/`+0x7FC`/
  `+0x83C`/`+0x89C` holders), and `0x80AF9EF0` is the `0x800`-byte sibling's
  constructor (vtable `0x80B64CB8`, embedded `+0x59C`/`+0x6D4` helpers);
- `0x80AF9C00` returns the `Slid_` state's extra word `0x80353A64`;
- `0x80AF8CD0` is the slide entry: it seeds the `-0.1875`/`-4.5` speed set,
  the side byte, the `+0x614`/`+0x630` attack blocks, plays sound `0x1BC`,
  arms `0x800B86F0` and dispatches `0x80353A64`;
- `0x80AF9170` reports the slide attack until `+0x77C` reaches three;
- `0x80AF9290` is the upside-down `walk` entry (clip `walk` at `0x80B647F4`,
  side speed table `0x8042B6B0`, spawn angles `0xA000`/`0x6000`);
- `0x80AF93C0` is the walk frame step: it enters the slide when the probe is
  inside the `64.0` band, and otherwise returns to `0x80B77CF0`;
- `0x80AF95D0` is the turn frame step, which returns to `0x80B77CB0`.

### Identity evidence

- **`Wm_en_teresa` is read, not guessed.** `0x80AF6ED0` passes `teresa`
  (`0x80B6404C`) and `g3d/teresa.brres` (`0x80B64038`) to `0x800DF180`, and
  `0x80AF7330` passes `Wm_en_teresatail` (`0x80B64058`) to the `+0x664` clip
  slot. The nine variants come from the two pointer tables at
  `0x80B63FB0`/`0x80B64014`, which alias the `teresaA`/`teresa_other*` and
  `wait_other*` strings.
- **`daEnSakasaTogezo_c` is read, not guessed.**
  [`register_sakasa_togezo_states`] (`0x80AF98A0`, translated in an earlier
  wave) registers `0x80B77CB0`, `0x80B77CF0` and `0x80B77D2C` with callbacks
  `0x80AF9B40`, `0x80AF9B40` and `0x80AF9BA0` plus the extra producer
  `0x80AF9C00`. The two frame steps here dispatch exactly those three words,
  and `0x80AF8CD0` dispatches the `0x80353A64` word that `0x80AF9C00`
  returns for `Slid_`.
- **The `0x800`-byte constructor is a sibling, not a duplicate.** It installs
  the distinct vtable `0x80B64CB8` and its own `+0x59C`/`+0x6D4` helpers; the
  two skeletons share shape, not class.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AF6190` | `fn_80af6190` | 88 | high |
| `0x80AF6840` | `fn_80af6840` | 84 | high |
| `0x80AF6B50` | `fn_80af6b50` | 276 | high |
| `0x80AF6CD0` | `fn_80af6cd0` | 504 | medium |
| `0x80AF6ED0` | `fn_80af6ed0` | 572 | medium |
| `0x80AF7110` | `fn_80af7110` | 112 | high |
| `0x80AF7180` | `fn_80af7180` | 184 | medium |
| `0x80AF7240` | `fn_80af7240` | 208 | medium |
| `0x80AF7330` | `fn_80af7330` | 152 | medium |
| `0x80AF8660` | `fn_80af8660` | 88 | high |
| `0x80AF86C0` | `fn_80af86c0` | 92 | high |
| `0x80AF8800` | `fn_80af8800` | 224 | high |
| `0x80AF88E0` | `fn_80af88e0` | 136 | high |
| `0x80AF89A0` | `fn_80af89a0` | 48 | medium |
| `0x80AF89D0` | `fn_80af89d0` | 48 | medium |
| `0x80AF8A00` | `fn_80af8a00` | 236 | high |
| `0x80AF8CD0` | `fn_80af8cd0` | 396 | medium |
| `0x80AF9170` | `fn_80af9170` | 284 | high |
| `0x80AF9290` | `fn_80af9290` | 284 | medium |
| `0x80AF93C0` | `fn_80af93c0` | 464 | medium |
| `0x80AF95D0` | `fn_80af95d0` | 168 | medium |
| `0x80AF9B40` | `fn_80af9b40` | 88 | high |
| `0x80AF9BA0` | `fn_80af9ba0` | 92 | high |
| `0x80AF9C00` | `fn_80af9c00` | 12 | high |
| `0x80AF9CF0` | `fn_80af9cf0` | 224 | high |
| `0x80AF9DD0` | `fn_80af9dd0` | 136 | high |
| `0x80AF9E90` | `fn_80af9e90` | 48 | medium |
| `0x80AF9EC0` | `fn_80af9ec0` | 48 | medium |
| `0x80AF9EF0` | `fn_80af9ef0` | 268 | high |

## Deliberate skips

- **`0x80AF5C60` (260 bytes) is not translated.** Its export is dominated by
  paired-single `GQR` artifacts: `unaff_GQR0`/`unaff_GQR3`, `ldexpf` and
  `CONCAT22` fragments build the sin/cos operands of `0x80237C40`/`0x80237BD0`
  from a halfword angle. The scaling that connects `+0x104 + 0x4000` to the
  radians those functions expect is not recoverable from the C alone, so no
  honest body was written rather than inventing one. It is the only hole in
  the `0x80AF5C60..0x80AF7330` run.
- **The remaining 87 functions of `0x80AF0000` stay untranslated.** The next
  pass can resume at `0x80AF0370` (44 bytes, before this run) or at
  `0x80AFAD80` (the bitmask/count helpers `0x80AFAD80..0x80AFAE30`), whose
  `+0x524`/`+0x528` record is a different family from the two blocks here.
- **`fn_80af6ed0` is `medium`, not `high`.** All calls and names are literal,
  but `0x802DCD74` is printed with no argument and is treated as the
  current-actor accessor; the three binder signatures are inferred from their
  argument shapes.
- **`fn_80af7180` and `fn_80af7240` are `medium`.** `0x801611C0` is called
  with two `double` arguments and its result is used as a 16-bit angle, so it
  is treated as an `atan2`-style helper; the `0x80064740` probe's two outputs
  are known only by their use as a ground height and a side sign.
- **`fn_80af6cd0` is `medium`.** The matrix setters
  `0x8016A220`/`0x8016A1A0` are printed without names; their roles are
  inferred from the order in which the two holders are committed.
- **`fn_80af8cd0`, `fn_80af9290`, `fn_80af93c0` and `fn_80af95d0` are
  `medium`.** Their state transitions are certain because they dispatch the
  records named by [`register_sakasa_togezo_states`], but the per-field
  meaning of `+0x9D4`, `+0x38D` and the `+0x2E4` slot is inferred from the
  branches that read them.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  5,564 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none was already in
  `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave88` reports 29 rows for this CSV and
  `OK`.
- `cargo check --message-format=short` produces no diagnostic naming
  `src/game/enemies.rs`.
