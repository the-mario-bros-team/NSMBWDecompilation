# Coverage - Wave 137 (`game::enemies`)

## How to read this file

This page records the Wave 137 `game::enemies` lane: 25 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 119. Every row of the
machine-readable mirror (`ledger/agents/wave137-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Floating-point
constants are cited by address and read at run time with `platform::read_u32` /
`read_f32`, never transcribed as invented values. Raw words were decoded from
`work/rels/elf/d_enemiesNP.elf` at the mapping of `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000`).

The lane has two halves:

- **11 bodies** are the ready rows of `docs/SPEC_WAVE136_ENEMIES.md` (ranks 1-3
  and 5-12 of that page; rank 11, `0x80AFEC80`, was already landed by the
  wave-136 implementation lane and is **not** re-claimed here). The page's only
  conditional row, `0x80A6EFB0`, is included with an explicit ABI caveat.
- **14 bodies** are still-open rows of the `0x80B20000` planner bucket
  (26 untranslated before this pass, 12 after). They continue the wave-129
  controller actor (`0x80B22B40..0x80B29DD0`) through its `0x80B27CE0..0x80B2A070`
  tail.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 25 | 4,708 | 12 | 13 |

## What this pass covers

- **The class-list walk `0x80A0B8A0`.** `0x80162D50(0x83, cursor)`, then a
  `+0x754`/`+0x758` match test, then recursion with `r4 = found`. The recursion
  is what advances the cursor.
- **The two teardown pairs `0x80A2F670`/`0x80A2F6D0`.** `0x8015F800(this, 0)`
  plus `0x802B9280(this)` for a positive `free_count`; the 92-byte body carries
  the second `beq` at `0x80A2F6F4`, which re-tests the first compare and is
  never taken.
- **The altitude machine `0x80A6EFB0`.** `+0xB0 - +0x850` banded against `56.0`
  and `120.0`, writing `0`/`1`/`2` to `+0x860` and returning `1`/`2`/`0`.
- **The angle-to-speed pair `0x80AB1890`.** The `2^52` integer conversion of the
  `s16` at `+0x102` feeds `+0x164 = v/4096` and `+0x7DC = -3v/4096`.
- **The pad resets `0x80AB2620`, `0x80B0D070`, `0x80B0D0E0`.** Two
  `0x80166340(record, 3, 0)` stores guarded by `+0x988 == 1`, then
  `0x80AB1780(1.0f, this, 1)`; and the two `+0x6xx` initialisers whose four
  stores and two `0x80166F40` calls differ only in the literal `1` versus `3`.
- **The ten-argument reset `0x80AD6CA0`.** `0x800DB180(0,0,0,0,-22, this+0x4CD4,
  this, 0, 1, 0)` followed by `0x800DAE80(this+0x4CD4)`. All five float
  arguments are written by `fmr` copies of one `lfs`.
- **The `+0x18` destructors `0x80AED1B0`/`0x80AED210`.** `addic.` computes and
  null-tests the sub-object pointer, `0x8015F9F0` releases it and `0x802B9280`
  frees the actor for a signed positive count; the 100-byte copy repeats the
  null test.
- **The grid emitter `0x80B27CE0`.** Two nested loops over the row/column tables
  at `0x80B32C48`/`0x80B32C50` (indexed by `+0x8AC`), one `0x80077860` cell emit
  per iteration with the tile word read from `0x80B32C58 + 2*variation`.
- **The slot relays `0x80B28080`, `0x80B281C0`, `0x80B283B0`.** The seventeen
  `+0x6E8` slots fed to `0x80B27ED0`; the four-player `+0x8C4` mask relayed
  through `0x800BC070`/`0x80133950`/`0x80133860`; and the accumulator reset that
  emits effect `0x2F2` or `0x2F1`.
- **The full-turn relay `0x80B28480`.** `+/-512` from the two delta tables,
  accumulated through `0x802DE514` (integer `abs`), the `+0x1035` hit byte per
  active player, the `0x2F3` effect and the `+0xD4` class slot called with
  `0x80B79930`.
- **The `0x80B29450` break/attach step.** The `0x800212C0` mode gate, the
  `9`/`0xE`/`10` state arms, the `0x3C` hit request, the three
  `Wm_ob_cmnbox*` effects with the `0.5`/`0.7` scale blocks, effect `0x27F` and
  the `0x80162510` shutdown.
- **The `0x80B297A0`/`0x80B29910`/`0x80B299A0` per-frame steps, the
  `0x80B28F60` release wrapper and the `0x80B29DA0` destructor forwarder.**
- **The three model/matrix helpers `0x80B29390` (`box_iron`), `0x80B29EE0`
  (`arrow`) and `0x80B2A070` (`+0x7C` matrix writer).**

### Confirmations and corrections worth recording

- **`0x80AB1890`'s arithmetic is single precision.** The words are `fsubs`
  (`0xEC602028`, `0xEC212028`) and `fmuls` (`0xEC4200F2`, `0xEC000072`), i.e.
  primary opcode 59, not the double-precision opcode 63 that
  `docs/SPEC_WAVE136_ENEMIES.md` names. The result is unchanged for every `s16`
  (`v`, `v/4096` and `3v/4096` are exactly representable), but the body performs
  the products in `f32` to stay literal.
- **`0x80A6EFB0`'s middle band has no return value.** The shipped body does
  `li r3,2; b 0x800B2EE0`, so the value seen by the caller is the `void` setter's
  leftover `r3`. The port emits the call and then `return 2`, the first reading
  offered by the specification; the row is `medium` for that one datum.
- **`0x80B27CE0` takes a second argument.** Ghidra prints `FUN_80b27ce0(void)`,
  but the raw prologue ends with `rlwinm r22,r4,1,0,30` before any write to
  `r4`, and both callers pass `0`/`1`. The port models `variation`.
- **`0x80B27CE0`'s `x` accumulator does not reset per row.** Ghidra re-assigns
  `uVar6` inside the outer loop; the raw stream keeps incrementing `r31` by
  `0x10` for every cell and only decrements `y` per row (`addi r30,r30,-0x10`).
  The port follows the stream, not the decompiler's loop shape.
- **`0x80B28480` and `0x80B29450` are not receiver-less.** Ghidra's
  `func_0x802dcd70()`/`func_0x802dcd74()` are the `_savegpr_N` helpers described
  in `docs/REL_ELF_MAPPING.md`; the `mr r31,r3` / `mr r29,r3` right after the
  save proves the incoming `r3` is `this`, which is what the port uses.
- **`0x80B29DA0`'s adjustor is `+0x24`.** `addi r12,r5,0x24` with `r5` holding
  the first argument; the body is the same shape as the wave-135/136 forwarders
  and is `partial` for the same hidden-`r12` reason.
- **`0x80B281C0`'s variant-1 record is three floats 0x10 apart.** `+0x744`,
  `+0x754` and `+0x764` for slot `i` at `this + 4*i`; the `0x800BC070` output is
  then offset by `+0x738..+0x740` before it reaches `0x80133950`.
- **`0x80B29450`'s three effect arguments are in-REL strings.** `r30` is
  `0x80B6DA38`, so the `+0x128`/`+0x13C`/`+0x150` displacements resolve to
  `0x80B6DB60`/`0x80B6DB74`/`0x80B6DB88` = `"Wm_ob_cmnboxsmoke"`,
  `"Wm_ob_cmnboxgrain"` and `"Wm_ob_cmnboxpiece"`; they are the first argument
  of the three `0x8016C890` calls, not object pointers.
- **`0x80B29450`'s break-effect position does not include the `+0x720` lift.**
  The lift is added to the `0x28` copy used by the `0x3C` hit request only; the
  `0x34` copy used by the emitters and `0x80092830` keeps the raw `+0xB0`, with
  the variant-0 `16.0` drop applied on top.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A0B8A0` | `fn_80a0b8a0` | 108 | translated | high |
| `0x80A2F670` | `fn_80a2f670` | 88 | translated | high |
| `0x80A2F6D0` | `fn_80a2f6d0` | 92 | translated | high |
| `0x80A6EFB0` | `fn_80a6efb0` | 100 | translated | medium |
| `0x80AB1890` | `fn_80ab1890` | 104 | translated | high |
| `0x80AB2620` | `fn_80ab2620` | 112 | translated | high |
| `0x80AD6CA0` | `fn_80ad6ca0` | 100 | translated | medium |
| `0x80AED1B0` | `fn_80aed1b0` | 96 | translated | high |
| `0x80AED210` | `fn_80aed210` | 100 | translated | high |
| `0x80B0D070` | `fn_80b0d070` | 100 | translated | high |
| `0x80B0D0E0` | `fn_80b0d0e0` | 100 | translated | high |
| `0x80B27CE0` | `fn_80b27ce0` | 220 | translated | medium |
| `0x80B28080` | `fn_80b28080` | 120 | translated | medium |
| `0x80B281C0` | `fn_80b281c0` | 332 | translated | medium |
| `0x80B283B0` | `fn_80b283b0` | 180 | translated | medium |
| `0x80B28480` | `fn_80b28480` | 656 | translated | medium |
| `0x80B28F60` | `fn_80b28f60` | 40 | translated | medium |
| `0x80B29390` | `fn_80b29390` | 184 | translated | high |
| `0x80B29450` | `fn_80b29450` | 844 | translated | medium |
| `0x80B297A0` | `fn_80b297a0` | 360 | translated | medium |
| `0x80B29910` | `fn_80b29910` | 128 | translated | medium |
| `0x80B299A0` | `fn_80b299a0` | 104 | translated | medium |
| `0x80B29DA0` | `fn_80b29da0` | 48 | partial | medium |
| `0x80B29EE0` | `fn_80b29ee0` | 172 | translated | high |
| `0x80B2A070` | `fn_80b2a070` | 220 | translated | high |

### Why one row is `partial`

- **`0x80B29DA0`.** The visible call `0x802DCBFC(second, second, first)` and the
  `addi r12,r5,0x24` adjustor are exact, but the hidden `r12` this-adjust
  pointer is outside the Rust ABI and cannot be modelled by `platform::call3`.
  Same limitation as the thirteen wave-135 and four wave-136 forwarders.

### Why twelve `translated` rows are `medium`

- **`0x80A6EFB0`** - the middle band's return value is a `void` tail call's
  leftover `r3`.
- **`0x80AD6CA0`** - both callees (`0x800DB180`, `0x800DAE80`) are `main.dol`
  and unexported; their contracts come from the wave-130/133 notes.
- **`0x80B27CE0`** - the grid loop and tile read are literal, but the
  `+0x8AC` dimension-table roles and `0x80077860`'s semantics are inferred.
- **`0x80B28080`** - `0x80B27ED0` is still untranslated, so its slot contract is
  inferred from its callers.
- **`0x80B281C0`** - `0x800BC070` and the two player setters are inferred from
  their argument shape.
- **`0x80B28480`** - the accumulator path and the reset branch are literal, but
  `0x800212C0`'s mode values, the delta-table roles and the `0x2F3` effect id
  are inferred from this cluster.
- **`0x80B283B0`**, **`0x80B29450`**, **`0x80B297A0`**, **`0x80B29910`**,
  **`0x80B299A0`** - the effect ids and the `main.dol` helper roles are inferred
  from the surrounding block; every offset and store is literal.
- **`0x80B28F60`** - `0x8007F950`'s unlink contract is read from that callee's
  own export, not from a translated caller in this file.

## Deliberate skips

The twelve still-open rows of the `0x80B20000` bucket are left for a later pass:

- **GQR/`ldexpf` quantiser rows:** `0x80B20010` (152), `0x80B22750` (128),
  `0x80B23480` (180), `0x80B23890` (156), `0x80B27DC0` (272), `0x80B27ED0`
  (432), `0x80B28100` (188), `0x80B28F90` (356) and `0x80B2A360` (324). Every
  one reads `unaff_GQR0`/`unaff_GQR3` and emits `ldexpf` fiction; per
  `docs/REL_ELF_MAPPING.md` they stay class C until GQR3 is pinned. `0x80B27DC0`
  and `0x80B27ED0` are still called as opaque callees by two bodies above.
- **`0x80B250F0` (56 bytes).** The export ends in `halt_baddata` after the
  `0x8042A200 & 1` guard; the control flow past the truncation is not
  recoverable.
- **`0x80B25280`/`0x80B253D0` (328 bytes each).** GQR prologue plus an incoming
  `f31`, an indirect `+0x6C` vtable call and a stack-window lookup; the
  float-quantiser path cannot be reproduced honestly.

No other `d_enemiesNP` address was touched, and `0x80AFEC80` (spec rank 11) was
skipped because it was already a ledger row from wave 136.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 25 sizes sum to 4,708
  bytes (1,100 for the eleven specification rows and 3,608 for the fourteen
  bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 25 was in
  `ledger/functions.csv` before this pass (checked against the address column,
  including `0x80AFEC80`).
- The module header was updated: cluster item 119, the cluster count (118 to
  119), the function totals (3,175 to 3,200 public translations, 3,104 to 3,129
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave137` reports 25 rows for this lane
  (`translated/high=12`, `translated/medium=12`, `partial/medium=1`), 52 unique
  addresses across the wave and `OK`.
