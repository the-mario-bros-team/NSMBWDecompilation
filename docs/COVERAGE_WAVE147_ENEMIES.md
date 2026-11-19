# Coverage - Wave 147 (`game::enemies`)

## How to read this file

This page records the Wave 147 `game::enemies` lane: 15 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 129 and mirrored
one row per body in `ledger/agents/wave147-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Raw words
were decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` headers
as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment only),
and every body was followed from its entry to `blr`. Byte counts are the `size`
column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 15 | 7560 | 11 | 4 |

Status split: 15 `translated` (11 high, 4 medium), no `partial` and no `stub`.

Covered address range: `0x80A1C8F0..0x80B16060` (6388 bytes of
`docs/SPEC_WAVE146_ENEMIES.md` ready rows and 1172 bytes of `0x80A50000`
planner-bucket rows).

## What this pass covers

- **All ten "ready" `docs/SPEC_WAVE146_ENEMIES.md` rows.** `0x80A84F10` (the
  `I_kinoko` brres/clip binder over the three pointer tables at
  `0x80B4E218`/`0x80B4E2F4`/`0x80B4E34C`), `0x80A56620` (the multi-slot side
  classifier over `+0x2484`), `0x80B0D6E0` (the turn-order heading/pitch
  governor with the four `0.625/1.0` and `0.0078125/0.03125/0.5` tables),
  `0x80AB0570` (the `1532/16` lane ladder initialiser), `0x80AB0EE0` (the
  five-slot world-matrix refresh), `0x80A1C8F0` (the player-attack collision
  dispatcher and its 31-entry table at `0x80B38334`), `0x80B16060` (the
  `0x11D`-variant initialiser), `0x80A1E990` (the `qr3` aim/placement step),
  `0x80B16F40` (the `150.0`-gated travel-vector step) and `0x80AD98B0` (the
  self-initialising four-player overlap probe).
- **Five `0x80A50000` planner-bucket rows.** `0x80A56920` (the growing-radius
  segment ring fill with four `qr3` sites), `0x80A5AEF0` (the `0x80357BF4`
  constant producer), `0x80A589A0` (the four-player `96.0` distance test that
  `fn_80a59230` had recorded as unresolved), `0x80A58D70` (the `+0x1146`
  wobble step) and `0x80A584E0` (the `+0x780` facing step).

### `qr3` sites

Six data `qr3` sites are ported here, and every one is the unsigned-halfword
widen pinned by `docs/REL_ELF_MAPPING.md` (`GQR3 = 0x00050005`, `W = 1`,
`I = 3`):

| address | site | width | port |
|---|---|---|---|
| `0x80A1EA10` | sine angle, mode A | u16 | `(angle as u16) as f32 * 1/256` |
| `0x80A1EA40` | cosine angle, mode A | u16 | same halfword |
| `0x80A1EAD8` | sine angle, mode B | u16 | `(yaw as u16) as f32 * 1/256` |
| `0x80A1EAF8` | cosine angle, mode B | u16 | same halfword |
| `0x80B1704C` | sine angle | u16 | `(yaw as u16) as f32 * 1/256` |
| `0x80B17080` | cosine angle | u16 | same halfword |

`0x80A56920` has four further `qr3` sites (`0x80A569BC/0x80A569D8` and the
mirror pair `0x80A56A30/0x80A56A58`), all the same `(f32)(u16)` load, and
`0x80A58D70` has one (`0x80A58E28`). Every other `psq_*` in the 15 rows is a
`qr0` `f27..f31` stack spill and is not ported.

### Export/specification corrections made while porting

Four places disagree with the specification page or the Ghidra stack naming,
and the raw stream won:

- **`0x80A1E990` sine/cosine.** The specification says `pos.x = target.x +
  radius*sin`. The raw words at `0x80A1EA20` call `0x80237C40` (the cosine
  table at `0x802F74A4`) and at `0x80A1EA4C` call `0x80237BD0` (the sine table
  at `0x802F74A0`); the port therefore uses cosine for x and sine for y, which
  is the standard `r*cos/r*sin` circle. Both table identities were verified
  from `work/dol/main.elf` (entry 0 of `0x802F74A0` is `0.0`, entry 0 of
  `0x802F74A4` is `1.0`).
- **`0x80B16060` state descriptors.** The specification prints
  `0x80B68EB8/0x80B68EF8/0x80B68F38`. The raw `lis r29,0x80B8` at
  `0x80B1607C` and `addi r29,r29,-0x7158` at `0x80B16098` resolve to
  `0x80B78EA8`, so the three passed descriptors are
  `0x80B78EB8/0x80B78EF8/0x80B78F38` (BSS state records, 0x40 bytes apart).
- **`0x80AB0EE0` bone buffer.** The export names the three bone reads
  `local_3c/local_2c/local_1c`. The raw stream passes `r1+0x78` (mode A) and
  `r1+0x48` (mode B) to `0x80168180` and reads the translation column at
  `+0xC/+0x1C/+0x2C` of a `0x30`-byte matrix. The port reads those offsets and
  says so in the doc block.
- **`0x80A58D70` `+0xEC`.** The export folds the `+0xEC = 0.0` store into a
  comma expression on the `ground == 0` arm. The raw words at
  `0x80A58DB0..0x80A58DB8` store it before the `+0x500` test, so the port
  writes it unconditionally.

