# Coverage - wave 28 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-eighth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 26 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   continuation after the wave-27 `kurage` constructor, through the
   `daEnJimenFPakkun_c` behaviour and into the world-map cloud actor.
3. **What is still missing?** - *Deferred neighbours* names the damaged exports
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

| Metric | Before wave 28 | Wave 28 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 653 | 26 | 679 |
| Of those, `translated` | 634 | 26 | 660 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Selected bytes | - | 3,500 | - |

The 26 new bodies cover 3,500 bytes (0.67 % of the 524,956 bytes that were
still untranslated before this pass; 3,500 / 524,956 = 0.67 %) and 26 of the
2,902 remaining function entries (0.90 %). Confidence is **22 high** and
**4 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 679 bodies, 660 `translated`, 15 `partial`, 2,876
function entries and 521,456 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 26 bodies were written before
   any ledger row was added. The `daEnJimenFPakkun_c` state table at
   `0x80A90740` was already translated as
   [`register_jimen_fpakkun_states`](../src/game/enemies.rs) and was therefore
   skipped; its three state descriptors are reused, not re-registered.
2. **Continued at the wave-27 edge.** Wave 27 ends at the `kurage` constructor
   `0x80A8EBD0`; the next export is `0x80A8F2F0`. The two `0x802DCBFC`
   forwarders at `0x80A8EB70`/`0x80A8EBA0` remain deferred for the wave-27
   reason (hidden `r12` this-adjust), so this run starts exactly at the next
   complete body.
3. **Identified the classes from resources and tables.**
   - The two constructors at `0x80A8F590`/`0x80A8FCD0` allocate `0x780` bytes
     and repeat the wave-27 `kurage` field layout store for store. Their class
     labels are `// inferred:` from that shared layout; only the vtable
     (`0x80B4FDF0`, `0x80B500F0`) and the `u16 3` at `+0x778` differ.
   - The `0x778`-byte constructor at `0x80A90B40` installs class vtable
     `0x80B504F8`, immediately after the state vtable `0x80B504AC` used by the
     `daEnJimenFPakkun_c` table at `0x80A90740`. The wait/attack steps switch
     through that class's `+0xD4` slot to the three descriptors registered by
     that table (`0x80B73DF8`, `0x80B73E38`, `0x80B73E78`).
   - The `0x728`-byte constructor at `0x80A91640` sits next to the
     `Wm_en_movecloud` string (`0x80B508F8`) and the `g3d/jugem.brres` /
     `jugem` / `cloud` / `cloud_wait` strings (`0x80B50908..0x80B50928`). The
     class is therefore the world-map cloud actor, `// inferred:`.
4. **Read every numeric constant from the REL image.** Used here:
   `0x80B2E974 = 0.0`, `0x80B2E978 = 8.0`, `0x80B2E988 = 224.0`,
   `0x80B2E98C = 1.5`, `0x80B2EAD0 = 0.0`, `0x80B2EAE0 = 1.0`. The aim tables
   at `0x80B2E950`/`0x80B2E954` are `i16` pairs `+10012/-10012` and
   `+640/-640`. The effect tables at `0x80B2E9C0` are three-`u32` records:
   index `0` = `3,1,0`, index `1` = `2,1,1`, index `2` = `3,1,2`.
5. **Kept damaged exports out.** `0x80A91D10` is `halt_baddata`, `0x80A91D30`
   has an unrecovered jump table, `0x80A91DA0` depends on `0x80A91D10`,
   `0x80A91BB0` has stack matrix slots Ghidra does not map, `0x80A91D70` is an
   untyped global `f64` subtraction, and `0x80A91F40` takes its actor from the
   `0x802DCD70` prologue fragment. None was forced into a guess; see
   *Deferred neighbours*.
