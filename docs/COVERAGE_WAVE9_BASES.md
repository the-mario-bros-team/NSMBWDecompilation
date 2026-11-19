# Coverage - wave 9 (`d_basesNP` lift and single-state prop actors)

## How to read this file

This page records the ninth translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - the table below lists the 24 functions appended to
   `src/game/bases.rs`: eight StateID tables, their eight registered-state
   destructors, and their eight constructors.
2. **Why these functions?** - *Selection* explains why eight complete
   lifecycles were preferred over isolated tables.
3. **What is still missing?** - *Deferred candidates* records the nearby
   class whose constructor is a rejected paired-single reconstruction, and
   *Unknowns preserved* lists every unresolved call instead of guessing it.

Column meanings:

- **Status** - `translated` = control flow and every observed call/store are
  represented; `partial` = entry/exit and major effects only; `stub` = address
  only.
- **Confidence** - strength of the evidence: `high` = bytes/flow are explicit
  and cross-checked; `medium` = flow is explicit but a role or class
  attribution is inferred; `low` = mostly guesswork.
- **Evidence** - file under `work/ghidra/rel-d_basesNP/export/`; that C file is
  the source of truth.

**Synthetic base warning.** Every address on this page is an analysis address
from `tools/rel2elf.mjs`, using the canonical base `0x80600000`; the entry point
is `0x806000F0`. The REL has no embedded load address, and the retail loader may
place it anywhere. These are **not** runtime addresses. Main.dol addresses
(`0x8000xxxx`, `0x8032xxxx`, `0x8042xxxx`) are imported absolute values.

**Evidence markers in the Rust.** `// observed:` = visible in the Ghidra
output; `// inferred:` = role guessed from callers/shape; `// unknown:` =
unresolved; `// partial:` = deliberately incomplete.

## Inventory

| Metric | Wave 7 | Wave 9 | Cumulative (`game/bases.rs`) |
|---|---:|---:|---:|
| Translated functions in `src/game/bases.rs` | 30 | 24 | 167 |
| Actor classes with a complete lifecycle triple | 0 | 8 | 8 |
| State registrations covered | 40 | 10 | 50 |
| Selected bytes | 11,060 | 4,940 | - |

The 24 new bodies select 4,940 of the 1,109,860 indexed bytes (0.45 %) and
24 of the 5,409 eligible entries (0.44 %). Confidence is **16 high** and
**8 medium**; there are no `partial` or `stub` rows.

## Selection

This pass was evidence-first and lifecycle-first:

1. **Started from `functions.tsv` and the existing `bases.rs`.** Every address
   already translated in waves 2-7 was removed from the candidate set before
   selection.
2. **Swept the remaining `s_da<Class>_c__StateID_*` strings in the
   `0x806E0xxx..0x80787xxx` range.** The eight classes below were chosen because
   each has a compact, complete story: a state table with one or two states, a
   destructor that is exactly the callback stored in the table, and a
   constructor immediately adjacent to the table.
3. **Required complete control flow.** Functions whose C body contains
   `unaff_GQR*`, `ldexpf`, or other paired-single reconstruction were rejected.
   This is why `0x806DB340` is not in this pass (see *Deferred candidates*).
4. **Reused the verified registration template.** Every table builds a
   0x30-byte descriptor at stride 0x40, reads nine parameter words, installs
   the shared state vtable, and calls [`fn_80600160`](../src/game/bases.rs)
   with a 12-byte link node. The existing `register_state_desc` helper covers
   all 10 descriptors selected here.
5. **Verified class names against the state strings.** The StateID strings
   were read from the REL data section, not guessed. Constructors carry no
   class string of their own, so their attribution is marked `// inferred:`
   from the vtable they install and from adjacency to the table.

## Clusters

### Lift helpers (`0x806E0FB0..0x806E3970`)

- `daLiftSpinRotationSync_c` (`0x806E0FB0`): one `StateI...` state, state
  vtable `0x807FC7C0`; constructor `0x806E1240` (`0x438` bytes, class vtable
  `0x807FC808`).
- `daLiftWaterWheel_c` (`0x806E3630`): `StateID_NoDi...` and
  `StateID_Move...`, state vtable `0x807FCD88`; constructor `0x806E3970`
  (`0x778` bytes, class vtable `0x807FCDF8`, four `0x802DC9A0` arrays).

### Peach-castle and light props (`0x80720DF0..0x80721030`)

