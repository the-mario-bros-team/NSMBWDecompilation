# Coverage - Wave 187 lane E (`game::bases`)

## How to read this file

This page documents lane E of the Wave 187 pass over `src/game/bases.rs`, the
semantic slice of the `d_basesNP` REL. Every address is a **static analysis
address** under the canonical synthetic base **`0x80600000`**; the Wii REL
loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the line `Synthetic base: 0x80600000; this is not a runtime
address.`

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or an arithmetic
width had to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 1,548 | 9 | 1 |

All ten rows are `translated`; none is `partial` and none was deferred. The
lane covers the complete 136-to-184-byte band of the wave-187 assignment; the
bodies live in `work/wave187/laneE_block.rs` and are appended to
`src/game/bases.rs` by the coordinator, not by this lane.

## What this lane covers

| cluster | bodies |
|---|---|
| `Wm_jr_bomb` fuse re-arm | `0x806583E0`: radius chain from `fn_80657fe0`, `+0x5F6` step, `+0xE8` velocity from the `+0x348` table byte |
| lerp helpers | `0x8065E4E0` (window inverse-lerp), `0x80660E00` (snap a target onto `centre +/- radius`) |
| actor name matchers | `0x806628D0`, `0x80665320`, `0x80666940`, `0x806676C0`, `0x8066F5E0` — five bit-identical `strchr`/`strcmp` compare bodies over vtable slot `+0x20` |
| placement cursor | `0x80662F40` (republishes four approach coordinates through `0x80052080`), `0x80663DF0` (two lerp pairs, three matrix setters, post-selection slide) |

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x806583E0` | `fn_806583e0` | 180 | high |
| `0x8065E4E0` | `fn_8065e4e0` | 180 | high |
| `0x80660E00` | `fn_80660e00` | 148 | medium |
| `0x806628D0` | `fn_806628d0` | 136 | high |
| `0x80662F40` | `fn_80662f40` | 184 | high |
| `0x80663DF0` | `fn_80663df0` | 176 | high |
| `0x80665320` | `fn_80665320` | 136 | high |
| `0x80666940` | `fn_80666940` | 136 | high |
| `0x806676C0` | `fn_806676c0` | 136 | high |
| `0x8066F5E0` | `fn_8066f5e0` | 136 | high |

**Totals:** 10 bodies, 1,548 bytes. The address window runs from `0x806583E0`
to `0x8066F5E0`.

## Deferred

None. Every assigned address was decoded from the raw words and is present in
`work/wave187/laneE_block.rs` and `ledger/agents/wave187-lanee.csv`.

## Where the binary differed from the prose

1. **Five "different" functions are one body.** `0x806628D0`, `0x80665320`,
   `0x80666940`, `0x806676C0` and `0x8066F5E0` were compared word for word: all
   34 instruction words match, and only the three branch displacements differ
   (`0x4BC7F34D` vs `0x4BC7C8FD`, ...), each landing on the same relative
   target. Each still gets its own body and ledger row. The body is the same
   shape as the already-translated `fn_807449f0`, so the translation matches it.
2. **`0x80662F40`'s `xoris`/`0x43300000` block is dead.** The export opens with
   `lwz r4,[0x80429CA0]`, `xoris r3,r4,0x8000`, `stw r3,0x14(r1)` and
   `stw 0x43300000,0x10(r1)`, but nothing ever loads that double: there is no
   `lfd` from `r1+0x10` on any reachable path. Ghidra marks the same thing
   (`Removing unreachable block (ram,0x80662fdc)`), so the count is used as a
   plain `int` and the loop carries no conversion.
3. **`0x80662F40`'s frame calls are not semantic.** `bl 0x802DCD74` and
   `bl 0x802DCDC0` are the `_savegpr`/`_restgpr` helpers from the brief, and
   `psq_l f31,...,0,0` / `stfd f31` are the `qr0` prologue save only. None is
   emitted.
4. **`0x80663DF0`'s first argument is a float in `f1`.** The export spells the
   signature `FUN_80663df0(undefined8 param_1, int param_2)` and the raw words
   start with `fmr f2,f1` before loading `0x807C8BF0 = 1.0` into `f1`; the
   callee `0x8015F470` is called as `(f32, f32, u32)` by the already-translated
   neighbours. The body therefore takes `value: f32` and forwards it as the
   second argument, matching `call_f32x2_u32(0x8015_F470, 1.0, value, &+0x484)`.
5. **`0x8065E4E0` returns `f32`, not `double`.** Every exit loads `f1` with an
   `lfs` (`0.0`/`1.0`) or with an `fdivs` of two `lfs` values; the export's
   `double` is the unprototyped-callee guess. Same for `0x80660E00`.
6. **`0x806583E0`'s `+0xE8` store is a signed-byte scale, not a raw bit copy.**
   The `fsubs f2,f2,f4` at `0x8065845C` subtracts the double
   `0x807C89A0 = 0x43300000_80000000` (`2^52 + 2^31`) from
   `CONCAT44(0x43300000, table[+0x348] ^ 0x80000000)`, which is exactly the
   signed-integer-to-double ladder already documented for `fn_8065e460` and
   `fn_806b5a80`; the result is `table[+0x348] as f32 * 0.7`.
7. **`0x80660E00` mixes widths.** The magnitude test and the negation are
   `fcmpo`/`fneg` on `f1` as a double, while both edges are computed with
   `fadds`/`fsubs`. The body follows the export: `f64` parameter, `f64`
   arithmetic, `f32` edge values (Ghidra's `(float)(dVar1 + param_1)`).

Confirmations rather than corrections: `0x806583E0`'s whole step expression is
`2.0 * (32768.0 * (0.7 / (pi * (2.0 * radius)))) * 1.5` with the constants at
`0x807C8978`/`0x807C89B8`/`0x807C8930`/`0x807C89B4`/`0x807C8938`; `+0x5F6` is
written through `fctiwz` + `sth`, the same ladder as `fn_80658280`'s fuse word;
and `0x8065E4E0`'s two `cror` folds (`0x4C401382` = `cr0.EQ := LT | EQ`,
`0x4C411382` = `cr0.EQ := GT | EQ`) make both window ends inclusive exactly as
Ghidra prints them.

## Notes on inference

- `0x80660E00` is the only `medium` row: the guards, both edges and the chosen
  branch are literal, but the source-level parameter type cannot be settled
  from the words (a double negation and compare, single-precision edges). The
  translation follows the export's own rounding instead of guessing a different
  arithmetic width. `0x807C8AE8` is `0.0`, so the absolute value is the
  documented `fcmpo` + `fneg` idiom.
- The nine `high` rows have every guard, store, constant and call argument
  visible in the export and the raw words.
- `0x80052080` (called by `0x80662F40`) is not defined in this REL; the body is
  called address-only with the argument shape fixed at the call site, the same
  treatment as `fn_8066fa10`.
- `0x80663DF0`'s `+0x3E0`/`+0x474` field roles (`state`, chosen coordinate) are
  taken from the neighbouring `fn_80663000`/`fn_80663290`, which read the same
  placement-cursor actor.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (10 rows, 1,548 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every body was decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000`, and constants were read out
  of `work/rels/elf/d_basesNP.elf`: `0x807C8978 = 2.0`, `0x807C89B4 = pi`,
  `0x807C89B8 = 32768.0`, `0x807C8930 = 0.7`, `0x807C8938 = 1.5`,
  `0x807C89A0 = 176.0` (high word of `2^52 + 2^31`), `0x807C8AE8 = 0.0`,
  `0x807C8AF4 = 1.0`, `0x807C8BD8 = 2.0`, `0x807C8BF0 = 1.0`,
  `0x807C8C08 = 32.0`.
- The block was compiled against the real helper signatures in a scratch
  harness (`abi_call1..3`, `call_f32x2_u32`, `vmethod0`, `platform::*`):
  `rustc --edition 2021 --crate-type lib` reports no error or warning.
- `tools/check-lane-csv.mjs` only validates bodies that are already merged into
  `src/game/bases.rs`, so it must be re-run by the coordinator after
  `work/wave187/merge.cjs`. The CSV itself has the canonical 7-column header,
  7 fields on every row, unique lowercase plain-hex addresses and existing
  evidence files.
