# Coverage - Wave 86 (`game::worldmap`, `0x80130000` bucket)

## How to read this file

Each row is one `pub unsafe fn` body in `src/game/worldmap.rs`, named `fn_`
plus its original `main.dol` address. Byte sizes come from
`work/ghidra/main-gb/export/functions.tsv`; the evidence path is the Ghidra C
export the body was transcribed from. Field offsets are literal offsets from
the actor pointer. `r2` is the shared `SDA2_BASE` (`0x80433080`); the
`r2`-relative float and double values were read from `work/dol/main.elf`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/worldmap.rs` | `game::worldmap` | 29 | 4556 | 16 | 13 |

## What this pass covers

- **`0x80130500`..`0x80130B20` - sea-land actor state entry.** `0x80130500`
  resets `+0x2061`, `+0x14A4`, `+0x205C` and `+0x2060` and sends controller
  sound `0x24` with a mode-selected second sample. `0x80130A70` derives the
  slide sound sample from `+0x10C`/`+0x110` (`1.0 + |speed| - 0.5625`, clamped
  to `[1.0, 2.0]`) and sends sound `0x26`, or sound `0x27` at zero speed.
  `0x80130B20` runs the existing `wm_sealand_smoke_effect` when `+0x10DC` bit
  `0` is clear, then enters state `1`.
- **`0x801335D0`..`0x801337E0` - slope contact entry.** `0x801335D0` clears
  status bits `0x49`/`0x4A`, ages `+0x1C9C` and posts the `0x80324EB0`
  four-entry transition record. `0x80133640`/`0x801336E0` resolve the
  `+0x1464` name component against `0x80375FF0`; the latter stores the contact
  point into `+0x1C7C`/`+0x1C80`/`+0x1C84`, copies the heading to `+0x1C90`,
  the height delta to `+0x1C88` and `0.7f` to `+0x1C8C`. `0x801337E0` is the
  pure name/state test.
- **`0x80133A60`..`0x80134050` - slide hand-off.** `0x80133A60` clears the
  four slope bits and restarts the `+0x114` curve pointer. `0x80133AE0`
  converts the signed `+0x10A0` timer with the compiler's
  `0x4330000080000000` bias pair, divides by `4096.0` and stores the new
  `+0xB0` height. `0x80133CB0` zeroes the motion fields and selects the
  `0x802F4E9C` speed by `+0x1090` (`4.5`, `10`, `10`, `1`). `0x80133F50`
  picks the `0.875`/`1.125` facing-scaled slide speed, sends `1.0`/`1.5`
  through slot `+0x8C` and dispatches the two jump-table states.
  `0x80134050` posts the exit event `0x80375CAC` or the `0x80324EE0`
  transition.
- **`0x801341B0`..`0x80134BC0` - landing ladder.** `0x801341B0` gates the
  `0x80376070` event on `+0x9D`, `+0x1DD4`, slot `+0x3C8`, `+0xEC <= 3.0` and
  `+0x10D8` bit `0x800`. `0x80134280` is the full sled reset: bits
  `0x35`/`0x97`/`0x9F`, six zero stores, `8.0 + 16.0 * floor(+0xAC / 16)` into
  `+0x1DCC`, `400.0` through slot `+0x42C`, then either the `0x2B` sound or
  the `0x2D` hand-off. The three 52-byte setters `0x801344E0`/`0x801345C0`/
  `0x80134700` enter states `1`/`2`/`3` with sounds `0x2C`/`0x2D`/`0x2E`.
  `0x80134850`/`0x801348F0` probe the contact with the facing-scaled base
  speed, and `0x80134990` advances the position or flips the facing for
  `+0x10D4` bits `0`/`0x180000`. `0x80134B10` and `0x80134BC0` are the two
  guarded `0x80375EAC` hints (`5`, `2`, `0`).
- **`0x801376D0`..`0x80137B20` - countdown state.** `0x801376D0` clears the
  motion words, sends sound `0x52`, reloads `+0x2518 = 8` and copies `+0x14A0`
  into `+0x14A4`. `0x801377A0` re-faces the actor through slot `+0x444`.
  `0x80137800` ages `+0x2518` and runs the `0x801378B0` emitter.
  `0x801379B0` clears bit `6`; `0x801379F0` posts `0x80375C2C(1)` when the
  `0x80137B20` gate reports zero; `0x80137A90` posts event `0x13C` and seeds
  `+0x14A8 = 0x3C`; `0x80137B20` ticks the companion emitter and returns the
  `+0x14A8 != 0` flag.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80130500` | `fn_80130500` | 172 | medium |
