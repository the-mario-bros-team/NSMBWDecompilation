# Coverage - wave 20 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twentieth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found:
   the two FireSnake callbacks named by the translated table, the two
   constant words used by the Gabon death descriptors, and the untouched
   `gabon`/`gabon_rock` handler block between them.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   entries that touch this range and why each was left out.

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

| Metric | Before wave 20 | Wave 20 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 418 | 30 | 448 |
| Of those, `translated` | 403 | 30 | 433 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| `daEnFireSnake_c` callbacks | - | 2 | - |
| `daEnGabon_c` callbacks | - | 2 | - |
| Chain walkers | - | 2 | - |
| `gabon`/`gabon_rock` constructors and binders | - | 4 | - |
| Frame/state helpers | - | 18 | - |
| Constant producers | - | 2 | - |
| Selected bytes | - | 5,268 | - |

The 30 new bodies cover 5,268 bytes (0.93 % of the 572,464-byte denominator
used by the wave-19 page; 568,176 bytes were still untranslated before this
pass, and 5,268 / 568,176 = 0.93 %) and 30 of the 3,137 remaining function
entries (0.96 %). Confidence is **21 high** and **9 medium**. There are no
`stub` rows and no new `partial` rows.

Cumulative after this pass: 448 bodies, 433 `translated`, 15 `partial`, 3,107
function entries and 562,908 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added. `0x80A5F450` and `0x80A5F4B0` appeared in the
   file only as text references inside the wave-5 `register_gabon_states`
   doc comment, never as bodies.
2. **Started from the translated FireSnake table.** The table at `0x80A5A350`
   has fourteen `daEnFireSnake_c::StateID_*` records. Twelve use callback
   `0x80A5AE30` and `DieFall` uses `0x80A5AE90`; both callbacks were
   untranslated. The adjacent `0x80A5AFE0` is the five-link chain walker that
   sits in the same block. This gives a tight, fully evidenced group.
3. **Finished the `gabon`/`gabon_rock` gap.** The range
   `0x80A5C0C0..0x80A5EE80` lies between the translated FireSnake helpers and
   the translated `daEnGabon_c` table at `0x80A5EF70`. It contains two
   constructors: `0x80A5D330` allocates `0x668` bytes and installs vtable
   `0x80B478D0`, and its binder `0x80A5D590` loads the `gabon_rock` model.
   The previously translated `0x720`-byte constructor at `0x80A5F810` binds
   the `gabon` model, and `0x80A5C0C0` indexes the `fruit` tables by `+0x678`,
   a field that only exists inside the `0x720` layout.
4. **Used the state descriptors as the class link.** The three handlers
   `0x80A5E3E0`, `0x80A5E6A0` and `0x80A5E8C0` write the Gabon `Attack`
   descriptor `0x80B71BE0` through class-vtable slot `+0xD4`, and
   `0x80A5DAF0` queries the `Move` descriptor `0x80B71B60`. The object at
   `0x668` bytes therefore runs the `daEnGabon_c` state records even though
   its own class vtable is different.
5. **Included the two constant producers.** `0x80A5F510` and `0x80A5F520`
   are twelve-byte `li`/`blr` bodies returning `0x80357BF4` and `0x80357BB4`.
   The `daEnGabon_c` table calls them to build the tenth word of the two death
   descriptors, so their role is explicit.
6. **Excluded every export with damaged control flow.** The unreachable block
   in `0x80A5C660`, the `halt_baddata()` export at `0x80A5D9D0`, and the
   GQR/`ldexpf` artefacts in `0x80A5DBC0`, `0x80A5E1A0` and `0x80A5E590` were
   not guessed. `0x80A5DBC0` and `0x80A5E1A0` are still called by address from
   the new bodies. `0x80A5EB00` was left out because it calls the
   `halt_baddata()` export, so its post-call flow cannot be verified.
7. **Verified the two dropped ABI operands against the machine code.** The
   export drops the `f64` operand of three `0x801690E0` calls; the instruction
   stream loads `lfs f1,0x4C(r31)` where `r31 = 0x80B2D150`, i.e. the `0.0` at
   `0x80B2D19C`. The export also drops the second argument of `0x8023A200` in
   `0x80A5E230`; `r4` is still the incoming `param_2` at that call. Both are
   documented as `// inferred:` in the Rust.
