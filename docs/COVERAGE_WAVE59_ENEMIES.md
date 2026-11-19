# Coverage - Wave 59 (enemies: `0x80A447D0..0x80A46CC0`)

## How to read this file

This page records the Wave 59 enemies lane: the `0x80A40000` bucket block
starting just after the translated `daEnCoinJugem_c` registration helpers,
promoted to `src/game/enemies.rs`. Every row corresponds to a real
`pub unsafe fn` body in that file, and the `evidence` column of the lane CSV
names the Ghidra C export each body was reconstructed from.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these are runtime addresses.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 4,164 | 13 | 8 | 8 |

## What this pass covers

The pass is the run `0x80A447D0..0x80A46CC0`, read from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. The bucket holds 189
functions; 22 of them were already in `ledger/functions.csv`
(`0x80A41180`..`0x80A4A730`), and none of those addresses is repeated here.

The run contains four actor groups that share one construction skeleton: a
registration pair, a zeroed allocation, the base constructor, a `+0x534`
holder, a `+0x698` matrix block, a `+0x74C` model block, three-element
animation banks (`+0x76C`/`+0x7AC`/`+0x7EC` repeated at `+0x8F0`), two
`0x803295xx` object pools and a `+0xAC4` target slot. Class sizes and final
vtables are what separate them.

- **Group 1 (`0x80A447D0..0x80A44CA0`, 0x8E8 bytes, vtable `0x80B40DCC`).**
  Two 48-byte argument-swapping forwarders, the allocating constructor, a
  timer/virtual-call frame step, the `+0x534` holder reset, the table-driven
  state entry and the contact/reflect update.
- **Group 2 (`0x80A44F80..0x80A45780`, 0xAE8 bytes, vtable `0x80B41444`).**
  Deleting destructor, two forwarders, the allocating constructor, the
  spawn-word placement initialiser, the class-`0x23C` target scan, the
  `+0xACC` slot copy, the lane-offset re-application and the retry step.
- **Group 3 (`0x80A45AB0..0x80A46180`, 0xAE0 bytes, vtable `0x80B41AC4`).**
  Deleting destructor, two forwarders, the allocating constructor, the
  `-x`/`-z` matrix commit, the ±`0x3FF` pitch ramp and the ramp integrator.
- **Group 4 (`0x80A466F0..0x80A46CC0`, 0xAE8 bytes, vtable `0x80B4213C`).**
  Deleting destructor, two forwarders, the second 0xAE8 constructor, the
  `+0x534` holder reset and the four-call update.

- **Constants read from the image.** `work/rels/elf/d_enemiesNP.elf` was read
  at the analysis address (via its program headers) for every table cited:
  `0x80B2C734 = 0.0`; `0x80B2C748..0x754 = 0.5, -0.5, 1.0, -1.0`;
  `0x80B2C768..0x774 = 3.5, 3.75, 4.0, 3.0`; `0x80B2C778/0x80B2C77C = ±0.3`;
  `0x80B2C780 = 8.0`, `0x80B2C784 = -3000.0`, `0x80B2C788 = 500.0`,
  `0x80B2C78C = 1.0`, `0x80B2C790 = 0.0`, `0x80B2C794 = 256.0`,
  `0x80B2C7B0 = 8.0`. `0x80B2C798` is `0x4330000000000000`, the MWCC
  int-to-double bias used by the spawn-word decode, not a game constant.
- **Identity evidence.** The final vtables `0x80B40DCC`, `0x80B41444`,
  `0x80B41AC4` and `0x80B4213C` are stored at `+0x60` last in each
  constructor; the class names are not printed anywhere in this run, so the
  actors are identified by size, vtable and shared offsets only.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A447D0` | `fn_80a447d0` | 48 | low |
| `0x80A44800` | `fn_80a44800` | 48 | low |
| `0x80A44830` | `fn_80a44830` | 260 | high |
| `0x80A44A40` | `fn_80a44a40` | 92 | medium |
| `0x80A44AD0` | `fn_80a44ad0` | 40 | high |
| `0x80A44BA0` | `fn_80a44ba0` | 236 | medium |
| `0x80A44CA0` | `fn_80a44ca0` | 184 | medium |
| `0x80A44F80` | `fn_80a44f80` | 88 | high |
| `0x80A450A0` | `fn_80a450a0` | 48 | low |
| `0x80A450D0` | `fn_80a450d0` | 48 | low |
| `0x80A45100` | `fn_80a45100` | 384 | high |
| `0x80A45280` | `fn_80a45280` | 728 | medium |
| `0x80A45650` | `fn_80a45650` | 92 | medium |
| `0x80A456B0` | `fn_80a456b0` | 12 | high |
| `0x80A456C0` | `fn_80a456c0` | 160 | medium |
| `0x80A45780` | `fn_80a45780` | 100 | medium |
| `0x80A45AB0` | `fn_80a45ab0` | 88 | high |
| `0x80A45BD0` | `fn_80a45bd0` | 48 | low |
| `0x80A45C00` | `fn_80a45c00` | 48 | low |
| `0x80A45C30` | `fn_80a45c30` | 384 | high |
| `0x80A46090` | `fn_80a46090` | 104 | high |
| `0x80A46100` | `fn_80a46100` | 124 | high |
| `0x80A46180` | `fn_80a46180` | 128 | high |
| `0x80A466F0` | `fn_80a466f0` | 88 | high |
| `0x80A46810` | `fn_80a46810` | 48 | low |
| `0x80A46840` | `fn_80a46840` | 48 | low |
| `0x80A46870` | `fn_80a46870` | 376 | high |
| `0x80A46B30` | `fn_80a46b30` | 40 | high |
| `0x80A46CC0` | `fn_80a46cc0` | 72 | medium |

## Deliberate skips

- **`0x80A46200` (300 bytes).** The vertical step decodes a `short` through
  `CONCAT22`/`ldexpf` GQR artifacts and its export removes two unreachable
  blocks. The body could not be reconstructed without guessing the conversion,
  so it is skipped; [`fn_80a46180`] covers the ramp side of the same state.
- **`0x80A46BF0` (172 bytes).** The spin step tests `0x80162D50(0x79, 0)` but
  then reads `iVar5 + 0x394` on both paths, including the null path. The
  export therefore dereferences a null object, so no honest body exists.
  [`fn_80a46cc0`] keeps the call as address `0x80A46BF0`.
- **`0x80A46D10` and `0x80A46DF0`.** The next state-entry/update pair sits
  immediately after the run; it was left for a later pass to keep this lane
  inside the 25-30 function budget.
- **Registration pair identity.** `0x802DCD74`/`0x802DCDC0` and
  `0x802DCD6C`/`0x802DCDB8` are printed without argument detail. The
  constructors reproduce the observed order and mark the calls `// inferred:`.
- **Forwarder identity.** All six 48-byte forwarders reproduce the literal
  `(second, second, first)` call to `0x802DCBFC`. The target's role is not
  established, so those six rows are `low`.
- **Spawn-word fields.** The four fields packed into `+0x04` are presented as
  lane (`bits 0..3`), lift (`bits 4..6`), the height branch (`bit 8`) and a
  key byte (`bits 28..31`) because that is how they are consumed here; the
  naming is inferred and the row stays `medium`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to
  4,164 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/enemies.rs`; the only error in the tree belongs to
  `src/game/stage.rs` (another Wave 59 lane).
- `node tools/check-lane-csv.mjs wave59` reports `OK` with
  `wave59-enemies.csv: 29 rows`.
