# Coverage - wave 25 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-fifth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 28 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   contiguous `daEnItem_c` handler cluster immediately after the wave-24 block,
   `0x80A875B0..0x80A89460`.
3. **What is still missing?** - *Deferred neighbours* names the one skipped
   export inside the run and the next cluster after the gap.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand
  prevents a complete model; `stub` = address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or an ABI detail is
  inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C file
  is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 25 | Wave 25 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 568 | 28 | 596 |
| Of those, `translated` | 553 | 28 | 577 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| `daEnItem_c` handler bodies | - | 28 | - |
| Selected bytes | - | 5,936 | - |

The 28 new bodies cover 5,936 bytes (1.09 % of the 542,220 bytes that were
still untranslated before this pass; 5,936 / 542,220 = 1.09 %) and 28 of the
2,987 remaining function entries (0.94 %). Confidence is **24 high** and
**4 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 596 bodies, 577 `translated`, 15 `partial`, 2,959
function entries and 536,284 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 28 bodies were written before
   any ledger row was added.
2. **Continued the wave-24 edge.** The wave-24 page names
   `0x80A87640..0x80A88D50` as the next handler cluster and defers it intact.
   A density scan of `functions.tsv` showed that the run is even larger: it
   starts at `0x80A875B0` (one 128-byte entry before `0x80A87640`) and
   continues without a hole through `0x80A89460`. This pass takes that whole
   contiguous run except one damaged export.
3. **Kept one class, one lifecycle.** The run is the per-state tail of the
   `daEnItem_c` switch-item actor whose state records live in BSS at
   `0x80B737xx` and `0x80B73AF8`. The wave-24 block already installed the
   helpers used here (`0x80A85C30`, `0x80A85D20`, `0x80A85E10`, `0x80A85EF0`,
   `0x80A85F10`, `0x80A86070`, `0x80A86160`, `0x80A86420`, `0x80A86470`,
   `0x80A865E0`, `0x80A86750`, `0x80A85A20`, `0x80A85AD0`), so the cluster
   reuses them instead of re-declaring behaviour.
4. **Took the whole state machine, not isolated helpers.** The selected run
   contains the drop/launch entries (`0x80A875B0`, `0x80A877E0`,
   `0x80A87980`), their per-frame updates (`0x80A878E0`, `0x80A87A90`,
   `0x80A87C40`, `0x80A87D50`, `0x80A87DE0`), the hover retire tail
   (`0x80A87F20`), the placed/ground movers (`0x80A88040..0x80A888C0`), the
   owner-follow copier (`0x80A88D50`) and the type/state mappers
   (`0x80A891C0..0x80A89460`).
5. **Read every numeric constant from the REL image.** The new constants are
   data-section floats, not text literals:
   `0x80B2E5A8 = 3.0`, `0x80B2E5AC = 16.0`, `0x80B2E5D0 = 12.0`,
   `0x80B2E6A8 = -2.0`, `0x80B2E6AC = -4.0`, `0x80B2E6B0 = 32.0`,
   `0x80B2E6B4 = 272.0`, `0x80B2E6B8 = 96.0`, `0x80B2E6BC = -0.0625`,
   `0x80B2E6C0 = 1.8/-1.8`, `0x80B2E6C8 = 1.8/-1.8`,
   `0x80B2E6D0 = 1.1`, `0x80B2E6D4 = 0.3`, `0x80B2E6D8 = 2.1`,
   `0x80B2E6DC = 3.4`, `0x80B2E6E0 = 3.0/-3.0`,
   `0x80B2E6E8 = 2.0/-2.0`, and the AABB words
   `0x80B2E4D8`/`0x80B2E4E8` (`0, 16, 0, 32`). The trigger descriptors
   `0x80B2E518` and `0x80B2E524` hold `0x00000800`; `0x80B2E530` holds
   `0x00000801`.
6. **Excluded the one export with damaged control flow.** `0x80A88BC0`
   (368 bytes, the ten-slot ring walk) is printed with `unaff_GQR0`/
   `unaff_GQR3`, four `ldexpf` artefacts and a prologue/epilogue pair printed
   as calls to `0x802DCD6C`/`0x802DCDB8`. It is deferred rather than guessed;
   its neighbours `0x80A88A20` and `0x80A88D50` are clean and were taken.
