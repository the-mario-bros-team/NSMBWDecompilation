# Coverage - wave 23 enemy handlers (`d_enemiesNP`)

## How to read this file

This page records the twenty-third translation pass over the enemy REL,
`d_enemiesNP`. It answers three questions:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/enemies.rs`, with their original addresses, Rust names,
   plain-English roles, status, confidence and Ghidra evidence.
2. **Why these functions?** - *Selection* explains how the run was found:
   the `daikonbouBig` block at `0x80A69010..0x80A6B420` and the
   `kinopio_balloon` block at `0x80A6BA60..0x80A6C6C0`.
3. **What is still missing?** - *Deferred neighbours* names the clean blocks
   deliberately left for a later pass and every export skipped because its
   control flow is damaged.

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

| Metric | Before wave 23 | Wave 23 | Cumulative (`game/enemies.rs`) |
|---|---:|---:|---:|
| Function bodies in `src/game/enemies.rs` | 508 | 30 | 538 |
| Of those, `translated` | 493 | 30 | 523 |
| Of those, `partial` | 15 | 0 | 15 |
| Of those, `stub` | 0 | 0 | 0 |
| `daikonbouBig` constructors / binders | - | 4 | - |
| `daikonbouBig` dispose callbacks / constants | - | 4 | - |
| `daikonbouBig` chain walker / name predicate | - | 2 | - |
| `daikonbouBig` height and band helpers | - | 4 | - |
| `daikonbouBig` scene predicate | - | 1 | - |
| `daikonbouBig` effect spawners | - | 6 | - |
| `daikonbouBig` destructor | - | 1 | - |
| `kinopio_balloon` constructor / binder | - | 2 | - |
| `kinopio_balloon` placement / contact / coin handler | - | 3 | - |
| `kinopio_balloon` burst / frame / dispose | - | 3 | - |
| Selected bytes | - | 6,564 | - |

The 30 new bodies cover 6,564 bytes (1.18 % of the 554,516 bytes that were
still untranslated before this pass; 6,564 / 554,516 = 1.18 %) and 30 of the
3,047 remaining function entries (0.98 %). Confidence is **24 high** and
**6 medium**. There are no `stub` rows and no new `partial` rows.

Cumulative after this pass: 538 bodies, 523 `translated`, 15 `partial`, 3,017
function entries and 547,952 bytes still untranslated.

## Selection

This pass was continuation-first and evidence-first:

1. **Checked the merged ledger and the file first.** Every address below was
   absent from `ledger/functions.csv` for `game::enemies` and from
   `src/game/enemies.rs` before this pass. All 30 bodies were written before
   any ledger row was added.
2. **Continued immediately after wave 22.** The previous pass stopped at
   `0x80A66C40`; the next export is `0x80A673D0`. That export is damaged, so
   the pass opened the first clean block after it: the `goal_set`/four-slot
   object at `0x80A67A60..0x80A68A50`. That block was left whole for a later
   coherent pass and the run moved to the two model families that follow.
3. **Took the whole `daikonbouBig` family.** Both constructors allocate
   `0x6A8` bytes and install class tables `0x80B49884`/`0x80B49CB4`; both
   binders load `g3d/daikonbouBig.brres` (`0x80B49854`, `0x80B49C84`) and the
   `spine` node (`0x80B4987C`, `0x80B49CAC`). The class label is `// inferred:`
   from those strings.
4. **Took the whole `kinopio_balloon` block.** `0x80A6BA60` allocates
   `0x630` bytes and installs `0x80B4A110`; `0x80A6C0E0` loads
   `g3d/kinopio_balloon.brres` (`0x80B4A0C8`) and the `wait` clip
   (`0x80B4A0F8`). `0x80A6BE90` is the green-coin sparkle/collection handler
   that retires the balloon and spawns `Wm_ob_greencoinkira`.
5. **Excluded every export with damaged control flow.** `0x80A673D0` and
   `0x80A682A0` contain GQR/`ldexpf` artefacts; `0x80A6C9A0` and
   `0x80A6EFB0` also have GQR or `halt_baddata()` damage. None were guessed.
6. **Reused only established helpers.** `0x8015F800`, `0x802B9280`,
   `0x801628C0`, `0x80094E80`, `0x80069020`, `0x80169CD0`, `0x80168D10`,
   `0x8016A5E0`, `0x8008C200`, `0x80163...` and the animation binders are
   already used by earlier waves. The nine new opaque callees are kept by
   address.
