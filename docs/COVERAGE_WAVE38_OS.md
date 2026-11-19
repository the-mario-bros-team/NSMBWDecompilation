# Coverage - Wave 38 (runtime::os)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/os.rs` | `runtime::os` | 25 | 5736 | 21 | 4 |

## What this pass covers

- **Text decoder mode (`0x801B00A0-0x801B1090`, 8 functions).** The `r13-0x6158` mode word, the `r13-0x4fc8` decoder pointer, the mode 0/1 decoders, glyph expansion, font loading and glyph location. All readers share the `0xffff` initialise sequence, which is factored into `wave38_text_mode_ensure`.
- **Exception mask and dispatch (`0x801B11B0-0x801B1610`, 7 functions).** The `r13-0x4fa8` handler table, the `0x800000c4`/`0x800000c8` mask pair, the `0x801b12b0` per-bit decoder over the sub-registers, the set/clear drains (`0x801b1510`/`0x801b1590`), and the `0x801b1610` exception dispatcher that rebuilds the mask and calls the handler under the scheduler lock.
- **IOS memory-map and reset init (`0x801B25A0-0x801B2D50`, 10 functions).** The four small getters for `0x80003100`/`0x80003118`/`0x80003104`/`0x8000311c`, the conditional `0xf0000000` mask set, the IOS flush/routing helper, the `0x801b2740..0x801b2b10` stub selector, and the one-shot init that installs five handlers at `0x801b2620`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801B00A0` | `fn_801b00a0` | 76 | high |
| `0x801B00F0` | `fn_801b00f0` | 120 | high |
| `0x801B0480` | `fn_801b0480` | 280 | high |
| `0x801B05A0` | `fn_801b05a0` | 248 | high |
| `0x801B0AF0` | `fn_801b0af0` | 620 | high |
| `0x801B0D60` | `fn_801b0d60` | 480 | high |
| `0x801B0F40` | `fn_801b0f40` | 328 | high |
| `0x801B1090` | `fn_801b1090` | 176 | high |
| `0x801B11B0` | `fn_801b11b0` | 20 | high |
| `0x801B11D0` | `fn_801b11d0` | 16 | high |
| `0x801B11E0` | `fn_801b11e0` | 196 | high |
| `0x801B12B0` | `fn_801b12b0` | 604 | high |
| `0x801B1510` | `fn_801b1510` | 128 | high |
| `0x801B1590` | `fn_801b1590` | 128 | high |
| `0x801B1610` | `fn_801b1610` | 680 | medium |
| `0x801B25A0` | `fn_801b25a0` | 12 | high |
| `0x801B25B0` | `fn_801b25b0` | 12 | high |
| `0x801B25C0` | `fn_801b25c0` | 12 | high |
| `0x801B25D0` | `fn_801b25d0` | 12 | high |
| `0x801B25E0` | `fn_801b25e0` | 60 | high |
| `0x801B2670` | `fn_801b2670` | 200 | medium |
| `0x801B2BC0` | `fn_801b2bc0` | 24 | medium |
| `0x801B2BE0` | `fn_801b2be0` | 288 | medium |
| `0x801B2D00` | `fn_801b2d00` | 68 | high |
| `0x801B2D50` | `fn_801b2d50` | 204 | high |

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/os.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/runtime/os.rs` (remaining errors are in unrelated files not owned by this lane).

## Deliberate skips

- `0x801B0170`, `0x801B06A0`, `0x801B0860`: the fixed-point font-placement bodies contain Ghidra's signed-division noise and the `0x802dcd6c`/`0x802dcd70` backing-store helpers; skipped rather than risk inventing arithmetic. `0x801B06A0` is also the mode 3/4/5 decoder, whose UTF decoders (`0x801b63b0`/`0x801b64d0`) are outside this lane, so its body would be address-dispatched at every step.
- `0x801B3140`, `0x801B3230`: 4-byte thunks to already-translated targets (`0x801b4b20`, `0x801b5d80`); no distinct body.
- Later `0x801B6000+` bands are not in this contiguous cluster (date/time, Unicode tables, StateTM, NAND/boot-info and IPC); left for their semantic lanes.