- `daPeachCastleSequenceMgrObj_c` (`0x80720DF0`): one truncated `s_...`
  state, state vtable `0x8080A1C0`; in-place constructor `0x80721030`
  (`0x8080A270`, later `0x8080A518`, self at `+0x648`).

### Single-state house/prop actors (`0x8072D970..0x8072DC00`)

- `daHouseLight_c` (`0x8072D970`): `StateID_Wait...`, state vtable
  `0x8080CB3C`; constructor `0x8072DC00` (`0x958` bytes, class vtable
  `0x8080CBF0`, interface vtable `0x8080CCD0` at `+0x3D0`, two arrays).

### Stand and torch props (`0x8073C970..0x8073CC40`, `0x80743CC0..0x80743F50`)

- `daStand_c` (`0x8073C970`): `StateID_Wait...`, state vtable `0x8080EDC0`,
  plus four tuning words copied to `0x80831298..0x808312A4`; constructor
  `0x8073CC40` (`0x908` bytes, class vtable `0x8080EFD8`).
- `daTorchiLight_c` (`0x80743CC0`): `StateID_Still...`, state vtable
  `0x80810448`; constructor `0x80743F50` (`0x3F0` bytes, class vtable
  `0x808104B4`).

### World-map and wire props (`0x80755290..0x80755290`, `0x80787630..0x80787990`)

- `daWireCircle_c` (`0x80755290`): `StateID_Move...`, state vtable
  `0x80813614`; constructor `0x80755520` (`0x4D8` bytes, class vtable
  `0x80813680`, model part at `+0x3F4`).
- `daWmYoganPillar_c` (`0x80787630`): `StateID_Initi...` and
  `StateID_Wait...`, state vtable `0x80819190`, plus a class-name node and
  five configuration copies; in-place constructor `0x80787990`
  (`0x808192F4`, interface vtable `0x807FEF40` at `+0x39C`, vtable
  `0x808193A8` at `+0x404`).

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave9-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x806E0FB0` | `fn_806e0fb0` | register the single `daLiftSpinRotationSync_c` state | translated | high | `806e0fb0_FUN_806e0fb0.c` |
| `0x806E10C0` | `fn_806e10c0` | destroy a registered lift-sync state | translated | high | `806e10c0_FUN_806e10c0.c` |
| `0x806E1240` | `fn_806e1240` | construct the `0x438`-byte lift-sync helper | translated | medium | `806e1240_FUN_806e1240.c` |
| `0x806E3630` | `fn_806e3630` | register the two `daLiftWaterWheel_c` states | translated | high | `806e3630_FUN_806e3630.c` |
| `0x806E37F0` | `fn_806e37f0` | destroy a registered water-wheel state | translated | high | `806e37f0_FUN_806e37f0.c` |
| `0x806E3970` | `fn_806e3970` | construct the `0x778`-byte water wheel | translated | medium | `806e3970_FUN_806e3970.c` |
| `0x80720DF0` | `fn_80720df0` | register the `daPeachCastleSequenceMgrObj_c` state | translated | high | `80720df0_FUN_80720df0.c` |
| `0x80720F00` | `fn_80720f00` | destroy a registered sequence-manager state | translated | high | `80720f00_FUN_80720f00.c` |
| `0x80721030` | `fn_80721030` | construct the sequence manager in place | translated | medium | `80721030_FUN_80721030.c` |
| `0x8072D970` | `fn_8072d970` | register the `daHouseLight_c` `StateID_Wait` state | translated | high | `8072d970_FUN_8072d970.c` |
| `0x8072DA80` | `fn_8072da80` | destroy a registered house-light state | translated | high | `8072da80_FUN_8072da80.c` |
| `0x8072DC00` | `fn_8072dc00` | construct the `0x958`-byte house light | translated | medium | `8072dc00_FUN_8072dc00.c` |
| `0x8073C970` | `fn_8073c970` | register the `daStand_c` state and four tuning words | translated | high | `8073c970_FUN_8073c970.c` |
| `0x8073CAC0` | `fn_8073cac0` | destroy a registered stand state | translated | high | `8073cac0_FUN_8073cac0.c` |
| `0x8073CC40` | `fn_8073cc40` | construct the `0x908`-byte stand | translated | medium | `8073cc40_FUN_8073cc40.c` |
| `0x80743CC0` | `fn_80743cc0` | register the `daTorchiLight_c` state | translated | high | `80743cc0_FUN_80743cc0.c` |
| `0x80743DD0` | `fn_80743dd0` | destroy a registered torch-light state | translated | high | `80743dd0_FUN_80743dd0.c` |
| `0x80743F50` | `fn_80743f50` | construct the `0x3F0`-byte torch light | translated | medium | `80743f50_FUN_80743f50.c` |
| `0x80755290` | `fn_80755290` | register the `daWireCircle_c` state | translated | high | `80755290_FUN_80755290.c` |
| `0x807553A0` | `fn_807553a0` | destroy a registered wire-circle state | translated | high | `807553a0_FUN_807553a0.c` |
| `0x80755520` | `fn_80755520` | construct the `0x4D8`-byte wire circle | translated | medium | `80755520_FUN_80755520.c` |
| `0x80787630` | `fn_80787630` | register the two `daWmYoganPillar_c` states and config | translated | high | `80787630_FUN_80787630.c` |
| `0x80787870` | `fn_80787870` | destroy a registered world-map pillar state | translated | high | `80787870_FUN_80787870.c` |
| `0x80787990` | `fn_80787990` | construct the world-map pillar in place | translated | medium | `80787990_FUN_80787990.c` |

**Totals:** 24 translated, 0 partial, 0 stub. 16 high confidence, 8 medium
(the eight constructors).

## Deferred candidates

These are the nearest untranslated neighbours; each was deliberately left for a
later pass.

| Address range | Why deferred |
|---|---|
| `0x806DBED0`, `0x806DC210` | `daLiftScalesStep_c` four-state table and its destructor. The matching constructor `0x806DB340` is a Ghidra paired-single reconstruction (`unaff_GQR0`, `ldexpf`); the whole class waits for a paired-single pass rather than translating a guessed body. |
| `0x806DC5B0` | `daLiftScalesStep_c` model loader (`test_lift` BRRES); belongs with the deferred constructor. |
| `0x806E2180`, `0x806E3090` | `daLiftWaterWheel_c` execute/update after construction; they carry float state math but no class attribution beyond the same cluster. |
| `0x806F98C0`, `0x806F9D50`, `0x806F9E20` | `daMiniGameGunBatteryMgrObj_c` table, constructor and model/init body; self-contained and safe to batch next. |
| `0x806EE170`, `0x806EE4B0`, `0x806EE630` | `daLineTrain_c` four-state table, destructor and constructor; constructor is in-place and complete. |
| `0x80664BA0`, `0x80666260`, `0x806671D0` | Koopa Jr/Rubble/Scaffold state tables; deferred only because their lifecycle triples also need their larger destructors and post-table init blocks. |
| `0x80651640` | still **rejected**: paired-single reconstruction. |

## Unknowns preserved

1. **Constructor class attribution.** None of the eight constructors contains a
   class string. Each is tied to its class only by the vtable it installs and
   by adjacency to the StateID table; all eight constructor rows are therefore
   `medium` confidence and the Rust carries `// inferred:` notes.
