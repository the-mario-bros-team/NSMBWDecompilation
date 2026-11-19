# Coverage - wave 24 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-fourth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   completely untranslated `daEnItem_c` switch-item block at
   `0x80A855A0..0x80A873C0`.
3. **What is still missing?** - *Deferred neighbours* names the clean siblings
   left for a later pass and every export skipped because its control flow is
   damaged.

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

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Before wave 24 | Wave 24 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 538 | 30 | 568 |
| Of those, `translated` | 523 | 30 | 553 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| Switch-item lifecycle bodies | - | 30 | - |
| Selected bytes | - | 5,732 | - |

The 30 new bodies cover 5,732 bytes (1.05 % of the 547,952 bytes that were
still untranslated before this pass; 5,732 / 547,952 = 1.05 %) and 30 of the
3,017 remaining function entries (0.99 %). Confidence is **25 high** and
**5 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 568 bodies, 553 `translated`, 15 `partial`, 2,987
function entries and 542,220 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Found the last large untouched run.** A density scan over the export
   (`functions.tsv` cross-referenced with the merged ledger) showed that the
   module is now covered in islands and that the only sizeable clean run near
   the wave-23 edge is the `daEnItem_c` block. `0x80A85200..0x80A88D50` was
   completely untranslated; the pass took the coherent middle
   (`0x80A855A0..0x80A873C0`) and deferred the transform/constructor head.
3. **Identified the class from two independent strings.** The export's state
   lists carry `daEnItem_c::StateID_KinokoMove` (`0x80B4EC04`) and the model
   binder at `0x80A84F10` loads `g3d/I_kinoko.brres` (`0x80B4E2F4`) plus the
   clips `I_kinoko` (`0x80B4E34C`) and `I_kinoko_switch` (`0x80B4E398`). The
   `daEnItem_c` label is therefore `// inferred:` but reinforced twice.
   The family constructor is `0x80A83DE0` (allocates `0xE68` bytes, class
   table `0x80B4E530`); it is deferred because its prologue/epilogue fragments are
   printed as calls to `0x802DCD6C`/`0x802DCDB8`.
4. **Took the whole lifecycle, not isolated helpers.** The selected run walks
   the item from the landing smoke (`0x80A86070`), through the pickup sparkle
   (`0x80A855A0`) and full collection (`0x80A85AD0`), the pop/twirl machine
   (`0x80A86680`, `0x80A86750`), the fall/rise entries (`0x80A86BF0`,
   `0x80A86E40`, `0x80A87310`) and the per-frame input handler
   (`0x80A873C0`).
5. **Excluded every export with damaged control flow.** `0x80A85200`,
   `0x80A85EF0`, `0x80A863D0`, `0x80A86400` and `0x80A87030` contain
   `halt_baddata()` / GQR / `ldexpf` artefacts. None were guessed; the three
   that appear inside the selected run (`0x80A85200`, `0x80A85EF0`,
   `0x80A86600`) are called by address with a note.
6. **Reused only established helpers.** `0x8016C890`, `0x80198C30`,
   `0x80162510`, `0x80162D00`, `0x8006A4B0`, `0x8005F900`, `0x80070720`,
   `0x80075270`, `0x800955F0`, `0x8006CDE0`, `0x8006CD90`, `0x80199390`,
   `0x801651F0`, `0x80169F80`, `0x801680E0`, `0x8023A0B0`, `0x80169020`,
   `0x80168080` and `0x801C0BD0` already had constants in the file. The nine
   new opaque callees are kept by address.
7. **Read every numeric constant and table from the REL image.**
   `0x80B2E4B4 = 0.0`, `0x80B2E580 = -2600.0`, `0x80B2E58C = 8.0`,
   `0x80B2E590 = 1.0`, `0x80B2E59C = 0.5`, `0x80B2E5B0 = -8.0`,
   `0x80B2E5B4 = -0.1875`, `0x80B2E5B8 = 600.0`, `0x80B2E5C0 = 4.0`,
   `0x80B2E5C4 = 3.5`, `0x80B2E5C8 = 4000.0`, `0x80B2E5E0 = 2.0`,
   `0x80B2E5E4 = 2.5`, `0x80B2E5E8 = -1.0`, `0x80B2E5F8 = 0.03125`,
   `0x80B2E620 = 5500.0`, `0x80B2E628 = 80.0`, `0x80B2E670 = 1.8`,
   `0x80B2E674 = -0.7`, `0x80B2E678 = 0.7`, `0x80B2E67C = -0.046875`,
   `0x80B2E680 = 1.8`, `0x80B2E684 = 0.0625`, `0x80B2E688 = -0.09375`,
   `0x80B2E68C = 0.125`, `0x80B2E690 = 26.0`, `0x80B2E694 = -1.8`,
   `0x80B2E698 = -2.4`, `0x80B2E69C = -0.15625`, `0x80B2E6A0 = 48.0`,
   `0x80B2E4B8 = 1.0/-1.0` and `0x80B2E4C0 = 0.15/-0.15` were read as
   big-endian `f32`. The pop table `0x80B2E640` holds `-1.4, 1.0, -1.4, 1.4,
   -1.4, 4.0`; the twirl table `0x80B2E658` holds `0.03125` five times and
   `0.046875`; the flag table `0x80B2E62C` holds `1, 0, 1, 0, 0`.
