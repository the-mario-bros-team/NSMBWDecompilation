# Coverage - Wave 88 (`game::player`, the `0x80050000` bucket)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 28 | 2228 | 21 | 7 |

## How to read this file

This document covers the Wave-88 slice of the `0x80050000` bucket of
`work/ghidra/main-gb/export/functions.tsv`. Every body lives in
`src/game/player.rs`; the ledger rows are in `ledger/agents/wave88-player.csv`.
The module now holds three hundred eighty-three translated functions.

* **Address** - virtual entry point in the DOL image (not a file offset).
* **Name** - the Ghidra name. No original symbol table survives, so
  `FUN_<address>` is the honest identity; the Rust body keeps the
  `fn_<address>` name.
* **Role** - one plain sentence. Roles marked "inferred" come from shared field
  offsets and call shapes, not from a recovered symbol.
* **Status** - all rows are `translated`: the whole control flow and every call
  are present.
* **Confidence** - strength of the evidence, not code quality: `high` =
  instruction-level match, `medium` = a receiver, a field role or a callee
  argument had to be inferred.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`, plus the stage-record table at `0x80429E98` and the
four-player array at `0x8042A1C8` that waves 18/22/63 already documented. Every
`r13 - k` in the exports below is resolved against that base; constants whose
value or owner the C does not establish stay `// unknown:`.

## What this pass covers

- **Movement-record arc and masks (`0x8005E2B0..0x8005E690`, 8 bodies)** - the
  `+0x12` move-bit setter [`fn_8005e3b0`] (mode `0..8`, with the unsigned
  `mode - 7 > 1` test preserved), the `+0x04`/`+0x06` refresh
  [`fn_8005e2b0`], the start/stop arc step [`fn_8005e300`], the low-nibble
  leaf [`fn_8005e4e0`], the two `0x400`/`0x200` tests
  [`fn_8005e590`]/[`fn_8005e5d0`], and the two `+0x19` wrappers
  [`fn_8005e630`]/[`fn_8005e690`]. These sit between the wave-49 leaves at
  `0x8005E460..0x8005E580` and the wave-63 attributes at `0x8005E6E0`; the
  movement record itself is the `0x3B0`-byte stage record, reached through
  [`W22_FRAME_ARRAY_80429F50`].
- **Table reset pair (`0x8005EA60`, `0x8005ECE0`, 2 bodies)** - [`fn_8005ea60`]
  writes the sixteen slot-table words in the export's interleaved order
  (including the `{0, 1, 3, 2}` slot -> character-id map), raises presence bit
  `0`, then calls [`fn_8005ece0`], which clears the two four-entry id tables and
  resets the per-character counter/aux words of the four mapped characters.
- **Spawn-manager resolvers (`0x8005EE70..0x8005FB90`, 13 bodies)** - the
  indexed stage-record getter [`fn_8005ee70`] (the parameterised form of the
  host-side `w18_stage_record` helper), the spawn request
  [`fn_8005eee0`], the table A accessors [`fn_8005f8c0`]/[`fn_8005f900`], the
  table B insert/remove/count [`fn_8005f990`]/[`fn_8005fa00`]/[`fn_8005fb00`],
  the two resolvers [`fn_8005fb70`]/[`fn_8005fb90`] that waves 49 and 74 were
  already calling by address, and the name scan [`fn_8005fa60`].
- **Presence and per-index leaves (`0x8005F880..0x8005FC70`, 5 bodies)** - the
  two-mask test [`fn_8005f880`], the presence-byte -> mode-word map
  [`fn_8005f920`], the character-flag mode getter [`fn_8005fbe0`], the
  per-index byte/word writer and readers
  [`fn_8005fc20`]/[`fn_8005fc40`]/[`fn_8005fc50`], and the four-slot mask
  builder [`fn_8005fc70`].

