# Coverage - wave 5 (`d_basesNP` select screens and `daYoshiEgg_c`/`daShipGear_c`)

## How to read this file

This page records the fifth translation pass over `d_basesNP`, the base gameplay
REL (`game::bases`). It answers:

1. **What was added?** - the table below lists the 30 functions appended to
   `src/game/bases.rs`, with their synthetic addresses, Rust names,
   plain-English roles, status, confidence, and Ghidra evidence.
2. **Why these functions?** - the *Selection* section explains how the
   `dNumberOfPeopleChange_c` screen, the `dSelectPlayer_c` screen, the
   six-state `daYoshiEgg_c` actor, and the single-state `daShipGear_c` actor
   were chosen.
3. **What is still missing?** - *Deferred candidates* lists the neighbouring
   functions intentionally left for later, and *Unknowns preserved* records
   every unresolved address or role instead of guessing it.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the *evidence*: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role, ownership, or
  hidden ABI operand is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8000xxxx`, `0x8037xxxx`, `0x8042xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 4 | Wave 5 | Cumulative |
|---|---:|---:|---:|
| Translated functions in `game/bases.rs` | 83 | 30 | 113 |
| `dNumberOfPeopleChange_c` screen | 0 | 9 | 9 |
| `dSelectPlayer_c` screen | 0 | 8 | 8 |
| `daYoshiEgg_c` actor | 0 | 10 | 10 |
| `daShipGear_c` actor | 0 | 3 | 3 |
| State registrations covered | 41 | 26 | 67 |
| Selected bytes | 19,260 | 9,212 | 28,472 |
| Coverage of indexed bytes | 1.73 % | 0.83 % | 2.57 % |
| Coverage of eligible entries (5,409) | 1.53 % | 0.55 % | 2.09 % |

The module still contains 5,615 function entries and 79 `StateID`-bearing
files. This pass deliberately takes four coherent screen/actor clusters, not a
range. Confidence is **23 high** and **7 medium**; there are no `partial` or
`stub` rows.

## Selection

The pass was evidence-first and cluster-first:

1. **Started from `functions.tsv` and the existing `bases.rs`.** The 83
   wave-2/3/4 addresses were removed from the candidate set before selection.
2. **Swept the remaining class/state-name files.** The four clusters were
   chosen because each has a complete, self-contained story: a multi-state
   registration table, constructors/destructors, and a small set of input,
   layout, or update helpers.
3. **Required complete control flow.** Functions whose only body is a
   `0x802DCBFC` forwarder were rejected (`0x80634C90`, `0x80634CC0`,
   `0x807A4F00`, `0x807A4F30`, `0x80735260`, `0x80735290`). The paired-single
   reconstruction at `0x807A58E0` was rejected again; it is called by address
   from [`fn_807a5bc0`](../src/game/bases.rs).
4. **Verified the registration template.** Every selected table builds 0x30-byte
   descriptors at `descriptor - 0x10` nodes, uses the same nine-word parameter
   order `6,7,8,3,4,5,0,1,2`, and calls [`fn_80600160`](../src/game/bases.rs).
   The existing `register_state_desc` helper covers all four tables without a
   second template.
5. **Verified the class names against the state strings.** The eleven
   `dNumberOfPeopleChange_c::StateID_*`, eight `dSelectPlayer_c::StateID_*`,
   six `daYoshiEgg_c::StateID_*`, and one `daShipGear_c::StateID_Move` strings
   were read from the REL data section, not guessed. Constructors that have no
   string of their own carry `// inferred:` notes.

## Clusters

### `dNumberOfPeopleChange_c` player-count screen (`0x80634CF0..0x80635EA0`)

The pre-stage "number of people" selector. The eleven states are
`InitialSetup`, `OnStageAnimeEndWait`, `InfoOnStageAnimeEndWait`,
`NowEntrantRecruit`, `StoryModeCourseSelectExitWait`, `InfoWindowExitWait`,
`ButtonOnStageAnimeEndWait`, `ButtonChangeAnimeEndWait`, `ButtonSelect`,
`ButtonDecision`, and `ExitAnimeEndCheck`. Shared vtable `0x807DE514`, callback
`0x80635580`.

- `fn_80634cf0` also performs a one-time copy of the two section-4 words at
  `0x807C7C34`/`0x807C7C38` into the BSS pair `0x807DDA74`/`0x807DDA78`,
  guarded by `0x80826878`.
