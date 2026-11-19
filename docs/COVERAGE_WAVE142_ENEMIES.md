# Coverage - Wave 142 (`game::enemies`)

## How to read this file

This page records the Wave 142 `game::enemies` lane: 17 functions of REL module 3,
`d_enemiesNP`, appended to `src/game/enemies.rs` as section 124 and mirrored one
row per body in `ledger/agents/wave142-enemies.csv`. Each `evidence` cell names
the Ghidra C export the body was reconstructed from; the byte counts are the
`size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All addresses are **synthetic analysis addresses** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` with the segment shift of
`docs/REL_ELF_MAPPING.md` (`file_offset = address - 0x80A00000 + 0x1000`).

The lane has two parts:

- **12 bodies** are the whole "ready" set of `docs/SPEC_WAVE141_ENEMIES.md`
  (ranks 1-12). Every one of them was class D only because of a
  `psq_l ...,qr3`; they are unblocked by the pinned `GQR3 = 0x00050005`
  (unsigned 16-bit, scale 0), so each such site is written as `(f32)*(u16*)`.
- **5 bodies** are still-open rows of the `0x80B00000` planner bucket: the
  teresa-tail family's four-slot target search (`0x80B0D380`) with its two
  consumers (`0x80B0D2A0`, `0x80B0D9A0`), the frame forwarder
  (`0x80B0DB50`), and the ground-glide step `0x80B07D70` - one of the fifteen
  `qr3` rows that `docs/SPEC_WAVE141_ENEMIES.md` listed for the next wave.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 17 | 7688 | 14 | 3 |

Status split: 17 `translated` (14 high, 3 medium), no `partial` and no `stub`.

## What this pass covers

- **The BeansKuribo hop/orbit step `0x80A0E580`.** The hop height `+0x5E8` is
  integrated by `+0x5EC` and ping-pongs between the player-relative ceiling
  `|probe.y - 4.0 - player.y|` and the `0.0` floor (each bounce flips `+0x615`
  and negates the velocity), and the actor is placed on an eight-unit circle
  around the player from the `+0x612` orbit angle. The player scratch word is
  banded (`< 4` -> 1, `4..=8` -> 2, `> 8` -> 3), the pad claim consumes it, and
  the flag word drives the side byte, the `0x1FC`/`0x1FA` cues, the pose push
  and the pad release.
- **The two-node matrix commit `0x80B0C540`.** Two `+0x394`-handle probes gate a
  `0x400` spin step; the y is lifted by `3.0 * sin(spin)`, the primary matrix is
  committed into `+0x588`, row 3 and row 0 are read back into
  `+0x988..0x990`/`+0x9A0..0x9A8`, the render-only `+0x5C8` node is rebuilt when
  `+0x9C8` is set, and the copied triad's third component is dropped by `24.0`
  (or `24.0 * +0xE4`) before the zero-angle `+0x548` commit.
- **The `daEnPukuPukuParent_c::StateID_Move` chain solver `0x80AEB450`.** Three
  segment shapes share one per-segment write: a travelling wave over the
  sixteen-entry amplitude table (state 0/1), a hanging ramp
  `y - 24i - 8i*sin` (state 2/4) and a standing ramp `y + 24i + 8i*sin`
  (state 3).
- **The `daEnPolter_c::StateID_Wait` hover `0x80AE4A50`.** An `8.0 * sin` bob
  sets y under the global base height, the `+0x69C` steering state takes one-pole
  steps of `0.3`/`1.0` outside an `8.0` dead zone, the `+0x6BC` sweep adds
  `20.0`/`40.0 * cos`, and the mode selects the pitch pair or the full
  mirror/ramp pair. The `+0x500` message and the `+0x658` tick only run when a
  target was found.
- **The `daEnCrow_c` trio.** `0x80A4B740` steers on a circle: the `+0x5BC`
  phase feeds the forward cosine, the `+0x5C0` phase (clamped to a half turn)
  feeds the lateral sine from the per-index `0xB0` speed table, the actor faces
  its world motion through `atan2_16`, and it gives up `16.0` past the player on
  its own side. `0x80A4B420` dives: side-selected speed and yaw offset, a `-90`
  pitch approach, `speed*cos` / `0.375*sin` velocity and a player probe that
  fires the hit message (caching the player position) or the miss message.
  `0x80A4BBD0` climbs on a phase clamped at a quarter turn and levels the roll
  at 16 units/frame.
- **The `daEnGesso_c` pair plus the bob.** `0x80A635D0` is the two-phase bob
  (`1024.0 * sin` around `+0x728`, three state-1 frames and thirty state-2
  frames, then a `144.0`-unit pad query fanning out three messages);
  `0x80A63FF0` is the drifting jet (`(sin, cos)` rotated a quarter turn, a
  `+0x733` countdown to the z snap, then deceleration); `0x80A641F0` is the
  coast stop (y sine, speed decay floored at `0.0`, three-way message gate).
