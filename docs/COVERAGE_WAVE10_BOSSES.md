# Coverage - Wave 10, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-10
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). The first-pass 23
  functions are documented in `docs/COVERAGE_REL_d_en_bossNP.md`; wave 3 adds
  20 (`docs/COVERAGE_WAVE3_BOSSES.md`), wave 4 adds 24
  (`docs/COVERAGE_WAVE4_BOSSES.md`), wave 5 adds 25
  (`docs/COVERAGE_WAVE5_BOSSES.md`), and wave 6 adds 25
  (`docs/COVERAGE_WAVE6_BOSSES.md`). This file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value here is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8015xxxx`, `0x8016xxxx`, `0x8006xxxx`,
  `0x8009xxxx`, `0x802Dxxxx`, `0x8042xxxx`) are absolute and unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
  Sizes are from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing; `stub` is not used here.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores and calls are exact and the role is established by a sibling
  function or a string. `medium` means the stores and branches are exact but
  an object role, a dropped argument or an untranslated helper is inferred.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. Strings appear only where the export points at the exact
  data address.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 5,268 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 18 |
| confidence `medium` | 7 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 142 |
| cumulative bytes in `game::bosses` | 66,260 |
| cumulative share of 770 functions | 18.44% |
| cumulative share of 356,396 text bytes | 18.59% |

The pass closes the middle of the Castle-Iggy call graph that the earlier
waves left untouched: the two remaining constructors and their teardown
wrappers, four clip/state entries, four per-frame handlers, five effect
spawners, a heading step, a collision predicate, a nearest-child selector and
the builder that creates and links the six phase objects at `+0xEA0`.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The constants read at
`0x80C573xx`, `0x80C574xx`, `0x80C576xx`, `0x80C5ADxx`, `0x80C5B8xx`,
`0x80C5C6xx` and `0x80C6xxxx` are analysis-image data words, not hardware
addresses.

## Cluster 1 - constructors and teardown (1,100 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C067C0` | 92 | `fn_80c067c0` | null-guarded release wrapper | translated | high |
| `0x80C06B80` | 208 | `fn_80c06b80` | construct the 0x1138-byte object | translated | medium |
| `0x80C06C90` | 152 | `fn_80c06c90` | destroy the 0x1138-byte object | translated | high |
| `0x80C0A0D0` | 312 | `fn_80c0a0d0` | construct the 0x12E8-byte object and two arrays | translated | medium |
| `0x80C0A250` | 88 | `fn_80c0a250` | initialize one 0x128-byte array element | translated | medium |
| `0x80C0A350` | 88 | `fn_80c0a350` | destroy one 0x10-byte array element | translated | high |
| `0x80C0E380` | 160 | `fn_80c0e380` | initialize the `+0x1004` tail fields and phase sub-object | translated | high |

What is proven:

- Both constructors allocate through `0x801628C0` and are bracketed by the
  same lock helpers (`0x802DCD74` / `0x802DCDC0`).
- `0x80C06B80` installs vtables `0x80C663E0`, `0x80C66A50` and `0x80C5AD98`;
  `0x80C0A0D0` installs `0x80C676C8`, `0x80C67D78` and `0x80C5B970`. The
  sub-object vtables are `0x80310ED8` in both constructors.
- `0x80C0A0D0` builds a two-element array of `0x128` bytes at `+0xEC4`
  (constructor `0x80C0A250`, destructor `0x80C005B0`) and a three-element
  array of `0x10` bytes at `+0x114C` (constructor `0x80C0A330`, destructor
  `0x80C0A350`). The `0x802DC9A0` argument order is
  `(base, ctor, dtor, element_size, count)`.
- `0x80C0E380` zeroes exactly the six phase slots `+0xEA0..+0xEB4` that the
  two `+0x668/+0x66C` link fields and the wave-4/5 handlers use.
- The three release wrappers (`0x80C067C0`, `0x80C0A350`, `0x80C06C90`)
  use the same `if (this) { teardown; if (free_it > 0) release; }` shape.

