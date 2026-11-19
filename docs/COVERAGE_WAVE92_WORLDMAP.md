# Coverage - Wave 92 (`game::worldmap`, `0x80100000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); the three `r2` floats below were read from
`work/dol/main.elf` at the absolute addresses named in their constant comments.
The export never names the class, so the groups are labelled by address and the
bodies keep literal offsets.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 29 | 4,076 | 15 | 14 |

No body is `low` confidence.

## What this pass covers

- **`0x8010C780`..`0x8010CFA0` - the record walkers and arrays.** `0x8010C780`
  and `0x8010C9A0` walk a `0x264`-stride array (active byte `+0x304`, four
  children at `+0x2E0..+0x2EC`). The controller also owns a 30-entry
  `0x224`-stride array at `+0x148` (cursor `+0x4A10`, fade pair
  `+0x304`/`+0x308`) and a four-entry `0x224`-stride array at `+0x4180`
  (cursor `+0x4A14`, active byte `+0x439C`, fade pair
  `+0x433C`/`+0x4340`). `0x8010C780` closes one record and clears bit 0 of its
  child's `+0xBB` byte; `0x8010C9A0` copies the `+0xB9` byte into the four
  child objects; `0x8010CEF0` and `0x8010CFA0` advance the two cursors and
  seed `0.8f`/`1.0f`/`1.1f` from `0x8042D340`/`0x8042D33C`/`0x8042D348`.
- **`0x8010CA00`/`0x8010CA90`/`0x8010D1A0`/`0x8010D2D0` - the two
  constructors and their destructors.** `0x8010CA00` installs vtable
  `0x80322830` at `+0x60`, builds the two arrays with the `0x80014F10`/
  `0x80014F80` ctor/dtor pair and publishes itself at `r13 - 0x53B0`.
  `0x8010D1A0` installs vtable `0x803228B8`, builds the five sub-objects at
  `+0x74`, `+0x260`, `+0x44C`, `+0x638` and `+0x8DC`, and builds the embedded
  `+0xB0C` state object with interface tables `0x80322938`/`0x80322968`/
  `0x80322980`/`0x80322908`. The allocator wrappers `0x8010C9D0`/`0x8010D160`
  and the destructors `0x8010CA90`/`0x8010D2D0`/`0x8010D310`/`0x8010D370`
  complete the pair.
- **`0x8010D6D0` - the four-subobject readiness chain.** A short-circuit over
  `0x80019E00(+0x74)`, `0x80019110(+0x260)`, `0x8001AB20(+0x44C)` and
  `0x800162E0(+0x638)`.
- **`0x8010D760`..`0x8010DB90` - the `+0xB0C` state machine.** Controller
  records selected through `0x800B4710` feed a four-bit mask; state `+0xB68`
  moves to `3`/`4`/`5`; the `+0xB0C` vtable posts `0x80375008`,
  `0x80374F48`/`0x80374F88`/`0x80374F08`, `0x80375088` and `0x80374EC8`.
  `0x8010D8C0`/`0x8010D970`/`0x8010DA20` are the three per-player entry
  selectors; they release the four `0x80377C88` entries first, then select
  `+0x260`/`+0x44C`/`+0x74` and bind the handle at `+0x440`/`+0x62C`/`+0x254`.
- **`0x8010DFE0`..`0x8010E490` - the per-subobject update/release handlers.**
  They share the `r13 - 0x53A0` busy word, the `+0xB8C` selection byte, the
  `+0xB09`/`+0xB8D`/`+0xB8E`/`+0xB8F`/`+0xB90` flags and the five state words
  `+0xB68..+0xB70`/`+0xB7C..+0xB88`. `0x8010E490` is the full release path and
  is also called by the existing `fn_801082d0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8010C780` | `fn_8010c780` | 108 | medium |
| `0x8010C9A0` | `fn_8010c9a0` | 48 | high |
| `0x8010C9D0` | `fn_8010c9d0` | 48 | high |
| `0x8010CA00` | `fn_8010ca00` | 144 | high |
| `0x8010CA90` | `fn_8010ca90` | 164 | high |
| `0x8010CEF0` | `fn_8010cef0` | 176 | medium |
| `0x8010CFA0` | `fn_8010cfa0` | 172 | medium |
| `0x8010D160` | `fn_8010d160` | 48 | high |
| `0x8010D190` | `fn_8010d190` | 16 | high |
| `0x8010D1A0` | `fn_8010d1a0` | 228 | high |
| `0x8010D2D0` | `fn_8010d2d0` | 64 | high |
| `0x8010D310` | `fn_8010d310` | 96 | high |
| `0x8010D370` | `fn_8010d370` | 100 | high |
| `0x8010D6D0` | `fn_8010d6d0` | 132 | high |
| `0x8010D760` | `fn_8010d760` | 316 | medium |
| `0x8010D8C0` | `fn_8010d8c0` | 176 | medium |
| `0x8010D970` | `fn_8010d970` | 176 | medium |
| `0x8010DA20` | `fn_8010da20` | 176 | medium |
| `0x8010DAD0` | `fn_8010dad0` | 88 | medium |
| `0x8010DB50` | `fn_8010db50` | 60 | high |
| `0x8010DB90` | `fn_8010db90` | 228 | medium |
| `0x8010DFE0` | `fn_8010dfe0` | 152 | medium |
| `0x8010E0A0` | `fn_8010e0a0` | 112 | medium |
| `0x8010E130` | `fn_8010e130` | 112 | medium |
| `0x8010E1C0` | `fn_8010e1c0` | 160 | medium |
| `0x8010E280` | `fn_8010e280` | 100 | high |
| `0x8010E310` | `fn_8010e310` | 196 | medium |
| `0x8010E400` | `fn_8010e400` | 120 | high |
| `0x8010E490` | `fn_8010e490` | 360 | high |

## Deliberate skips

- **`0x8010DB30`.** The export records
  `WARNING: Could not recover jumptable at 0x8010DB4C. Too many branches` and
  treats the indirect jump as a call, so the real tail of the function is not
  present in the C. Only the `+0xB8C = 1` store and the event post would be
  reproducible, which is not a complete body. The four callers keep the raw
  address, and this file declares `W92_FUN_8010DB30` for them.
- **The remaining untranslated entries of the `0x80100000` bucket.** This pass
  selected the contiguous `0x8010C780..0x8010E490` run. The other 60 entries
  stay untranslated for a later pass; examples are `0x80101AA0`, `0x80102DB0`,
  `0x80103410`, `0x80106620`, `0x801068B0`, `0x80107050`, `0x80107170`,
  `0x80107460`, `0x80108F40`, `0x80108F70`, `0x80109780`, `0x801097F0`,
  `0x8010A3E0`, `0x8010A9B0`, `0x8010AA30`, `0x8010AA40`, `0x8010AA90`,
  `0x8010AAA0`, `0x8010ACC0`, `0x8010AD00` and `0x8010AF70`. None is claimed
  here.
- **`0x8010A3E0`** remains the stack-record layout description that wave 81
  already listed as a skip; this pass does not change that decision.

## Verification

- Every address and byte size matches `functions.tsv`; the 29 sizes sum to
  4,076 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 29 were untranslated
  entries of the `0x80100000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs` (the build still stops on other lanes' errors).
- `node tools/check-lane-csv.mjs wave92` reports `OK` for
  `wave92-worldmap.csv`.