- **The hit-response splash `0x80ABA080`.** `atan2_16(5.2, range*scale)` gives
  the direction, cue `0x706` plays, and the receiving actor's vtable slot
  `0x3FC` gets the blended impulse `s + 0.5*(5.2 - s)` and `5.2*cos`.
- **The vertical ping-pong platform `0x80A46200`.** A zero manager word re-seeds
  the travel target; otherwise the signed target drives `+0xACE` by `±0x400`
  while `+0xACC` counts to `0x8000`, and `4.0 * sin` displaces the base height.
- **The teresa-tail family (bucket).** `0x80B0D380` scans the four player slots
  for the nearest one in front of the actor (side split, optional strict-side
  filter, `1e9` seed, `0x80064740` fallback); `0x80B0D2A0` turns that target
  into an x delta plus a staged neighbour anchor; `0x80B0D9A0` advances the
  follow step and raises `0x80B78988` on arrival; `0x80B0DB50` forwards the
  frame; `0x80B07D70` glides the actor onto the ground surface.

### Corrections to `docs/SPEC_WAVE141_ENEMIES.md`

The specification's helper table and constants are right, but eight details do
not survive a word-level read. The ports follow the words (and the Ghidra export,
which agrees with the words in every case):

- **Rank 2 (`0x80B0C540`).** The `24.0` drop applies to the **third** component
  (z) of the copied triad, not to y, and the lift uses **`sin`**, not `cos`.
  The body also writes a second triad: `holder_row(..., 0, ...)` into
  `+0x9A0..+0x9A8`, which the prose omits, and the `+0x9C8` branch rebuilds the
  primary matrix from the position/angles before copying it to `+0x5C8`.
- **Rank 3 (`0x80AEB450`).** Modes 0/1 use **`sin`**, not `cos`; the amplitude
  table base is `0x80B3115C` (`0x80B31150 + 0x0C`), which the prose states
  correctly, but the pre-scale is `0x80B311B8`.
- **Rank 4 (`0x80AE4A50`).** The bob step is **`8.0 * sin`**, not `0.5 * sin`
  (`0x80B30E74` is the same word as the `8.0` dead zone), and the `+0x500`
  message plus the `+0x658` node tick sit **inside** the target-found branch, so
  a target-less frame returns immediately.
- **Rank 5 (`0x80A4B740`).** The phase bounds are **`0x10000000`** and
  **`0x8000000`** in accumulator units (the prose's `0x1000`/`0x800` are the
  post-`>>12` turn units), the sine argument comes from **`+0x5C0`** (only the
  cosine uses `+0x5BC`), and the two `i16` side tables are
  `{+4551, -4551}`/`{+10013, -10013}` and `{+12288, -12288}`, i.e. **positive
  first**, not negative first. The `0xD0`/`0xE0` rate tables are likewise
  `{256, 192, 160, 128, 96, 80, 72, 64}` and
  `{640, 544, 496, 456, 432, 416, 400, 386}` (the prose transposes each pair).
- **Rank 6 (`0x80A4B420`).** The landing gate is **`+0x5BC > 0xBFFFFFF`**
  (`0xC000` turn units), not `0xC00`.
- **Rank 7 (`0x80A635D0`).** The pad query is `48.0 + 96.0 = 144.0` - the prose
  is right - but the two timer reloads are `0x1E` (30) and `3` in the opposite
  fields from the prose's wording: `+0x733` holds 30 for state 2 and `+0x734`
  holds 3 for state 1.
- **Rank 11 (`0x80A4BBD0`).** The phase clamp is **`0x10000000`**, not `0x1000`.

The two `main.dol` global identities the wave-141 record corrected elsewhere
repeat here: the scene/height manager is `0x80429DD0` (not `0x80439DD0`) and the
record manager is `0x80429D40` (not `0x80439D40`). The crow's side table is
`0x8042C1A0`, and the BeansKuribo pose globals are `0x8042A050`/
`0x8042CC88`/`0x8042CC8C`.

## Functions

