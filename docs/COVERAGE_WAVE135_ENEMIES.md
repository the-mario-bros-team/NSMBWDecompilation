# Coverage - Wave 135 (`game::enemies`)

## How to read this file

This page records the Wave 135 `game::enemies` lane: 29 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 117. Every row of the
machine-readable mirror (`ledger/agents/wave135-enemies.csv`) is one real
`pub unsafe fn fn_<addr>` body, and the `evidence` column names the Ghidra C
export each body was reconstructed from. Sizes are the byte counts from
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Floating-point
constants are cited by address and read with `platform::read_u32` +
`f32::from_bits`, never transcribed as invented values. Raw words were decoded
from `work/rels/elf/d_enemiesNP.elf` at the mapping in
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`).

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 29 | 3,016 | 12 | 17 |

The lane has two halves:

- **12 bodies** are the twelve rows of `docs/SPEC_WAVE134_ENEMIES.md` (ranks
  75-86 of the wave-127 class-B list): eleven marked **ready** and one
  **conditional** (`0x80A369B0`). They were written from the specification's
  instruction decode and then re-checked against the export and the raw words.
- **17 bodies** are still-open entries of the `0x80A90000` planner bucket
  (30 untranslated before this pass, 13 after). They are one coherent cluster:
  thirteen 48-byte `0x802DCBFC` deleting-destructor trampolines that differ only
  in their compile-time `r12` this-adjustor (`+0x18` or `+0x24`), the three short
  actor rows `0x80A91D10`/`0x80A9E0D0`/`0x80A9F770` and the `0x880`-byte actor
  constructor `0x80A9E5B0`.

## What this pass covers

- **The specification bodies (`0x80A2F910..0x80B27980`).** The four-frame
  `E9A`/`E9E`/`E9F` flag with the `EA9`→`EAA` countdown and the SE at `a+0x61C`;
  the `sanbo` model binder (both node names and the `(this+4) & 1` selector); the
  `+0xAC += 4.0 * base[+0x398]` step with its `0.1` SE threshold; the `+0x548`
  sub-object reset plus effect `0x2D9`; the four-player class probe; the `1024.0`
  spin ramp with the `+0x102` `s16` step; the per-player contact record over the
  three delta arrays; the six sub-object vtable resets; the 300-frame retry arm
  with SE `0x1E4`; the side-indexed heading step and clamp; the `:`-suffix
  node-name predicate; and the `block_pata` direction step.
- **The trampoline cluster (13 bodies).** `0x80A90AE0`, `0x80A90B10`,
  `0x80A915E0`, `0x80A91610`, `0x80A93AB0`, `0x80A93AE0`, `0x80A95D00`,
  `0x80A95D30`, `0x80A981F0`, `0x80A98220`, `0x80A9A7A0`, `0x80A9A7D0` and
  `0x80A9D960`. Each is `stwu`/`mflr`, `mr r5,r3`, `mr r3,r4`,
  `addi r12,r5,{0x18|0x24}`, `bl 0x802DCBFC`, `nop`, epilogue.
- **The three short actor rows (`0x80A91D10`, `0x80A9E0D0`, `0x80A9F770`).** The
  `Wm_mr_cloud_on` effect spawn at `this+0x6EC`; the two state stores with the
  `0x800646E0(this, 3)` tail; and the `+0x872` gate with SE `0x1D2`.
- **The `0x880`-byte constructor (`0x80A9E5B0`).** Allocation, base constructor,
  class vtable `0x80B52918`, record descriptor `0x80B52B98`, the
  `+0x52C`/`+0x54C` holders, the six `0x40`-byte `0x80169CD0` records and the two
  six-entry `0xC`-byte `0x80A2FC50`/`0x80A00200` record arrays.

### Corrections to earlier waves

- **`0x80A91D10` is not unrecoverable.** Wave 118's section header and the
  `fn_80a91da0` doc block called its 32 bytes `halt_baddata` control flow. The
  raw words are a clean five-argument tail call:
  `addi r5,r5,0x6EC` + `b 0x8016C890`, i.e.
  `0x8016C890(0x80B508E8, 0, this + 0x6EC, 0, 0)` where `0x80B508E8` is the
  `Wm_mr_cloud_on` name. Ghidra stopped only because the `b` leaves the REL. The
  stale comments were corrected in place, and the body is translated here.
- **`0x80A9E0D0` is not truncated.** Its 24 bytes are two stores
  (`+0x175 = 9`, `+0x178 = 0x8F`) plus a `b 0x800646E0` with `r4 = 3`; the export
  stopped at the out-of-REL tail, not at bad data.

### Confirmations worth recording

- **The `+0xE9F` byte is `4`.** `0x80A7979C` is `fsubs f1,f1,f2`: both operands
  are loaded as `f64` and then rounded to single, so `2^52 + 4` and `2^52` both
  become `2^52`, the difference is `0.0`, and `4.0 + 0.0` truncates to `4`. A
  port that subtracts in double would store `8` and be wrong.
- **`0x80ADE3A0`'s predicate is `!(delta >= 0.0)`.** `0x80ADE3F0` is
  `cror 2,1,2` (`CR0.EQ := GT | EQ`), so the `bne` stores `1` when the step
  decreased `+0xAC` and also on the unordered (NaN) case.
- **The trampoline adjustors are literal.** `addi r12,r5,0x18` and
  `addi r12,r5,0x24` are the only difference between the thirteen bodies; the
  `bl` displacement differs because each thunk sits at a different distance from
  `0x802DCBFC`.
- **`0x80A7E630` is an 8-byte thunk.** Its two words are
  `addi r3,r3,0x5FC` and `b 0x800944C0`, so `fn_80a7ed20`'s final call is
  faithfully a call to that thunk address.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A2F910` | `fn_80a2f910` | 136 | high |
