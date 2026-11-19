# Coverage - Wave 212, lane E (`game::player`, the `0x80068320..0x80068D6C` grid cluster)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 2592 | 7 | 3 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred. All ten bodies in this
lane were fully recoverable.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives for
  these functions, so the Rust body keeps the `fn_<address>` name.
* **Role** - one plain sentence per function, in "What this pass covers". A
  role is marked "inferred" when it comes from shared field offsets and call
  shapes rather than from a recovered symbol.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = a field role or a callee's
  meaning had to be inferred.
* All ten bodies use the SDA bases the module already fixes,
  `r2 = 0x80433080` (observed at `0x8000423C`, the `lis`/`ori` in
  `__init_registers`) and `r13 = 0x8042F680` (observed at `0x80004248`).
  The existing constants `W22_SCENE_80429E00`, `PLAYER_ARRAY_8042A1C8`,
  `W18_STAGE_TABLE_80429E98`, `W18_FLAGS_80429F80` and
  `W98_WORLD_80429DD0` are re-used unchanged.

## What this pass covers

- **The wide sweep `0x80068320` (768 B).** It converts the scene object's four
  floats (`+0x38..+0x44`) into a grid rectangle (cell size 16) around the
  player and walks `count` 16-byte entries, skipping any whose flag byte has
  bit 0 set. Per entry it resolves the 40-byte grid-table row at
  `0x8030A040 + entry[0] * 0x28` and decides visibility either from the
  `+0x24` flag word alone (`mode != 0`) or from `0x80068290` / `0x8008E800`
  name matching plus a two-axis `abs` rectangle test (`mode == 0`). Survivors
  go to `0x80068DE0`. `// inferred:` the entry is a per-frame candidate list
  and `out` is the frame's `u16` value array.
- **The pin updater `0x80068620` (552 B).** It early-outs on
  `0x80429F80 & 1` and on `player[+0xBCB] == 1`, builds the same rectangle,
  then either re-pins the cached cells `+0xBD4`/`+0xBD8` from the facing bytes
  `+0x80`/`+0x81` (when the world floats `+0x9000C`/`+0x90014` are equal) or
  runs three `fn_80068850` sweeps over the rectangle's corners. It always
  stores `+0xBCC`/`+0xBD0` from the scene floats at the end. `// inferred:`
  `+0x80`/`+0x81` are the two facing selectors.
- **The list sweep `0x80068850` (844 B).** It resolves the current slot record
  (`stage + slot*0x3B0 + tag*4`), reads the `0xB0` list pointer and the `+0x1B0`
  count, and for each 16-byte list entry converts the `u16` centre at `+0x02`
  / `+0x04` into grid edges using the row's `+0x0C..+0x22` fields. It either
  tests both axes (`mode == 0`) or only the axis whose pinned cell matches,
  honouring the direction words `+1` (suppress), `-1` (far edge) and `0`
  (near edge). Hits are marked through `0x80068DE0`, the byte mask at
  `player + stride + 0x7E0` and the `u16` array at `player + 2*stride + 0x10`.
- **The four 30-step counters and their three float getters.** `0x80068BA0`,
  `0x80068C30`, `0x80068CC0` and `0x80068D50` bump the words at `+0x00`,
  `+0x04`, `+0x08` and `+0x0C` and wrap to zero at 30. `0x80068BC0` returns
  `1900 - 16*n`, `0x80068C50` returns `-2500 + 16*n`, and `0x80068CE0`
  returns `400 - 8*n`, each advancing its counter through the matching
  increment helper. The fourth counter's getter is `0x80068D70`, outside this
  lane.

## Evidence notes

* Every size was re-checked against prologue + `blr`:
  `0x80068320` -> 768, `0x80068620` -> 552 (padding at `0x80068848`),
  `0x80068850` -> 844 (padding at `0x80068B9C`), `0x80068BC0` -> 100,
  `0x80068C50` -> 100, `0x80068CE0` -> 100, and the four increment bodies
  -> 32 each. All match the `main-gb` `functions.tsv`.
* Ghidra prints the `0x802DCD40`, `0x802DCD5C` and `0x802DCD8C` register-save
  stubs as calls and swaps `r3`/`r4` in the process; the Rust bodies read the
  incoming arguments directly, as the module's prologue note already requires.
  The `ldexpf` blocks in the three getter exports are the same artefact around
  the `xoris`/`0x43300000` integer-to-float sequence.
* Ghidra's `FUN_80068290`, `FUN_800682F0`, `FUN_80068310`, `FUN_8008E800` and
  `FUN_80068DE0` are not translated in this lane and are called by absolute
  address. `0x800680A0` is a one-instruction thunk to `0x80068070`, so all
  interval tests use `0x80068070`.
