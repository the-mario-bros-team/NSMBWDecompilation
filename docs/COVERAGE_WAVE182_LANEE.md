# Coverage - Wave 182 lane E (`game::bases`, `0x8067BC30`..`0x806842D0`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane E block) | `game::bases` | 10 | 884 | 10 | 0 |

Ten bodies from the REL `d_basesNP`. Source of the bodies:
`work/wave182/laneE_block.rs`; per-function ledger:
`ledger/agents/wave182-lanee.csv`. Nothing was deferred.

## What this pass covers

- **Fire-effect actor probe helpers (`0x8067BC30`, `0x8067CD10`).** `0x8067BC30`
  is the short sibling of the already-translated `fn_80667d00`: `0x8006DB30` on
  `+0x1EC`, then `0x800D9940` on `+0x2DC`, OR `0x8000` into `+0x274` on a hit,
  then `0x8006FA40` and `0x8006EF40`, returning the raw flag word. `0x8067CD10`
  is the same prologue without the hit test and without the flag update.
- **Collision binding and effect dispatch (`0x8067C9C0`, `0x8067CAA0`).**
  `0x8067C9C0` calls `0x8006D090(+0x1EC, this, 0x807C9580, 0x807C958C,
  0x807C9598)` and caches `this + 0x2DC` at `+0x23C`. `0x8067CAA0` reads bit 20
  of the flags word at `+0x4`, picks `1.0` or `2.0` from the two-entry table at
  `0x807C95D8`, replicates it into a three-float vector, and calls the `+0x42C`
  interface through vtable slot `+0xA0` with
  `"Wm_en_pakkunfire"`, `0`, `this + 0xAC`, `0`, vector.
- **Four byte-identical release wrappers (`0x8067C340`, `0x8067D140`,
  `0x8067D1A0`, `0x8067F350`).** All run `0x8015F800(actor, 0)` and, when the
  free flag is positive, `0x802B9280(actor)`, returning `actor`.
- **Two heap factories (`0x8067D4F0`, `0x806842D0`).** `0x8067D4F0` allocates
  `0x3F8` bytes via `0x801628C0`, runs `0x80066FC0` and stores vtable
  `0x807EC830` at `+0x60`; the three neighbours at `0x8067D540`/`0x8067D590`/
  `0x8067D5E0` are byte-identical copies. `0x806842D0` allocates `0x8A8` bytes,
  calls the already-translated [`fn_80683750`], and stores vtable `0x807ED83C`
  at `+0x60`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8067BC30` | `fn_8067bc30` | 116 | high |
| `0x8067C340` | `fn_8067c340` | 92 | high |
| `0x8067C9C0` | `fn_8067c9c0` | 80 | high |
| `0x8067CAA0` | `fn_8067caa0` | 100 | high |
| `0x8067CD10` | `fn_8067cd10` | 76 | high |
| `0x8067D140` | `fn_8067d140` | 88 | high |
| `0x8067D1A0` | `fn_8067d1a0` | 92 | high |
| `0x8067D4F0` | `fn_8067d4f0` | 76 | high |
| `0x8067F350` | `fn_8067f350` | 88 | high |
| `0x806842D0` | `fn_806842d0` | 76 | high |

## Deferred

None.

## Corrections and notes

- **`0x806842D0` calls `0x80683750`, not `0x80684A30`.** The dispatcher's own
  doc comment on `fn_80684a30` says the allocator wrapper "runs it"; the raw
  branch at `0x806842F4` is `0x4BFFF45D`, i.e. `0x806842F4 - 0xBA4 = 0x80683750`,
  the base crumbling-board builder. The derived object then gets vtable
  `0x807ED83C` at `+0x60` (`lis 0x807F` + `addi -0x27C4`), which is a third
  vtable distinct from both `0x807ED3E8` (base) and `0x807EDB10`
  (`fn_80684a30`).
- **The release wrappers have a duplicated null guard.** `0x8067C340` and
  `0x8067D1A0` emit `bc 12,2` twice on the same `cr0` result
  (`0x8067C360`/`0x8067C364`, `0x8067D1C0`/`0x8067D1C4`). It is a compiler
  artifact; the translation keeps a single `if`.
- **`0x8067CAA0` extracts a single table index bit with `rlwinm 14,29,29`.** That
  is `(flags >> 18) & 4`, i.e. `0` or `4`, selecting `0x807C95D8 = 0x3F800000`
  (`1.0`) or `0x807C95DC = 0x40000000` (`2.0`) from the ELF.
- **`0x807EC5BC` is the string body, not the table entry.** The Ghidra export
  prints `s__Wm_en_pakkunfire_807ec5bb + 1`; the byte at `0x807EC5BB` is the
  `0x22` length prefix, and `0x807EC5BC` is `"Wm_en_pakkunfire"`.
- **`0x8067CAA0` keeps its vtable pointer at `+0x42C`.** The call is
  `lwzu r12,0x42C(r3)`, so the slot address `vtable + 0xA0` is loaded and
  invoked with `abi_call6!` rather than through a `vmethod*` helper.
- **`0x80066FC0` loses its `this` in the export.** In `0x8067D4F0` the allocator
  result is still in `r3` at the call site (`0x8067D514`), so the translation
  forwards `object`, matching the existing `fn_80651390` note.
- **The three-float vector in `0x8067CAA0` is a stack scratch block**
  (`r1+0x8`/`+0xC`/`+0x10`, three `stfs` of the same `f0`); it is passed by
  pointer as the sixth argument.

## Verification

- The lane-E block type-checks standalone (`rustc --edition 2021
  --emit=metadata --crate-type lib`) against stubbed copies of
  `platform::read_u32/read_f32/write_u32`, `platform::call1/2/4/5/6` and a
  `fn_80683750` stub; the harness lives outside the repo in the session temp
  directory (`%TEMP%/opencode/w182e`).
- Every export in `work/ghidra/rel_d_basesNP/export/` was read, and each byte
  size matches `functions.tsv` (116/92/80/100/76/88/92/76/88/76).
- Every constant was re-read from `work/rels/elf/d_basesNP.elf`:
  `0x807C95D8 = 1.0`, `0x807C95DC = 2.0`, `0x807EC5BC =
  "Wm_en_pakkunfire"`, vtable literals `0x807EC830`/`0x807ED83C`, descriptors
  `0x807C9580`/`0x807C958C`/`0x807C9598`.
- Only helpers that already exist in `src/game/bases.rs` are used:
  `abi_call1!`/`abi_call2!`/`abi_call4!`/`abi_call5!`/`abi_call6!`,
  `platform::read_u32`/`read_f32`/`write_u32`, and [`fn_80683750`].
- No `_savegpr`/`_restgpr` helper, GQR or `psq` artifact is emitted.
- `src/game/bases.rs` was not modified by this lane.
