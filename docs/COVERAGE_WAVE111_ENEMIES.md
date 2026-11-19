# Coverage - Wave 111 (`game::enemies`, the `0x80AD0000` bucket)

## How to read this file

This page records the Wave 111 `game::enemies` lane: 29 functions of the
`0x80AD0000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 92 of that file. Every row of the machine
readable mirror (`ledger/agents/wave111-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

The pass covers two neighbouring actors that close the bucket. Neither has a
`StateID` table in this export, so both are described by the model strings and
by the state records they are switched to:

- **Parachute actor (`0x840` bytes)**, `0x80ADC440..0x80ADCC80`. Named by the
  `para_drop` (`0x80B5EDD4`), `para_receive` (`0x80B5EDF0`) and `walk`
  (`0x80B5EE00`) clips plus the `root_parachute` joint (`0x80B5EDE0`). Its
  constructor and `g3d` binder belong to an earlier pass
  (`fn_80adc0f0` / `0x80ADC0F0`).
- **Winged-Kuribo actor (`0x8E8` bytes)**, `0x80ADE970..0x80ADFDA0`. Named by
  the `wing_kuri` node (`0x80B5F69C`), the `jnt_wing` joint (`0x80B5F6A8`) and
  the `walk` clip (`0x80B5F6B4`). Its constructor is in this pass
  (`0x80ADEAF0`, [`fn_80adeaf0`]).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 5,032 | 15 | 10 |

The remaining four bodies are `low`: the three 48-byte argument-swapping
forwarders (`0x80ADEA90`, `0x80ADEAC0`, `0x80ADFD70`, `0x80ADFDA0`) keep the
same unestablished `0x802DCBFC` target as the four copies already in the file,
so only their literal reshuffle is asserted.

## What this pass covers

- **`0x80ADC440..0x80ADC830`, the parachute placement helpers.** The deployed
  matrix refresh with the `+0x814` lift and the `+0x7EA`/`+0x7E8`/`+0x7E6`
  angle triple (`0x80ADC440`); the `root_parachute` node probe whose `+0x0C`
  word indexes the holder matrix (`0x80ADC500`, `0x80ADC570`); the lean/yaw
  sway stepper over the `{27,-27}` / `{272,-272}` tables and the `±0xC0` yaw
  walk towards `±0xAAA` (`0x80ADC5F0`); the twenty-sample roll ring at
  `+0x7EC..+0x812` (`0x80ADC790`); the `+0xEC` spring between `-0.4` and
  `-0.32` (`0x80ADC720`); and the `+0x394` state probe with the `0xFF` bytes
  and the 240-frame retry (`0x80ADC830`).
- **`0x80ADC920..0x80ADCC80`, the parachute states.** The `para_drop` entry
  (`0x80ADC920`), the deployed frame step that rebuilds `+0xAC` from
  `14.0 * roll / 4096.0` and copies the pitch from the `+0x800` sway
  (`0x80ADCA00`), the `para_receive` entry with the loop-once flag
  (`0x80ADCB00`), the attached frame step (`0x80ADCBC0`) and the `walk` entry
  (`0x80ADCC80`).
- **`0x80ADE970..0x80ADEFC0`, the winged actor's construction and rig.** The
  release helper and its two forwarders (`0x80ADE970`, `0x80ADEA90`,
  `0x80ADEAC0`); the `0x8E8` constructor with the interim vtable `0x80304938`,
  the two holder layers, the `+0x5F8` helper and the `+0x720` sub-object
  (`0x80ADEAF0`); the damage entry that arms `0x8006D090` and picks the
  per-variant height from `0x8042BB20[+0x391]` (`0x80ADEC50`); the `jnt_wing`
  matrix step and its node probe (`0x80ADEDE0`, `0x80ADEEE0`); the state
  re-check (`0x80ADEED0`); and the guard release through the `+0x2D4` hook
  (`0x80ADEFC0`).
- **`0x80ADF030..0x80ADF850`, the winged frame steps.** The facing/speed/state
  step over `0x80B30B90` and `0x8042C1A4` (`0x80ADF030`); the contact callback
  that retires the rig on a slow impact (`0x80ADF160`); the walk entry with
  both clip binders and the `0x800B2EE0` frame (`0x80ADF4B0`); the flap step
  with the `+0x8D8`/`+0x8DA` timer and the `3.0`/`1.25` phases
  (`0x80ADF620`); and the turning step over `0x8015F4F0` (`0x80ADF850`).
