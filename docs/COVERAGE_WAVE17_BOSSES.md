# Coverage - Wave 17, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-17
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions),
  `docs/COVERAGE_WAVE14_BOSSES.md` (25), `docs/COVERAGE_WAVE15_BOSSES.md` (25)
  and `docs/COVERAGE_WAVE16_BOSSES.md` (25). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8005xxxx` through `0x8042xxxx`) are
  absolute DOL addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every table
  row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` is not used in
  this pass.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores, call arguments and branch tree are exact; `medium` means the
  body is exact but a dropped argument, an object role or a helper role is
  reconstructed by convention or from a sibling.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 2,860 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 17 |
| confidence `medium` | 8 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 291 |
| cumulative bytes in `game::bosses` | 93,948 |

The pass closes the small-body tail of the module: state enter/exit hooks, the
`+0x70`/`+0x78` input-flag helpers, the short collision and damage selectors,
and the scenario/phase selectors that the large AI runners call. It also adds
the `face_1` heading helper that receives its joint directly, complementing the
wave-16 variant.

## Synthetic-base warning in one place

| item | value |
|---|---:|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The BSS words at
`0x80C6FF7C`, `0x80C702BC` and `0x80C6C81C` are analysis-image words, not
hardware addresses.

## Cluster 1 - state enter/exit hooks (344 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C38FC0` | 16 | `fn_80c38fc0` | tick the embedded `+0xEC4` sub-object | translated | medium |
| `0x80C38B50` | 112 | `fn_80c38b50` | exit sound, phase notify, counter clear | translated | high |
| `0x80C38BC0` | 112 | `fn_80c38bc0` | mirror exit order | translated | high |
| `0x80C38EE0` | 104 | `fn_80c38ee0` | tick and notify without counter clear | translated | high |

What is proven:

- All three hooks call class slot `+0x658` on the actor vtable at `+0x60` and,
  when it returns non-zero, send the descriptor `0x8082CDC8` through the
  child's own class slot `+0xD4`.
- `0x80C38B50` calls `0x800A9190` first, clears the 16-bit counter at `+0xEB8`,
  then calls the `+0xEC4` tick. `0x80C38BC0` calls `0x800A9280`, ticks first,
  then clears the counter. `0x80C38EE0` calls `0x800A99D0`, ticks, and leaves
  the counter untouched.
- `0x80C38FC0` is a single virtual call through the `+0xEC4` vtable slot
  `+0x10`. The receiver is the sub-object address by the embedded-object
  convention used by `0x80C06C90`, which installs the `0x80329958` vtable at
  the same offset.

## Cluster 2 - input and flag helpers (532 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C560B0` | 120 | `fn_80c560b0` | three callbacks gated by `+0x78`/`+0x70` | translated | medium |
| `0x80C561E0` | 84 | `fn_80c561e0` | set/clear a flag and reset `+0x150` | translated | high |
| `0x80C562A0` | 148 | `fn_80c562a0` | countdown or dispatch the `0x80` flag | translated | high |
| `0x80C56950` | 180 | `fn_80c56950` | `0.2` position blend and previous copy | translated | high |

What is proven:

- `0x80C560B0` uses the vtable at `+4`, not `+0x60`; slots `+0x64`, `+0x44`
  and `+0x68` run only when `+0x78 & 0x10` is set and `+0x70 & 0x40` is clear.
  The first slot is printed without an argument; the receiver is passed by
  convention and the row is `medium`.
- `0x80C561E0` preserves the comma-expression side effect: `+0x150` is zeroed
  whenever `flag == 1`, before the `+0x70 & 1` and `enable == 1` tests.
- `0x80C562A0` computes `leading_zeros(flags & 0x80) >> 5` (0 for `0x80`, 1 for
  zero) and sends it through slot `+0x48`; a non-zero `+0x150` is decremented
  instead. Slot `+0x4C` runs whenever `+0x70 & 0x10` is set.
- `0x80C56950` is `new * 0.8 + old * 0.2` per component using `0x80C5933C`
  (`1.0`) and `0x80C59374` (`0.2`), then copies the result into `+0x30..+0x38`.

