# Coverage - Wave 154 (`game::enemies`)

## How to read this file

This page records the Wave 154 `game::enemies` lane: 25 functions of REL module
3, `d_enemiesNP`, appended to `src/game/enemies.rs` as section 136 and mirrored
one row per body in `ledger/agents/wave154-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Raw words
were decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` program
headers as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment; the
`0x80B2A5xx`/`0x80B32DC0` segments were mapped from their own headers). Every
body was followed from its entry to its terminal `blr` or tail `b`. Byte counts
are the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 25 | 3588 | 14 | 11 |

Status split: 25 `translated` (14 high, 11 medium), no `partial` and no `stub`.
Covered address range: `0x80A05DF0..0x80AEE820`.

## What this pass covers

- **The nine "ready" rows of `docs/SPEC_WAVE153_ENEMIES.md`:**
  - `0x80A637E0` - `daEnGesso_c` near-player state select: `+0x737` countdown,
    the `+0x544` owner step, the `0x80` angle step, and the `96.0`/`144.0`
    `0x80097800` x bands into `0x80B722D0`/`0x80B721D0`.
  - `0x80AD45E0` - carrier offset commit: holder `+0x700..+0x708`, bias
    `+0x618..+0x620`, `+0xB4 = 500.0`, the `+0x83C` y delta and the three
    `+0x828..+0x830` commitments.
  - `0x80A80E50` - `daEnIgaKuribo_c` `Kyoro` entry: `walk`/`look` clips, the
    `0.0` velocity reset, `+0x658 = 0xD0`, `+0x65A = 0xC000`, the single `qr3`
    widen into `1280.0 * sin` at `+0x100`, and `+0x65C = rand(90) + 300`.
  - `0x80A06220` - fire-cannon slot spawn: the `0x80A06190` index gate, the
    `0x80A06130` `0x14`-byte record, `(x, y, z - 10.0 * (index+1))`, and the
    five-argument `0x80064610(0x98, index, pos, scratch, 0)`.
  - `0x80AD46C0` - carrier frame step: `+0x720` selects the `0x80101AA0` height
    wrap (with the `+0x838` delta) or the three-term direct y.
  - `0x80AF3570` - cannon guide world position: two `qr3` widens of `+0x7A4`
    into `32.0 * sin` / `32.0 * cos` around `+0x798`/`+0x79C`.
  - `0x80B05A80` - `daEnSnakeBlock_c` binder over `4 + +0x2B18` clips and the
    `+0x2B24 = count + 60` timer.
  - `0x80B0DE10` - `daEnTeresa_c` state step: the `0x80B0D4D0` turn-order gate
    into `0x80B78988`, the `+0x9E0`/`0x80B0D150` wake into `0x80B78948`, then
    `0x80B0CFD0`.
  - `0x80ACE690` - net-Noko clip blend commit: `+0x544` bank name into `+0x588`
    over `+0x548`, with the `+0x5F0`-selected `2.0`/`1.5` rate.
- **Its three "conditional" rows**, all literal once the unnamed engine state is
  treated as opaque (the same treatment wave 153 gave its two conditional rows):
  - `0x80A4B140` - `daEnCrow_c` nearest-player scan over the `0x80429CA8` mask
    and the `+0x60`/`+0xE0` predicate.
  - `0x80A5DBC0` - `daEnGabon_c` angle step: `0x10000 / speed`, the `+0x20`
    slot frame, one `qr3` widen and the `+0x660` amplitude.
  - `0x80A2EB40` - `daEnBubble_c` water probe over `0x800EB930` plus the stage
    mode byte `3` and the `+0x8FEC4` surface.
- **The untaken snake-block sibling `0x80B05B80`** - word-identical to
  `0x80B05A80` without the `+0x2B24` store; it sat one row below the wave-153
  cut.
- **Three carrier steps beside `0x80AD45E0`** - `0x80AD47E0` and `0x80AD4850`
  acquire the `0x1E9`/`0x1C0` carrier records and enter `0x80B76630`;
  `0x80AD48C0` dispatches `0x80AD4000` for `+0x84C == 4` and `0x80AD3D50`
  otherwise, then `0x80021780`.