8. **Read the data constants from the REL image.** Address minus base equals
   the file offset for this module, so every `f32`, `f64` and `i16` value in
   the new section was checked directly. The Rust keeps the original data
   addresses instead of baking the values in.

### The selected blocks (`0x80A5AE30..0x80A5F6F0`, 30 functions)

The FireSnake group is two callbacks and one chain walker. The `gabon` group
is the `fruit` binder and two animation helpers, then the `0x668` constructor,
its `gabon_rock` binder, the transform/clip/branch helpers and the four
per-frame handlers. The tail is the Gabon callbacks, the two constant words,
the second chain walker and the name predicate.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A5AE30` | `fn_80a5ae30` | FireSnake registration callback (twelve normal states) | translated | high | `80a5ae30_FUN_80a5ae30.c` |
| `0x80A5AE90` | `fn_80a5ae90` | FireSnake `DieFall` registration callback | translated | high | `80a5ae90_FUN_80a5ae90.c` |
| `0x80A5AFE0` | `fn_80a5afe0` | Walk five `+0x30` child links while vtable `+0x0C` returns zero | translated | medium | `80a5afe0_FUN_80a5afe0.c` |
| `0x80A5C0C0` | `fn_80a5c0c0` | Load the `fruit` model and bind its three clips | translated | high | `80a5c0c0_FUN_80a5c0c0.c` |
| `0x80A5C210` | `fn_80a5c210` | Seed the animation frame from the global counter and a 16-bit random | translated | medium | `80a5c210_FUN_80a5c210.c` |
| `0x80A5C4B0` | `fn_80a5c4b0` | Advance the `+0x674`/`+0x67C` blink timer and re-seed the holder | translated | high | `80a5c4b0_FUN_80a5c4b0.c` |
| `0x80A5D1B0` | `fn_80a5d1b0` | `0x668` object registration callback | translated | medium | `80a5d1b0_FUN_80a5d1b0.c` |
| `0x80A5D330` | `alloc_gabon_rock_actor` | Allocate and construct the `0x668`-byte `gabon_rock` object | translated | high | `80a5d330_FUN_80a5d330.c` |
| `0x80A5D590` | `fn_80a5d590` | Load `gabon_rock` and bind its body/wait clips | translated | high | `80a5d590_FUN_80a5d590.c` |
| `0x80A5D770` | `fn_80a5d770` | Rebuild the matrix from position/angles with two pitch offsets | translated | high | `80a5d770_FUN_80a5d770.c` |
| `0x80A5D8D0` | `fn_80a5d8d0` | Copy clip-node 0/6/3 translations into `+0x63C`/`+0x648`/`+0x654` | translated | high | `80a5d8d0_FUN_80a5d8d0.c` |
| `0x80A5D950` | `fn_80a5d950` | Apply the per-side turn step and clamp the middle angle | translated | high | `80a5d950_FUN_80a5d950.c` |
| `0x80A5DAF0` | `fn_80a5daf0` | Test the `Move` state against the per-side turn limit | translated | medium | `80a5daf0_FUN_80a5daf0.c` |
| `0x80A5DCA0` | `fn_80a5dca0` | Spawn effect `0x176` at the actor and link it back | translated | medium | `80a5dca0_FUN_80a5dca0.c` |
| `0x80A5DD20` | `fn_80a5dd20` | Write a variant-selected state to the target at `+0x630` | translated | medium | `80a5dd20_FUN_80a5dd20.c` |
| `0x80A5E230` | `fn_80a5e230` | Re-bind one named animation into the `+0x5C0` holder | translated | medium | `80a5e230_FUN_80a5e230.c` |
| `0x80A5E2C0` | `fn_80a5e2c0` | Spawn the landing smoke at a two-word point | translated | high | `80a5e2c0_FUN_80a5e2c0.c` |
| `0x80A5E360` | `fn_80a5e360` | Bind `wait` and clear the three speed words | translated | high | `80a5e360_FUN_80a5e360.c` |
| `0x80A5E3E0` | `fn_80a5e3e0` | Attack-window frame handler with landing smoke and x-window switch | translated | high | `80a5e3e0_FUN_80a5e3e0.c` |
| `0x80A5E6A0` | `fn_80a5e6a0` | Turn/window frame handler with `+0x5FC`/`+0x638` interlock | translated | high | `80a5e6a0_FUN_80a5e6a0.c` |
| `0x80A5E8C0` | `fn_80a5e8c0` | Landing frame handler with start/20.0 exit test | translated | high | `80a5e8c0_FUN_80a5e8c0.c` |
| `0x80A5EA90` | `fn_80a5ea90` | Bind `throw` and spawn the item | translated | high | `80a5ea90_FUN_80a5ea90.c` |
| `0x80A5ECC0` | `fn_80a5ecc0` | Bind `dead` and retire the object | translated | high | `80a5ecc0_FUN_80a5ecc0.c` |
| `0x80A5EE80` | `fn_80a5ee80` | Transform + clip-node frame helper | translated | high | `80a5ee80_FUN_80a5ee80.c` |
| `0x80A5F510` | `fn_80a5f510` | Constant producer `0x80357BF4` | translated | high | `80a5f510_FUN_80a5f510.c` |
| `0x80A5F520` | `fn_80a5f520` | Constant producer `0x80357BB4` | translated | high | `80a5f520_FUN_80a5f520.c` |
| `0x80A5F450` | `fn_80a5f450` | `daEnGabon_c` registration callback (four normal states) | translated | high | `80a5f450_FUN_80a5f450.c` |
| `0x80A5F4B0` | `fn_80a5f4b0` | `daEnGabon_c` death-state registration callback | translated | high | `80a5f4b0_FUN_80a5f4b0.c` |
| `0x80A5F610` | `fn_80a5f610` | Second five-link chain walker | translated | medium | `80a5f610_FUN_80a5f610.c` |
| `0x80A5F6F0` | `fn_80a5f6f0` | Compare the state-name suffix after the last `:` | translated | medium | `80a5f6f0_FUN_80a5f6f0.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A5B180` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand (wave-19 deferral) |
| `0x80A5B1B0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand (wave-19 deferral) |
| `0x80A5BB30` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand (wave-19 deferral) |
| `0x80A5BB60` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand (wave-19 deferral) |
| `0x80A5C660` | 436 B | Export removes an unreachable block at `0x80A5C6E4` |
| `0x80A5D2D0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A5D300` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |
| `0x80A5D9D0` | 68 B | Export is `halt_baddata()`; no verifiable body |
| `0x80A5DBC0` | 212 B | GQR/`ldexpf` artefacts in the angle delta; called by address |
| `0x80A5E1A0` | 136 B | GQR/`ldexpf` artefacts in the clip binder; called by address |
| `0x80A5E590` | 252 B | GQR/`ldexpf` artefacts in the phase setup |
| `0x80A5EB00` | 448 B | Calls `0x80A5D9D0`, whose export is `halt_baddata()` |

## Unknowns preserved in code

1. **Class name of the `0x668` object.** The constructor vtable `0x80B478D0`
   and the `gabon_rock` model are explicit, but no `StateID` string names the
   class. The Rust uses `alloc_gabon_rock_actor` and says so in the doc.
2. **`0x8023A200` second argument in `0x80A5E230`.** The export prints one
   argument, but the incoming `r4` still holds `param_2` at the call. The Rust
   passes `name` and marks it `// inferred:`.
3. **Dropped `f64` operands of `0x801690E0`.** Three call sites print no
   `double`. The instruction stream loads `0.0` from `0x80B2D19C` before each
   call, so the Rust passes it and marks the inference.
4. **`0x802DCC4C`.** The helper converts the `f32` period at `+0x5AC` to an
   unsigned integer; its name is outside this lane, so it stays by address.
5. **`0x8068B860` and effect id `0x176`.** The activation callback and the
   effect identity are not named in the export; both stay by address/id.
6. **State constants `0x8082ACB0`/`0x8082AC70`.** These are main-DOL state
   records written to the target at `+0x630`; their names are outside this
   lane.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` reports no warning or error in
`src/game/enemies.rs`, and `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave20-enemies.csv`.