- `fn_806356a0` constructs the screen: vtable `0x807DE588` at `+0x60`, four
  interface vtables at `+0x70..+0x7C`, self pointer at `+0x80`, interface
  object at `+0x88`, and the published global `0x80826890`.
- `fn_806357a0`, `fn_806357e0` and `fn_80635840` are the registered
  destructor-shaped helpers. `fn_806357e0` and `fn_80635840` are byte-identical;
  both bodies are kept because the REL keeps both entries.
- `fn_80635b10` and `fn_80635d00` handle the confirm mask (`0x800B63A0` with
  masks 8/4), query the view object with `0x900`, and dispatch the state
  objects at `0x80826960` / `0x80826A20`.
- `fn_80635ea0` requests exit through the animation object at `+0xB0` and sets
  the global byte at `0x80429F7C + 0xB5A` (`0x8042AAD6`).

### `dSelectPlayer_c` stage/player select screen (`0x8063F300..0x80640240`)

The stage/player selector used before a level. The eight states are
`StartWait`, `OnStageAnimeEndWait`, `InfoOnStageAnimeEndWait`,
`ButtonChangeAnimeEndWait`, `StartMemberSelect`, `MultiStartMemberSelect`,
`StartMemberButtonAnime`, and `ExitAnimeEndWait`. Shared vtable `0x807E158C`,
callback `0x8063FE80`.

- `fn_8063f300` moves or confirms the highlighted slot. The frame-record flags
  come from `0x80429F50[0x8042A464] + 4 + 0x1C`; bit 3 moves down, bit 2 moves
  up. A changed index dispatches `0x80827560`; an unchanged index confirms with
  the view object and then polls `0x800B53A0(0)` for cancel.
- `fn_8063f550` starts animation `0x17` and then `0x18` or `0x19`; `fn_8063f5c0`
  returns to `StartWait` when all three animations report idle.
- `fn_8063ffa0` constructs the screen (vtable `0x807E1A08`, layout at `+0x74`).
- `fn_80640180` loads `sequenceBG/sequenceBG.arc`, the `sequenceBG_00.brlyt`
  layout, nine `sequenceBG_00_inWindow.brlan` animations, `A00_Window`, and the
  0x1B `P_BG50` panes.
- `fn_80640240` walks the `msk_BG*.tpl` / `msk_cloud.tpl` table at `0x807E1970`,
  builds `sequenceBGTexture/<name>` in a 136-byte stack buffer, and applies each
  texture to the pane selected by the parallel index table at `0x807C81A4`.

### `daYoshiEgg_c` actor (`0x807A4930..0x807A5BC0`)

A six-state Yoshi egg. The states are `UpMove`, `DownMove`, `MultiJumpUp`,
`MultiJumpDown`, `ItemMove`, and `ItemCreate`. Shared vtable `0x8081CB7C`,
callback `0x807A4DE0`.

- `fn_807a4f60` allocates `0x500` bytes: base actor `0x80064110`, vtable
  `0x8081CC0C`, an interface at `+0x394`, type descriptor `0x80317860` at
  `+0x3B0`, a state/helper object at `+0x3D4`, and cleared fields through
  `+0x4F8`.
- `fn_807a55e0` constructs the smaller `0x418`-byte sibling (vtable
  `0x8081CE40`, helper at `+0x3D4`).
- `fn_807a5230` is the contact handler: it compares the first byte of the two
  virtual `+0x6C` name strings and either clears the collision bit or runs the
  hit effect.
- `fn_807a5410` plays `Wm_mr_yoshifirehit` + sound `0x266`, or
  `Wm_mr_yoshiicehit` + sound `0x269`, then calls `0x8008C380` and `0x80162510`.
- `fn_807a5830` reads the spawn word through `+0x3E8` and creates id `0x26B`
  (top nibble 2) or id `0x209`, copying `+0x404` into the new object's `+0x804`.
- `fn_807a5a00` advances the five-step input/release ladder against bits
  `1,2,4,8,0x10` of `0x8006A220`.
- `fn_807a5b20` ticks the exit timer at `+0x3FC` and dispatches `0x808332A0` at
  zero. `fn_807a5bc0` accumulates the rotation at `+0x3F0`, wraps at the
  full-turn constant `0x807D0540`, calls the untranslated `0x807A58E0`, and
  dispatches `0x808332E0` after the target count.

### `daShipGear_c` actor (`0x80735030..0x807352C0`)

A single-state ship gear. The one state is `StateID_Move`; vtable `0x8080DA18`,
callback `0x80735140`.