The two arg orderings that matter were cross-checked against the untranslated
wrapper `0x8005F4D0`, which calls `0x8005F900(slot)` explicitly, then
`0x8005F570(param_2, slot)` and `0x8005EEE0(slot, param_1, table[param_3], 0)`.
That call fixes the four-argument shape of [`fn_8005eee0`] and confirms that
[`fn_8005f900`]'s value (not a side effect) is what the caller tests.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8005E2B0` | `fn_8005e2b0` | 80 | high |
| `0x8005E300` | `fn_8005e300` | 168 | medium |
| `0x8005E3B0` | `fn_8005e3b0` | 164 | high |
| `0x8005E4E0` | `fn_8005e4e0` | 12 | high |
| `0x8005E590` | `fn_8005e590` | 56 | high |
| `0x8005E5D0` | `fn_8005e5d0` | 56 | high |
| `0x8005E630` | `fn_8005e630` | 72 | medium |
| `0x8005E690` | `fn_8005e690` | 72 | medium |
| `0x8005EA60` | `fn_8005ea60` | 164 | high |
| `0x8005EC90` | `fn_8005ec90` | 8 | high |
| `0x8005ECE0` | `fn_8005ece0` | 164 | high |
| `0x8005EE70` | `fn_8005ee70` | 32 | high |
| `0x8005EEE0` | `fn_8005eee0` | 100 | high |
| `0x8005F880` | `fn_8005f880` | 60 | high |
| `0x8005F8C0` | `fn_8005f8c0` | 56 | high |
| `0x8005F900` | `fn_8005f900` | 20 | medium |
| `0x8005F920` | `fn_8005f920` | 100 | high |
| `0x8005F990` | `fn_8005f990` | 100 | high |
| `0x8005FA00` | `fn_8005fa00` | 88 | high |
| `0x8005FA60` | `fn_8005fa60` | 156 | medium |
| `0x8005FB00` | `fn_8005fb00` | 108 | high |
| `0x8005FB70` | `fn_8005fb70` | 20 | medium |
| `0x8005FB90` | `fn_8005fb90` | 80 | medium |
| `0x8005FBE0` | `fn_8005fbe0` | 64 | high |
| `0x8005FC20` | `fn_8005fc20` | 28 | high |
| `0x8005FC40` | `fn_8005fc40` | 12 | high |
| `0x8005FC50` | `fn_8005fc50` | 20 | high |
| `0x8005FC70` | `fn_8005fc70` | 168 | high |

Byte total: 2228. Address range covered: `0x8005E2B0..0x8005FC70`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket holds 78 untranslated functions after this pass.

* **Large action-state dispatcher `0x8005E040` (624 B)** - the double-buffered
  `+0x0E..+0x3C` history shift with two thunk calls. It is reconstructable but
  needs its own pass over the `+0x0C..+0x40` record layout; translating it here
  would have doubled the lane's review surface for one body.
* **Spawn wrappers `0x8005F4D0` (156 B) and `0x8005F570` (80 B)** - read only to
  fix the argument order of [`fn_8005eee0`] and to confirm the
  `0x80354E60`/`0x80354E80` roles. `0x8005F4D0` indexes a table at
  `0x8042BA90` whose contents are not exported, so its own body would have to
  carry an `// unknown:` word with no way to describe it.
* **Presence-bit setters `0x8005FDB0` and `0x8005FE30`** - both call
  `0x8005F900` with the argument printed as missing and read the `+0x60`
  vtable `+0xE0` slot; they are the same inference as [`fn_8005fb90`], kept out
  to avoid three more `medium` rows that only repeat one call shape.
* **Counter leaves `0x8005FEB0`, `0x8005FED0`, `0x8005FEF0`, `0x8005FFB0`** -
  four tiny bodies over the `0x80429CA0` count and the `0x80354E90` counters.
  [`fn_8005ece0`] and [`fn_8005ea60`] already document the tables they touch,
  and wave 74's [`W74_SLOT_COUNT_8005FFB0`] still calls `0x8005FFB0` by address,
  so these are a clean follow-up rather than a gap in this pass.
* **Float/double helpers `0x8005ED90`, `0x8005EF50`** - `0x8005ED90` converts
  two `unsigned` values through the `0x43300000` double trick against
  `r2 - 0x7610`/`r2 - 0x7618` and then reads the `0x802EF188` table; neither the
  doubles nor the per-stance table are resolvable from the C export.
  `0x8005EF50` (1408 B) is a full per-frame update over the same tables.
* **Unrecovered call sites `0x8005EE90`, `0x8005F5C0`, `0x8005F880`'s
  neighbours** - `0x8005EE90` reads `+0x0B` from the result of `0x8008E3D0`,
  which can return zero, so the body would encode a read through a null entry.
  `0x8005F5C0` (696 B) opens with the outlined-prologue artefact
  `0x802DCD74`, which is the same "arguments live in the caller's frame"
  problem wave 74 already flagged for `0x8005BFD0`.
* `0x8005E620` and `0x8005E680` are four-byte `b` thunks onto
  [`fn_8005e590`]/[`fn_8005e5d0`]; they carry no body of their own.

## Verification

- Every address and byte size matches `functions.tsv` (checked mechanically for
  all 28 rows against the `# Original` line).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` reports no error and no warning for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave88` reports `OK` for
  `ledger/agents/wave88-player.csv` (28 rows, 7 fields each).
