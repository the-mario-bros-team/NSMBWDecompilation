# Coverage - Wave 113 (`game::enemies`, the `0x80B10000` bucket)

## How to read this file

This page records the Wave 113 `game::enemies` lane: 27 functions of the
`0x80B10000` bucket of REL module 3, `d_enemiesNP`, appended to
`src/game/enemies.rs` as section 94 of that file. Every row of the machine
readable mirror (`ledger/agents/wave113-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 27 | 5,328 | 8 | 19 |

## What this pass covers

- **`0x80B1CC50..0x80B1D9B0`, the `wanwan` head actor.** `0x80B1CC50` loads
  `g3d/wanwan.brres` and resolves the `wanwan` node, which ties the run to the
  chain actor of wave 96 ([`fn_80b1f970`] onward) that resolves `wanwan_kui`
  and `wanwan_chain_ring` from the same archive. Covered here: the model and
  dual-clip binder, the territory step (two name tests, `+0x164`/`+0x168`
  ground distances and the `0x80064F50` retire), the timer reset, the `+0x7C`
  matrix rebuild, the ground-bound angle step with its `0x802DE514` compares,
  the ground snap with the `Wm_en_landsmoke_s` effect and sound `0x1C7`, the
  XZ chase distance and the `+0x5F0` spawn gate.
- **`0x80B1DC00..0x80B1EF80`, the head actor's state block.** The two clip
  binders, the height-band approach chooser (`0..5`), the idle entry, the
  speed seeder, the eight-slot mode ring at `+0x65C`, the three-state step, the
  attack entry with the `+0x1EC` collision install, the hop entry and counter,
  the run-away gate, the collision responder that copies the seven-word
  contact template into `+0x414..+0x42C`, the turnaround entry and the facing
  step.
- **`0x80B175B0..0x80B17670`, three pure angle helpers.** A clamped `i16` step
  towards a target, the toggle-driven mirror step, and the two-axis caller
  that reads the `0x80B69E64..0x80B69E6A` limits.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80B175B0` | `fn_80b175b0` | 104 | high |
| `0x80B17620` | `fn_80b17620` | 72 | high |
| `0x80B17670` | `fn_80b17670` | 248 | medium |
| `0x80B1CC50` | `fn_80b1cc50` | 336 | medium |
| `0x80B1CDA0` | `fn_80b1cda0` | 368 | medium |
| `0x80B1CFB0` | `fn_80b1cfb0` | 84 | high |
| `0x80B1D210` | `fn_80b1d210` | 120 | medium |
| `0x80B1D290` | `fn_80b1d290` | 72 | medium |
| `0x80B1D2E0` | `fn_80b1d2e0` | 188 | medium |
| `0x80B1D440` | `fn_80b1d440` | 212 | high |
| `0x80B1D680` | `fn_80b1d680` | 92 | medium |
| `0x80B1D9B0` | `fn_80b1d9b0` | 140 | medium |
| `0x80B1DC00` | `fn_80b1dc00` | 112 | medium |
| `0x80B1DC70` | `fn_80b1dc70` | 116 | medium |
| `0x80B1DCF0` | `fn_80b1dcf0` | 192 | medium |
| `0x80B1DDB0` | `fn_80b1ddb0` | 180 | medium |
| `0x80B1DED0` | `fn_80b1ded0` | 248 | medium |
| `0x80B1E200` | `fn_80b1e200` | 260 | medium |
| `0x80B1E320` | `fn_80b1e320` | 340 | medium |
| `0x80B1E480` | `fn_80b1e480` | 172 | high |
| `0x80B1E6C0` | `fn_80b1e6c0` | 240 | medium |
| `0x80B1E7B0` | `fn_80b1e7b0` | 112 | high |
| `0x80B1E820` | `fn_80b1e820` | 220 | high |
| `0x80B1E900` | `fn_80b1e900` | 316 | medium |
| `0x80B1EA50` | `fn_80b1ea50` | 420 | medium |
| `0x80B1EEB0` | `fn_80b1eeb0` | 188 | medium |
| `0x80B1EF80` | `fn_80b1ef80` | 176 | high |

## Deliberate skips

The bucket had 66 untranslated addresses before this pass and 39 after it. The
neighbours this pass examined and rejected are:

- **`0x80B1D090` (328 bytes), `0x80B1D520` (352 bytes), `0x80B1DA40` (180
  bytes), `0x80B1DFE0` (540 bytes), `0x80B1E540` (380 bytes).** Every one is
  wrapped in a GQR conversion block (`unaff_GQR0`/`unaff_GQR3`, `ldexpf`) whose
  quantisation feeds the values written back to the actor, exactly like the
  `0x80A0E580` skip recorded in `docs/COVERAGE_WAVE112_ENEMIES.md`. The head
  actor therefore still calls `0x80B1D520` through its raw address (noted in
  [`fn_80b1e200`]) instead of a translated body.
- **`0x80B1B0E0` (32 bytes).** The export is a single `halt_baddata()`, so no
  honest body exists.
- **`0x80B1B3F0` (248 bytes).** It calls `0x80B1B0E0`, the truncated function
  above, and uses that call's result as an animation flag, so the branch cannot
  be reconstructed.
- **`0x80B18F20` (228 bytes).** The export declares one argument but the body
  recurses with two (`FUN_80b18f20(param_1,iVar2)`), so the walker's real
  prototype is unresolved.
- **`0x80B19EE0` / `0x80B19FA0` (104 / 176 bytes), `0x80B1AD30` (264 bytes),
  `0x80B1B3F0` neighbours.** The exports print their callees without a
  receiver (`FUN_80b19e30()`, `func_0x80162d00(...)` returning a 64-bit pair)
  and read uninitialised stack words, so both the callee identity and the
  argument order would be guesses.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 27 sizes sum to
  5,328 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 27 was in
  `ledger/functions.csv` before this pass.
- `node tools/check-lane-csv.mjs wave113` prints
  `wave113-enemies.csv: 27 rows` and `OK`.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- The module header was updated: a new cluster item 94, the cluster count
  (ninety-three to ninety-four), the function totals (2,500 to 2,527 public
  translations, 2,429 to 2,456 `fn_<addr>` bodies) and the records list.