7. **Read every numeric constant from the REL image.** `0x80B2D710 = 1.0`,
   `0x80B2D714 = 0.0`, `0x80B2D72C = 96.0`, `0x80B2D730 = 100.0`,
   `0x80B2D734 = 1728.0`, `0x80B2D738 = 288.0`, `0x80B2D7C8 = 1.0`,
   `0x80B2D7CC = 0.0`, `0x80B2D7E0 = 80.0`, `0x80B2D7E4 = 96.0`,
   `0x80B2D7E8 = 100.0`, `0x80B2D7EC = 1728.0`, `0x80B2D7F0 = 288.0`,
   `0x80B2D878 = 1.0`, `0x80B2D87C = 0.0`, `0x80B2D880 = 8.0`,
   `0x80B2D8A8 = 5500.0` and `0x80B2D8AC = 24.0` were read as big-endian
   `f32` values. The `0x80B2D888` table holds the frame ids `8..15`.

## Translated functions

| Address | Rust name | Role | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80A69010` | `fn_80a69010` | Dispose/registration callback for the first `daikonbouBig` record | translated | high | `80a69010_FUN_80a69010.c` |
| `0x80A69190` | `fn_80a69190` | Allocate/construct the first `0x6A8`-byte `daikonbouBig` actor | translated | high | `80a69190_FUN_80a69190.c` |
| `0x80A69270` | `fn_80a69270` | Load `daikonbouBig` and bind model/`spine` | translated | high | `80a69270_FUN_80a69270.c` |
| `0x80A695A0` | `fn_80a695a0` | Height above the ground probe, clamped to `0.0` | translated | high | `80a695a0_FUN_80a695a0.c` |
| `0x80A69600` | `fn_80a69600` | Distance outside a `96.0`-unit horizontal band | translated | high | `80a69600_FUN_80a69600.c` |
| `0x80A69670` | `fn_80a69670` | Spawn effect `0x2E4`/`0x2DF` from height and band | translated | high | `80a69670_FUN_80a69670.c` |
| `0x80A697A0` | `fn_80a697a0` | Spawn effect `0x2E5`/`0x2E0` on the actor id | translated | high | `80a697a0_FUN_80a697a0.c` |
| `0x80A69A00` | `fn_80a69a00` | Spawn effect `0x2E7`/`0x2E2` on the actor id | translated | high | `80a69a00_FUN_80a69a00.c` |
| `0x80A6A290` | `fn_80a6a290` | Dispose/registration callback for the second record | translated | high | `80a6a290_FUN_80a6a290.c` |
| `0x80A6A350` | `fn_80a6a350` | Constant producer `0x80353C64` | translated | high | `80a6a350_FUN_80a6a350.c` |
| `0x80A6A360` | `fn_80a6a360` | Constant producer `0x80353BE4` | translated | high | `80a6a360_FUN_80a6a360.c` |
| `0x80A6A450` | `fn_80a6a450` | Walk the `+0x30` child chain until the vtable filter accepts | translated | medium | `80a6a450_FUN_80a6a450.c` |
| `0x80A6A530` | `fn_80a6a530` | Compare the state-name suffix after the last `:` | translated | medium | `80a6a530_FUN_80a6a530.c` |
| `0x80A6A650` | `fn_80a6a650` | Allocate/construct the second `0x6A8`-byte actor | translated | high | `80a6a650_FUN_80a6a650.c` |
| `0x80A6A730` | `fn_80a6a730` | Load `daikonbouBig` and bind model/`spine`, second variant | translated | high | `80a6a730_FUN_80a6a730.c` |
| `0x80A6A9C0` | `fn_80a6a9c0` | Scene-band predicate with an `80.0` margin | translated | medium | `80a6a9c0_FUN_80a6a9c0.c` |
| `0x80A6AA60` | `fn_80a6aa60` | Distance outside a `96.0`-unit band, second variant | translated | high | `80a6aa60_FUN_80a6aa60.c` |
| `0x80A6AAD0` | `fn_80a6aad0` | Height above the ground probe, second variant | translated | high | `80a6aad0_FUN_80a6aad0.c` |
| `0x80A6AB30` | `fn_80a6ab30` | Spawn effect `0x2E4`/`0x2DF`, second variant | translated | high | `80a6ab30_FUN_80a6ab30.c` |
| `0x80A6AC60` | `fn_80a6ac60` | Spawn effect `0x2E5`/`0x2E0`, second variant | translated | high | `80a6ac60_FUN_80a6ac60.c` |
| `0x80A6AEC0` | `fn_80a6aec0` | Spawn effect `0x2E7`/`0x2E2`, second variant | translated | high | `80a6aec0_FUN_80a6aec0.c` |
| `0x80A6B420` | `fn_80a6b420` | Release every holder and optionally free the actor | translated | high | `80a6b420_FUN_80a6b420.c` |
| `0x80A6BA60` | `fn_80a6ba60` | Allocate/construct the `0x630`-byte `kinopio_balloon` actor | translated | high | `80a6ba60_FUN_80a6ba60.c` |
| `0x80A6BD30` | `fn_80a6bd30` | Place the balloon model from position and Euler angles | translated | medium | `80a6bd30_FUN_80a6bd30.c` |
| `0x80A6BE30` | `fn_80a6be30` | Contact callback: copy the player type byte to `+0x622` | translated | medium | `80a6be30_FUN_80a6be30.c` |
| `0x80A6BE90` | `fn_80a6be90` | Green-coin sparkle/collection, sound table and retire | translated | high | `80a6be90_FUN_80a6be90.c` |
| `0x80A6C0E0` | `fn_80a6c0e0` | Load `kinopio_balloon` and bind the `wait` clip | translated | medium | `80a6c0e0_FUN_80a6c0e0.c` |
| `0x80A6C2F0` | `fn_80a6c2f0` | Spawn `Wm_en_burst_ss`, effect `0x244`, switch state | translated | high | `80a6c2f0_FUN_80a6c2f0.c` |
| `0x80A6C3E0` | `fn_80a6c3e0` | Run the coin handler then advance the three holders | translated | high | `80a6c3e0_FUN_80a6c3e0.c` |
| `0x80A6C6C0` | `fn_80a6c6c0` | Dispose/registration callback for the balloon record | translated | high | `80a6c6c0_FUN_80a6c6c0.c` |

## Deferred neighbours

### Clean blocks left whole

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A67A60`..`0x80A68A50` | 4,060 B | Coherent `goal_set`/four-slot object block: 14 clean bodies plus two damaged exports, needs a dedicated pass |
| `0x80A673D0` | 1,080 B | GQR/`ldexpf` artefacts and one removed unreachable block |

