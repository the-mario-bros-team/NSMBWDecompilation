# Coverage - wave 15 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the fifteenth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   tail of the segmented-enemy block that wave 14 left at `0x80A713B0`, plus
   the untranslated first half of `daEnHanachan_c`.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   entries around the selected range, and why each was left out.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand
  prevents a complete model; `stub` = address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ABI detail or
  receiver is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.
`main.dol` addresses (`0x8000xxxx`, `0x803xxxxx`, `0x8042xxxx`) are imported
absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 15 | Wave 15 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 302 | 30 | 332 |
| Of those, `translated` | 293 | 28 | 321 |
| Of those, `partial` | 9 | 2 | 11 |
| Of those, `stub` | 0 | 0 | 0 |
| Constructors / model binders | - | 1 | - |
| Chain / segment handlers | - | 8 | - |
| State callbacks and thunks | - | 3 | - |
| Probes, name and target selection | - | 3 | - |
| Transform and matrix helpers | - | 4 | - |
| Collision and bounds responses | - | 2 | - |
| Effects and player latch/release | - | 5 | - |
| State logic and release paths | - | 2 | - |
| Forwarders | - | 2 | - |
| Selected bytes | - | 4,764 | - |

The 30 new bodies cover 4,764 bytes (0.81 % of the 587,492 bytes that the gap
report listed as untranslated before this pass) and 30 of the 3,253 remaining
function entries (0.92 %). Confidence is **21 high** and **9 medium**. The two
48-byte forwarders are `partial`: their visible body is complete, but the
hidden `r12` operand of `0x802DCBFC` cannot be modelled by `platform::call3`.
There are no `stub` rows.

Cumulative after this pass: 332 bodies, 321 `translated`, 11 `partial`, 3,223
function entries and 582,728 bytes still untranslated.

## Selection

This pass was evidence-first and continuation-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Finished the wave-14 block.** Wave 14 stopped at `0x80A713B0`. The next
   clean entries after its deferred GQR exports are `0x80A71820`,
   `0x80A71910`, `0x80A719A0`, `0x80A71D60`, `0x80A72150`, `0x80A721F0`,
   `0x80A723F0` and `0x80A72500`; they all use the same `+0x540`/`+0x544`/
   `+0x238` chain and close the two segmented actors.
3. **Took the still-missing half of `daEnHanachan_c`.** The table at
   `0x80A729E0` was translated in wave 4, but the two callbacks it names
   (`0x80A72EB0`, `0x80A72F10`) and the extra word factory (`0x80A72F70`)
   still had no bodies. The class block then continues with the model binder,
   the matrix plumbing and the four-slot player latch up to `0x80A744D0`.
4. **Cross-checked the class identity from data, not from names.** The
   constructor at `0x80A73260` allocates `0x528` bytes, and `0x80A735D0`
   binds the archive `g3d/bros_mega.brres` with the model `bros_mega` and the
   clip `walk` (`0x80B4B7B8..0x80B4B7D8`). The state descriptors
   `0x80B72A68`/`0x80B72AE8` are the `Dush`/`Recover` entries written by the
   wave-4 table, so the block is `daEnHanachan_c`.
5. **Excluded every export with damaged control flow.** `0x80A71700` and the
   five chain entries at `0x80A715E0`, `0x80A71AA0`, `0x80A71C60` and
   `0x80A71F40` print `Removing unreachable block` plus GQR/`ldexpf`
   artefacts. They are still called by address where a translated body needs
   them.
6. **Read the data constants from the REL image.** Address minus base equals
   the file offset for this module, so the `f32`/`i16`/string values were
   checked directly; the Rust keeps the original data addresses instead of
   baking the values in.

### The selected block (`0x80A71820..0x80A744D0`, 30 functions)

The first eight entries are the chain actors' walk/attack/landing handlers.
`0x80A71820` installs the attack box and both walk-clip vectors,
`0x80A71910`/`0x80A719A0` maintain the per-segment hit offsets and effect
slots, and `0x80A71D60`/`0x80A721F0`/`0x80A723F0` are three one-segment-per-
timer sequences. `0x80A72150` and `0x80A72500` reset the chain and play the
landing effect.

