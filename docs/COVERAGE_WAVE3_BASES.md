# Coverage - wave 3 second pass (`d_basesNP`)

## How to read this file

This page records the second translation pass over `d_basesNP`, the base
gameplay REL (`game::bases`). It answers:

1. **What was added?** - the table below lists the 25 functions appended to
   `src/game/bases.rs`, with their synthetic addresses, Rust names,
   plain-English roles, status, confidence, and Ghidra evidence.
2. **Why these functions?** - the *Selection* section explains how the
   deferred select-screen tails, the `0x806511D0..0x80651910` state/collision
   helpers, four class-state registration tables, and the registered actor
   destructor were chosen.
3. **What is still missing?** - *Remaining work* lists the tables and functions
   that remain deferred, and *Unknowns preserved* records every unresolved
   address or role instead of guessing it.

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

## Range inventory

| Metric | Wave 2 | Wave 3 | Cumulative |
|---|---:|---:|---:|
| Translated functions in `game/bases.rs` | 30 | 25 | 55 |
| Character-change select cluster | 19 | 8 | 27 |
| `0x8065` actor/state cluster | 7 | 13 | 20 |
| Class-state registration functions | 1 | 4 | 5 |
| State registrations covered by those functions | 4 | 7 | 11 |
| Selected bytes | 5,844 | 4,440 | 10,284 |
| Coverage of indexed bytes | 0.53 % | 0.40 % | 0.93 % |

The module still contains 5,615 function entries and 79 `StateID`-bearing
files. This pass intentionally takes a coherent slice, not the whole actor
inventory.

## Translated functions

