# Coverage - Wave 35 salvage (repo-wide body audit)

## What this file records

While Wave 35 was running, `tools/audit-unledgered.mjs` scanned every Rust
module outside the five lane write scopes for `pub unsafe fn` bodies that have
a complete `# Original` block but no row in `ledger/functions.csv`. It found
**59 real translations** that had never been ledgered. They were verified
against `functions.tsv` and their evidence files, then recovered here.

This is not new code. It is work that existed on disk but was missing
from the ledger, so the measured total understated the real coverage.

## Summary

| File | Module | Bodies | Bytes | Address range |
|---|---|---:|---:|---|
| `src/render/gx_band.rs` | `render::gx_band` | 25 | 3,596 | `0x80281D80`..`0x802AEE70` |
| `src/runtime/dvd.rs` | `runtime::dvd` | 23 | 5,820 | `0x801D00A0`..`0x801D36F0` |
| `src/engine/dsys.rs` | `engine::dsys` | 7 | 852 | `0x800E5060`..`0x800E5480` |
| `src/engine/frame.rs` | `engine::frame` | 3 | 24 | `0x80107850`..`0x80109340` |
| `src/ui/menu.rs` | `ui::menu` | 1 | 96 | `0x801735B0`..`0x801735B0` |
| **total** | | **59** | **10,388** | |

## Recovered functions

### `src/render/gx_band.rs`

| address | function | bytes | confidence | evidence |
|---|---|---:|---|---|
| `0x80281D80` | `fn_80281d80` | 236 | high | `work/ghidra/main-gb/export/80281d80_FUN_80281d80.c` |
| `0x80281E70` | `fn_80281e70` | 128 | high | `work/ghidra/main-gb/export/80281e70_FUN_80281e70.c` |
| `0x80282EF0` | `fn_80282ef0` | 124 | high | `work/ghidra/main-gb/export/80282ef0_FUN_80282ef0.c` |
| `0x80282F70` | `fn_80282f70` | 112 | high | `work/ghidra/main-gb/export/80282f70_FUN_80282f70.c` |
| `0x80282FF0` | `fn_80282ff0` | 164 | high | `work/ghidra/main-gb/export/80282ff0_FUN_80282ff0.c` |
| `0x80283260` | `fn_80283260` | 152 | high | `work/ghidra/main-gb/export/80283260_FUN_80283260.c` |
| `0x80283300` | `fn_80283300` | 176 | high | `work/ghidra/main-gb/export/80283300_FUN_80283300.c` |
| `0x802833B0` | `fn_802833b0` | 104 | high | `work/ghidra/main-gb/export/802833b0_FUN_802833b0.c` |
| `0x80283530` | `fn_80283530` | 112 | high | `work/ghidra/main-gb/export/80283530_FUN_80283530.c` |
| `0x80287000` | `fn_80287000` | 300 | high | `work/ghidra/main-gb/export/80287000_FUN_80287000.c` |
| `0x80289070` | `fn_80289070` | 336 | high | `work/ghidra/main-gb/export/80289070_FUN_80289070.c` |
| `0x802945E0` | `fn_802945e0` | 248 | high | `work/ghidra/main-gb/export/802945e0_FUN_802945e0.c` |
| `0x802A1860` | `fn_802a1860` | 64 | high | `work/ghidra/main-gb/export/802a1860_FUN_802a1860.c` |
| `0x802A88E0` | `fn_802a88e0` | 72 | high | `work/ghidra/main-gb/export/802a88e0_FUN_802a88e0.c` |
| `0x802AB550` | `fn_802ab550` | 296 | high | `work/ghidra/main-gb/export/802ab550_FUN_802ab550.c` |
| `0x802AB7C0` | `fn_802ab7c0` | 80 | high | `work/ghidra/main-gb/export/802ab7c0_FUN_802ab7c0.c` |
| `0x802AB810` | `fn_802ab810` | 60 | high | `work/ghidra/main-gb/export/802ab810_FUN_802ab810.c` |
| `0x802AC080` | `fn_802ac080` | 52 | high | `work/ghidra/main-gb/export/802ac080_FUN_802ac080.c` |
| `0x802AC1E0` | `fn_802ac1e0` | 256 | medium | `work/ghidra/main-gb/export/802ac1e0_FUN_802ac1e0.c` |
| `0x802AC460` | `fn_802ac460` | 28 | high | `work/ghidra/main-gb/export/802ac460_FUN_802ac460.c` |
| `0x802AC480` | `fn_802ac480` | 188 | high | `work/ghidra/main-gb/export/802ac480_FUN_802ac480.c` |
| `0x802AC7F0` | `fn_802ac7f0` | 52 | high | `work/ghidra/main-gb/export/802ac7f0_FUN_802ac7f0.c` |
| `0x802AC830` | `fn_802ac830` | 124 | high | `work/ghidra/main-gb/export/802ac830_FUN_802ac830.c` |
| `0x802ACB40` | `fn_802acb40` | 56 | high | `work/ghidra/main-gb/export/802acb40_FUN_802acb40.c` |
| `0x802AEE70` | `fn_802aee70` | 76 | high | `work/ghidra/main-gb/export/802aee70_FUN_802aee70.c` |

