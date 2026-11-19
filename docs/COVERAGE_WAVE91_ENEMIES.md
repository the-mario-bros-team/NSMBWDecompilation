# Coverage - Wave 91 (`game::enemies`, the `dossun` and `jugem` blocks of the `0x80A50000` bucket)

## How to read this file

This page records the Wave 91 `game::enemies` lane: 28 functions of the
`0x80A50000` bucket of REL module 3, `d_enemiesNP`, promoted to
`src/game/enemies.rs`. Every row in the tables below is one real
`pub unsafe fn fn_<addr>` body in that file, and the `evidence` column of the
machine-readable mirror (`ledger/agents/wave91-enemies.csv`) names the Ghidra C
export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Constants
are read through `platform::read_u32` / `read_f32` / `read_u16` / `read_u8`,
never transcribed as invented values. The values quoted in the comments were
dumped from `work/rels/elf/d_enemiesNP.elf` with a small ELF reader; the
strings at `0x80B454D0..0x80B45564` and `0x80B45B00..0x80B45B18` were read the
same way.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 28 | 4,056 | 15 | 9 | 4 |

The bucket holds 181 functions in
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`; 71 were already in
`ledger/functions.csv` before this pass (the wave-78 block
`0x80A50430..0x80A52D60` and the wave-19/20 `0x80A5AE30..0x80A5FE90` material),
28 are claimed here and 82 remain.

## What this pass covers

Three runs, all inside the part of the bucket that sits between the wave-78
block and the wave-19 `FireSnake`/`gabon` material.

**Run 1, `0x80A51A30..0x80A521F0`, the third `0x6E8`-byte descriptor
skeleton.**

- `0x80A51A30` is a deleting destructor: `0x8015F800(state, 0)` plus
  `0x802B9280` when the mode is positive. `0x80A51A90` is the constant
  producer for the shared record `0x80358084` (the same word as
  `0x80A50490`, `0x80A50F90` and `0x80A52690`).
- `0x80A51B80` walks the `+0x30` owner chain five times and recurses on the
  sixth owner, calling the `+0xC` predicate of every owner. `0x80A51C60`
  compares the `:`-suffixed name supplied by the `+0x20` slot with a candidate
  string using `0x802E1C40` and `0x802E1AB4`.
- `0x80A51D20` and `0x80A51D50` are the two 48-byte argument-swapping
  forwarders of the skeleton.
- `0x80A51D80` is the `0x6E8`-byte constructor: class vtable `0x80B44DEC`, the
  `+0x524` model-holder context, the `+0x544` model record, the `+0x584`
  animation holder (vtable `0x80B330E0`), the `+0x600` render block and the
  `+0x6D8` self-referencing sub-object. It is byte-identical to
  [`fn_80a50780`] and `0x80A51280` except for that final vtable.
- `0x80A521F0` is the 3x4-matrix position helper: `0x80152480` copies the
  matrix at `transform+0x14D8` into a 12-word buffer and the translation
  column is used to move `+0xAC`/`+0xB0` by the difference between the cached
  `+0x5E8`/`+0x5EC` origin and the live position.

**Run 2, `0x80A53710..0x80A54120`, the `dossun` actor.**

The block is identified by its asset strings: the binder loads
`g3d/dossun.brres`, the model/archive name `dossun`, and the two name tables at
`0x80B454E8`/`0x80B454F0` hold `dossun` and `dossun_big`. The landing effect
names are `Wm_en_dossunfall01`, `02` and `03`; the fallback smoke pair is
`Wm_en_sndlandsmk` / `Wm_en_landsmoke`. The actor's class vtable is reached only
through `+0x60` (slot `+0xD4` is the state setter); its constructor is not in
this bucket's function list, so it is not translated here.

- `0x80A53710` binds the archive, the per-variant model and the per-variant
  clip: `0x800690C0` resets `+0x524`, `0x800DF180` fills `+0x540`,
  `0x80239E30`/`0x8023A120` look up the names, `0x80169D90` binds the model
  with id `0x60`, `0x80167C40`/`0x80167D60` bind the animation pair and the
  two `0.0` blend calls finish the setup.
- `0x80A539C0` raises the landing effects at `(+0xAC, +0xB0, 5500.0)`: variant
  `0` uses the `0.6` scale triple and only `fall01`, variant `1` passes a null
  scale and raises all three.
- `0x80A53BD0` counts `+0x5DC` down and, at zero, tests
  `0x800978C0(delay[variant], 256.0)` where the delay table at `0x80B2CDF0`
  holds `48.0` and `80.0`.
- `0x80A53CA0`/`0x80A53DE0` leave the current state once `0x80169020` reports
  the clip finished, to `0x80B714B0` and `0x80B71530` respectively, and always
  advance the `+0x584` holder through its own `+0x14` slot.
- `0x80A53E70` is the contact dispatcher: mode `1` reacts to the `0x8000` bit,
  mode `2` forces `0x400000`, the clear case enters `0x80B71570` with the
  `0x40` countdown, the set case arms `+0x5BC` and calls `0x8006D000` on the
  contact block (`// unknown:` - Ghidra never decompiled that callee). Both
  emit `0x800D8C60`, the landing effects and the sound `0x211`/`0x212`.