The Hanachan half starts with the two identical state callbacks and the
`0x80357BF4` thunk, then the five-level `+0x30` probe and the `:`-suffix name
comparison. `0x80A735D0` is the model binder, `0x80A737B0` the pose/matrix
builder, and `0x80A739C0`/`0x80A73A50`/`0x80A73F80` the per-side matrix
plumbing. The combat half is `0x80A73B60` (collision dispatch), `0x80A73E20`
(playfield bounds), `0x80A73ED0` (target resolution), `0x80A73F20`/`0x80A74410`/
`0x80A744D0` (effects) and `0x80A740A0`/`0x80A74140` (four-slot player latch
and release). `0x80A73FD0` is the countdown that chooses the next state.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A71820` | `fn_80a71820` | Write the attack box and both walk-clip vectors on every segment | translated | high | `80a71820_FUN_80a71820.c` |
| `0x80A71910` | `fn_80a71910` | Refresh both hit offsets of every segment from the owner position | translated | high | `80a71910_FUN_80a71910.c` |
| `0x80A719A0` | `fn_80a719a0` | Release the two auxiliary containers of every segment | translated | high | `80a719a0_FUN_80a719a0.c` |
| `0x80A71D60` | `fn_80a71d60` | One-segment-per-timer grow sequence, then `Dush` | translated | high | `80a71d60_FUN_80a71d60.c` |
| `0x80A72150` | `fn_80a72150` | Rebind the walk clips, step every segment, reset the attack index | translated | medium | `80a72150_FUN_80a72150.c` |
| `0x80A721F0` | `fn_80a721f0` | Attack sequence with per-segment slots, then `Recover` | translated | high | `80a721f0_FUN_80a721f0.c` |
| `0x80A723F0` | `fn_80a723f0` | Trigger the per-segment reaction in order, then `Dush` | translated | high | `80a723f0_FUN_80a723f0.c` |
| `0x80A72500` | `fn_80a72500` | Reset the chain, write `+0x88`, play the landing effect | translated | medium | `80a72500_FUN_80a72500.c` |
| `0x80A72EB0` | `fn_80a72eb0` | Shared `daEnHanachan_c` state callback | translated | high | `80a72eb0_FUN_80a72eb0.c` |
| `0x80A72F10` | `fn_80a72f10` | `DieFall` state callback (same body) | translated | high | `80a72f10_FUN_80a72f10.c` |
| `0x80A72F70` | `fn_80a72f70` | Return the shared extra word `0x80357BF4` | translated | high | `80a72f70_FUN_80a72f70.c` |
| `0x80A73060` | `fn_80a73060` | Five-level `+0x30` probe with a recursive tail | translated | high | `80a73060_FUN_80a73060.c` |
| `0x80A73140` | `fn_80a73140` | Compare the object's `:`-suffix name | translated | high | `80a73140_FUN_80a73140.c` |
| `0x80A73200` | `fn_80a73200` | First `0x802DCBFC` destructor forwarder | partial | medium | `80a73200_FUN_80a73200.c` |
| `0x80A73230` | `fn_80a73230` | Second `0x802DCBFC` destructor forwarder | partial | medium | `80a73230_FUN_80a73230.c` |
| `0x80A735D0` | `fn_80a735d0` | Bind the `bros_mega` model and `walk` clip | translated | high | `80a735d0_FUN_80a735d0.c` |
| `0x80A737B0` | `fn_80a737b0` | Place the `+0x544` holder from position, angles and `+0xD4` | translated | high | `80a737b0_FUN_80a737b0.c` |
| `0x80A73980` | `fn_80a73980` | Release `+0x148` and `+0x5D8` | translated | high | `80a73980_FUN_80a73980.c` |
| `0x80A739C0` | `fn_80a739c0` | Fill four per-side matrices and two holder matrices | translated | high | `80a739c0_FUN_80a739c0.c` |
| `0x80A73A50` | `fn_80a73a50` | Copy the target's matrix into the per-side block | translated | medium | `80a73a50_FUN_80a73a50.c` |
| `0x80A73B60` | `fn_80a73b60` | Collision dispatch: base response or damage | translated | medium | `80a73b60_FUN_80a73b60.c` |
| `0x80A73E20` | `fn_80a73e20` | Playfield bounds test against `0x80429E00` | translated | high | `80a73e20_FUN_80a73e20.c` |
| `0x80A73ED0` | `fn_80a73ed0` | Resolve `+0x740` to an unclaimed target | translated | high | `80a73ed0_FUN_80a73ed0.c` |
| `0x80A73F20` | `fn_80a73f20` | Spawn effect `0x75` at the per-side matrix | translated | medium | `80a73f20_FUN_80a73f20.c` |
| `0x80A73F80` | `fn_80a73f80` | Copy matrix `slot` into a three-word destination | translated | high | `80a73f80_FUN_80a73f80.c` |
| `0x80A73FD0` | `fn_80a73fd0` | Countdown and phase-driven state choice | translated | high | `80a73fd0_FUN_80a73fd0.c` |
| `0x80A740A0` | `fn_80a740a0` | Latch up to four same-side players | translated | medium | `80a740a0_FUN_80a740a0.c` |
| `0x80A74140` | `fn_80a74140` | Release the latched players | translated | medium | `80a74140_FUN_80a74140.c` |
| `0x80A74410` | `fn_80a74410` | Ground smoke at a caller position | translated | high | `80a74410_FUN_80a74410.c` |
| `0x80A744D0` | `fn_80a744d0` | Ground smoke at `+0x6B0`/`+0xB0` | translated | high | `80a744d0_FUN_80a744d0.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A715E0` | 280 B | Removed unreachable block; GQR/`ldexpf` artefacts |
| `0x80A71700` | 280 B | Removed unreachable block; GQR artefacts (called by address) |
| `0x80A71AA0` | 444 B | Removed unreachable block; GQR artefacts |
| `0x80A71C60` | 240 B | GQR artefacts in the `0x43300000` conversions |
| `0x80A71F40` | 520 B | Removed unreachable block; GQR artefacts |
| `0x80A74570` | 136 B | GQR/`ldexpf` preamble; the `f64` first argument cannot be recovered |
| `0x80A74600` | 176 B | Calls `0x80A74570` with a `double` first argument |
| `0x80A746E0` | 416 B | Clean per-frame handler; left for the next pass |
| `0x80A748C0` | 344 B | Clean per-frame handler; left for the next pass |
| `0x80A74A20` | 132 B | Calls `0x80A74570`; f64 magic-number conversion |
| `0x80A74AC0` | 452 B | Clean three-mode handler; left for the next pass |
| `0x80A74CB0` | 228 B | Clean angle handler; left for the next pass |
| `0x80A74DA0` | 112 B | Calls `0x80A74570`; f64 magic-number conversion |
| `0x80A74E20` | 256 B | Clean two-branch handler; left for the next pass |
| `0x80A74F20` | 140 B | Calls `0x80A74570`; f64 magic-number conversion |
| `0x80A74FC0` | 460 B | Clean two-mode handler; left for the next pass |

The clean entries `0x80A746E0`, `0x80A748C0`, `0x80A74AC0`, `0x80A74CB0`,
`0x80A74E20` and `0x80A74FC0` are the rest of `daEnHanachan_c` and are the
best next targets. `0x80A75800` is the `daEnHimanBros_c` table, already
translated in wave 4.

## Unknowns preserved in code

1. **Two auxiliary segment binders.** `0x800B66F0` and `0x800B6790` receive a
   segment plus the three-float vectors at `0x80B2DB04`/`0x80B2DB28`. Their
   names and exact effect are outside this lane; they stay addresses.
2. **`0x80A71700`.** The walk binder is GQR-damaged in the export. Its two
   callers (`0x80A72150`, `0x80A72500`) pass the actor in `r3`, which Ghidra
   does not print; the Rust passes it explicitly and marks the inference.
3. **Collision-type query `0x800964A0`.** The export prints no arguments. The
   Rust keeps the zero-argument call and reads the victim from
   `collision + 4`, as the caller does.
4. **`0x806F48A0` and the player latches.** The matrix copy into
   `+0x6E0 + 0x30 * side` and the `0x80137510`/`0x80137650` player
   lock/unlock pair are only known from their call shapes. Their gameplay
   meaning is `// inferred:`.
5. **Two descriptor objects.** `0x80B72B68` and `0x80B72BE8` are passed to the
   class state-switch slot `+0xD4` but are not among the six registered
   `daEnHanachan_c` states. They keep `DESC_` names and an `// unknown:` note.
6. **Two overlapping `f64` globals.** `0x8042CC88`/`0x8042CC8C` are printed as
   `_DAT_` doubles, but the sibling branch of the same function promotes the
   two `f32` values `0.0` and `24.0`. The Rust reads them as `f32` and marks
   the inference.
7. **`0x80064610` return type.** The export declares the effect factory
   `void`, while every caller uses `r3` as an object handle. `fn_80a73f20`
   returns that word and the doc records the discrepancy.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. `cargo check` reports no warning or error in
`src/game/enemies.rs`. This lane modified only `src/game/enemies.rs`, this
page and `ledger/agents/wave15-enemies.csv`.