6. **Reused the established shape.** `fn_80a90110` is the same aim-step pattern
   as the wave-21 `fn_80a70eb0`, `fn_80a91440` is the same five-link walker as
   the wave-27 `fn_80a8e9d0`, `fn_80a91520` is the same suffix predicate as
   `fn_80a8eab0`, and `fn_80a920d0` is the same model binder shape as the
   wave-9 `fn_80b002d0`.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A8F2F0` | `fn_80a8f2f0` | Unlinks the embedded objects at `+0x148`, `+0x5E8`, `+0x68C` | translated | high | `80a8f2f0_FUN_80a8f2f0.c` |
| `0x80A8F330` | `fn_80a8f330` | Matrix rebuild `0x8002F3F0` then turn step `0x8002F540` | translated | medium | `80a8f330_FUN_80a8f330.c` |
| `0x80A8F470` | `fn_80a8f470` | Sound `0x1AC` at the `+0xAC` position | translated | high | `80a8f470_FUN_80a8f470.c` |
| `0x80A8F590` | `fn_80a8f590` | `0x780`-byte jellyfish constructor, vtable `0x80B4FDF0` | translated | high | `80a8f590_FUN_80a8f590.c` |
| `0x80A8FBC0` | `fn_80a8fbc0` | Second sound-`0x1AC` request | translated | high | `80a8fbc0_FUN_80a8fbc0.c` |
| `0x80A8FCD0` | `fn_80a8fcd0` | `0x780`-byte jellyfish constructor, vtable `0x80B500F0` | translated | high | `80a8fcd0_FUN_80a8fcd0.c` |
| `0x80A90030` | `fn_80a90030` | Participant window `224.0` plus `1.5 * vy` vertical test | translated | high | `80a90030_FUN_80a90030.c` |
| `0x80A90110` | `fn_80a90110` | Aim step `+/-640` toward `+/-10012` with clamp | translated | high | `80a90110_FUN_80a90110.c` |
| `0x80A901B0` | `fn_80a901b0` | Wait entry: timer `0x78`, `fire_wait_up` at `8.0` | translated | high | `80a901b0_FUN_80a901b0.c` |
| `0x80A90250` | `fn_80a90250` | First attack step and countdown switch | translated | high | `80a90250_FUN_80a90250.c` |
| `0x80A90330` | `fn_80a90330` | Attack selector: up/down clip and side store | translated | high | `80a90330_FUN_80a90330.c` |
| `0x80A909C0` | `fn_80a909c0` | `daEnJimenFPakkun_c` state callback | translated | high | `80a909c0_FUN_80a909c0.c` |
| `0x80A90B40` | `fn_80a90b40` | `0x778`-byte constructor, vtable `0x80B504F8` | translated | high | `80a90b40_FUN_80a90b40.c` |
| `0x80A90C20` | `fn_80a90c20` | Three-layer fire effect from the `0x80B2E9C0` table | translated | medium | `80a90c20_FUN_80a90c20.c` |
| `0x80A91280` | `fn_80a91280` | Second state callback copy | translated | high | `80a91280_FUN_80a91280.c` |
| `0x80A912E0` | `fn_80a912e0` | Third state callback copy | translated | high | `80a912e0_FUN_80a912e0.c` |
| `0x80A91340` | `fn_80a91340` | Constant producer, returns `0x803534AC` | translated | high | `80a91340_FUN_80a91340.c` |
| `0x80A91350` | `fn_80a91350` | Constant producer, returns `0x8035346C` | translated | high | `80a91350_FUN_80a91350.c` |
| `0x80A91440` | `fn_80a91440` | Five `+0x30` parent tests, then recursion | translated | high | `80a91440_FUN_80a91440.c` |
| `0x80A91520` | `fn_80a91520` | Name-suffix compare after the last `:` | translated | high | `80a91520_FUN_80a91520.c` |
| `0x80A91640` | `fn_80a91640` | `0x728`-byte cloud constructor, vtable `0x80B50938` | translated | high | `80a91640_FUN_80a91640.c` |
| `0x80A91B40` | `fn_80a91b40` | Flag-gated child visibility word at `+0x500` | translated | high | `80a91b40_FUN_80a91b40.c` |
| `0x80A91EA0` | `fn_80a91ea0` | Forward event to `0x80A91DA0` with mode `0` | translated | medium | `80a91ea0_FUN_80a91ea0.c` |
| `0x80A91EC0` | `fn_80a91ec0` | Second copy of the same forwarder, mode `1` | translated | medium | `80a91ec0_FUN_80a91ec0.c` |
| `0x80A92050` | `fn_80a92050` | Configures/links `+0x148`, clears idle speeds, switches state | translated | high | `80a92050_FUN_80a92050.c` |
| `0x80A920D0` | `fn_80a920d0` | `jugem` model and `cloud_wait` clip binder | translated | high | `80a920d0_FUN_80a920d0.c` |

## What the handlers do

### The jellyfish constructors (6 bodies)

`0x80A8F590` and `0x80A8FCD0` build two more `0x780`-byte actors with the
wave-27 `kurage` layout: base vtable `0x8030448C`, model holder `+0x524`,
animation holder `+0x584` (`0x80B330E0`), helper vtable `+0x5BC`
(`0x803295D8` then `0x80329518`), small holder `+0x5C8`, two `0x8008C200`
sub-objects `+0x5E8`/`+0x68C`, and the class vtable last. `0x80A8F590` also
writes `u16 3` at `+0x778`; `0x80A8FCD0` leaves it at zero.

`0x80A8F2F0` unlinks the base object at `+0x148` and both `0x8008C200`
sub-objects, so it is the paired teardown for those two constructors.
`0x80A8F470`/`0x80A8FBC0` submit sound id `0x1AC` twice, and `0x80A8F330`
refreshes the model matrix and then computes the turn deltas.

### The `daEnJimenFPakkun_c` wait/attack machine (6 bodies)

`0x80A901B0` is the wait entry: it stores the facing byte, arms `+0x770` with
`0x78`, plays `fire_wait_up` at an `8.0` blend and switches to the `Wai...`
descriptor. `0x80A90250` is the first attack step; it releases the `+0x544`
holder, runs the base updates in the down direction, steps the aim, and
switches to the first `Atta...` descriptor when the countdown expires.
`0x80A90330` is the selector: while the countdown is positive it decrements
it, queries a target, clamps the aim, stores the side at `+0x774`, binds
`fire_attack_down` or `fire_attack_up` and switches to the second `Atta...`
descriptor.

The shared pieces are `0x80A90030` (target window) and `0x80A90110` (aim
step). The callback `0x80A909C0` is the one registered by the state table.

### Infrastructure and the cloud actor (14 bodies)

`0x80A91340`/`0x80A91350` are literal producers. `0x80A91440` and
`0x80A91520` are the usual chain walker and name predicate. `0x80A91280` and
`0x80A912E0` repeat the `0x8015F800`/`0x802B9280` callback.

`0x80A91640` constructs the `0x728`-byte `Wm_en_movecloud` actor, `0x80A91B40`
toggles the visibility word of the child resolved from `+0x6E8`, `0x80A92050`
resets the state node and `0x80A920D0` binds `g3d/jugem.brres`, the `jugem`
model and the `cloud_wait` clip. `0x80A91EA0`/`0x80A91EC0` forward two event
modes to the untranslated `0x80A91DA0`.

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A91BB0` | 352 B | The three output words are separate Ghidra locals (`local_2c`, `local_1c`, `local_c`) whose stack matrix layout is not recoverable. |
| `0x80A91D10` | 32 B | Export is `halt_baddata`; no instruction flow. |
| `0x80A91D30` | 56 B | Jump table at `0x80A91D64` was not recovered ("too many branches"). |
| `0x80A91DA0` | 252 B | Its body calls `0x80A91D10`, so the state transition cannot be completed. |
| `0x80A91D70` | 36 B | Untyped global `f64` subtraction with no named fields; low evidence. |
| `0x80A91F40` | 260 B | Actor receiver comes from the `0x802DCD70` prologue fragment; the frame is not recoverable (same class of artefact as the wave-26/27 notes). |

