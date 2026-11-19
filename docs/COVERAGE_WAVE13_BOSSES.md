# Coverage - Wave 13, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 25 functions added to `src/game/bosses.rs` by the wave-13
  pass over the boss-actor REL `d_en_bossNP` (REL module 4). Earlier passes are
  documented in `docs/COVERAGE_REL_d_en_bossNP.md` (23 functions) and the
  wave-3/4/5/6/10/12 boss documents (20 + 24 + 25 + 25 + 25 + 24). This file
  does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value here is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x8006xxxx`, `0x800Axxxx`, `0x8010xxxx`,
  `0x8015xxxx`, `0x8016xxxx`, `0x801Cxxxx`, `0x8023xxxx`, `0x802Bxxxx`,
  `0x802Dxxxx`) and the absolute DOL words `0x80354E50..0x80354E5C`,
  `0x80310ED8`, `0x80429D40`, `0x80429F80` and `0x8042A028` are unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
  Sizes are from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing; `stub` is not used here.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes, stores and calls are exact. `medium` means the stores and branches are
  exact but a dropped receiver or a missing argument is reconstructed by
  convention.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table, so no Nintendo
  symbol is invented. Strings appear only where the export points at the exact
  data address.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any medium row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 25 |
| text bytes added | 4,324 |
| status `translated` | 25 |
| status `partial` | 0 |
| status `stub` | 0 |
| confidence `high` | 18 |
| confidence `medium` | 7 |
| confidence `low` | 0 |
| cumulative functions in `game::bosses` | 191 |
| cumulative bytes in `game::bosses` | 74,580 |

The pass adds the self-contained `0x1238`-byte actor library that sits between
the wave-12 `0xFD0` handlers and the later Castle registrars: its
factory and constructor, its release and chain utilities, a collision box test,
a timer clamp and two per-frame handlers. It then adds three base phase-object
constructors, the `0x80C4ACA0` controller-step handler and five `+0x2E4`
flash/input helpers.

## Synthetic-base warning in one place

| item | value |
|---|---:|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in `src/game/bosses.rs` is an
analysis address. A live dump will not match it. The constants read at
`0x80C57Axx`, `0x80C57Bxx`, `0x80C58Fxx`, `0x80C591xx`, `0x80C600xx` and
`0x80C601xx` are analysis-image data words, not hardware addresses.

## The `0x1238` actor layout

`// inferred:` from the field accesses shared by the sixteen functions in
cluster 1. Every offset below is exact and comes from the export.

| offset | meaning |
|---|---|
| `+0x060` | class vtable pointer (a field, not the first word) |
| `+0x0AC/+0x0B0/+0x0B4` | position x/y/z (`f32`) |
| `+0x120` | 32-bit flag word; bit 3 (`0x8`) is cleared by `0x80C1DBC0` |
| `+0x148` | object passed to `0x8008C380` at phase 15 |
| `+0x1EC` | object passed to the ground probe `0x80072BC0` |
| `+0x394` | sub-object with its own vtable (slot `+0x10`) |
| `+0x544` | animation controller (clip slot `+0x24`, speed slot `+0x30`) |
| `+0x5F0` | signed halfword copied into the controller call |
| `+0x5F8` | query object (vtable slots `+0x0C/+0x10/+0x14`) |
| `+0x600` | model context |
| `+0x604` | model object (vtable slot `+0x1C`) |
| `+0x644` | animation object |
| `+0x67C` | model-switch object (`0x80166110(model, 0/1)`) |
| `+0x6A4` | table of two model-switch triggers (`+0x04`, `+0x3C`) |
| `+0x6AC` | secondary animation |
| `+0x760` | effect table (`+0x0C` gates the demo clip) |
| `+0x1080` | signed retry counter |
| `+0x1084` | phase counter |
| `+0x1114` | effect controller (vtable slot `+0xB0`) |
| `+0x1228/+0x122C` | box origin x/y, seeded from `+0xAC/+0xB0` |
| `+0x1230` | fourth constructor word, copied from `0x80C57BEC` |

