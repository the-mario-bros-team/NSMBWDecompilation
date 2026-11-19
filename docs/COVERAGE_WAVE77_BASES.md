# Coverage - Wave 77 (`game::bases`, `0x8062` bucket)

## How to read this file

This pass continues `src/game/bases.rs`, the semantic slice of the
`d_basesNP` REL. Every address is a **static analysis address** under the
canonical synthetic base `0x80600000` (produced by `tools/rel2elf.mjs`), never a
runtime address: the Wii REL loader chooses the real load address, so the
numbers only identify positions inside the REL image. Cross-module pointers
(`0x8037xxxx`, `0x8042xxxx`) are `main.dol` addresses imported by the REL and
keep their absolute values.

Byte counts come from `work/ghidra/rel-d_basesNP/export/functions.tsv`; every
cited evidence file is the Ghidra C export at
`work/ghidra/rel-d_basesNP/export/<addr>_FUN_<addr>.c`. In the table below,
"role" is the plain-English behaviour, and confidence is `high` when guards,
stores, constants, strings and call arguments are literal, `medium` when the
role of a field or callee had to be inferred (those bodies carry `// observed:`,
`// inferred:` or `// unknown:` notes), and `low` when only the instruction
sequence is certain.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 30 | 3372 | 15 | 15 |

No body in this pass is `low` confidence.

## What this pass covers

The `0x8062` bucket held 204 functions at lane start; 54 were already in
`ledger/functions.csv` (the `dMiniGameCannon_c`, `dMiniGameWire_c` and
`dModeSelect_c` tables, their destructors and in-place constructors, the
`dModelPlayArrow_c` / `dModelPlayBase_c` / `dModelPlayDate_c` /
`dModelPlayManager_c` / `dMultiCourseSelect_c` state tables and constructors,
and the paired register-thunk rejections listed by earlier waves), leaving 150.
This pass fills two screen member sets:

- **Mode-select screen members, `0x80622EB0..0x80623860`** - fourteen bodies of
  the screen whose class vtable is `0x807DA178` and whose six state descriptors
  `0x80825580..0x808256C0` are registered by [`fn_80623ab0`] (Ghidra names them
  `dModeSelect_c::StateID_*`):
  - the two destructor pairs [`fn_80622eb0`] / [`fn_80622ef0`] (no embedded
    sub-object) and [`fn_80622f30`] / [`fn_80622f90`] (tear down `+0x18` first);
  - the `modeSelect/modeSelect.arc` + `modeSelect_09.brlyt` loader
    [`fn_80623170`] with the `A00_Window`, `T_story_00`, `P_story_00` and
    `T_guideS_00` bindings;
  - the pad/scene gate [`fn_80623260`], the `T_guideS_00` scroll clamp
    [`fn_806233a0`], the layout/animation reset [`fn_806234d0`] and the row
    animation swap [`fn_806235b0`];
  - the controller-driven row cursor [`fn_806236b0`], which reads the player
    flags word at `+0x1C` of `0x80429F50[0x8042A464] + 4`;
  - the four `StateID_*` transitions [`fn_80623530`], [`fn_80623620`],
    [`fn_806237e0`] and [`fn_80623860`].

- **Model-play screen members, `0x80625560..0x80625FF0`** - sixteen bodies of
  the screen whose layout block is at `+0xAC` and whose state descriptors
  `0x80825888` (`StartWait`) / `0x80825908` (`Action`) are the
  `dModelPlayBase_c::StateID_*` entries registered by [`fn_80626240`]:
  - the two destructor pairs [`fn_80625560`] / [`fn_806255a0`] and
    [`fn_806255e0`] / [`fn_80625640`];
  - the `modelPlayBase/modelPlayBase.arc` + `modelPlayBase_07.brlyt` loader
    [`fn_806257e0`] with the `A00_window`, `T_subjectS_00`, `T_totalCoin_00`
    and `N_model1_Pos_00` bindings;
  - the coin recount [`fn_806258d0`] (a `10 * 0x2A * 3` scan that stores its
    total at `+0x288`) and its two consumers, the effect spawn [`fn_80625ac0`]
    (`Wm_2d_moviecoinvanish`) and the coin spend [`fn_80625ed0`];
  - the seven animation/state gates [`fn_80625bf0`], [`fn_80625c70`],
    [`fn_80625d90`], [`fn_80625e10`], [`fn_80625e70`], [`fn_80625f40`],
    [`fn_80625fb0`] and [`fn_80625ff0`], three of which are byte-identical
    `Action`-transition copies for their own state slots.

Recurring layout facts used by this pass: the mode-select screen keeps its NW4R
layout at `+0x74` (vtable word at `+0x7C`) and its state object at `+0x20C`;
the model-play screen keeps its layout at `+0xAC` (vtable word at `+0xB4`) and
its state object at `+0x70`; both dispatch a state descriptor through virtual
slot `+0x18` of the state object; layout animations are started with
`0x800C92F0`, bound with `0x800C93D0` and queried with `0x800C9610` /
`0x800C9640`; sound cues go through `0x80195380(0x8042A488, id, 1)`.

The `0x80625970` bad-instruction export is called by [`fn_80625ed0`] and is
recorded there as an address-only call with a `// partial:` note.

## Functions

