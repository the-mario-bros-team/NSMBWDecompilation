# Coverage - Wave 141 (`game::enemies`)

## How to read this file

This page records the Wave 141 `game::enemies` lane: 17 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 123 and mirrored one
row per body in `ledger/agents/wave141-enemies.csv`. Each `evidence` cell names
the Ghidra C export the body was reconstructed from; the byte counts are the
`size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` with the segment shift of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`).

The lane has two parts:

- **11 bodies** are the "ready" rows of `docs/SPEC_WAVE140_ENEMIES.md` (ranks
  2-12). Rank 1 is the specification's single **conditional** row and was
  deliberately skipped (see *Deliberate skips*).
- **6 bodies** are still-open rows of the `0x80AC0000` planner bucket: the
  `class 0x88` actor predicate at `0x80ACB000`, the two-slot follower chain it
  serves (`0x80ACB130`, `0x80ACB330`, `0x80ACB4B0`, `0x80ACB5D0`) and the
  side-flip/jump step at `0x80ACDE00`.

Every `psq_l fD,0(rA),1,qr3` in this tranche loads the `u16` staged two
instructions earlier. `GQR3 = 0x00050005` is an unsigned 16-bit type at scale 0,
so each site is written as `(f32)*(u16*)` per `docs/REL_ELF_MAPPING.md`.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 17 | 7260 | 7 | 10 |

Status split: 17 `translated` (7 high, 10 medium), no `partial` and no `stub`.

## What this pass covers

- **The two waypoint chases.** `0x80AEDD40` and `0x80AEF130` share one body
  shape: resolve the actor index from `+4 >> 20 & 0xF`, find the active player's
  944-byte record, pick `rec_base + 16 * (waypoint_count + index)`, build the
  `(way_x, -halfword, 0.0)` delta, and cache it at `+0x3F8..+0x400` /
  `+0x688..+0x690` whenever either component leaves the `±0.01` band. The cached
  vector is normalised and turned into a smoothed pitch/yaw pair (`+0x3E4`/
  `+0x3E6` or `+0x674`/`+0x676`), then into a `+0xE8` step along
  `(cos(pitch)*sin(yaw), sin(pitch))`, with the raw waypoint coordinate
  snapping the position back inside `±0.1` and a bracket test on both axes
  driving the arrival branch. Rank 2 notifies through `+0x394`/`+0x18`, rank 3
  through `+0x60`/`+0xD4`, and rank 3 additionally forces `+0xB4 = 0.0` while
  `+0x38F` is clear.
- **The side-flip steering latch `0x80AA93F0`.** The delta is taken against the
  *biased* target `(target + +0xD0/+0xD4) - this`, the `+0x9C0` countdown gates
  the retarget, and the octant from `0x80096240` is compared with the `+0x348`
  side byte. Equal means the direction is rebuilt as `(±cos, -sin)` in `+0xE8`/
  `+0xEC` and copied to `+0x9A0`; different means the byte is rewritten and
  `+0x9BC = 1`. The `0x28` (40-frame) cooldown re-arms inside a unit-radius
  neighbourhood.
- **The vertical seek `0x80AFDAC0`.** `0x8015F470` steps x toward the player by
  `0.5`; the y target is `player.y + 8 + 16 * (s32)+0x608`. Below the target the
  actor fires the `0x1B5` event once (guarded by `+0x5D4 >= 0`) and snaps y;
  at or above it banks `+0x5CA` by `±0xE00` and sways `+0x102` by
  `3072.0 * sin`. The `+0x60`/`+0xD4` event fires when the x-step and `+0x5D8`
  both report arrival.
- **The orbit/offset follow `0x80A429C0`.** Mode `1` writes
  `+0x104 = +0xAF0 + owner_angle` while the cos/sin pair uses the independent
  `+0xAF2` offset, places the actor at `owner + radius*(cos, sin)` minus the
  `+0xD0/+0xD4/+0xD8` bias and calls the owner's `0x80707400`; every other mode
  writes `owner_angle + 0x4000` and uses the ellipse radii `+0xACC`/`+0xAD0`.
- **The four-corner probe `0x80AA4070`.** `0x80AA3F50` fills the swept box, the
  `+0xA4E` facing sweeps it by `12.0*(cos, sin)`, the hull of the swept and raw
  corners is expanded by `1.0`, and `(min,min)`, `(max,max)`, `(max,min)`,
  `(min,max)` are tested with `fn_80aa4020`; the final boolean is the
  `neg`/`or`/`rlwinm` form.
- **The angle/velocity clamp `0x80AA35E0`.** `0x801611C0` writes `+0xA4E`, the
  pair `+0xE8`/`+0xEC` is integrated by `0.03125*(cos, sin)` and clamped to
  `±1.25`, and the Q12 pair receives `-4096*sin` / `+4096*cos`.
- **The row-table fall `0x80AFD910`.** `+0xAC += 0x80AFD170(this)`, y snaps
  one-sidedly to `player.y + 8 + 16*(s32)+0x608`, `+0x5CC` steps by `256`
  toward `0x80B31750` (or row `(+0x604 - 1) & 3` of `0x80B31758`, indexed by
  `player+0x52C`), and `+0x102 = +0x5CC + 1536.0*sin(+0x5CA)` after the bank
  steps by `±0x800`.
- **The trail normal `0x80B1D520`.** `+0x658` indexes the six-entry
  `0x80B32670` table, `+0x608` receives the negated angle, the length is
  `14.0 + 0x800E9020(this + 0x668)`, the x sign comes from
  `0x80B32668[(i >> 31) + (i > 2)]`, and the normalised 2-D trail direction
  lands in `+0x614`/`+0x618`.
- **The screen-space rotate `0x80A436C0`.** `0x4000 - +0x910` mirrors the
  angle, `8.0 + +0x8FC + other+0x4D4` is the radius, the rotated point lands in
  `+0x908`/`+0x90C`, the clip test receives `((int)x & 0xFFFF, (int)-y &
  0xFFFF)` and a hit sets bit 0 of `0x80429D14`; the return polarity is
  inverted (`1` = miss).
- **The balloon rise `0x80A24AD0`.** The rise target is
  `*(f32*)(*(u32*)0x80429DD0 + 0x8FEC4)` minus two `4.0` margins; a zero
  `0x80429D40 + 0x2C` word re-seeds `+0x71C`/`+0x710`/`+0x712`; the bob angle
  steps by `±0x400`, the phase by `0x400` and wraps at `0x8000`, and
  `+0xB0 = 8.0*sin(angle) + (rise + +0x61C)` before `fn_80a24580`.
- **The `class 0x88` follower family (bucket).** `0x80ACB000` gates an id
  through `0x80162D00`, the `+8 == 0x88` tag, `0x80ACAED0`, and a
  `+0x394`/`+0x28` -> `+0x14` vtable chain. `0x80ACB130` relinks one slot of the
  two-slot chain (head `+0x5C0`, slots `+0x5C4`) with a one-shot append;
  `0x80ACB330` clamps each node's `+0xE8` to `[-3.0, +3.0]` over `{+24, -24}`
  anchor steps; `0x80ACB4B0` spawns `count` objects with
  `0x80064610(0x88, side & 1, this + 0xAC, 0, 0)`; `0x80ACB5D0` gates every node
  with `0x80064F50(node, 2)` and, when all accept, runs the `0x80ACB540` release
  walk and retires the owner. `0x80ACDE00` is the side-flip step: the
  `0x15 << side` mask in `+0x274` flips `+0x348`, negates `+0xE8`, re-seeds
  `+0x5F4`/`+0x5F8`, resets the `+0x588` clip and rebuilds through `0x80ACDB00`;
  otherwise a `-1` result from the `0x80B761C0 + 8*side` action record snaps x to
  `+0xB8` and switches state with `0x80B76220`.

### Corrections to `docs/SPEC_WAVE140_ENEMIES.md`

The specification's prose is right about the constants and the callee table but
wrong about four details; the ports follow the words (and Ghidra's export, which
agrees with the words in every case):

- **Rank 4 (`0x80AA93F0`).** There is **no** `+0xA4E` write and **no** first
  `sin`/`cos` pair computed from `+0xA4E`. The body builds only the
  `(target + bias) - this` delta and calls `fn_80aa9350`; the velocity is
  `(±cos, **-sin**)`, with the `-sin` unconditional.
- **Rank 5 (`0x80AFDAC0`).** The two y arms are **swapped** in the prose. The
  literal `fcmpo`/`bge` banks and sways while `this.y >= y_target`, and fires the
  `0x1B5` event while `this.y < y_target`. The Ghidra export prints the same
  sense.
- **Rank 8 (`0x80AA35E0`).** The Q12 pair is `+0x100 = -4096*sin` and
  `+0x102 = +4096*cos`; the prose pairs `+0x100` with cos and `+0x102` with sin.
  The velocity order in the prose is correct.
- **Ranks 2/3.** `0x8015F140` takes five arguments, not four:
  `(dst = this+0xE8, f1 = *(f32*)(rec+4), f2 = 1.0, f3 = *(f32*)(rec+8),
  f4 = 1.0e-4)`. The prose omits the `1.0` rate and shifts the dead-band. Rank 3
  also has **no** `+0x3E0` latch and **no** `rec + 0xC` test: its `0x80B77790`
  payload belongs to the variant-`0` arrival branch, while rank 2's
  `0x80B77650` payload is the `rec + 0xC` one.
- **Rank 12 (`0x80A24AD0`).** The record-manager global is **`0x80429D40`**
  (`0x80430000 - 0x62C0`), not `0x80439D40`.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A24AD0` | `fn_80a24ad0` | 312 | translated | high |
