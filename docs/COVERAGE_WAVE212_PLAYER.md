# Coverage - Wave 212 (`game::player`, main.dol part 3)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 60 | 11,972 | 48 | 12 |

Third 60 `game::player` bodies, `0x80060010`-`0x80069270`.

## What landed

- **Roster and spawn helpers (lane A).** `0x80060010`, `0x80064C10`,
  `0x80065520`, `0x80066740`, `0x800667B0`, `0x80066820`, `0x800668F0`,
  `0x800669B0`, the 540-byte `0x80066A40` and the 572-byte `0x80066C60`.
- **Constructor and release bodies (lane B).** `0x80066EA0`, `0x80066EC0`,
  `0x80066F70`, `0x80066FB0`, `0x80066FC0`, `0x800670A0`, `0x800670E0`,
  `0x80067140`, `0x800671B0`, `0x80067260`.
- **State registration and destructors (lane C).** `0x80067270`, `0x80067510`,
  `0x80067550`, five 48-byte `0x802DCBFC` forwarders (`0x80067740`-`0x80067800`),
  the 492-byte `0x80067830` (`dActorState_c` / `dActorMultiState_c`
  registration) and `0x80067A80`.
- **Hit/compare walkers (lane D).** `0x80067B40`, `0x80067C80`, `0x80067D60`,
  `0x80067E40`, `0x80067F20`, `0x80068070`, the 4-byte `0x800680A0`,
  `0x80068290`, `0x800682F0`, `0x80068310`.
- **Large sweeps (lane E).** the 768-byte `0x80068320`, 552-byte `0x80068620`,
  844-byte `0x80068850`, then the ramp getter pairs `0x80068BA0`/`0x80068BC0`,
  `0x80068C30`/`0x80068C50`, `0x80068CC0`/`0x80068CE0` and `0x80068D50`.
- **Spawn/nearest bodies (lane F).** `0x80068D70`, the 576-byte `0x80068DE0`,
  `0x80069020`, `0x80069060`, `0x800690C0`, the 4-byte `0x800690E0`,
  `0x800690F0`, `0x80069110`, `0x80069240`, the 360-byte `0x80069270`.

## Corrections and recoveries

- **Signed float conversions.** The bias pair at `0x8042BBC8` is
  `0x43300000_80000000` (`2^52 + 2^31`), so `xoris` plus subtract yields
  `x as i32 as f32`, not an unsigned conversion. This applies to the counter in
  `0x80068D70` and all five conversions in `0x80068DE0`.
- `0x80066A40`: the `mode == 2` test is on the second register
  (`cmpi cr0,27,r27,2`), and the `cntlzw` word selects `count == 1`.
- `0x80066C60`: `xori`/`srawi`/`and`/`subf` decodes to `count >= 2`.
- `0x80064C10`'s descriptor is `0x80308FF0`; `0x80066820` clears with `0.0f`
  (`0x8042BBB8`), not the adjacent `-8000.0f`.
- `0x80065520` seeds `best` from `1.0e9` (`0x8042BB38`); its `ldexpf` prologue
  is an `f31` save.
- `0x800680A0` is exactly 4 bytes: `0x4BFFFFD0` (`b 0x80068070`) plus padding.
- `0x80068310`'s `rlwinm r3,r0,27,5,31` is `cntlzw >> 5`.
- `0x80069270` reads a live-in `f30` it never writes (a caller-supplied
  threshold); modelled as an explicit `near: f32` parameter.
- `0x80068DE0` returns the spawned actor or `0` although Ghidra types it `void`.
- The five forwarders' `r12` descriptors are at `+0x18 / +0x0C / +0x24 / +0x18 /
  +0x0C`, a per-thunk detail wave 106 could not state. Only `r12` is
  unmodellable through `call3`, so they carry explicit `// partial:` comments.
- `0x800690E0`'s export is named `800690e0_thunk_FUN_8016a8e0.c`; the CSV cites
  that real filename.
- `0x80068070`'s triple branch (`bc 4,2` / `bclr 12,0` / `bclr 12,1`) is
  `a == b && d >= e && c <= f`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave212`: 60 rows, 60 unique, 48 high + 12 medium.
- Ledger: 19,557 rows (96.57%); `game::player` 717/818 (87.65%).
- `scripts/check.ps1` exit 0.

## Next

113 `game::player` bodies remain, then `src/engine/dsys.rs` (248),
`src/resource/archive.rs` (198), `src/game/stage.rs` (132) and the two
`ui::hud` stragglers.
