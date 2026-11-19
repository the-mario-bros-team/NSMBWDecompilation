# Coverage - wave 16 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the sixteenth translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found: the
   remaining `daEnHimanBros_c` handlers named by its own state table, the
   class callbacks that follow that table, and the first handlers of the next
   (`switch`-model) class block.
3. **What is still missing?** - *Deferred neighbours* names the untranslated
   entries around the selected ranges, and why each was left out.

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

| Metric | Before wave 16 | Wave 16 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 332 | 30 | 362 |
| Of those, `translated` | 321 | 28 | 349 |
| Of those, `partial` | 11 | 2 | 13 |
| Of those, `stub` | 0 | 0 | 0 |
| Constructors | - | 1 | - |
| Per-state handlers | - | 9 | - |
| State callbacks and factories | - | 4 | - |
| Probes and name helpers | - | 2 | - |
| Destructor forwarders | - | 2 | - |
| Variant/model handlers | - | 9 | - |
| State logic and release paths | - | 3 | - |
| Selected bytes | - | 5,536 | - |

The 30 new bodies cover 5,536 bytes (0.95 % of the 582,728 bytes that the gap
report listed as untranslated before this pass) and 30 of the 3,223 remaining
function entries (0.93 %). Confidence is **21 high** and **9 medium**. The two
48-byte forwarders are `partial`: their visible body is complete, but the
hidden `r12` operand of `0x802DCBFC` cannot be modelled by `platform::call3`.
There are no `stub` rows.

Cumulative after this pass: 362 bodies, 349 `translated`, 13 `partial`, 3,193
function entries and 577,192 bytes still untranslated.

## Selection

This pass was evidence-first and continuation-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Used the state table as the class proof.** Wave 15 stopped after
   `0x80A744D0` and left `0x80A746E0..0x80A75520` unattributed. The table at
   `0x80A75800` registers eleven `daEnHimanBros_c` descriptors and the
   handlers pass exactly those descriptors to the class state-switch slot
   `+0xD4`: `0x80B72D68` = `Turn`, `0x80B72D28` = `Walk`, `0x80B72B68` =
   `Attack`, `0x80B72BA8` = `AttackEd`, `0x80B72C28` = `JumpAttack`,
   `0x80B72C68` = `JumpAttack_Ed`, `0x80B72CA8` = `Quake`, `0x80B72CE8` =
   `Quake_Ed`. That is stronger evidence than adjacency, so the nine handlers
   are labelled `daEnHimanBros_c`.
3. **Finished the class block.** `0x80A76090`/`0x80A760F0` are the two
   registration callbacks, `0x80A76150`/`0x80A76160` the two extra-word
   factories, `0x80A76250` the five-level `+0x30` probe, `0x80A76330` the
   `:`-suffix name comparison and `0x80A763F0`/`0x80A76420` the two
   `0x802DCBFC` forwarders. The constructor `0x80A76450` was already
   translated in wave 4 and is only referenced.
4. **Started the next block.** `0x80A77820..0x80A786A0` are per-frame and
   collision handlers of the class whose table is absent from the export.
   Identity evidence is data-only: the model table at `0x80B4BEE8` starts with
   `switch`, `switch_koopa` and the two `g3d/switch*.brres` paths, and the
   handlers use `Wm_ko_switch`, `Wm_ob_switch` and `Wm_en_burst_s`. They are
   therefore documented as the `switch`-model object block; the descriptor
   names stay `DESC_`.
5. **Closed with the next constructor.** `0x80A79460` is a clean `0xEB0`-byte
   constructor. It is included as a constructor because every store and
   argument is explicit; its class link is `// inferred:` from adjacency and
   is not claimed as proven.
6. **Excluded every export with damaged control flow.** `0x80A74570` (GQR
   `2^52` artefacts), `0x80A74600`, `0x80A778F0`/`0x80A77BB0`
   (`halt_baddata()`), `0x80A77A50` (GQR/`ldexpf`), `0x80A78350` (GQR
   promotions in the sound path) and `0x80A787D0` (large mixed handler) are
   still called by address where a translated body needs them.
7. **Read the data constants from the REL image.** Address minus base equals
   the file offset for this module, so the `f32`/`i16`/string values were
   checked directly; the Rust keeps the original data addresses instead of
   baking the values in.

### The selected blocks (`0x80A746E0..0x80A79460`, 30 functions)

The first nine entries are the `daEnHimanBros_c` state handlers:
`0x80A746E0` is the walk handler that picks `Turn` or the countdown-driven
`Attack`/`Jump_St`; `0x80A748C0`/`0x80A74CB0` are the two turn clamps;
`0x80A74AC0` is the three-phase `Attack`; `0x80A74E20` chooses `Quake` or
`JumpAttack`; `0x80A74FC0` is `JumpAttack_Ed`; `0x80A75210`/`0x80A75520` are
the two land-to-`Walk` steps; `0x80A75360` is `Quake_Ed`.

The second group is the class plumbing: two registration callbacks, two
constant factories, the recursive probe, the name comparison and two
destructor forwarders.