8. **Resolved the two relocated name tables through the REL relocations.**
   `0x80B4E4B0` holds `out`, `out2`, `out3`, `wait`, `wait2`;
   `0x80B4E434` holds `Wm_ob_itemlandsmoke`, `Wm_ob_itemsndlandsmk`,
   `Wm_ob_itemsealandsmk`. Both tables are zero in the static image because
   their words carry `ADDR32` relocations (addends `0x1B630`, `0x1B644`,
   `0x1B65C` at `0x80B4E434..0x80B4E43C`); the Rust therefore keeps the load
   and documents the names.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A855A0` | `fn_80a855a0` | Pickup sparkle `Wm_ob_itemget` at position plus offset | translated | high | `80a855a0_FUN_80a855a0.c` |
| `0x80A85A20` | `fn_80a85a20` | Burst effect, sound `0x259`, retire | translated | high | `80a85a20_FUN_80a85a20.c` |
| `0x80A85AD0` | `fn_80a85ad0` | Full collection: owner reset, four-rider release, burst, retire | translated | high | `80a85ad0_FUN_80a85ad0.c` |
| `0x80A85C70` | `fn_80a85c70` | Follow parent lift downward (`+0xB0`/`+0xD50`) | translated | high | `80a85c70_FUN_80a85c70.c` |
| `0x80A85CD0` | `fn_80a85cd0` | Follow parent lift sideways (`+0xAC`) | translated | high | `80a85cd0_FUN_80a85cd0.c` |
| `0x80A85D20` | `fn_80a85d20` | Choose release y (`-2600`/`4000`/`600`) | translated | high | `80a85d20_FUN_80a85d20.c` |
| `0x80A85D60` | `fn_80a85d60` | Set release speed and switch to landing/idle | translated | high | `80a85d60_FUN_80a85d60.c` |
| `0x80A85E10` | `fn_80a85e10` | Reset motion fields and switch to the same states | translated | high | `80a85e10_FUN_80a85e10.c` |
| `0x80A85F10` | `fn_80a85f10` | `Wm_ob_itemappear` phase machine | translated | high | `80a85f10_FUN_80a85f10.c` |
| `0x80A86070` | `fn_80a86070` | Landing-smoke selector | translated | high | `80a86070_FUN_80a86070.c` |
| `0x80A86160` | `fn_80a86160` | Water splash check | translated | high | `80a86160_FUN_80a86160.c` |
| `0x80A861E0` | `fn_80a861e0` | Spawn `Wm_ob_startail` while item type `2` | translated | medium | `80a861e0_FUN_80a861e0.c` |
| `0x80A86280` | `fn_80a86280` | Spawn `Wm_ob_itempropeller` for type `0xB` | translated | medium | `80a86280_FUN_80a86280.c` |
| `0x80A86330` | `fn_80a86330` | Spawn `Wm_ob_itemfall` / `Wm_ob_redcioinitem02` | translated | high | `80a86330_FUN_80a86330.c` |
| `0x80A86420` | `fn_80a86420` | Contact test and `+0x45D` latch | translated | high | `80a86420_FUN_80a86420.c` |
| `0x80A86470` | `fn_80a86470` | Held/released damping after contact | translated | medium | `80a86470_FUN_80a86470.c` |
| `0x80A86540` | `fn_80a86540` | Select and bind one switch clip | translated | high | `80a86540_FUN_80a86540.c` |
| `0x80A865E0` | `fn_80a865e0` | Ensure the `wait` clip | translated | high | `80a865e0_FUN_80a865e0.c` |
| `0x80A86610` | `fn_80a86610` | Step `+0xEC` toward `+0xF8` | translated | high | `80a86610_FUN_80a86610.c` |
| `0x80A86680` | `fn_80a86680` | Open the pop sequence | translated | high | `80a86680_FUN_80a86680.c` |
| `0x80A86750` | `fn_80a86750` | Pop/twirl state machine (`+0xDC8` = 0..5) | translated | high | `80a86750_FUN_80a86750.c` |
| `0x80A869F0` | `fn_80a869f0` | Start the `out` pop pose | translated | high | `80a869f0_FUN_80a869f0.c` |
| `0x80A86A50` | `fn_80a86a50` | Follow/landing frame handler | translated | high | `80a86a50_FUN_80a86a50.c` |
| `0x80A86B60` | `fn_80a86b60` | Airborne follow handler with state guard | translated | high | `80a86b60_FUN_80a86b60.c` |
| `0x80A86BF0` | `fn_80a86bf0` | Enter the drop or the propeller pop state | translated | medium | `80a86bf0_FUN_80a86bf0.c` |
| `0x80A86D70` | `fn_80a86d70` | Per-frame dispatch by item type | translated | high | `80a86d70_FUN_80a86d70.c` |
| `0x80A86E40` | `fn_80a86e40` | Drop onto the owner from `48.0` above the floor | translated | high | `80a86e40_FUN_80a86e40.c` |
| `0x80A86ED0` | `fn_80a86ed0` | Fall and wobble handler | translated | high | `80a86ed0_FUN_80a86ed0.c` |
| `0x80A87310` | `fn_80a87310` | Enter the rise pose | translated | medium | `80a87310_FUN_80a87310.c` |
| `0x80A873C0` | `fn_80a873c0` | Main per-frame input handler | translated | high | `80a873c0_FUN_80a873c0.c` |

## Deferred neighbours

### Clean siblings left for a later pass

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A83DE0` | 420 B | Family constructor; prologue/epilogue fragments are printed as calls to `0x802DCD6C`/`0x802DCDB8` |
| `0x80A84A00` | 400 B | Ten-slot render walk; same prologue fragment pattern |
| `0x80A84F10` | 744 B | `I_kinoko` model/clip binder; belongs with the constructor |
| `0x80A85260` | 528 B | Clean `+0xDFA` wobble/stand handler; needs the ground-contact block as context |
| `0x80A85470` | 92 B | Clean effect-mask helper for the stand state |
| `0x80A85520` | 112 B | Clean ground-contact speed selector |
| `0x80A85590` | 16 B | Vtable forwarder with an unrecovered jumptable warning |
| `0x80A85610` | 64 B | Clean carrier-slot comparison |
| `0x80A85650` | 976 B | Flow is clean, but it calls the two truncated exports `0x80A863D0`/`0x80A86400` |
| `0x80A85C30` | 56 B | Clean countdown helper for `+0xDD4`/`+0xDD6` |
| `0x80A86510` | 48 B | No-argument helper whose `r3` use is not recoverable from the export |
| `0x80A86600` | 8 B | One-line forwarder to `fn_80a86540` with index `1` |
| `0x80A86B00` | 36 B | `out3` variant of `fn_80a869f0` |
| `0x80A87640`..`0x80A88D50` | 23 entries | Next handler cluster; needs its own pass |