- `fn_80735030` registers the one state descriptor at `0x80830F20` with the
  parameter block at `0x8080D9D8` and node `0x80830F10`.
- `fn_80735140` is the registered destructor.
- `fn_807352c0` allocates `0x3D0` bytes, runs base constructor `0x80066FC0`,
  and installs vtable `0x8080DA60`.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave5-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80634CF0` | `fn_80634cf0` | register the eleven `dNumberOfPeopleChange_c` states and one-time words | translated | high | `80634cf0_FUN_80634cf0.c` |
| `0x80635580` | `fn_80635580` | destroy a registered player-count state | translated | high | `80635580_FUN_80635580.c` |
| `0x806356A0` | `fn_806356a0` | construct the player-count screen | translated | high | `806356a0_FUN_806356a0.c` |
| `0x806357A0` | `fn_806357a0` | release a state without the interface destructor | translated | high | `806357a0_FUN_806357a0.c` |
| `0x806357E0` | `fn_806357e0` | destroy the `+0x18` sub-object and optionally free | translated | high | `806357e0_FUN_806357e0.c` |
| `0x80635840` | `fn_80635840` | byte-identical twin of `0x806357E0` | translated | high | `80635840_FUN_80635840.c` |
| `0x80635B10` | `fn_80635b10` | confirm input and dispatch state object `0x80826960` | translated | medium | `80635b10_FUN_80635b10.c` |
| `0x80635D00` | `fn_80635d00` | alternate confirm input and dispatch | translated | medium | `80635d00_FUN_80635d00.c` |
| `0x80635EA0` | `fn_80635ea0` | request exit through the animation object | translated | high | `80635ea0_FUN_80635ea0.c` |
| `0x8063F300` | `fn_8063f300` | move/confirm the highlighted player slot | translated | medium | `8063f300_FUN_8063f300.c` |
| `0x8063F550` | `fn_8063f550` | start entry animation `0x17` and `0x18`/`0x19` | translated | high | `8063f550_FUN_8063f550.c` |
| `0x8063F5C0` | `fn_8063f5c0` | return to `StartWait` when all animations idle | translated | high | `8063f5c0_FUN_8063f5c0.c` |
| `0x8063F850` | `fn_8063f850` | register the eight `dSelectPlayer_c` states | translated | high | `8063f850_FUN_8063f850.c` |
| `0x8063FE80` | `fn_8063fe80` | destroy a registered stage-select state | translated | high | `8063fe80_FUN_8063fe80.c` |
| `0x8063FFA0` | `fn_8063ffa0` | construct the `dSelectPlayer_c` screen | translated | high | `8063ffa0_FUN_8063ffa0.c` |
| `0x80640180` | `fn_80640180` | load and bind the `sequenceBG` layout | translated | high | `80640180_FUN_80640180.c` |
| `0x80640240` | `fn_80640240` | load `sequenceBGTexture` masks and apply them | translated | medium | `80640240_FUN_80640240.c` |
| `0x807A4930` | `fn_807a4930` | register the six `daYoshiEgg_c` states | translated | high | `807a4930_FUN_807a4930.c` |
| `0x807A4DE0` | `fn_807a4de0` | destroy a registered Yoshi-egg state | translated | high | `807a4de0_FUN_807a4de0.c` |
| `0x807A4F60` | `fn_807a4f60` | construct the `0x500`-byte Yoshi egg | translated | high | `807a4f60_FUN_807a4f60.c` |
| `0x807A5230` | `fn_807a5230` | handle a Yoshi-egg contact event | translated | medium | `807a5230_FUN_807a5230.c` |
| `0x807A5410` | `fn_807a5410` | play the fire/ice hit effect and sound | translated | high | `807a5410_FUN_807a5410.c` |
| `0x807A55E0` | `fn_807a55e0` | construct the `0x418`-byte egg sibling | translated | medium | `807a55e0_FUN_807a55e0.c` |
| `0x807A5830` | `fn_807a5830` | spawn from the egg's spawn record | translated | medium | `807a5830_FUN_807a5830.c` |
| `0x807A5A00` | `fn_807a5a00` | advance the input/release state ladder | translated | high | `807a5a00_FUN_807a5a00.c` |
| `0x807A5B20` | `fn_807a5b20` | tick the exit timer and dispatch | translated | high | `807a5b20_FUN_807a5b20.c` |
| `0x807A5BC0` | `fn_807a5bc0` | advance rotation and count completed turns | translated | high | `807a5bc0_FUN_807a5bc0.c` |
| `0x80735030` | `fn_80735030` | register `daShipGear_c::StateID_Move` | translated | high | `80735030_FUN_80735030.c` |
| `0x80735140` | `fn_80735140` | destroy the registered ship-gear state | translated | high | `80735140_FUN_80735140.c` |
| `0x807352C0` | `fn_807352c0` | construct a `daShipGear_c` actor | translated | high | `807352c0_FUN_807352c0.c` |

**Totals:** 30 translated, 0 partial, 0 stub. 23 high confidence, 7 medium
(`0x80635B10`, `0x80635D00`, `0x8063F300`, `0x80640240`, `0x807A5230`,
`0x807A55E0`, `0x807A5830`). The four registration functions add 26 state
descriptors (11 + 8 + 6 + 1).

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x80634C90`, `0x80634CC0` | byte-identical forwarders to the unresolved indirect stub `0x802DCBFC`. |
| `0x80640480` | adjacent screen constructor with vtable `0x807E1CA8` and layout at `+0x70`; the nearby `staffCredit` strings and layout offset do not match `dSelectPlayer_c`, so its class identity needs its own evidence. |
| `0x8063F450`, `0x8063F4E0` | sound/exit and exit-animation helpers next to the selected select-screen inputs; they call `0x8010C780` and the same `0x800C92F0` family and can be batched with the rest of the screen. |
| `0x80640530`..`0x80640EA0` | the remaining staff-credit/select-screen helpers before `0x806415D0`; same reason as `0x80640480`. |
| `0x807A4F00`, `0x807A4F30` | byte-identical forwarders to `0x802DCBFC`. |
| `0x807A57F0` | **rejected**: `halt_baddata` after the truncated input branch; the C cannot be represented faithfully until the instruction bytes are re-analysed. |
| `0x807A58E0` | paired-single reconstruction (`unaff_GQR0`, `ldexpf`); called by address from `fn_807a5bc0` with an explicit `// unknown:` note. |
| `0x807A5CC0` | another timer/exit helper immediately after `0x807A5BC0`; safe to batch with the next Yoshi-egg pass. |
| `0x80735260`, `0x80735290` | byte-identical `0x802DCBFC` forwarders. |
| `0x80651640` | still **rejected**: paired-single reconstruction from wave 3/4. |
| `0x806043C0`, `0x806083B0`, `0x8060CBE0`, `0x8060F010`, `0x80618D00` | remaining menu/state tables; unchanged from wave 4. |

