# Coverage - wave 25 (`d_basesNP` ship tail, Larry/Lemmy bosses and lifts)

## How to read this file

This page records the twenty-fifth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - *Translated functions* lists the 30 bodies appended
   to `src/game/bases.rs`: the `KoopaShip` transform/callback-array tail, eight
   Larry boss bodies, five Lemmy axle bodies and eleven `test_lift`/`lift_dokan`
   elevator bodies.
2. **Why these functions?** - *Selection* explains how the pass continued the
   address-ordered sweep above the wave-24 window and which decompiler outputs
   were skipped.
3. **What is still missing?** - *Deferred candidates* records the duplicate
   release helpers, the paired-single reconstruction and the large player tick
   that need their own pass.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, table meaning or
  called body is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8006xxxx`, `0x8016xxxx`, `0x8023xxxx`, `0x802Dxxxx`, `0x8032xxxx`,
`0x8037xxxx`, `0x8042xxxx`, `0x807Fxxxx`, `0x8082xxxx`) are imported absolute
values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output (or in the instruction stream where the C drops an argument);
`// inferred:` = role guessed from callers/shape; `// unknown:` = unresolved;
`// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 25 |
|---|---:|
| New translated functions | 30 |
| `KoopaShip` tail (`0x806BF2C0..0x806C0010`) | 6 |
| Larry boss cluster (`0x806C1820..0x806C3F00`) | 8 |
| Lemmy axle actor (`0x806C5DB0..0x806C6680`) | 5 |
| `test_lift` / `lift_dokan` (`0x806C76A0..0x806C8AD0`) | 11 |
| Selected bytes | 5,216 |
| Translated functions in `src/game/bases.rs` (cumulative ledger) | 611 |
| Cumulative selected bytes | 205,392 (18.51 % of indexed bytes) |

The 30 new bodies select 5,216 of the 1,109,860 indexed bytes (0.47 %) and 30 of
the 5,409 eligible entries (0.55 %). 29 rows are `high` confidence; one,
`0x806C7BA0`, is `medium` because it calls the untranslated probe `0x806C7890`.
No row is `partial` or `stub`.

## Selection

The pass started from `functions.tsv`, the existing `bases.rs` and every
`ledger/agents/*.csv` row with `module = game::bases`. All addresses already
translated by waves 2-24 were removed before selection. Wave 24 stopped at
`0x806BFB80`; this pass starts at `0x806BF2C0` (the ship helpers wave 24
deferred) and moves upward to `0x806C8AD0`.

1. **Finish the deferred ship bodies.** `0x806BF2C0`/`0x806BF340` are the two
   small `KoopaShip` transform helpers listed as deferred in wave 24;
   `0x806BFD50` is their shared matrix update, `0x806BFDF0` the
   `Position_Peach` bone copy, `0x806BFF00` the jump actor's two-array
   initialiser and `0x806C0010` the ship's four-array initialiser.
2. **Follow the boss actor that starts at `0x806C1820`.** Its start-up body
   (`0x806C1820`) introduces the collision triangle `+0x1EC` and the
   attack-variant index `+0x484`; `0x806C1CC0` resolves the
   `larry_attack` bone table, `0x806C2140` spawns the burst and fire-snake
   effects, and `0x806C3990`/`0x806C3DA0` are the `wait_ball`/ball-drop pair.
3. **Take the Lemmy axle actor as a complete lifecycle.** `0x806C5DB0` and
   `0x806C62D0` are the heap and in-place constructors (vtables `0x807F8CC0`
   and `0x807F8A10`); `0x806C5EE0`/`0x806C6480` load
   `g3d/boss_lemmy_ashiba.brres` with the `main` and root pane names;
   `0x806C6680` is the frame update.
4. **Take the elevator family.** `0x806C76A0`/`0x806C77E0` are the
   `test_lift` model tick and loader; `0x806C7A20`/`0x806C7A70`/`0x806C7BA0`
   are its angle step, player attach and ride-count check;
   `0x806C85D0`/`0x806C86B0`/`0x806C87B0`/`0x806C87E0`/`0x806C8A20`/
   `0x806C8AD0` are the two-part `lift_dokan` loader, transform, offset
   accumulator and frame callbacks.
5. **Rejected decompiler failures.** `0x806C1040` has an unrecovered `r4`
   (`extraout_r4`); `0x806C3790` is a paired-single reconstruction
   (`ldexpf`, `unaff_GQR0`); `0x806C7C20` is a 668-byte player tick dominated
   by double conversions and was left for a dedicated pass.

## Clusters

### `KoopaShip` tail (`0x806BF2C0..0x806C0010`)

- `0x806BF2C0` clears the 16-bit angle `+0x102`, runs `0x800E81C0` twice
  (`+0x5B0` and `+0x614`, two entries each) and overwrites the first word of
  the second block with `0x807CAE4C`.
- `0x806BF340` runs the nested transform virtual `+0x1C`, updates `+0x5B0`,
  and writes `+0x478 + 0x800E85A0(+0x5B0, +0x564)` to the height `+0xB0`.
- `0x806BFD50` is the shared matrix builder: `+0x7C` from
  `+0xAC..+0xB4`, then the angles `+0x104`/`+0x100`/`+0x102`, then the model
  binds `0x8016A170`/`0x8016A150`/`0x8016A1A0`.
- `0x806BFDF0` is the `Position_Peach` bone copy to `+0x8E0`; the C drops the
  receiver of `0x806BFD50`, and the instruction stream (`0x806BFE04`) passes
  `r31` in `r3`, so the Rust passes the actor.
- `0x806BFF00` builds two 2-entry `0x18`-stride callback arrays at
  `+0x5D8`/`+0x63C`; `0x806C0010` builds four 4-entry arrays at
  `+0x690`/`+0x724`/`+0x7B8`/`+0x84C` and seeds 29 words from
  `0x807CAE88..0x807CAEB0`.

### Larry boss cluster (`0x806C1820..0x806C3F00`)

- `0x806C1820` is the start-up body: virtual `+0x118` cached at `+0x484`,
  virtuals `+0x130`/`+0x110`, the constant at `0x807CAFE4` at `+0xDC`, the
  collision triangle `+0x1EC` and sphere `+0x148`, seven seeds at
  `+0x318..+0x32C`, the current player position at `+0x468`, flag bit `0x80`,
  and interface `DAT_8082CB04`.
- `0x806C1CC0` indexes the `s_larry_attack_807F7B38` table with `+0x484`,
  resolves the bone and copies 60 bytes of transform into `+0x474`.
- `0x806C2140` emits `Wm_en_burst_m` and `Wm_en_firesnk_icehitsmk_h` and queues
  sound `0x51F`; `0x806C2530` resets the collision sphere and queues `0x51D`.
- `0x806C3010` is a `0x84`-byte helper constructor (vtable `0x807F8068`).
- `0x806C3990` enters `wait_ball` (pointer `0x807F8130` at `+0x824`), and
  `0x806C3DA0` drops the ball and spawns `Wm_le_ballice`.
- `0x806C3F00` registers the three frame callbacks
  `0x806B4A10`/`0x806C3710`/`0x806C3790` through `0x8007FB80`.

The class attribution is `// inferred:` from the `larry_attack` and
`Wm_le_ballice` strings; the REL carries no class name for it.

### Lemmy axle actor (`0x806C5DB0..0x806C6680`)

- `0x806C5DB0` allocates `0x6A8` bytes, runs `0x80094E80`, installs vtable
  `0x807F8CC0` and constructs `+0x524`/`+0x544`/`+0x588`/`+0x594`/`+0x5B4`.
- `0x806C62D0` is the in-place twin with vtable `0x807F8A10`. It clears
  `+0x5B0` before rewriting `+0x588`, unlike the heap constructor.
- `0x806C5EE0` opens `s_g3d_boss_lemmy_ashiba_brres_807F89C8` and binds the
  `main` pane; `0x806C6480` repeats the body with the root name
  `s_boss_lemmy_ashiba_807F89E4`.
- `0x806C6680` advances the `+0x5C0` animation and runs virtuals `+0x10` and
  `+0x2AC`.

### `test_lift` / `lift_dokan` (`0x806C76A0..0x806C8AD0`)

- `0x806C77E0` opens `s_g3d_test_lift_brres_807F9164` and binds `s_test_lift`
  into `+0x3F0`; `0x806C76A0` rebuilds its matrix every other animation phase.
- `0x806C7A20` is a `0x100`-step angle clamp; `0x806C7A70` reparents the four
  players whose `+0x2A78` owner matches; `0x806C7BA0` checks the ride count.
- `0x806C85D0` opens `s_g3d_lift_dokan_brres_807F93A0` and binds the two
  `s_lift_dokan_A` panes at `+0x3F0`/`+0x3FC`; `0x806C86B0` positions both
  with offsets `0x807CB154`/`0x807CB158`.
- `0x806C87B0`/`0x806C87E0` move the signed offset `+0x564` into x or y;
  `0x806C8A20` is the frame callback and `0x806C8AD0` advances the animation
  and spin angle.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave25-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806BF2C0` | `fn_806bf2c0` | reset the ship transform registers and two matrix blocks | translated | high | `806bf2c0_FUN_806bf2c0.c` |
| `0x806BF340` | `fn_806bf340` | advance the ship transform and offset its height | translated | high | `806bf340_FUN_806bf340.c` |
| `0x806BFD50` | `fn_806bfd50` | build the shared ship model matrix from position and angles | translated | high | `806bfd50_FUN_806bfd50.c` |
| `0x806BFDF0` | `fn_806bfdf0` | copy the player's `Position_Peach` bone into the ship | translated | high | `806bfdf0_FUN_806bfdf0.c` |
| `0x806BFF00` | `fn_806bff00` | initialise the jump actor's two 2-entry callback arrays | translated | high | `806bff00_FUN_806bff00.c` |
| `0x806C0010` | `fn_806c0010` | build the ship's four 4-entry callback arrays | translated | high | `806c0010_FUN_806c0010.c` |
| `0x806C1820` | `fn_806c1820` | start the Larry boss actor and bind its collision blocks | translated | high | `806c1820_FUN_806c1820.c` |
| `0x806C1CC0` | `fn_806c1cc0` | look up the Larry attack bone by variant | translated | high | `806c1cc0_FUN_806c1cc0.c` |
| `0x806C2140` | `fn_806c2140` | spawn the burst/fire-snake effects and sound `0x51F` | translated | high | `806c2140_FUN_806c2140.c` |
| `0x806C2530` | `fn_806c2530` | reset the collision sphere and queue sound `0x51D` | translated | high | `806c2530_FUN_806c2530.c` |
| `0x806C3010` | `fn_806c3010` | allocate a `0x84`-byte helper with vtable `0x807F8068` | translated | high | `806c3010_FUN_806c3010.c` |
| `0x806C3990` | `fn_806c3990` | enter the `wait_ball` state of the Larry actor | translated | high | `806c3990_FUN_806c3990.c` |
| `0x806C3DA0` | `fn_806c3da0` | drop the ball and spawn `Wm_le_ballice` | translated | high | `806c3da0_FUN_806c3da0.c` |
| `0x806C3F00` | `fn_806c3f00` | register the three Larry frame callbacks | translated | high | `806c3f00_FUN_806c3f00.c` |
| `0x806C5DB0` | `fn_806c5db0` | allocate and construct the Lemmy axle actor (`0x6A8`) | translated | high | `806c5db0_FUN_806c5db0.c` |
| `0x806C5EE0` | `fn_806c5ee0` | load `boss_lemmy_ashiba` and bind the `main` pane | translated | high | `806c5ee0_FUN_806c5ee0.c` |
| `0x806C62D0` | `fn_806c62d0` | construct the Lemmy axle actor in place | translated | high | `806c62d0_FUN_806c62d0.c` |
| `0x806C6480` | `fn_806c6480` | load `boss_lemmy_ashiba` and bind the root pane | translated | high | `806c6480_FUN_806c6480.c` |
| `0x806C6680` | `fn_806c6680` | run the Lemmy axle frame update | translated | high | `806c6680_FUN_806c6680.c` |
| `0x806C76A0` | `fn_806c76a0` | lift the `test_lift` model on even animation phases | translated | high | `806c76a0_FUN_806c76a0.c` |
| `0x806C77E0` | `fn_806c77e0` | load `test_lift` and bind its pane | translated | high | `806c77e0_FUN_806c77e0.c` |
| `0x806C7A20` | `fn_806c7a20` | step a 16-bit angle toward its target by `0x100` | translated | high | `806c7a20_FUN_806c7a20.c` |
| `0x806C7A70` | `fn_806c7a70` | attach the four players to the `test_lift` platform | translated | high | `806c7a70_FUN_806c7a70.c` |
| `0x806C7BA0` | `fn_806c7ba0` | notify the ride state machine when the count is reached | translated | medium | `806c7ba0_FUN_806c7ba0.c` |
| `0x806C85D0` | `fn_806c85d0` | load the two `lift_dokan` model parts | translated | high | `806c85d0_FUN_806c85d0.c` |
| `0x806C86B0` | `fn_806c86b0` | position both `lift_dokan` parts from the actor state | translated | high | `806c86b0_FUN_806c86b0.c` |
| `0x806C87B0` | `fn_806c87b0` | copy x or y into the offset accumulator `+0x564` | translated | high | `806c87b0_FUN_806c87b0.c` |
| `0x806C87E0` | `fn_806c87e0` | apply the offset plus speed to x or y | translated | high | `806c87e0_FUN_806c87e0.c` |
| `0x806C8A20` | `fn_806c8a20` | frame callback that repositions both lift parts | translated | high | `806c8a20_FUN_806c8a20.c` |
| `0x806C8AD0` | `fn_806c8ad0` | advance the lift offset and spin angle | translated | high | `806c8ad0_FUN_806c8ad0.c` |

**Totals:** 30 translated, 0 partial, 0 stub (29 high, 1 medium).

## Deferred candidates

| Address | Size | Why deferred |
|---|---:|---|
| `0x806BF4B0`, `0x806BF5D0` | 88, 48 | Release helper and a `0x802DCBFC` thunk; duplicates of [`fn_806ba180`]. |
| `0x806BF600`, `0x806BF630` | 48 each | Deleting-destructor thunks around `0x802DCBFC`; no independent behaviour. |
| `0x806BF660`, `0x806BF670` | 12 each | Return the constant `0x80377A04`; getter veneers. |
| `0x806C07D0`, `0x806C0830`, `0x806C0C30`, `0x806C0C70`, `0x806C0CD0`, `0x806C6E40`, `0x806C6F00`, `0x806C8200`, `0x806C84C0`, `0x806C8500`, `0x806C8560` | 64..100 | Release/destroy helpers; the same null-gate + `0x8015F800` + `0x802B9280` shape as the already translated [`fn_806ba180`]. |
| `0x806C0970`, `0x806C5B80`, `0x806C7040`, `0x806C2E10` | 224 each | Five-link `+0x30` recursive walks; duplicates of [`fn_806bc410`]. |
| `0x806C0A50`, `0x806C5C60` | 136 each | Name-suffix compares after the first `:`; duplicate of [`fn_806bc4f0`]. |
| `0x806C1040` | 676 | **Rejected**: the smooth-curve body reads an unrecovered `r4` (`extraout_r4`). |
| `0x806C3790` | 388 | **Rejected**: paired-single reconstruction (`ldexpf`, `unaff_GQR0`). |
| `0x806C7C20` | 668 | Player tick dominated by double conversions and the `0x80429E98`/`0x8042A1C8` tables; needs a dedicated FP pass. |
| `0x806C8880` | 308 | Actor init with a 12-argument `0x8007FA40` registration; reuse the `call12_f64x4` wrapper from `game::enemies` first. |
| `0x806C9100` | 176 | `s_g3d_lift_zen_yogan_down_brres_807F96CC` loader; next elevator family. |
| `0x806C97D0` | 360 | Untouched body above this pass's window. |

## Unknowns preserved

1. **`0x806B5180`.** Called by [`fn_806c3990`]; its export is a rejected
   paired-single reconstruction, so the Rust calls the address with an
   `// unknown:` note instead of inventing its effect.
2. **`0x806C7890`.** Called by [`fn_806c7ba0`] with three stack outputs; the
   body is untranslated and the buffer widths are read from the frame layout
   (`sp+0x10` twelve bytes, `sp+0x0C` and `sp+0x08` four bytes each). This is
   why that row is `medium`.
3. **`0x800E81C0` element width.** [`fn_806bf2c0`] passes a count of `2` for
   each block; whether an entry is a word or a 16-bit pair is not recovered.
4. **Dropped receivers.** `0x806BFDF0` and `0x806C8A20` call their target with
   no argument in the C. The instruction stream shows `r31`/the incoming `r3`
   unchanged at the call, so the Rust passes the actor and marks it
   `// observed:`.
5. **Class attribution.** The Larry, Lemmy and lift actors carry no
   `s_<Class>_c__` string in this window. `boss_lemmy_ashiba` and `lift_dokan`
   are `// inferred:` from the BRRES file names; the Larry actor is inferred
   from `s_larry_attack_807F7B38` and `Wm_le_ballice`.
6. **Callback bodies.** The three Larry callbacks registered by
   [`fn_806c3f00`] are stored as raw addresses; `0x806C3710` is untranslated
   and `0x806C3790` is the rejected paired-single body.
7. **`0x8007FB80` argument widths.** The C prints ten arguments; the
   instruction stream loads `f1`..`f3` and `r3`..`r9`, which the local
   `call_f32x3_u32x7` wrapper models.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` contains 611 translated ledger rows after this pass; this
pass adds 30, all with `# Original` blocks and the synthetic-base warning.
`node tools/doc-lint.mjs src` reports `ok: true`, 0 violations over 68 files.
`cargo check` exits 0; the remaining warnings are in other lanes' files
(`src/runtime/bt.rs`, `src/runtime/input.rs`).
