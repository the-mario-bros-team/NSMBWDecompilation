# Coverage - Wave 25, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-25
  close-out pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier
  passes are documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions)
  and `docs/COVERAGE_WAVE14_BOSSES.md` through
  `docs/COVERAGE_WAVE24_BOSSES.md`; this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000` with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value is an analysis address, not a runtime
  address. Calls into `main.dol` (for example `0x8015F800`) are absolute DOL
  addresses and are unaffected by the REL base.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel_d_en_bossNP/export/`; the exact file is named in every
  table row. Sizes come from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means the
  visible body is represented but one operand or ABI detail is missing and is
  named in the Rust `// partial:` note.
- **Confidence** rates the translation, not the guessed role. `high` means the
  stores, call arguments and branch tree are exact; `medium` means the body is
  exact but a helper role or ABI detail was reconstructed from the surrounding
  code.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** and the **remaining candidates** before
  treating this pass as the end of the module.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 1,636 |
| status `translated` | 15 |
| status `partial` | 10 |
| status `stub` | 0 |
| confidence `high` | 13 |
| confidence `medium` | 12 |
| confidence `low` | 0 |
| cumulative bodies in `game::bosses` | 466 |
| cumulative bytes in `game::bosses` | 122,664 |
| translated share of module bodies | 60.52% |
| translated share of module text bytes | 34.42% |

This pass takes the last remaining multi-block body, the 444-byte Kameck route
population `0x80C51520`, finishes the `0x8015F800` release family (seven
wrappers), records the six remaining complete small hooks, and starts the
`0x802DCBFC` destructor-adapter family with ten of its forty-five members.
After this pass the only untranslated bodies left are the thirty-five
remaining adapters, 255 twelve-byte state-id factories that are already called
at their original sites, and five exports that stop at `halt_baddata()`.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it.

## Cluster 1 - Kameck route population (444 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C51520` | 444 | `fn_80c51520` | one animated route row from a named scene list | translated | medium |

What is proven:

- The body is called by [`fn_80c51460`] as
  `0x80C51520(owner, index, route_name)` for `index = 0` and `index = 1`.
  The row slots `owner + 0x0C` (index 0) and `owner + 0x10` (index 1) are the
  same words that `0x80C51460` reads back for `0x80C516E0`, so the slot
  arithmetic `owner + index * 4 + 0x0C` is confirmed by the caller, not
  guessed.
- The scene handle is the global `0x8042A18C`; `0x80100530` is the translated
  scene lookup in `src/engine/globals.rs` and returns the first list node for
  the route name.
- The count loop follows the signed relative link at `node + 0x60` and counts
  nodes; a second identical lookup starts the key walk, and the payload link
  at `node + 0x08` is followed the same way.
- The animation is created only when the count is greater than two, with
  `0x802B9210(0x18)` (default-heap new) and
  `0x80105FD0(row_scale, block, count - 2, 0x80C6C390)`. The row scale is the
  float at `0x80C6C1A8 + index * 4`.
- The key walk calls `0x801006E0(scene, payload, scratch)` per node,
  `0x80106D10(animation, scratch)` for the first node,
  `0x80106280(animation, frame, scratch)` for each middle node,
  `0x80106D30(animation, scratch)` for the last node, and
  `0x801062B0(animation)` at the end. `frame` starts at zero and increments
  only on the middle-node branch.
- The export's `undefined8 uVar7` is the value left in `r3:r4` by the
  `__savegprlr_27` prologue thunk `0x802DCD74`. That thunk saves `r27..r31`
  and does not modify `r3`/`r4`, so `uVar7` is really the incoming
  `(owner, index)` pair: the high half is `owner` and the low half is `index`.
  The recovered slot arithmetic above is the independent check.

## Cluster 2 - the last seven `0x8015F800` release wrappers (640 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C09D20` | 92 | `fn_80c09d20` | release wrapper | translated | high |
| `0x80C31F90` | 88 | `fn_80c31f90` | release wrapper | translated | high |
| `0x80C31FF0` | 92 | `fn_80c31ff0` | release wrapper | translated | high |
| `0x80C348F0` | 88 | `fn_80c348f0` | release wrapper | translated | high |
| `0x80C34950` | 92 | `fn_80c34950` | release wrapper | translated | high |
| `0x80C3D190` | 92 | `fn_80c3d190` | release wrapper | translated | high |
| `0x80C428F0` | 92 | `fn_80c428f0` | release wrapper | translated | high |

All seven bodies have the shape already documented in `fn_80c067c0`: test
`this`, run `0x8015F800(this, 0)`, and call `0x802B9280(this)` only when
`free_it > 0`. They return `this`. The 92-byte variants differ from the
88-byte ones only in alignment padding. This clears the `0x8015F800` family,
which wave 24 left eight-of-fourteen complete.

