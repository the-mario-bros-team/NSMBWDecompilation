# Coverage - wave 7 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the seventh translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 28 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains why the two densest
   remaining handler runs were chosen: the `daEnPata_c` per-frame block and the
   coin-family block between the `daEnCoin_c` and `daEnCoinFloor_c` tables.
3. **What is still missing?** - *Remaining work* lists the deferred
   neighbours, including the GQR-mangled functions that were not translated.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  accounted for; `partial` = entry/exit and major effects only; `stub` =
  address only.
- **Confidence** - strength of the *evidence*: `high` = bytes/flow are
  explicit and cross-checked; `medium` = flow is explicit but a role, ABI
  detail or class ownership is inferred; `low` = mostly guesswork.
- **Evidence** - file under
  `work/ghidra/rel-d_enemiesNP/export/`; that C file is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`. The REL has no
embedded load address; the retail loader may place it anywhere. These are not
runtime addresses.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Range inventory

| Metric | Before wave 7 | Wave 7 | Cumulative |
|---|---:|---:|---:|
| Public translation functions in `game/enemies.rs` | 104 | 28 | 132 |
| `StateID`-bearing state tables translated | 43 | 0 | 43 |
| Handler / helper / constructor functions | 104 | 28 | 132 |
| Deferred state tables | 0 | 0 | 0 |

All 43 `StateID` tables were already covered before this pass. Wave 7 adds no
table: it targets the per-frame code that the tables dispatch to but do not
name.

## Translated functions

### `daEnPata_c` per-frame block (`0x80AE41C0..0x80AE5110`)

The state table at `0x80AE3080` registers `UDfly`, `LRfly`, `PataJ` and
`PataJTurn` with the shared callback `0x80AE33C0`. The handlers below sit in
the same text block and share the model holders `+0x62C`, `+0x658`, the class
vtable `+0x60` and the state-switch slot `+0xD4`.

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80AE41C0` | `fn_80ae41c0` | Hurt/death latch, effect `0x2EF`, retire | translated | high | `80ae41c0_FUN_80ae41c0.c` |
| `0x80AE4300` | `fn_80ae4300` | Yaw-ramp add/subtract with clamps | translated | high | `80ae4300_FUN_80ae4300.c` |
| `0x80AE4380` | `fn_80ae4380` | Oscillate yaw `+0x100` by ramp `+0x6C2` | translated | high | `80ae4380_FUN_80ae4380.c` |
| `0x80AE4410` | `fn_80ae4410` | Turn toward a world-space x/z vector | translated | high | `80ae4410_FUN_80ae4410.c` |
| `0x80AE44C0` | `fn_80ae44c0` | Advance animation holders `+0x588`/`+0x600` | translated | medium | `80ae44c0_FUN_80ae44c0.c` |
| `0x80AE4680` | `fn_80ae4680` | Initialise fall: position, speeds, edge flag | translated | high | `80ae4680_FUN_80ae4680.c` |
| `0x80AE4790` | `fn_80ae4790` | Falling update and floor switch | translated | high | `80ae4790_FUN_80ae4790.c` |
| `0x80AE48E0` | `fn_80ae48e0` | Settle yaw/pitch and switch when both stop | translated | high | `80ae48e0_FUN_80ae48e0.c` |
| `0x80AE4D20` | `fn_80ae4d20` | Bounce wait: clip, rotate, ground line | translated | medium | `80ae4d20_FUN_80ae4d20.c` |
| `0x80AE4E20` | `fn_80ae4e20` | Bounce steer with edge-sign rotation | translated | medium | `80ae4e20_FUN_80ae4e20.c` |
| `0x80AE4F10` | `fn_80ae4f10` | Contact knockback vector and fall restart | translated | medium | `80ae4f10_FUN_80ae4f10.c` |
| `0x80AE5020` | `fn_80ae5020` | Main update: clip, rotation, hit test, tail | translated | medium | `80ae5020_FUN_80ae5020.c` |
| `0x80AE5110` | `fn_80ae5110` | Destructor for the Pata-family actor | translated | high | `80ae5110_FUN_80ae5110.c` |

### Coin family (`0x80A41DC0..0x80A43F40`)

