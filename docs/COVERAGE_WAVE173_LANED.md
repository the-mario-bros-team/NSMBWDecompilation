# Coverage - Wave 173 lane D (`game::bases` 360-364 byte sub-band)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` (lane D block) | `game::bases` | 4 | 1,444 | 3 | 1 |

Fourth batch from the 360-byte band; all four bodies are in the 360-364 byte
sub-band, and three of them (`0x8074C3D0`, `0x8074DBA0`, `0x8079F790`) are
exactly 360 bytes. Source of the bodies: `work/wave173/laneD_block.rs`.

## What this pass covers

- **Swept-box actor hit test (1 body).** `0x80699CC0` measures the X/Y delta
  between the actor's current position (`+0xAC`/`+0xB0`, delta source
  `+0xE8`/`+0xEC`) and a caller-supplied previous position, scales it by `20.0`
  and clamps it to `min(delta, 0)` / `max(0, delta)` to build a growing
  axis-aligned window (`36.0` tall, `24.0 * 0.5` wide) around the actor. The
  point must be strictly inside both windows.
- **Player-pane portrait placement (1 body).** `0x8074C3D0` draws sprite `159`
  at `(8.0 + cam[+0x38] + cam[+0x40], row, 1500.0)`. The row is either
  `cam[+0x3C] - 16.0*index - 32.0` from a no-repeat `rand(8)` latch at `+0x418`
  or `8.0 + player[+0xB0]` when `panel[+0x430] != 0`. The `rand(2)` variant bit
  is drawn only when `player[+0x10C] > -0.5` and is packed into draw-arg bits
  12+; bit 31 is always set. The returned sprite handle is stored at
  `panel + slot*4 + 0x3D0`.
- **Horizontal-sweep sprite (1 body).** `0x8074DBA0` draws sprite `388` at
  `cam[+0x38] + 0.5*cam[+0x40] +/- 0.25*cam[+0x40]` (the half-height is cached
  in `f31` across the `0x8074D9D0` call), `(cam[+0x3C] - cam[+0x44]) - 16.0`.
  The depth comes from `0x8074DB30(actor)` (`4500.0 - 16.0*actor[+0x3D0]`) when
  the `0x80429D40` `+0x44` ceiling is set and within `128.0` of X, otherwise
  from `0x80068BC0(*(0x80429D48))`.
- **World-map pane republish (1 body).** `0x8079F790` walks the manager's four
  `+0x384` state words; state `3` sets `0x80354E50[i]` and refreshes
  `0x80354E70[0x80354E60[i]] = 0x807C3B60(player, i)`, other states clear the
  visibility word. It then stores the two ids at manager `+0x50`/`+0x54`,
  clears `+0x6D`, conditionally sets `+0x3A9`/`+0x395`, runs `0x800FD120`
  (which may rewrite both low bytes), `0x807BAF10`, and hands a 16-byte stack
  record to `0x800BB780`. Returns `1`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80699CC0` | `fn_80699cc0` | 364 | high |
| `0x8074C3D0` | `fn_8074c3d0` | 360 | high |
| `0x8074DBA0` | `fn_8074dba0` | 360 | medium |
| `0x8079F790` | `fn_8079f790` | 360 | high |

## Deferred

None.

## Corrections and notes

- A-form float decoding: `fmuls` reads `FRA * FRC` (bits 21-25), while
  `fadds`/`fsubs`/`fdivs` read `FRA +/- FRB` (bits 16-20). Generic decoders that
  print the `FRB` slot for every opcode 59 word misreport `fmuls`. Two cases in
  this lane matter: `0x8074C4BC` is `16.0 * index` (not `16.0 * 32.0`) in
  `fn_8074c3d0`, and `0x8074DBA0`'s `fmuls f31,f0,f1` is `0.5 * cam[+0x40]`
  (not `0.5 * 0.5`). Both match Ghidra once decoded correctly.
- `fn_80699cc0`'s clamp pair is `cror`-folded: the `bc 4,1` / `bc 4,0` around
  `fmr f6,f0` / `fmr f4,f0` / `fmr f5,f0` / `fmr f4,f0` are `min` / `max`, and
  Ghidra's reconstruction is correct. The `this+0xF0 - previous[2]` subtraction
  at `0x80699CF8` feeds a stack slot that is never read; the body omits it.
- `fn_8074c3d0`'s draw gate is `player[+0x10C] <= -0.5` (`cror` folds
  `LT|EQ`), which *suppresses* the `rand(2)` variant. Ghidra prints the
  equivalent `-0.5 < player[+0x10C]` form.
- `fn_8074dba0`'s tail does not set `r3`, so it effectively returns the leftover
  of `0x80064610`; the export types it `void` and so does this body.
- `fn_8079f790` returns `1`: `li r3,1` sits immediately before the
  `0x802DCDA0` restore helper, whose `r3` survives per the brief.
- The 16-byte stack record in `fn_8079f790` is only written at offsets `0x15`,
  `0x16`, `0x17`, `0x18..0x1B` and `0x1C..0x1F`; offsets `0x10..0x14` are never
  written by the shipped body and are kept at zero here.
- The coordinator's `work/wave173/merge.cjs` had already appended this block to
  `src/game/bases.rs` from an intermediate revision. The only differences from
  the final block are three doc-comment wordings (`fcmpo` -> `fcmpu` and the
  `0x80699CC0` argument/confidence paragraphs); no code differs. Per the lane
  rules this file was not edited to carry those doc fixes.

## Verification

- The lane-D block type-checks standalone against stubbed helper signatures
  (`rustc --crate-type lib --emit=metadata`).
- All four evidence exports exist under
  `work/ghidra/rel-d_basesNP/export/`; every constant, branch and field offset
  was read from the raw words of `work/rels/elf/d_basesNP.elf`.
