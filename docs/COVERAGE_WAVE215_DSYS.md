# Coverage - Wave 215 (`engine::dsys`, main.dol part 1)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/dsys.rs` | `engine::dsys` | 60 | 8,560 | 46 | 14 |

First 60 `engine::dsys` bodies, `0x800F0010`-`0x800F2BC0`. The module is now
**209 / 397**.

## What landed

- **Frame/metric bodies (lane A).** `0x800F0010`, `0x800F0090`, `0x800F0170`,
  `0x800F0510`, `0x800F0580`, `0x800F0630`, `0x800F0730`, the 376-byte
  `0x800F0860`, `0x800F09E0`, `0x800F0AD0`.
- **Start-name bodies (lane B).** `0x800F0B60`, `0x800F0C10`, `0x800F0CC0`,
  `0x800F0D50`, `0x800F0E20`, `0x800F0EB0`, `0x800F0EC0`, `0x800F0F30`,
  `0x800F0F70`, `0x800F0FC0`.
- **Record appenders (lane C).** `0x800F10B0`, four 8-byte leaves
  (`0x800F1150`-`0x800F11E0`), `0x800F11F0`, `0x800F1390`, `0x800F1550`,
  `0x800F17B0`, `0x800F1850`.
- **Metric records and node ctors (lane D).** `0x800F18B0`, `0x800F1940`,
  `0x800F1980`, `0x800F1A00`, `0x800F1AA0`, `0x800F1AB0`, three 8-byte leaves
  (`0x800F1B60`-`0x800F1B80`), `0x800F1B90`.
- **Track block (lane E).** `0x800F1BD0`, `0x800F1BE0`, `0x800F1BF0`,
  `0x800F1C60`, `0x800F1D10`, `0x800F1E10`, `0x800F1E30`, `0x800F1E90`,
  `0x800F1F80`, `0x800F20B0`.
- **Descriptor walk and table marshaller (lane F).** `0x800F21A0`,
  `0x800F22B0`, `0x800F22F0`, `0x800F24B0`, `0x800F2680`, `0x800F2710`,
  `0x800F2770`, `0x800F2AF0`, `0x800F2B60`, the 376-byte `0x800F2BC0`.

## Corrections and recoveries

- **`_savegpr` stubs leave `r3`/`r4` intact.** `bl 0x802DCD60..0x802DCD88` are
  pure `stw r26..r31,-N(r11)` + `blr`, so `0x800F0090`'s manager/key and
  `0x800F0860`'s two arguments are the functions' own incoming registers, not
  values "returned" by the stub as Ghidra's `undefined8` suggests.
- `0x800F0860` is `void(void)` in the export only because it has no xref; its
  body consumes `r3`/`r4`, so its args were recovered (medium).
- `0x800F0CC0` is 140 bytes of code in a 144-byte slot; its result leaves through
  the `0x802DCDC0` restore stub, which is why Ghidra prints `void`.
- `0x800F0EB0` has no `bclr`: it is `addi r3,r3,8` / `b 0x80168230`. Its true
  three-argument arity came from the call site in `0x800F0EC0`.
- `0x800F18B0`'s export contains `LAB_800f1920`, a destructor that is a separate
  **untabled** function (no `functions.tsv` row); translated as auxiliary
  `fn_800f1920` and documented rather than added to the ledger.
- `0x800F2BC0`'s flag lands at `record + 0x50`, not the next record: the `0x54`
  stride is relative to the `+4` base that `0x800F2AF0` fills.
- `0x800F2BC0`'s `0x800F9530` test clears the flag when the id is `>= 0`
  (Ghidra's `-1 < iVar4`), matching the `bge` target.
- `0x800F2710` ends with `stfs f0` from `0x8042CF30`, so `+0x134` is a float
  field while `+0x128`/`+0x12C`/`+0x130` are word copies.
- `0x800F0730`'s `cror 2,0,2` + `rlwinm. 3,31,31` + `bc 4,2` decodes to the
  `|value| <= tolerance` branch; neighbouring `cror 2,1,2` sites read as `>=`.
- Record families mapped by stride and base: `+0xD8` (0x48), `+0x688` (0x14),
  `+0x548` (0x14), with counts at `+0x7CC`, `+0x7D0`, `+0x7C8`.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-lane-csv.mjs wave215`: 60 rows, 60 unique, 46 high + 14 medium.
- Ledger: 19,730 rows (97.42%); `engine::dsys` 209/397.
- `scripts/check.ps1` exit 0.

## Next

188 `engine::dsys` bodies, then `src/resource/archive.rs` (198),
`src/game/stage.rs` (132) and two `ui::hud` stragglers.
