# Coverage - wave 27 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-seventh translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 27 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   continuation after the wave-26 `kurage` helpers, which completes that
   actor's state machine.
3. **What is still missing?** - *Deferred neighbours* names the two exports
   skipped inside the run and the next cluster after it.

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

| Metric | Before wave 27 | Wave 27 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 626 | 27 | 653 |
| Of those, `translated` | 607 | 27 | 634 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Selected bytes | - | 6,840 | - |

The 27 new bodies cover 6,840 bytes (1.29 % of the 531,796 bytes that were
still untranslated before this pass; 6,840 / 531,796 = 1.29 %) and 27 of the
2,929 remaining function entries (0.92 %). Confidence is **18 high** and
**9 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 653 bodies, 634 `translated`, 15 `partial`, 2,902
function entries and 524,956 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 27 bodies were written before
   any ledger row was added.
2. **Continued at the wave-26 edge.** Wave 26 ends at `0x80A8C3B0`; the next
   export in `functions.tsv` is `0x80A8C3C0`, so this run starts exactly where
   the previous pass stopped and continues through the constructor at
   `0x80A8EBD0`.
3. **Identified the class from its resources.** `0x80A8C550` loads
   `g3d/kurage.brres` (`0x80B4F57C`), the model `kurage` (`0x80B4F590`) and
   the clips `wait` (`0x80B4F610`), `eye_anim` (`0x80B4F618`), `BGM_anim`
   (`0x80B4F624`), `swim_up_st` (`0x80B4F630`), `swim_up_ed` (`0x80B4F63C`),
   `swim_up` (`0x80B4F648`), `swim_down_st` (`0x80B4F650`),
   `swim_down` (`0x80B4F660`) and `eye_search_anim` (`0x80B4F670`); the eye
   material is `mat_kurage_eye` (`0x80B4F680`). The class is therefore the
   `kurage` (jellyfish) actor; `daEnKurage_c` is `// inferred:` from those
   names, not a recovered symbol.
4. **Completed one lifecycle.** The run covers the variant initialiser
   (`0x80A8C3C0`), the model/clip binder (`0x80A8C550`), the release path
   (`0x80A8CA50`), the four swim/dive handlers
   (`0x80A8CC20`/`0x80A8CCD0`/`0x80A8CF60`/`0x80A8D020`), the idle bob and
   surface steps (`0x80A8D370`/`0x80A8D430`/`0x80A8D580`), the eye search and
   tracking block (`0x80A8D970..0x80A8E240`), the two state callbacks
   (`0x80A8E810`/`0x80A8E870`), the two constant producers
   (`0x80A8E8D0`/`0x80A8E8E0`), the chain walker and name predicate
   (`0x80A8E9D0`/`0x80A8EAB0`) and the `0x780`-byte constructor
   (`0x80A8EBD0`).
5. **Cross-checked duplicated bodies.** `0x80A8E870` is byte-for-byte
   `0x80A8E810` (`0x8015F800` plus the optional `0x802B9280`), the same
   three-way callback already translated as [`fn_80a8b8d0`]. The eye and
   state-switch call shapes match the wave-26 helpers.