### `src/runtime/dvd.rs`

| address | function | bytes | confidence | evidence |
|---|---|---:|---|---|
| `0x801D00A0` | `fn_801d00a0` | 232 | medium | `work/ghidra/main-gb/export/801d00a0_FUN_801d00a0.c` |
| `0x801D0190` | `fn_801d0190` | 160 | high | `work/ghidra/main-gb/export/801d0190_FUN_801d0190.c` |
| `0x801D0230` | `fn_801d0230` | 256 | high | `work/ghidra/main-gb/export/801d0230_FUN_801d0230.c` |
| `0x801D0330` | `fn_801d0330` | 284 | high | `work/ghidra/main-gb/export/801d0330_FUN_801d0330.c` |
| `0x801D0450` | `fn_801d0450` | 160 | high | `work/ghidra/main-gb/export/801d0450_FUN_801d0450.c` |
| `0x801D04F0` | `fn_801d04f0` | 148 | high | `work/ghidra/main-gb/export/801d04f0_FUN_801d04f0.c` |
| `0x801D0590` | `fn_801d0590` | 144 | medium | `work/ghidra/main-gb/export/801d0590_FUN_801d0590.c` |
| `0x801D0620` | `fn_801d0620` | 120 | medium | `work/ghidra/main-gb/export/801d0620_FUN_801d0620.c` |
| `0x801D06A0` | `dvd_error_log_set_callback_801d06a0` | 104 | medium | `work/ghidra/main-gb/export/801d06a0_FUN_801d06a0.c` |
| `0x801D0710` | `dvd_compare_disc_header_801d0710` | 240 | high | `work/ghidra/main-gb/export/801d0710_FUN_801d0710.c` |
| `0x801D0800` | `dvd_report_error_801d0800` | 204 | medium | `work/ghidra/main-gb/export/801d0800_FUN_801d0800.c` |
| `0x801D08D0` | `dvd_set_error_reporter_801d08d0` | 92 | high | `work/ghidra/main-gb/export/801d08d0_FUN_801d08d0.c` |
| `0x801D0980` | `dvd_check_drive_authorization_801d0980` | 636 | medium | `work/ghidra/main-gb/export/801d0980_FUN_801d0980.c` |
| `0x801D1B10` | `dvd_low_get_no_disc_buffer_sizes_801d1b10` | 528 | high | `work/ghidra/main-gb/export/801d1b10_FUN_801d1b10.c` |
| `0x801D1D20` | `dvd_low_get_no_disc_open_partition_params_801d1d20` | 664 | medium | `work/ghidra/main-gb/export/801d1d20_FUN_801d1d20.c` |
| `0x801D1FC0` | `dvd_low_close_partition_801d1fc0` | 360 | high | `work/ghidra/main-gb/export/801d1fc0_FUN_801d1fc0.c` |
| `0x801D2C20` | `dvd_low_read_disk_bca_801d2c20` | 364 | high | `work/ghidra/main-gb/export/801d2c20_FUN_801d2c20.c` |
| `0x801D3250` | `dvd_cover_status_801d3250` | 16 | high | `work/ghidra/main-gb/export/801d3250_FUN_801d3250.c` |
| `0x801D3280` | `dvd_low_prepare_cover_regs_801d3280` | 364 | high | `work/ghidra/main-gb/export/801d3280_FUN_801d3280.c` |
| `0x801D33F0` | `dvd_low_prepare_status_regs_801d33f0` | 364 | high | `work/ghidra/main-gb/export/801d33f0_FUN_801d33f0.c` |
| `0x801D3560` | `dvd_low_prepare_control_regs_801d3560` | 364 | high | `work/ghidra/main-gb/export/801d3560_FUN_801d3560.c` |
| `0x801D36E0` | `fn_801d36e0` | 8 | high | `work/ghidra/main-gb/export/801d36e0_FUN_801d36e0.c` |
| `0x801D36F0` | `fn_801d36f0` | 8 | high | `work/ghidra/main-gb/export/801d36f0_FUN_801d36f0.c` |