## Cluster 1 - the `0x1238` actor library (2,356 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C1A330` | 152 | `fn_80c1a330` | demo clip bind and controller clip `0x567` | translated | high |
| `0x80C1B260` | 52 | `fn_80c1b260` | half of class slot `+0x4A4` | translated | high |
| `0x80C1C280` | 92 | `fn_80c1c280` | `0x8015F800` teardown + optional release | translated | high |
| `0x80C1C490` | 224 | `fn_80c1c490` | recursive `+0x30` chain resolver | translated | medium |
| `0x80C1C570` | 136 | `fn_80c1c570` | `:`-suffix name compare | translated | medium |
| `0x80C1C690` | 48 | `fn_80c1c690` | `0x1238` factory | translated | medium |
| `0x80C1C740` | 184 | `fn_80c1c740` | `0x1238` constructor | translated | high |
| `0x80C1C840` | 88 | `fn_80c1c840` | `0x80168D80` teardown + optional release | translated | high |
| `0x80C1CC80` | 76 | `fn_80c1cc80` | controller step and phase increment | translated | high |
| `0x80C1D0B0` | 76 | `fn_80c1d0b0` | class slot `+0x11C` on a selected target | translated | medium |
| `0x80C1D130` | 148 | `fn_80c1d130` | `80`/`40` selection from two queries | translated | high |
| `0x80C1D2D0` | 120 | `fn_80c1d2d0` | zero-margin box test | translated | high |
| `0x80C1D6F0` | 84 | `fn_80c1d6f0` | low bound plus bounded random | translated | medium |
| `0x80C1D780` | 156 | `fn_80c1d780` | clip `0x4DB`/`0x4DC` selection | translated | high |
| `0x80C1DBC0` | 340 | `fn_80c1dbc0` | per-frame timer, phase and clip-end handler | translated | high |
| `0x80C1E420` | 380 | `fn_80c1e420` | per-frame probe and transition handler | translated | medium |

What is proven:

- `0x80C1C740` installs `0x80C601CC` at `+0x60`, clears `+0xE74`, stores
  `0x80C60810` at `+0xE78`, points `+0xE7C` at the actor, initializes
  `+0x1090`/`+0x10D0`, overwrites `+0x10D0` with `0x80C60960`, registers the
  `0x114`-byte effect controller at `+0x1114` with `0x80C04C80`/`0x80C04CC0`,
  and seeds `+0x1228/+0x122C/+0x1230`.
- `0x80C1C690` allocates exactly `0x1238` bytes with `0x801628C0` and calls the
  constructor only on success.
- `0x80C1C280` and `0x80C1C840` are the matching release wrappers: base teardown
  `0x8015F800`/`0x80168D80` with argument `0`, then `0x802B9280` only when the
  caller asks for it, returning the pointer unchanged.
- `0x80C1C490` is five unrolled virtual checks on the `+0x30` child followed by
  a recursive call on the fifth child. `0x80C1C570` repeats the `:`-split idiom
  already documented for `0x80C0E230` and `0x80C04A10`.
- `0x80C1D2D0` compares a two-float point against `+0x1228/+0x122C` plus the
  four margins at `0x80C60104/08/0C/10`; all four margins are zero in this
  image, so the test is equality with the origin.
- `0x80C1DBC0` decays `+0xE74` by 7 with a floor at 0, clears flag bit 3 at
  phase 15, drives the two `+0x6A4` triggers at phase 0, forwards the position
  to the `+0x1114` controller, and calls class slot `+0xD4` with the BSS
  descriptor `0x80C6E6C4` when the animation finished.
- `0x80C1E420` raises `+0xE74` by 7 with a ceiling at 255, then gates the three
  list probes on the signed retry counter `+0x1080`: success reseeds the counter
  and calls class slots `+0x4B0`/`+0xD4` with `0x80C6E744`; failure decrements
  the counter and sets `0x80C6E684`.
- `0x80C1A330` mirrors the wave-12 clip-bind ritual (`0x8023A0B0` →
  `0x801651F0` → `0x80169F80` → `0x801680E0`) and then starts controller clip
  `0x567` unconditionally.
- `0x80C1D780` selects `0x4DB` from slot `+0x10` and `0x4DC` from slot `+0x14`;
  `0x80C1D130` selects the halfword `0x50` (80) or `0x28` (40).