- **Two cluster helpers** - `0x80AF42C0` (guide-angle sweep through the 16-byte
  `0x80AF3840` thunk, then `0x80B779F0` and `+0xB50 = 0`) and `0x80A4B0B0` (the
  crow clip binder over `+0x540`/`+0x544`/`+0x584`).
- **`0x80A05DF0`** - the fire-cannon sized destructor (`0x80329958` vtable,
  `0x8016D4B0`, `0x802D7B20`, `0x802B9280` for a positive mode).
- **All six remaining `0x80AE0000` planner-bucket bodies** (the bucket is now
  complete): `0x80AED170` (sized free), `0x80AEDA60`/`0x80AEDA90`/
  `0x80AEE7F0`/`0x80AEE820` (four `0x802DCBFC` deleting-destructor forwarders
  with `+0x18`/`+0x0C`/`+0x18`/`+0x24`), and `0x80AEDB90` (the `0x408`-byte
  allocator with vtable `0x80B61BA8`).

### `qr3` sites

| address | sites | port |
|---|---|---|
| `0x80A80E50` | `0x80A80ECC` `psq_l f1,0(r4),3,1`, `r4 = r1 + 8` | `(f32)(u16)0xC000 * 0x80B2E320` (= 192.0 turns) |
| `0x80A5DBC0` | `0x80A5DC38` `psq_l f1,0(r3),3,1`, `r3 = r1 + 8` | `(f32)(u16)product * 0x80B2D1BC` |
| `0x80AF3570` | `0x80AF35A8` (`r5 = r1 + 10`) and `0x80AF35D4` (`r3 = r1 + 8`) | the same `+0x7A4` halfword widened twice at `0x80B313D4` |

`GQR3` is `0x00050005` (type 5 = unsigned 16-bit, scale 0, `W = 1`), so each
load is exactly `(f32)(u16)halfword`. Every other `psq_l`/`psq_st` in the 25
rows is a `qr0` `f31` spill.

### Specification corrections made while porting

1. **`0x80AD45E0`: the `+0x830` store has three terms.** The Ghidra export
   prints `+0x830 = *(holder+0xB4) + *(this+0x708)`; the raw stream also adds
   `*(this+0x620)` (`fadds f3,f4,f3` at `0x80AD4674`, stored at `0x80AD46A4`).
   The port follows the stream, as the specification's pseudo-code does.
2. **`0x80A5DBC0`: the speed factor is sign-extended from 16 bits.** The word at
   `0x80A5DBF8` is `extsh r30,r0` (`0x7C1E0734`), not `extsb`; the factor is
   `(i16)(0x10000 / (i32)speed)`. The difference is observable: at speed `2.0`
   the quotient is `0x8000`, which `extsh` turns into `-32768` while the
   specification's `i8 a` reading would have kept the low byte `0`.
3. **`0x80A2EB40`: the scan answer is `extsb`, not `(i16)`.** The raw is
   `extsb r0,r3` at `0x80A2EB94`, so only the low byte of the `0x800EB930`
   result is compared against `1`. The last comparison re-materialises
   `+0xB0 + dy` after the mode-byte load (`0x80A2EBC0..0x80A2EBC8`); there is no
   store between, so the port reuses the single `y` value.
4. **`0x80B05A80`/`0x80B05B80`: the `"ridden"` name is dropped by the callee.**
   `0x80B04120`'s raw stream forwards its incoming `r4` to `0x8023A190`
   (`0x80B04134..0x80B04138`), but the existing translation
   [`fn_80b04120`] takes only the owner. The port keeps the same treatment as
   `fn_80b05740` (which documents the same limitation) rather than changing a
   callee that five other bodies already use.

### Deliberate skips

- `0x80A03C30` (92 bytes, the `+0x69C` twenty-four-frame stage-effect timer whose
  tail call carries a fourth integer register `r6 = 24`) is decoded but not
  claimed: whether `0x80078470` reads `r6` cannot be established, and the row is
  outside both the specification and the planner bucket.
