# Coverage - Wave 131 (`game::bases`)

## How to read this file

This page documents the Wave 131 pass over `src/game/bases.rs`, the semantic slice
of the `d_basesNP` REL. Every address is a **static analysis address** under the
canonical synthetic base **`0x80600000`** produced by `tools/rel2elf.mjs`; the
Wii REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the same warning line. Cross-module pointers (`0x8035xxxx`,
`0x8042xxxx`) keep their absolute `main.dol` values; this REL's own `.bss`
message cells keep their `0x8082xxxx`/`0x8083xxxx` synthetic values.

Every cited file is a real Ghidra C export under
`work/ghidra/rel-d_basesNP/export/`, and every byte count was re-read from
`work/ghidra/rel-d_basesNP/export/functions.tsv` for this pass. All 26 bodies end
in a `blr` in the last word of their TSV extent, and every internal branch target
is inside that extent; the only out-of-extent branches are the direct `bl` calls
listed per body.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee identity, a field role or a `.bss` table's
element meaning had to be inferred. Comments in the bodies keep `// observed:`
/ `// inferred:` / `// unknown:` markers.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 26 | 8,908 | 20 | 6 |

All 26 rows are `translated`; none is `partial`. The module header now reports
3333 bodies (3307 + 26).

## What this pass covers

- **The nine "ready" class-B bodies from `docs/SPEC_WAVE130_BASES.md`
  (5,616 bytes)** plus the three "conditional" ones (1,376 bytes) that the same
  specification proved implementable with one inferred role each; together the
  whole twelve-address batch, 6,992 bytes:

  | cluster | bodies |
  |---|---|
  | per-player overlays | `0x8072FE10` cross-hair box drawer (two lazy `.bss` matrices, two id tables) and `0x80642A20` four-player model refresh (mode-6 vs normal arm) |
  | Lemmy attack effects | `0x80673F50` 16-pixel grid burst emitter (two 16-bit toggles, `Wm_en_burst_ss`) and `0x806B5180` stomp-actor initialiser (nine virtual calls + the 24-float static block) |
  | carried/trail actors | `0x8070D790` trail step (speed ramp, contact vector, snap-back) and `0x80740370` ring-buffer trail builder |
  | menu/actor gates | `0x807247D0` menu-gated facing flip and `0x80748B20` score-popup placer |
  | geometry probes | `0x80781DB0` horizontal cosine probe (`qr0` paired-single subtract) and `0x80780FA0` enemy-distance blend |
  | sequences | `0x806FDD50` six-state timer machine and `0x80647E60` packed-descriptor spawner |

