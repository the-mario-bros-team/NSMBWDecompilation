# Coverage - Wave 3 second pass, `d_en_bossNP` (`game::bosses`)

## How to read this file

- **Scope** - the 20 functions added to `src/game/bosses.rs` by the wave-3
  second pass over the boss-actor REL `d_en_bossNP` (REL module 4). The
  first-pass 23 functions are documented in
  `docs/COVERAGE_REL_d_en_bossNP.md`; this file does not repeat them.
- **Addresses are synthetic.** The REL-to-ELF adapter places the module at
  `0x80C00000`, with entry `0x80C000F0`. The retail loader picks a different
  base, so every `0x80C...` value here is an analysis address, not a runtime
  address. Calls into `main.dol` (`0x80xxxxxx`) are absolute and unaffected.
- **Evidence** is the Ghidra C under
  `work/ghidra/rel-d_en_bossNP/export/`; the exact file is named in every row.
  Sizes are from `functions.tsv`.
- **Status** follows `docs/RUST_STYLE.md`: `translated` means the recorded
  control flow and observable stores are represented; `partial` means one
  documented piece is missing (see **Unknowns**); `stub` is not used here.
- **Confidence** rates the translation, not the guessed role. `high` means the
  bytes and stores are exact and the role is already established by a registrar
  or an identical sibling. `medium` means the stores are exact but object roles,
  virtual-call prototypes or constant widths are inferred. `low` means a call
  convention is unresolved.
- **Names** stay `fn_80xxxxxx`. The REL has no symbol table; no Nintendo symbol
  is invented. Boss/state names appear only where an existing registrar or an
  export string proves them.
- Start with the **summary**, then the **clusters**, then the **function
  table**. Read the **unknowns** before treating any partial row as complete.

## Summary

| metric | value |
|---|---:|
| functions added in this pass | 20 |
| text bytes added | 11,728 |
| status `translated` | 17 |
| status `partial` | 3 |
| status `stub` | 0 |
| confidence `high` | 11 |
| confidence `medium` | 8 |
| confidence `low` | 1 |
| cumulative functions in `game::bosses` | 43 |
| cumulative bytes in `game::bosses` | 39,496 |
| cumulative share of 770 functions | 5.58% |
| cumulative share of 356,396 text bytes | 11.08% |

The pass was chosen for dependency value, not address coverage: six large
action/AI bodies that were deferred in the first pass, the nine state-object
teardown runners referenced by the registrars, and five small helpers that
close existing call sites.

## Synthetic-base warning in one place

| item | value |
|---|---|
| canonical base | `0x80C00000` |
| entry | `0x80C000F0` |
| BSS start | `0x80C6C900` |
| BSS size | `0x487C` |

Every `0x80C...` address in this document and in the Rust file is an analysis
address. A live dump will not match it. The motion constants read at
`0x80C57CE8`, `0x80C57F74`, `0x80C580B8`, `0x80C581D8` and neighbours are
analysis-image data words, not hardware addresses.

## Cluster 1 - controller and destructor helpers

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C00200` | 64 | `fn_80c00200` | array release callback for `0x802DC9A0`/`0x802DCA98` | partial | medium |
| `0x80C003C0` | 88 | `fn_80c003c0` | actor destructor variant (same body as `0x80C04C10`) | translated | high |
| `0x80C04C80` | 60 | `fn_80c04c80` | controller constructor; installs vtable `0x80C5A938` | translated | medium |
| `0x80C04CC0` | 92 | `fn_80c04cc0` | controller destructor via `0x802D7B20` | translated | high |

`0x80C04C80` and `0x80C04CC0` are installed as the constructor/destructor pair
of the embedded controller in `0x80C04B30` (already translated). `0x80C00200`
is the release callback for the ten-element controller arrays used by
`0x80C22CA0`; its export shows a zero-argument release call, so the object
argument is not modelled and the row is marked partial.

## Cluster 2 - recursive chain resolver

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C04930` | 224 | `fn_80c04930` | follow `+0x30` links while vtable slot `+0x0C` returns zero, then recurse | partial | low |

The unrolled five-link shape, the `== 0` tests and the final recursion are
exact. The export prints each virtual call as `(**(code **)(... + 0xC))()` with
no arguments; whether the receiver travels in `r3` is not proven. The function
is marked partial rather than guessing a receiver.

## Cluster 3 - sibling state-object teardown runners

| address | size | Rust | registrar / states | status | confidence |
|---|---:|---|---|---|---|
| `0x80C04730` | 92 | `fn_80c04730` | Iggy action states (12) | translated | high |
| `0x80C11830` | 88 | `fn_80c11830` | Castle Morton demo states (5) | translated | high |
| `0x80C11890` | 92 | `fn_80c11890` | Castle Morton action states (10) | translated | high |
| `0x80C15820` | 92 | `fn_80c15820` | Castle Roy states (20) | translated | high |
| `0x80C2A050` | 92 | `fn_80c2A050` | Koopa demo cage states (3) | translated | high |
| `0x80C37940` | 92 | `fn_80c37940` | Larry states (9) | translated | high |
| `0x80C464B0` | 92 | `fn_80c464b0` | Morton states (14) | translated | high |
| `0x80C4A4B0` | 92 | `fn_80c4A4B0` | Roy states (17) | translated | high |
| `0x80C4EFA0` | 92 | `fn_80c4EFA0` | Wendy states (17) | translated | high |

