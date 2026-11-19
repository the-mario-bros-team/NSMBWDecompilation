# Coverage - Wave 102 (UI: home-button menu, dialogs and quad writer)

This pass fills the last translated gaps in the `0x80170000` bucket of the UI
lane. Nineteen bodies were added to `src/ui/menu.rs`; seven entries were left
out because the Ghidra export does not carry enough information to reconstruct
them honestly (see "Deliberate skips").

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/menu.rs` | `ui::menu` | 19 | 6,876 | 8 | 11 |

## What this pass covers

- **Grid and small accessors** (`0x801704D0..0x80172660`): the record
  allocator with the `-4`/`4` size argument, the 8x8 grid setter, the vector
  normaliser that reads two `r2` (SDA2) constants, the dialog pane-list holder
  constructor, the event-word clear plus virtual dispatch, and the `+0x2C`
  store.
- **Hit test** (`0x80172910`): projects a local point through the pane
  transform (`0x801C07A0`/`0x801C1160`) and compares it against the six-float
  bounds from `0x80183960`; the four comparisons are literal.
- **Four-byte tails** (`0x801734F0`, `0x80173870`, `0x80174AE0`, `0x80174DE0`):
  two empty bodies, the `blr` identity accessor used by `0x801795B0`, and the
  branch thunk to `0x80191250`.
- **The two constructors the wave-21/26 wrappers already called**
  (`0x80175510`, `0x80175F70`): the pane-tree builder (dialog node, four player
  groups, root group, `back_01` node, three named pane groups, two control
  objects, four scroll-text players, draw-info block and three `T_msg_00`
  strings) and the first-run pane setup (button panes, `N_Trans` players, eight
  control-object setup calls, four widget records, pane visibility bits and the
  scroll timer).
- **List and animation helpers** (`0x80178B10`, `0x8017DA20`, `0x8017DCC0`):
  the manager query plus `fn_801738f0`, the per-record name binding loop, and
  the `RLMC`/`RLTS`/`RLTP`/`RLIM` animation tag evaluator.
- **Quad writer** (`0x8017EA10`, `0x8017EF70`): the four-vertex GX FIFO writer
  with the literal colour order `0, 1, 3, 2` and four row offsets, plus the
  alpha-scaling wrapper that feeds it.

Two call sites in already-translated bodies were corrected to pass the live
menu returned by `0x80175500`: `fn_80174e10` now calls
`fn_80175510(fn_80175500())` and `fn_80174f30` calls
`fn_80175f70(fn_80175500())`. The register trace at `0x80174F10..0x80174F14`
shows the same `r3` flowing from `0x80175500` into both constructors.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801704D0` | `fn_801704d0` | 228 | medium |
| `0x801707E0` | `fn_801707e0` | 68 | medium |
| `0x80170980` | `fn_80170980` | 120 | medium |
| `0x80170C50` | `fn_80170c50` | 128 | medium |
| `0x80172370` | `fn_80172370` | 36 | medium |
| `0x80172660` | `fn_80172660` | 8 | high |
| `0x80172910` | `fn_80172910` | 300 | medium |
| `0x801734F0` | `fn_801734f0` | 4 | high |
| `0x80173500` | `fn_80173500` | 8 | high |
| `0x80173870` | `fn_80173870` | 4 | medium |
| `0x80174AE0` | `fn_80174ae0` | 4 | high |
| `0x80174DE0` | `fn_80174de0` | 4 | high |
| `0x80175510` | `fn_80175510` | 2,144 | medium |
| `0x80175F70` | `fn_80175f70` | 1,184 | medium |
| `0x80178B10` | `fn_80178b10` | 36 | high |
| `0x8017DA20` | `fn_8017da20` | 188 | medium |
| `0x8017DCC0` | `fn_8017dcc0` | 728 | medium |
| `0x8017EA10` | `fn_8017ea10` | 1,376 | high |
| `0x8017EF70` | `fn_8017ef70` | 308 | high |

## Deliberate skips

The bucket held 26 untranslated entries. Seven were not written:

| address | bytes | why |
|---|---:|---|
| `0x801701A0` | 612 | The body builds and reuses a 0xC0-byte matrix stack and the export loses the float argument lists of `0x8022D3C0` and `0x80231870`; two of the interleaved `f1`/`f2` ordinates are only reachable from the unrolled `lfd`/`fadds` sequence, so a Rust body would be guesswork. |
| `0x801769B0` | 7,420 | Five-hundred-line list-animation state machine; not attempted in this pass. |
| `0x80178C50` | 2,388 | Per-player scrolling-text update; 900-line state machine over the four `+0x2AC` widgets. Not attempted. |
| `0x801795B0` | 2,212 | Companion input state machine for the same widgets. Not attempted. |
| `0x8017A120` | 3,660 | Home-button page transition state machine. Not attempted. |
| `0x8017AF70` | 2,764 | Dialog page state machine. Not attempted. |
| `0x8017BA40` | 4,692 | Largest remaining overlay state machine. Not attempted. |

They are all reachable from the bodies added here (`0x80175510` and
`0x80175F70` call `0x801769B0`), so a later pass can pick them up without
re-deriving the object layout.

## Verification

- Every address and byte size above matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`
  (`0x80174DE0` uses `80174de0_thunk_FUN_80191250.c`).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/ui/menu.rs`.
- No address was already in `ledger/functions.csv`; all 26 were absent.
- `cargo check --message-format=short` reports no diagnostics for
  `src/ui/menu.rs`.
- `node tools/check-lane-csv.mjs wave102` reports `OK`.
