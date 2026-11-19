# Coverage — wave 11 path/rail band (`0x8000C000..0x80014000`)

## How to read this file

This is the completion report for the wave-11 lane that owns the `main.dol`
band `0x8000C000..0x80014000` and writes into `src/game/path.rs`. It answers:

1. **What was added?** 25 new function bodies in `src/game/path.rs`. The 12
   path/rail bodies that were already in the file are not counted again.
2. **How sure is it?** `confidence` measures the evidence, not code quality.
   `high` means the Ghidra C and the raw instruction words agree on every
   load, store, branch and call operand. `medium` means the control flow is
   exact but an ABI detail or a record's meaning is inferred.
3. **Where is the evidence?** Every `# Original` block in the Rust names the
   Ghidra file under `work/ghidra/main-gb/export/`; call targets and constants
   were cross-checked in `work/dol/main.elf`.
4. **What is still missing?** The large state machines and the course-select
   guide table (`0x8000C5C0`, `0x8000CA50`, `0x8000E060`, `0x800129A0`) remain
   untranslated; they are listed under "Not selected" below.

Addresses are **virtual** `sys/main.dol` addresses (image base `0x80004000`),
not file offsets. The CSV uses the plain lowercase hex spelling
(`8000d7e0`) expected by `ledger/agents/*.csv`.

## Scope and counts

| Metric | Count |
|---|---:|
| New function bodies in `src/game/path.rs` | 25 |
| Bytes of Ghidra function size | 3,044 |
| `translated` | 25 |
| `partial` | 0 |
| `stub` | 0 |
| Confidence `high` | 23 |
| Confidence `medium` | 2 |
| New staging modules | 0 |
| Files written | `src/game/path.rs`, this document, `ledger/agents/wave11-path.csv` |

The selection rule was: every address must be (a) inside
`0x8000C000..0x80014000`, (b) absent from `ledger/functions.csv` and from
`src/game/path.rs`, and (c) backed by a complete Ghidra body plus a readable
instruction stream. The 25 addresses below satisfy all three. No ledger row
was created for an address without a body.

The band contains far more than 25 eligible functions. The pass deliberately
took the complete, small-to-medium bodies that share the path/rail object
layout or the course-select cursor family, leaving the multi-thousand-byte
state machines for a dedicated pass.

## Function table

`bytes` is the Ghidra function size. The evidence column is the file name
inside `work/ghidra/main-gb/export/`.

### A. Follower seed and actor-attach wrappers

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x8000D7E0` | [`fn_8000d7e0`] | clear two follower transform slots and start the rail follower at speed `8.0` | translated | medium | 60 | `8000d7e0_FUN_8000d7e0.c` |
| `0x8000F370` | [`fn_8000f370`] | prepare the model sub-object at `+8`, run the base initialiser and store `+0x24` | translated | high | 108 | `8000f370_FUN_8000f370.c` |
| `0x8000F800` | [`fn_8000f800`] | return the object from virtual slot `+0x14`, advanced by four bytes | translated | high | 48 | `8000f800_FUN_8000f800.c` |

### B. Cursor constructor and deleting-destructor family

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x8000FC30` | [`fn_8000fc30`] | build the cursor object, its shared base and eight embedded slots; publish the singleton | translated | high | 540 | `8000fc30_FUN_8000fc30.c` |
| `0x8000FE90` | [`fn_8000fe90`] | deleting destructor: free when `release > 0`, return the pointer | translated | high | 64 | `8000fe90_FUN_8000fe90.c` |
| `0x8000FED0` | [`fn_8000fed0`] | deleting destructor, second emitted copy | translated | high | 64 | `8000fed0_FUN_8000fed0.c` |
| `0x8000FF10` | [`fn_8000ff10`] | deleting destructor with a partial destructor call first | translated | high | 88 | `8000ff10_FUN_8000ff10.c` |
| `0x8000FFB0` | [`fn_8000ffb0`] | deleting destructor, third emitted copy | translated | high | 64 | `8000ffb0_FUN_8000ffb0.c` |
| `0x8000FFF0` | [`fn_8000fff0`] | deleting destructor, fourth emitted copy | translated | high | 64 | `8000fff0_FUN_8000fff0.c` |
| `0x80010030` | [`fn_80010030`] | deleting destructor, fifth emitted copy | translated | high | 64 | `80010030_FUN_80010030.c` |
| `0x80010070` | [`fn_80010070`] | deleting destructor that releases the `+0x18` sub-object first | translated | high | 96 | `80010070_FUN_80010070.c` |
| `0x800100D0` | [`fn_800100d0`] | same as `0x80010070` with a redundant second null test | translated | high | 100 | `800100d0_FUN_800100d0.c` |