## Cluster 2 - base phase-object constructors and the `0x80C4ACA0` handler (520 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C37CB0` | 164 | `fn_80c37cb0` | `0xFF0` base constructor | translated | high |
| `0x80C46840` | 156 | `fn_80c46840` | `0x10E8` base constructor | translated | high |
| `0x80C4A870` | 124 | `fn_80c4a870` | `0xF98` base constructor | translated | high |
| `0x80C4ACA0` | 76 | `fn_80c4aca0` | controller step and clear | translated | high |

What is proven:

- All three constructors start with `0x800A88A0`, install a class vtable at
  `+0x60`, and return the allocation (or `0`).
- `0xFF0`: vtable `0x80C663E0`, `+0xE74 = 0`, secondary vtable `0x80C66A50` at
  `+0xEBC`, self-pointer at `+0xEC0`, controller `+0xEC4` through `0x802D7AA0`
  then overwritten with `0x80310ED8`, and six clears at `+0xFD8..+0xFE8`. The
  same base vtable is installed first by the translated wave-10 `0x1138`
  constructor `0x80C06B80`.
- `0x10E8`: vtable `0x80C69A00`, profile `0x80C6A058` at `+0xE74`,
  self-pointer at `+0xE78`, and two `0x802DC9A0` registrations (`+0xEA8` with
  `0x114` and the shared callbacks, `+0xFBC` with `0x128` and
  `0x80C0A250`/`0x80C005B0`). The untranslated `0x80C11C70` constructor installs
  the same base vtable before switching to `0x80C5D3B8`.
- `0xF98`: vtable `0x80C6AC34`, profile `0x80C5E988` at `+0xE74`, self-pointer
  at `+0xE78`, controller `+0xE7C` through `0x802D7AA0` then overwritten with
  `0x80C5A938`. The wave-12 `0xFD0` constructor `0x80C15C20` installs the same
  base vtable before switching to `0x80C5E360`.
- `0x80C4ACA0` steps the `+0x394` controller, calls `0x80064F50(actor, 0)` and
  returns `1`.

## Cluster 3 - `+0x2E4` flash/input helpers (1,448 bytes)

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C4D570` | 344 | `fn_80c4d570` | `demo_escapeA2` clip and velocity clear | translated | high |
| `0x80C4FDF0` | 288 | `fn_80c4fdf0` | mark selected effect slots visible | translated | high |
| `0x80C504E0` | 388 | `fn_80c504e0` | controller-count phase selection | translated | high |
| `0x80C509B0` | 232 | `fn_80c509b0` | frame velocity and travel length | translated | medium |
| `0x80C50B10` | 196 | `fn_80c50b10` | two-slot destructor | translated | high |

What is proven:

- `0x80C4D570` binds the string at `0x80C6AC0C` (`demo_escapeA2`) to both the
  primary animation `+0x644` and the secondary animation `+0x6AC`, clears
  `+0xE8/+0xEC/+0xF0`, sends 1.0 to class slot `+0x550`, starts controller clip
  `0x558` with the halfword at `+0x5F0`, sets `+0x3D4 = 1` on the global at
  `0x80429D40 + 0x18` when present, calls class slot `+0x59C`, writes the
  vertical speed `0x80C58FF8` (-160.0) to `+0xB4` and sets `+0x360 = 1`.
- `0x80C4FDF0` loops twice over the flag pairs at `0x80354E50..0x80354E5C`,
  reads the counts at `+0x2F4/+0x2F8` (plus `8*i`) and walks the corresponding
  ten-word slice of `0x80C591F8` backwards, setting bit 0 of each slot object's
  `+0xBB` byte.
- `0x80C504E0` rejects the frame when either top bit of `0x80429F80` is set.
  Otherwise it reads up to four controller slot counts from
  `0x8042A028 + 4 + 4*i` at `+0x50`, returns early on a negative value, stores
  each non-negative value and marks "ready" when any value is below two. Ready
  runs `0x800C92F0`, `0x80C4FDF0`, `0x800C9560`, class slot `+0x10` and phase 4;
  not ready runs `0x80C4FDE0` and phase 7.
- `0x80C509B0` snapshots `+0x0C/+0x10/+0x14`, calls `0x80160AE0` with three
  floats from `+0x2C/+0x30/+0x34`, stores the three deltas at
  `+0x18/+0x1C/+0x20`, measures the base-relative travel vector with
  `0x801C14C0`, and clears the valid byte `+0x28` when the length is at most
  `+0x24 * +0x38`.
- `0x80C50B10` writes the vtable `0x80C6C474`, releases the two `+0x0C` words
  with `0x80106200(object, 1)`, destroys the two `+0x14` objects through their
  vtable slot `+0x08`, and optionally releases the object.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C1A330` | `FUN_80c1a330` | `fn_80c1a330` | demo clip bind | translated | high | `80c1a330_FUN_80c1a330.c` |
