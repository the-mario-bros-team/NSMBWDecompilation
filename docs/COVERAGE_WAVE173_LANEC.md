# Coverage - Wave 173, lane C (`game::bases`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane C block) | `game::bases` | 4 | 1,440 | 4 | 0 |

Lane C of wave 173: four REL bodies, each 360 bytes, from four separate
address ranges. All four are translated; nothing is deferred.

## What this pass covers

- **`BGM_anim_dummy` camera hand-off (2 bodies).** `0x8064F510` and `0x8064F820`
  are further members of the [`fn_8064f390`]/[`fn_8064f6a0`] family already in
  `bases.rs`, but they are the *camera-guard* arm rather than the
  clip-length-gate arm. They test `+0x600` (mode `0`/`1`) against two camera
  bytes at `0x8082_83E8 + 0x3F4..0x3F7`, rebind the `+0x5F8`/`+0x5F4` clip into
  the `+0x5BC`/`+0x584` mover through `0x801651F0`, copy the camera speed
  (`+0x3EC`/`+0x3F0`) straight into `+0x5FC`, apply it with `0x801680E0`,
  restart the clip with `0x80168FE0`/`0x80169F80(0.0, ...)`, and dispatch the
  `0xD4` callback with `0x8082_8338` (`F510`) or `0x8082_82F8` (`F820`). They
  do **not** call `0x8064EC70` and do **not** divide by `0x806505B0`; the speed
  is copied, not computed.
- **Death sequence (1 body).** `0x80693BE0` masks `+0x4` with `0xF`: state `1`
  resolves `0x80162D00(+0x594)` and spawns `Wm_en_explosion` plus
  `Wm_en_bombheibreak` into the target's `+0x62C`, queues sound `0x1CC` and
  calls `0x80693D50(this, 0)`; state `2` sets the target's `+0x556` byte.
  Every state then queues sound `0x26C`, builds the effect point
  `(x, y + 32.0, 400.0)` and spawns `0x80092720` with the random variant
  `0x800B2EE0(2) | 0x20`, before `0x80064D10(this, 1)`.
- **Subtree id collector (1 body).** `0x806C97D0` recursively appends unique
  actor ids to a word array. `+0x8` type `0x19B` selects the group list at
  `+0x460`, `0x1A1` the list at `+0x4CC`; both walk `+0xC` then `+0x6C`. With no
  group it follows `+0x38C == 1` (name-byte `< 4`, recurse into
  `0x80162D00(+0x2A78)`) or `+0x38C == 3` (append only). Deduplication and the
  15-entry bound are the existing [`fn_806c9760`].

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8064F510` | `fn_8064f510` | 360 | high |
| `0x8064F820` | `fn_8064f820` | 360 | high |
| `0x80693BE0` | `fn_80693be0` | 360 | high |
| `0x806C97D0` | `fn_806c97d0` | 360 | high |

## Corrections and notes

- `0x8064F510`/`0x8064F820`: both exports lose the `f1` argument of
  `0x801680E0` (`0x8064_F588` / `0x8064_F898` load it with `lfs` from the camera
  record and the value is still live at the `bl`). The bodies pass that value
  explicitly through `call_f32_u32`, matching how [`fn_8064f390`] handles the
  same call.
- `0x8064F510`: Ghidra hoists `DAT_808283e8` to entry and reuses it for the
  `+0x3EC`/`+0x3F0` speed read; the body keeps one load, as the raw `lwz
  r30,-0x7c18(r4)` at `0x8064_F534` does.
- `0x80693BE0`: `0x80092720` has no Rust body in `bases.rs` and stays
  address-only (`abi_call4!`); its flag word is the `ori 0x20` of
  `0x8069_3D0C`. `0x80693D50` is an unclaimed sibling in the same range, also
  address-only.
- `0x806C97D0`: the export's `uVar7 = func_0x802dcd74()` / `func_0x802dcdc0()`
  pair is `_savegpr_27`/`_restgpr_27` (`bl 0x802DCD74`, `bl 0x802DCDC0`). The
  helpers leave `r3..r5` intact, so the body takes `this`, `entries` and
  `count` straight from the entry registers and returns `count`; the
  `puVar1 != 0` test is the surviving `r3 == 0` compare at `0x806C_97E4`.
  The `lbz`/`extsb` at `0x806C_9898`/`0x806C_989C` (`X31_954` in the export) is
  an unsigned `< 4` test on the name byte.

## Verification

- The lane block type-checks against stub `platform`/`abi_call*` definitions
  (`rustc --crate-type lib`, exit 0); it is not compiled into `bases.rs` because
  lane blocks are appended by the coordinator.
- Every constant was re-read from `work/rels/elf/d_basesNP.elf`:
  `0x807C_8708 = 0.0`, `0x807C_9E6C = 32.0`, `0x807C_9E70 = 400.0`.
- Strings `0x807F_0730 = "Wm_en_explosion"` and
  `0x807F_071C = "Wm_en_bombheibreak"` were read from the ELF bytes.
- All four evidence exports exist under `work/ghidra/rel-d_basesNP/export/` and
  their byte sizes match `functions.tsv` (360 each).