### `src/engine/dsys.rs`

| address | function | bytes | confidence | evidence |
|---|---|---:|---|---|
| `0x800E5060` | `fn_800e5060` | 100 | high | `work/ghidra/main-gb/export/800e5060_FUN_800e5060.c` |
| `0x800E5110` | `fn_800e5110` | 140 | high | `work/ghidra/main-gb/export/800e5110_FUN_800e5110.c` |
| `0x800E51A0` | `fn_800e51a0` | 92 | high | `work/ghidra/main-gb/export/800e51a0_FUN_800e51a0.c` |
| `0x800E5200` | `fn_800e5200` | 208 | high | `work/ghidra/main-gb/export/800e5200_FUN_800e5200.c` |
| `0x800E52D0` | `fn_800e52d0` | 96 | high | `work/ghidra/main-gb/export/800e52d0_FUN_800e52d0.c` |
| `0x800E5400` | `fn_800e5400` | 128 | high | `work/ghidra/main-gb/export/800e5400_FUN_800e5400.c` |
| `0x800E5480` | `fn_800e5480` | 88 | high | `work/ghidra/main-gb/export/800e5480_FUN_800e5480.c` |

### `src/engine/frame.rs`

| address | function | bytes | confidence | evidence |
|---|---|---:|---|---|
| `0x80107850` | `fn_80107850` | 8 | high | `work/ghidra/main-gb/export/80107850_FUN_80107850.c` |
| `0x80107F10` | `fn_80107f10` | 8 | high | `work/ghidra/main-gb/export/80107f10_FUN_80107f10.c` |
| `0x80109340` | `fn_80109340` | 8 | high | `work/ghidra/main-gb/export/80109340_FUN_80109340.c` |

### `src/ui/menu.rs`

| address | function | bytes | confidence | evidence |
|---|---|---:|---|---|
| `0x801735B0` | `fn_801735b0` | 96 | high | `work/ghidra/main-gb/export/801735b0_FUN_801735b0.c` |

## Verification

- Every address above is present in `work/ghidra/main-gb/export/functions.tsv`.
- Every byte count above is read from that inventory, not from older coverage prose.
- Every cited evidence file exists under `work/ghidra/main-gb/export/` and contains the body the function documents.
- Every address has exactly one `pub unsafe fn` body in the listed Rust file.
- The recovery CSVs live in `ledger/agents/salvage-*.csv`; `merge-ledger.mjs` includes them, so the totals are reproducible.
- No placeholder (`todo!`, `unimplemented!`) exists in any of the five files.

## Related repairs made during the same audit

- `src/game/bosses.rs`: 227 evidence paths used `rel_d_en_bossNP`; the real directory is `rel-d_en_bossNP`. The five historical boss CSVs (`wave24`-`wave27`, `wave29`) carried the same typo in 122 ledger rows. All were corrected.
- `src/audio/ax.rs`: three stale byte counts (`0x801A0E60` 56->48, `0x801A0050` 24->84, `0x8019B660` 624->544) were aligned with `functions.tsv`.
- `src/game/player.rs` and `src/render/gx_band.rs`: ten stale byte counts were aligned with `functions.tsv`; the exports show the bodies are complete and the larger inventory size is alignment padding.
- `src/runtime/dvd.rs`: two constant-return hooks (`0x801D36E0`, `0x801D36F0`) were documented as 16 bytes; the inventory and the export both say 8.