| `0x80130A70` | `fn_80130a70` | 172 | medium |
| `0x80130B20` | `fn_80130b20` | 148 | medium |
| `0x801335D0` | `fn_801335d0` | 112 | high |
| `0x80133640` | `fn_80133640` | 160 | medium |
| `0x801336E0` | `fn_801336e0` | 248 | medium |
| `0x801337E0` | `fn_801337e0` | 116 | high |
| `0x80133A60` | `fn_80133a60` | 124 | high |
| `0x80133AE0` | `fn_80133ae0` | 116 | medium |
| `0x80133CB0` | `fn_80133cb0` | 212 | medium |
| `0x80133F50` | `fn_80133f50` | 244 | medium |
| `0x80134050` | `fn_80134050` | 172 | high |
| `0x801341B0` | `fn_801341b0` | 196 | high |
| `0x80134280` | `fn_80134280` | 396 | medium |
| `0x801344E0` | `fn_801344e0` | 52 | high |
| `0x801345C0` | `fn_801345c0` | 52 | high |
| `0x80134700` | `fn_80134700` | 52 | high |
| `0x80134850` | `fn_80134850` | 152 | medium |
| `0x801348F0` | `fn_801348f0` | 148 | medium |
| `0x80134990` | `fn_80134990` | 276 | high |
| `0x80134B10` | `fn_80134b10` | 176 | high |
| `0x80134BC0` | `fn_80134bc0` | 192 | high |
| `0x801376D0` | `fn_801376d0` | 200 | high |
| `0x801377A0` | `fn_801377a0` | 92 | high |
| `0x80137800` | `fn_80137800` | 168 | medium |
| `0x801379B0` | `fn_801379b0` | 60 | high |
| `0x801379F0` | `fn_801379f0` | 156 | high |
| `0x80137A90` | `fn_80137a90` | 132 | high |
| `0x80137B20` | `fn_80137b20` | 60 | medium |

## Deliberate skips

- **GQR reconstruction records `0x80130340`, `0x80130CE0`, `0x80133950` and
  `0x80133B60`.** Their exports begin with the `unaff_GQR0`/`ldexpf` prologue
  and a removed unreachable block, so the float loads cannot be recovered
  without the live `GQR0` base. Same policy as waves 60/64.
- **Jump-table records `0x80133E10` and `0x80133EC0`.** Ghidra reports
  `Could not recover jumptable` at `0x80133E58`/`0x80134510` and drops the
  trailing controller/id arguments of the `+0x5C` sound call. The two bodies
  are still called by raw address from `fn_80133f50`, whose own call sites pass
  only `actor`.
- **`0x80137550`/`0x80137590`.** `0x80137550` calls `0x80137590` with no
  visible arguments while the callee consumes `param_2`/`param_3`; the
  register hand-off is not reconstructable from the export.
- **`0x801378B0`/`0x80137B60`.** Both build a 24-byte stack record through
  `0x8006CED0`/`0x800D5790` and `0x8008FBA0`, and `0x80137B60` reads the
  `r13 - 0x62F8` table. `fn_80137800` and `fn_80137b20` call them by raw
  address instead of inventing the record layout.
- **The rest of the bucket.** `0x80135810`, `0x801358E0`, `0x801359C0`,
  `0x801363C0`, `0x80137320`, `0x80137C10`, `0x80137E20`, `0x80138140`,
  `0x80138510`, `0x801386B0`, `0x80138750`, `0x80138790`, `0x80138990`,
  `0x80138A90`, `0x80138D40`, `0x80138DD0`, `0x80138E60`, `0x80138F40`,
  `0x801391F0`, `0x80139420`..`0x8013A880`, `0x8013B0B0`, `0x8013BB90`,
  `0x8013BF10`, `0x8013E850`, `0x8013EA90`..`0x8013F6E0` remain untranslated.

## Verification

- Every address and byte size matches `functions.tsv`; the 29 sizes sum to
  4,556 bytes.
- Every evidence file exists at the cited
  `work/ghidra/main-gb/export/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/worldmap.rs`.
- No address was already in `ledger/functions.csv`; all 29 were untranslated
  entries of the `0x80130000` bucket at the start of the pass.
- `cargo check --message-format=short` reports no error and no diagnostic that
  names `src/game/worldmap.rs`.
- `node tools/check-lane-csv.mjs wave86` reports `OK`;
  `wave86-worldmap.csv` has 29 rows with unique addresses and valid evidence
  paths.