## Cluster 3 - collision and damage (612 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C3E370` | 136 | `fn_80c3e370` | side-step water-column probe | translated | medium |
| `0x80C3E400` | 84 | `fn_80c3e400` | growth and vertical-speed clamp | translated | high |
| `0x80C3E4C0` | 160 | `fn_80c3e4c0` | two-way damage descriptor select | translated | high |
| `0x80C476D0` | 128 | `fn_80c476d0` | `0x529`/`0x52A` clip select | translated | high |
| `0x80C47750` | 96 | `fn_80c47750` | `+0x4B4` acceleration and state switch | translated | high |
| `0x80C4B700` | 104 | `fn_80c4b700` | model-space floor snap | translated | medium |

What is proven:

- `0x80C3E370` probes `(x + table[side], y + 8, z)` where the two-entry table
  at `0x80C58848` is `10.0`/`-10.0`; the origin word is `0.0`
  (`0x80C58824`). It returns `1` when `0x800757B0` is non-zero.
- `0x80C3E400` grows `+0xE90` by `0.009375` and clamps it at `0.15`, then adds
  the clamped value to `+0xEC` and clamps that at `2.0`.
- `0x80C3E4C0` and `0x80C476D0` share the controller query shape of the wave-16
  `0x80C43FD0`/`0x80C477B0` pair. The first selects `0x80C702BC` or
  `0x80C6FF7C`; the second selects clip `0x529` or `0x52A` on slot `+0x28`.
- `0x80C47750` is the second half of `0x80C477B0` with the `0x803585C4` test
  removed: slot `+0x4B4` → `+0x114`, `+0x848 = 1`, then `0x80358204`.
- `0x80C4B700` builds a point with `0x800E7880`, subtracts `+0xD4` from its
  `y`, and only raises `+0xB0` (one-way clamp).

## Cluster 4 - aim, countdown and phase (772 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C0EB30` | 84 | `fn_80c0eb30` | height to clamped slot index | translated | high |
| `0x80C0EBA0` | 80 | `fn_80c0eba0` | count acceleration steps | translated | high |
| `0x80C41490` | 208 | `fn_80c41490` | `face_1` heading offset | translated | medium |
| `0x80C430D0` | 108 | `fn_80c430d0` | `+0xE7C` countdown and expiry | translated | high |
| `0x80C43490` | 108 | `fn_80c43490` | four-word angle block | translated | high |
| `0x80C446F0` | 112 | `fn_80c446f0` | side normalization and `0.3` rate | translated | high |
| `0x80C50460` | 128 | `fn_80c50460` | scene-dependent `+0x2E4` phase | translated | high |
| `0x80C525A0` | 120 | `fn_80c525a0` | scenario phase id | translated | medium |

What is proven:

- `0x80C0EB30` subtracts `+0xED0`, divides by `61.3333`, clamps to `0..5` and
  truncates. The call site in `0x80C0EE40` already used this shape.
- `0x80C0EBA0` is a `do/while` that adds `+0x114` to the table word at
  `[+0x754]+0x20` while the value stays above zero.
- `0x80C41490` is the wave-16 `0x80C36F60` body with the joint in `param_3`:
  `0x8023B300` → `+8` name → `0x802E1AB4("face_1")`, then
  `angle * 90 / 0x4000` added to the third rotation component.
- `0x80C430D0` ticks the `+0x394` chain slot `+0x10`, decrements `+0xE7C`,
  calls `0x80C43900` at zero, then `0x80064F50`.
- `0x80C43490` writes `12.0`, `19.0`, `12.0` to `+0x168/+0x16C/+0x170` and a
  signed zero to `+0x164` (the integer side converted through the bias at
  `0x80C58B20` and multiplied by `0.0`).
- `0x80C446F0` uses `0x80096240`, stores the side at `+0x348`, clears bit
  `0x200` of `+0x120`, sets `+0x500 = 200`, and selects `+0xF4` from the
  `4.0`/`-4.0` table at `0x80C58A38`.
- `0x80C50460` only runs when `0x800C9610(actor, 0)` is zero, writes `-1` to
  `+0x2F4..+0x300`, and selects phase `3` or `6` from the scene words.
- `0x80C525A0` reads the global at `0x8042A1A0` (`+0x240`) and the staging word
  at `0x80C6C81C`.