All functions live in `src/game/bases.rs`. Addresses are synthetic
(`base 0x80600000`); evidence paths are relative to
`work/ghidra/rel-d_basesNP/export/`. The machine-readable ledger is
`ledger/agents/wave3-bases.csv` (`module = game::bases`).

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x80603110` | `fn_80603110` | re-apply the preview and dispatch by slot presence | translated | high | `80603110_FUN_80603110.c` |
| `0x806031D0` | `fn_806031d0` | re-apply the preview and wait for the owner's dirty byte | translated | high | `806031d0_FUN_806031d0.c` |
| `0x80603260` | `fn_80603260` | cycle the character preview or accept the slot | translated | medium | `80603260_FUN_80603260.c` |
| `0x806034D0` | `fn_806034d0` | play the "leave preview" sound and mark the screen dirty | translated | high | `806034d0_FUN_806034d0.c` |
| `0x80603570` | `fn_80603570` | apply the right-side preview after the direction animation | translated | high | `80603570_FUN_80603570.c` |
| `0x80603610` | `fn_80603610` | wait for layout/parent state, then dispatch | translated | high | `80603610_FUN_80603610.c` |
| `0x80603680` | `fn_80603680` | undo the side preview after the direction animation | translated | high | `80603680_FUN_80603680.c` |
| `0x80603700` | `fn_80603700` | commit the preview into the three four-entry arrays | translated | high | `80603700_FUN_80603700.c` |
| `0x806511D0` | `fn_806511d0` | destroy the registered `daBigShell_c::StateID_Wait` state | translated | high | `806511d0_FUN_806511d0.c` |
| `0x806512F0` | `fn_806512f0` | forward a state destructor to `0x802DCBFC` (hidden `r12`) | partial | medium | `806512f0_FUN_806512f0.c` |
| `0x80651320` | `fn_80651320` | second copy of the same forwarder | partial | medium | `80651320_FUN_80651320.c` |
| `0x80651350` | `fn_80651350` | release a state object without the name destructor | translated | medium | `80651350_FUN_80651350.c` |
| `0x80651390` | `fn_80651390` | allocate/construct the `0x3E0`-byte object (class unknown) | translated | high | `80651390_FUN_80651390.c` |
| `0x806513F0` | `fn_806513f0` | advance state when the scene clock reaches a threshold | translated | medium | `806513f0_FUN_806513f0.c` |
| `0x80651470` | `fn_80651470` | call interface virtual `+0x10`, return 1 | translated | high | `80651470_FUN_80651470.c` |
| `0x806514D0` | `fn_806514d0` | create an `0xAF` actor from a sprite record | translated | medium | `806514d0_FUN_806514d0.c` |
| `0x806515B0` | `fn_806515b0` | probe vertical extent with the rejected paired-single helper | translated | medium | `806515b0_FUN_806515b0.c` |
| `0x806517C0` | `fn_806517c0` | return whether any of four players is above the actor | translated | medium | `806517c0_FUN_806517c0.c` |
| `0x80651830` | `fn_80651830` | compute and store a scaled sprite height at `+0x3D4` | translated | medium | `80651830_FUN_80651830.c` |
| `0x80651910` | `fn_80651910` | dispatch state `0x808286A0` when a higher player exists | translated | medium | `80651910_FUN_80651910.c` |
| `0x80690F70` | `fn_80690f70` | register `daHandLight_c::StateID_Move` plus a tenth word | translated | high | `80690f70_FUN_80690f70.c` |
| `0x806B23F0` | `fn_806b23f0` | register `daKazanRock_c` `Fall` and `BigFall` | translated | high | `806b23f0_FUN_806b23f0.c` |
| `0x806E6810` | `fn_806e6810` | register `daLiftZenShiro_c` `Search`, `Move`, `Vanish` | translated | high | `806e6810_FUN_806e6810.c` |
| `0x8074EAF0` | `fn_8074eaf0` | register `daWakiWoodBox_c::StateID_Move` and four words | translated | high | `8074eaf0_FUN_8074eaf0.c` |
| `0x80650980` | `fn_80650980` | destroy/free a `daBgmInterlockingDummyBlockMgr_c` actor | translated | high | `80650980_FUN_80650980.c` |

**Totals:** 23 translated, 2 partial, 0 stub. 15 high confidence, 10 medium
(`0x80603260`, `0x806512F0`, `0x80651320`, `0x80651350`, `0x806513F0`,
`0x806514D0`, `0x806515B0`, `0x806517C0`, `0x80651830`, `0x80651910`). The
two partial rows are the forwarders whose hidden `r12` operand cannot be
represented by `platform::call3`.

## Selection

### Select-screen tails (8 functions)

The wave-2 page deferred `0x80602C20`, `0x80602EE0`, `0x80602FB0`,
`0x80602FF0`, `0x80603110`, and `0x806031D0`. The first four were already
translated in `game::player` by the time this pass started, so they are **not**
duplicated here. This pass takes the remaining tails `0x80603110` and
`0x806031D0`, plus `0x80603260` (preview cycling), `0x806034D0`/`0x80603570`/
`0x80603680` (sound and direction transitions), `0x80603610` (layout/parent
gate), and `0x80603700` (the actual commit). The cluster uses the same object
layout as wave 2: layout at `+0x90`, interface at `+0x228`, slot arrays at
`+0x280`/`+0x28C`/`+0x290`, selected slot at `+0x2D8`, preview at `+0x2E0`.

One Ghidra loss is explicitly repaired rather than guessed: `0x80603700` calls
`FUN_806027c0()` with no visible argument; the Rust passes `screen`.

The preview cycle at `0x80603260` is the only new branch that needs the
`0x80429F50` four-pointer array. Its entry `+4` is the per-player record; bit 3
of `record + 0x1C` advances the preview, bit 2 rewinds it, and the two paths
dispatch `0x80822CD8`. The duplicate-character path reuses wave-2
`fn_80602600` and the table at `0x807C66B0`.

`0x806034D0` and `0x80603570` are the "apply" side: sound `0x78`, direction
animation 3 for `+0x2A0 == 4` and 2 for `+0x2A0 == 5`, right-side preview
(`side = 1`), and the owner's `+0x264` preview copy. `0x80603680` is the
inverse transition (left-side preview, `side = 0`, clears `+0x296`), and
`0x80603610` only dispatches `0x80822C58` when the parent's `+0x26A` is clear
and the layout reports idle.

### State/collision helpers (12 functions)

The first helper, `0x806511D0`, is the only function in this group with direct
class evidence: `0x806510C0` registers it as the callback for
`daBigShell_c::StateID_Wait`. The rest form a tight block operating on an object
with `+0x394` interface vtable, `+0x3D0` state/flag word, `+0x3D4` float, and
`+0x3D8` pointer.

The constructor `0x80651390` allocates exactly `0x3E0` bytes and is **not** the
`daBigShell` actor: `fn_80650e60` (the `big_shell` model loader) writes through
`+0x4F4`, which cannot fit in a `0x3E0`-byte object. Its class identity is
therefore left unknown. It is grouped here because its field offsets match the
handlers exactly.

`0x806512F0` and `0x80651320` are byte-for-byte identical forwarders to the
unresolved indirect-jump stub `0x802DCBFC`; like `fn_80501920` in
`game::profile`, they carry a hidden `r12` this-adjust operand that
`platform::call3` cannot model, so they are marked `partial:` in the body.
`0x80651350` is the release-only variant: it skips the `0x8015F800` name
destructor and calls `0x802B9280` when the free flag is positive.

The behavior helpers are:

- `0x806513F0` - scene-clock threshold test; on success copies `actor[+4] & 0xF`
  to `+0x3D0` and dispatches `0x80828620`, otherwise calls the generic teardown
  `0x80162510`.
- `0x80651470` - interface virtual `+0x10`, constant return 1.
- `0x806514D0` - sprite/stage-table lookup and `0x80064610(0xAF, ...)` actor
  creation; returns the created object's first word.
- `0x806515B0` - calls the rejected paired-single routine `0x80651640` and the
  truncated `0x80651590`, then compares the object's `+0x3D4` float with the
  scene clock.
- `0x806517C0` - scans the four player entries through `0x8005F900` and reports
  whether one is above the actor on `+0xAC`.
- `0x80651830` - recomputes the `+0x3D4` float from the current sprite record
  using the classic `0x43300000` integer-to-double construction.
- `0x80651910` - dispatches `0x808286A0` when `0x806517C0` finds a higher
  player.

### Class-state registrations (4 functions, 7 states)

These are samples from the 89-file class/state-name family. Each uses the same
0x30-byte descriptor shape as wave-2 `fn_80609b70`: name object, vtable, nine
parameter words copied in the order `6,7,8,3,4,5,0,1,2`, then a `0x80600160`
push. The shared helper `register_state_desc` in `bases.rs` implements that
template once.

| Function | Class / states | Vtable | Callback | Node |
|---|---|---|---|---|
| `0x80690F70` | `daHandLight_c::StateID_Move` (+ tenth word) | `0x807EFC88` | `0x80691100` | `0x8082AF30` |
| `0x806B23F0` | `daKazanRock_c::StateID_Fall`, `StateID_BigFall` | `0x807F524C` | `0x806B25B0` | `0x8082C0B8`, `0x8082C0F8` |
| `0x806E6810` | `daLiftZenShiro_c::StateID_Search`, `Move`, `Vanish` | `0x807FD750` | `0x806E6A90` | `0x8082E240`, `0x8082E280`, `0x8082E2C0` |
| `0x8074EAF0` | `daWakiWoodBox_c::StateID_Move` | `0x8081207C` | `0x8074EC30` | `0x80831A48` |

`0x80690F70` is the only one with a tenth parameter word: `0x80691160` returns
the constant `0x80355D0C`, stored at descriptor `+0x30`. Its node is
`descriptor - 0x0C`, not the usual `descriptor - 0x10`. `0x8074EAF0` also copies
two REL constants into four BSS words after the registration.

### Registered stage-actor destructor (1 function)

`0x80650980` is the callback named by `0x80650870`
(`daBgmInterlockingDummyBlockMgr_c`). It runs `0x8015F800(actor, 0)` and, when
the free flag is positive, `0x802B9280(actor)`, returning the actor unchanged.
This is the same destructor shape as the state-object callbacks above, but it is
the actor destructor rather than a state descriptor destructor.

## Remaining work

### Deferred select-screen table

`0x806043C0` (`dCharacterChangeSelectBase_c`, 3,860 bytes) still contains 22
state registrations. The 22 descriptors are at `0x80822A58..0x80822F18`
(stride `0x40`), with parameter blocks at `0x807D1590..0x807D1CC0` (stride
`0x24`) and the shared vtable `0x807D1CF8`. It is mechanical, but it was left
for a dedicated pass so the 22 strings and the `0x80822F48` one-time flag can be
checked together.

### Deferred `0x8065` tables and helpers

- `0x80651990` and `0x80651A60` - the five-entry `0x80828430` table
  (stride `0x60`, 24 words per entry) and its shifting/spawn path. The table
  layout is visible but the entries' fields are not named.
- `0x80651590` - only the `+0x3D8 == 0` early return is decompiled; the rest is
  `halt_baddata()`.
- `0x80651640` - rejected paired-single reconstruction (`ldexpf`,
  `unaff_GQR0`, unreachable blocks). Do not translate it until a paired-single
  pass exists.
- `0x80650020`, `0x80650350`, `0x80650390`, `0x80650B00`, `0x80651F60` - small
  destructor/release helpers adjacent to the translated actor code. They were
  not needed by this slice.

### Remaining class/state-name tables

The larger deferred examples remain `0x806017E0` (`dCharacterChangeSelectArrow_c`),
`0x806083B0` (`dCollectionCoin_c`), `0x8060CBE0` (`dCourseClear_c`),
`0x8060F010` (`dCourseTimeUp_c`), and `0x80618D00` (`dFileSelect_c`, 44
`StateID` references). Smaller single-state tables include `0x806E0FB0`
(`daLiftSpinRotationSync_c`) and `0x80720DF0`
(`daPeachCastleSequenceMgrObj_c`).

## Unknowns preserved

1. **Class identity of the `0x3E0` object.** `0x80651390` installs vtable
   `0x807E5328` and inherits the `0x80066FC0` interface, but no state-name
   string or vtable evidence names the class. It is explicitly **not**
   `daBigShell_c`: the `big_shell` model loader reaches `+0x4F4`, beyond a
   `0x3E0`-byte allocation. The wave-2 page's earlier attribution is corrected
   here.
2. **Hidden `r12` in `0x802DCBFC`.** `0x806512F0` and `0x80651320` show only
   three visible arguments; the indirect stub also consumes a hidden
   this-adjust pointer. The Rust keeps the visible call and marks it `partial:`.
3. **Paired-single and bad-data callees.** `0x80651640` is the rejected
   paired-single function; `0x80651590` is truncated by `halt_baddata()`. Both
   are called by address in `0x806515B0` and never guessed.
4. **Lost arguments after save thunks.** `0x80603260` calls `0x80602460`
   without an argument, `0x80603700` calls `0x806027c0` without one, and
   `0x80651910` calls `0x806517C0` without one. Each Rust call passes the
   screen/actor pointer documented at the call site. The player-lane functions
   `0x80602C20`/`0x80602EE0` have the same limitation in `game::player`.
5. **`0x3E0` object teardown.** `0x806513F0` calls `0x80162510()` with no visible
   argument; the Rust passes the actor because the callee body operates on the
   object header. The exact owner of that teardown call remains inferred.
6. **Sprite/stage record roles.** `0x806514D0` and `0x80651830` use
   `descriptor[+4]` as a byte id and `descriptor[+8]` as a table index. The
   arithmetic is explicit, but the semantic names come from the stage tables in
   `src/game/stage.rs`, not from debug types.
7. **Overlapping `_DAT_` globals.** Ghidra prints `_DAT_807efc68` in
   `0x80690F70` because two symbols overlap. The Rust reads the 4-byte word at
   `0x807EFC68`, matching the store to descriptor `+0x14`.

## Verification

```powershell
cargo check
node tools/doc-lint.mjs
```

`doc-lint` reports `ok: true`, 0 violations. `cargo check` exits 0 for the whole
crate. `bases.rs` was also compiled in isolation against a copy of
`platform.rs` with `rustc --edition 2021 --crate-type lib`, which exits 0.