7. **Kept `main.dol` callees by address.** The six new opaque callees are
   `0x80060250`, `0x800B5180`, `0x80096F60`, `0x80097050`, `0x800B1100` and
   `0x80162590`; `0x800E20A0`, `0x8006D090`, `0x8006EF40` and `0x80070CB0`
   already had constants in the file. The two `main.dol` tables
   `0x80354E60`/`0x80354EA0` are read through the platform helpers and marked
   `// unknown:`.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A875B0` | `fn_80a875b0` | Arms the drop speed pair, ground probe, contact test | translated | high | `80a875b0_FUN_80a875b0.c` |
| `0x80A87640` | `fn_80a87640` | Held-item frame: input bits, sound `0x264`, collection | translated | high | `80a87640_FUN_80a87640.c` |
| `0x80A877E0` | `fn_80a877e0` | Drop/launch entry selected by phase `+0xDC0` | translated | high | `80a877e0_FUN_80a877e0.c` |
| `0x80A878E0` | `fn_80a878e0` | Arm-then-drop frame with `+0xD74` countdown | translated | high | `80a878e0_FUN_80a878e0.c` |
| `0x80A87980` | `fn_80a87980` | Drop entry, special type `0xB`, damped contact | translated | high | `80a87980_FUN_80a87980.c` |
| `0x80A87A90` | `fn_80a87a90` | Drop frame split by type `0xB`; collection burst | translated | high | `80a87a90_FUN_80a87a90.c` |
| `0x80A87BB0` | `fn_80a87bb0` | Ground-move entry, speed `4.0`/`-4.0`, timer `8` | translated | high | `80a87bb0_FUN_80a87bb0.c` |
| `0x80A87C40` | `fn_80a87c40` | Ground-move frame with `+0xDD8` countdown | translated | high | `80a87c40_FUN_80a87c40.c` |
| `0x80A87CC0` | `fn_80a87cc0` | Motion reset and contact-box re-arm | translated | high | `80a87cc0_FUN_80a87cc0.c` |
| `0x80A87D50` | `fn_80a87d50` | Reset frame; waits for `+0xEC <= 0.0` | translated | high | `80a87d50_FUN_80a87d50.c` |
| `0x80A87DE0` | `fn_80a87de0` | Hover/launch entry with AABB writes | translated | high | `80a87de0_FUN_80a87de0.c` |
| `0x80A87F20` | `fn_80a87f20` | Hover retire tail with score and effect | translated | high | `80a87f20_FUN_80a87f20.c` |
| `0x80A88040` | `fn_80a88040` | Re-arms the drop when `+0xDFE` is set | translated | high | `80a88040_FUN_80a88040.c` |
| `0x80A88090` | `fn_80a88090` | Placed-state reset and trigger box install | translated | high | `80a88090_FUN_80a88090.c` |
| `0x80A88150` | `fn_80a88150` | Placed-state frame with `+0xD74` countdown | translated | high | `80a88150_FUN_80a88150.c` |
| `0x80A881D0` | `fn_80a881d0` | Placed entry, type `0xB` trigger without a box | translated | high | `80a881d0_FUN_80a881d0.c` |
| `0x80A88300` | `fn_80a88300` | Moving-item frame with the four-bit mask | translated | high | `80a88300_FUN_80a88300.c` |
| `0x80A88480` | `fn_80a88480` | Launch velocity with `0x800B2F10` jitter | translated | medium | `80a88480_FUN_80a88480.c` |
| `0x80A88610` | `fn_80a88610` | Bounce entry, type `0xB` twirl and fixed tilt | translated | medium | `80a88610_FUN_80a88610.c` |
| `0x80A88810` | `fn_80a88810` | Type dispatch to the `0xB` frame or `0x80A88300` | translated | high | `80a88810_FUN_80a88810.c` |
| `0x80A888C0` | `fn_80a888c0` | Hover entry with `80.0` drop, ground probe | translated | high | `80a888c0_FUN_80a888c0.c` |
| `0x80A88A20` | `fn_80a88a20` | Yaw step, `+10` counter, state switch | translated | medium | `80a88a20_FUN_80a88a20.c` |
| `0x80A88D50` | `fn_80a88d50` | Owner position/velocity copy and score request | translated | medium | `80a88d50_FUN_80a88d50.c` |
| `0x80A891C0` | `fn_80a891c0` | Player action state to `0/1/2` mapper | translated | high | `80a891c0_FUN_80a891c0.c` |
| `0x80A89240` | `fn_80a89240` | Guarded `+0xD8C` -> `+0xDCA` type mapper | translated | high | `80a89240_FUN_80a89240.c` |
| `0x80A89390` | `fn_80a89390` | Unconditional `+0xD8C` -> `+0xDCA` mapper | translated | high | `80a89390_FUN_80a89390.c` |
| `0x80A89430` | `fn_80a89430` | Idle-type predicate (`0`, `6`, `0xD`) | translated | high | `80a89430_FUN_80a89430.c` |
| `0x80A89460` | `fn_80a89460` | Clears `+0x36D` for types `3`, `0xF`, `0x10` | translated | high | `80a89460_FUN_80a89460.c` |

## What the handlers do

The cluster is the part of `daEnItem_c` that runs after the item has been
placed or dropped:

- **Drop and launch.** `+0xDC0` chooses how the drop starts, `+0xD74` is the
  shared fall timer, `+0xDAC` selects the visual variant and `+0xDCA` is the
  effective item type (`2` = red coin-like, `0xB` = propeller switch).
- **Hover variant.** `+0xD98 == 3` selects the mover that keeps the item over
  its owner. It writes the AABB `(+0x318..+0x32C)` and extents
  `(+0x338..+0x344)`, runs the ground probe and steps `+0xB0`.
- **Bounce variant.** The other path asks `0x800B2F10` for a jitter and builds
  the launch speed from `0.3`/`2.1` (or `0.5`/`1.1` for type `0xB`) plus the
  side-symmetric tilt tables.
- **Retire.** `0x80A87F20` advances the yaw, counts `+0xDCC` down, awards the
  score through `0x80060250`/`0x800B5180` (selected by the `0x8042A21F` byte),
  requests the score effect through `0x800E20A0` and calls `0x80162510`.
- **Type mapping.** `0x80A89240`/`0x80A89390` derive `+0xDCA` from the move id
  `+0xD8C`; `0x80A891C0` reads the owner action state from player `+0x1090`.

## Deferred neighbours

### Skipped export inside the run

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A88BC0` | 368 B | Ten-slot ring walk (`+0xC24`, 10 x `0xC` entries). Ghidra prints `unaff_GQR0`/`unaff_GQR3`, four `ldexpf` artefacts and a prologue/epilogue pair as calls to `0x802DCD6C`/`0x802DCDB8`, so the real ABI (including the actor in `r3`) is not recoverable from the export. |