### Repair to an existing note

`fn_80a59230` (wave 127) recorded `0x80A589A0` as "GQR-damaged ... exact
arithmetic is not recoverable". The export is complete and the body is now
`fn_80a589a0`; the call site already used the same `(actor) -> 0/1` contract,
so no call-side change was needed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A1C8F0` | `fn_80a1c8f0` | 636 | high |
| `0x80A1E990` | `fn_80a1e990` | 560 | high |
| `0x80A56620` | `fn_80a56620` | 736 | medium |
| `0x80A56920` | `fn_80a56920` | 476 | medium |
| `0x80A584E0` | `fn_80a584e0` | 232 | medium |
| `0x80A589A0` | `fn_80a589a0` | 144 | high |
| `0x80A58D70` | `fn_80a58d70` | 308 | medium |
| `0x80A5AEF0` | `fn_80a5aef0` | 12 | high |
| `0x80A84F10` | `fn_80a84f10` | 744 | high |
| `0x80AB0570` | `fn_80ab0570` | 664 | high |
| `0x80AB0EE0` | `fn_80ab0ee0` | 640 | high |
| `0x80AD98B0` | `fn_80ad98b0` | 544 | high |
| `0x80B0D6E0` | `fn_80b0d6e0` | 696 | high |
| `0x80B16060` | `fn_80b16060` | 608 | high |
| `0x80B16F40` | `fn_80b16f40` | 560 | high |

### Why four rows are `medium`

- **`0x80A56620`** - the loop, strides and probe geometry are literal; the
  `0x400`/low-byte encoding of the two cell tests is the recorded contract of
  `0x80070BA0`/`0x80070BF0`, not a proof.
- **`0x80A56920`** - the loop, four `qr3` sites and all stores are literal;
  the `12.0`/`1/256`/`8.0` roles are read from the cited words and the
  sine/cosine identities are the same verified pair.
- **`0x80A58D70`** - every constant and store is literal; the `+0xE0`/`+0xDC`
  ramps are damped acts with inferred names.
- **`0x80A584E0`** - the call order and both scaled stores are literal; the
  `4544.0` radius role is inferred from the `+0x780` pair.

## Deliberate skips

- **`0x80A0F610` (660 bytes, spec rank 5, conditional).** The per-side yaw
  seeds at `0x80B6EF08` are BSS and nothing in this REL writes them; the
  specification itself leaves the table unsigned, so the body was not
  invented.
- **`0x80A85260` (528 bytes, spec rank 12, conditional).** The ground read
  `*(f32 *)(*(u32 *)0x80429E00 + 0x3C)` is a `main.dol` object whose layout
  is outside this REL.
- **`0x80A56B10` (672 bytes, bucket).** The main path enters
  `lwzu r12,0x20D0(r3)` and a vtable call at `+0xA0` at `0x80A56B8C`; the
  walk cannot follow it, so the body length is not established. This is the
  same reason `docs/SPEC_WAVE146_ENEMIES.md` held it back.
- **`0x80A53850` (368 bytes, bucket).** `0x800EB930` returns a float in `f1`
  in addition to its `r3` verdict (`extraout_f1` in the export); the
  argument/return pair is not established, so the lift/gate arithmetic cannot
  be stated honestly.
- **`0x80A5C660` (436 bytes, bucket).** The body depends on `0x802DCD70`,
  `0x80275560`, `0x8019ECE0`, `0x80267420` and a `f1` side channel
  (`extraout_f1`); the float state threading is not recoverable from this
  export alone.
- **`0x80A5DBC0` (212 bytes, bucket).** The `0x10000 / fVar2` reciprocal, the
  `0x43300000` fold and the `0x80168080` holder read are literal, but the
  second angle source at `+0x660` is not established as a role; left for a
  later pass rather than guessing.
- **`0x80A5E1A0` (136 bytes, bucket).** The export drops the clip-name
  argument of `0x8023A0B0` and folds the first two float registers into an
  `undefined8`; the argument layout cannot be stated without a second
  disassembly pass.
- **`0x80A5E590` (252 bytes, bucket).** It calls `0x80A5E1A0`/`0x80A5E230`
  and three vtable slots; with `0x80A5E1A0` unresolved the body is not
  reconstructible honestly.
- **`0x80A5EB00` (448 bytes, bucket).** Five unledgered `main.dol` helpers and
  a four-way clip test at `0x801690E0`; the clip-name roles are not
  established.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 15 sizes sum to 7560
  bytes (6388 for the ten specification rows, 1172 for the five bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; all 15 addresses were absent from
  `ledger/functions.csv` and from every earlier `fn_<addr>` body before this
  pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The six data `qr3` sites of ranks 9/10 and the five additional sites of the
  bucket rows are ported as `(f32)(u16)`; every other `psq` is a `qr0` spill.
- The module header was updated: cluster item 129, the cluster count
  (128 to 129), the function totals (3384 to 3399 public translations, 3313 to
  3328 `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave147` reports 15 rows for this lane
  (`translated/high=11`, `translated/medium=4`), 15 unique addresses and `OK`.
