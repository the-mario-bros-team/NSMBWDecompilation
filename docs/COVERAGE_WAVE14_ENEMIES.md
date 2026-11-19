# Coverage - wave 14 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the fourteenth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   segmented-enemy block (`0x80A6E930..0x80A713B0`) that starts inside the
   `daEnGuru_c` actor translated in wave 13 and continues through the two
   larger chain actors' constructors and combat callbacks.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   entries inside the selected range, and why each was left out.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = the body is represented but a hidden ABI operand or
  unresolved callee prevents a complete model; `stub` = address only.
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

| Metric | Before wave 14 | Wave 14 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 272 | 30 | 302 |
| Of those, `translated` | 267 | 26 | 293 |
| Of those, `partial` | 5 | 4 | 9 |
| Of those, `stub` | 0 | 0 | 0 |
| Constructors | - | 2 | - |
| Collision / contact callbacks | - | 3 | - |
| Animation, timer and render helpers | - | 10 | - |
| Probes, aim and target selection | - | 4 | - |
| State callbacks, thunks and structural helpers | - | 11 | - |
| Selected bytes | - | 4,104 | - |

The 30 new bodies cover 4,104 bytes (0.69 % of the 591,596 bytes that the gap
report listed as untranslated before this pass) and 30 of the 3,283 remaining
function entries (0.91 %). Confidence is **21 high** and **9 medium**. The four
48-byte forwarders are `partial`: their visible body is complete, but the
hidden `r12` operand of `0x802DCBFC` cannot be modelled by `platform::call3`.
There are no `stub` rows.

## Selection

This pass was evidence-first and continuation-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Took the block that follows wave 13.** Wave 13 ended at `0x80A6E680`
   (`guru` model binder). The next text entries, `0x80A6E930` and
   `0x80A6E9C0`, use the same `+0x540`/`+0x544`/`+0x584` holders and the
   `rotate` clip name, so they close the `guru` machine before the next
   constructor begins.
3. **Identified the two larger actors from their constructors.** `0x80A6EE90`
   builds a `0x868`-byte object with class vtable `0x80B4ABA0`; `0x80A70500`
   builds a `0x988`-byte object with class vtable `0x80B4B000`. Both share the
   chain layout: a count at `+0x540`, the first segment inline at `+0x544`, a
   `+0x238` next-segment link, and a `+0x584` rotate object.
4. **Confirmed the shared segment layout from the builder.** `0x80A707E0`
   (deferred) allocates `0x278`-byte segments, writes the same
   `0x803158D8`/`0x8031588C` vtables and the same `+0x5C4..+0x5D4` position
   block that `0x80A70500` installs on the root segment.
5. **Cross-checked the data constants.** The `f32`/`f64`/`i16` values
   referenced by the handlers were located in `work/rels/d_enemiesNP.rel`
   (address minus base equals the file offset for this module); the Rust keeps
   the original data addresses instead of baking the values in.
6. **Excluded every export with damaged control flow.** Six entries in the
   range print `Removing unreachable block` plus GQR/`ldexpf` artefacts, and
   one (`0x80A6EFB0`) ends in `halt_baddata`. They are listed under *Deferred
   neighbours* and are still called by address where a translated body needs
   them.

### The segmented-enemy block (`0x80A6E930..0x80A713B0`, 30 functions)

