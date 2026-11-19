# Coverage - Wave 156 (`game::enemies`, REL `d_enemiesNP`)

## How to read this file

This page records the Wave 156 `game::enemies` lane: five of the eight large
`d_enemiesNP` bodies that wave 155 left unclaimed, appended to
`src/game/enemies.rs` as section 138 and mirrored one row per body in
`ledger/agents/wave156-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none
of these is a runtime address; every function doc repeats that note. Raw words
were decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` program
headers as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment). Every
body was followed from its entry to its terminal `blr`; byte counts are the
`size` column of `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

Relative branches into `main.dol` are cited at their plain `main.dol` address
(`0x800690C0`, `0x800DF180`, `0x8015F340`, `0x801611C0`, `0x802DCBC0`, ...),
which is also the value the Ghidra exports and the existing translations use.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 5 | 5600 | 1 | 4 |

Status split: 5 `translated`, no `partial` and no `stub`. Covered address
range: `0x80A10880..0x80B04E90`.

## What this pass covers

- **`0x80A22820` - `staffRollBlock` model/clip binder.** The straight-line
  binder: the `0x800690C0` holder reset, the two `0x800DF180` model
  allocations (`g3d/staffRollBlock.brres` + `staffRollBlock`,
  `g3d/block_broken.brres` + `block_broken`), the three 14-pointer arrays at
  `0x80B398B8`/`0x80B398F0`/`0x80B39928` that supply one model, one
  `AnmTexSrt` and one `AnmTexPat` name per variant, and the
  `0x80169D90`/`0x80167420`/`0x80167800`/`0x80169F80`/`0x80167B20`/
  `0x80166830`/`0x80166BD0` chain that installs both elements. The variant is
  `q = (+0x7FC >> 4)` (signed divide by 16), and the `0x80167B20` frame is the
  compiler's `f32` conversion of `16 * (low nibble) - 15 * bit30` of `+0x7FC`,
  decoded from the `rlwinm`/`subf`/`xoris 0x8000`/`lfd`/`fsubs` sequence rather
  than from the export's `GQR0` noise.
- **`0x80A10880` - segmented-trail frame step.** Side speed from
  `0x80B35230` (halved when the `+0x544` slot `+0x2C` returns zero), the
  `0x80072BC0`/`0x8006FA40` wall tests, the `0x92C` ring's `0x801611C0`
  angle with the `side == 1` mirror, the `0x80075270`/`0x80075560` ground probe
  with its `0x934`/`0x935` transition block (motion block with `6500.0f` and a
  `{1,1,1}` scale, `0x283` sound, `0x80078410` puff), and the `+0x540`-node
  walk that emits one interpolated point per node at `29.0 * (i + 1)` along the
  `+0x928` ring of `0x1C`-byte records.
- **`0x80A92F40` - player-anchored chaser step.** Player resolution through
  `0x8005F900`/`0x8005FA60` (selected by `+0x70C`), the two early-out clip
  paths, the `1/64` scale ramp clamped to `1.25`, the `+0xEC` ceiling test with
  the `(y as i32) & !0xF` mask idiom, the `0x60`/`0x02` contact snaps, the
  `+0xEA8`/`+0xEA4` facing quartet, the `0x80B2EB2C`/`0x80B2EB30` turn steps
  with the `0x200`/`0x400`/`0x800` bounds, the stage and player-manager x/y
  clamps and the `+0x548` child's slot `+0x1C`.
- **`0x80B04E90` - stage-section polyline direction buffer.** The two passes
  over the section's 16-byte points (packed `+4` word: section nibble, start
  index, point offset), `0x802DE514` (`abs16`) run-length accumulation into
  `+0x2B20`, the `0x802B9250(total + 2)` allocation into `+0x2B1C` and the
  per-step pair/excess fill with direction bytes `0`/`1`/`2`/`3`/`4`, including
  the NUL at both ends.
- **`0x80A9B3C0` - four-slot shot-record builder.** The `+0xC88` s8
  arm/idle/advance state, the `0x80B2EFDC` launch angle, the `0x20` or
  `(s8)0x800B2EE0(6) * 0x20 + 0x10` countdown (bit 28 of `this+4`), the
  `+0xC50 = 0x40` pre-timer, the shot point `(x, y + 8 + 16*slot, z - 16)`, the
  `+0xC83`/`+0x318` reach test against the stage region, the `0x80A9AF80` hit
  query, the `0x80AAA360` factory with its `+0xA04` record store and `0x1BD`
  sound, and both `0x802DCBC0` keyframe scans (up through `+0xC81`, then down
  from `slot`) against the `{0, -1, 0x80A9B870}` patterns at
  `0x80B51CC8`/`0x80B51CD4`/`0x80B51CE0`/`0x80B51CEC`.

### `qr3` sites

None. Every `psq` word in these five bodies is a `qr0` `f31`/`f30`/`f28`..`f31`
prologue spill; no body reads or writes a quantised pair, so
`GQR3 = 0x00050005` is not exercised by this pass.

### Stream details this pass had to settle

1. **`0x80A22820`'s frame word is not the quotient.** The export renders the
   `+0x7FC` conversion as a `GQR0` parameter block. The raw words are
   `rlwinm 28,0,3` (low nibble to bits 28-31), `rlwinm 1,31,31` (bit 30),
   `subf`, `rlwinm 4,0,31` (rotate back), `add`, `xoris 0x8000`, `lfd`,
   `fsubs f31,f0,f1` with `f1` = `0x80B2B7D0` = `2^52 + 2^31`, i.e. the
   `(f32)(s32)` conversion of `16 * nibble - 15 * bit30`. The quotient itself is
   the separate `srawi 4`/`addze` pair used only as the table index.
2. **`0x80A92F40`'s float fields are `int`-typed in the export.** `piVar4[0x2b]`,
   `[0x2c]`, `[0x37]`..`[0x3b]`, `[0x3d]`, `[0x45]`..`[0x47]` are
   `f32[+0xAC]`, `f32[+0xB0]`, `f32[+0xDC]`..`f32[+0xEC]`, `f32[+0xF4]` and
   `f32[+0x114]`..`f32[+0x11C]`; the body uses those types.
3. **`0x80A92F40`'s y-mask arm re-converts.** The `(u32)y & 0xFFFFFFF0` value
   is produced by `fctiwz` (truncation) and then the `0x43300000`/`xoris`
   signed conversion back to `f32`, so the port is `(y as i32) & !0xF` cast to
   `f32`, not a bit-AND on the float.
4. **`0x80B04E90` dereferences address `0x3C` when the section is absent.** When
   `*(u32*)(section + 0x2C) == 0` the code clears its section pointer and then
   still loads `0x3C(r4)`; the port keeps the address computation and says so
   in the body, rather than inventing a null guard.
5. **`0x80A9B3C0`'s initial `0.0f` z store is dead.** The shot arm stores
   `0.0f` at the point's z and then overwrites it with `z - 16.0` before the
   `0x80AAA360` call; only the second store is observable, so the port starts
   from it (and documents the removed store).
6. **`0x80A9B3C0`'s mask byte is loaded once.** The export reuses `uVar9` for
   `countLeadingZeros(result)` inside the scans; the raw keeps the `+0xC80`
   mask in `r7` and reloads nothing, so the port reads it once and keeps it.

### Deliberate skips

Three `functions.tsv` rows of this module still have no body. None was
attempted, and none is claimed:

| address | TSV size | why it is left |
|---|---:|---|
| `0x80A7BEC0` | 1464 | `walk_*` clip-sequence player with contact turn: 366 instructions, 23 direct calls, six vtable `bctrl`, and 24 state bytes (`+0xE9A`..`+0xEA8`). The branch structure is only partially reduced in `docs/SPEC_WAVE144_ENEMIES.md` (the specification itself marks the clip-name cursor arithmetic and the two pointer arrays as a sketch); a faithful port needs a full instruction-level walk of the five clip-name index terms and of the `0x80B73158` smoke block, which this pass did not have the budget to prove. |
| `0x80A3A4A0` | 2620 | Fire Snake 150-segment body manager: 950-line export, 655 instructions, per-segment paired-single prologue noise. Not attempted; a faithful port needs its own raw walk of the segment record layout. |
| `0x80A3DEA0` | 2940 | param-nibble constructor with a 16-state dispatch: 417-line export over the `+0xA7C`/`+0x718`/`+0x6E8` state block. Not attempted; the dispatch table and the sub-object layouts are not proved by any existing body. |

The module therefore stands at **3481 `fn_<addr>` bodies plus 71 descriptive
helpers = 3552 public translations**, which covers 3552 of the 3555
`functions.tsv` rows; the three rows above are the only uncovered entries.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A10880` | `fn_80a10880` | 940 | medium |
| `0x80A22820` | `fn_80a22820` | 700 | medium |
| `0x80A92F40` | `fn_80a92f40` | 1172 | medium |
| `0x80A9B3C0` | `fn_80a9b3c0` | 1196 | medium |
| `0x80B04E90` | `fn_80b04e90` | 1592 | high |