6. **Read every numeric constant from the REL image.** The values used here
   are data-section floats: `0x80B2E768 = 0.0`, `0x80B2E774 = 1.0`,
   `0x80B2E778 = -0.4`, `0x80B2E780 = 1.0`, `0x80B2E7BC = 1468.0`,
   `0x80B2E7C0 = 3.0`, `0x80B2E7C4 = -125.0`, `0x80B2E7C8 = 200.0`,
   `0x80B2E7CC = 500.0`, `0x80B2E7D0 = 16.0`, `0x80B2E7D4 = 50.0`,
   `0x80B2E7D8 = 15.0`, `0x80B2E7EC = -2.5`, `0x80B2E7F0 = 5.0`,
   `0x80B2E7F4 = 0.2`, `0x80B2E7F8 = 1.05`, `0x80B2E7FC = 2.0`,
   `0x80B2E800 = 2.5`, `0x80B2E804 = 120.0`, `0x80B2E808 = 7.0`,
   `0x80B2E80C = 0.95`, `0x80B2E810 = 6.0`, `0x80B2E814 = 0.5`,
   `0x80B2E818 = 0.45`, `0x80B2E81C = 4.0`, `0x80B2E820 = 80.0`,
   `0x80B2E824 = 2.4`, `0x80B2E828 = 0.98`, `0x80B2E82C = 17.0`,
   `0x80B2E830 = 10.0`, `0x80B2E834 = 13.0`, `0x80B2E838 = 11.0`,
   `0x80B2E840 = 1.87`, `0x80B2E848 = 0.08`, `0x80B2E84C = 23.0`,
   `0x80B2E85C = 25.0`, `0x80B2E860 = 170.0`, `0x80B2E864 = 48.0`,
   `0x80B2E868 = 0.03`, `0x80B2E86C = 0.02`, `0x80B2E870 = 0.15`,
   `0x80B2E874 = 100.0`, `0x80B2E878 = -0.2`, `0x80B2E87C = -0.15`,
   `0x80B2E880 = -0.1`, `0x80B2E884 = 0.1`, `0x80B2E888 = -0.06`,
   `0x80B2E88C = 0.06`, `0x80B2E890 = 150.0`, `0x80B2E894 = 70.0`.
   `0x80B2E7E0` is the eight-byte `double 2^52 + 2^31` used by the
   unsigned-to-double conversion at `0x80A8C3C0`; it is read with
   [`read_f64`].