- `0x80A54010` decrements `+0x5DC` and raises the mode byte `+0x5B8` to `3`
  when it reaches `0x20`, or jumps straight to `0x80B715B0` at zero.
- `0x80A540B0` seeds `+0xEC` with `0.875` and enters `0x80B71430` when the
  `0x8006EF40` landing probe succeeds, arming the `0x0C` countdown.
- `0x80A54120` is the wait/fall entry: it clears the two halfword angles
  `+0x100`/`+0x102`, requests sound `0x20F`/`0x210`, refreshes `+0x148`
  through `0x8008C380` and seeds `+0xB4` (`4500.0`), `+0x114` (`-0.25` for
  `dossun_big`, `-0.2` for `dossun`), `+0xF8` (`-8.0`), `+0xEC` (`2.0`) and
  the halfword state id `0x0D` at `+0x500`.

**Run 3, `0x80A54C00..0x80A55220`, the `0x5F0`-byte `jugem` actor.**

- `0x80A54C00` and `0x80A54C60` are the two deleting destructors of the
  five-entry skeleton that fronts the class (identity of the teardown, sizes
  `88` and `92` because of alignment padding).
- `0x80A54CC0` returns `0x80358004`; `0x80A54CD0` returns the shared
  first base-class record `0x80357BF4` (the same word as `0x80A4C8A0`).
- `0x80A54DC0` is the owner-chain walker, `0x80A54EA0` the name predicate and
  `0x80A54F60`/`0x80A54F90` the two 48-byte forwarders.
- `0x80A54FC0` allocates and constructs the `0x5F0`-byte actor with class
  vtable `0x80B45B20` and the three holders `+0x524`, `+0x544` and `+0x584`.
- `0x80A550F0` commits the transform: the `0x801C0BD0` matrix build (position
  in `f1`-`f3`, as in `fn_80aa6fc0`), the three rotation setters
  `0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0` for `+0x102`/`+0x100`/`+0x104`,
  the `0x8016A170` matrix bind, the `+0xDC` scale vector through
  `0x8016A150`, the `0x8016A1A0` clear, and the model commit through slot
  `+0x14`. It returns `1`.
