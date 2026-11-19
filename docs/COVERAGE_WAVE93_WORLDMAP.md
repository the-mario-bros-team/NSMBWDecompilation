# Coverage - Wave 93 (`game::worldmap`, `0x80130000` bucket remainder)

## How to read this file

Each row is one `pub unsafe fn` body appended to `src/game/worldmap.rs`, named
`fn_` plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; every evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the object pointer. `r13` is `SDA_BASE` (`0x8042F680`) and `r2` is `SDA2_BASE`
(`0x80433080`); the `r2` floats below were read from `work/dol/main.elf` at the
absolute addresses named in their constant comments. The export never names the
class, so the bodies keep literal offsets and the group is labelled by address.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 28 | 7,108 | 12 | 16 |

No body is `low` confidence.

## What this pass covers

- **`0x80137590..0x80137C10` - the trick entry and frame emitters.**
  `0x80137590` gates the action on `0x8004DD00()`, input bit 0 of `+0x10D4`
  and the record `0x803763B0`, then seeds the `+0x14A8` countdown.
  `0x801378B0` and `0x80137B60` build a 24-byte frame with `0x8006CED0` /
  `0x800D5790`, pick a colour from `0x802F4EC4` / `0x80370DA8`, and submit it
  through `0x8008FBA0` with the `r13 - 0x6300` / `r13 - 0x62F8` argument
  table. `0x80137C10` tests a candidate body (mode 4 moves by `12.0f` and
  `4.0f * side`) and posts the result word on `+0x294`.
- **`0x80138140..0x80138A90` - the five-state `+0x14A4` ladder.**
  `0x80138140` owns the `0.08f`/`0.15f` `+0x11C` value, the `+0x14AC`
  ten-frame countdown and the `17.0f` distance tests around `0x80168080` /
  `0x80169020`. `0x80138790` restores `+0x14A4` from `+0x14A0`, sets status
  bit `0x40` and starts the per-player `+0x2CE8` selector. `0x80138A90` ticks
  the `+0x2790`/`+0x2794` timers and drives states `0..=4` with input ids
  `0x53`/`0x95`/`0x54`/`0xBB`.
- **`0x80138D40..0x801391F0` - the four trick-input gates.**
  `0x80138D40` re-arms action `0x26` under a `0x78`-frame guard,
  `0x80138DD0`/`0x80138E60` consume `0x8035476C` and post `0x80376570` with
  payload `2`/`0`, `0x80138F40` runs the long input/global chain for payload
  `3`, and `0x801391F0` hands the interaction mode to the partner's `+0x11C`.
- **`0x80139420..0x80139D80` - the motion/reset and attach helpers.**
  `0x80139420` accumulates position plus offsets; `0x80139460` calls the
  skipped GQR record `0x801396C0` by raw address; `0x801394A0` creates the
  `0x19F` record and resets the model; `0x801395C0` clears `+0x2A6C`;
  `0x80139640` selects the map-object exit effect; `0x80139950`/`0x801399A0`
  are the `+0x1008` attach pair; `0x801397D0`/`0x80139A90`/`0x80139B80` run the
  partner hand-off and detach; `0x80139D00` copies one `0x44`-stride record and
  `0x80139D80` posts the `0x5C` float triple before entering state `1`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80137590` | `fn_80137590` | 180 | high |
| `0x801378B0` | `fn_801378b0` | 152 | medium |
| `0x80137950` | `fn_80137950` | 96 | high |
| `0x80137B60` | `fn_80137b60` | 168 | medium |
| `0x80137C10` | `fn_80137c10` | 528 | medium |
| `0x80138140` | `fn_80138140` | 976 | medium |
| `0x801386B0` | `fn_801386b0` | 148 | medium |
| `0x80138750` | `fn_80138750` | 56 | high |
| `0x80138790` | `fn_80138790` | 512 | medium |
| `0x80138990` | `fn_80138990` | 256 | high |
| `0x80138A90` | `fn_80138a90` | 680 | medium |
| `0x80138D40` | `fn_80138d40` | 136 | high |
| `0x80138DD0` | `fn_80138dd0` | 144 | high |
| `0x80138E60` | `fn_80138e60` | 224 | medium |
| `0x80138F40` | `fn_80138f40` | 564 | medium |
| `0x801391F0` | `fn_801391f0` | 448 | medium |
| `0x80139420` | `fn_80139420` | 52 | high |
| `0x80139460` | `fn_80139460` | 64 | high |
| `0x801394A0` | `fn_801394a0` | 288 | medium |
| `0x801395C0` | `fn_801395c0` | 124 | high |
| `0x80139640` | `fn_80139640` | 120 | high |
| `0x801397D0` | `fn_801397d0` | 372 | medium |
| `0x80139950` | `fn_80139950` | 72 | high |
| `0x801399A0` | `fn_801399a0` | 100 | high |
| `0x80139A90` | `fn_80139a90` | 232 | medium |
| `0x80139B80` | `fn_80139b80` | 196 | medium |
| `0x80139D00` | `fn_80139d00` | 128 | medium |
| `0x80139D80` | `fn_80139d80` | 92 | medium |

## Deliberate skips

- **`0x80137E20`, `0x80138510` and `0x801396C0`.** Each export is a
  GQR-reconstruction record (`unaff_GQR0`/`unaff_GQR3` float loads) or, in the
  case of `0x801396C0`, a record whose body depends on the same missing GQR
  base. They cannot be reconstructed from the C alone. `0x801396C0` is still
  reachable from `fn_80139460` through its raw address with the export's
  `(f64, f64, u32)` prototype.
- **`0x80137550`.** The export is only `FUN_80137590();` followed by
  `+0x14AC = 0`. The call drops all three arguments of `0x80137590` (delay and
  event payload), so the body could not be reconstructed without inventing
  them; the store alone is not a complete translation.
- **The rest of the `0x80130000` bucket.** This pass selected the contiguous
  `0x80137590..0x80139D80` run. The remaining entries outside it (for example
  `0x80130F40`, `0x801313D0`, `0x80131700`, `0x80131990`, `0x80131F20`,
  `0x80132340`, `0x8013A180`, `0x8013BF10`, `0x8013E850`, `0x8013F1C0`) stay
  untranslated for a later pass. `0x80130340`, `0x80130CE0`, `0x80133950`,
  `0x80133B60`, `0x80135070`, `0x80135810`, `0x801358E0`, `0x801359C0` and
  `0x801363C0` are GQR records already listed as skips by waves 60, 73 and 86.

## Verification

- Every address and byte size matches `functions.tsv`; the 28 sizes sum to
  7,108 bytes.
- Every evidence file exists at the cited `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 28 were untranslated
  entries of the `0x80130000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error or warning that names
  `src/game/worldmap.rs` (the build stops on other lanes' errors).
- `node tools/check-lane-csv.mjs wave93` reports no problem naming
  `wave93-worldmap.csv`.