### Damaged exports inside or next to the run

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A85200` | 228 B | Ends in `halt_baddata()` after eight stores |
| `0x80A85EF0` | 20 B | One store, then `halt_baddata()` |
| `0x80A863D0` | 36 B | `halt_baddata()` only |
| `0x80A86400` | 32 B | `halt_baddata()` only |
| `0x80A87030` | 732 B | GQR/`ldexpf` artefacts and one removed unreachable block |

## Unknowns preserved in code

1. **Class label.** `daEnItem_c` is `// inferred:` from the surviving
   `daEnItem_c::StateID_*` strings and the `I_kinoko` model/`I_kinoko_switch`
   clip binder. The profile registry in `d_profileNP` remains the authority.
2. **`0x8042C1B8` multiplier.** It scales the halved twirl rate when an item
   becomes held. The owner and meaning are in `main.dol` and outside this lane.
3. **Two vtable receivers.** In `fn_80a861e0` and `fn_80a86280` Ghidra
   prints the model-root call with no arguments; the receiver is modelled as
   `actor + 0x394` from the register setup (`medium`).
4. **`0x803580C4`.** A `main.dol` record/id compared against the object slot
   `+0x14`. Its type and meaning are unresolved.
5. **Address-only callees.** `0x80A85200`, `0x80A85520`, `0x80A85590`,
   `0x80A85650`, `0x80A85C30`, `0x80A85EF0`, `0x80A86510`, `0x80A86600`
   and `0x80A84ED0` are kept by address.
6. **Relocated name tables.** The clip and landing-smoke tables read zero in
   the static image; their contents were recovered from the REL `ADDR32`
   relocations, not from the Ghidra text.
7. **Field roles.** `+0xDF2`, `+0xDF4`, `+0xDFA`, `+0xDFB`, `+0xE00`,
   `+0xE04`, `+0xE08`, `+0xE0C` and `+0xE10` are named from the branch shape,
   not from a symbol; the block header in `src/game/enemies.rs` marks the whole
   map `// inferred:`.
8. **Soft-float returns.** `0x80A85D20` and `0x80A84ED0` are printed with
   `double` returns although both bodies load a float; the Rust models them as
   `f32` (see `fn_80a85d20`).

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. `cargo check` succeeds and reports no error or warning in
`src/game/enemies.rs`; its remaining warnings are in other lanes' files and
were not modified here.

This lane modified only `src/game/enemies.rs`, this page and
`ledger/agents/wave24-enemies.csv`.