## Cluster 5 - per-frame position refresh (116 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C2E720` | 116 | `fn_80c2e720` | re-read position from slot `+0x3F4` | translated | medium |

What is proven:

- When `+0x7AC` is non-zero, class slot `+0x3F4` returns a position source that
  `0x800A4270` converts into three words; those words overwrite
  `+0xAC/+0xB0/+0xB4`.
- `0x8009B540(actor)` runs afterwards and the function always returns `1`.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C0EB30` | `FUN_80c0eb30` | `fn_80c0eb30` | slot index from height | translated | high | `80c0eb30_FUN_80c0eb30.c` |
| `0x80C0EBA0` | `FUN_80c0eba0` | `fn_80c0eba0` | acceleration step count | translated | high | `80c0eba0_FUN_80c0eba0.c` |
| `0x80C2E720` | `FUN_80c2e720` | `fn_80c2e720` | position refresh | translated | medium | `80c2e720_FUN_80c2e720.c` |
| `0x80C38B50` | `FUN_80c38b50` | `fn_80c38b50` | exit hook | translated | high | `80c38b50_FUN_80c38b50.c` |
| `0x80C38BC0` | `FUN_80c38bc0` | `fn_80c38bc0` | mirror exit hook | translated | high | `80c38bc0_FUN_80c38bc0.c` |
| `0x80C38EE0` | `FUN_80c38ee0` | `fn_80c38ee0` | tick and notify | translated | high | `80c38ee0_FUN_80c38ee0.c` |
| `0x80C38FC0` | `FUN_80c38fc0` | `fn_80c38fc0` | `+0xEC4` tick | translated | medium | `80c38fc0_FUN_80c38fc0.c` |
| `0x80C3E370` | `FUN_80c3e370` | `fn_80c3e370` | side water-column probe | translated | medium | `80c3e370_FUN_80c3e370.c` |
| `0x80C3E400` | `FUN_80c3e400` | `fn_80c3e400` | growth/speed clamp | translated | high | `80c3e400_FUN_80c3e400.c` |
| `0x80C3E4C0` | `FUN_80c3e4c0` | `fn_80c3e4c0` | damage descriptor select | translated | high | `80c3e4c0_FUN_80c3e4c0.c` |
| `0x80C41490` | `FUN_80c41490` | `fn_80c41490` | `face_1` heading offset | translated | medium | `80c41490_FUN_80c41490.c` |
| `0x80C430D0` | `FUN_80c430d0` | `fn_80c430d0` | countdown | translated | high | `80c430d0_FUN_80c430d0.c` |
| `0x80C43490` | `FUN_80c43490` | `fn_80c43490` | angle block | translated | high | `80c43490_FUN_80c43490.c` |
| `0x80C446F0` | `FUN_80c446f0` | `fn_80c446f0` | side normalization | translated | high | `80c446f0_FUN_80c446f0.c` |
| `0x80C476D0` | `FUN_80c476d0` | `fn_80c476d0` | clip select | translated | high | `80c476d0_FUN_80c476d0.c` |
| `0x80C47750` | `FUN_80c47750` | `fn_80c47750` | acceleration/state switch | translated | high | `80c47750_FUN_80c47750.c` |
| `0x80C4B700` | `FUN_80c4b700` | `fn_80c4b700` | floor snap | translated | medium | `80c4b700_FUN_80c4b700.c` |
| `0x80C4B870` | `FUN_80c4b870` | `fn_80c4b870` | type-`0x107` notify | translated | medium | `80c4b870_FUN_80c4b870.c` |
| `0x80C50460` | `FUN_80c50460` | `fn_80c50460` | phase init | translated | high | `80c50460_FUN_80c50460.c` |
| `0x80C525A0` | `FUN_80c525a0` | `fn_80c525a0` | scenario phase id | translated | medium | `80c525a0_FUN_80c525a0.c` |
| `0x80C560B0` | `FUN_80c560b0` | `fn_80c560b0` | three-callback gate | translated | medium | `80c560b0_FUN_80c560b0.c` |
| `0x80C561E0` | `FUN_80c561e0` | `fn_80c561e0` | flag set/clear | translated | high | `80c561e0_FUN_80c561e0.c` |
| `0x80C562A0` | `FUN_80c562a0` | `fn_80c562a0` | countdown/dispatch | translated | high | `80c562a0_FUN_80c562a0.c` |
| `0x80C56950` | `FUN_80c56950` | `fn_80c56950` | position blend | translated | high | `80c56950_FUN_80c56950.c` |
| `0x80C56B20` | `FUN_80c56b20` | `fn_80c56b20` | input mask map | translated | high | `80c56b20_FUN_80c56b20.c` |

## Data words read from the analysis image

These values are quoted so the constants in the Rust bodies are auditable.
They are analysis-image words, not runtime addresses.

| address | value | used by |
|---|---|---|
| `0x80C57660` | `61.333332` | `fn_80c0eb30` |
| `0x80C57690` | `0.0` | `fn_80c0eb30`, `fn_80c0eba0` |
| `0x80C5769C` | `5.0` | `fn_80c0eb30` |
| `0x80C58824` | `0.0` | `fn_80c3e370` |
| `0x80C5882C` | `8.0` | `fn_80c3e370` |
| `0x80C58848` / `0x80C5884C` | `10.0` / `-10.0` | `fn_80c3e370` |
| `0x80C58854` | `0.0625` | `fn_80c3e400` |
| `0x80C58A38` / `0x80C58A3C` | `4.0` / `-4.0` | `fn_80c446f0` |
| `0x80C58AE0` | `0.0` | `fn_80c43490` |
| `0x80C58AE4` | `12.0` | `fn_80c43490` |
| `0x80C58AE8` | `19.0` | `fn_80c43490` |
| `0x80C58AEC` | `12.0` | `fn_80c43490` |
| `0x80C58B20` | double bias `2^52 + 2^31` | `fn_80c43490` |
| `0x80C58B50` | `0.3` | `fn_80c446f0` |
| `0x80C5933C` | `1.0` | `fn_80c56950` |
| `0x80C59374` | `0.2` | `fn_80c56950` |
| `0x80C67554` | `2.0` | `fn_80c3e400` |
| `0x80C67558` | `0.15` | `fn_80c3e400` |
| `0x80C6AB2C` | signed `1` | `fn_80c4b700` |
| `0x80C6C81C` | signed `1` (staging word) | `fn_80c525a0` |
| `0x80C6FF7C` | BSS descriptor | `fn_80c3e4c0` |
| `0x80C702BC` | BSS descriptor | `fn_80c3e4c0` |

## Unknowns

- **Dropped arguments.** `0x800A4270` is printed with two arguments and
  `0x8009B540` with one, but their roles are not proven; the call shapes are
  preserved. Class slot `+0x3F4` in `fn_80c2e720` and slot `+0x64` in
  `fn_80c560b0` are printed with no argument and receive the actor by
  convention, so those two rows are `medium`.
- **List probe.** `0x80162D50` is printed as `(type)` in `fn_80c4b870`. The
  sibling `fn_80c4b0e0` proves the real shape is `(type, previous)`; the
  translation walks the list with that cursor. The export's recursive call also
  carries a second argument, which supports the same reading.
- **Embedded sub-object.** `0x80C38FC0` calls slot `+0x10` of the vtable at
  `+0xEC4` with no printed argument. The receiver is the sub-object address by
  the convention used by `0x80C06C90`.
- **Unnamed helpers.** `0x800A9190`, `0x800A9280`, `0x800A99D0`, `0x800A9EC0`,
  `0x800A4270`, `0x8009B540` and `0x80C43900` are still called by raw address.
- **Scene words.** `0x80315B98` is read as a byte (matching the display lane)
  and `0x80315B9D` as the byte compared with `')'`; both are DOL globals.
- **Staging word.** `0x80C6C81C` is a 4-byte analysis-image word with value
  `1`; its gameplay meaning (a scenario/staging selector) is inferred from the
  `4`/`5` mode test.
- **Opaque kinds.** The type `0x107` notification in `fn_80c4b870` and the
  descriptors `0x8082CDC8`/`0x8082C600` are numeric ids only.

## Verification

- `cargo check` passes (exit 0) and reports no diagnostics for
  `src/game/bosses.rs` (`--message-format short`).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave17-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 17 `high`, 8 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-17 ledger
  were edited.
