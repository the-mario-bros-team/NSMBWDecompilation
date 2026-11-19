# Coverage - Wave 87 (`game::player`, the `0x80040000` bucket remains)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/player.rs` | `game::player` | 30 | 2000 | 25 | 5 |

## How to read this file

This document covers the Wave-87 slice of the `0x80040000` bucket of
`work/ghidra/main-gb/export/functions.tsv`, the last bucket with a large
untranslated remainder that belongs to the gameplay actor class. Every body
lives in `src/game/player.rs`; the ledger rows are in
`ledger/agents/wave87-player.csv`. The module now holds three hundred
fifty-five translated functions.

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
  identity had to be inferred.
* **Evidence** - the Ghidra C file under `work/ghidra/main-gb/export/`.

The bodies use the two SDA bases this module already fixes: `r2 = 0x80433080`
and `r13 = 0x8042F680`. Every `r2 - k` quoted below is resolved against that
base, and the `0x8042B8xx` float block keeps the values earlier waves already
documented (`0x8042B874` = `1.0f`, `0x8042B878` = `0.0f`, `0x8042B89C` =
`1.5f`, `0x8042B8A0` = `3.0f`). Constants the export does not resolve stay
named `// unknown:` rather than guessed.

## What this pass covers

- **Constant table getters (`0x80040600..0x80042B30`, 14 bodies)** - twelve-byte
  `lis`/`addi`/`blr` leaves that publish a fixed main.dol descriptor instead of a
  per-instance value. Four return distinct tables
  (`0x80353AE4`, `0x80353B24`, `0x80353AA4`, `0x80353B64`); the other ten all
  return `0x80377A04`, the same constant the `game::bases` lane documents for the
  `0x8066AFF0` family. They are the largest remaining contiguous run in this
  bucket and the whole body is the address, so no behaviour is inferred.
- **Chain and name twins (`0x800435D0`, `0x800436B0`, 2 bodies)** -
  [`fn_800435d0`] repeats the five unrolled `+0x30` child probes and the
  recursion of [`fn_80041a00`]; [`fn_800436b0`] repeats the `prefix:suffix`
  comparison of [`fn_80041ae0`] and differs only in that it does not null-test
  the name's `strrchr` result before adding one. Both are byte-identical twins of
  already-translated bodies, which is why the field roles are not re-derived.
- **Teardown wrappers (`0x80044E50..0x80046410`, 4 bodies)** - the
  "reset the member, then free only when `destroy > 0`" shape of
  [`fn_800418c0`]. [`fn_80044e50`] and [`fn_80044eb0`] use
  `0x8015F800`/`0x802B9280`, [`fn_80046070`] uses
  `0x800671B0`/`0x80162920`, and [`fn_80046410`] uses
  `0x802D7B20`/`0x802B9280`. The comma expression in the guard is preserved, so
  the member reset still runs on every non-null call.
- **Sub-object tick (`0x80045EF0`, 1 body)** - [`fn_80045ef0`] runs `0x8007F950`
  on the `+0x3E0` sub-object and returns `1`.
- **State gates and interface leaves (`0x80046A50..0x80047EE0`, 8 bodies)** -
  [`fn_80046a50`] resets the `+0xEA4` move state, forces mode `8` while the
  global mode word `0x8042A204` is `2`, then dispatches class vtable `+0x424`;
  [`fn_80046ab0`] is the long flag rewrite (`0x7D` guard, three `+0x1044`
  copies into `+0xB8`, then the `0x83`->`0x82` swap); [`fn_80047190`],
  [`fn_80047390`] and [`fn_80047440`] are the `+0x1464` interface leaves for
  slots `+0x14`, `+0x18` and `+0x10`; [`fn_80047800`] and [`fn_80047920`] are
  the two speed/rate gates that share the `+0x2C8` magnitude and the `+0x10D4`
  bit `0x800000`; [`fn_80047ee0`] is the guard in front of the `0x80047F10`
  probe.
- **One-store leaf (`0x8004A790`, 1 body)** - [`fn_8004a790`] clears `+0x1008`.