### C. Cursor state writers

| address | Rust function | role | status | conf. | bytes | evidence |
|---|---|---|---|---:|---:|---|
| `0x800105C0` | [`fn_800105c0`] | write the four cursor transform floats and the ready byte | translated | medium | 196 | `800105c0_FUN_800105c0.c` |
| `0x800108C0` | [`fn_800108c0`] | step the cursor position by 20, clamp, and mirror the byte into eight elements | translated | high | 144 | `800108c0_FUN_800108c0.c` |
| `0x80010F40` | [`fn_80010f40`] | test one player bit in the `{1,2,4,8}` mask at `+0x434` | translated | high | 32 | `80010f40_FUN_80010f40.c` |
| `0x80011020` | [`fn_80011020`] | show/hide cursor slot 1 from the `+0x437` flag | translated | high | 144 | `80011020_FUN_80011020.c` |
| `0x80011120` | [`fn_80011120`] | mark cursor slot 1 ready and reset element 8 | translated | high | 84 | `80011120_FUN_80011120.c` |
| `0x80011180` | [`fn_80011180`] | show/hide cursor slot 1 from the `+0x436` flag | translated | high | 152 | `80011180_FUN_80011180.c` |
| `0x800112C0` | [`fn_800112c0`] | clear `+0x43A` and apply the `+0x43D` state to elements 0 and 1 | translated | high | 80 | `800112c0_FUN_800112c0.c` |
| `0x80011310` | [`fn_80011310`] | show/hide cursor slot 2 from the `+0x43B` flag | translated | high | 180 | `80011310_FUN_80011310.c` |
| `0x800113E0` | [`fn_800113e0`] | show cursor elements 3 and 4 | translated | high | 64 | `800113e0_FUN_800113e0.c` |
| `0x80011470` | [`fn_80011470`] | clear `+0x43B` and reset elements 3, 4, 6 and 7 | translated | high | 104 | `80011470_FUN_80011470.c` |
| `0x800114E0` | [`fn_800114e0`] | show/hide cursor slot 2 from the `+0x43A` flag | translated | high | 188 | `800114e0_FUN_800114e0.c` |
| `0x800115C0` | [`fn_800115c0`] | show slot 3 when the group is ready and player 0 is absent or another member is present | translated | high | 108 | `800115c0_FUN_800115c0.c` |
| `0x800126A0` | [`fn_800126a0`] | mirror the selected course id and restart two cursor animations on change | translated | high | 208 | `800126a0_FUN_800126a0.c` |

Every row above is also in `ledger/agents/wave11-path.csv` with module
`game::path` and `rust_file = src/game/path.rs`.

## The two medium-confidence bodies

### `0x8000D7E0` — hidden register arguments

Ghidra declares `FUN_8000D7E0(this, point)`, but the call site at
`0x8000D810` only sets `r3` and `r4`; `r5`, `r6` and `r7` still hold whatever
the caller left there. The callee `0x8000DE70` consumes those three registers
as `pose`, `arg_a` and `arg_b`:

```
0x8000DE84  or   r31,r6,r6     ; arg_a
0x8000DE8C  or   r30,r5,r5     ; pose
0x8000DE9C  or   r4,r7,r7      ; arg_b -> r4 for 0x80164FD0
```