| address | function | bytes | confidence | role |
|---|---|---:|---|---|
| `0x80622EB0` | `fn_80622eb0` | 64 | medium | release without sub-object |
| `0x80622EF0` | `fn_80622ef0` | 64 | medium | release without sub-object (second copy) |
| `0x80622F30` | `fn_80622f30` | 96 | medium | release `+0x18` sub-object then object |
| `0x80622F90` | `fn_80622f90` | 100 | medium | release `+0x18` sub-object then object |
| `0x80623170` | `fn_80623170` | 240 | high | `modeSelect` archive, layout and pane bindings |
| `0x80623260` | `fn_80623260` | 84 | medium | scene/pad/`0x80600510` gate folded to a boolean |
| `0x806233A0` | `fn_806233a0` | 116 | medium | `T_guideS_00` scroll step and wrap |
| `0x806234D0` | `fn_806234d0` | 96 | high | layout reset and animations `0`,`1`,`2` |
| `0x80623530` | `fn_80623530` | 84 | high | animation-end to `StateID_Action` |
| `0x806235B0` | `fn_806235b0` | 100 | high | row indicator swap (`previous + 5`, `chosen + 1`) |
| `0x80623620` | `fn_80623620` | 88 | high | animation-end to `StateID_OnStage` |
| `0x806236B0` | `fn_806236b0` | 252 | medium | controller-driven row cursor |
| `0x806237E0` | `fn_806237e0` | 88 | high | animation-end to `StateID_DateSelect` |
| `0x80623860` | `fn_80623860` | 104 | medium | clear flags and return to `StateID_Start` |
| `0x80625560` | `fn_80625560` | 64 | medium | release without sub-object |
| `0x806255A0` | `fn_806255a0` | 64 | medium | release without sub-object (second copy) |
| `0x806255E0` | `fn_806255e0` | 96 | medium | release `+0x18` sub-object then object |
| `0x80625640` | `fn_80625640` | 100 | medium | release `+0x18` sub-object then object |
| `0x806257E0` | `fn_806257e0` | 240 | high | `modelPlayBase` archive, layout and pane bindings |
| `0x806258D0` | `fn_806258d0` | 156 | medium | coin recount stored at `+0x288` |
| `0x80625AC0` | `fn_80625ac0` | 212 | medium | `Wm_2d_moviecoinvanish` spawn record |
| `0x80625BF0` | `fn_80625bf0` | 128 | high | show coin widget, animations and sound `0xA7` |
| `0x80625C70` | `fn_80625c70` | 88 | high | animation-end to `Action` |
| `0x80625D90` | `fn_80625d90` | 116 | high | mode-selected enter animation and sound `0x7D` |
| `0x80625E10` | `fn_80625e10` | 88 | high | animation-end to `Action` (copy) |
| `0x80625E70` | `fn_80625e70` | 96 | high | mode-selected leave animation, clears busy byte |
| `0x80625ED0` | `fn_80625ed0` | 108 | medium | spend one coin, spawn vanish effect, sound `0xA8` |
| `0x80625F40` | `fn_80625f40` | 88 | high | animation-end to `Action` (copy) |
| `0x80625FB0` | `fn_80625fb0` | 64 | high | start animation `4`, set busy byte |
| `0x80625FF0` | `fn_80625ff0` | 88 | high | animation-end to `StartWait` |

## Address range

`0x80622EB0..0x80625FF0`, in two runs:

- `0x80622EB0..0x80623860` - fourteen bodies. The `dModeSelect_c` constructor
  `0x80622E00` sits immediately above the run, and the already-translated
  `dModeSelect_c` table `0x80623AB0` / destructor `0x80623F80` /
  constructor `0x806240A0` follow it.
- `0x80625560..0x80625FF0` - sixteen bodies. The `dModelPlayBase_c` table
  `0x80626240` (translated in wave 15) sits immediately above the run, and the
  `dModelPlayArrow_c` loader `0x80624400` / constructor `0x806254B0` above it.

## Deliberate skips

- `0x80621FB0`, `0x80621FE0`, `0x80623A50`, `0x80623A80` (48/48/48/48) - all
  four exports are the same `func_0x802dcbfc(param_2, param_2, param_1)` thunk
  with a duplicated argument, the `0x802DCBFC` stub family that waves 73 and 76
  already rejected.
- `0x80625970` (24) - its export is `halt_baddata()` after a "Control flow
  encountered bad instruction data" warning. Only the call site survives, and it
  is recorded inside [`fn_80625ed0`].
- `0x80625D90`'s sibling `0x80624150..0x80624C80` (the `dModelPlayArrow_c`
  member set) and `0x806268C0..0x80627E10` (the `dModelPlayDate_c` member set)
  are untouched, **not** rejected: they were left for a later pass to keep this
  lane at 30 bodies.
- The rest of the bucket, `0x80628450..0x8062FFF0`, is untouched; the
  paired-single and indirect-jump stub failures noted by earlier waves are not
  re-litigated here.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (30 rows, 3,372 bytes).
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: the 54 pre-existing rows of
  this bucket were listed before writing and none is in this pass.
- `node tools/check-lane-csv.mjs wave77` reports
  `wave77-bases.csv: 30 rows` and `OK`.
- `cargo check --message-format=short` produces no diagnostic for
  `src/game/bases.rs`.