## Cluster 2 - clip/state entries and per-frame handlers (1,832 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C07990` | 220 | `fn_80c07990` | bind `throw_ed_wait`, clear motion, state `0x1E` | translated | high |
| `0x80C07A80` | 272 | `fn_80c07a80` | gravity/floor probe and phase dispatch | translated | high |
| `0x80C07B90` | 244 | `fn_80c07b90` | bind `jump_st`, set facing and launch | translated | high |
| `0x80C081B0` | 480 | `fn_80c081b0` | jump/land handler with side-aware heading | translated | medium |
| `0x80C0B810` | 220 | `fn_80c0b810` | bind `jump_HB2` and launch from a random rig | translated | high |
| `0x80C0B900` | 224 | `fn_80c0b900` | descent handler and floor-line transition | translated | high |
| `0x80C0B9E0` | 112 | `fn_80c0b9e0` | play clip `0x4C8` and start a fall | translated | medium |
| `0x80C0C9F0` | 60 | `fn_80c0c9f0` | reset action counter and contact block | translated | high |

What is proven:

- The per-frame ritual is the same as wave 5: model slot `+0x604/+0x1C`,
  physics `0x8006CDE0`, `0x800955F0`, ground probe `0x80072BC0(+0x1EC)`.
- `0x80C07990` and `0x80C07B90` are the two remaining direct clip entries in
  the `0x80C57xxx` string band; the names are pointers at `0x80C5AD6C`
  (`throw_ed_wait`) and `0x80C5AD80` (`jump_st`).
- `0x80C07B90` writes the facing byte at `+0x348` as a boolean comparison
  between the target x `+0xE84` and the actor x `+0xAC`.
- `0x80C081B0` derives the signed heading `+0x1004` from the side table at
  `0x8042C1A0` and class slot `+0x520`, then calls the heading step
  `0x80C077C0`; this is the only call site of that helper in the pass.
- `0x80C0B900` compares the actor y `+0xB0` against the reference height
  `+0x113C + 0x80C5B858` before firing class slot `+0xD4`.

## Cluster 3 - effects, aim and selection (1,852 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C06EB0` | 348 | `fn_80c06eb0` | spawn ids `0x1F`/`0x1F`/`0x20` around `+0xE84` | translated | high |
| `0x80C077C0` | 168 | `fn_80c077c0` | step a 16-bit heading and report overshoot | translated | high |
| `0x80C0A6F0` | 412 | `fn_80c0a6f0` | spawn the three rig effects at `+0x1140` | translated | high |
| `0x80C0A890` | 124 | `fn_80c0a890` | spawn effect `0x187` above the actor | translated | high |
| `0x80C0AC50` | 264 | `fn_80c0ac50` | collision-cell query and impact sound | translated | medium |
| `0x80C0B2C0` | 148 | `fn_80c0b2c0` | select a duration by two class-slot queries | translated | medium |
| `0x80C0B440` | 156 | `fn_80c0b440` | nearest rig by x distance | translated | high |
| `0x80C0B520` | 232 | `fn_80c0b520` | spawn `Wm_ob_witchcraftup` at the rig | translated | high |

What is proven:

- All effect spawns use the same five-argument factory `0x80064610(id, 0,
  &pos, 0, 0)` that wave 5 documented for `0x80C00900`/`0x80C00980`.
- `0x80C0A6F0`'s nine-word offset table is exactly
  `[-s, c0, c1, c1, c2, c1, +s, c0, c1]` with `s = 0x80C57480 *
  0x80C5B83C`; the three handles land at `+0x1140/+0x1144/+0x1148`, the
  first two are linked to the third's `+0x6A0` block through `+0x74C`.
- `0x80C0B520` is the second `Wm_ob_witchcraftup` spawner and mirrors
  `0x80C07870` (translated in wave 6) with a different position source and
  scale table.
- `0x80C0AC50` is the only collision query in the pass: grid
  `0x80429DD0`, signed 16-bit x/y, side byte `+0x38F`, result `0x30`, then
  the impact sound through `0x80429EE8`.
- `0x80C0B440` computes the same abs/bound minimum over the three rig x
  values that the wave-4 sled family uses for member selection.

