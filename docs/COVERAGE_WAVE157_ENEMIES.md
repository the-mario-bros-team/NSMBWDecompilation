# Coverage - Wave 157 (`game::enemies`, REL `d_enemiesNP`)

## How to read this file

This page records the Wave 157 `game::enemies` lane, which closed the last three
untranslated bodies of `d_enemiesNP` and therefore the module itself. The three
functions are appended to `src/game/enemies.rs` as section 139 and mirrored one
row per body in `ledger/agents/wave157-enemies.csv`.

Every `address` is a **synthetic analysis address** produced by
`tools/rel2elf.mjs` (base `0x80A00000`). A REL stores no load address, so none of
these is a runtime address; every function doc repeats that note. Raw words were
decoded from `work/rels/elf/d_enemiesNP.elf` through the `PT_LOAD` program
headers as described in `docs/REL_ELF_MAPPING.md`
(`file_offset = address - 0x80A00000 + 0x1000` for the first segment). Every
body was followed from its entry to its terminal `blr`/tail call; the byte counts
below are the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

Table and callee addresses outside this REL are cited at their plain `main.dol`
or `d_basesNP` analysis address (`0x80077810`, `0x8064F990`, `0x803537F0`, ...),
which is also the value the Ghidra exports and the existing translations use.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 3 | 7024 | 0 | 3 |

Status split: 3 `translated`, no `partial` and no `stub`. Covered address
range: `0x80A3A4A0..0x80A7BEC0`.

With this pass the module has **3555 public translations** (`3484` of them
`fn_<addr>` bodies plus the 71 descriptive helpers) against 3555
`functions.tsv` rows, i.e. **3555/3555**.

## What this pass covers

- **`0x80A3A4A0` - Fire Snake 150-segment body manager.** The four-player pass
  that writes one `0x8015F140` distance threshold per slot into `+0x7AF0[4]`
  (three constant quartets selected by the `0x53`/`0x2B`/`0xC`/`0x2E`/`0x29`
  state probes), the `+0x7B04` order counter and the 150-iteration segment
  machine: the state-1 seed (random length from `0x800B2F10`, `+0x3490`,
  `+0x3C60`, `+0x1550 = 0x96`, the two colour quads and a `0x800B2EE0(0x10000)`
  angle), the shared build step, the state-2 shrink, the state-3 colour ramp
  and the state-10 launch offset, plus the nearest-of-four-player test that
  flips a segment into state 10. The tail commits `0x801C0BD0`/`0x8016A170`
  and resets the `+0x7B00` band.
- **`0x80A3DEA0` - sixteen-case constructor / state dispatcher.** The kind-word
  decode (`+0x718`, `+0xA7C`, `+0x71C`, `+0x348`, `+0xA80`, `+0xA95`, `+0xA99`),
  the home-position snapshot, the `0x80064680(0x178, ...)` chain offset over
  `0x801625B0`/`0x801625D0`, the ground snap at `+0xB4`, the stage-owner
  release/claim pair (`0x80161800`/`0x80161750`, `0x80078520`,
  `0x80014820`/`0x800146B0`), the `0x80027310` kind apply, the two callback
  triples at `+0x6D0..+0x6D8` and `+0x574..+0x57C`, and the dispatch through
  the shipped jump table at `0x80B3EE94` (sixteen targets, all inside the body)
  that installs one of four `main.dol` state records or one of eleven
  `0x80B70xxx` records, including the two-stage `0x80077810` spawn probe of
  form 15.
- **`0x80A7BEC0` - `0xEB0` walker clip-sequence player.** `0x802DCD54` +
  [`fn_80a79df0`], the `+0xE9A`/`+0xEA0` entry latch with its `+0xE92` step,
  the landing sound `0x1E2` + `Wm_en_landsmoke_s` block, the pair/row ladder
  (`+0xEA1` toggle, `+0xEA2`/`+0xEA3` swap, `+0x348` flip), the
  `walk_st_*`/`walk_ed_*` clip tables at `0x80B2DE58` (36 eight-byte
  `{start, end}` records selected by `side*18 + pair*9 + row*3 + row`) and the
  `0x80B2DE44` timing array, the `0x80B2DE38` collision-box bit, and the
  advance/end arms that rebind clips through `0x8023A0B0`,
  `0x801651F0`, `0x80169F80`, `0x801680E0` and `0x80168FE0`.

### `qr3` sites

`0x80A3A4A0` contains the module's last `qr3` reads: four `psq_l f0, d(r1), 3, 1`
words at `0x80A3A954`, `0x80A3A9B0`, `0x80A3A9CC` and `0x80A3A9EC`. With
`GQR3 = 0x00050005` (type 5 = unsigned 16-bit, scale 0) and `W = 1` they widen a
halfword to `f32`, so they are rendered `(f32)(u16)` and the body keeps the
`1/256` turn unit of `0x80B2C36C` for the sine/cosine callees. The other 36 `psq`
words in that body are the `f14..f31` `psq_st`/`psq_l` save/restore pairs of the
544-byte frame (`W = 1, I = 0`), i.e. plain single-float spills; they are elided
and the frame is modelled by locals. `0x80A3DEA0` and `0x80A7BEC0` have no `qr3`
site at all (one GQR0 `f31` spill each).

