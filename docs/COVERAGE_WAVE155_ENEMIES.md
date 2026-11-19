# Coverage - Wave 155 (`game::enemies`, REL `d_enemiesNP`)

## How to read this file

This page records the Wave 155 `game::enemies` lane: 17 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 137 and mirrored
one row per body in `ledger/agents/wave155-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Raw words
were decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` program
headers as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment; the
`0x80B2A5xx`/`0x80B32DC0` data segments were mapped from their own headers).
Every body was followed from its entry to its terminal `blr` or tail `b`. Byte
counts are the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

One decoding fact is repeated in every doc block: a relative branch whose target
lies in `main.dol` decodes `0x0100_0000` too high when the image is read at the
synthetic base (the shipped REL was linked at a base `0x0100_0000` below
`0x80A00000`). The bodies therefore cite the plain `main.dol` address
(`0x80078470`, `0x800E41C0`, `0x80064610`, `0x801680E0`, `0x802DCBFC`,
`0x80168180`, `0x8015F4F0`), which is also the value the Ghidra exports and the
existing translations use.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 17 | 1948 | 8 | 8 | 1 |

Status split: 17 `translated`, no `partial` and no `stub`. Covered address
range: `0x80A03C30..0x80B14D80`.

## What this pass covers

- **All twelve rows of `docs/SPEC_WAVE154_ENEMIES.md`.**
  - The five "ready" rows:
    - `0x80A03C30` - `daEnBuku_c` 24-frame stage effect: the `+0x69C`
      decrement/reload pair, the `32.0`/`-32.0` side offset at `0x80B2A9A8`
      against the `+0x688`/`+0x68C` anchor, and the six-register tail request
      `0x80078470(x, y, *(u32*)0x80429DD0, 3, 1, 24)`.
    - `0x80AF3850` - burst-cannon steer snap: the `fn_80af38b0` angle, the
      `+-400` bias, the `0x51EB851F` / `srawi 8` truncating divide by `800` and
      the final `extsh`, i.e. a round-to-nearest-multiple-of-`800` with ties
      away from zero.
    - `0x80AF0370` - `daEnRedRing_c` rumble: the `(u8)(+0x355 - 1)` port, the
      mask pointer read through `0x8042A078`, and the seven-register tail
      request `0x800E41C0(mask, port, 60, 1, 0, 1, 0)`.
    - `0x80A933E0` - `block_cloud` splash request: the two packed nibble fields
      and the tail `0x80064610(0x54, resource, out, 0, 0)`.
    - `0x80B14D80` - thrown-object request: `(mode & 0x1113) | 0x8000` and the
      tail `0x80064610(0x39, resource, out, 0, 0)`.
  - The seven "conditional" rows, all landed:
    - `0x80ACDB00` - the `daEnNetNokoLR_c` `net_walk2` binder. The `frame`
      parameter is `f32` (the stream moves `f31` with `fmr` and never widens
      it), and the existing caller `fn_80acde00` was corrected from `call1`
      (which dropped `f1`) to `call1_ret_f32` + an `f32` ABI call, so the live
      `0x80168080` frame is forwarded exactly as the raw stream does. The
      spawn-site caller at `0x80ACDA70` already loads `0.0f` into `f1`
      immediately before its call, so it needs no change.
    - `0x80A48940` - the `daEnCrasher_c` `+0x674` accumulator. Written with an
      `f32` delta (`fadds`/`stfs`), and its only caller `fn_80a49490` was
      corrected from `call2_f64` to the same `f32` ABI so the `0.125f`
      `lfs` value is not promoted to a double.
    - `0x80A25F00`, `0x80A2F9D0`, `0x80A2FA00`, `0x80A43EE0`, `0x80A43F10` -
      the five hidden-`r12` destructor thunks, `+0x24`/`+0x18`/`+0x24`/
      `+0x18`/`+0x24`. They keep the file's established `partial` convention
      (`call3(0x802DCBFC, param_2, param_2, param_1)`) and each doc names the
      vtable slot that installs it. The five vtable words were re-read from the
      image and match the specification: `0x80B3ABE8` -> `0x80A25F00`,
      `0x80B3C74C` -> `0x80A2F9D0`, `0x80B3C750` -> `0x80A2FA00`,
      `0x80B406E8` -> `0x80A43EE0`, `0x80B406EC` -> `0x80A43F10`.