| `0x80C1B260` | `FUN_80c1b260` | `fn_80c1b260` | half of slot `+0x4A4` | translated | high | `80c1b260_FUN_80c1b260.c` |
| `0x80C1C280` | `FUN_80c1c280` | `fn_80c1c280` | teardown wrapper | translated | high | `80c1c280_FUN_80c1c280.c` |
| `0x80C1C490` | `FUN_80c1c490` | `fn_80c1c490` | chain resolver | translated | medium | `80c1c490_FUN_80c1c490.c` |
| `0x80C1C570` | `FUN_80c1c570` | `fn_80c1c570` | suffix name compare | translated | medium | `80c1c570_FUN_80c1c570.c` |
| `0x80C1C690` | `FUN_80c1c690` | `fn_80c1c690` | `0x1238` factory | translated | medium | `80c1c690_FUN_80c1c690.c` |
| `0x80C1C740` | `FUN_80c1c740` | `fn_80c1c740` | `0x1238` constructor | translated | high | `80c1c740_FUN_80c1c740.c` |
| `0x80C1C840` | `FUN_80c1c840` | `fn_80c1c840` | teardown wrapper | translated | high | `80c1c840_FUN_80c1c840.c` |
| `0x80C1CC80` | `FUN_80c1cc80` | `fn_80c1cc80` | controller step | translated | high | `80c1cc80_FUN_80c1cc80.c` |
| `0x80C1D0B0` | `FUN_80c1d0b0` | `fn_80c1d0b0` | class slot `+0x11C` | translated | medium | `80c1d0b0_FUN_80c1d0b0.c` |
| `0x80C1D130` | `FUN_80c1d130` | `fn_80c1d130` | `80`/`40` selection | translated | high | `80c1d130_FUN_80c1d130.c` |
| `0x80C1D2D0` | `FUN_80c1d2d0` | `fn_80c1d2d0` | box test | translated | high | `80c1d2d0_FUN_80c1d2d0.c` |
| `0x80C1D6F0` | `FUN_80c1d6f0` | `fn_80c1d6f0` | bounded random | translated | medium | `80c1d6f0_FUN_80c1d6f0.c` |
| `0x80C1D780` | `FUN_80c1d780` | `fn_80c1d780` | clip selection | translated | high | `80c1d780_FUN_80c1d780.c` |
| `0x80C1DBC0` | `FUN_80c1dbc0` | `fn_80c1dbc0` | per-frame handler | translated | high | `80c1dbc0_FUN_80c1dbc0.c` |
| `0x80C1E420` | `FUN_80c1e420` | `fn_80c1e420` | per-frame probe handler | translated | medium | `80c1e420_FUN_80c1e420.c` |
| `0x80C37CB0` | `FUN_80c37cb0` | `fn_80c37cb0` | `0xFF0` base constructor | translated | high | `80c37cb0_FUN_80c37cb0.c` |
| `0x80C46840` | `FUN_80c46840` | `fn_80c46840` | `0x10E8` base constructor | translated | high | `80c46840_FUN_80c46840.c` |
| `0x80C4A870` | `FUN_80c4a870` | `fn_80c4a870` | `0xF98` base constructor | translated | high | `80c4a870_FUN_80c4a870.c` |
| `0x80C4ACA0` | `FUN_80c4aca0` | `fn_80c4aca0` | controller step and clear | translated | high | `80c4aca0_FUN_80c4aca0.c` |
| `0x80C4D570` | `FUN_80c4d570` | `fn_80c4d570` | `demo_escapeA2` entry | translated | high | `80c4d570_FUN_80c4d570.c` |
| `0x80C4FDF0` | `FUN_80c4fdf0` | `fn_80c4fdf0` | slot visibility | translated | high | `80c4fdf0_FUN_80c4fdf0.c` |
| `0x80C504E0` | `FUN_80c504e0` | `fn_80c504e0` | phase selection | translated | high | `80c504e0_FUN_80c504e0.c` |
| `0x80C509B0` | `FUN_80c509b0` | `fn_80c509b0` | velocity/travel helper | translated | medium | `80c509b0_FUN_80c509b0.c` |
| `0x80C50B10` | `FUN_80c50b10` | `fn_80c50b10` | two-slot destructor | translated | high | `80c50b10_FUN_80c50b10.c` |

