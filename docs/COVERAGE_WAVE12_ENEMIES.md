# Coverage - wave 12 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twelfth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 25 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the two runs were
   found: the `bakubaku`-model block (`0x80A03150..0x80A03B10`) and the
   `daEnBuku_c` per-state handler block (`0x80A03E70..0x80A05970`).
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   helpers that sit inside or beside the selected bodies.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` =
  address only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ABI detail or
  receiver is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_enemiesNP/export/`; that C
  file is the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80A00000`; the entry
point is `0x80A000F0`. The REL has no embedded load address, and the retail
loader may place it anywhere. These are **not** runtime addresses.
`main.dol` addresses (`0x8000xxxx`, `0x8032xxxx`, `0x8042xxxx`) are imported
absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 12 | Wave 12 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Translated functions in `src/game/enemies.rs` | 220 | 25 | 245 |
| Functions from the `bakubaku`-model block | 0 | 6 | - |
| Functions from the `daEnBuku_c` handler block | 0 | 19 | - |
| State callbacks added | 0 | 2 | - |
| Selected bytes | 0 | 3,320 | - |

The 25 new bodies cover 3,320 bytes (0.55 % of the 599,152 bytes that the gap
report listed as untranslated before this pass) and 25 of the 3,335 remaining
function entries (0.75 %). Confidence is **20 high** and **5 medium**; there
are no `partial` or `stub` rows.

## Selection

This pass was evidence-first and continuation-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 25 bodies were written before
   any ledger row was added.
2. **Took the `bakubaku`-model block as one run.** `0x80A03B10` is the anchor:
   it loads `g3d/bakubaku.brres` (`0x80B33674`) under the name `bakubaku`
   (`0x80B33688`), binds the `swim` clip (`0x80B33694`), and uses the same
   `+0x524`/`+0x540`/`+0x544`/`+0x584` holder layout that
   `0x80A03850`/`0x80A03880`/`0x80A038B0` manipulate. `0x80A03150` is the
   `fState` callback shape used elsewhere in the module and `0x80A032D0` is
   the small `0x528`-byte constructor immediately before the run.
3. **Took the `daEnBuku_c` handlers by state object.** The already-translated
   state table `0x80A05500` (`register_buku_states`) stores `0x80A05910` in
   the `Swim`, `SrcSwim`, `AttackSwim` and `AttackWait` records, and
   `0x80A05970` in `DieFall`. The handlers at `0x80A04EA0` and `0x80A05290`
   jump through the class vtable slot `+0xD4` to the `0x80B6E838` and
   `0x80B6E7F8` state objects, which names the states they belong to. The
   `0x1168`-byte constructor at `0x80A05CC0` already documented the
   `+0x6D0..+0x6E0` slot words used by the grab bookkeeping.
4. **Cross-checked the data constants.** The `f32` values referenced by the
   handlers were read from `work/rels/d_enemiesNP.rel` at
   `address - 0x80A00000` and are listed in the Rust constant block.

### `bakubaku`-model block (`0x80A03150..0x80A03B10`, 6 functions)

`0x80A03150` is the `fState` callback. `0x80A032D0` allocates a `0x528`-byte
actor and installs vtable `0x80B333E8`. `0x80A03850`/`0x80A03880` are the two
`return 1` vtable helpers around the `+0x544` and `+0x5BC` sub-objects.
`0x80A038B0` builds the actor matrix, applies the `+scale`/`-scale` pair from
`+0x6A8` and snapshots matrices `0` and `3` into `+0x67C..+0x690`.
`0x80A03B10` binds the model and clip.

### `daEnBuku_c` handler block (`0x80A03E70..0x80A05970`, 19 functions)

The slide-slot bookkeeping (`0x80A03E70`, `0x80A03EA0`, `0x80A03ED0`,
`0x80A03F40`, `0x80A042C0`) works on four parallel byte arrays (`+0x6C4`,
`+0x6CC`, `+0x6D4`) and two halfword timers (`+0x6B4`, `+0x6BC`). The movement
half (`0x80A045A0` through `0x80A04F70`) selects clips, computes the chase
delta, decides facing, turns the yaw, wobbles the swim pitch and initialises
`AttackWait`. `0x80A05290` is the `SrcSwim` step. The two callbacks close the
run.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A03150` | `fn_80a03150` | Generic `fState` callback | translated | high | `80a03150_FUN_80a03150.c` |
| `0x80A032D0` | `fn_80a032d0` | Allocate/construct a `0x528`-byte actor | translated | high | `80a032d0_FUN_80a032d0.c` |
| `0x80A03850` | `fn_80a03850` | Run the `+0x544` holder's `+0x14` slot, return `1` | translated | high | `80a03850_FUN_80a03850.c` |
| `0x80A03880` | `fn_80a03880` | Update `+0x5BC` through `0x8008C380`, return `1` | translated | high | `80a03880_FUN_80a03880.c` |
| `0x80A038B0` | `fn_80a038b0` | Build matrix with mirrored Y scale, snapshot matrices 0/3 | translated | high | `80a038b0_FUN_80a038b0.c` |
| `0x80A03B10` | `fn_80a03b10` | Bind `g3d/bakubaku.brres`, `bakubaku` model and `swim` clip | translated | high | `80a03b10_FUN_80a03b10.c` |
| `0x80A03E70` | `fn_80a03e70` | Clear the four `+0x6C8`/`+0x6D0` byte pairs | translated | high | `80a03e70_FUN_80a03e70.c` |
| `0x80A03EA0` | `fn_80a03ea0` | Release one player's grab effect | translated | high | `80a03ea0_FUN_80a03ea0.c` |
| `0x80A03ED0` | `fn_80a03ed0` | Release every flagged grab slot | translated | high | `80a03ed0_FUN_80a03ed0.c` |
| `0x80A03F40` | `fn_80a03f40` | Finish pending grabs, arm the `0x3D` timer | translated | medium | `80a03f40_FUN_80a03f40.c` |
| `0x80A042C0` | `fn_80a042c0` | Tick four timers and resolve the slots that reach `1` | translated | medium | `80a042c0_FUN_80a042c0.c` |
| `0x80A04540` | `fn_80a04540` | Spawn the `7` effect, forward to `0x8009F840` | translated | medium | `80a04540_FUN_80a04540.c` |
| `0x80A045A0` | `fn_80a045a0` | Select and bind one animation index | translated | high | `80a045a0_FUN_80a045a0.c` |
| `0x80A04640` | `fn_80a04640` | Spawn the four-word `7` effect record | translated | high | `80a04640_FUN_80a04640.c` |
| `0x80A04750` | `fn_80a04750` | Copy the horizontal chase delta into both vectors | translated | high | `80a04750_FUN_80a04750.c` |
| `0x80A04790` | `fn_80a04790` | Facing/turn decision, returns `1` on a completed turn | translated | high | `80a04790_FUN_80a04790.c` |
| `0x80A048D0` | `fn_80a048d0` | Ground probe for the turn flag | translated | high | `80a048d0_FUN_80a048d0.c` |
| `0x80A04940` | `fn_80a04940` | Turn the yaw toward a direction vector | translated | high | `80a04940_FUN_80a04940.c` |
| `0x80A049D0` | `fn_80a049d0` | `Swim` frame step: pitch wobble, turn, move | translated | high | `80a049d0_FUN_80a049d0.c` |
| `0x80A04CA0` | `fn_80a04ca0` | Reset the `Swim` velocity and clip | translated | high | `80a04ca0_FUN_80a04ca0.c` |
| `0x80A04EA0` | `fn_80a04ea0` | `AttackSwim` frame step | translated | medium | `80a04ea0_FUN_80a04ea0.c` |
| `0x80A04F70` | `fn_80a04f70` | `AttackWait` initialiser | translated | high | `80a04f70_FUN_80a04f70.c` |
| `0x80A05290` | `fn_80a05290` | `SrcSwim` frame step | translated | medium | `80a05290_FUN_80a05290.c` |
| `0x80A05910` | `fn_80a05910` | `daEnBuku_c` state callback (four records) | translated | high | `80a05910_FUN_80a05910.c` |
| `0x80A05970` | `fn_80a05970` | `daEnBuku_c::StateID_DieFall` state callback | translated | high | `80a05970_FUN_80a05970.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A03C30` | 92 B | Ends in `halt_baddata()`; the export truncates the control flow after the `0x18` timer store |
| `0x80A03C90` | 212 B | GQR/`ldexpf` artefacts; the export removes five unreachable blocks |
| `0x80A03FF0` | 708 B | GQR/`ldexpf` artefacts and three removed unreachable blocks |
| `0x80A04A90` | 348 B | GQR/`ldexpf` artefacts and one removed block |
| `0x80A04BF0` | 176 B | GQR/`ldexpf` artefacts and one removed block |
| `0x80A04D10` | 360 B | GQR/`ldexpf` artefacts; calls `0x80A049D0` without its receiver |
| `0x80A05000` | 616 B | GQR/`ldexpf` artefacts and one removed block |
| `0x80A059D0` | 12 B | Returns `0x80357BF4`; the extra-state word installed for `DieFall` |
| `0x80A05DF0` | 116 B | `daEnBuku_c` container helper; outside the state-handler flow |
| `0x80A07080` | 224 B | Recursive chain finder deferred by wave 11; recursion shape needs review |
| `0x80A073B0` | 12 B | Returns `0x80377A04`; no caller in this export |
| `0x80A073C0` | 12 B | Returns `0x80377A04`; no caller in this export |
| `0x80A07C80` | 16 B | Export reports bad instruction data and `halt_baddata()` |

