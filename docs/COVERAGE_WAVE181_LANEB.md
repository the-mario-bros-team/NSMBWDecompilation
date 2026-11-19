# Coverage - Wave 181 lane B (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane B block) | `game::bases` | 8 | 772 | 8 | 0 |

Eight bodies from the sub-300-byte sweep, all inside the `0x8060` block.
Source of the bodies: `work/wave181/laneB_block.rs`. Six of the eight belong to
`dCollectionCoinDate_c` (`0x80609370..0x806099A0`, plus the shared state
destructor `0x80609E70`); the remaining two are `dCourseClear_c` state
transitions (`0x8060C840`, `0x8060C960`) next to the already translated
`fn_8060c910`.

## What this pass covers

- **`dCollectionCoinDate_c` frame/state trio.** `0x80609370` runs the active
  state object at `+0x208` (slot `+0x10`) and the layout update at `+0x70`
  (vtable word `+0x78`, slot `+0x10`) when both latches `+0x288`/`+0x289` are
  set, and returns `1`. `0x80609860` refreshes the panes (`fn_80609440`, already
  translated) and then switches the state machine to the `NowDisp` descriptor
  `0x80823240`. `0x806098E0` refreshes and clears `+0x28B`, and switches to the
  `Exit` descriptor `0x80823280` when `+0x28A` is set.
- **Descriptor identity.** `0x80823200`/`0x80823240`/`0x80823280` are registered
  by the already translated `fn_80609b70`; the name strings behind them
  (`0x807D2C5C`, `0x807D2C88`, `0x807D2CB0`) read
  `dCollectionCoinDate_c::StateID_OnStageSetup`, `..._NowDisp`, `..._Exit`.
  That is what fixes the class and the state names in all three bodies.
- **Link helper.** `0x806099A0` calls the candidate's vtable slot `+0xC` with
  the candidate as receiver; on a zero result it stores the pointer at
  `owner + 0xC` and returns `owner + 4`, otherwise `0`.
- **State descriptor destructor.** `0x80609E70` is passed to `fn_80600160` as
  the delete callback of all three coin-date states. It is the
  [`fn_8060a040`]/*delete-with-count* shape plus a leading
  `0x8015F800(this, 0)` destructor call; `amount > 0` (signed) gates the
  `0x802B9280` heap release; it returns `this`.
- **`dCourseClear_c` transitions.** `0x8060C5F0` gates on `+0x374`/`+0x375`,
  executes the state at `+0x330`, resets the layout (`0x800C9560`), updates it,
  and re-uploads the two-line text through the still-untranslated `0x8060C6D0`.
  `0x8060C840` and `0x8060C960` share the `0x800C9610(layout, -1) == 0`
  finished-animation guard and switch the `+0x330` machine to the `Disp`
  (`0x80823600`) and `GoalDeMoEndWait` (`0x80823680`) descriptors registered by
  `fn_8060cbe0`; the second also clears `+0x376`/`+0x375`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80609370` | `fn_80609370` | 104 | high |
| `0x80609860` | `fn_80609860` | 72 | high |
| `0x806098E0` | `fn_806098e0` | 104 | high |
| `0x806099A0` | `fn_806099a0` | 96 | high |
| `0x80609E70` | `fn_80609e70` | 88 | high |
| `0x8060C5F0` | `fn_8060c5f0` | 120 | high |
| `0x8060C840` | `fn_8060c840` | 88 | high |
| `0x8060C960` | `fn_8060c960` | 100 | high |

## Deferred

None.

## Corrections and notes

- **`0x80823240`/`0x80823280`/`0x80823600`/`0x80823680` are BSS, not
  strings.** The ELF's last `PT_LOAD` is `p_vaddr = 0x80822900`,
  `p_filesz = 0`, `p_memsz = 0x12484`, so `(address - 0x80600000) + 0x1000`
  runs past the file end for these addresses. Ghidra's `&DAT_80823240` is the
  address of a zero-initialised runtime descriptor; the bodies pass the address
  itself, so no data read is modelled and no "string" is claimed at the call
  site. The names come from the descriptor's *name* words
  (`0x807D2C88` etc.), which are in `.rodata` and were read out of the file.
- **`+0x78` layout vtable.** In `0x80609370` and `0x8060C5F0` the vtable word is
  at `this + 0x78` while the receiver is `this + 0x70` (`lwz r12,0x78(r31)` /
  `addi r3,r31,112`). Both follow the established
  `read_u32(this + 0x78)` -> `abi_call1!` form used by `fn_80621dc0`; they are
  not `vmethod1(this + 0x70, ...)`.
- **`+0x208`/`+0x330` receivers are at offset 0.** `lwzu r12,0x208(r3)` and
  `lwzu r12,0x330(r3)` load the vtable and leave the receiver at
  `this + 0x208` / `this + 0x330`, so `vmethod0`/`vmethod1` are exact there.
- **`0x80609E70` has two calls, `fn_8060a040` only one.** The first call is
  `abi_call2!(0x8015_F800, this, 0)`; the count test is `cmpi cr7` + `bgt`,
  i.e. signed `> 0`, matching [`fn_8060a040`]. The `0x802B9280` release stays an
  address because its Rust body is unclaimed.
- **`0x800C9560` in `0x8060C5F0`.** Used with `this + 0x70` in the same position
  as the pre-existing calls to it (e.g. `fn_80621dc0` neighbourhood); it is
  described by its argument only.

## Verification

- All eight bodies were type-checked standalone against the real helper shapes
  (`platform::read_u8/u32/write_u8/write_u32`, `abi_call1!`/`abi_call2!`,
  `vmethod0`/`vmethod1`, `fn_80609440`) with `rustc --edition 2021
  --crate-type lib`: no errors. `src/game/bases.rs` was not modified; the
  coordinator's `merge.cjs` appends the block.
- The evidence exports exist for all eight addresses. Every branch, byte offset,
  vtable slot and descriptor address was checked against the raw words
  (`bc 12,2` guards, `lwzu`/`lwz r12` slots `+0x10`/`+0x18`, trailing
  `li r3,1` in `0x80609370`/`0x8060C5F0`).