- **The three rows below the specification's cut.**
  - `0x80A07C80` - the three staged arguments `(this+0x5B4, 0, this+0x710)` and
    the tail `b 0x80168180`. The callee has no export row, so the row is `low`
    confidence: only the instruction sequence is certain.
  - `0x80AF3840` - the 16-byte guide-angle step trampoline
    `0x8015F4F0(this+0x7A4, (s16)r4, 0x400)`.
  - `0x80A337B0` - the 4-byte alias `b 0x80A337C0`, now a one-line forwarder to
    the translated [`fn_80a337c0`]; the stale "needs no body of its own" note
    beside that body was corrected.
- **Two bodies that no earlier lane claimed.**
  - `0x80A85260` - the `switch`-model `0xDFA` four-state handler: the `8.0`
    fall arm against `*(u32*)0x80429E00 + 0x3C`, the `0x80072BC0` poll whose
    `0x8000` bit or zero value means "alive" (the raw is `cntlzw`/`rlwinm`), the
    optional `0x8006EF40` veto, the `+0xBD4`/`+0xBE0 = 0x800` entry stores and
    the `0x80A86070`/`0x80A865E0`/`0x80A85D60` frame chain.
  - `0x80A0F610` - the `0x288`-byte chain-segment builder, the wider sibling of
    the already-translated [`fn_80a707e0`] (`0x278`-byte segments).
- **Three of the four remaining rows of the `0x80A20000` planner bucket**
  (`0x80A25F00`, `0x80A2F9D0`, `0x80A2FA00`). The fourth, `0x80A22820`, is
  listed under the deliberate skips below.

### `qr3` sites

None. Every `psq` word in these 17 bodies is a `qr0` `f31` prologue spill
(`0x80ACDB00`, `0x80A0F610`). No body reads or writes a quantised pair, so
`GQR3 = 0x00050005` is not exercised by this pass.

### Stream details this pass had to settle

1. **`0x80A933E0`'s masks are wider than the caller's fields.** The raw is
   `rlwinm r4,r4,20,0,11` and `rlwinm r0,r5,16,0,15`, i.e. `a`'s low **12** bits
   and `b`'s low **16** bits, not the 4-bit `kind & 0xF` / `kind >> 4 & 0xF` the
   specification's pseudo-code shows. The only caller passes 4-bit values, so
   the two readings agree there; the port keeps the literal masks.
2. **`0x80A0F610` and `0x80A707E0` disagree on the `0x800B6830` angle width.**
   `0x80A707E0` loads it with `lfd f1,160(r30)` (a double), while `0x80A0F610`
   loads `0x8042C5C8` with `lfs` (a single). The new body therefore uses the
   `f32` adapter; this is not the older `call4_f64` shape.
3. **`0x80A0F610` writes `+0x234` outside its null guard.** The
   `stfs f28,0x234(r28)` at `0x80A0F850` is the branch target of the failed
   allocation, so on that path the shipped code stores through address `0x234`.
   The port reproduces the stream and says so in the body comment.
4. **`0x80A85260` re-reads the timer word.** Both the `+0xDFA == 0` arm's second
   `lwz` and the two impact arms' `0x800976B0` result are separate loads/calls;
   the port keeps each call site, including the two `0x80072BC0` polls.
5. **`0x80A48940`'s tail is `0x801680E0`, not `0x80168FE0`.** The raw word at
   `0x80A4896C` is `0x4B71F774`, which is the rate setter. This confirms the
   correction `docs/SPEC_WAVE154_ENEMIES.md` makes to
   `ANALYSIS_WAVE127_ENEMIES.md`.

### Deliberate skips

Eight untranslated bodies remain in the file after this pass. None of them was
attempted, and none is claimed:

| address | TSV size | why it is left |
|---|---:|---|
| `0x80A22820` | 700 | `staffRollBlock` initialiser. The six clip/archive calls and the `block_00..block_004` returns are decoded, but the `+0x7FC` low-nibble staging at `0x80A22880..0x80A228C8` (`rlwinm`/`subf`/`srawi`/`addze`/`xoris` into the `0x43300000` double) does not reduce to a value the export states, and the result is the frame argument of `0x80167B20`. Rather than guess a frame, the row is left. This is the last row of the `0x80A20000` planner bucket. |
| `0x80A10880` | 940 | Frame handler with two `psq` prologue pairs, a `(u32 ^ 0x80000000)` ↔ double conversion for a segment index, five per-segment `0x1c`-byte vector steps and a virtual `+0x2c` call. The vector interpolation is readable but the `+0x92C`/`+0x928` record roles and the `0x80429EE8`/`0x80429D60`/`0x80429DD0` effect arguments are runtime state; a port would need the whole `0x1c` record layout proved first. |
| `0x80A92F40` | 1172 | Not read past its export; the body is a frame handler of the same family as `0x80A10880`. Left rather than guessed. |
| `0x80A9B3C0` | 1196 | Same family; not attempted. |
| `0x80A7BEC0` | 1464 | Same family; not attempted. |
| `0x80B04E90` | 1592 | Same family; not attempted. |
| `0x80A3A4A0` | 2620 | The export is a 950-line decompile with the paired-single prologue noise repeated per block; a faithful port needs a full raw walk. Not attempted. |
| `0x80A3DEA0` | 2940 | 417-line export of a frame handler over the `+0xA7C`/`+0x718`/`+0x6E8` state block; not attempted. |

The first row (`0x80A22820`) is the only one where the remaining doubt is a
single arithmetic idiom; the rest are size, not ambiguity. All eight are well
above the wave's 25-30 function budget and each would need its own raw-stream
walk.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A03C30` | `fn_80a03c30` | 92 | high |
| `0x80A07C80` | `fn_80a07c80` | 16 | low |
| `0x80A0F610` | `fn_80a0f610` | 660 | medium |
| `0x80A25F00` | `fn_80a25f00` | 48 | medium |
| `0x80A2F9D0` | `fn_80a2f9d0` | 48 | medium |
| `0x80A2FA00` | `fn_80a2fa00` | 48 | medium |
| `0x80A337B0` | `fn_80a337b0` | 4 | high |
| `0x80A43EE0` | `fn_80a43ee0` | 48 | medium |
| `0x80A43F10` | `fn_80a43f10` | 48 | medium |
| `0x80A48940` | `fn_80a48940` | 48 | high |
| `0x80A85260` | `fn_80a85260` | 528 | medium |
| `0x80A933E0` | `fn_80a933e0` | 36 | high |
| `0x80ACDB00` | `fn_80acdb00` | 148 | high |
| `0x80AF0370` | `fn_80af0370` | 44 | high |
| `0x80AF3840` | `fn_80af3840` | 16 | medium |
| `0x80AF3850` | `fn_80af3850` | 88 | high |
| `0x80B14D80` | `fn_80b14d80` | 28 | high |

### Why eight rows are `medium` and one is `low`

- **the five forwarders** - the visible call and each this-adjust are literal,
  but the hidden `r12` descriptor cannot be expressed in Rust.
- **`0x80A85260`** - every branch and store is literal; the `0x800976B0` query
  and the `0x80429E00 + 0x3C` boundary are `main.dol` state whose meaning is
  inferred from the guard shape.
- **`0x80A0F610`** - all 165 instructions are decoded; the `0x80B6EF08` side
  table is BSS filled elsewhere, `0x8042C5C8` is a runtime angle, and the
  `0x8008C200`/`0x8016A5E0` sub-object layouts are only shaped by the stream.
- **`0x80AF3840`** - four literal instructions; the callee's argument roles are
  inferred from its `(s16*, s16, size)` shape.
- **`0x80A07C80`** - `low`, because `0x80168180` has no export row anywhere, so
  only the staged registers are certain. The body forwards `r3` unchanged.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 17 sizes sum to 1,948
  bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_<name>.c` (the alias row cites
  `80a337b0_thunk_FUN_80a337c0.c`, its real export name).
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; none of the 17 addresses was in `ledger/functions.csv`
  or had a body in `src/**/*.rs` before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The five vtable words quoted in the forwarder docs were re-read from
  `0x80B3ABE0`, `0x80B3C730`, `0x80B3C780` and `0x80B406D0` and match.
- The module header was updated: cluster item 137, the cluster count (136 to
  137), the function totals (3530 to 3547 public translations, 3459 to 3476
  `fn_<addr>` bodies), the wave list and the "start here" list.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave155` reports 17 rows for this lane, 17
  unique addresses and `OK`.

## Note for the coordinator

`fn_80a707e0` (wave 152, section 134) ends its loop with
`link = object + 0x238;`, while the raw stream at `0x80A70A28` is
`or r29,r30,r30` (i.e. `link = object`). `fn_80a0f610`'s stream has the same
shape and the new body uses `link = object`. That earlier body was left
untouched by this lane; the two should be reconciled in a later pass.