### Duplicate or trivial exports inside the touched ranges

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A6A2F0` | 92 B | Byte-identical dispose callback of `0x80A6A290` |
| `0x80A6B6A0` | 88 B | Dispose callback duplicate |
| `0x80A6B700` | 92 B | Dispose callback duplicate |
| `0x80A6B760` | 12 B | Constant producer `0x80353C64` |
| `0x80A6B770` | 12 B | Constant producer `0x80353BE4` |
| `0x80A6B860` | 224 B | Byte-identical chain walker of `0x80A6A450` |
| `0x80A6B940` | 136 B | Byte-identical name predicate of `0x80A6A530` |
| `0x80A6C7E0` | 48 B | `0x802DCBFC` forwarder with a hidden `r12` operand |

### Damaged exports after the selected range

| Address | Size | Reason deferred |
|---|---:|---|
| `0x80A6C9A0` | 228 B | GQR/`ldexpf` artefacts and one removed unreachable block |
| `0x80A6EFB0` | 100 B | Export ends in `halt_baddata()` |
| `0x80A6F8C0` | 16 B | Soft-float constant return prints an unusable `double` expression |

## Unknowns preserved in code

1. **Class labels.** Both model names and class-table addresses are explicit,
   but no `StateID` string is referenced by either block. The labels are
   `// inferred:` from the model strings; the profile registry in
   `d_profileNP` remains the authority.
2. **The two `daikonbouBig` variants.** They allocate the same `0x6A8` bytes
   and bind the same model/node, but use different class tables and different
   blend weights. Their state ownership is not recoverable from this export.
3. **`main.dol` pointer variables.** `0x80429D40`, `0x80429D5C`, `0x80429D60`,
   `0x80429E00` and `0x80429F10` are pointer variables in BSS; the Rust
   dereferences them once, matching the instruction stream. `0x80429CFA` is a
   plain byte flag.
4. **`0x801C0BD0` ABI.** The export prints three `double` arguments followed
   by a matrix pointer. The Rust models that shape with a local helper; the
   exact parameter types are inferred from the cast pattern.
5. **`0x800B34D0` role.** The call takes a position, a word from the
   `0x80B2D888` frame table and the sign-extended `+0x622` direction byte. The
   "frame/facing offset" name is inferred from its call site.
6. **Green-coin slot.** The global counter at `0x80429F10 + 0x134` is clamped
   to `8` and used as a 1-based table index; the meaning of the counter is
   inferred, not named by the export.
7. **Chain-walker receiver.** `0x80A6A450` calls the vtable slot with the child
   as `r3`; the receiver is inferred from the register setup because Ghidra
   prints the call with no arguments.
8. **Hidden actor parameter.** `0x80A6C0E0` reads its receiver from the
   compiler save fragment `0x802DCD74`; the Rust makes it an explicit
   `actor` parameter and omits the fragment.

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
`ledger/agents/wave23-enemies.csv`.