7. **Kept damaged exports honest.** `0x80A8CCD0`, `0x80A8DB70` and
   `0x80A8DEF0` retain GQR/`ldexpf` prologue and epilogue artefacts from the
   paired-single helpers. The artefacts are outside the real body and are
   omitted; each function is marked `medium` with a note in its doc comment.
   The control flow itself is complete.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A8C3C0` | `fn_80a8c3c0` | Variant initialiser: scale, limits, depth `6/8/10/12`, state `0x80B73CB8` | translated | high | `80a8c3c0_FUN_80a8c3c0.c` |
| `0x80A8C550` | `fn_80a8c550` | `kurage` model and five-clip binder; calls `0x80A8BD20` | translated | high | `80a8c550_FUN_80a8c550.c` |
| `0x80A8CA50` | `fn_80a8ca50` | Release: model slot `+0x14`, blend block, ring slot `+0x10` | translated | high | `80a8ca50_FUN_80a8ca50.c` |
| `0x80A8CC20` | `fn_80a8cc20` | Starts the swim-up stroke (`-2.5`, `swim_up_st`) | translated | high | `80a8cc20_FUN_80a8cc20.c` |
| `0x80A8CCD0` | `fn_80a8ccd0` | Four-step swim-up handler; ends on `swim_up_ed` | translated | medium | `80a8ccd0_FUN_80a8ccd0.c` |
| `0x80A8CF60` | `fn_80a8cf60` | Starts the swim-down stroke (`-0.4` or `-1.0`) | translated | high | `80a8cf60_FUN_80a8cf60.c` |
| `0x80A8D020` | `fn_80a8d020` | Dive handler: states `1..4`, motion ramp, countdown | translated | medium | `80a8d020_FUN_80a8d020.c` |
| `0x80A8D370` | `fn_80a8d370` | Ends the dive: `wait` clip, `random(200)+100`, reference snapshot | translated | high | `80a8d370_FUN_80a8d370.c` |
| `0x80A8D430` | `fn_80a8d430` | Bobs between `reference -/+ 4.0` and leaves on a turning point | translated | medium | `80a8d430_FUN_80a8d430.c` |
| `0x80A8D580` | `fn_80a8d580` | Surface handler: `swim_up_st`, frame `0.0`, time `23.0` | translated | high | `80a8d580_FUN_80a8d580.c` |
| `0x80A8D970` | `fn_80a8d970` | Eye state: chain query `+0x28`/`+0x14`, target search, search clip | translated | medium | `80a8d970_FUN_80a8d970.c` |
| `0x80A8DAE0` | `fn_80a8dae0` | Locked-slot proximity test (`200.0` on Y and X) | translated | high | `80a8dae0_FUN_80a8dae0.c` |
| `0x80A8DB70` | `fn_80a8db70` | Four-slot target search, stores the slot and the lock flag | translated | medium | `80a8db70_FUN_80a8db70.c` |
| `0x80A8DD20` | `fn_80a8dd20` | Copies the locked player position into `out[0..2]` | translated | high | `80a8dd20_FUN_80a8dd20.c` |
| `0x80A8DD80` | `fn_80a8dd80` | Two-axis eye step (`0.03`/`0.02`) then the material write | translated | high | `80a8dd80_FUN_80a8dd80.c` |
| `0x80A8DE20` | `fn_80a8de20` | Clears eye tracking and restores `eye_anim` | translated | medium | `80a8de20_FUN_80a8de20.c` |
| `0x80A8DEF0` | `fn_80a8def0` | Eye yaw/pitch clamps and the `+/-0.06` centred band | translated | medium | `80a8def0_FUN_80a8def0.c` |
| `0x80A8E0A0` | `fn_80a8e0a0` | Writes yaw/pitch into `mat_kurage_eye` (clears flag `0x8`) | translated | medium | `80a8e0a0_FUN_80a8e0a0.c` |
| `0x80A8E190` | `fn_80a8e190` | Eye blend tick; `150.0` random blink interval | translated | medium | `80a8e190_FUN_80a8e190.c` |
| `0x80A8E240` | `fn_80a8e240` | Eye timer ramp `50.0..70.0` in `2.0` steps | translated | high | `80a8e240_FUN_80a8e240.c` |
| `0x80A8E810` | `fn_80a8e810` | State callback `0x8015F800` + optional `0x802B9280` | translated | high | `80a8e810_FUN_80a8e810.c` |
| `0x80A8E870` | `fn_80a8e870` | Second copy of the same state callback | translated | high | `80a8e870_FUN_80a8e870.c` |
| `0x80A8E8D0` | `fn_80a8e8d0` | Constant producer, returns `0x80358004` | translated | high | `80a8e8d0_FUN_80a8e8d0.c` |
| `0x80A8E8E0` | `fn_80a8e8e0` | Constant producer, returns `0x80357BF4` | translated | high | `80a8e8e0_FUN_80a8e8e0.c` |
| `0x80A8E9D0` | `fn_80a8e9d0` | Five `+0x30` owner links with the `+0xC` query, then recursion | translated | high | `80a8e9d0_FUN_80a8e9d0.c` |
| `0x80A8EAB0` | `fn_80a8eab0` | Name-suffix compare after the last `:` | translated | high | `80a8eab0_FUN_80a8eab0.c` |
| `0x80A8EBD0` | `fn_80a8ebd0` | `0x780`-byte `kurage` constructor, vtable `0x80B4FB18` | translated | high | `80a8ebd0_FUN_80a8ebd0.c` |

## What the handlers do

### The swim/dive state machine (10 bodies)

`0x80A8C3C0` is the entry that follows construction: it loads the model, stores
the movement/limit block (`+0x318..+0x324`), snapshots the current position
into `+0x684..+0x68C`, derives the depth from the setup word's low two bits
(`6/8/10/12` units of `16.0`) and seeds the random timer. It then enters the
BSS state `0x80B73CB8`.

The actual stroke machine lives in `+0x360`:

- step `1` climbs the swim offset `+0x828` by `+0x898 * 1.05` until it crosses
  zero (`0x80A8CCD0`), or waits for the draw timer in `0x80A8D020`;
- step `2` computes a vertical ramp from `+0xEC` and the remaining distance to
  `+0x688 - +0x880` and switches the eye/body blend;
- step `3` advances `+0xEC` by the ramp until the clip finishes, then binds
  `swim_up`/`swim_down` or the `_st`/`_ed` pair;
- step `4` waits for the clip and switches state again.

`0x80A8CF60`/`0x80A8CC20` are the two entry helpers, `0x80A8D370` ends a dive
and `0x80A8D580` surfaces. `0x80A8D430` is the idle bob: it moves `+0xB0`
between `+0x69C - 4.0` and `+0x69C + 4.0` and leaves when the countdown
`+0x890` reaches zero or the target flag is set.

### Eye search and tracking (7 bodies)

`0x80A8D970` runs once per frame: it asks the chain object at `+0x394`
(slot `+0x28`) for its current node, queries that node (slot `+0x14`), and
then either searches the four player slots (`0x80A8DB70`) or tests the locked
slot (`0x80A8DAE0`). When the eye blend `+0x618` is above zero and mode `0` is
active, it binds `eye_search_anim`.

`0x80A8DD80`'s neighbours turn the locked position into angles:
`0x80A8DD20` copies the player position, `0x80A8DD80` steps both axes with
`0x8015F470` (`0.03` yaw, `0.02` pitch), `0x80A8DEF0` recomputes the deltas
with the `0.15`/`48.0` yaw gain and the `0.2`/`100.0` pitch gain, and
`0x80A8E0A0` writes the two angles into the `mat_kurage_eye` material. The
blink loop is `0x80A8E240` (timer `50.0..70.0`) plus `0x80A8E190` (blend and
random interval `150.0`).

### Infrastructure (10 bodies)

`0x80A8C550` is the class binder: resource `g3d/kurage.brres`, model `kurage`,
clips `wait`, `eye_anim`, `BGM_anim`, `swim_up_st`, `swim_up_ed`, `swim_up`,
`swim_down_st`, `swim_down` and `eye_search_anim`, then the deferred
constructor `0x80A8BD20` and the ring sub-object at `+0x85C`/`+0x840`.
`0x80A8CA50` releases the model, the wave-26 blend block and the ring.

The remaining helpers are generic actor plumbing: the two state callbacks
(`0x80A8E810`/`0x80A8E870`), the two constant producers
(`0x80A8E8D0`/`0x80A8E8E0`), the `+0x30` chain walker (`0x80A8E9D0`), the
name-suffix predicate (`0x80A8EAB0`) and the `0x780`-byte constructor
(`0x80A8EBD0`).

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A8EB70` | 48 B | Only the `0x802DCBFC(param_2, param_2, param_1)` forwarder; the hidden `r12` this-adjust cannot be modelled (same stub as the wave-13 note). |
| `0x80A8EBA0` | 48 B | Second copy of the same `0x802DCBFC` forwarder. |

