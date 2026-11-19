# Coverage - Wave 175 lane B (`game::bases`, 396-400 byte band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 5 | 1,984 | 5 | 0 |

Five bodies from the 396-400 byte band. Source of the bodies:
`work/wave175/laneB_block.rs`; per-function ledger:
`ledger/agents/wave175-laneb.csv`.

## What this pass covers

- **Generic shot per-frame update (1 body).** `0x8069FF00` is the un-specialised
  version of the body that the already-translated `fn_8069F640` repeats at its
  tail: `0x8069E5F0(this)`, the `+0x544` slot `+0x1C`, then the owner's `+0x2DC`
  slot whose result carries the inner object at `+0x5F8`. Two `== 0` virtual
  queries (`+0x0C`, `+0x10`) on that inner object decide whether the owner's
  `+0x380` slot runs. It then writes the `+0x740` transform through
  `0x8015F470(30.0 * signed_table_byte(this[+0x348]), 5.0, ...)`, runs the
  `+0x33C` slot, clamps `+0xEC` to `0.0` behind `0x8006EF40`/`0x8006FA40`, and
  emits the `0x8082B734` cell through `+0xD4` when `0x80169020(this+0x584)`
  is non-zero.
- **Prop re-initialiser (1 body).** `0x806F7500` unpacks the spawn word at
  `+0x004` into `+0x574` (`>>4 & 1`), `+0x56C` (`>>8 & 3`) and `+0x570`
  (`>>12 & 3`), suppresses `+0x56C` when the two conflict, and seeds about thirty
  floats from the `0x807CC2E0..0x807CC2F0` pool (`1.0`, `0.0`, `256.0`, `48.0`,
  `8.0`) plus the two `main.dol` scalars `0x8042BB08`/`0x8042BB0C`. The `>>16 & 1`
  bit picks `+0xAC += 8.0` or `+0xB0 -= 8.0`. It finishes with the five family
  setup helpers (`0x806F7A40`, `0x806F7830`, `0x806F80D0`, `0x806F8050` whose
  result lands in `+0x568`, `0x806F8160`), the `0x8082EBB0` cell and
  `0x800690E0(this+0x590)`, returning `1`.
- **World-map shot-effect burst (1 body).** `0x8074F600` lazily initialises the
  two effect parameter blocks at `0x80831D2C` (`0`/`0x2000`/`0xE000`) and
  `0x80831D34` (`2.0` triple), emits `Wm_mr_landsmoke` (`0x80812390`) with the
  scale block and `Wm_ob_cmnshothit`/`Wm_ob_cmnshotspark`/`Wm_ob_cmnshotstar`
  (`0x808123A0`/`0x808123B4`/`0x808123C8`) without it through `0x8016C890`,
  queues sound `0x2A0` from `this+0xAC` through `0x8006A4B0`/`0x80198C30`, and
  then walks the four bits of the active mask at `+0x6A4`, re-seating each live
  player via `0x8013FAA0` plus the `+0x60` vtable slot `+0x134` with
  `index * 5`.
- **World-map scene-ready selector (1 body).** `0x807B7F30` splits on
  `0x80315B98`. The zero path raises `(*0x80429F7C)[+0x60]`, bumps one
  coin/star counter only when `0x8005FFB0() == 1` and `x < 10 && y < 0x2A` (the
  `cntlzw`/`srwi 5` gate), then either runs the `0x80428730 == 9` exit sequence
  (`0x8008EDB0`, `0x800FBD30(0)`, two byte clears) or the
  `0x800FFE70`/`0x800FFF90` refresh. The other path emits
  `0x800FB9E0(0, 1, x, y)` and forwards `this` to `0x807B7D60`.
- **Player-record rebuild (1 body).** `0x80637D20` walks the four `0x3C`-byte
  records at `+0x7BC` (the layout `fn_80637A40` already documents): it stamps
  `+0x20` with the lane index and `+0x24` with `table[lane]` from `0x80354E60`,
  then either clears nine fields or copies the live player's `+0x48`/`+0x28`/
  `+0x08` words (four-byte lane stride off `*0x80429FB0`) plus the
  `+0x58 + table[lane]` byte, before zeroing `+0x34`/`+0x28` and calling
  `0x806373C0`. It closes with `fn_80637A40`, twelve `0x800C93D0` calls on
  `+0x70` for indices `0x21..=0x2C`, `0x80637050`, `0x80637310`,
  `0x80637610(this, 1)`, three flag bytes and the `+0x6A8` slot with
  `0x80826AE0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8069FF00` | `fn_8069ff00` | 396 | high |