| `0x80A429C0` | `fn_80a429c0` | 476 | translated | medium |
| `0x80A436C0` | `fn_80a436c0` | 328 | translated | high |
| `0x80AA35E0` | `fn_80aa35e0` | 408 | translated | high |
| `0x80AA4070` | `fn_80aa4070` | 444 | translated | medium |
| `0x80AA93F0` | `fn_80aa93f0` | 496 | translated | high |
| `0x80ACB000` | `fn_80acb000` | 180 | translated | medium |
| `0x80ACB130` | `fn_80acb130` | 196 | translated | medium |
| `0x80ACB330` | `fn_80acb330` | 252 | translated | medium |
| `0x80ACB4B0` | `fn_80acb4b0` | 140 | translated | medium |
| `0x80ACB5D0` | `fn_80acb5d0` | 188 | translated | medium |
| `0x80ACDE00` | `fn_80acde00` | 232 | translated | medium |
| `0x80AEDD40` | `fn_80aedd40` | 1208 | translated | high |
| `0x80AEF130` | `fn_80aef130` | 1180 | translated | high |
| `0x80AFD910` | `fn_80afd910` | 388 | translated | medium |
| `0x80AFDAC0` | `fn_80afdac0` | 480 | translated | high |
| `0x80B1D520` | `fn_80b1d520` | 352 | translated | medium |