### Next cluster

| Address | Size | Note |
|---|---:|---|
| `0x80A8F2F0` | 64 B | First export after a `0x644`-byte hole; the next actor block starts here. |
| `0x80A8F330`..`0x80A90030` | 3.3 KB | More handlers of the same block; not started. |

## Unknowns preserved in code

1. **Class name.** `kurage` comes from the BRRES/model strings; `daEnKurage_c`
   is not in the export. The field map in the Rust is marked `// inferred:`.
2. **BSS descriptors.** `0x80B73CB8`, `0x80B73CF8` and `0x80B73D38` are
   written by the runtime state registration, so their static contents are
   empty and the state names are `// unknown:`.
3. **Dropped receivers.** The export prints several calls without their `r3`
   receiver: `0x80A8C550` at the top of `0x80A8C3C0`, `0x80A8DB70` in
   `0x80A8D020`, `0x80A8D970` in `0x80A8D020`/`0x80A8D430`, `0x80A8E0A0` in
   `0x80A8DE20`, the `+0x28`/`+0x14` chain queries in `0x80A8D970`, the
   `+0x6C` query in `0x80A8DB70` and the base constructor in `0x80A8EBD0`.
   Each is passed the actor/candidate and marked `// inferred:`.
4. **GQR artefacts.** `0x80A8CCD0`, `0x80A8DB70` and `0x80A8DEF0` keep
   `unaff_GQR0`/`ldexpf` blocks from the paired-single helpers. They are
   prologue/epilogue noise, not part of the body, and are omitted.
5. **Material helpers.** `0x8023B760`, `0x8025C660` and `0x8025C470` are
   `main.dol` addresses. Their exact record layout is inferred from the two
   stores at words 5 and 6 and the flag clear at word 0.
6. **`0x80A8DB70` first-byte store.** The export stores `(int)*pcVar5`, the
   first byte of the pointer returned by the `+0x6C` virtual call. The Rust
   keeps that store; why the original keeps only one byte is `// unknown:`.
7. **`0x80A8BD20`.** The `kurage` sub-constructor stays deferred (wave-26
   note); it is called by address from `0x80A8C550` and nowhere translated.
8. **`0x80A8E0A0` tag.** `0x802F8FF0` is stored as the `+0x548` query tag; its
   meaning is `// unknown:`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`cargo check` succeeds; the remaining warnings are in other lanes' files
(`src/runtime/bt.rs` and `src/runtime/input.rs` at the time of this pass) and
were not touched here. `node tools/doc-lint.mjs src` reports 0 violations
across 68 files.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave27-enemies.csv`.