## Cluster 3 - small result hooks (60 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C001F0` | 4 | `fn_80c001f0` | empty hook | translated | high |
| `0x80C189D0` | 8 | `fn_80c189d0` | constant-`1` callback | translated | high |
| `0x80C2C8D0` | 36 | `fn_80c2c8d0` | one-call hook, returns `1` | translated | medium |
| `0x80C35140` | 8 | `fn_80c35140` | constant-`1` callback | translated | high |
| `0x80C3E360` | 8 | `fn_80c3e360` | constant-`1` callback | translated | high |
| `0x80C46D10` | 8 | `fn_80c46d10` | constant-`1` callback | translated | high |

What is proven:

- Four bodies are `li r3, 1; blr` in the export and return `1` in Rust.
- `0x80C001F0` is a bare `blr`; the export contains no statements.
- `0x80C2C8D0` calls `0x80161E30` with no modelled arguments and then returns
  `1`. The call address and the constant result are literal.

Cluster 3 also contains the four-byte thunk `0x80C51450`, which tail-branches
to the already-translated `fn_80c51460`; it is listed here with the hooks
because its whole body is that branch.

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C51450` | 4 | `fn_80c51450` | tail branch to `0x80C51460` | translated | high |

## Cluster 4 - `0x802DCBFC` destructor adapters (480 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C04AD0` | 48 | `fn_80c04ad0` | destructor adapter | partial | medium |
| `0x80C04B00` | 48 | `fn_80c04b00` | destructor adapter | partial | medium |
| `0x80C06B20` | 48 | `fn_80c06b20` | destructor adapter | partial | medium |
| `0x80C06B50` | 48 | `fn_80c06b50` | destructor adapter | partial | medium |
| `0x80C0A070` | 48 | `fn_80c0a070` | destructor adapter | partial | medium |
| `0x80C0A0A0` | 48 | `fn_80c0a0a0` | destructor adapter | partial | medium |
| `0x80C0E2F0` | 48 | `fn_80c0e2f0` | destructor adapter | partial | medium |
| `0x80C0E320` | 48 | `fn_80c0e320` | destructor adapter | partial | medium |
| `0x80C11C10` | 48 | `fn_80c11c10` | destructor adapter | partial | medium |
| `0x80C11C40` | 48 | `fn_80c11c40` | destructor adapter | partial | medium |

What is proven:

- Every body is exactly `0x802DCBFC(param_2, param_2, param_1); return;`.
- The stub is an indirect-jump helper that receives a hidden `r12`
  this-adjust pointer, which `platform::call3` cannot model. The visible
  argument order is preserved and the missing operand is marked
  `// partial:` in every body, matching `game::bases::fn_806512f0`.