## Cluster 4 - speed and the six-object chain (484 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---:|---|
| `0x80C0BEA0` | 96 | `fn_80c0bea0` | deceleration from current horizontal speed | translated | high |
| `0x80C0E4C0` | 388 | `fn_80c0e4c0` | spawn and link the six phase objects | translated | high |

What is proven:

- `0x80C0BEA0` uses the `0x43300000` integer-to-double idiom with the bias at
  `0x80C57490`, then divides `|+0xE8|` by that value.
- `0x80C0E4C0` spawns six effects of id `0x10E` with x spacing
  `0x80C57660`, writes their handles to `+0xEA0..+0xEB4`, and runs exactly two
  link passes of three slots each. The previous pointer goes to `+0x66C`, the
  next pointer to `+0x668`; the chain head and tail fields are the
  pre-existing `+0xE9C` and the 4th slot's neighbour.
- Return value is 1 only when every factory call returned non-null.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C067C0` | `FUN_80c067c0` | `fn_80c067c0` | release wrapper | translated | high | `80c067c0_FUN_80c067c0.c` |
| `0x80C06B80` | `FUN_80c06b80` | `fn_80c06b80` | 0x1138 constructor | translated | medium | `80c06b80_FUN_80c06b80.c` |
| `0x80C06C90` | `FUN_80c06c90` | `fn_80c06c90` | 0x1138 destructor | translated | high | `80c06c90_FUN_80c06c90.c` |
| `0x80C06EB0` | `FUN_80c06eb0` | `fn_80c06eb0` | three small effects | translated | high | `80c06eb0_FUN_80c06eb0.c` |
| `0x80C077C0` | `FUN_80c077c0` | `fn_80c077c0` | heading step and overshoot test | translated | high | `80c077c0_FUN_80c077c0.c` |
| `0x80C07990` | `FUN_80c07990` | `fn_80c07990` | `throw_ed_wait` state entry | translated | high | `80c07990_FUN_80c07990.c` |
| `0x80C07A80` | `FUN_80c07a80` | `fn_80c07a80` | gravity/floor handler | translated | high | `80c07a80_FUN_80c07a80.c` |
| `0x80C07B90` | `FUN_80c07b90` | `fn_80c07b90` | `jump_st` state entry | translated | high | `80c07b90_FUN_80c07b90.c` |
| `0x80C081B0` | `FUN_80c081b0` | `fn_80c081b0` | jump/land heading handler | translated | medium | `80c081b0_FUN_80c081b0.c` |
| `0x80C0A0D0` | `FUN_80c0a0d0` | `fn_80c0a0d0` | 0x12E8 constructor | translated | medium | `80c0a0d0_FUN_80c0a0d0.c` |
| `0x80C0A250` | `FUN_80c0a250` | `fn_80c0a250` | 0x128-byte element ctor | translated | medium | `80c0a250_FUN_80c0a250.c` |
| `0x80C0A350` | `FUN_80c0a350` | `fn_80c0a350` | 0x10-byte element dtor | translated | high | `80c0a350_FUN_80c0a350.c` |
| `0x80C0A6F0` | `FUN_80c0a6f0` | `fn_80c0a6f0` | three rig effects | translated | high | `80c0a6f0_FUN_80c0a6f0.c` |
| `0x80C0A890` | `FUN_80c0a890` | `fn_80c0a890` | effect `0x187` | translated | high | `80c0a890_FUN_80c0a890.c` |
| `0x80C0AC50` | `FUN_80c0ac50` | `fn_80c0ac50` | collision query | translated | medium | `80c0ac50_FUN_80c0ac50.c` |
| `0x80C0B2C0` | `FUN_80c0b2c0` | `fn_80c0b2c0` | duration selection | translated | medium | `80c0b2c0_FUN_80c0b2c0.c` |
| `0x80C0B440` | `FUN_80c0b440` | `fn_80c0b440` | nearest rig by x | translated | high | `80c0b440_FUN_80c0b440.c` |
| `0x80C0B520` | `FUN_80c0b520` | `fn_80c0b520` | `Wm_ob_witchcraftup` | translated | high | `80c0b520_FUN_80c0b520.c` |
| `0x80C0B810` | `FUN_80c0b810` | `fn_80c0b810` | `jump_HB2` launch | translated | high | `80c0b810_FUN_80c0b810.c` |
| `0x80C0B900` | `FUN_80c0b900` | `fn_80c0b900` | descent handler | translated | high | `80c0b900_FUN_80c0b900.c` |
| `0x80C0B9E0` | `FUN_80c0b9e0` | `fn_80c0b9e0` | clip `0x4C8` entry | translated | medium | `80c0b9e0_FUN_80c0b9e0.c` |
| `0x80C0BEA0` | `FUN_80c0bea0` | `fn_80c0bea0` | speed deceleration | translated | high | `80c0bea0_FUN_80c0bea0.c` |
| `0x80C0C9F0` | `FUN_80c0c9f0` | `fn_80c0c9f0` | contact reset | translated | high | `80c0c9f0_FUN_80c0c9f0.c` |
| `0x80C0E380` | `FUN_80c0e380` | `fn_80c0e380` | tail/phase initializer | translated | high | `80c0e380_FUN_80c0e380.c` |
| `0x80C0E4C0` | `FUN_80c0e4c0` | `fn_80c0e4c0` | six-phase chain builder | translated | high | `80c0e4c0_FUN_80c0e4c0.c` |

## Unknowns

- **Owner.** No export string in this band names the boss. The functions share
  the six phase objects at `+0xEA0..+0xEB4`, the three rig objects at
  `+0x1140..+0x1148` and the phase-effect object at `+0x11BC` with the wave-4
  runners `0x80C0CF60`/`0x80C10610`, so the docs call the family "Castle
  Iggy" but do not prove the attribution.
- **Dropped receivers.** `0x8016D4B0` in `0x80C06C90` and `0x802D7AA0` in
  `0x80C0A250` are printed without arguments. The first is translated as a
  no-argument call (matching the wave-5 destructor); the second passes
  `this`, because every sibling constructor does so. Both rows are `medium`.
- **`0x80C0A330`.** It is the element constructor passed to `0x802DC9A0` by
  `0x80C0A0D0`, but it has no separate row in `functions.tsv`. Its address is
  kept raw in the Rust file.
- **Untranslated callees.** `0x80C38900` (`0x80C081B0`) and `0x80C3F840`
  (`0x80C0B9E0`) have no translated body in this lane; the call sites keep the
  raw address.
- **Paired-single noise.** `0x80C0E4C0` has a GQR/`ldexpf` preamble in the
  export. It is compiler bookkeeping, documented and not translated; the
  gameplay body after it is complete. The same applies to the two dead
  `longlong` locals at the top of `0x80C0AC50`.
- **Virtual prototypes.** The `+0x5F8` slots `+0xC/+0x10` in `0x80C0B2C0`,
  the `+0xD4` transitions, and the `+0xA0/+0x74/+0x2C` effect slots in
  `0x80C0B520` are named by offset only; the printed argument shapes are
  preserved, including the `f64` first argument of `+0x2C`.
- **Opaque constants.** `0x80C5738C/90/98/9C/A0/B0/B8/BC/C8`, `0x80C57474/
  78/80/84/88/8C/90/9C/A4/A8/AC/B0/C0`, `0x80C57650/54/58/5C/60`,
  `0x80C5AD20/6C/80`, `0x80C5B83C/54/58/60/64/68/70/74/78/7C/8A8/8CC/8D0/
  8E0`, `0x80C5B88C/8E`, `0x80C5B944`, `0x80C5B964`, `0x80C5C01C/C028`,
  `0x80C5C650`, `0x80C663E0/6A50/76C8/7D78/8FE8`, `0x8042C1A0` and
  `0x8042C1A4` are read from their original addresses; their numeric values
  are not claimed.

## Verification

- `cargo check` passes (exit 0) with no error or warning in
  `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave10-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 18 `high`, 7 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-10 ledger
  were edited.