- `0x80ACDB00` (148 bytes, the `net_walk2` binder) is decoded but not claimed:
  its blend reaches `0x80168FE0` in `f1` as a leftover of the caller's
  `0x80168080` return, and the existing caller `fn_80acde00` calls it with only
  `r3`, so a body would have to invent the blend argument.
- Still out of scope and untouched: `0x80A10880`, the four bucket bodies above
  1,200 bytes, and every other still-untranslated address outside this pass's
  claims. They were not attempted rather than guessed.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A05DF0` | `fn_80a05df0` | 116 | medium |
| `0x80A06220` | `fn_80a06220` | 212 | high |
| `0x80A2EB40` | `fn_80a2eb40` | 188 | medium |
| `0x80A4B0B0` | `fn_80a4b0b0` | 136 | high |
| `0x80A4B140` | `fn_80a4b140` | 236 | medium |
| `0x80A5DBC0` | `fn_80a5dbc0` | 212 | medium |
| `0x80A637E0` | `fn_80a637e0` | 236 | high |
| `0x80A80E50` | `fn_80a80e50` | 220 | high |
| `0x80ACE690` | `fn_80ace690` | 172 | high |
| `0x80AD45E0` | `fn_80ad45e0` | 224 | high |
| `0x80AD46C0` | `fn_80ad46c0` | 212 | high |
| `0x80AD47E0` | `fn_80ad47e0` | 80 | medium |
| `0x80AD4850` | `fn_80ad4850` | 80 | medium |
| `0x80AD48C0` | `fn_80ad48c0` | 72 | high |
| `0x80AED170` | `fn_80aed170` | 64 | high |
| `0x80AEDA60` | `fn_80aeda60` | 48 | medium |
| `0x80AEDA90` | `fn_80aeda90` | 48 | medium |
| `0x80AEDB90` | `fn_80aedb90` | 76 | high |
| `0x80AEE7F0` | `fn_80aee7f0` | 48 | medium |
| `0x80AEE820` | `fn_80aee820` | 48 | medium |
| `0x80AF3570` | `fn_80af3570` | 196 | high |
| `0x80AF42C0` | `fn_80af42c0` | 132 | medium |
| `0x80B05A80` | `fn_80b05a80` | 180 | high |
| `0x80B05B80` | `fn_80b05b80` | 172 | high |
| `0x80B0DE10` | `fn_80b0de10` | 180 | high |

### Why eleven rows are `medium`

- **`0x80A4B140`** - every gate and both distance terms are literal; the
  `0x80429CA8` mask byte and the `+0xE0` predicate are `main.dol` state whose
  meaning is not established by this module.
- **`0x80A5DBC0`** - the division, widen, sine and amplitude are literal; the
  `+0x20` slot on the `+0x584` element is unnamed.
- **`0x80A2EB40`** - both gates and the strict `<` are literal; the manager
  record layout and the two stage fields are `main.dol` state, inferred from the
  neighbouring `fn_80a2ec00`.
- **`0x80AD47E0`, `0x80AD4850`** - the guards and the state word are literal;
  `0x80037060`/`0x80037110` have no export row, so their `0x1E9`/`0x1C0` scans
  were decoded from the `main.dol` image.
- **`0x80AF42C0`** - both conditions and both stores are literal; `0x80AF3840`
  is decoded as the 16-byte `0x8015F4F0` tail thunk.
- **`0x80A05DF0`** - the guard, vtable word and three calls are literal; the
  class that owns `0x80329958` is not named.
- **the four forwarders** - the visible call and each this-adjust are literal,
  but the hidden `r12` operand cannot be modelled in Rust.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 25 sizes sum to 3,588
  bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; none of the 25 addresses was in `ledger/functions.csv`
  and none had a body in `src/**/*.rs` before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The three `qr3` sites are ported as `(f32)(u16)`; every other `psq` in these
  rows is a `qr0` spill.
- The module header was updated: cluster item 136, the cluster count (135 to
  136), the function totals (3505 to 3530 public translations, 3434 to 3459
  `fn_<addr>` bodies), the wave list and the "start here" list.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave154` reports 25 rows for this lane, 25
  unique addresses and `OK`.