The third group is the `switch`-model block: the four-slot release loop, two
latch-to-state helpers, the base update, the holder steps, the physics reset,
the variant dispatcher, the two-step model loader, the burst effect, the
burst handler and the timer-driven release path. The last cleanup helper in
that range (`0x80A786A0`) may belong to the HimanBros class instead, because
it resets the `+0x754` holder that constructor initialises.

The last entry is the `0xEB0`-byte constructor with four containers.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A746E0` | `fn_80a746e0` | Walk handler; faces the actor, then `Turn` or the countdown-driven `Attack`/`Jump_St` | translated | high | `80a746e0_FUN_80a746e0.c` |
| `0x80A748C0` | `fn_80a748c0` | Turn clamp on `+0x102`, then `Walk` | translated | high | `80a748c0_FUN_80a748c0.c` |
| `0x80A74AC0` | `fn_80a74ac0` | Three-phase attack; effect link, target claim, `AttackEd` | translated | high | `80a74ac0_FUN_80a74ac0.c` |
| `0x80A74CB0` | `fn_80a74cb0` | Turn step of `0x200` per frame, then `Walk` | translated | high | `80a74cb0_FUN_80a74cb0.c` |
| `0x80A74E20` | `fn_80a74e20` | Jump/quake smoke and `Quake`/`JumpAttack` choice | translated | high | `80a74e20_FUN_80a74e20.c` |
| `0x80A74FC0` | `fn_80a74fc0` | `JumpAttack_Ed` with per-side clamp and frame-28 hit | translated | medium | `80a74fc0_FUN_80a74fc0.c` |
| `0x80A75210` | `fn_80a75210` | Landing smoke at frame 3.0, then `Walk` | translated | high | `80a75210_FUN_80a75210.c` |
| `0x80A75360` | `fn_80a75360` | `Quake_Ed`; stop clip, sound `0x1EE`, player latch, 60-frame timer | translated | high | `80a75360_FUN_80a75360.c` |
| `0x80A75520` | `fn_80a75520` | Wait step; `Walk` when the clip finishes | translated | high | `80a75520_FUN_80a75520.c` |
| `0x80A76090` | `fn_80a76090` | First `daEnHimanBros_c` registration callback | translated | high | `80a76090_FUN_80a76090.c` |
| `0x80A760F0` | `fn_80a760f0` | Second registration callback (same body) | translated | high | `80a760f0_FUN_80a760f0.c` |
| `0x80A76150` | `fn_80a76150` | Return extra word `0x80357BF4` | translated | high | `80a76150_FUN_80a76150.c` |
| `0x80A76160` | `fn_80a76160` | Return extra word `0x80357BB4` | translated | high | `80a76160_FUN_80a76160.c` |
| `0x80A76250` | `fn_80a76250` | Five-level `+0x30` probe with recursive tail | translated | high | `80a76250_FUN_80a76250.c` |
| `0x80A76330` | `fn_80a76330` | Compare the object's `:`-suffix name | translated | high | `80a76330_FUN_80a76330.c` |
| `0x80A763F0` | `fn_80a763f0` | First `0x802DCBFC` destructor forwarder | partial | medium | `80a763f0_FUN_80a763f0.c` |
| `0x80A76420` | `fn_80a76420` | Second `0x802DCBFC` destructor forwarder | partial | medium | `80a76420_FUN_80a76420.c` |
| `0x80A77820` | `fn_80a77820` | Release the four per-player blend slots | translated | medium | `80a77820_FUN_80a77820.c` |
| `0x80A77970` | `fn_80a77970` | Latch to `DESC_80B72E68`, else clear `+0x8A2` | translated | medium | `80a77970_FUN_80a77970.c` |
| `0x80A779B0` | `fn_80a779b0` | Latch to `DESC_80B72EA8` | translated | medium | `80a779b0_FUN_80a779b0.c` |
| `0x80A779E0` | `fn_80a779e0` | Base update when variant 0 is armed | translated | medium | `80a779e0_FUN_80a779e0.c` |
| `0x80A77E60` | `fn_80a77e60` | Step the `+0x544` holder and the variant-4 `+0x5BC` holder | translated | high | `80a77e60_FUN_80a77e60.c` |
| `0x80A77EC0` | `fn_80a77ec0` | Physics reset and variant-animation rebind | translated | high | `80a77ec0_FUN_80a77ec0.c` |
| `0x80A77F50` | `fn_80a77f50` | Per-frame dispatcher over `+0x874` | translated | medium | `80a77f50_FUN_80a77f50.c` |
| `0x80A78000` | `fn_80a78000` | Two-step `Wm_ko_switch`/`Wm_ob_switch` model loader | translated | high | `80a78000_FUN_80a78000.c` |
| `0x80A78140` | `fn_80a78140` | `Wm_en_burst_s` effect at the actor position | translated | high | `80a78140_FUN_80a78140.c` |
| `0x80A78220` | `fn_80a78220` | Variant sound, untranslated state write, break effect | translated | high | `80a78220_FUN_80a78220.c` |
| `0x80A785E0` | `fn_80a785e0` | Timer-driven state choice or retirement | translated | medium | `80a785e0_FUN_80a785e0.c` |
| `0x80A786A0` | `fn_80a786a0` | Clear `+0x124` and reset both holders | translated | high | `80a786a0_FUN_80a786a0.c` |
| `0x80A79460` | `fn_80a79460` | Allocate and construct the `0xEB0`-byte actor | translated | high | `80a79460_FUN_80a79460.c` |

## Deferred neighbours

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A74570` | 136 B | GQR/`ldexpf` preamble; the `double` first argument is a `2^52` artefact |
| `0x80A74600` | 176 B | Calls `0x80A74570` with a `double` first argument |
| `0x80A75190` | 112 B | Same `0x80A74570` `2^52` argument pattern |
| `0x80A752E0` | 104 B | Same `0x80A74570` `2^52` argument pattern |
| `0x80A754B0` | 88 B | Same `0x80A74570` `2^52` argument pattern |
| `0x80A771D0` | 580 B | Model binder with four animation lookups; clean but large, left for the next pass |
| `0x80A77420` | 288 B | Variant-animation binder called by `fn_80a77ec0`; clean two-branch handler |
| `0x80A77540` | 316 B | Contact callback already translated in an earlier wave |
| `0x80A77680` | 224 B | Clean contact callback; next pass |
| `0x80A77760` | 192 B | Clean contact callback with per-slot flags; next pass |
| `0x80A778F0` | 116 B | Export ends in `halt_baddata()` |
| `0x80A77A50` | 340 B | GQR/`ldexpf` artefacts in the constant setup |
| `0x80A77BB0` | 124 B | Export ends in `halt_baddata()` (called by address) |
| `0x80A77C30` | 548 B | `countLeadingZeros` branch pair; clean but needs a separate pass |
| `0x80A78350` | 616 B | GQR promotions in the sound/camera path |
| `0x80A78700` | 64 B | Duplicate of `0x80A786A0`; one body is enough |
| `0x80A787D0` | 356 B | Mixed input/rumble handler; clean but large |
| `0x80A78940` | 216 B | Variant damage/release handler; next pass |
| `0x80A78A20` | 76 B | Release thunk; next pass |
| `0x80A78A70` | 356 B | Clean mixed handler; next pass |
| `0x80A792E0` | 88 B | Third registration callback (same body as `0x80A76090`) |
| `0x80A79400` | 48 B | Third `0x802DCBFC` forwarder |
| `0x80A79430` | 48 B | Fourth `0x802DCBFC` forwarder |