`fn_8000d7e0` therefore exposes them as explicit parameters and forwards all
three. This preserves the register state that the original caller supplied;
the control flow itself is a straight line.

### `0x800105C0` — player-record meaning

The arithmetic is exact: the `u16` at `[[0x8042A480]]+6` is converted to
`f32`, `330.0` is subtracted, the result is multiplied by `0.5`, and `640.0`
and `330.0` are stored as the other two extents. What the `u16` counts (players
or controllers) is inferred from the `{1,2,4,8}` player mask used by
`0x80010F40`, not from a string or a type descriptor.

## Verified facts used by the translations

### Small-data bases and globals

| expression | absolute | meaning |
|---|---|---|
| `r13-0x5200` | `0x8042A480` | pointer to the course-select player record |
| `r13-0x5A84` | `0x80429BFC` | cursor singleton written by `0x8000FC30` |
| `r13-0x5730` | `0x80429F50` | course manager; `[+4][+8]` is the selected id |
| `r13-0x5704` | `0x80429F7C` | course state object; `+0x3D9` mirrors `id != 0` |
| `r2-0x7F9C` | `0x8042B0E4` | zero word |
| `r2-0x7F98` | `0x8042B0E8` | `8.0f` default follower speed |
| `r2-0x7F80` | `0x8042B100` | `0.0f` (existing rail sum) |
| `r2-0x7F2C` | `0x8042B154` | `0.0f` |
| `r2-0x7F28` | `0x8042B158` | `0.5f` |
| `r2-0x7F24` | `0x8042B15C` | `330.0f` |
| `r2-0x7F20` | `0x8042B160` | `640.0f` |
| `r2-0x7F10` | `0x8042B170` | byte table `{1, 2, 4, 8}` |

`r13 = 0x8042F680` and `r2 = 0x80433080` are the values installed by the DOL
startup sequence (see `docs/COVERAGE_r80005_80014.md`).

### Cursor object layout

| offset | contents |
|---|---|
| `+0x000` | vtable `0x802FEA28` (never replaced) |
| `+0x008` | shared course-select base |
| `+0x098..+0x0A4` | four transform floats written by `0x800105C0` |
| `+0x0A8` | ready byte |
| `+0x1A0`, `+0x1DC`, `+0x218`, `+0x254`, `+0x290`, `+0x2CC`, `+0x308`, `+0x344` | eight 0x24-byte slot objects |
| `+0x384`, `+0x3F0`, `+0x388`, `+0x3F4`, `+0x38C`, `+0x3F8`, `+0x390`, `+0x3FC` | eight element pointers whose byte `+0xB8` mirrors the cursor position |
| `+0x3CC`, `+0x3D0` | two animation panes used by `0x800126A0` |
| `+0x404` | mirrored course id |
| `+0x42C` / `+0x430` | current position / target position |
| `+0x434` | player bit mask |
| `+0x435..+0x44E` | per-slot state bytes |

Each slot is built with four vtable words `0x802FEA68`, `0x802FEAF8`,
`0x802FEA98`, `0x802FEAB0`, a back-pointer to the owner at `+0x10`, zero at
`+0x14`, a call to `0x8015FC90(slot+0x18, slot+4, slot+8, descriptor)`, and a
final replacement of its first word by `0x802FEA38`.

### Call targets

| callee | observed interface |
|---|---|
| `0x8016A690` | `(obj+8, source, 0x20)` |
| `0x800C8950` | `(cursor+8)` |
| `0x8015FC90` | `(slot+0x18, slot+4, slot+8, descriptor)` |
| `0x802B9280` | `(object)`; zeroes `r4` itself and tail-branches to `0x802B8F70` |
| `0x8015F9F0` | `(object, 0)` partial destructor |
| `0x801DD1D0` | bool query, no arguments; `0x800B54B0` is a tail thunk |
| `0x800C9490` | `(base+8, index)` hide |
| `0x800C9610` | `(base+8, index)` present test |
| `0x800C92F0` | `(base+8, index, value)` state store |
| `0x800C9380` | `(base+8, index)` show |
| `0x800C9A60` | `(pane, stamp, 4, action, 0)` animation start |
| `0x800CDC60` | frame stamp from `[0x80369CB4]`; ignores `r3` |
| `0x80164FD0` | `(this, this+8, 0)` actor base (via the `0x80007510` thunk) |