## Unknowns preserved

1. **Input-mask semantics.** The `0x800B63A0(record, 8/4)` probes, the
   `0x900` argument to the view object's slot `+0x20`, and the
   `0x800B53A0(0)` cancel poll are called by address. Their exact button
   identities are inferred from the surrounding select-screen code, not from
   symbols.
2. **`0x80640240` helper identities.** `0x800046B4` (buffer clear),
   `0x802E1A68` (name join), `0x802B3300` (heap/allocator accessor),
   `0x802B4C60` (texture attach), and virtual slot `+0x68` on the pane object
   are described by shape only.
3. **`0x807A5410` effect APIs.** `0x8016C890` (effect spawn),
   `0x8006A4B0` (position conversion), `0x80198C30` (sound queue), and
   `0x8008C380`/`0x80162510` (cleanup/destructor) are called by address; their
   Nintendo names are not invented.
4. **`0x807A58E0`.** The paired-single reconstruction is not translated. The
   C prints `FUN_807a58e0()` without arguments, but [`fn_807a5bc0`] passes
   `actor` because the callee body uses `r3`.
5. **`0x80635EA0` global byte.** `0x80429F7C + 0xB5A` is written directly as
   the C shows (`0x8042AAD6`); the owning object and the byte's name are not
   resolved in this lane.
6. **`0x80634CF0` one-time words.** The BSS pair `0x807DDA74`/`0x807DDA78` is
   copied from `0x807C7C34`/`0x807C7C38`, but the meaning of the two words is
   unknown.
7. **`0x80640480` class identity.** Deferred rather than labelled
   `dSelectPlayer_c`: its vtable and layout offset differ, and the adjacent
   strings are `staffCredit`, so the class attribution is left open.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`cargo check` exits 0 for the whole crate. `doc-lint` reports `ok: true`, 0
violations over 68 files. `src/game/bases.rs` contains 113 translated
functions; this pass adds 30, all with `# Original` blocks and the synthetic-base
warning.