### Next cluster

| Address | Size | Note |
|---|---:|---|
| `0x80A921F0` | 204 B | First export after this run; an `f32` side/position comparison near the cloud block. |
| `0x80A920D0..0x80A933E0` | 4.8 KB | The remaining cloud handlers and effects. |

## Unknowns preserved in code

1. **Class labels.** The two `0x780` constructors are labelled from the shared
   `kurage` layout only; `daEnKurage_c` is never printed by the export. The
   `0x728` actor is labelled from the adjacent `Wm_en_movecloud` string.
2. **`0x80078470`.** `main.dol` has no export at this address, so the
   three-layer effect meaning in `fn_80a90c20` is named from the six call sites
   and the record table, not from a callee body.
3. **Dropped query argument.** `0x80A90030` is declared with one parameter in
   the export, but its caller at `0x80A90330` passes a stack block. The second
   parameter is `// inferred:` and forwarded to `0x80064740`.
4. **`0x80A91DA0`.** Still untranslated; [`fn_80a91ea0`] and
   [`fn_80a91ec0`] call it by address with `// unknown:` markers.
5. **`0x80B74038` descriptor.** The class table that writes it is outside this
   run, so the state name is `// unknown:`.
6. **Base-constructor receiver.** Every constructor still calls `0x80094E80`
   without the printed receiver; the actor in `r3` is `// inferred:` exactly as
   in waves 1-27.
7. **`0x80B2EA80` descriptor.** The ten-word block is copied verbatim by
   `0x8008C3E0`; the meaning of its `12.0` pair and the trailing bytes is
   `// unknown:`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`cargo check` succeeds. The remaining warnings are in other lanes' files
(`src/runtime/bt.rs` and `src/runtime/input.rs`) and were not touched here.
`node tools/doc-lint.mjs src` reports 0 violations across 68 files.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave28-enemies.csv`.
