# Coverage - Wave 32 (UI: HUD warning scenes and menu quad/dialog helpers)

Pass 10 of the UI lane, salvaged after the model quota ended the original
subagent mid-run. The bodies, evidence blocks and module-header sections were
already on disk and verified before being entered in the ledger; no row was
created without a compiled body.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 23 | 3,512 | 15 | 8 |
| `src/ui/menu.rs` | `ui::menu` | 2 | 868 | 1 | 1 |
| **total** | | **25** | **4,380** | **16** | **9** |

## What this pass covers

- `dWarningError_c` / `dWarningNunchuk_c` / `dWarningYoKo_c` scene
  halves: open/close notification helpers, the error-info and Yokomochi state
  descriptor builders, the nunchuk record teardown and the per-scene release
  paths.
- Menu grid and four-player widget accessors, the quad draw state used by the
  pause menu, and the dialog-tree tagging helpers around `0x80171680` and
  `0x8017F7A0`.

## Repair performed during salvage

The lane stopped after writing the bodies but before its ledger and coverage
files, leaving one defect that blocked the crate build:

1. `EVENT_YOKO_OPEN` was defined twice with different values. The original
   `FUN_8001ab50` passes `&DAT_80352538` (the open **state descriptor**), while
   `FUN_8001af70` registers `&DAT_80352528` as the open state's event object.
   The duplicate definition was removed and `fn_8001ab50` now passes
   `STATE_YOKO_OPEN`; `fn_8001af70` keeps `EVENT_YOKO_OPEN`.
2. Nothing else was missing: the section compiled as soon as the constant was
   disambiguated.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8001A220` | `fn_8001a220` | 48 | medium |
| `0x8001A250` | `fn_8001a250` | 48 | medium |
| `0x8001A280` | `fn_8001a280` | 824 | high |
| `0x8001A5C0` | `fn_8001a5c0` | 88 | high |
| `0x8001A6B0` | `fn_8001a6b0` | 176 | medium |
| `0x8001A760` | `fn_8001a760` | 64 | high |
| `0x8001A7A0` | `fn_8001a7a0` | 64 | high |
| `0x8001A7E0` | `fn_8001a7e0` | 96 | high |
| `0x8001A840` | `fn_8001a840` | 100 | high |
| `0x8001A8B0` | `fn_8001a8b0` | 120 | high |
| `0x8001A930` | `fn_8001a930` | 348 | medium |
| `0x8001AA90` | `fn_8001aa90` | 96 | medium |
| `0x8001AB00` | `fn_8001ab00` | 20 | high |
| `0x8001AB50` | `fn_8001ab50` | 104 | high |
| `0x8001AC10` | `fn_8001ac10` | 88 | high |
| `0x8001AD00` | `fn_8001ad00` | 88 | high |
| `0x8001AD70` | `fn_8001ad70` | 20 | high |
| `0x8001AD90` | `fn_8001ad90` | 16 | high |
| `0x8001ADA0` | `fn_8001ada0` | 96 | medium |
| `0x8001AF10` | `fn_8001af10` | 48 | medium |
| `0x8001AF40` | `fn_8001af40` | 48 | medium |
| `0x8001AF70` | `fn_8001af70` | 824 | high |
| `0x8001B2B0` | `fn_8001b2b0` | 88 | high |
| `0x80171680` | `fn_80171680` | 452 | high |
| `0x8017F7A0` | `fn_8017f7a0` | 416 | medium |

## Verification

- Every address and byte size above matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in its `rust_file`.
- None of the addresses was present in `ledger/functions.csv` before this pass.
- `cargo check` exits 0 with no diagnostics from either UI file.
- `node tools/doc-lint.mjs src` reports 0 violations.