### Why ten rows are `medium`

- **`0x80A429C0`** - `0x80707400` has no export in this module; only its
  `owner` argument is established.
- **`0x80AA4070`** - `0x80AA3F50` is not translated in this file, so the
  `[f32;4]` box layout comes from the specification.
- **`0x80AFD910`** - `0x80AFD170` is not translated in this file; its `f1`
  delta is used as returned, with the role taken from the specification.
- **`0x80B1D520`** - `0x800E9020` has no export; only `r3 = this + 0x668` and
  the `f1` result are established.
- **`0x80ACB330`** - the two table steps (`0x80B306B8`/`0x80B306BC`) and the
  `-3.0`/`+3.0` bounds (`0x80B306A8`/`0x80B306C0`) are literal and the clamp arms
  are plain `fmr` copies of the constants; only the `fn_80acb000` predicate step
  is inferred.
- **The other four bucket rows** (`0x80ACB000`, `0x80ACB130`, `0x80ACB4B0`,
  `0x80ACDE00`) - `0x80ACAED0` and `0x80035BB0` have no export, and the `+0x5C0`
  head-word / `0x80B760F8` notification roles are inferred from the chain shape
  and the BSS payload slots.
- **`0x80ACB5D0`** - the `0x80064F50(node, 2)` gate is literal at every call
  site, but the callee has no export, so *why* a node may be rejected is not
  established; the release pair (`0x80ACB540`, then `0x80064D10(this, 0)`) is
  exact.

## Deliberate skips

- **`0x80A3A4A0` (2620 bytes, Fire Snake 150-segment body manager).** This is the
  specification's single **conditional** row: the slot-state dispatch at
  `0x80A3A74C` tests a loop-carried `r0` whose producer is overwritten by the
  sprite-byte packer, so the entry into the render block for the first two slots
  of a frame has two coherent readings. It was left for a lane that can trace a
  live frame rather than guess a state value.
- **The other 14 untranslated `0x80AC0000` rows.** `0x80AC0750` (240),
  `0x80AC0840` (396), `0x80AC1800` (276), `0x80AC8850` (304), `0x80AC9870`
  (464), `0x80ACC8F0` (652), `0x80ACCC00` (428), `0x80ACCE20` (368),
  `0x80ACDB00` (148), `0x80ACDCC0` (304), `0x80ACDEF0` (252), `0x80ACE690`
  (172), `0x80ACE900` (296) and `0x80ACEE40` (372) are the model/matrix and
  action-record bodies of the bucket. `0x80ACDB00` in particular is the
  `+0x544`/`+0x548` matrix rebuild whose four engine callees
  (`0x8023A0B0`, `0x801651F0`, `0x80169F80`, `0x80168FE0`) need their own
  argument-order verification pass; it is called by `fn_80acde00` through its
  analysis address instead.

No address that already had a `ledger/functions.csv` row or a `fn_<addr>` body
in `src/game/enemies.rs` was re-claimed.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 17 sizes sum to 7260
  bytes (6072 for the eleven specification rows, 1188 for the six bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice, and none of the 17 was in
  `ledger/functions.csv` before this pass.
- All 17 bodies were decoded word-by-word from the ELF and cross-checked against
  the Ghidra export; the five specification corrections above were re-verified
  against both sources.
- The module header was updated: cluster item 123, the cluster count (122 to
  123), the function totals (3283 to 3300 public translations, 3212 to 3229
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` exits `0` with no error and no warning
  naming `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave141` reports 17 rows for this lane
  (`translated/high=7`, `translated/medium=10`), 17 unique addresses and `OK`.