- **`0x80ADFC50..0x80ADFDA0`, the closing trio.** A third copy of the release
  helper and a second pair of 48-byte forwarders.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80ADC440` | `fn_80adc440` | 192 | high |
| `0x80ADC500` | `fn_80adc500` | 112 | medium |
| `0x80ADC570` | `fn_80adc570` | 60 | medium |
| `0x80ADC5F0` | `fn_80adc5f0` | 300 | high |
| `0x80ADC720` | `fn_80adc720` | 100 | medium |
| `0x80ADC790` | `fn_80adc790` | 160 | high |
| `0x80ADC830` | `fn_80adc830` | 220 | medium |
| `0x80ADC920` | `fn_80adc920` | 180 | high |
| `0x80ADCA00` | `fn_80adca00` | 256 | medium |
| `0x80ADCB00` | `fn_80adcb00` | 168 | high |
| `0x80ADCBC0` | `fn_80adcbc0` | 184 | high |
| `0x80ADCC80` | `fn_80adcc80` | 176 | high |
| `0x80ADE970` | `fn_80ade970` | 88 | high |
| `0x80ADEA90` | `fn_80adea90` | 48 | low |
| `0x80ADEAC0` | `fn_80adeac0` | 48 | low |
| `0x80ADEAF0` | `fn_80adeaf0` | 352 | high |
| `0x80ADEC50` | `fn_80adec50` | 196 | high |
| `0x80ADEDE0` | `fn_80adede0` | 124 | high |
| `0x80ADEE60` | `fn_80adee60` | 112 | medium |
| `0x80ADEED0` | `fn_80adeed0` | 240 | medium |
| `0x80ADEFC0` | `fn_80adefc0` | 112 | high |
| `0x80ADF030` | `fn_80adf030` | 304 | medium |
| `0x80ADF160` | `fn_80adf160` | 124 | medium |
| `0x80ADF4B0` | `fn_80adf4b0` | 344 | medium |
| `0x80ADF620` | `fn_80adf620` | 468 | high |
| `0x80ADF850` | `fn_80adf850` | 180 | high |
| `0x80ADFC50` | `fn_80adfc50` | 88 | high |
| `0x80ADFD70` | `fn_80adfd70` | 48 | low |
| `0x80ADFDA0` | `fn_80adfda0` | 48 | low |

## Deliberate skips

The bucket's other 41 untranslated addresses were left alone; the ones closest
to this pass are listed so a later lane does not have to rediscover them.

- **`0x80ADDA40`, `0x80ADDAC0`, `0x80ADE240`, `0x80ADE3A0` and
  `0x80AD3C10`, `0x80AD46C0`.** Each body is wrapped in a GQR conversion block
  (`unaff_GQR0`/`unaff_GQR3`, `ldexpf`) and in two cases the value that reaches
  the caller is only expressible through that quantisation, exactly like the
  `0x80B23480` skip recorded in `docs/COVERAGE_WAVE110_ENEMIES.md`.
- **`0x80AD42B0` (36 bytes).** The export ends in `halt_baddata()` after a bad
  instruction, so no honest body is possible.
- **`0x80AD3D50`, `0x80AD4000`, `0x80AD42E0`, `0x80AD4420`, `0x80AD45E0`,
  `0x80AD47E0`, `0x80AD4850`, `0x80AD48C0`.** A contiguous actor block around
  the `+0x81C`/`+0x84C`/`+0x854` state words. Its two big frame handlers are
  still readable, but the block calls `0x80AD42B0`, so translating it now
  would leave every body pointing at an unreconstructed switch. Left for a
  pass that can take the whole block.
- **`0x80AD0960` and `0x80AD0A90`.** The two remaining helpers of the
  quicksand block (`0x80AD0070..0x80AD1DF0`, wave 51). Both are GQR-shaped
  (`0x80AD0960` is 296 bytes of conversion arithmetic) and were not needed for
  this lane.
- **`0x80AD5760`, `0x80AD5CD0`, `0x80AD6CA0`, `0x80AD7310`, `0x80AD8900`,
  `0x80AD98B0`, `0x80ADA6E0`.** Isolated helpers of the `0x4C30` container
  family and the `obj_obake_door` neighbourhood. They are not part of either
  actor above and each needs its own signature study.
- **The `block_pata` block (`0x80ADD0E0..0x80ADDD10`).** The `g3d/block_pata`
  actor's release helper, forwarders, `0x898` constructor, `fly_normal`/
  `fly_sea` binders and frame handlers. Four of its members are the GQR bodies
  listed above and its state entry `0x80ADD350` mixes a `+4` spawn-word decode
  with a second GQR conversion, so the block was deferred whole rather than
  split across two lanes.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  5,032 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 29 was in
  `ledger/functions.csv` (70 of the bucket's 189 rows were untranslated before
  this pass, 41 after it).
- `node tools/check-lane-csv.mjs wave111` prints
  `wave111-enemies.csv: 29 rows` and `OK`.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 92, the cluster count
  (ninety-one to ninety-two), the function totals (2,442 to 2,471 public
  translations, 2,371 to 2,400 `fn_<addr>` bodies) and the records list.
