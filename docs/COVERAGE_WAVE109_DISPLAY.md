# Coverage - Wave 109 (engine::display)

## How to read this file

`src/engine/display.rs` is the semantic home of the engine-level 2-D
presentation layer: the `Pausewindow_c` state machine, the `dGameDisplay_c`
descriptors, the `Wm_2d_*` effects and the engine band
`0x800D0000..0x80200000`. This pass works the last open bucket, `0x80150000`:
it entered with ten untranslated addresses of its 209 and leaves one.

"Bodies" counts `pub unsafe fn` functions; "bytes" is the sum of the Ghidra
sizes re-read from `work/ghidra/main-gb/export/functions.tsv` for this pass.

"high" means every store, branch and call is visible in the export. "medium"
means one field role, callee shape or vtable argument type had to be recovered
from the export plus the surrounding call sites, and that recovery is spelled
out in the body's `# Confidence` block.

Only `src/engine/display.rs` was edited. `ledger/functions.csv` is read-only and
the coordinator merges this lane's rows.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/engine/display.rs` | `engine::display` | 9 | 3068 | 3 | 6 |

## What this pass covers

Three clusters, all in `0x801502E0..0x8015F130`:

- **The pending-item display pass** (`0x801502E0`, 256 bytes).
  [`fn_801502e0`] runs every frame on the scene's `+0x1558` item: it reads the
  item's centre height with `0x8008C570`, branches on the `+0x1560` phase,
  attaches the item's `+0x04` target or refreshes the world-map interface, picks
  the bottom or top edge from `0x8008C550`/`0x8008C530`, builds a three-float
  position with `0x8008C510` and the `+0xB4` value, hands it to `0x80152880`,
  publishes the height through `0x800FFDB0` and clears the item with
  [`fn_801501b0`].
- **The interaction request pair** (`0x80151E50`, `0x80152CF0`, 452 + 232
  bytes). [`fn_80151e50`] consumes one request against the player under the
  cursor: it compares the player's `+0x1524` course word with the target's
  identity word, re-posts special moves `0x13..0x15`, and for requests `7`/`0xD`
  asks the player's `+0x3F4` slot, updates the shared `r13 - 0x5678` effect
  object, attaches the target through `0x8014EA30` and fires its `+0x3FC`
  callback. [`fn_80152cf0`] resolves the scene's `+0x1520` registry id, requires
  a forced commit plus the target's `+0x38C == 1`, uses the target's `+0x130`
  slot and then switches the layout to `0x80376E88` and starts animation
  `0x8035421C`.
- **The route-HUD layout passes and the band scans**
  (`0x801548A0..0x80154A70`, 464 + 648 + 268 bytes, and `0x80156930..0x80156AB0`,
  372 + 372 bytes). [`fn_801548a0`] resolves the `0x3B0`-byte stage entry from
  the `r13 - 0x57E8` table, writes the four section-4B selector bytes, the zoom
  and four screen scales. [`fn_80154a70`] rebuilds the per-slot view rectangle
  from `0x8008E410`, derives the two aspect ratios, clamps them with
  `r2 - 0x5578`, scales `+0x90028` by the `r2 - 0x5570` double and sets the
  eight-case orientation pair. [`fn_80154e50`] returns the four-slot y-band
  mask, and the two 372-byte siblings [`fn_80156930`]/[`fn_80156ab0`] count the
  players inside the x or y band and report the tracked extent.
- **The CRC-32 thunk** (`0x8015F130`, 4 bytes). [`fn_8015f130`] is a four-byte
  branch to `0x801B8620`, whose body Ghidra folds into the thunk's export.

The two band scans share the `r2 - 0x557C` / `r2 - 0x54D8` sentinel pair and the
`+0x40` flag word. That word is a useful cross-check inside this pass: the x-band
export reads it as a float and tests bits `0x10000000`/`0x20000000`/`0x40000000`,
while the y-band export reads the same address as the high halfword and tests
`0x1000`/`0x2000`/`0x4000` — the same three bits, so both bodies use the word
form with a comment.

## Functions

| address | function | bytes | confidence |
|---|---|---|---:|
| `0x801502E0` | `fn_801502e0` | 256 | high |
| `0x80151E50` | `fn_80151e50` | 452 | medium |
| `0x80152CF0` | `fn_80152cf0` | 232 | medium |
| `0x801548A0` | `fn_801548a0` | 464 | medium |
| `0x80154A70` | `fn_80154a70` | 648 | medium |
| `0x80154E50` | `fn_80154e50` | 268 | high |
| `0x80156930` | `fn_80156930` | 372 | medium |
| `0x80156AB0` | `fn_80156ab0` | 372 | medium |
| `0x8015F130` | `fn_8015f130` | 4 | high |

Byte total: `256 + 452 + 232 + 464 + 648 + 268 + 372 + 372 + 4 = 3068`.

## Deliberate skips

One address of the bucket stays untranslated.

**Sixteen-register varargs block builder (1 address).** `0x8015F730` (128 bytes)
copies eight doubles (`f1..f8`, `param_1..param_8`) into a 0x40-byte stack block
and eight words (`r3..r10`, `param_9..param_16`) into the 0x20-byte block before
it, then writes a three-word trailer `{0x01000000, &stack0x00000008, &block}`
and calls the `0x8015F6E0` task trampoline with `(param_9, &block[0x60])`. Three
things cannot be recovered from the export: the meaning of the `0x01000000`
tag, the fact that the double block is only reachable through the negative
offsets of that trailer (so its field order inside the descriptor is a guess),
and `&stack0x00000008`, which is the caller's stack-argument pointer and has no
parameter to bind to. Any Rust body would invent the descriptor layout and the
stack source, so the address is left for a pass that can read the raw
instruction words.

## Verification

- Every address and byte size matches `functions.tsv`, re-read for this pass.
- Every evidence file exists at the cited path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/engine/display.rs`.
- No address was already in `ledger/functions.csv`; the bucket had 10
  untranslated functions before this pass and 1 after it.
- `node tools/check-lane-csv.mjs wave109` reports this lane clean.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/engine/display.rs`; the remaining crate errors name other lanes' files
  that were being edited in parallel.