- `0x80A55220` binds `g3d/jugem.brres`, the `jugem` model (id `0x20`) and the
  `throw` clip, with the `1.0` blend word at `0x80B2CE60`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A51A30` | `fn_80a51a30` | 92 | high |
| `0x80A51A90` | `fn_80a51a90` | 12 | high |
| `0x80A51B80` | `fn_80a51b80` | 224 | medium |
| `0x80A51C60` | `fn_80a51c60` | 136 | medium |
| `0x80A51D20` | `fn_80a51d20` | 48 | low |
| `0x80A51D50` | `fn_80a51d50` | 48 | low |
| `0x80A51D80` | `fn_80a51d80` | 168 | medium |
| `0x80A521F0` | `fn_80a521f0` | 140 | medium |
| `0x80A53710` | `fn_80a53710` | 312 | high |
| `0x80A539C0` | `fn_80a539c0` | 360 | high |
| `0x80A53BD0` | `fn_80a53bd0` | 148 | high |
| `0x80A53CA0` | `fn_80a53ca0` | 104 | high |
| `0x80A53DE0` | `fn_80a53de0` | 104 | high |
| `0x80A53E70` | `fn_80a53e70` | 372 | medium |
| `0x80A54010` | `fn_80a54010` | 128 | high |
| `0x80A540B0` | `fn_80a540b0` | 112 | medium |
| `0x80A54120` | `fn_80a54120` | 248 | high |
| `0x80A54C00` | `fn_80a54c00` | 88 | high |
| `0x80A54C60` | `fn_80a54c60` | 92 | high |
| `0x80A54CC0` | `fn_80a54cc0` | 12 | high |
| `0x80A54CD0` | `fn_80a54cd0` | 12 | high |
| `0x80A54DC0` | `fn_80a54dc0` | 224 | medium |
| `0x80A54EA0` | `fn_80a54ea0` | 136 | medium |
| `0x80A54F60` | `fn_80a54f60` | 48 | low |
| `0x80A54F90` | `fn_80a54f90` | 48 | low |
| `0x80A54FC0` | `fn_80a54fc0` | 132 | medium |
| `0x80A550F0` | `fn_80a550f0` | 240 | high |
| `0x80A55220` | `fn_80a55220` | 268 | high |

## Deliberate skips

- **`0x80A53850` (368 bytes) is not translated.** It is the `dossun` ground
  probe that sits between the binder and the landing-effect emitter. The C
  export assigns the return of `0x800EB930` to a `char` and then uses
  `extraout_f1` as a `double` on the following lines. The callee
  (`work/ghidra/main-gb/export/800eb930_FUN_800eb930.c`) is declared `void`
  and writes its height only through the `float *` fourth argument, so
  `extraout_f1` is a decompiler artifact of an untracked `lfs f1`, not a second
  return register. Reconstructing the `mode == 3` clamp without knowing which
  value `f1` holds would mean inventing operands, and the ELF is the only
  remaining source for that instruction. The surrounding bodies
  ([`fn_80a53710`] before it, [`fn_80a539c0`] after it) are translated, so only
  this one probe is missing from the run.
- **Nine bodies are `medium` because a field role or a callee receiver is
  inferred**, not because any instruction is guessed: `fn_80a51b80`,
  `fn_80a54dc0` (the `+0xC` slot's argument is the live `r3`),
  `fn_80a51c60`, `fn_80a54ea0` (the two `main.dol` string calls are named by
  role), `fn_80a51d80`, `fn_80a54fc0` (the base constructor's receiver is
  implicit in `r3`), `fn_80a521f0` (the C under-reports the 0x30-byte matrix
  copy as a 12-byte array) and `fn_80a53e70`, `fn_80a540b0` (the base updates
  print no argument). Counted exactly: nine `medium` rows.
- **Four bodies are `low`.** `fn_80a51d20`, `fn_80a51d50`, `fn_80a54f60` and
  `fn_80a54f90` are 48-byte argument-swapping forwarders into `0x802DCBFC`;
  the reshuffle is literal but neither the target's role nor the hidden `r12`
  this-adjust pointer is established. Every body carries the same
  `// partial:` note.
- **The rest of the bucket stays untranslated.** After this pass the largest
  untouched runs are `0x80A55550..0x80A55DE0` (a fourth descriptor skeleton),
  `0x80A56470..0x80A59F00` (a long handler block in front of the `0x80A5A350`
  container), the `0x80A5A350` container itself, the skipped `0x80A53850` and
  the fourteen scattered bodies at `0x80A5AEF0..0x80A5F7E0` that the wave-19
  and wave-20 passes left around the `FireSnake`/`gabon` material.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 28 sizes sum to
  4,056 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 28 was
  already in `ledger/functions.csv`.
- `node tools/check-lane-csv.mjs wave91` prints `28 rows` for this CSV and
  `OK`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/enemies.rs`. The only compile errors in the workspace during this
  pass are in `src/game/bases.rs`, `src/game/stage.rs` and
  `src/game/worldmap.rs`, which belong to other Wave 91 lanes.