- **The `dMiniGameCannon_c` player-select screen from the `0x80610000` planner
  bucket (14 bodies, 1,916 bytes).** `0x8061EC00` pane-transform copy per active
  player, `0x8061EE50` occupancy probe, `0x8061EF20` pane show/reset,
  `0x8061F030`/`0x8061F130`/`0x8061F6F0` layout-finished message arms,
  `0x8061F260` four-slot row refresh, `0x8061F500` three-way message select,
  `0x8061F620` row state applied to the models, and `0x8061F750`/`0x8061F790`/
  `0x8061F840`/`0x8061F880`/`0x8061F8C0` animation start/stop and row re-bind
  helpers.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x8072FE10` | `fn_8072fe10` | 684 | high |
| `0x80642A20` | `fn_80642a20` | 660 | medium |
| `0x80673F50` | `fn_80673f50` | 656 | high |
| `0x806B5180` | `fn_806b5180` | 620 | high |
| `0x8070D790` | `fn_8070d790` | 584 | high |
| `0x807247D0` | `fn_807247d0` | 584 | medium |
| `0x80781DB0` | `fn_80781db0` | 564 | high |
| `0x80740370` | `fn_80740370` | 552 | medium |
| `0x806FDD50` | `fn_806fdd50` | 536 | high |
| `0x80748B20` | `fn_80748b20` | 520 | medium |
| `0x80780FA0` | `fn_80780fa0` | 520 | high |
| `0x80647E60` | `fn_80647e60` | 512 | high |
| `0x8061EC00` | `fn_8061ec00` | 360 | medium |
| `0x8061EE50` | `fn_8061ee50` | 92 | high |
| `0x8061EF20` | `fn_8061ef20` | 268 | high |
| `0x8061F030` | `fn_8061f030` | 88 | high |
| `0x8061F130` | `fn_8061f130` | 96 | high |
| `0x8061F260` | `fn_8061f260` | 140 | high |
| `0x8061F500` | `fn_8061f500` | 180 | high |
| `0x8061F620` | `fn_8061f620` | 200 | medium |
| `0x8061F6F0` | `fn_8061f6f0` | 84 | high |
| `0x8061F750` | `fn_8061f750` | 56 | high |
| `0x8061F790` | `fn_8061f790` | 128 | high |
| `0x8061F840` | `fn_8061f840` | 60 | high |
| `0x8061F880` | `fn_8061f880` | 64 | high |
| `0x8061F8C0` | `fn_8061f8c0` | 100 | high |

**Totals:** 26 bodies, 8,908 bytes (20 high, 6 medium). The address window runs
from `0x8061EC00` to `0x80781DB0` in four clusters (`0x8061`, `0x8064`, `0x806B`/
`0x806F`, `0x8070`/`0x8072`/`0x8074`/`0x8078`).

## Where the instruction stream corrects the specification

These are the places where the wave-130 prose disagreed with the shipped bytes;
the bodies follow the bytes and say so at the call site:

1. **`0x807247D0` - the probe gate is not "C == 0 && D == 0 && E != 0".**
   `0x807248A8` initialises `r28 = 1`; it is cleared only at `0x8072495C`, when
   the `C`/`D` pair already answered zero *and* probe `E` also answers zero
   (`0x8072491C` skips probe `E` when `r27` says `C` or `D` was nonzero, leaving
   `r28 = 1`). The flip therefore happens unless all three probes answer zero.
2. **`0x80780FA0` - the callee's `f1` is `0.25`, not `1.0`.** Ghidra prints
   `FUN_80780A40(1.0, this)` because it lost the `fsub f31,f1,f0` at
   `0x80781028`; the instruction stream passes `f31 = 1.0 - 0.75`.
3. **`0x80673F50` - the flag arm's y term is signed.** `-(u16)y` is built with
   `neg` + `xoris ...,0x8000`, so the `0x4330...` conversion reads the negated
   16-bit value, not the 16-bit pattern (`8.0 - (f32)(u16)y`); the body keeps the
   `0u32.wrapping_sub` form so the transcription stays literal.
4. **`0x80647E60` - the direction byte offset is `((packed >> 26) & 7) * 4`.**
   `rlwinm r0,r30,6,27,29` extracts bits 28..30 as a 4-byte-scaled offset, which
   is numerically identical to the specification's `((packed >> 26) & 7) * 4`;
   both forms are recorded in the body.

## Deliberate skips

Read and rejected, with the reason:

- **The `0x802B9280` array-destroy stubs.** `0x80612C50`, `0x80612C90`,
  `0x80614430`, `0x80615560` and `0x8061AFF0` are the identical
  `if (ptr != 0 && count > 0) bl 0x802B9280; return ptr;` stub. The callee sets
  `r4 = 0` and jumps to `0x802B8F70`, so the incoming count is discarded and the
  real element destructor is not visible in this REL; claiming them would invent
  the helper's contract.
- **`0x8061B890` (628 bytes).** The body round-trips ten `0x2A`-byte player
  records through `0x800CE320`/`0x800CE2C0` and then writes four interleaved
  `0x80354E90`/`0x80354EA0` arrays; the record element roles are not recoverable
  from this REL, and a literal transcription would be a wall of unverified
  `.bss` offsets.
- **`0x80613680` (596 bytes).** A four-slot `+0x27C` state machine over
  `0x807C6AD0` with `0x800C9490`/`0x800C92F0`/`0x800C9610` layout calls; the
  per-slot `+0x268`/`+0x29C` fields have no visible type, so it stays for a pass
  that can pin the NW4R layout record.
- **The rest of the `0x80610000` bucket.** 78 of the bucket's 92 untranslated
  addresses remain: the five array-destroy stubs above, the two large
  unrecoverable bodies above, and 71 addresses not attempted this pass. The
  planner bucket is advisory and this lane prioritised the specified class-B
  batch and a coherent cluster over padding.

## Notes on inference

- [`fn_80642a20`] is `medium`: `0x80354E60` (player -> object id), `0x80354E70`
  (per-player current index) and `0x80354E80` (per-player claimed flag) are
  main.dol `.bss`, zero at load; all three are read/written literally and carry
  `// unknown:` notes.
- [`fn_807247d0`] is `medium`: `0x807CD1C0` is only a coherent four-entry table
  (1.0, -1.0, -0.1875, -4.5); the body reproduces the raw
  `read_f32(0x807CD1C0 + (slot & 0xFF) * 4)` and says so.
- [`fn_80740370`] is `medium`: the two 64-halfword rings at `+0x74C`/`+0x76C`
  and the `u8` cursor at `+0x7EC` have no visible element type; the `-2`-stride
  mirror fill is transcribed exactly (index `& 0x3F`, then `* 2` for the byte
  offset) rather than modelled as an array.
- [`fn_80748b20`] is `medium`: the `+0x38`/`+0x3C`/`+0x40` lanes of
  `[0x80429E00]` and the `+0xB0`/`+0xD4` lanes of the `0x80064740` actor are
  exact offsets whose anchor/bounding-box roles are not visible here.
- [`fn_8061ec00`] is `medium`: `0x8061EBD0` maps a source slot to a pane index
  (`0x12` = none) and the `+0x248` panels / `+0x280` holders are NW4R panes; the
  copied lanes are literal, the element roles are inferred.
- [`fn_8061f620`] is `medium`: `0x800B4730`'s second argument (the object id) and
  the `id + 3` frame passed to `0x800C92F0` are read from the same player table
  the rest of the cluster uses; both are literal at the call site.
- New host-side helper added by this pass: `draw_screen_line` (the five-argument
  `0x80077860` form with the caller's own `& 0xFFFF` truncations).
- The `unaff_GQR0`/`ldexpf` blocks printed at the head and tail of `0x80673F50`,
  `0x806B5180`, `0x80781DB0`, `0x80740370`, `0x80748B20`, `0x80780FA0` and
  `0x8061EC00` are the prologue/epilogue spill noise described in
  `docs/REL_ELF_MAPPING.md`: every `psq_l`/`psq_st` is a `qr0` `f29..f31`
  save/restore paired with `stfd`/`lfd`, except `0x80781DB0` where the `ps_sub`
  triple is genuine `qr0` data-side arithmetic. No body uses `qr3` or
  `mtspr GQRn`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (26 rows, 8,908 bytes),
  re-read from the TSV rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 26 were checked against
  the ledger's address column before being written, and none is duplicated.
- `node tools/check-lane-csv.mjs wave131` reports `OK`;
  `wave131-bases.csv` has 26 rows and 26 unique addresses.
- `cargo check --message-format=short` reports no error and no warning naming
  `src/game/bases.rs`.