## Not selected

These entries in the same band are still untranslated and were **not** given
ledger rows:

| address | size | why not selected |
|---|---:|---|
| `0x8000C5C0` | 1168 | GX/paired-single setup, not path/rail |
| `0x8000CA50` | 1444 | GX/paired-single setup, not path/rail |
| `0x8000D740` | 160 | paired-single/GQR transform; Ghidra output is low fidelity |
| `0x8000D820` | 72 | wrapper around `0x8000D740`; inherits its GQR uncertainty |
| `0x8000E060` | 2016 | large paired-single/GX state machine |
| `0x8000E910` | 732 | quad fill; GQR-heavy, belongs to a render pass |
| `0x8000EDD0` | 20 | indirect vtable jump Ghidra could not recover |
| `0x8000EDF0` | 192 | depends on `0x8000E910` and the unresolved `0x8000EDD0` |
| `0x8000F950` | 580 | rail polyline builder with GQR/prologue damage; needs a dedicated pass |
| `0x8000FBA0` | 72 | wrapper whose `r7` input is only meaningful with `0x8000F950` |
| `0x800105B0` | 8 | below the 16-byte selection rule |
| `0x80010690`, `0x80010950`, `0x80010B50`, `0x80010DD0` | 360/424/640/356 | larger cursor orchestration with several unresolved callees |
| `0x80012940`, `0x80012970` | 48/48 | tail calls through the unresolved `0x802DCBFC` jump stub |
| `0x800129A0` | 6204 | course-select guide state table |

## Method and verification

1. Filtered `work/ghidra/main-gb/export/functions.tsv` on
   `0x8000C000 <= entry < 0x80014000`, then crossed the result with
   `ledger/functions.csv` and the existing `src/game/path.rs`.
2. Read the Ghidra C for every candidate and rejected any body whose control
   flow was truncated or whose only external effect was an unresolved
   indirect jump.
3. For every selected body, decoded the raw instruction words from
   `work/dol/main.elf` and checked: call targets, argument registers, stack
   arrangement, signed comparisons, and every load/store displacement. This
   caught the `0x8000FC30` slot-installer argument order (`slot+4` before
   `slot+8`) and the byte re-load in `0x800112C0`.
4. Resolved every `r2`/`r13`-relative constant by computing
   `r2 = 0x80433080`, `r13 = 0x8042F680`, then reading the word/float from the
   image.
5. Wrote the bodies in `src/game/path.rs` only. No `lib.rs`, `mod.rs`,
   `Cargo.toml`, `platform.rs`, generated document or other lane's file was
   changed.
6. Ran `cargo check` and `node tools/doc-lint.mjs src` after the edit.
   `cargo check` finishes with only the pre-existing warnings in other modules;
   `src/game/path.rs` adds no warning. The doc linter reports 0 violations.

## Uncertainties

1. **Hidden register arguments** in `0x8000D7E0` (documented above). The Rust
   signature is wider than Ghidra's because the original forwards `r5..r7`.
2. **Slot and descriptor types.** The eight `0x803516B8..0x80351DB8` words and
   the nine `0x803516B8..0x803518F8` accessor words are all zero in the shipped
   image. Only their use as opaque virtual-method arguments is proven.
3. **`0x802B9280` identity.** The shape is CodeWarrior `operator delete`
   (`r4` forced to zero, tail branch into the allocator), but the allocator's
   owner is not reconstructed here.
4. **Player presence semantics.** `0x80010F40` proves the bit test, and
   `0x800105C0` proves the `u16`-to-float arithmetic; whether the `u16` is a
   player count or a controller count is inferred.
5. **No runtime validation.** `cargo check` proves type correctness, not
   PowerPC execution. All claims above are static evidence from the shipped
   image.
