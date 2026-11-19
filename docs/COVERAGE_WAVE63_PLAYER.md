# Coverage - Wave 63 (game::player — four-slot player manager and attribute leaves)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 29 | 2464 | 19 | 10 |

## How to read this file

This document covers the Wave-63 slice of the gameplay player actor from the
`0x80050000` bucket of `work/ghidra/main-gb/export/functions.tsv`. Every body
lives in `src/game/player.rs`; the ledger rows are in
`ledger/agents/wave63-player.csv`. The module now holds two hundred sixty-nine
translated functions.

* **Address** — virtual entry point in the DOL image (not a file offset).
* **Name** — the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the `fn_<address>`
  name.
* **Role** — one plain sentence. Roles marked "inferred" below come from the
  shared field offsets, not from a recovered symbol.
* **Status** — all rows are `translated`: the full control flow and every call
  are present.
* **Confidence** — strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a callee argument or a return convention
  had to be restored.
* **Evidence** — the Ghidra C file under `work/ghidra/main-gb/export/`.

Addresses follow the module's established SDA bases `r13 = 0x8042F680` and
`r2 = 0x80433080`, so `r13 - 0x59E0` is `0x80429CA0` and `r13 - 0x59D8` is
`0x80429CA8`. Both are documented as constants in the Wave-63 block.

## What this pass covers

- **Four-slot player manager (`0x8005C930..0x8005D0D0`, 13 bodies)** — the
  manager is a state machine over the four gameplay players. The SDA byte at
  `0x80429CA8` carries one presence bit per slot, `0x8005FB90` resolves a slot
  index to its player object, the state word lives at `+0x08`, and `+0x8C`
  caches the first slot currently in the "damage or held" state (`-1` when
  none). The pass covers the three whole-slot predicates
  (`fn_8005c930`, `fn_8005cc00`, `fn_8005cdd0`), the three whole-slot actions
  (`fn_8005ca50`, `fn_8005cb70`, `fn_8005ce50`), the mode-4 counter
  `fn_8005cad0`, the request dispatcher `fn_8005ccd0`, the state-advance
  `fn_8005ced0`, the per-frame tick `fn_8005cfb0`, the limit-word writer
  `fn_8005cb60`, and the `+0x60..+0x6C` history accessors
  (`fn_8005d0c0`, `fn_8005d0d0`). Start at `fn_8005cfb0`.
- **Attribute and movement leaves (`0x8005E6E0..0x8005E9A0`, 16 bodies)** — the
  read-only side of the movement-flag halfwords: three four-byte branch thunks
  into the already-translated leaves (`fn_8005e6e0` -> `0x8005E570`,
  `fn_8005e770` -> `0x8005E580`, `fn_8005e830` -> `0x8005E500`), the `+0x19`
  flag write, the tri-state mode reader `fn_8005e740`, the `+0x17`/`+0x18` byte
  pair, the two-bit attribute report `fn_8005e7a0`, the two gated low-bit
  readers, the ten-word record scan `fn_8005e8b0`, the `+0x12 & 0x80`-gated
  `+0x0C`/`+0x0E`/`+0x10` set/clear quartet, and the type-`0xE` spawn wrapper
  `fn_8005e9a0` that binds an optional source object through `0x8014EA30`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8005C930` | `fn_8005c930` | 160 | medium |
| `0x8005CA50` | `fn_8005ca50` | 120 | medium |
| `0x8005CAD0` | `fn_8005cad0` | 136 | high |
| `0x8005CB60` | `fn_8005cb60` | 12 | high |
| `0x8005CB70` | `fn_8005cb70` | 140 | high |
| `0x8005CC00` | `fn_8005cc00` | 208 | medium |
| `0x8005CCD0` | `fn_8005ccd0` | 252 | high |
| `0x8005CDD0` | `fn_8005cdd0` | 124 | medium |
| `0x8005CE50` | `fn_8005ce50` | 120 | high |
| `0x8005CED0` | `fn_8005ced0` | 216 | medium |
| `0x8005CFB0` | `fn_8005cfb0` | 160 | high |
| `0x8005D0C0` | `fn_8005d0c0` | 8 | high |
| `0x8005D0D0` | `fn_8005d0d0` | 36 | high |
| `0x8005E6E0` | `fn_8005e6e0` | 4 | high |
| `0x8005E6F0` | `fn_8005e6f0` | 72 | high |
| `0x8005E740` | `fn_8005e740` | 40 | medium |
| `0x8005E770` | `fn_8005e770` | 4 | high |
| `0x8005E780` | `fn_8005e780` | 8 | high |
| `0x8005E790` | `fn_8005e790` | 16 | high |
| `0x8005E7A0` | `fn_8005e7a0` | 132 | medium |
| `0x8005E830` | `fn_8005e830` | 4 | high |
| `0x8005E840` | `fn_8005e840` | 68 | medium |
| `0x8005E890` | `fn_8005e890` | 24 | medium |
| `0x8005E8B0` | `fn_8005e8b0` | 92 | high |
| `0x8005E910` | `fn_8005e910` | 32 | high |
| `0x8005E930` | `fn_8005e930` | 36 | high |
| `0x8005E960` | `fn_8005e960` | 32 | high |
| `0x8005E980` | `fn_8005e980` | 24 | high |
| `0x8005E9A0` | `fn_8005e9a0` | 184 | medium |

Byte total: 2464. Address range covered: `0x8005C930..0x8005E9A0`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket still holds 132 untranslated functions after this pass.

* `0x8005E620`, `0x8005E630`, `0x8005E680`, `0x8005E690` — form the other half
  of the `0x8005E6xx` thunk family but reach the two bodies
  `0x8005E590`/`0x8005E5D0`, which index a table through `r13 - 0x5730`
  (`0x80429F50`). That table's element type and the `+8 == 1` test inside it are
  not resolved, so the thunks cannot be reduced to a call to a translated body.
  Left for a later pass rather than guessed.
* `0x8005FB90` — the slot accessor every body in this pass calls. It is not
  translated because the export types its callee `0x8005F900` as `void` while
  the caller consumes that callee's `r3`; the accessor's own contract therefore
  cannot be stated honestly. The Wave-63 block documents it as a `call1`
  constant instead.
* `0x8005B8A0` (1100 B), `0x8005C410` (692 B), `0x8005D100` (380 B),
  `0x8005D280` (1064 B), `0x8005D880` (628 B), `0x8005E040` (624 B),
  `0x8005EF50` (1408 B) — the large dispatchers that sit between the clusters
  selected above; each needs its own call-graph pass.
* `0x8005C1A0..0x8005C410` and `0x8005D7E0..0x8005DB00` — the remaining
  slot-object and per-slot-table helpers; not touched here because the
  manager group and the leaf group already met the lane target.
* `0x8005DC80`, `0x8005DDC0` — still paired-single code in the export, as
  recorded in `docs/COVERAGE_WAVE49_PLAYER.md`.
* `0x80058280`, `0x800582A0`, `0x800582F0` — the `r13 - 0x5678` / `r2 - 0x77BC`
  seams already recorded as skips in Wave 49; unchanged.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave63` reports `OK`.