The `daEnCoin_c` table at `0x80A41180` registers sixteen states with the
callback `0x80A41DC0` (fourteen) or `0x80A41E20` (`EatOut`, `DieGoal`). The
handlers and helpers below use the model blocks `+0x534`, `+0x698` and
`+0x8D0` installed by `alloc_coin_actor` (`0x80A42180`). The run ends just
before the `daEnCoinFloor_c` table at `0x80A43C00`.

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A41DC0` | `fn_80a41dc0` | Common Coin state callback (`0x8015F800`, optional `0x802B9280`) | translated | high | `80a41dc0_FUN_80a41dc0.c` |
| `0x80A41F80` | `fn_80a41f80` | Walk five `+0x30` owner links, then recurse once | translated | high | `80a41f80_FUN_80a41f80.c` |
| `0x80A42060` | `fn_80a42060` | Compare actor-name suffix after the last `:` | translated | medium | `80a42060_FUN_80a42060.c` |
| `0x80A42300` | `fn_80a42300` | Spawn/placement: grid probe, model transfer, target search | translated | high | `80a42300_FUN_80a42300.c` |
| `0x80A426F0` | `fn_80a426f0` | Place model from position and animation state | translated | medium | `80a426f0_FUN_80a426f0.c` |
| `0x80A42810` | `fn_80a42810` | Find player actor by class `0x1A3`/`0x1A4` and id byte | translated | high | `80a42810_FUN_80a42810.c` |
| `0x80A428B0` | `fn_80a428b0` | Offset, distance and yaw to a target | translated | medium | `80a428b0_FUN_80a428b0.c` |
| `0x80A42BC0` | `fn_80a42bc0` | Retarget and switch to the tracked state | translated | medium | `80a42bc0_FUN_80a42bc0.c` |
| `0x80A42C20` | `fn_80a42c20` | Initialise launch: model, speeds, random facing | translated | medium | `80a42c20_FUN_80a42c20.c` |
| `0x80A42CC0` | `fn_80a42cc0` | Wait/landing update and yaw align | translated | medium | `80a42cc0_FUN_80a42cc0.c` |
| `0x80A432D0` | `fn_80a432d0` | Construct a `0x918`-byte coin-family actor | translated | medium | `80a432d0_FUN_80a432d0.c` |
| `0x80A43910` | `fn_80a43910` | Commit model position and yaw at `+0x8D0` | translated | high | `80a43910_FUN_80a43910.c` |
| `0x80A439A0` | `fn_80a439a0` | Find/validate a target and switch state | translated | medium | `80a439a0_FUN_80a439a0.c` |
| `0x80A43A40` | `fn_80a43a40` | Per-frame model and grid update | translated | high | `80a43a40_FUN_80a43a40.c` |
| `0x80A43F40` | `fn_80a43f40` | Construct a `0xA00`-byte coin-family actor | translated | medium | `80a43f40_FUN_80a43f40.c` |

## Selection

### Why the Pata block (13 functions)

The `daEnPata_c` state table names four flight/attack states but none of its
callbacks. The 13 functions form a complete state machine:

- [`fn_80ae4680`], [`fn_80ae4790`] and [`fn_80ae48e0`] are the fall path:
  initial position/speeds, descent and floor switch, then the settle test.
- [`fn_80ae4d20`], [`fn_80ae4e20`] and [`fn_80ae5020`] are the bounce and main
  updates, all sharing the clip predicate `0x801662E0 != 0x80B30EA0`.
- [`fn_80ae41c0`] is the shared hurt/death reaction, and [`fn_80ae4f10`] is
  the contact response.
- [`fn_80ae4300`]/[`fn_80ae4380`]/[`fn_80ae4410`] steer the yaw; the two
  animation advances and the destructor close the object.

Every branch, store, constant and callee is visible in the export. The only
inferences are which `StateID` owns each handler, which the table does not
encode, and the floating-point call ABI described under *Unknowns*.

### Why the coin block (15 functions)

The Coin table names sixteen states and binds two callbacks. This pass takes
the callback pair's entry point, the target search, the placement path and the
model helpers that every Coin state shares:

- [`fn_80a42300`] is the placement state machine; it calls
  [`fn_80a42810`]/[`fn_80a428b0`] and [`fn_80a426f0`].
- [`fn_80a41f80`] and [`fn_80a42060`] are identity/owner helpers used by the
  coin block.
- [`fn_80a42c20`]/[`fn_80a42cc0`] run the launch and landing phases.
- [`fn_80a43910`]/[`fn_80a439a0`]/[`fn_80a43a40`] are the model commit,
  target validation and per-frame grid update.
- [`fn_80a432d0`]/[`fn_80a43f40`] are the two remaining coin-family
  constructors in this run.

`0x80A429C0`, `0x80A436C0` and `0x80A43810` are deliberately left as address
callees: they contain paired-single (GQR) quantisation noise in the export and
were not selected for a faithful rewrite in this pass.

## Remaining work

### Deferred handlers in the same blocks

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80AE3ED0` | 432 B | GQR/paired-single mangling in the export |
| `0x80AE4080` | 320 B | GQR/paired-single mangling in the export |
| `0x80AE4530` | 172 B | depends on the two mangled helpers above |
| `0x80AE4600` | 124 B | depends on `0x80AE4080` |
| `0x80A429C0` | 476 B | GQR/paired-single mangling in the export |
| `0x80A436C0` | 328 B | GQR/paired-single mangling in the export |
| `0x80A43810` | 252 B | GQR/paired-single mangling in the export |

These remain address-based calls inside the translated bodies; none is
presented as a symbol.

### Unknowns preserved in code

1. **Handler ownership.** The `StateID` that dispatches each Pata handler is
   not recoverable from the table, because the table stores vtable methods.
   All names stay `fn_<address>`, and the docs say so.
2. **Constructor class labels.** `fn_80a432d0` installs class vtable
   `0x80B40034`; `fn_80a43f40` installs `0x80B40708`, which sits immediately
   after the `daEnCoinFloor_c` state vtable `0x80B406BC`. The profile registry
   in `d_profileNP` remains the final authority, so the names stay
   address-based.
3. **Floating-point ABI.** Ghidra prints floating-point parameters and returns
   as `double`. The new local adapters (`call2_f64_f64`, `call4_f64x3`,
   `call6_f64x4`, `call4_f64`, `call1_f64_ret_f32`, `call2_ret_f32`) keep the
   values in the floating-point register class; the exact callee signatures
   are inferred from the call sites.
4. **`0x802E1C40` / `0x802E1AB4`.** Used as `strchr` and string compare by
   [`fn_80a42060`]; the roles are inferred from the `':'` argument and the
   boolean result.
5. **Grid probing.** The `0x80077810`/`0x800777B0` pair is called with the
   quantised x and `-z` cell coordinates; the quantisation masks are explicit,
   the grid object is opaque.
6. **Prologue/epilogue pseudo-calls.** The constructors still show
   `0x802DCD74`/`0x802DCDC0` register save/restore fragments. They are omitted
   as in every earlier wave, with a note in the Rust docs.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` succeeds and
`node tools/doc-lint.mjs src` reports 0 violations across 68 files. This lane
modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave7-enemies.csv`.
