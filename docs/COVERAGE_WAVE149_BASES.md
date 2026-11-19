# Coverage - Wave 149 (`game::bases`)

## How to read this file

This page documents the Wave 149 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL. Every address is a **static analysis address**
under the canonical synthetic base **`0x80600000`**; the Wii REL loader chooses
the real load address, so these numbers identify positions inside the REL image,
not runtime addresses. Every `# Original` block in the source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a vtable slot or a field role had
to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 16 | 6,816 | 14 | 2 |

All 16 rows are `translated`; none is `partial`. The module header now reports
3707 bodies (3691 + 16). The `0x806C` planner bucket drops from 74 to 66
untranslated addresses.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE148_BASES.md`** (6,236 bytes), each
  re-read against its export and the raw instruction stream:

  | cluster | body |
  |---|---|
  | `daLiftScales_c` rider tilt | `0x806DA520`: presence gate on `+0x5B4` and the `0x8042A078` mask, `0.020751953125` / `0.078125` rate arms, `-3.5..3.5` clamp, zero-crossing rejection, ceiling `+0x5A8`/tilt `+0x5AC`, cue `0x2B5`, four-slot rider scan into `+0x5BD` |
  | `daObjWendy_c` rail height | `0x8071E130`: cue `0x554` from the two camera probes, mode-3 arm with the `0x806601B0` ratio helper, `216.0` lift, `100 * t` step, `0.02` lerp, floor test against `[0x80429E00]+0x3C/0x44` |
  | broom ride state machine | `0x80788CB0`: `0x800F6390` reset caches, `0.9*roll + 0.1*rate` blend, 100/10-unit distance gates, `0x8075F9E0` spawn, dismount arm (`0x80785600`, `0x80789B80`, effect `0x17`), state-2 release through `0x80789E60` |
  | World-6 ramp mean | `0x80763290`: table `0x80815494`, stride `0x18`, four records, `1.0f` scale |
  | World-7 ramp mean | `0x80794C00`: table `0x8081B4EC`, stride `0x1C`, three records, `1.7f` scale |
  | `cobCastle` gate entry | `0x8075F270`: state word `+0x298` from `0x800FD820` and its sub-queries, publish through `0x801016B0`, messages `0x284`/`0x2A3`, tail `0x8075F570` |
  | `daCastleBossKey_c` key placement | `0x8066DDC0`: `96 / (1 + count)` spacing, per-character table `+0x3F0`, `0x80052080` height hand-off, `0x80051F30`/`0x80052170` catch state |
  | forest-manager course sync | `0x807727D0`: area-to-message map, per-slot cell copy `0x800BBC10`, RNG respawn ids, `0x292` retry loop, final `0x271` |

- **Eight `0x806C`-bucket bodies** (580 bytes):

  | cluster | bodies |
  |---|---|
  | constant getters | `0x806C17F0`, `0x806C1800`, `0x806C1810` (all `lis`/`addi` of `0x80377A04`) |
  | release wrappers | `0x806C84C0` (`0x802B9280` when `free > 0`), `0x806C8500`/`0x806C8560` (`0x8015F9F0(state+0x18, 0)` then the free), `0x806C2CD0` (`0x8015F800(state, 0)` then the free) |
  | four-character model loop | `0x806C12F0`: walks `+0x40C` (`+0x04` actor, `+0x08` next), ids `0..3` through `0x8005F900`, then model slots `+0x400` (two `f32` zeros and `1, 2, 0`) and `+0x434` (`0x1E, 0`) |

## Where the binary differed from the wave-148 prose

1. **`0x806DA520`'s presence gate is inverted relative to the spec's step 1-2.**
   The words at `0x806DA54C..0x806DA580` are `cmpwi cr0,r0,0` / `beq
   0x806DA5A4`, then the mask test ending in `or.` / `bne 0x806DA5A4`. The main
   integration body at `0x806DA5A4` therefore runs when `+0x5B4 == 0` **or**
   when a pad word matches; the rider drop `0x806D9F70` plus the `0x8082D9E8`
   message is the `+0x5B4 != 0 && no mask hit` arm. This matches the spec's own
   field map ("0 disables the mask test"), not its numbered steps.
2. **`0x806DA520`'s rate arms are `load == 1` and `load == 2`.** At
   `0x806DA5C0`/`0x806DA5C8` the `cmplwi` compares against `1` and `2`
   (`beq 0x806DA5D0` / `beq 0x806DA5DC`), so `1` subtracts and `2` adds
   `0.020751953125`, and the `else` decay of `0.078125` covers `load` values `0`
   and `3`. The spec's step 3 attached the subtraction to `load == 0`.
3. **`0x807727D0` keeps the cell write and the message inside the
   "message selected" arm.** `0x807728EC` is `cmpwi cr0,r30,0` / `beq
   0x80772A58`, i.e. the whole block from the `0x800BBBD0` write through the
   `0x800F2A00` publish is skipped for an area with no message. The spec listed
   those calls after the `if (bVar1)` block.

Two confirmations rather than corrections:

- The `0x80763290`/`0x80794C00` unrolled eight-term loop really does add each
  term to the accumulator one at a time (`fmuls f2,...` then `fadds f1,f1,f2`
  at `0x8076344C..0x807634A0`), so the shared `course_ramp_mean` helper's single
  loop is bit-identical and does not reorder the floating-point sum.
- `0x80788CB0`'s `(2 - v) | (v - 2)) >> 31` idiom at `0x80788EE4` is
  `v == 2 ? 0 : 1`, as the spec says.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8066DDC0` | `fn_8066ddc0` | 764 | high |