### Stream details this pass had to settle

1. **`0x80A3A4A0`'s segment-order gate is `>=`, not `<=`.** The export prints
   `iVar22 <= *(iVar11 + 0x7b04)` for the state-0 arm; the raw is
   `cmp r17,r0` / `bc 12,1` (branch when `r17 < r0`), so the port uses
   `index >= counter`.
2. **`0x80A3A4A0`'s colour bytes are adds, not constants.** The export renders
   the `+0x24F0` quad as the literals `199`/`207`; the raw is
   `lbz r0,3(r21)` / `addi r0,r0,199` / `addi r0,r0,207`, so the port keeps the
   `+0x24F3` byte as the base of both ramps. The same applies to the `+0x1D20`
   ramp: `cmpli r0,1` / `bc 12,1` keeps `255,255,255` only when the adjusted
   byte is exactly `0` (form `b == 2`), not for the `< 2` window the export
   prints.
3. **`0x80A3DEA0`'s placement flag is `form != 1`.** `addi r0,r3,-1` /
   `cmpli r0,1` / `bc 12,1` sets `r4 = 0` only for `form == 1`; Ghidra's
   `form - 1U < 2` would also fire for `form == 2`.
4. **`0x80A3DEA0`'s `+0x1550` tests are signed `>= 0`.** The export prints
   `< 1`; the raw is `cmpi r4,0` / `bc 12,1` (skip when negative), so both arm
   and decrement use `i32` reads.
5. **`0x80A3DEA0`'s `+0x690` value is a load, not a symbol.** The export prints
   `DAT_8042bb19`; the raw is `lbzx r0, (0x8042BB18 + 1)` and the branch only
   runs when `+0xA80 == 1`, so `0x8042BB19` is exactly the byte read.
6. **`0x80A7BEC0`'s start-clip blend is `45.0`, not `40.0`.** The raw loads
   `f28 = *(0x80B2DE40)` (45.0) for the two `0x80A7A2C0`/`0x80A7A360` calls and
   only points `+0x618` at the `40.0` entry `0x80B2DE44`.
7. **`0x80A7BEC0` sets `+0xE9B` and `+0xE9C` from `+0xEA5`.** Both stores use
   `r4 = *(0xEA5)`; the export printed `0` for `+0xE9C`.
8. **`0x80A7BEC0`'s record index is the same in both blocks.** The first block
   computes `side*18 + pair*9 + old(+0xEA3)*3 + old(+0xEA2)` and the end arm
   `side*18 + pair*9 + (+0xEA2)*3 + (+0xEA3)`; because the byte swap happens
   first, both select the same eight-byte record, which the port states once.

### Blockers checked and dismissed

The two blockers quoted for these addresses in earlier waves were re-tested
against the raw stream and do not hold, confirming `AUDIT_WAVE156_ENEMIES.md`:

- `0x80A3A4A0` has **no** GQR3/GQR4 arithmetic: its 40 `psq` words are 18
  `f14..f31` spill pairs plus the four `qr3` halfword reads listed above.
- `0x80A7BEC0`'s 24 "unreachable" export lines are the `f28..f31` prologue and
  epilogue spill pairs; the `GQR0` blocks are those spills, not quantisation.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80A3A4A0` | `fn_80a3a4a0` | 2620 | medium |
| `0x80A3DEA0` | `fn_80a3dea0` | 2940 | medium |
| `0x80A7BEC0` | `fn_80a7bec0` | 1464 | medium |

### Why all three rows are `medium`

- **`0x80A3A4A0`** - all 655 instructions decode and every table, constant and
  call is literal, but the roles of `0x801C1650`/`0x802BE470` (a probe point and
  its distance) and of the `+0x3490`/`+0x3C60`/`+0x5B0` floats are inferred from
  this body's staging, and the four `0x8015F140` quartets are cited by data
  address because their meaning is unknown.
- **`0x80A3DEA0`** - all 735 instructions decode and the jump table and state
  records were re-read from the image, but the two `0x80077810` probes, the
  `0x80078520` region handle and the stacked `0x80B2C4F0`/`0x80B2C4FC`/
  `0x80B2C508` tables are read from their use only.
- **`0x80A7BEC0`** - all 366 instructions decode and both clip tables plus the
  effect/sound blocks were re-read, but the meaning of the `+0xE92` step and of
  the `+0xEA0`/`+0xEA1`/`+0xEA2`/`0xEA3` ladder is inferred from this body, and
  the `+0x618` end marker doubles as the address of `0.25f`.

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`; the three sizes sum to
  7,024 bytes.
- Every evidence file exists at the cited repo-relative path
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`; none of the three addresses was in
  `ledger/functions.csv` or had a body before this pass.
- Every `# Original` block carries the `Synthetic base: 0x80A00000` line.
- The module header was updated: cluster item 139, the cluster count (138 to
  139), the function totals (3552 to 3555 public translations, 3481 to 3484
  `fn_<addr>` bodies), the "Wave 157 closes the module" sentence, the
  "every `functions.tsv` row has a body" note and the coverage-doc list.
- `cargo check --message-format=short` reports no error or warning naming
  `src/game/enemies.rs`.
- `node tools/check-lane-csv.mjs wave157` reports 3 rows for this CSV, 3 unique
  addresses and `OK`.