## Unknowns

- **Owner.** No export string in this band names the boss. The `demo_escapeA2`
  string, the `+0x2E4` phase counters and the `0x1238` actor's `0x1114` effect
  controller suggest the same water/escape family as wave 12, but the documents
  do not assign a name. The three constructors are therefore named by their
  allocation size, not by a boss.
- **Dropped receivers.** `0x80C1C490` (slot `+0x0C`), `0x80C1C570` (slot
  `+0x20`), `0x80C1CC80`/`0x80C4ACA0` (slot `+0x10`), `0x80C1D0B0` (slot
  `+0x11C`), `0x80C1D130`/`0x80C1D780` (slots `+0x0C/+0x10/+0x14`),
  `0x80C1B260` (slot `+0x4A4`), `0x80C1DBC0`/`0x80C1E420` (slots `+0xD4` and
  `+0x4B0`) and `0x80C4D570` (slots `+0x550`/`+0x59C`) are printed without
  arguments. Each receiver is reconstructed by the sibling convention; those
  rows are `medium` where a second reconstruction is also needed.
- **Dropped arguments.** `0x80C1C690` calls the constructor with no printed
  argument; `0x80C1D0B0` calls `0x80162D00` with no argument; `0x80C1D6F0` is
  declared with no parameters but called with the actor by `0x80C1E420`. Each
  is documented at the call site.
- **Untranslated callees.** `0x80C1D350` and `0x80C1D430` (the two list probes
  used by `0x80C1E420`), `0x80C4FDE0` (the not-ready phase helper) and the DOL
  helpers `0x80160AE0`, `0x800B2ED0` and `0x80064F50` are kept as raw
  addresses. Their roles are inferred from the argument shapes.
- **Opaque constants.** `0x80C57A18/1C`, `0x80C57BEC`, `0x80C58F74/78`,
  `0x80C58FF8`, `0x80C591F8`, `0x80C600CA/CC`, `0x80C600F0/F4/F8/FC`,
  `0x80C60104/08/0C/10/18/1C/30/34`, the vtables `0x80C5A938`, `0x80C5E988`,
  `0x80C601CC`, `0x80C60810`, `0x80C60960`, `0x80C663E0`, `0x80C66A50`,
  `0x80C69A00`, `0x80C6A058`, `0x80C6AC34`, `0x80C6C474` and the BSS
  descriptors `0x80C6E684/6C4/744` are read or written at their original
  addresses. Their numeric values are not claimed; the string at `0x80C6AC0C`
  is the one exception (`demo_escapeA2`).
- **`0x80C1D2D0` margins.** The four margin words are zero in this analysis
  image, so the export's `min <= point <= max` reduces to equality with the
  origin. They are still loaded from memory in the translation.

## Verification

- `cargo check` passes (exit 0); `src/game/bosses.rs` produces no warning.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- All 25 evidence files were read in full; every `# Original` block names the
  file and the size from `functions.tsv`.
- The 25 rows in `ledger/agents/wave13-bosses.csv` match the Rust functions one
  to one: 25 `translated`; 18 `high`, 7 `medium`.
- No files outside `src/game/bosses.rs`, this document and the wave-13 ledger
  were edited.