All nine have the identical body already modelled by `fn_80c046d0`: clear the
state base through `0x8015F800`, then release through `0x802B9280` when the
second argument is positive. They are separate functions in the REL, so they
keep separate Rust entries and evidence blocks. The registrars prove which
state family installs each one.

## Cluster 4 - action runners with a `+0x360` state counter

| address | size | Rust | states | role | status | confidence |
|---|---:|---|---:|---|---|---|
| `0x80C2F0C0` | 1,352 | `fn_80c2F0C0` | 6 | jump start/loop/attack, landing, damage, end | translated | medium |
| `0x80C30D10` | 1,404 | `fn_80c30D10` | 6 | same shape, second constant bank; toggles meshes 5/8 | translated | medium |
| `0x80C331C0` | 1,324 | `fn_80c331C0` | 7 | jump arc against `+0xC84/+0xC88`, apron flags | translated | medium |
| `0x80C32800` | 2,492 | `fn_80c32800` | 12 | awake/demo flight, jump arc, airship demos, damage, wait | translated | medium |

All four dispatch on the actor state word at `+0x360`. The module doc has the
shared actor layout. The common ritual is: per-frame model update through
vtable `+0x1C`, animation-end test through `0x80169020`, clip lookup through
`0x8023A0B0`, binding through `0x801651F0`, blend arguments through
`0x80169F80`/`0x801680E0`, then a state increment. States 3/4 and 9/10 also
integrate a vertical arc by hand and derive the horizontal speed written to
`+0xE8`.

What is proven:

- every state number, clip pointer, field offset, table address and branch is
  taken directly from the C export;
- the `+0x360` increments happen at the same transition in every state;
- the two constant banks are kept separate (`0x80C57Fxx`/`0x80C580xx` versus
  `0x80C581xx`) instead of being folded into one table;
- the `0x80C581FC` value is both the per-frame integration step and the
  animation-rate argument, exactly as the export shows.

## Cluster 5 - spawn and gameplay setup

| address | size | Rust | role | status | confidence |
|---|---:|---|---|---|---|
| `0x80C22CA0` | 2,520 | `fn_80c22CA0` | spawn/trail/collision manager over the `0x80C613D8` table | partial | medium |
| `0x80C4FF10` | 1,284 | `fn_80c4FF10` | snapshot/restore of four actor and four wing slots, scene-graph setup | translated | medium |

`0x80C22CA0` is the largest function added in this pass. It constructs the
ten-entry spawn array through `0x802DC9A0`, sweeps collisions through
`0x8065C9A0`, applies nearby-actor callbacks for kinds `0x113`, `0x105`,
`0x1C9` and `0x1B0`, and advances four `0x140`-byte trail records. Its paired-
single/GQR preamble is Ghidra noise and is documented, not translated.

`0x80C4FF10` snapshots the four `+0x294` actor slots and four `+0x2C8` wing
slots, clears their `+0xBB` bit 0, and either restores a selected subset (mode
flag `0x80315B98 != 1`) or configures the two `+0x1F8`/`+0x1FC` objects plus
the `+0x288`/`+0x28C` pair (mode `1`). The two 10-pointer groups are cleared
afterwards.

## Function table