### Why four rows are `medium` and one is `high`

- **`0x80A22820`** - all 175 instructions decode, but the two
  `0x8000F1B0`/`0x8000F0F0` suffix builders and the nibble frame word are
  described from their use, and the export renders the whole prologue as
  quantiser arithmetic.
- **`0x80A10880`** - the `+0x544` sub-object and the `+0x77C` chain-node classes
  are unnamed, so both `+0x2C` vtable slots, the `+0x928`/`+0x92C` ring roles
  and the effect arguments are inferred from this body's staging only.
- **`0x80A92F40`** - all 293 instructions decode, but the four
  `0x8005E4F0`..`0x8005E520` player predicates, `0x80A922C0` and
  `0x80056CF0` are `main.dol`/module helpers whose contracts are read from this
  body's use.
- **`0x80A9B3C0`** - all 299 instructions decode and the five keyframe patterns
  were re-read from the image (`{0, -1, 0x80A9B870}`), but the
  `0x800B2EE0`/`0x802DCBC0`/`0x80A9B1B0`/`0x80A9AF80`/`0x80AAA360` contracts
  are inferred, and the slot argument comes from an indirect caller.
- **`0x80B04E90`** - `high`, because every operand is the object, a documented
  stage global or the section record: no float, no indirect call, no table.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the five sizes sum to
  5,600 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; none of the five addresses was in
  `ledger/functions.csv` or had a body before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The module header was updated: cluster item 138, the cluster count (137 to
  138), the function totals (3547 to 3552 public translations, 3476 to 3481
  `fn_<addr>` bodies), the "Wave 156 adds" sentence, and the coverage-doc
  records list (which was missing waves 154 and 155 as well).
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave156` reports 5 rows for this lane,
  5 unique addresses and `OK`.
