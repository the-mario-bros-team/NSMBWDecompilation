# Coverage - Wave 81 (`game::worldmap`, `0x80100000` bucket remainder)

## How to read this file

Every row is one `pub unsafe fn` body in `src/game/worldmap.rs`, named `fn_`
plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the object pointer. `r13` is the shared `SDA_BASE` (`0x8042F680`) and `r2` is
the shared `SDA2_BASE` (`0x80433080`); both are already documented in the
module header.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 30 | 4420 | 16 | 14 |

## What this pass covers

- **`0x80109350`..`0x80109A50` - constructor, loader and load machine.** The
  group is the `HomeButton2` helper class: `0x80109350` builds it (six
  `0x8008F140` records, `0x40`-byte load block at `+0xA0`, `0x40`-byte string
  block at `+0xE0`, `0xB0`-byte scratch block at `+0x120`, `0x20`-byte
  allocation at `+0x84`), `0x80109480` performs the six `0x8008F1B0` loads and
  starts the animation bank, and `0x80109660` drives the three-state machine at
  `+0x1D0` (`0` idle, `1` running, `2` waiting, `3` shutting down).
  `0x80109950` seeds the region progress fraction and `0x80109A50` posts the
  message-bank request.
- **`0x80109AD0`..`0x80109B10` - the five embedded name strings.** One pointer
  store each: `HomeButton2_SpeakerSe.arc`, `home.csv`, `config.txt`,
  `HomeBtnIcon.brtex`, `HomeButtonSe.arc` at `+0x8C..+0x9C`. The string
  addresses (`0x80322230`, `0x8032224C`, `0x80322264`, `0x8032227C`,
  `0x8032229C`) were confirmed in `main.elf`.
- **`0x80109B20`, `0x80109C00`, `0x8010A030` - the `0x80377C88` status table.**
  Four entries walked between `0x802DCD68`/`0x802DCDB4` (refresh, writing
  `+0xE0..+0xEC` of each `0x10`-byte record), `0x802DCD74`/`0x802DCDC0`
  (vtable slot `0x20` with `0x8000`) and four `0x802BC9C0` releases.
- **`0x80109C80`, `0x80109D10`, `0x8010A090` - load-step completion.** The
  teardown runs the `+0x1E4` callback and the `0x8016F660`/`0x8016F640` pause
  snapshot; the poller runs the `+0x1E8`/`+0x1EC` callbacks and the
  `0x80108380`/`0x80108390`/`0x801083A0`/`0x801083B0` branches; the request
  gate accepts a stop only in state `2` with `+0x1D4 == 0`.
- **`0x80109E70`, `0x80109EC0`, `0x8010A010`, `0x8010A0F0` - fade timer and
  flag word.** The timer converts `(now - start) / (bus_clock / 4000)` with the
  compiler's `2^52` idiom and walks a piecewise curve with the `r2` constants
  `250.0`, `0.004`, `1250.0`, `1500.0` and `1.0`. `0x8010A0F0` rebuilds the
  `+0x1D8` bits `0x1`/`0x2`/`0x4`/`0x10`/`0x80` from five independent tests,
  including the mode halfword at `r13 - 0x6F4E`.
- **`0x8010A2C0`, `0x8010A350` - two 16-byte palette tables.** Both fill
  `0x80374970`/`0x80374980` once (guarded by `r13 - 0x53C8`/`-0x53C7`) and then
  copy the word selected by the `r13 - 0x545B` byte.