| `0x806F7500` | `fn_806f7500` | 396 | high |
| `0x8074F600` | `fn_8074f600` | 396 | high |
| `0x807B7F30` | `fn_807b7f30` | 396 | high |
| `0x80637D20` | `fn_80637d20` | 400 | high |

## Deferred

None. All five assigned bodies are translated.

## Corrections and notes

- **Two dropped `this` arguments recovered.** `0x8074F600` opens with
  `bl 0x802DCD74` and `0x80637D20` with `bl 0x802DCD64`; those addresses are
  `_savegpr_27` and `_savegpr_23`, so `r3` survives and the exports' `void`
  signatures are wrong. The `mr r31,r3` in the raw words restores the argument,
  and both bodies are emitted as `fn_xxxxxxxx(this: usize)`.
- **`0x8074F600` export writes the branch arms as constants.** Ghidra folds the
  two lazy-init blocks into `DAT_80831d2c = 0; DAT_80831d2e = 0x2000; ...`,
  which is right, but it types the guards as `char` compares; the raw words are
  `lbz` + `extsb`-free byte tests against `0`, so `read_u8(..) == 0` is used.
- **`0x8069FF00` signed-byte ladder.** The export's
  `(double)CONCAT44(0x43300000, byte ^ 0x80000000) - DAT_807ca208` is the
  compiler's `signed char -> float` idiom, and `0x807CA208` reads as
  `0x4330000080000000` (`2^52 + 2^31`), the standard subtrahend. This is the same
  correction the file already records for `fn_8069F640` (`0x807C9868`) and
  `fn_806843B0`; the body writes `read_u8(0x8042C1A0 + ..) as i8 as f32`.
- **`0x806F7500` dead stack stores.** The raw words also store `0x8042BB0C`
  twice at `r1+8`/`r1+0xC` and the `256.0 + 0x8042BB08` sum twice at
  `r1+0x10`/`r1+0x14`. Nothing in the body reads that frame, so the four stores
  are omitted. The two `main.dol` scalars `0x8042BB08`/`0x8042BB0C` cannot be
  read out of the REL image and stay runtime `read_f32` calls.
- **`0x806F7500` FPR/GPR name aliasing.** The listing prints `stfs r5` / `stw r4`
  for the same register numbers; the float stores use `f4`/`f5` (`0.0`/`1.0`)
  while `0x570`/`0x56C` are *integer* stores of the `rlwinm` results. Both were
  separated by tracking which instruction last wrote the GPR.
- **`0x80637D20` record offsets agree with `fn_80637A40`.** `+0x7BC` is record
  `+0x00`, `+0x7DC`/`+0x7E0` are `+0x20`/`+0x24` (the `+0x24` sort key the
  existing doc names), `+0x7E4`/`+0x7E5`/`+0x7E6` are `+0x28`/`+0x29`/`+0x2A`
  and `+0x7F0` is `+0x34`. `0x80429FB0` is a *pointer* that is dereferenced, so
  `+0x48`/`+0x28`/`+0x08` are read with a four-byte lane stride off it while the
  `+0x58` byte is indexed by `table[lane]`.
- **`0x80637D20` asymmetric `+0x14`.** The export shows `+0x7D0` (`record+0x14`)
  written only in the "player exists" arm and never cleared in the other arm; the
  raw words `0x80637D94` and `0x80637DEC` confirm this, so the body preserves the
  asymmetry instead of zeroing the field.
- **`0x807B7F30` undefined `r3` at the two tail calls.** `0x800FFE70` and
  `0x800FFF90` are reached with `r3` holding a stale value that the image never
  defines on that edge, so they are emitted as `platform::call0`. The `x < 10` /
  `y < 0x2A` bounds and the `v < 8` counter bump are the raw `cmpli` limits.

## Verification

- `work/wave175/laneB_block.rs` type-checks standalone against stubbed helper
  signatures (`rustc --edition 2021 --crate-type lib`).
- All five evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch, field offset and
  call target was re-read from the raw words of `work/rels/elf/d_basesNP.elf`.
- The lane CSV is written against `src/game/bases.rs` as the brief requires; the
  bodies live in `work/wave175/laneB_block.rs` until the coordinator appends
  them.