### Next cluster

| Address | Size | Note |
|---|---:|---|
| `0x80A8A5C0`.. | 88 B + | Next `daEnItem_c`/item-helper group after a 0x1160-byte gap. Not started; it needs its own class identification pass. |

### Untouched helpers next to the run

`0x80A85200`, `0x80A85260`, `0x80A85470`, `0x80A85520`, `0x80A85590`,
`0x80A85650`, `0x80A85C30`, `0x80A85EF0`, `0x80A86510` and `0x80A86600` are
still address-only callees. They are called by the new bodies exactly as the
export shows and are documented in the function-level `# Side effects`
sections.

## Unknowns preserved in code

1. **BSS state records.** `0x80B73AF8` (and the `0x80B737xx` records used by
   wave 24) live in BSS and are filled by the runtime state registration. Their
   static contents are empty, so `0x80A88A20` keeps the record as an address
   and the state name is `// unknown:`.
2. **`0x80354E60` / `0x80354EA0`.** The `+10` counter update in `0x80A88A20`
   indexes two `main.dol` tables. The element types are not visible in this
   REL, so both stay address-only with `// unknown:` notes.
3. **`0x800B2F10`.** The jitter helper is called with one `f64` and returns a
   single-precision value in `f1`; the Rust models that ABI directly. Its
   distribution and owner are outside this lane.
4. **`0x800B1100`.** The call in `0x80A88D50` takes two `(double)`-printed
   floats, one position word, one player byte and a flag. The argument roles
   are `// inferred:` from the shape only.
5. **`0x80060250` / `0x800B5180`.** The two score paths are chosen by the
   `0x8042A21F` enable byte. The export reads `+0xD90` as a word and narrows
   it to `char` for `0x80060250`; the Rust keeps that narrowing.
6. **Field roles.** `+0xD8C`, `+0xD98`, `+0xDA8`, `+0xDC0`, `+0xD68`,
   `+0xDFE`, `+0x36D` and the AABB block `+0x318..+0x344` are named from the
   branch shape, not from a symbol. The section header in `src/game/enemies.rs`
   marks the whole map `// inferred:`.
7. **`+0x0D8/+0x0DC/+0x0E0`.** `0x80A88D50` copies one owner word
   (`+0x8F8`) into all three slots. The export does not distinguish them, so
   they are kept as three separate stores exactly as printed.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`node tools/doc-lint.mjs src` reports 0 violations across 68 files.
`cargo check` succeeds and reports no error or warning in
`src/game/enemies.rs`; the remaining warnings are in other lanes' files and were
not modified here.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave25-enemies.csv`.