| `0x806C12F0` | `fn_806c12f0` | 192 | medium |
| `0x806C17F0` | `fn_806c17f0` | 12 | high |
| `0x806C1800` | `fn_806c1800` | 12 | high |
| `0x806C1810` | `fn_806c1810` | 12 | high |
| `0x806C2CD0` | `fn_806c2cd0` | 92 | high |
| `0x806C84C0` | `fn_806c84c0` | 64 | high |
| `0x806C8500` | `fn_806c8500` | 96 | high |
| `0x806C8560` | `fn_806c8560` | 100 | high |
| `0x806DA520` | `fn_806da520` | 804 | high |
| `0x8071E130` | `fn_8071e130` | 788 | high |
| `0x8075F270` | `fn_8075f270` | 768 | medium |
| `0x80763290` | `fn_80763290` | 780 | high |
| `0x807727D0` | `fn_807727d0` | 764 | high |
| `0x80788CB0` | `fn_80788cb0` | 788 | high |
| `0x80794C00` | `fn_80794c00` | 780 | high |

**Totals:** 16 bodies, 6,816 bytes. The address window runs from `0x8066DDC0` to
`0x80794C00`.

## Notes on inference

- `0x8075F270` is `medium`: `0x800FC0A0` and `0x800FBFB0` are entered with the
  previous call's return value in `r3` (Ghidra prints them with no arguments;
  the raw stream at `0x8075F36C`/`0x8075F378` shows `r3` still holding it), and
  `0x800FC370` is entered with whatever `r3` the publish call left behind, so it
  is written as `call0_ret` with an explicit `// unknown:` note.
- `0x806C12F0` is `medium`: the chain `+0x40C`/`+0x04`/`+0x08`, the character
  byte read and both vtable slots are literal, but the meaning of slots `+0x400`
  and `+0x434` and of the `0x1E` word is not proven.
- `0x8071E130`'s `0x806601B0` is the three-pointer ratio helper already wrapped
  by the existing `call_u32x3_f32`; its exact arms live in
  `work/ghidra/rel-d_basesNP/export/806601b0_FUN_806601b0.c` (260 bytes,
  untranslated).
- `0x806DA520` reuses the already-translated siblings [`fn_806da120`],
  [`fn_806d9f70`] and [`fn_806d9fc0`]; `0x80788CB0` and `0x8066DDC0` reuse
  [`fn_8065c210`].
- The float literals `0.020751953125` (`0x807CB8C0`), `0.078125`
  (`0x807CB8BC`), `-3.5`/`3.5` (`0x807CB8C4`/`0x807CB8C8`), `0.5`
  (`0x807CB88C`), `0.9`/`0.1`/`5.0`/`1.0` (`0x807CFA64`/`0x807CFA58`/
  `0x807CFA5C`/`0x807CFA50`), `100.0`/`10.0` (`0x80814C10`/`0x80814C24`),
  `8.0`/`216.0`/`0.02`/`800`/`100`/`300` (`0x80809634`, `0x8080963C`,
  `0x80809658`, `0x8080964C`, `0x80809650`, `0x80809640`), the four World-6
  records and the three World-7 records were read from
  `work/rels/elf/d_basesNP.elf` through
  `file_offset = (address - 0x80600000) + 0x1000`. The `0x8037xxxx`,
  `0x8042xxxx` and `0x8082xxxx` (BSS) words are `main.dol`/REL globals and are
  named by address.

## Deliberate skips

- The other 66 `0x806C`-bucket rows were not started. The 25 untranslated
  48-byte bodies in that bucket are `func_0x802DCBFC(...)` wrappers whose export
  duplicates one argument (`func_0x802dcbfc(param_2, param_2, param_1)`); the
  argument order cannot be proven from the export alone, so none was written.
- `0x806CB140` (208 bytes) is a GQR3-quantised offset body whose export carries
  `unaff_GQR0`/`unaff_GQR3` lane splits; it needs the wave-138 `GQR3 =
  0x00050005` argument proven at each `psq` site before a body is honest.
- `0x806CB210` (84 bytes) is only a two-call wrapper around the untranslated
  `0x806CB140`, so it was left open with it.
- No address in `ledger/functions.csv` was re-claimed; every equal-size twin
  taken here (`0x806C1800`, `0x806C1810`, `0x806C8560`) was checked against the
  raw words and is a real separate entry in the REL.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (16 rows, 6,816 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 16 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave149` reports `OK` for
  `wave149-bases.csv`: 16 rows, 16 unique addresses, no duplicates. (The only
  validator problems in that run name `wave149-enemies.csv`, which belongs to
  the parallel lane.)
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
- All 16 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`.