Start at [`fn_800435d0`] for the structural twin, then [`fn_80046ab0`] for the
densest body, then [`fn_80047800`] for the only body where two separate clamp
tables had to stay `// unknown:`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80040600` | `fn_80040600` | 12 | high |
| `0x80040610` | `fn_80040610` | 12 | high |
| `0x80040620` | `fn_80040620` | 12 | high |
| `0x80040630` | `fn_80040630` | 12 | high |
| `0x80040920` | `fn_80040920` | 12 | high |
| `0x80040930` | `fn_80040930` | 12 | high |
| `0x80040940` | `fn_80040940` | 12 | high |
| `0x80040950` | `fn_80040950` | 12 | high |
| `0x80042AE0` | `fn_80042ae0` | 12 | high |
| `0x80042AF0` | `fn_80042af0` | 12 | high |
| `0x80042B00` | `fn_80042b00` | 12 | high |
| `0x80042B10` | `fn_80042b10` | 12 | high |
| `0x80042B20` | `fn_80042b20` | 12 | high |
| `0x80042B30` | `fn_80042b30` | 12 | high |
| `0x800435D0` | `fn_800435d0` | 224 | medium |
| `0x800436B0` | `fn_800436b0` | 136 | high |
| `0x80044E50` | `fn_80044e50` | 88 | high |
| `0x80044EB0` | `fn_80044eb0` | 92 | high |
| `0x80045EF0` | `fn_80045ef0` | 40 | high |
| `0x80046070` | `fn_80046070` | 88 | high |
| `0x80046410` | `fn_80046410` | 92 | high |
| `0x80046A50` | `fn_80046a50` | 96 | medium |
| `0x80046AB0` | `fn_80046ab0` | 252 | high |
| `0x80047190` | `fn_80047190` | 80 | high |
| `0x80047390` | `fn_80047390` | 156 | high |
| `0x80047440` | `fn_80047440` | 36 | medium |
| `0x80047800` | `fn_80047800` | 280 | medium |
| `0x80047920` | `fn_80047920` | 124 | high |
| `0x80047EE0` | `fn_80047ee0` | 36 | medium |
| `0x8004A790` | `fn_8004a790` | 12 | high |

Byte total: 2000. Address range covered: `0x80040600..0x8004A790`
(non-contiguous; the gaps are the deliberate skips below).

## Deliberate skips and why

The bucket holds 77 untranslated functions after this pass.

* **Thirty-seven more twelve-byte getters** - `0x80047270..0x80047380` (18) and
  `0x8004D6F0..0x8004D810` (19). Both runs are the same shape as the fourteen
  bodies above and are trivially reconstructable, but they are identical
  duplicates of one another; translating forty-seven copies of
  `return &DAT_80377a04` would pad the ledger without adding coverage. They are
  left for a sweep that can state the ownership of `0x80377A04` at the same
  time. `0x80044940` and `0x80044950` are two more members of the same run.
* **Unrecoverable interface thunks** - `0x800408F0`, `0x80041BA0`, `0x80041BD0`,
  `0x80043770`, `0x800437A0` and `0x8004DB00`. Every one is a twelve- or
  fifty-two-byte tail call into `0x802DCBFC`, whose own export is an
  `UNRECOVERED_JUMPTABLE` over `in_r12` with the warning *"Could not recover
  jumptable at 0x802dcc20"*. Neither the real callee nor its argument list can be
  stated honestly, so no body was invented.
* **Unresolved float constants** - `0x800459F0`, `0x80045B10`, `0x80045D00`,
  `0x80045590` and `0x80045A10`. Each multiplies or offsets by a `float` in the
  `0x8042B81x..0x8042B85x` block that the C export only names as `r2 - k`; no
  data export exists in the repository to read the value. `0x80045A10`
  additionally carries the `ldexpf`/GQR artefact and an unreachable-block
  warning. `0x80045B10` also calls `0x800459F0`, so the pair would have to be
  guessed together.
* **Double-argument virtual calls** - `0x80047C40` and `0x80047DC0`. Both pass
  three `(double)`-promoted floats plus the receiver to a vtable slot and read
  data doubles (`0x80370E14`, `0x80370EC8`, `0x80370ECC`) that are not
  resolvable; the argument contract cannot be reconstructed from the C.
* **Large dispatchers and constructors** - `0x800414E0` (888 B), `0x800441E0`
  (432 B), `0x80044550` (520 B), `0x80045710` (732 B), `0x800460D0` (828 B),
  `0x800467C0` (652 B), `0x80046FC0` (452 B), `0x80047630` (348 B),
  `0x80048630` (704 B), `0x80047F10` (360 B), `0x800410F0` (212 B),
  `0x80044FF0` (224 B), `0x800450D0` (136 B), `0x80044460` (240 B),
  `0x80042650` (164 B), `0x80042480` (128 B), `0x80040F70` (128 B),
  `0x80040DE0` (144 B), `0x80040E70` (80 B), `0x80040EC0` (76 B),
  `0x800437D0` (72 B) and the remaining 48-byte
  `0x80045190`/`0x800451C0` leaves. These are multi-table dispatchers over the
  `+0x30`/`+0x48` blocks or initialisers that read the same unresolved
  `0x8042B8xx` data; each needs its own call-graph pass.
* `0x80046490` is one byte-shape away from [`fn_80046410`] but its reset callee
  is printed as `FUN_8027A200()` with no argument even though the actor is still
  in `r3`. Rather than pass a guessed receiver it stays untranslated; the three
  siblings that do print their argument are above.

## Verification

- Every address and byte size matches `functions.tsv` (checked mechanically for
  all 30 rows against the `# Original` line).
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/game/player.rs`.
- No address was already in `ledger/functions.csv`, and none of the 30
  duplicates a wave-3..74 body.
- `cargo check --message-format=short` is free of errors and of diagnostics for
  `src/game/player.rs`.
- `node tools/check-lane-csv.mjs wave87` reports `OK` for
  `ledger/agents/wave87-player.csv` (30 rows, 7 fields each).