| `0x80A369B0` | `fn_80a369b0` | 152 | medium |
| `0x80A3F8F0` | `fn_80a3f8f0` | 176 | high |
| `0x80A46BF0` | `fn_80a46bf0` | 172 | high |
| `0x80A79750` | `fn_80a79750` | 188 | high |
| `0x80A7ED20` | `fn_80a7ed20` | 176 | high |
| `0x80A90AE0` | `fn_80a90ae0` | 48 | medium |
| `0x80A90B10` | `fn_80a90b10` | 48 | medium |
| `0x80A915E0` | `fn_80a915e0` | 48 | medium |
| `0x80A91610` | `fn_80a91610` | 48 | medium |
| `0x80A91D10` | `fn_80a91d10` | 32 | high |
| `0x80A93AB0` | `fn_80a93ab0` | 48 | medium |
| `0x80A93AE0` | `fn_80a93ae0` | 48 | medium |
| `0x80A95D00` | `fn_80a95d00` | 48 | medium |
| `0x80A95D30` | `fn_80a95d30` | 48 | medium |
| `0x80A981F0` | `fn_80a981f0` | 48 | medium |
| `0x80A98220` | `fn_80a98220` | 48 | medium |
| `0x80A9A7A0` | `fn_80a9a7a0` | 48 | medium |
| `0x80A9A7D0` | `fn_80a9a7d0` | 48 | medium |
| `0x80A9D960` | `fn_80a9d960` | 48 | medium |
| `0x80A9E0D0` | `fn_80a9e0d0` | 24 | medium |
| `0x80A9E5B0` | `fn_80a9e5b0` | 252 | medium |
| `0x80A9F770` | `fn_80a9f770` | 104 | medium |
| `0x80ADE3A0` | `fn_80ade3a0` | 136 | high |
| `0x80AFF0D0` | `fn_80aff0d0` | 184 | high |
| `0x80B0CFD0` | `fn_80b0cfd0` | 160 | high |
| `0x80B0DB90` | `fn_80b0db90` | 156 | high |
| `0x80B19FA0` | `fn_80b19fa0` | 176 | high |
| `0x80B27980` | `fn_80b27980` | 168 | high |

### Why seventeen rows are `medium` (thirteen of them `partial`)

- **The thirteen trampolines** are `partial`: the visible call
  `0x802DCBFC(param_2, param_2, param_1)` and the adjustor are exact, but the
  hidden `r12` this-adjust pointer is outside the Rust ABI and cannot be modelled
  by `platform::call3`. This is the same limitation recorded for the wave-122
  `0x80AEB9A0`/`0x80AEB9D0` rows.
- **`0x80A369B0`** reads `0x8042C1A8` (the per-frame step) at run time because
  that `main.dol` table is not published anywhere in this tree; only
  `0x8042C1A4`'s `{+0x2000, -0x2000}` pair is known.
- **`0x80A9E0D0`** and **`0x80A9E5B0`** name callees whose roles are inferred from
  the record layouts (`0x800646E0`, `0x80094E80`), not from a decoded body.
- **`0x80A9F770`**: the 28-byte callee `0x80A9F750` also reads `r4`, which this
  stream never sets; only its `r3` argument is reproduced.

## Deliberate skips

- **`0x80A9F750` (28 bytes).** Its body is a guard on `(s8)other[+0x1080] < 7`
  and a tail call to `0x800CE8C0` with `r3 = *(u32*)0x80429FB0`, but `r4` (the
  `other` it tests) is never set by its only in-REL caller `0x80A9F770`, so no
  honest argument can be named. `0x80A9F770` calls it by address instead.
- **`0x80A91D30` (56 bytes).** Its `+0x708 < 0x5A` guard is clear, but the
  `+0x5C0` slot `+0xA0` dispatch is preceded by an unrecoverable jump table in
  the export; it was already skipped by wave 118.
- **`0x80A99360` (228 bytes).** Consumes `GQR0` paired-single data and three
  `main.dol` floats with a sign-table multiply; not byte-exact yet.
- **`0x80A9B3C0` (1,196 bytes).** `GQR0` plus a 32-word table copy; left for a
  later pass.
- **`0x80A933E0` (36 bytes).** Two `rlwinm` words write `r0`/`r4` that are then
  overwritten or unused, and the tail callee `0x80064610` has no established
  role; the intended argument construction cannot be stated without guessing.
- **No address already in `ledger/functions.csv` was touched**, and no other
  lane's file, CSV or doc was modified.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 29 sizes sum to 3,016
  bytes (1,980 for the twelve specification rows and 1,036 for the seventeen
  bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice and none of the 29 was in
  `ledger/functions.csv` before this pass (checked against the address column).
- The module header was updated: cluster item 117, the cluster count (116 to
  117), the function totals (3,120 to 3,149 public translations, 3,049 to 3,078
  `fn_<addr>` bodies), the "start here" list, the records list, and the wave-118
  skip list (which no longer claims `0x80A91D10`/`0x80A9E0D0` are unrecoverable).
- `cargo check --message-format=short` emits no error and no warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave135` reports 29 rows, 29 unique addresses
  and `OK`.