- **`0x8010A6C0`, `0x8010A7B0`, `0x8010A8A0`, `0x8010AA60`..`0x8010AA80` -
  GX FIFO emitter writers and thunks.** The two writers are byte-identical
  (`0xCC008000 = 1/1024 * ((param & 0x3E0) + 4)`, plus `24/1024` when bit
  `0x800` is clear); the third adds an explicit mode branch; the three
  four-byte thunks branch to them unchanged.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80109350` | `fn_80109350` | 296 | medium |
| `0x80109480` | `fn_80109480` | 468 | medium |
| `0x80109660` | `fn_80109660` | 208 | medium |
| `0x80109730` | `fn_80109730` | 80 | high |
| `0x80109950` | `fn_80109950` | 236 | medium |
| `0x80109A40` | `fn_80109a40` | 16 | high |
| `0x80109A50` | `fn_80109a50` | 120 | medium |
| `0x80109AD0` | `fn_80109ad0` | 16 | high |
| `0x80109AE0` | `fn_80109ae0` | 16 | high |
| `0x80109AF0` | `fn_80109af0` | 16 | high |
| `0x80109B00` | `fn_80109b00` | 16 | high |
| `0x80109B10` | `fn_80109b10` | 16 | high |
| `0x80109B20` | `fn_80109b20` | 224 | medium |
| `0x80109C00` | `fn_80109c00` | 128 | medium |
| `0x80109C80` | `fn_80109c80` | 144 | medium |
| `0x80109D10` | `fn_80109d10` | 344 | medium |
| `0x80109E70` | `fn_80109e70` | 68 | high |
| `0x80109EC0` | `fn_80109ec0` | 328 | medium |
| `0x8010A010` | `fn_8010a010` | 20 | high |
| `0x8010A030` | `fn_8010a030` | 92 | high |
| `0x8010A090` | `fn_8010a090` | 88 | high |
| `0x8010A0F0` | `fn_8010a0f0` | 460 | medium |
| `0x8010A2C0` | `fn_8010a2c0` | 136 | medium |
| `0x8010A350` | `fn_8010a350` | 132 | medium |
| `0x8010A6C0` | `fn_8010a6c0` | 236 | high |
| `0x8010A7B0` | `fn_8010a7b0` | 240 | high |
| `0x8010A8A0` | `fn_8010a8a0` | 264 | medium |
| `0x8010AA60` | `fn_8010aa60` | 4 | high |
| `0x8010AA70` | `fn_8010aa70` | 4 | high |
| `0x8010AA80` | `fn_8010aa80` | 4 | high |

## Deliberate skips

- **`0x80109780` and `0x80109A40`-adjacent stack record (`0x801097F0`).**
  `0x80109780` builds a `0x88`-byte stack record (`local_88`, `local_84`,
  `local_70`, `local_6C`, `local_54`) and hands it to `0x802D0CC0`/`0x802D0F40`;
  `0x801097F0` does the same with a `0xD8`-byte record plus four `r2` floats and
  a `0x802D1850` rectangle call. Ghidra does not show the record's field roles,
  so a Rust body would have to invent the layout. `0x80109730` still calls
  `0x80109780` at its real address and says so in its doc block.
- **`0x8010A3E0`.** The 724-byte description of the first layout screen calls
  twenty-two `0x801C8xxx` functions with literal arguments but also builds a
  four-byte colour through `CONCAT31` byte shuffling and passes `&local_18`
  from an uninitialised-looking slot. The call sequence is reproducible, but
  the stack colour records are not, so the body was left out rather than
  guessed. Its two palette sources (`0x8010A2C0`/`0x8010A350`) are translated.
- **`0x80108F70`.** The 752-byte sibling of `0x801097F0` (64-byte `auStack_60`
  matrix, `local_68`/`local_64`) has the same unnamed stack-record problem.
- **The rest of the bucket.** `0x80101AA0`, `0x80102DB0`, `0x80103410`,
  `0x80106620`, `0x801068B0`, `0x80107050`..`0x80107460`, `0x80108F40`, and
  `0x8010A3E0`..`0x8010FF70` remain untranslated. They are not part of this
  contiguous pick; none of them is referenced by the 30 bodies above except
  `0x80107850` and `0x80107F10`, which were already translated in wave 67 and
  are called through their existing constants.

## Verification

- Every address and byte size matches `functions.tsv`; the 30 sizes sum to
  4,420 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path
  (the three `0x8010AA6x`/`AA7x`/`AA8x` records use their `thunk_FUN_*`
  filenames).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 30 were untranslated
  entries of the `0x80100000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no diagnostic that
  names `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave81` reports `OK`; `wave81-worldmap.csv`
  has 30 rows with unique addresses and valid evidence paths.