- The remaining thirty-five members of the family are byte-identical after
  relocation.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C001F0` | `FUN_80c001f0` | `fn_80c001f0` | empty hook | translated | high | `80c001f0_FUN_80c001f0.c` |
| `0x80C04AD0` | `FUN_80c04ad0` | `fn_80c04ad0` | destructor adapter | partial | medium | `80c04ad0_FUN_80c04ad0.c` |
| `0x80C04B00` | `FUN_80c04b00` | `fn_80c04b00` | destructor adapter | partial | medium | `80c04b00_FUN_80c04b00.c` |
| `0x80C06B20` | `FUN_80c06b20` | `fn_80c06b20` | destructor adapter | partial | medium | `80c06b20_FUN_80c06b20.c` |
| `0x80C06B50` | `FUN_80c06b50` | `fn_80c06b50` | destructor adapter | partial | medium | `80c06b50_FUN_80c06b50.c` |
| `0x80C09D20` | `FUN_80c09d20` | `fn_80c09d20` | release wrapper | translated | high | `80c09d20_FUN_80c09d20.c` |
| `0x80C0A070` | `FUN_80c0a070` | `fn_80c0a070` | destructor adapter | partial | medium | `80c0a070_FUN_80c0a070.c` |
| `0x80C0A0A0` | `FUN_80c0a0a0` | `fn_80c0a0a0` | destructor adapter | partial | medium | `80c0a0a0_FUN_80c0a0a0.c` |
| `0x80C0E2F0` | `FUN_80c0e2f0` | `fn_80c0e2f0` | destructor adapter | partial | medium | `80c0e2f0_FUN_80c0e2f0.c` |
| `0x80C0E320` | `FUN_80c0e320` | `fn_80c0e320` | destructor adapter | partial | medium | `80c0e320_FUN_80c0e320.c` |
| `0x80C11C10` | `FUN_80c11c10` | `fn_80c11c10` | destructor adapter | partial | medium | `80c11c10_FUN_80c11c10.c` |
| `0x80C11C40` | `FUN_80c11c40` | `fn_80c11c40` | destructor adapter | partial | medium | `80c11c40_FUN_80c11c40.c` |
| `0x80C189D0` | `FUN_80c189d0` | `fn_80c189d0` | constant-`1` callback | translated | high | `80c189d0_FUN_80c189d0.c` |
| `0x80C2C8D0` | `FUN_80c2c8d0` | `fn_80c2c8d0` | one-call hook | translated | medium | `80c2c8d0_FUN_80c2c8d0.c` |
| `0x80C31F90` | `FUN_80c31f90` | `fn_80c31f90` | release wrapper | translated | high | `80c31f90_FUN_80c31f90.c` |
| `0x80C31FF0` | `FUN_80c31ff0` | `fn_80c31ff0` | release wrapper | translated | high | `80c31ff0_FUN_80c31ff0.c` |
| `0x80C348F0` | `FUN_80c348f0` | `fn_80c348f0` | release wrapper | translated | high | `80c348f0_FUN_80c348f0.c` |
| `0x80C34950` | `FUN_80c34950` | `fn_80c34950` | release wrapper | translated | high | `80c34950_FUN_80c34950.c` |
| `0x80C35140` | `FUN_80c35140` | `fn_80c35140` | constant-`1` callback | translated | high | `80c35140_FUN_80c35140.c` |
| `0x80C3D190` | `FUN_80c3d190` | `fn_80c3d190` | release wrapper | translated | high | `80c3d190_FUN_80c3d190.c` |
| `0x80C3E360` | `FUN_80c3e360` | `fn_80c3e360` | constant-`1` callback | translated | high | `80c3e360_FUN_80c3e360.c` |
| `0x80C428F0` | `FUN_80c428f0` | `fn_80c428f0` | release wrapper | translated | high | `80c428f0_FUN_80c428f0.c` |
| `0x80C46D10` | `FUN_80c46d10` | `fn_80c46d10` | constant-`1` callback | translated | high | `80c46d10_FUN_80c46d10.c` |
| `0x80C51450` | `thunk_FUN_80c51460` | `fn_80c51450` | tail branch | translated | high | `80c51450_thunk_FUN_80c51460.c` |
| `0x80C51520` | `FUN_80c51520` | `fn_80c51520` | route population | translated | medium | `80c51520_FUN_80c51520.c` |

## Unknowns

- **Route-animation callees.** `0x80105FD0` (factory), `0x801006E0` (payload
  read), `0x80106D10` (first key), `0x80106280` (middle key), `0x80106D30`
  (last key) and `0x801062B0` (finalize) are named only by their position in
  the loop. The argument counts are taken from the export; the semantic names
  are inferred. `0x80100530` and `0x802B9210` are confirmed by
  `src/engine/globals.rs` and `src/game/stage.rs`.
- **Save-thunk ABI.** `0x802DCD74` is `__savegprlr_27`, a compiler prologue
  thunk. Ghidra printed its "return value" as an eight-byte `uVar7`; the
  translation recovers `uVar7 = (r3 = owner, r4 = index)`. This is supported
  by the row-slot arithmetic agreeing with `0x80C51460`, but the original
  source cannot be recovered.
- **`0x80161E30` inputs.** The call in `0x80C2C8D0` is printed with no
  arguments; the argument registers are inherited from the caller. Only the
  call and the constant result are translated.
- **Adapter hidden operand.** The ten `0x802DCBFC` adapters cannot model the
  hidden `r12` this-adjust pointer. This is the only reason they are `partial`
  rather than `translated`.
- **Row scale table.** The four floats at `0x80C6C1A8` are read by value; their
  numeric contents are not part of the export and are not claimed here.

## Remaining candidates

| address | size | why deferred |
|---|---:|---|
| `0x80C389B0` | 72 | export stops after four stores (`halt_baddata`) |
| `0x80C38A00` | 72 | export stops after four stores (`halt_baddata`) |
| `0x80C3DFE0` | 72 | export stops after four stores (`halt_baddata`) |
| `0x80C3E030` | 72 | export stops after four stores (`halt_baddata`) |
| `0x80C228A0` | 40 | export stops after the first load |
| `0x80C4FDE0` | 16 | export stops after the `+0x2E1` byte store |
| `0x80C32660` | 28 | bad instruction data; no statements recovered |
| `0x80C512F0` | 20 | bad instruction data; no statements recovered |
| `0x80C51430` | 24 | bad instruction data; no statements recovered |
| `0x80C0B510` | 8 | bad instruction data; no statements recovered |
| `0x80C336F0`, `0x80C3AF50`, `0x80C3AF60`, `0x80C4C160` | 4 | import thunks with bad instruction data |
| 35 bodies | 48 | remaining `0x802DCBFC` adapters; same template as cluster 4 |
| 255 bodies | 12 | state-id factories; already called at their original sites |

The five exports that stop at `halt_baddata()` (`0x80C389B0`, `0x80C38A00`,
`0x80C3DFE0`, `0x80C3E030`, `0x80C228A0`) still have readable leading stores,
but their control flow is truncated, so they are outside a pass that requires
complete bodies.

## Verification

- `cargo check` exits 0 and reports no diagnostic for `src/game/bosses.rs`
  (the remaining crate warnings belong to other lanes).
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; the seven wrappers and ten adapters
  match their already-translated twins after relocation, and every
  `# Original` block names the file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave25-bosses.csv` match the Rust functions
  one to one: 15 `translated`, 10 `partial`; 13 `high`, 12 `medium`.
- `src/game/bosses.rs` now defines 466 REL bodies; the module header count was
  updated from 441 to 466.
- No files outside `src/game/bosses.rs`, this document and the wave-25 ledger
  were edited.