These remain address-based calls or untouched entries; none is presented as a
new symbol.

## Unknowns preserved in code

1. **`bakubaku` actor class.** No `StateID` table in this export registers the
   actor that `0x80A03B10` binds. The block is named by its model string and
   by the holder offsets it shares with `0x80A038B0`; the class label is not
   recoverable and no symbol is invented.
2. **`0x528`-byte constructor.** Only the allocation size, base constructor
   and class vtable `0x80B333E8` survive; the class name and the rest of the
   layout are unknown.
3. **Grab-slot meaning.** The four byte arrays at `+0x6C4`, `+0x6CC`, `+0x6D4`
   and the two halfword timers behave as a small state machine. The gameplay
   meaning (what is grabbed and by whom) is inferred from the calls to
   `0x8005F900`/`0x8013E630`/`0x8013E6C0`/`0x8013E7C0` and marked `medium`.
4. **Compiler-helper receivers.** `0x80A03F40` and `0x80A042C0` read their
   actor pointer from `0x802DCD68`/`0x802DCD70`, and `0x80A03FF0`-style
   neighbours use the same trick. The Rust keeps the helper call and names the
   result `actor`.
5. **Dropped call arguments.** Ghidra prints `0x8005F900()`,
   `0x80A04640()`, `0x80A049D0()`, `0x80A04A90(1.25)` and
   `0x80A04BF0(speed)` without the receiver. The translations supply the only
   receiver in scope and mark the inference.
6. **`+0x348` facing byte.** Flipped by `fn_80a04790` and read by
   `fn_80a04940`; the compass meaning of the two values is inferred from the
   `0x4000` (quarter-turn) offsets.
7. **Effect record.** The four `7` words and the mask `0x0B000000` are copied
   literally; the record's field names are outside this export.
8. **`0x8009F840`.** The three-word callback forwarded by `fn_80a04540` is
   unnamed in this export; the wrapper is treated as an override of a base
   method, not as a named engine symbol.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `cargo check` succeeds and
`node tools/doc-lint.mjs src` reports 0 violations across 68 files. This lane
modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave12-enemies.csv`.
