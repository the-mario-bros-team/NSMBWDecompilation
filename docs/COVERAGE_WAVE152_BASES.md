# Coverage - Wave 152 (`game::bases`)

## How to read this file

This page documents the Wave 152 pass over `src/game/bases.rs`, the semantic
slice of the `d_basesNP` REL (REL module 2). Every address is a **static
analysis address** under the canonical synthetic base **`0x80600000`**; the Wii
REL loader chooses the real load address, so these numbers identify positions
inside the REL image, not runtime addresses. Every `# Original` block in the
source carries the
`Synthetic base: 0x80600000; this is not a runtime address.` line.

"Confidence" is `high` when every guard, store, constant and call argument is
literal, and `medium` when a callee contract, a field role or a return-value
convention had to be named rather than resolved.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/bases.rs` | `game::bases` | 10 | 5,712 | 5 | 5 |

All 10 rows are `translated`; none is `partial`. The module header now reports
3,749 bodies (3,739 + 10). Two of the rows come from the advisory `0x807b`
planner bucket, which drops from 71 to 69 untranslated addresses; the eight
large spec rows lower the global backlog by their own count.

## What this pass covers

- **The eight "ready" rows of `docs/SPEC_WAVE151_BASES.md`** (5,292 bytes), each
  re-read word by word against `work/ghidra/rel-d_basesNP/export/*` and the raw
  instruction stream with `file_offset = (address - 0x80600000) + 0x1000`:

  | cluster | body |
  |---|---|
  | ring-filter window rebuild | `0x806C1040`: append at `+0x527 & 0x1F`, `divw` weights `cnt^2 - (cnt-i)^2` over the `+0x4E4` ring at stride 2, then a `+-3072` slew limiter |
  | attack/charge tick | `0x806C7C20`: scanner `0x806C7890`, speed table `0x807CB114`, `+-4.0` clamps, flash countdown `0x78`, `806C7A20` angle feed, section-4b clamp |
  | round-obstacle builder | `0x8070B550`: 40 floats into `+0x6F0..+0x78C`, sub-object `0x800B9050(+0x674, 48)`, descriptor `0x8080565C` patched to `12.15`, area `0x8008C3E0`/`0x8008C330` |
  | radius sweep | `0x8065DA00`: category-2 list walk, `0x802E8868` square root, effect `0x800925F0` for type `0x200` at `z = 5500.0`, handlers for `0x19B`/`0x33`/`0x69`/`0x68` |
  | scene bring-up | `0x807B87F0`: display lock off, twelve `0x80162B00` handles, `0x80429FDC` and `0x8042A1CC`, nine subsystem starts, three-way display fork |
  | two-pane orbiter | `0x80682AA0`: `qr3` u16 angle `x 1/256`, `cos`/`sin` radii `336.0`/`328.0`, two `0x801C0BD0` builds into `+0x7C`, panes `+0xA20`/`+0xA60` |
  | record-table spawner | `0x8068F220`: `0x8030A040 + index * 40`, screen-effect four-way arm, globals `0x80429D18..0x80429D28`, `0x80064680`, scales clamped to `512.0` then doubled |
  | four-slot sequence tick | `0x80638120`: allocator probe `0x900`, spawn arm (accumulate + latch + channel `+25`), despawn arm (`+0x8C0` delay, cue `150`/`151`), virtual `+0x6A8` |

- **Two `0x807b`-bucket bodies** (420 bytes):

  | cluster | body |
  |---|---|
  | four-slot selected-bit refresh | `0x807BE160`: early-out on `+0x22A`, clear bit 0 of `+0xBB` on the four children at `+0x210..+0x21C`, re-set one or three by `+0x22C - 1` and `+0x22B` |
  | frame-timer advance | `0x807BC660`: `-0.2` per frame on the current entry's `+0x2A0`, `1.0` hold, `+0x27C = +0x280`, holder `+0x26C` `+0x230 = index + 1`, virtual `+0x18` on `this + 0x20C` with BSS labels `0x80834178`/`0x80834238` |

## Where the binary differed from the wave-151 prose

1. **`0x806C7C20`'s section-4b limit is `-(raw + 128.0)`, not
   `(raw - 0x80000) / 4096`.** The spec quotes `addis r0,r3,0xfff8` /
   `xoris r0,r0,0x8000` but omits the `neg r3,r0` at `0x806C7E54`, which runs on
   the scaled halfword (`rlwinm r0,r3,12,0,19`) before the `addis`. The machine
   therefore builds `-(raw * 4096 + 0x80000)` and divides by `4096.0`
   (`0x807CB140`), i.e. exactly `-(raw + 128.0)`. Both formulas agree at
   `raw == 0` (`-128.0`), which is why the error survives a casual check.
2. **`0x8065DA00`'s `0x19B` gate is the inverse of the spec's Rust sketch.** The
   spec's code calls `0x8011EBB0` when the `+0x61C` chain *exists*; the prose one
   paragraph later says a zero `+0x60C` with a zero `+0x61C+0x18` *suppresses*
   the call, and the two contradict each other. The raw words are unambiguous:
   `addi r3,r0,1` at `0x8065DB78` makes the computed flag `+0x60C + 1`, the
   `bne` at `0x8065DB80` jumps straight to the test when `+0x60C != 0`, and the
   `beq` at `0x8065DBB4` skips the call when that flag is non-zero. The call
   therefore happens only when `+0x60C == 0` **and** the `+0x61C` chain is
   absent, which is what the Ghidra export prints (`if (!bVar5)`) and what this
   translation implements.
3. **`0x8065DA00`'s distance test is a branch, not a `<=`.** `fcmpo` + `bgt`
   skips only on a strictly greater ordered result, so an unordered (NaN)
   distance still enters the dispatch. The body keeps that with
   `if !(d > radius)`; a plain `d <= radius` would silently drop the NaN case.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80638120` | `fn_80638120` | 648 | high |
| `0x8065DA00` | `fn_8065da00` | 660 | medium |
| `0x80682AA0` | `fn_80682aa0` | 656 | medium |
| `0x8068F220` | `fn_8068f220` | 656 | medium |
| `0x806C1040` | `fn_806c1040` | 676 | high |
| `0x806C7C20` | `fn_806c7c20` | 668 | medium |
| `0x8070B550` | `fn_8070b550` | 668 | high |
| `0x807B87F0` | `fn_807b87f0` | 660 | high |
| `0x807BC660` | `fn_807bc660` | 220 | medium |
| `0x807BE160` | `fn_807be160` | 200 | high |

**Totals:** 10 bodies, 5,712 bytes. The address window runs from `0x80638120` to
`0x807BE160`; the bucket subset is `0x807BC660..0x807BE160`.

## Notes on inference

- `0x806C7C20` is `medium`: the `(sum, hit, flags)` contract of `0x806C7890` is
  taken from the spec's analysis of that callee, not from this body (which only
  passes three stack pointers). The speed table read is deliberately unbounded
  (`lfsx` with `count * 4`), and the limit conversion is the arithmetic
  re-derivation above.
- `0x8065DA00` is `medium`: the distance test, the type ladder, the effect call
  `0x800925F0(owner, &pos, id, 0xFF)` and both vtable chains
  (`+0x28` then `+0x14`) are literal; the per-type handlers are named by address
  only. The out pointer advances by 12 bytes even when the copy is skipped.
- `0x80682AA0` is `medium`: the `qr3` single-element load is decoded with the
  `GQR3 = 0x00050005` word pinned in `docs/REL_ELF_MAPPING.md` (type 5 = u16,
  scale 0), so the angle is exactly `((a3 + 0x4000) as u16) as f32 / 256`. The
  `cos`/`sin` pair `0x80237C40`/`0x80237BD0` is called through the `f64`
  prototype already used in `src/engine/display.rs`; what the three angle
  setters do with the staged `u16` is `// unknown:`.
- `0x8068F220` is `medium`: the record table `0x8030A040` is zero at file level
  and the `0x80064680` spawner is `main.dol` code, so the record contents and the
  spawn result are inputs. `record+0x14`/`+0x18` are read with `lwz` + `xoris` +
  `lfd`, so they are signed integers rather than floats.
- `0x807BC660` is `medium`: the `-0.2` step (`0x807D0A18`), the `1.0` hold
  (`0x807D0A10`), the index advance and the `lwzu r12,0x20C(r3)` receiver are
  literal, but the two message labels `0x80834178`/`0x80834238` are runtime-built
  REL BSS (empty at file level), so only their role as vtable arguments is
  provable.
- Confirmations rather than corrections: `0x8070B550`'s dead 36-float stack copy
  is present in the raw words and is omitted with a note; `0x80682AA0`'s second
  `0x801C0BD0` build is not hoisted; `0x80638120`'s allocator probe is a virtual
  call with `0x900` made on every frame, not a cached pointer test.
- `0x807BE160` is `high` and integer-only: the stored bit is bit 0 of each
  child's `+0xBB`, the index test is `(u32 at +0x22C) - 1 == 8` and both `+22A`
  / `+22B` gates are literal bytes.

## Deliberate skips

- The rest of the `0x807b` planner bucket was not started (the pass budget went
  to the eight 648-676-byte spec bodies plus two small bucket bodies). By this
  lane's TSV + ledger + `fn_`-token filter the largest remaining rows are
  `0x807BAC40` (584), `0x807BBE20` (584), `0x807BE750` (580), `0x807B2550` (548)
  and `0x807B7B40` (524).
- `0x807B8A90` (244 bytes) was decoded and left open: it reads the stage table,
  falls back to a null stage and then dereferences `stage + 0xC`, so the faithful
  body would contain a null-page read whose role cannot be explained. It is not
  claimed.
- `0x807BA440` (204 bytes) was left open: it formats two strings through
  `0x802E17DC` with the EABI varargs FP-flag clear (`crxor 6,6,6`) and the
  format/argument contract of that call is not provable from the call site.
- `0x807BAB20` (280 bytes) was left open: its second half is a `mtspr ctr` +
  `bdnz` scan over a `u16` array with an indirect loop count, and the array
  source (`+0x28` of the stage record) is not resolved.
- Nothing was written from `docs/SPEC_WAVE151_BASES.md` that the raw words did
  not support; the two deviations above are implemented as the binary behaves,
  not as the prose describes.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_basesNP/export/functions.tsv` (10 rows, 5,712 bytes),
  re-read from the TSV for this pass rather than from prose.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_basesNP/export/`, named `<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/bases.rs`, and every `# Original` block carries the
  `Synthetic base: 0x80600000; this is not a runtime address.` line.
- No address was already in `ledger/functions.csv`: all 10 were checked against
  the ledger's address column before being written, and the lane validator
  repeats that check.
- `node tools/check-lane-csv.mjs wave152` reports `OK` (10 rows for
  `wave152-bases.csv`, 28 unique addresses across the wave).
- `cargo check --message-format=short` reports no diagnostic naming
  `src/game/bases.rs`.
- All 10 bodies were decoded from the raw words with
  `file_offset = (address - 0x80600000) + 0x1000` and every literal was read out
  of `work/rels/elf/d_basesNP.elf`.