2. **Dropped `this` arguments.** `0x80066FC0`, `0x80094E80`, `0x800F5FD0` and
   `0x80064110` are base constructors called by address; Ghidra drops their
   `this` argument. The Rust passes the newly allocated object and documents
   the omission.
3. **`0x80721030` late init.** The body calls `0x80721790(actor)`; the callee
   is not translated and is kept by address with an `// unknown:` note.
4. **Sub-object initialisers.** `0x80169CD0`, `0x8016A740`, `0x80168D10`,
   `0x8016A5E0` and `0x802D7AA0` are main.dol routines whose roles are inferred
   from their call positions. They are emitted by address, never renamed.
5. **Array element constructors.** The array builder `0x802DC9A0` takes
   `(destination, constructor, destructor, element-size, count)`. Pairs
   `0x8016A740/0x8016A780`, `0x80169CD0/0x80169D20`,
   `0x80601E60/0x80601CD0`, `0x800DB130/0x806691D0`, `0x8007F7A0/0x8007F810`,
   and `0x806F0060/0x8064B270` are kept by address with `// unknown:` notes.
6. **Truncated state strings.** Ghidra truncates long class/state strings in
   its symbols (`StateI...`, `s_...`, `StateID_NoDi...`). The Rust names only
   the visible prefix and does not invent the missing characters.
7. **`0x80787630` configuration block.** The five word copies at
   `0x80818E44`, `0x80818E50..58` and `0x80832C7C` are explicit; their
   semantic fields are not named.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs src
```

`src/game/bases.rs` now contains 167 translated functions; this pass adds 24,
all with `# Original` blocks and the synthetic-base warning. `doc-lint` reports
`ok: true`, 0 violations over 68 files.
