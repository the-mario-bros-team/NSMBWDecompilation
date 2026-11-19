# Coverage - Wave 211 lane A (`game::player`, dokan model placement and anchor run)

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 10 | 1784 | 9 | 1 | 0 |

Status split: 10 `translated`, 0 `partial`, 0 deferred. Bytes are the
`functions.tsv` sizes: 112 + 932 + 80 + 88 + 12 + 108 + 84 + 124 + 156 + 88.

## How to read this file

* **Address** - virtual entry point in the DOL image, not a file offset.
* **Name** - the Ghidra name (`FUN_<address>`). No symbol table survives for
  these functions, so the Rust body keeps the `fn_<address>` name.
* **Role** - one plain sentence per function, in "What this pass covers". A
  role is marked "inferred" when it comes from shared field offsets and call
  shapes rather than from a recovered symbol.
* **Confidence** - strength of the evidence, not code quality: `high` = every
  address, offset and branch is literal; `medium` = a field role or a callee's
  meaning had to be inferred.

## Size verification

Every `functions.tsv` size was re-checked against the prologue and the closing
`blr` in `work/dol/main.elf`, and every row is exact (no prologue or alignment
artefact in this lane):

| address | tsv size | terminal `blr` | real extent |
|---|---:|---|---:|
| `0x80063200` | 112 | `0x8006326C` | 112 |
| `0x80063270` | 932 | `0x80063610` | 932 |
| `0x800636E0` | 80 | `0x80063730` | 80 |
| `0x80063820` | 88 | `0x80063874` | 88 |
| `0x80063880` | 12 | `0x80063888` | 12 |
| `0x80063890` | 108 | `0x800638F8` | 108 |
| `0x80063900` | 84 | `0x80063950` | 84 |
| `0x80063960` | 124 | `0x800639D8` | 124 |
| `0x80063AC0` | 156 | `0x80063B58` | 156 |
| `0x80063E00` | 88 | `0x80063E54` | 88 |

**One brief note.** The lane target list quoted **20 bytes** for `0x80063270`.
That is a transcription slip, not a real extent: the function opens with
`stwu r1,-0xC0` plus fourteen `stfd`/`psq_st` saves and closes with
`addi r1,r1,0xC0; blr` at `0x80063610`, so the body is
`0x80063270..0x80063614` = 932 bytes, matching `functions.tsv`. The following
addresses are separate stubs (`0x80063620` is a bare `blr`; `0x80063630` is a
different 164-byte function) and are not part of this lane.

## What this pass covers

The ten bodies are one closed run inside the `obj_dokan` actor that wave 210
lane F built at `0x80062600..0x800630B0`. The lane is the placement and
anchor-follower half of that class:

| address | function | role | confidence |
|---|---|---|---|
| `0x80063200` | [`fn_80063200`] | per-frame gate: despawn path (mode dispatch + `0x80162510`) when `+0x648 == 0.0f` or class slot `+0xD4` is non-zero, otherwise step the `+0x558` track through `0x8007FDA0` | high |
| `0x80063270` | [`fn_80063270`] | rebuild every model transform: main model, `count-2` parts and the cap, placed along the heading with `cos`/`sin` from `0x80237C40`/`0x80237BD0` | medium |
| `0x800636E0` | [`fn_800636e0`] | dispatch the `+0x640` mode to class slot `+0xEC` / `+0xF0` / `+0xF4` | high |
| `0x80063820` | [`fn_80063820`] | linear class-`0x23C` scan for the first object whose byte `+0x438` equals the tag | high |
| `0x80063880` | [`fn_80063880`] | store the anchor object's first word at `+0x3D0` | high |
| `0x80063890` | [`fn_80063890`] | drive `+0xAC` or `+0xB0` from `anchor+0x434 * this+0x3D4` and the axis byte `+0x3DD` | high |
| `0x80063900` | [`fn_80063900`] | resolve `+0x3D0` through `0x80162D00` and re-apply the axis drive | high |
| `0x80063960` | [`fn_80063960`] | attach a found anchor, re-apply the drive and hand `0x80355E10` to model-context slot `+0x18` | high |
| `0x80063AC0` | [`fn_80063ac0`] | per-frame update: `0x80064F50(this, 0)` gate, then model-context `+0x10` + class `+0xE8`, or `0x80064D10(this, 1)` | high |
| `0x80063E00` | [`fn_80063e00`] | release through `0x8015F800(this, 0)` and free through `0x802B9280` when `destroy > 0` | high |

## Object map used by the lane

Same layout wave 210 lane F documents for `fn_80062f00` / `fn_800630b0`:

| offset | type | role |
|---|---:|---|
| `+0x04` | `u32` | spawn parameter word; bits `16..24` are the body segment count |
| `+0x60` | `u32` | class vtable (a class slot is `(*(this+0x60))[slot](this)`) |
| `+0x7C` | 12 `f32` | pose matrix written by `0x801C0BD0` |
| `+0xAC..+0xB4` | 3 `f32` | scene position |
| `+0xDC..+0xE4` | 3 `f32` | scale triple handed to `0x8016A150` |
| `+0x100..+0x104` | 3 `u16` | rotation triple (Z, Y, X) |
| `+0x394` | object | model context (vtable at its first word) |
| `+0x3B4` | object | main model adapter |
| `+0x3C0 + i*0xC` | 32 objects | part adapters, `obj_dokan_B` |
| `+0x540` | object | cap model adapter |
| `+0x558` | record | track driven by `0x8007FDA0` |
| `+0x640` | `u32` | model mode; indexes `0x80309C54`, `0x80309C6C` and `0x802EF318` |
| `+0x644` | `f32` | body length `16.0f * count` (recomputed from `+0x04` by [`fn_80063270`]) |
| `+0x648` | `f32` | per-frame timer tested by [`fn_80063200`] |
| `+0x650` / `+0x652` | `u16` | heading copies written by `fn_80062f00` |
| `+0x3D0` | `u32` | current anchor handle |
| `+0x3D4` | `f32` | anchor-driven rate |
| `+0x3D8` | `f32` | anchor-driven base coordinate (shared by both axes) |
| `+0x3DD` | `u8` | anchor-driven axis selector (`0`/`1` = X, `2`/other = Y) |
| `+0x434` | `f32` | anchor speed read by [`fn_80063890`] |
| `+0x438` | `u8` | class-`0x23C` tag compared by [`fn_80063820`] |

## Constants resolved for this lane

Ghidra printed `unaff_r2` for every SDA float. With the base this module already
fixes (`r2 = 0x80433080`) the offsets resolve to:

| offset | address | value | use |
|---|---|---|---|
| `r2 - 0x75A8` | `0x8042BAD8` | `0x3B800000` = `1/256` | angle unit for the trig helpers |
| `r2 - 0x75A4` | `0x8042BADC` | `0x3F000000` = `0.5f` | half the body length (`pivot` term) |
| `r2 - 0x75B8` | `0x8042BAC8` | `0x41800000` = `16.0f` | per-segment step and `16 * count` |
| `r2 - 0x75A0` | `0x8042BAE0` | `0x41000000` = `8.0f` | loop's first offset (`length - 24`) |
| `r2 - 0x7598` | `0x8042BAE8` | `0x43300000` (high half) | `2^52` bias of the u32->float conversion |
| `r2 - 0x75B4` | `0x8042BACC` | `0x00000000` = `0.0f` | the `+0x648` timer compare |
| `r13 - 0x5458` | `0x8042A228` | (BSS) | callback pointer behind the `0x80064AF0` thunk |

`0x802EF318` holds `{8.0f, 8.0f, 16.0f}`, the cap offset indexed by `+0x640`.

## Reuse and new symbols

No existing constant or helper was redefined. The lane reuses
`FN_COS_80237C40_E` / `FN_SIN_80237BD0_E` + `call_f64_E`,
`FN_80162D00_E`, `FN_80162D50_E`, `FN_80162510_B`,
`W92_TRACK_RELEASE_8007FDA0`, `W72_RELEASE_8015F800`, `DEALLOC_802B9280`,
`call_interface_slot` and `call_interface_slot0`.
New lane symbols are suffixed `_A`: `FN_80064AF0_A`, `FN_801C0BD0_A`,
`FN_8016EEF0_A`, `FN_8016A170_A`, `FN_8016A150_A`, `FN_8016A1A0_A`,
`FN_80064F50_A`, `FN_80064D10_A`, `DAT_80355E10_A`,
`F32_ANGLE_UNIT_8042BAD8_A`, `W211A_CAP_OFFSET_TABLE_802EF318_A`,
`W211A_CLASS_VTABLE`, `W211A_MODEL_CONTEXT`, `W211A_MAIN_MODEL`,
`W211A_PART_BASE`, `W211A_CAP_MODEL`, `W211A_POSE`, `W211A_SCALE`,
`w211a_call_3f_1u` and `w211a_bind_part`. (`p32` and `call_3f_1u` are not
defined in `src/game/player.rs`, so the lane uses pointer casts and a local
`f32,f32,f32,u32` shim instead of assuming them.)

## Deferrals

None.

## Verification

A scratch copy of `src/`, `Cargo.toml` and `Cargo.lock` had
`work/wave211/laneA_block.rs` appended to `src/game/player.rs`;
`cargo check --lib` in that copy finished with **exit 0** and no errors.