`0x80A6E930` starts the `rotate` clip on the `guru` actor and `0x80A6E9C0`
advances it. `0x80A6EE90` and `0x80A70500` are the two facts of the family:
one `0x868`-byte and one `0x988`-byte chain root. The combat half is
`0x80A6F450` (latch the contacted actor's variant and enter `0x80358004`),
`0x80A70FE0` (forward ground probe), `0x80A71290` (land on top of another
actor) and `0x80A713B0` (bounce a body off this actor). `0x80A70300` and
`0x80A703E0` are the two structural helpers: a five-level chain probe and a
`strrchr`/`strcmp` name comparison. `0x80A70A70`/`0x80A70B00`/`0x80A70BD0`
are the per-segment render, container and teardown steps.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A6E930` | `fn_80a6e930` | Bind the `rotate` clip and start its two blends | translated | high | `80a6e930_FUN_80a6e930.c` |
| `0x80A6E9C0` | `fn_80a6e9c0` | Advance the rotate animation and the `+0x104` angle | translated | high | `80a6e9c0_FUN_80a6e9c0.c` |
| `0x80A6ED10` | `fn_80a6ed10` | Release callback: `0x8015F800`, optional `0x802B9280` | translated | high | `80a6ed10_FUN_80a6ed10.c` |
| `0x80A6EE30` | `fn_80a6ee30` | Second `0x802DCBFC` destructor forwarder | partial | medium | `80a6ee30_FUN_80a6ee30.c` |
| `0x80A6EE60` | `fn_80a6ee60` | Third `0x802DCBFC` destructor forwarder | partial | medium | `80a6ee60_FUN_80a6ee60.c` |
| `0x80A6EE90` | `fn_80a6ee90` | Allocate/construct the `0x868`-byte chain actor | translated | high | `80a6ee90_FUN_80a6ee90.c` |
| `0x80A6F2A0` | `fn_80a6f2a0` | Heading from `+0x5EC` toward `+0x61C`, biased by half a turn | translated | medium | `80a6f2a0_FUN_80a6f2a0.c` |
| `0x80A6F3B0` | `fn_80a6f3b0` | Resolve the `+0x718` id and return it only when free | translated | medium | `80a6f3b0_FUN_80a6f3b0.c` |
| `0x80A6F450` | `fn_80a6f450` | Contact: latch the variant and enter `0x80358004` | translated | high | `80a6f450_FUN_80a6f450.c` |
| `0x80A6F720` | `fn_80a6f720` | Emit effect `0x73` from the `+0x5EC` transform | translated | high | `80a6f720_FUN_80a6f720.c` |
| `0x80A6F770` | `fn_80a6f770` | Play `throw` and apply the variant blend rate | translated | high | `80a6f770_FUN_80a6f770.c` |
| `0x80A6F7F0` | `fn_80a6f7f0` | Choose the `+0x6DC` timer (15/30/45) from the variant | translated | high | `80a6f7f0_FUN_80a6f7f0.c` |
| `0x80A6F8F0` | `fn_80a6f8f0` | Play `jump_st` and clear the three speed fields | translated | high | `80a6f8f0_FUN_80a6f8f0.c` |
| `0x80A70140` | `fn_80a70140` | Release callback (second copy) | translated | high | `80a70140_FUN_80a70140.c` |
| `0x80A701A0` | `fn_80a701a0` | Release callback (third copy) | translated | high | `80a701a0_FUN_80a701a0.c` |
| `0x80A70200` | `fn_80a70200` | Return the constant `0x80352CF4` | translated | high | `80a70200_FUN_80a70200.c` |
| `0x80A70210` | `fn_80a70210` | Return the constant `0x80352C34` | translated | high | `80a70210_FUN_80a70210.c` |
| `0x80A70300` | `fn_80a70300` | Five-level chain probe with a recursive tail | translated | high | `80a70300_FUN_80a70300.c` |
| `0x80A703E0` | `fn_80a703e0` | Compare the object's short class name with a caller name | translated | high | `80a703e0_FUN_80a703e0.c` |
| `0x80A704A0` | `fn_80a704a0` | Fourth `0x802DCBFC` destructor forwarder | partial | medium | `80a704a0_FUN_80a704a0.c` |
| `0x80A704D0` | `fn_80a704d0` | Fifth `0x802DCBFC` destructor forwarder | partial | medium | `80a704d0_FUN_80a704d0.c` |
| `0x80A70500` | `fn_80a70500` | Allocate/construct the `0x988`-byte chain actor | translated | high | `80a70500_FUN_80a70500.c` |
| `0x80A70770` | `fn_80a70770` | Rebuild the render list and commit the `+0x524` holder | translated | high | `80a70770_FUN_80a70770.c` |
| `0x80A70A70` | `fn_80a70a70` | Run the per-segment `+0x0C`/`+0x14` model update | translated | high | `80a70a70_FUN_80a70a70.c` |
| `0x80A70B00` | `fn_80a70b00` | Allocate the `(count+1)`-slot segment container | translated | high | `80a70b00_FUN_80a70b00.c` |
| `0x80A70BD0` | `fn_80a70bd0` | Mode-2 chain teardown, then base teardown `0x80095480` | translated | high | `80a70bd0_FUN_80a70bd0.c` |
| `0x80A70EB0` | `fn_80a70eb0` | Step the `+0x102` aim angle and clamp it to the table target | translated | high | `80a70eb0_FUN_80a70eb0.c` |
| `0x80A70FE0` | `fn_80a70fe0` | Forward ground probe; reject steps over a drop | translated | medium | `80a70fe0_FUN_80a70fe0.c` |
| `0x80A71290` | `fn_80a71290` | Land on the other actor, else run the base response | translated | medium | `80a71290_FUN_80a71290.c` |
| `0x80A713B0` | `fn_80a713b0` | Bounce one body off this actor, once per body index | translated | medium | `80a713b0_FUN_80a713b0.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A6EA20` | 212 B | Removed unreachable blocks; GQR/`ldexpf` artefacts |
| `0x80A6EFB0` | 100 B | `halt_baddata()` truncates the middle branch |
| `0x80A6F8C0` | 48 B | `f64` magic-number subtraction; the intended operand cannot be recovered |
| `0x80A6FBC0` | 624 B | Removed unreachable block; GQR/`ldexpf` artefacts |
| `0x80A707E0` | 652 B | GQR/`ldexpf` artefacts in all three incoming vectors |
| `0x80A70E20` | 140 B | Removed unreachable block; GQR artefacts |
| `0x80A70F40` | 148 B | Removed unreachable blocks; GQR artefacts |
| `0x80A715E0` | 280 B | Removed unreachable block; GQR artefacts |
| `0x80A71700` | 280 B | Removed unreachable block; GQR artefacts |
| `0x80A71AA0` | 444 B | Removed unreachable block; GQR artefacts |
| `0x80A71C60` | 240 B | GQR artefacts |
| `0x80A71F40` | 520 B | Removed unreachable block; GQR artefacts |

The clean entries after `0x80A713B0` (`0x80A71820`, `0x80A71910`,
`0x80A719A0`, `0x80A71D60`, `0x80A72150`, `0x80A721F0`, `0x80A723F0`,
`0x80A72500`) are the continuation of the same two chain classes and are the
best next targets. `0x80A729E0` begins the next actor cluster.

`0x80A707E0` is still called by address from `fn_80a70770` and
`0x80A6EA20` from `fn_80a6e9c0`; both call sites carry an `// unknown:` note.
No behaviour is invented for them.

## Unknowns preserved in code

1. **Class names for the two chain actors.** The constructors sit between the
   `guru` block and the `daEnHanachan_c` table, but no `StateID` string names
   either vtable (`0x80B4ABA0`, `0x80B4B000`). They keep `fn_<address>` names
   and `// inferred:` labels.
2. **Segment semantics.** `+0x540` is the extra-segment count and `+0x238` the
   next-segment link, both observed. Whether a segment is a body part, a lift
   platform or an attachment is not provable from the export.
3. **`+0x504` body-index table.** `fn_80a713b0` indexes an `i16` array by the
   contacted actor's class-name byte. The array is observed; its role as a
   per-body bounce cooldown is `// inferred:`.
4. **Four 48-byte forwarders.** `0x80A6EE30`, `0x80A6EE60`, `0x80A704A0` and
   `0x80A704D0` (the module's other copy is the wave-13 `0x80A6C810`) call
   `0x802DCBFC`, which dispatches through a hidden `r12` this-adjust pointer.
   The Rust keeps the visible three arguments and marks the omission.
5. **`0x80352CF4` / `0x80352C34`.** The two twelve-byte functions return these
   words unchanged; no caller in this range explains them.
6. **Dropped receivers.** Ghidra prints `0x80162D00` with no argument in
   `fn_80a6f3b0` and `0x802DCBFC`-style calls with no `this`; both receive the
   only object in scope and the Rust passes it explicitly, marking the
   inference.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. `cargo check` reports no warning or error in
`src/game/enemies.rs`. This lane modified only `src/game/enemies.rs`, this
page and `ledger/agents/wave14-enemies.csv`.
