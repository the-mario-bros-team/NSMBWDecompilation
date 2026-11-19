# Coverage - Wave 39 (game::player — effect-record binders, history shift register, movement-bit stubs)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 30 | 2376 | 27 | 3 |

## How to read this file

This document covers the Wave-39 slice of the gameplay player actor from the
`0x80050000` bucket of `work/ghidra/main-gb/export/functions.tsv`. Every body
lives in `src/game/player.rs`; the ledger rows are in
`ledger/agents/wave39-player.csv`. The module's total is now two hundred ten
functions.

* **Address** — virtual entry point in the DOL image (not a file offset).
* **Name** — the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** — one plain sentence. Words such as "effect record" and "movement
  bit" are inferred from the shared field offsets, not from a recovered symbol.
* **Status** — all rows are `translated`: the full control flow and every call
  are present.
* **Confidence** — strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = some field or callee roles guessed.
* **Evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Bodies use `r2 = 0x80433080`; the `0x8042B874` (`1.0f`), `0x8042B878` (`0.0f`),
`0x8042BAE8` (flag-mask table), `0x802EEB60` / `0x802EEB70` / `0x802EEB80`
(scale tables) constants were re-read from `work/dol/main.elf`.

## What this pass covers

- **Effect-record cluster (`0x800534B0..0x800537B0`)** — the five model-effect
  records at `+0x148`/`+0x1164`/`+0x1208`/`+0x12AC`/`+0x1350` that
  `fn_80053650` writes: binders (`0x8008C3E0`), reset (`0x8008C2C0`), the
  mode-`5`/`0x11` writers, the `+0x136C` shadow copy, and the run-state
  mirror with its five stop gates. Start at `fn_800534b0`.
- **History/timer cluster (`0x800540F0..0x800543E0`)** — the three-state
  `+0x394` timer (`0x8015F480` with the `0x802EEB60`/`0x802EEB70` tables), the
  `+0x3A8`/`+0x3AC` countdown, the `+0x1010` flag-`0x23` store, the
  `+0x1124`/`+0x240` countdown, and the eleven-word `+0x10D4` history shift
  register with its clear-and-reseed wrapper and the `fn_800543e0` reset
  dispatcher (which calls `fn_80055530`, translated here). Start at
  `fn_80054300`.
- **Request/scale cluster (`0x80054EF0..0x800557E0`)** — the `+0x250`
  indexed-entry predicate, the `+0xB0`-plus-scale-table reader through actor
  slot `+0xE8`, the `+0x2CC`→`+0x1148/+0x1149` copy, the request-pair
  consumer through slots `+0x3F0`/`+0x450`, and the `0x8042BAE8` mask-table
  flag test (already called by `fn_80052650`).
- **Slot-array leaves (`0x8005D050`, `0x8005D090`)** — the four-slot `+0x70`
  publish scan and its `+0x60` value match.
- **Movement-bit stubs (`0x8005E4C0..0x8005E550`)** — eight 12-byte tests on
  the halfwords at `+0x04`/`+0x06` of the movement state object, called (some
  through one-instruction thunks such as `0x8005E830` → `0x8005E500`) from the
  actor's movement code.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x800534B0` | `fn_800534b0` | 144 | high |
| `0x80053600` | `fn_80053600` | 80 | high |
| `0x800536A0` | `fn_800536a0` | 8 | high |
| `0x80053740` | `fn_80053740` | 92 | high |
| `0x800537A0` | `fn_800537a0` | 8 | high |
| `0x800537B0` | `fn_800537b0` | 296 | high |
| `0x800540F0` | `fn_800540f0` | 196 | medium |
| `0x800541C0` | `fn_800541c0` | 32 | high |
| `0x800541E0` | `fn_800541e0` | 12 | high |
| `0x800541F0` | `fn_800541f0` | 44 | high |
| `0x80054230` | `fn_80054230` | 68 | high |
| `0x800542D0` | `fn_800542d0` | 36 | high |
| `0x80054300` | `fn_80054300` | 104 | high |
| `0x80054370` | `fn_80054370` | 104 | high |
| `0x800543E0` | `fn_800543e0` | 252 | high |
| `0x80054EF0` | `fn_80054ef0` | 140 | medium |
| `0x800552D0` | `fn_800552d0` | 84 | medium |
| `0x80055530` | `fn_80055530` | 232 | high |
| `0x80055620` | `fn_80055620` | 208 | high |
| `0x800557E0` | `fn_800557e0` | 36 | high |
| `0x8005D050` | `fn_8005d050` | 64 | high |
| `0x8005D090` | `fn_8005d090` | 40 | high |
| `0x8005E4C0` | `fn_8005e4c0` | 12 | high |
| `0x8005E4D0` | `fn_8005e4d0` | 12 | high |
| `0x8005E4F0` | `fn_8005e4f0` | 12 | high |
| `0x8005E500` | `fn_8005e500` | 12 | high |
| `0x8005E510` | `fn_8005e510` | 12 | high |
| `0x8005E520` | `fn_8005e520` | 12 | high |
| `0x8005E540` | `fn_8005e540` | 12 | high |
| `0x8005E550` | `fn_8005e550` | 12 | high |

Byte total: 2376. Address range covered: `0x800534B0..0x8005D090` (the
deliberate gaps are the paired-single functions listed below).

## Deliberate skips and why

* `0x80053940`, `0x80053AA0`, `0x80053AD0`, `0x80053C80`, `0x80053DF0`,
  `0x80053E60` — paired-single code rendered as `ldexpf`/`unaff_GQR0` with
  `unaff_r2` addressing; the float lanes cannot be reconstructed honestly from
  the C export alone (per the lane brief, these are skipped, not guessed).
* `0x80054F80`, `0x80055330`, `0x800556F0`, `0x80055810`, `0x80055900`,
  `0x80055A00`, `0x80055A70`, `0x80055DB0`, `0x80055EF0` — call `r13`-based
  globals (`unaff_r13 + 0x...`) whose identities are not resolved in this
  pass, or hide float arguments (`FUN_80055900()` with no visible
  registers), so the call seams cannot be justified. `0x800544F0`
  (2540 bytes) and `0x800592C0` (7620 bytes) are far beyond a single honest
  body for this pass.
* `0x8005E460`, `0x8005E480`, `0x8005E4A0`, `0x8005E4E0`, `0x8005E560`,
  `0x8005E570`, `0x8005E580` and the `0x8005E5xx`/`0x8005E6xx`/`0x8005E7xx`
  variants — same trivial shapes, but the lane target was met at 30 bodies and
  the remaining stubs (including the `unaff_r13`-dependent `0x8005E590` /
  `0x8005E5D0`) are left as clean future work rather than padding.
* `0x8005Bxxx` through `0x8005Dxxx` (except the two slot leaves) and the
  `0x8005E4xx` wrapper thunks (`0x8005E620`, `0x8005E680`, ...) — different
  object clusters than this pass's player-actor state block; not worked here.
* Everything in `0x80060000` — explicitly out of scope for this pass.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/game/player.rs`.