| address | function | bytes | status | confidence |
|---|---|---:|---|---|
| `0x80A0E580` | `fn_80a0e580` | 968 | translated | high |
| `0x80A46200` | `fn_80a46200` | 300 | translated | high |
| `0x80A4B420` | `fn_80a4b420` | 548 | translated | high |
| `0x80A4B740` | `fn_80a4b740` | 604 | translated | high |
| `0x80A4BBD0` | `fn_80a4bbd0` | 304 | translated | high |
| `0x80A635D0` | `fn_80a635d0` | 488 | translated | high |
| `0x80A63FF0` | `fn_80a63ff0` | 392 | translated | high |
| `0x80A641F0` | `fn_80a641f0` | 380 | translated | high |
| `0x80ABA080` | `fn_80aba080` | 308 | translated | high |
| `0x80AE4A50` | `fn_80ae4a50` | 660 | translated | high |
| `0x80AEB450` | `fn_80aeb450` | 700 | translated | high |
| `0x80B07D70` | `fn_80b07d70` | 252 | translated | medium |
| `0x80B0C540` | `fn_80b0c540` | 848 | translated | high |
| `0x80B0D2A0` | `fn_80b0d2a0` | 212 | translated | medium |
| `0x80B0D380` | `fn_80b0d380` | 328 | translated | high |
| `0x80B0D9A0` | `fn_80b0d9a0` | 332 | translated | medium |
| `0x80B0DB50` | `fn_80b0db50` | 64 | translated | high |

### Why three rows are `medium`

- **`0x80B0D2A0`** - every store and the `{0, 8, 16, -8}` table are literal, but
  the *roles* of `+0xD4` (a y bias) and `+0x9E6` (the table index) are inferred
  from the table values; the callee is [`fn_80b0d380`].
- **`0x80B0D9A0`** - the exact-equality arrival test, the cue `0x1E3`, the three
  state writes and the six-argument vtable call are literal; the approach test
  `0x80B0D6E0` is still untranslated and is called through its analysis
  address, so only its call position is established. The frame tail
  `0x80B0C890` *is* translated and is called directly.
- **`0x80B07D70`** - the gate, the `0x400` phase step, the `4.0` amplitude and
  the `1/256` pre-scale are literal; the two preludes `0x80B07880` and
  `0x80B07250` are untranslated and their internal behaviour is unknown.

## Deliberate skips

- **The other 12 untranslated `0x80B00000` rows.** `0x80B04E90` (1592),
  `0x80B0C900` (1140), `0x80B0D6E0` (696), `0x80B0D4D0` (520), `0x80B04A90`
  (512), `0x80B058A0` (476), `0x80B05740` (328), `0x80B0CD80` (292),
  `0x80B0CEB0` (288), `0x80B05A80` (180), `0x80B0DE10` (180) and `0x80B05B80`
  (172) are the matrix/render and approach bodies of the same family.
  `0x80B0CD80` and `0x80B0CEB0` in particular are built on
  `0x801651F0`, `0x80169F80`, `0x80168FE0`, `0x80166BD0`, `0x80166F10` and
  `0x80168FE0` - six engine calls whose argument order needs its own
  verification pass; guessing them would be worse than leaving the rows open.
  `0x80B0DE10` is held back with them because it is a pure chain of four of
  those siblings (`0x80B0D4D0`, `0x80B0C890`, `0x80B0D150`, `0x80B0CFD0`) with
  two of them still untranslated.
- **The wave-141 qr3 pool remainder.** `0x80AFDD50` (300), `0x80AB1900` (256),
  `0x80B07D70` (252 - taken here), `0x80A79630` (236), `0x80ABAB50` (236),
  `0x80A40290` (232), `0x80A20810` (228), `0x80A7ADA0` (220), `0x80AA3F50`
  (200), `0x80A7C600` (184), `0x80B23480` (180), `0x80AB21D0` (176),
  `0x80AFD170` (156), `0x80ADDAC0` (124) and `0x80ADDA40` (120) remain open;
  only the one that falls inside the owned `0x80B00000` bucket was taken.
- **The four non-qr3 class-D rows** `0x80A79810` (360), `0x80AEC4E0` (292),
  `0x80AEEFA0` (292) and `0x80A337B0` (4) are unchanged: the first three read an
  `extraout_` register the decompiler never defines and the last is a `b` alias,
  exactly as `docs/SPEC_WAVE141_ENEMIES.md` recorded.

No address that already had a `ledger/functions.csv` row or a `fn_<addr>` body
in `src/game/enemies.rs` was re-claimed.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the 17 sizes sum to 7688
  bytes (6500 for the twelve specification rows, 1188 for the five bucket rows).
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; no address appears twice, and none of the 17 was in
  `ledger/functions.csv` before this pass.
- All 17 bodies were decoded word-by-word from the ELF and cross-checked against
  the Ghidra export; the eight specification corrections above were re-verified
  against both sources, and every data-section constant cited (including the
  `{256, 192, ...}` / `{640, 544, ...}` s16 tables and the `1e9` search seed) was
  read out of `work/rels/elf/d_enemiesNP.elf`.
- The module header was updated: cluster item 124, the cluster count (123 to
  124), the function totals (3300 to 3317 public translations, 3229 to 3246
  `fn_<addr>` bodies), the "start here" list and the records list.
- `cargo check --message-format=short` exits `0` with no error and no warning
  naming `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave142` reports 17 rows for this lane
  (`translated/high=14`, `translated/medium=3`), 17 unique addresses and `OK`.
