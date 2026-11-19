# Coverage — Wave6 CRT `0x802D0000..0x802E0000`

## How to read this file

This tracks translation of the Metrowerks runtime / CRT band in `runtime::cpp`.

## Summary

| Metric | Count |
|---|---|
| Functions translated this pass | 29 |
| Status: translated | 8 |
| Status: partial | 21 |
| Module | runtime::cpp |
| Evidence | work/ghidra/main-gb/export, work/dol/main.elf |

## Translated functions

| Address | Rust name | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| 0x802d0800 | fn_802d0800 | r2-relative pointer/value accessor (observed) | partial | medium | 802d0800_FUN_802d0800.c |
| 0x802d0830 | fn_802d0830 | Initialize object from r2 globals | partial | medium | 802d0830_FUN_802d0830.c |
| 0x802d0cc0 | fn_802d0cc0 | Initialize object with lookups/calls | partial | medium | 802d0cc0_FUN_802d0cc0.c |
| 0x802d1250 | fn_802d1250 | Set field at +0x3c if different/defaulted | translated | high | 802d1250_FUN_802d1250.c |
| 0x802d19f0 | fn_802d19f0 | Recursive walk over structure | partial | medium | 802d19f0_FUN_802d19f0.c |
| 0x802d1aa0 | fn_802d1aa0 | Helper (unreconstructed) | partial | low | 802d1aa0_FUN_802d1aa0.c |
| 0x802d1b50 | fn_802d1b50 | Call helpers to update state | partial | medium | 802d1b50_FUN_802d1b50.c |
| 0x802d1b90 | fn_802d1b90 | Unreconstructed | partial | low | 802d1b90_FUN_802d1b90.c |
| 0x802d1da0 | fn_802d1da0 | Initialize globals | partial | medium | 802d1da0_FUN_802d1da0.c |
| 0x802d21f0 | fn_802d21f0 | Check/remove entry from table | partial | medium | 802d21f0_FUN_802d21f0.c |
| 0x802d2270 | fn_802d2270 | Unreconstructed | partial | low | 802d2270_FUN_802d2270.c |
| 0x802d2c20 | fn_802d2c20 | Initialize GX/TEV state | partial | medium | 802d2c20_FUN_802d2c20.c |
| 0x802d2e50 | fn_802d2e50 | Initialize state via helpers | translated | high | 802d2e50_FUN_802d2e50.c |
| 0x802d2f20 | fn_802d2f20 | Set byte DAT_804248cc with helper | translated | high | 802d2f20_FUN_802d2f20.c |
| 0x802d2f80 | fn_802d2f80 | Set byte DAT_804248cd with helper | translated | high | 802d2f80_FUN_802d2f80.c |
| 0x802d2fe0 | fn_802d2fe0 | Set byte DAT_804248ce with helper | translated | high | 802d2fe0_FUN_802d2fe0.c |
| 0x802d3040 | fn_802d3040 | Set bytes DAT_804248cf/d0 with helper | translated | high | 802d3040_FUN_802d3040.c |
| 0x802d3090 | fn_802d3090 | Compute values via FUN_802dcc4c | partial | medium | 802d3090_FUN_802d3090.c |
| 0x802d31a0 | fn_802d31a0 | Small fragment (unreconstructed) | partial | low | 802d31a0_FUN_802d31a0.c |
| 0x802d31d0 | fn_802d31d0 | Small fragment (unreconstructed) | partial | low | 802d31d0_FUN_802d31d0.c |
| 0x802d3200 | fn_802d3200 | Small fragment (unreconstructed) | partial | low | 802d3200_FUN_802d3200.c |
| 0x802d3230 | fn_802d3230 | Small fragment (unreconstructed) | partial | low | 802d3230_FUN_802d3230.c |
| 0x802d3270 | fn_802d3270 | Small fragment (unreconstructed) | partial | low | 802d3270_FUN_802d3270.c |
| 0x802d32a0 | fn_802d32a0 | Initialize object with vtable | partial | medium | 802d32a0_FUN_802d32a0.c |
| 0x802d3370 | fn_802d3370 | Heap/node merge (unreconstructed) | partial | low | 802d3370_FUN_802d3370.c |
| 0x802d3490 | fn_802d3490 | Heap init (unreconstructed) | partial | low | 802d3490_FUN_802d3490.c |
| 0x802d3590 | fn_802d3590 | Remove node from list | partial | medium | 802d3590_FUN_802d3590.c |
| 0x802d2e90 | fn_802d2e90 | Unreconstructed | partial | low | 802d2e90_FUN_802d2e90.c |
| 0x802d2da0 | fn_802d2da0 | Unreconstructed | partial | low | 802d2da0_FUN_802d2da0.c |

## Notes
- Skipped `0x802DCD40..0x802DCDBC` save/restore helpers (4-byte alternates/thunks as noted).
- Skipped 4-byte thunks and pure register-save fragments.
- Many functions use `r2`/`r13` SDA globals and unknown calls; marked `partial` where body ambiguous.
- All translations go to `src/runtime/cpp.rs` only.