The clean entries `0x80A771D0`, `0x80A77420`, `0x80A77C30`, `0x80A787D0`,
`0x80A78940` and `0x80A78A70` are the best next targets; together they cover
the rest of the `switch`-model class block.

## Unknowns preserved in code

1. **`daEnHimanBros_c` handler ownership.** The table proves the *destination*
   descriptors, not which state owns each handler. The docs therefore say
   "enters `StateID_X`" rather than "is `StateID_X`". The class label itself is
   solid: eleven `daEnHimanBros_c::StateID_*` registrations.
2. **`0x802DE514`.** It is the integer absolute value (`(v >> 31 ^ v) -
   (v >> 31)`); the two angle clamps rely on that reading.
3. **`0x801690E0`/`0x80169020`/`0x80168080`.** The animation-time query, the
   animation-finished query and the current-frame getter are named from their
   bodies in `main.dol`; only the call shapes are used here.
4. **`switch`-model class and states.** Its state table is absent from the
   export, so the descriptors keep `DESC_` names and the class label is
   `// inferred:` from the `switch` model/effect strings.
5. **`fn_80a77820` actor argument.** The export prints the function as `void`
   and obtains the actor from the `0x802DCD74` prologue fragment. The only
   caller passes the actor in `r3`, so the Rust takes it as a parameter and
   omits the save/restore fragments.
6. **`fn_80a77970`/`fn_80a779b0`.** Ghidra reports an unrecovered jump table at
   `0x80A77998`/`0x80A779D8`; the visible body is a single indirect
   state-switch call, which is how the Rust models it.
7. **`fn_80a74FC0` second-phase guard.** The decompiler reuses the `+0x1EC`
   probe value read at the top of the function; the Rust keeps that reuse and
   marks it `// observed:`.
8. **`0x80A79460` class link.** The constructor is complete and high
   confidence, but the class it belongs to is only adjacent-block evidence;
   the Rust keeps `fn_80a79460` and does not claim a class name.
9. **`0x80A786A0` ownership.** It resets `+0x754`, which the
   `daEnHimanBros_c` constructor `0x80A76450` initialises, so it may be a
   HimanBros cleanup even though it sits inside the `switch`-model range. The
   body is high confidence; the class label stays unknown.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

At the end of this pass `node tools/doc-lint.mjs src` reports 0 violations
across 68 files. `cargo check` reports no warning or error in
`src/game/enemies.rs`. This lane modified only `src/game/enemies.rs`, this
page and `ledger/agents/wave16-enemies.csv`.
