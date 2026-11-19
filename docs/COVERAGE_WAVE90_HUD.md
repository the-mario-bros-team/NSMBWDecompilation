# Coverage - Wave 90 (UI: HUD, four-player slot panels)

Pass 11 of the `ui::hud` lane. The lane's assigned 64 KiB bucket
(`0x80010000`, 89 untranslated of 253) contains no further HUD warning-scene
code: the remainder splits into the late half of a four-player panel state
machine, the `wipe*` transition family and small gaps in the `game::actor`
band. This pass takes the contiguous panel run and the one HUD teardown that
was still missing.

## How to read this file

Every row in **Functions** maps one Ghidra function to one body in
`src/ui/hud.rs`. `address` is the link-time address in `sys/main.dol`,
`bytes` is the size recorded in `work/ghidra/main-gb/export/functions.tsv`, and
`confidence` matches the `# Confidence` line of the body's doc block.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/ui/hud.rs` | `ui::hud` | 26 | 2,844 | 24 | 2 |

## What this pass covers

- **Four-player panel slot bodies, `0x80011660..0x80012070`.** Five bodies per
  player slot: the next-player notification, the hidden/ready transition, the
  hidden notification, the slide-in marker plus animation start, and the
  slide-in/idle notification. The four widget objects sit at `+0x218`,
  `+0x254`, `+0x290` and `+0x2CC`; slot markers are `+0x44A..+0x44D`.
- **Shared panel widgets and close state, `0x80012170..0x80012500`.** Three
  idle notifications for `+0x308`/`+0x344` and the close-state body that hides
  the panes at `+0x3A4..+0x3AC` and clears `+0x447`.
- **Yokomochi layout teardown, `0x8001AB20`.** The missing twin of
  `fn_800151f0`/`fn_800162e0`: one `0x800C9930` call on `+4`, between the
  already translated `0x8001AB00` and `0x8001AB50`.

The first two bullets are the late half of the state machine whose early half
(`0x800108C0..0x800115C0`) lives in `game::path`; wave 11 had assigned the
whole `0x8000C000..0x80014000` band to that module. Wave 90's bucket heuristic
attributed the remainder of `0x8001` to `ui::hud`, so the bodies are recorded
here with that ownership made explicit in the module header. No address in this
pass was in `ledger/functions.csv`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80011660` | `fn_80011660` | 184 | high |
| `0x80011740` | `fn_80011740` | 96 | high |
| `0x800117B0` | `fn_800117b0` | 76 | high |
| `0x80011800` | `fn_80011800` | 136 | high |
| `0x800118B0` | `fn_800118b0` | 108 | high |
| `0x80011950` | `fn_80011950` | 160 | high |
| `0x80011A10` | `fn_80011a10` | 96 | high |
| `0x80011A80` | `fn_80011a80` | 76 | high |
| `0x80011AD0` | `fn_80011ad0` | 136 | high |
| `0x80011B80` | `fn_80011b80` | 108 | high |
| `0x80011C20` | `fn_80011c20` | 160 | high |
| `0x80011CE0` | `fn_80011ce0` | 96 | high |
| `0x80011D50` | `fn_80011d50` | 76 | high |
| `0x80011DA0` | `fn_80011da0` | 136 | high |
| `0x80011E50` | `fn_80011e50` | 108 | high |
| `0x80011EF0` | `fn_80011ef0` | 160 | high |
| `0x80011FB0` | `fn_80011fb0` | 96 | high |
| `0x80012020` | `fn_80012020` | 76 | high |
| `0x80012070` | `fn_80012070` | 136 | high |
| `0x80012170` | `fn_80012170` | 88 | high |
| `0x80012240` | `fn_80012240` | 88 | high |
| `0x80012330` | `fn_80012330` | 132 | medium |
| `0x800123C0` | `fn_800123c0` | 144 | medium |
| `0x800124B0` | `fn_800124b0` | 76 | high |
| `0x80012500` | `fn_80012500` | 88 | high |
| `0x8001AB20` | `fn_8001ab20` | 8 | high |

## Host-side helpers

Two non-ledgered helpers keep the 26 bodies readable and preserve the original
call order: `notify_widget` (vtable slot `+0x18` with a descriptor) and
`player_slot_present` (the already-translated `0x80010F40` mask test). Neither
models a function that exists as a separate Ghidra entry, and neither is in the
CSV.

## Deliberate skips

- **`wipeCircle`/`wipeDoor`/`wipeKuppa`/`wipeFader` family,
  `0x8001B3A0..0x8001FF90`.** The transition classes are documented and owned by
  `src/engine/display.rs` (see its `W78_FN_8001B470`, `W78_FN_8001BC60`,
  `W78_FN_8001C240`, `W78_FN_8001C760` references). Translating them here would
  duplicate that lane.
- **`0x80014330..0x800148F0`.** These are small record accessors sandwiched
  between translated `game::actor` bodies (`0x80014200..0x80014970`); they
  belong to that module, not to the HUD.
- **`0x800129A0` (6,204 bytes), `0x80012940`, `0x80012970`.** The large body is
  the constructor/destructor core of the panel scene; it needs its own pass and
  cannot be reconstructed honestly in the remaining time.
- **`0x800105B0`, `0x80010690`, `0x80010800`, `0x80010950`, `0x80010B50`,
  `0x80010DD0`, `0x800126A0`.** The remaining `game::path`-band entries outside
  the chosen run; `0x800126A0` is already led to `game::path`.

## Verification

- Every address and byte size matches `work/ghidra/main-gb/export/functions.tsv`.
- Every evidence file exists at `work/ghidra/main-gb/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in `src/ui/hud.rs`.
- No address was already in `ledger/functions.csv` before this pass.
- `cargo check --message-format=short` reports no diagnostic for
  `src/ui/hud.rs`. A full-lane run made after this file landed failed only on
  other wave-90 files that were still being written (`game/worldmap.rs`,
  `audio/ax.rs`, `runtime/dvd.rs`); re-running once those lanes settle is
  expected to pass. The same check passed end-to-end while only this file was
  changed.