| address | Ghidra name | Rust function | role | status | confidence | evidence |
|---|---|---|---|---|---|---|
| `0x80C00200` | `FUN_80c00200` | `fn_80c00200` | array release callback | partial | medium | `80c00200_FUN_80c00200.c` |
| `0x80C003C0` | `FUN_80c003c0` | `fn_80c003c0` | actor destructor variant | translated | high | `80c003c0_FUN_80c003c0.c` |
| `0x80C04730` | `FUN_80c04730` | `fn_80c04730` | Iggy teardown runner | translated | high | `80c04730_FUN_80c04730.c` |
| `0x80C04930` | `FUN_80c04930` | `fn_80c04930` | recursive chain resolver | partial | low | `80c04930_FUN_80c04930.c` |
| `0x80C04C80` | `FUN_80c04c80` | `fn_80c04c80` | controller constructor | translated | medium | `80c04c80_FUN_80c04c80.c` |
| `0x80C04CC0` | `FUN_80c04cc0` | `fn_80c04cc0` | controller destructor | translated | high | `80c04cc0_FUN_80c04cc0.c` |
| `0x80C11830` | `FUN_80c11830` | `fn_80c11830` | Castle Morton demo runner | translated | high | `80c11830_FUN_80c11830.c` |
| `0x80C11890` | `FUN_80c11890` | `fn_80c11890` | Castle Morton action runner | translated | high | `80c11890_FUN_80c11890.c` |
| `0x80C15820` | `FUN_80c15820` | `fn_80c15820` | Castle Roy runner | translated | high | `80c15820_FUN_80c15820.c` |
| `0x80C22CA0` | `FUN_80c22ca0` | `fn_80c22ca0` | spawn/trail/collision manager | partial | medium | `80c22ca0_FUN_80c22ca0.c` |
| `0x80C2A050` | `FUN_80c2a050` | `fn_80c2a050` | Koopa demo-cage runner | translated | high | `80c2a050_FUN_80c2a050.c` |
| `0x80C2F0C0` | `FUN_80c2f0c0` | `fn_80c2f0c0` | six-state jump runner | translated | medium | `80c2f0c0_FUN_80c2f0c0.c` |
| `0x80C30D10` | `FUN_80c30d10` | `fn_80c30d10` | six-state jump runner, bank 2 | translated | medium | `80c30d10_FUN_80c30d10.c` |
| `0x80C32800` | `FUN_80c32800` | `fn_80c32800` | twelve-state demo/jump runner | translated | medium | `80c32800_FUN_80c32800.c` |
| `0x80C331C0` | `FUN_80c331c0` | `fn_80c331c0` | seven-state jump/apron runner | translated | medium | `80c331c0_FUN_80c331c0.c` |
| `0x80C37940` | `FUN_80c37940` | `fn_80c37940` | Larry runner | translated | high | `80c37940_FUN_80c37940.c` |
| `0x80C464B0` | `FUN_80c464b0` | `fn_80c464b0` | Morton runner | translated | high | `80c464b0_FUN_80c464b0.c` |
| `0x80C4A4B0` | `FUN_80c4a4b0` | `fn_80c4a4b0` | Roy runner | translated | high | `80c4a4b0_FUN_80c4a4b0.c` |
| `0x80C4EFA0` | `FUN_80c4efa0` | `fn_80c4efa0` | Wendy runner | translated | high | `80c4efa0_FUN_80c4efa0.c` |
| `0x80C4FF10` | `FUN_80c4ff10` | `fn_80c4ff10` | slot snapshot/restore and scene setup | translated | medium | `80c4ff10_FUN_80c4ff10.c` |

## Unknowns

- **`0x80C00200` release argument.** The export shows
  `func_0x802b9280();` with no argument, while the eight sibling teardown
  runners pass the object. The translation keeps the literal no-argument call
  and marks the row partial.
- **`0x80C04930` virtual-call receiver.** All five probes are printed as
  no-argument calls. Only the slot address (`vtable + 0x0C`), the `== 0` test
  and the link walk are proven.
- **`0x80C22CA0` paired-single preamble.** The first ~90 C lines decode the
  PowerPC GQR save/restore sequence and `ldexpf` scaling of `in_f27..in_f31`.
  They are compiler bookkeeping, not game state, and are not translated. The
  gameplay body after them is complete.
- **Virtual-call prototypes.** `0x80C22CA0` calls slots `+0x28`, `+0x14`,
  `+0x18`, `+0xA0`, `+0x3D4`, `+0x3F4` and `+0x3F8`; the argument counts come
  from the export, but the formal types and object roles are inferred. The
  action runners call `+0x3F0`, `+0x3F4`, `+0x3F8`, `+0xD4`, `+0x24`, `+0x1C`
  and `+0x40C`.
- **Motion constant values.** The C export names `DAT_80c57...`,
  `DAT_80c580...` and `DAT_80c581...` but does not give their numeric values.
  Every read is kept as a memory load from its original address; none is
  hard-coded.
- **`0x8006CED0` output width.** Two adjacent stack outputs are read as two
  `f32` values (the export reads x from one and y from the other). The exact
  stride is not proven; the calls and consumed words are exact.
- **`0x8006CDE0` extra double argument.** `0x80C331C0` prints a hoisted
  `double` as the first argument on three calls, while every other call site
  passes only the actor. The calls are modelled with the one argument; the
  double is treated as a decompiler artifact.
- **Deferred.** `0x80C33A20` (the 112-byte `Wm_jr_kickhit` effect trigger called
  by `0x80C32800`) and `0x80C04C70` (the array constructor paired with
  `0x80C00200`) remain untranslated. They are called through their raw
  addresses and marked at the call site.

## Verification

- `cargo check` passes with no errors or warnings for `src/game/bosses.rs`.
- `node tools/doc-lint.mjs src` reports `ok: true`, 68 files, 0 violations.
- The 20 evidence files were read in full; every `# Original` block names the
  file and size from `functions.tsv`.
- The 20 rows in `ledger/agents/wave3-bosses.csv` match the Rust functions one
  to one: 17 `translated`, 3 `partial`; 11 `high`, 8 `medium`, 1 `low`.
- No files outside `src/game/bosses.rs`, this document and the wave-3 ledger
  were edited.
