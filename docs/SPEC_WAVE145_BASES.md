# Wave 145 - `game::bases` - the next eight (0.9-1.0 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/table readers live under
`%TEMP%\opencode\w145` (`ppc.js`, `dump.js`, `summary.js`, `w145_scan5.js`),
never in the repository.

Earlier waves have already walked this bucket from the top; the last row claimed
by `docs/SPEC_WAVE144_BASES.md` is 1,040 bytes. This wave continues one rank
lower. The window is now 0.88-0.95 KB per body (7,448 bytes / 1,862
instructions), so every row gets a full field map, a numbered behaviour list and
a constant table.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80630820` | 972 | 243 | `80630820_FUN_80630820.c` | ready (medium) |
| 2 | `0x80797D30` | 960 | 240 | `80797d30_FUN_80797d30.c` | ready (medium) |
| 3 | `0x8077DC80` | 936 | 234 | `8077dc80_FUN_8077dc80.c` | ready (medium) |
| 4 | `0x8077A4B0` | 924 | 231 | `8077a4b0_FUN_8077a4b0.c` | ready (medium) |
| 5 | `0x8077C670` | 924 | 231 | `8077c670_FUN_8077c670.c` | ready (medium) |
| 6 | `0x80669560` | 916 | 229 | `80669560_FUN_80669560.c` | ready (high) |
| 7 | `0x8070FAB0` | 912 | 228 | `8070fab0_FUN_8070fab0.c` | ready (medium) |
| 8 | `0x807BFFA0` | 904 | 226 | `807bffa0_FUN_807bffa0.c` | ready (medium) |

**Ready: 8. Conditional: 0.**

Every `bytes` value is the `size` column of the TSV verbatim, `decompiled` is
`true` for all eight, and in all eight cases the size is the true instruction
span: the decoder found the final `blr` at `address + size - 4` in each body
(`0x80630BE8`, `0x807980EC`, `0x8077E024`, `0x8077A848`, `0x8077CA08`,
`0x806698F0`, `0x8070FE3C`, `0x807C0324`). There is no padding tail and no
`halt_baddata` cut this wave: **0** of the 1,862 words in the eight bodies is
undecodable, and no word falls outside the documented opcode set.

### Why none is conditional

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image, every callee is named by address (or by its ledger name), and
  the body can be written as code with no assumed calling convention beyond the
  standard PowerPC EABI. Data that lives in `main.dol` BSS (`0x8035xxxx`,
  `0x8037xxxx`, `0x8042xxxx`) or in this REL's own BSS (`0x8082xxxx`-
  `0x8083xxxx`) is an *input*, not an unknown constant, and is named by address.
* **conditional** - exactly one named input would still be missing.

Three rows were audited hard against that second clause and each passed for a
stated reason:

* `0x80797D30` has the widest fan-out in the set (34 distinct callees, 18 of
  them unclaimed `0x8079xxxx` siblings). Every one of those calls has its
  argument registers written immediately before the `bl`, so the call *shape* is
  fixed by the bytes; nothing is inferred from a callee body.
* `0x8070FAB0` compares a node name against two C strings that are **runtime
  built**: `0x80358004` is `main.dol` BSS and `0x8082F788` is this REL's BSS, so
  both read as zero in the shipped image. They are named inputs, not unknowns.
* `0x80669560` (and `0x807BFFA0`) read `0x80829600` / `0x80377DC8..0x80377DD0`,
  which are BSS too. In addition `0x80669560` uses `qr3` for four
  `u16 -> f32` conversions; the slot word is pinned by
  `docs/REL_ELF_MAPPING.md` (`GQR3 = 0x00050005`, type 5 unsigned 16-bit,
  scale 0), and type 5 is exactly what the surrounding arithmetic needs.

### This is the list wave 144 predicted, exactly

Re-deriving the candidate set from the current tree gives **1,957** rows
(`5,615` TSV rows, minus 16,849 addresses in `ledger/functions.csv`, minus the
1,467 addresses that already appear in a `#{1,6} ` heading of `docs/*.md`), and
the first eight are byte-for-byte the list wave 144 published as its "next
eight":

```
0x80630820 972 | 0x80797D30 960 | 0x8077DC80 936 | 0x8077A4B0 924
0x8077C670 924 | 0x80669560 916 | 0x8070FAB0 912 | 0x807BFFA0 904
```

The 924-byte tie between `0x8077A4B0` and `0x8077C670` is broken by address
ascending, exactly as wave 144 said it would be. Each of the eight was re-tested
against `ledger/functions.csv` (exact `address` field) and against every
heading in `docs/*.md` - including the `0x`-prefixed and bare spellings - before
being claimed here. The residual difference from wave 144's 1,973 is heading
bookkeeping, not selection: wave 144 added eight `## N. 0xADDR` headings of its
own and the two lanes' heading regexes differ, while every row below was
re-tested independently.

Applying the same filter after this wave, the next eight are:
`0x80760700` (896), `0x806E4F30` (880), `0x807318F0` (876), `0x80699380` (868),
`0x80648CB0` (860), `0x8069C2C0` (852), `0x807B77E0` (852), `0x807BEC10` (848).

## Method

**Address mapping.** Re-derived from the program headers of
`work/rels/elf/d_basesNP.elf` for this pass, not copied:

```
PT_LOAD  p_offset  p_vaddr     filesz    memsz
  0      0x0010F0  0x806000F0  0x1C6004  0x1C6004
  1      0x1C70F4  0x807C60F4  0x0004FC  0x0004FC
  2      0x1C75F0  0x807C65F0  0x00000C  0x00000C
  3      0x1C7600  0x807C6600  0x00A5FC  0x00A5FC
  4      0x1D1C00  0x807D0C00  0x051CB8  0x051CB8
  5      0x000000  0x80822900  0x000000  0x012484 (zero fill)
```

so `file_offset = 0x10F0 + (address - 0x806000F0) = (address - 0x80600000) + 0x1000`
for every code address in this document, as `docs/REL_ELF_MAPPING.md` requires.
Literal pools in `0x807Cxxxx` / `0x807D0xxx` resolve through segment 4 and are
read out verbatim below. Segment 5 (`0x80822900..0x80834D84`, `filesz = 0`) is
this REL's BSS: `0x80832BD4`, `0x80832EB0` and `0x80829600` are named by address
below and read as zero in the shipped image.
`0x8035xxxx`, `0x8037xxxx`, `0x8042xxxx`, `0x8043xxxx` are `main.dol` BSS and
are named the same way.

**Decoder.** A throwaway PowerPC decoder was written under `%TEMP%\opencode\w145`
and validated against the facts `docs/REL_ELF_MAPPING.md` pins, plus four
defects this wave found in its own first pass:

* `psq_l` = 56, `psq_st` = 60; quantiser index in bits 12-14, `W` in bit 15.
  Opcode 59 is the **single-precision float family**, not paired-single.
* Single-float A-form uses **`FRC` (bits 21-25 = `(word >> 6) & 0x1F`)** for
  `fmuls`/`fdivs` and `FRB` for `fadds`/`fsubs`. Reading `fmuls` through `FRB`
  silently turns every `fmuls fD,fA,fC` into `fD = fA * f0`, which is how an
  earlier pass "lost" the operands in rows 6 and 8.
* `cror 2,0,2` (word `0x4C401382`) and `cror 2,1,2` (word `0x4C411382`) are the
  CodeWarrior NaN-safe comparison idioms: after `fcmpo`, `EQ` becomes
  `LT|EQ` (`x <= y`) or `GT|EQ` (`x >= y`), and the following `bne` therefore
  means "store when `x <= y`" / "store when `x >= y`". `crxor 6,6,6`
  (`0x4CC63182`) is `crclr 6`, the vararg `bl` marker - not a data word.
* `psq_l fN,DQ(rA),qr3,W=1` on a halfword just stored to the stack is the
  CodeWarrior `(f32)(u16)` conversion (GQR3 type 5). Four of them in row 6 are
  the only `qr3` uses in this set; there is no quantisation anywhere else.

Register recovery: `0x802DCD40..0x802DCD74` are `__savegprlr_14..31` helpers and
`0x802DCD8C..0x802DCDC0` the matching `__restgprlr_*`; the incoming `r3`
survives all of them. Row 2 uses `0x802DCD74` (`__savegprlr_27`, saves
`r27..r31`), so Ghidra's `void FUN_80797d30(void)` has lost its receiver; rows 7
and 8 use `0x802DCD50`/`0x802DCD70` (`__savegprlr_18` / `__savegprlr_26`).
`0x802DCBFC` is *not* a save helper: it is the CodeWarrior virtual-call thunk
described in row 3.

---

## 1. `0x80630820` - the multi-course-select texture/panel refresher

**Evidence.** `work/ghidra/rel-d_basesNP/export/80630820_FUN_80630820.c`,
972 bytes, 243 words, synthetic `0x80630820`.

**What it is.** The per-frame refresher of the "MultiCorseSelectTexture" screen
(the internal spelling of the texture group is literally that - see the strings
below). It watches a three-word index triple `+0x298/+0x29C/+0x2A0`, and only
does work when the published index `+0x2A0` differs from the last one `+0x29C`.
It then builds the texture path `"MultiCorseSelectTexture/" + <course name>`,
loads it through the bres registry, hands the handle to the archive manager, and
drives thirteen BRK panes: two navigation panes, two coin-battle panes, the
texture banner, two topic panes, two course-number panes, the best-time pane,
the record-message pane and two help panes.

Three of the eight relation branches are decided by the game-mode flag word
`0x80429F80` (bit 0x40 = "special/2-player", bit 0x20 = "coin battle") and by
`FUN_80630BF0`, a 484-byte predicate that returns 0/2/4 and is the only
unclaimed REL callee in the body.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x298` | `i32` | index that was current when the refresh started |
| `+0x29C` | `i32` | last published index |
| `+0x2A0` | `i32` | requested index; equal to `+0x29C` means "no change" |
| `+0x2AC` | `i32` | world selector, 1..8 (row of the texture table); 10 = special |
| `+0x2B0` | `i32` | course selector, 0..9 (column of the texture table); 0x2A = special |
| `+0x248` | pane* | "previous" pane (back arrow) |
| `+0x24C` | pane* | "current/cursor" pane |
| `+0x254` | pane* | coin-battle left pane |
| `+0x258` | pane* | coin-battle right pane |
| `+0x260` | pane* | course-name pane (texture banner) |
| `+0x264` | pane* | topic pane |
| `+0x268` | pane* | topic label pane |
| `+0x26C` | pane* | course-number pane A (left) |
| `+0x270` | pane* | course-number pane B (right) |
| `+0x274` | pane* | best-time pane |
| `+0x278` | pane* | record-message pane |
| `+0x27C`, `+0x280` | pane* | the two help panes |
| `+0xBB` (of any pane) | `u8` | bit 0 = visible |

### Exact behaviour

Prologue `0x80630820..0x80630840`: `stwu r1,-0x80`; save LR, `r31`, `r30`,
`r29`; `r29 = this`; `r31 = 0x807DCCF8` (literal base for the string/table
block). Epilogue `0x80630BD0..0x80630BE8`.

1. **Change gate.** If `this->+0x2A0 == this->+0x29C` return. Otherwise
   `i = this->+0x298`; `this->+0x29C = this->+0x2A0`; if `i > 9` then `i -= 10`.
2. **Hide arm** (`0x80630868..0x806308C4`). If
   `(this->+0x29C != 0 && this->+0x29C != 10) &&
    (this->+0x2AC == 10 || this->+0x2B0 == 0x2A ||
     fn_80630800(this, this->+0x2AC, this->+0x2B0) == 0)`
   then clear bit 0 on `+0x248` and `+0x24C` and return. (The `r4`/`r5` for the
   three-argument call are loaded at `0x8063087C`/`0x80630888`; `r3` is dead.)
3. **Both-tenths arm** (`0x806308C8..0x80630904`). If `+0x29C == 10 &&
   +0x2AC == 10` then `pane(+0x248)->+0xBB &= 0xFE`,
   `pane(+0x24C)->+0xBB = (b & 0xFE) | 1`, return.
4. **Path build** (`0x80630908..0x80630928`).
   `memset(buf, 0, 100)` (`0x800046B4`); `strncat(buf, "MultiCorseSelectTexture/", 99)`
   (`0x802E1A68`; the literal is at `0x807DD600`).
5. **Texture-name arm** (`0x80630928..0x80630978`). If
   `(*(u32*)0x80429F80 & 0x40) == 0 || this->+0x29C != 0 || i >= 5`:
   `pane(+0x260)->+0xBB &= 0xFE` and `strncat(buf, fn_80630800(this, +0x2AC, +0x2B0), 99)`.
   Else: `pane(+0x260)->+0xBB |= 1` and
   `strncat(buf, table_0x807DD5D8[i], 99)` - the five `msk_coin0N.tpl` names.
6. **Load** (`0x806309AC..0x806309C4`).
   `tex = fn_800df180(*(u32*)0x8042A038 + 4, "MultiCorseSelectTexture", buf)`
   (`0x800DF180(archive, "MultiCorseSelectTexture" @0x807DD620, buf)`), then
   `fn_802b4c60(fn_802b3300(), tex, 0)`.
7. **Cursor move and hand-off** (`0x806309C4..0x80630A10`).
   `tex` is kept in `r31`; then the pane method
   `(*(void**)(this->+0x24C))[0x68](pane(+0x24C))` is called (raw
   `lwz r3,588(r29)`, `lwz r12,0(r3)`, `lwz r12,0x68(r12)`, `mtctr`,
   `bctrl`), and finally `fn_802b4c60(fn_802b3300(), tex, 0)`. There is exactly
   **one** `0x802B4C60` call, after the pane method.
   Then `pane(+0x248)->+0xBB &= 0xFE`, `pane(+0x24C)->+0xBB = (b & 0xFE) | 1`.
8. **Number/help arm** (`0x80630A14..0x80630AF8`). Same three-way test as step 5
   (`flag & 0x40`, `+0x29C != 0`, `i >= 5`):
   * large arm: clear `+0x264`, `+0x268`; set `+0x26C`, `+0x270`;
     `fn_80630560(this, this->+0x2AC + 1, 0x28, pane(+0x26C))`;
     `k = 0x800B4F40(this->+0x2AC, this->+0x2B0)`;
     if `k < 0x14` then `fn_80630620(this, k)` else
     `0x800C9A60(pane(+0x278), fn_800cdc60(), 0x32, fn_80630660(this, k), 0)`.
   * small arm: `fn_806306c0(this)` (which itself drives `+0x264..+0x278`).
9. **Reset four panes** (`0x80630AF8..0x80630B38`). Clear bit 0 on `+0x27C`,
   `+0x280`, `+0x254`, `+0x258`.
10. **Mode arm** (`0x80630B3C..0x80630BCC`). `m = fn_80630bf0(this)`.
    If `m == 2 || m == 4`:
    * if `(*(u32*)0x80429F80 & 0x20) != 0` then set `+0x254` visible when
      `m == 2`, else set `+0x258` visible;
    * if the bit is clear, set `+0x258` visible;
    * then `fn_800c92f0(this + 0x70, 5, 1)`.
    Else `fn_800c93d0(this + 0x70, 5, 0)`.
11. Return (`void`; the export's signature is accurate - `r3` is whatever the
    last call left).

### Data read out of the image

* `0x807DD600` - `"MultiCorseSelectTexture/"`.
* `0x807DD620` - `"MultiCorseSelectTexture"`.
* `0x807DD5D8` - pointer table, index `0..4`:
  `"msk_coin01.tpl"`, `"msk_coin02.tpl"`, `"msk_coin03.tpl"`, `"msk_coin04.tpl"`,
  `"msk_coin05.tpl"` (entries 5 and 6 are NULL, which is why the arm is gated on
  `i < 5`).
* `0x807DD420` - the course-texture table used by `fn_80630800`: 8 rows of
  10 pointers, `0x28` bytes per row, `table[row][column]`, row = world 1..8,
  column = course. Row 7 (world 8) has all ten entries; the other rows end with
  one or two NULLs:
  ```
  row 0: im_W1_1 im_W1_2 im_W1_3 im_W1_toride im_W1_4 im_W1_5 im_W1_6 im_W1_shiro
  row 1: im_W2_1 ... im_W2_shiro            (same 8 positions)
  row 2: im_W3_1 im_W3_2 im_W3_3 im_W3_obake im_W3_toride im_W3_4 im_W3_5 im_W3_shiro
  row 3: im_W4_1 im_W4_2 im_W4_3 im_W4_toride im_W4_4 im_W4_obake im_W4_5 im_W4_shiro im_W4_hikousen
  row 4: im_W5_1 im_W5_2 im_W5_3 im_W5_toride im_W5_4 im_W5_5 im_W5_obake im_W5_shiro
  row 5: im_W6_1 im_W6_2 im_W6_3 im_W6_4 im_W6_toride im_W6_5 im_W6_6 im_W6_shiro im_W6_hikousen
  row 6: im_W7_1 im_W7_2 im_W7_3 im_W7_toride im_W7_obake im_W7_4 im_W7_5 im_W7_6 im_W7_shiro
  row 7: im_W8_1 im_W8_2 im_W8_3 im_W8_toride im_W8_4 im_W8_5 im_W8_6 im_W8_7 im_W8_hikousen im_W8_shiro
  ```
* `0x80429F80` (main.dol BSS) flags; `0x8042A038` (main.dol BSS) archive handle.

**Signature.**

```rust
/// Refreshes the multi-course-select texture banner and its thirteen panes.
///
/// # Original
/// `FUN_80630820`, `0x80630820`, 972 bytes,
/// `work/ghidra/rel-d_basesNP/export/80630820_FUN_80630820.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - course-select screen object; index triple `+0x298..+0x2A0`,
///   selectors `+0x2AC`/`+0x2B0`, panes `+0x248..+0x280`, layout at `+0x70`.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - every pane write, string, table entry and branch is literal;
/// `0x80630BF0` is unclaimed and is described only by its 0/2/4 results.
pub unsafe fn fn_80630820(this: usize)
```

**Constants.** Special selectors `0`/`10` (`+0x29C`, `+0x2AC`), `0x2A`
(`+0x2B0`), `5` (coin rows), `0x14` (course count); message ids `0x28`, `0x32`;
masks `0xFE`/`0x01`; mode bits `0x40`/`0x20` of `0x80429F80`; buffer sizes
`100`/`99`; layout argument `5` at `+0x70`. Callees `0x800046B4`, `0x802E1A68`,
`0x800DF180`, `0x802B3300`, `0x802B4C60`, `0x800B4F40`, `0x800CDC60`,
`0x800C9A60`, `0x800C92F0`, `0x800C93D0`, `0x80630560`, `0x80630620`,
`0x80630660`, `0x806306C0`, `0x80630800`, plus the `+0x24C` vtable slot `0x68`.

**Verdict: ready (medium).** One unclaimed REL callee (`0x80630BF0`), one
unclaimed `main.dol` callee (`0x800B4F40`); both are called with visible,
non-ambiguous arguments and only their integer results are consumed.

---

## 2. `0x80797D30` - the course-tree node update (34-callee dispatcher)

**Evidence.** `work/ghidra/rel-d_basesNP/export/80797d30_FUN_80797d30.c`,
960 bytes, 240 words, synthetic `0x80797D30`.

**What it is.** The per-frame update of the course-select "node" actor: it
classifies the record at `+0x22C` against the current course tree
(`scene->+0x338C * 0x16518 + 0x3394`, the same area record
`fn_80799750`/`fn_80799860` use in `src/game/bases.rs`), fires the two
world-map effect records `0x36`/`0x37` on entry into a newly unlocked area,
and then runs a seven-arm state machine that either advances, retries, or
resets the selection.

The body's real signature is hidden: it starts with `bl 0x802DCD74`
(`__savegprlr_27`) and the export's `void FUN_80797d30(void)` is Ghidra's
attempt to model the helper's "return". Entry `r3` is the receiver and is live
throughout.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x22C` | `u32` | record/name id handed to every probe |
| `+0x23C` | `i32` | selection index (0..n) |
| `+0x1D0` | `ptr` | tree node; `+0x154` = kind/id |
| `+0x29C` | `ptr` | controller object; `+0x70` = flag word (bit 0x40) |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | position |
| `+0xB8`,`+0xBC`,`+0xC0` | `f32` | previous position (copied when bit 0x40 is set) |
| `+0x2C4`,`+0x2C8`,`+0x2CC` | `f32` | spawn point copied into the position |
| `+0x230` | `u32` | handle returned by `fn_80774fa0` |
| `+0x102`, `+0x108` | `s16` | published angle (both get `+0x1DA`) |
| `+0x1DA` | `s16` | source angle |

### Exact behaviour

Prologue `0x80797D30..0x80797D50`: frame `0x30`, LR at `+0x34`, `r31/r30/r29`
saved, `__savegprlr_27` via `r11 = r1+0x30`; `this = r3`; `scene = *(u32*)0x8042A18C`.
Epilogue `0x807980D8..0x807980EC`.

1. `local = fn_8079bb60(&buf, this, this->+0x22C)` - builds a 40-byte name
   record on the stack (the buffer is passed by address and is *not* otherwise
   used here).
2. `slot = *(u8*)0x8042A24D`; `cur = *(u8*)(*(u32*)0x80429F7C + slot + 0x395)`;
   `v = fn_807995a0(this)`.
3. If `cur == v && *(u8*)(*(u32*)0x80429F7C + slot + 0x3A9) == 0 &&
   fn_800fc9c0() != 0` then
   `wm_effect_record_submit(*(u32*)0x8042A27C, 0x36, (u8*)this + 0xAC, 2)` and
   `wm_effect_record_submit(*(u32*)0x8042A27C, 0x37, (u8*)this + 0xAC, 1)`.
4. **Arm A** - `fn_8079f5f0(this) != 0`: `fn_807980f0(this, &buf, 0.0f, 0.0f, 1, 1, 0)`
   then `fn_807984f0(this)`; done.
5. **Arm B** - `fn_8079b660(this) != 0 && fn_8075c010() != 0`:
   `fn_80798370(this)`; `fn_807980f0(this, &buf, 0.0f, 0.0f, 1, 1, 0)`; done.
6. **Arm C** - `fn_8079a7d0(this, this->+0x22C) != 0`:
   1. if `(ctrl->+0x70 & 0x40) != 0` then
      `this->+0xB8 = this->+0xAC`, `+0xBC = +0xB0`, `+0xC0 = +0xB4`.
   2. `fn_80c561e0(this->+0x29C, 0x40, 0)` - clear flag `0x40`
      (this callee lives in `d_en_bossNP`, not in this REL).
   3. `if (fn_8079a4b0(this, +0x22C) == 0 ||
         fn_800f57a0(scene + scene->+0x338C * 0x16518 + 0x3394, +0x22C, 0x400) != 0)`
      then
      * if `fn_800f9080() == 0 || fn_8079b0c0(this) == 0 ||
        fn_8079b720(this) != 0` then
        `fn_807980f0(this,&buf,0,0,1,1,0)`; `fn_80796a10(this)`.
      * else `i = fn_8079af60(this)`; if `this->+0x23C == i &&
        fn_80798b10(this) != 0` then
        `this->+0xAC = this->+0x2C4`, `+0xB0 = +0x2C8`, `+0xB4 = +0x2CC`;
        `this->+0x230 = fn_80774fa0(scene, this->+0x22C, this->+0x23C)`;
        `fn_80798c30(this, +0x23C)`; `fn_80799200(this, +0x23C, 1)`;
        `fn_80797810(this)`.
        else `fn_807980f0(this,&buf,0,0,1,1,0)`; `fn_80796cf0(this, 0)`;
        `fn_80797190(this)`.
      * then `fn_80799f10(this)`; done.
   4. else (the `0x400` probe failed): `fn_807980f0(this,&buf,0,0,1,1,0)`;
      `fn_8079f900(this, 0)`; `this->+0x22C = *(u32*)0x80832EB0 (BSS)`;
      `fn_80798310(this)`; done.
7. **Arm D** - `fn_8079a4b0(this, +0x22C) != 0`:
   `fn_8079f900(this, 0)`; `this->+0x22C = *(u32*)0x80832EB0 (BSS)`;
   `fn_80798310(this)`; done.
8. **Arm E** - the plain path:
   ```
   base = this->+0x1D0;  kind = *(u32*)(base + 0x154);  sel = this->+0x23C;
   fn_80799340(this, &buf);
   fn_8079a870(this, this->+0x23C);
   if (kind == 0x35 && sel == 1 && *(u32*)(this->+0x1D0 + 0x154) == 2)
       fn_807982a0(this);
   else { *(u16*)(this+0x108) = *(s16*)(this+0x1DA);
          *(u16*)(this+0x102) = *(s16*)(this+0x1DA); }
   ```
   `kind` is read *before* `fn_80799340` and re-read after `fn_8079a870`; the
   second read is a fresh memory load, so a callee that changes `+0x154`
   changes the outcome. That ordering is load-bearing.
9. Return `void`.

The `0.0f` pair passed to `fn_807980f0` comes from the literal at `0x807D0160`
(`lfs f1,0x160(0x807D0000)` twice into `f1`/`f2`); Ghidra prints it as
`(double)DAT_807d0160` twice, which is a decompiler widening of a single
precision load.

**Signature.**

```rust
/// Advances the course-select node actor by one frame.
///
/// # Original
/// `FUN_80797d30`, `0x80797D30`, 960 bytes,
/// `work/ghidra/rel-d_basesNP/export/80797d30_FUN_80797d30.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - node actor; record id `+0x22C`, selection `+0x23C`, node
///   `+0x1D0`, controller `+0x29C`, position `+0xAC..+0xC0`.
/// # Returns
/// Nothing. The export's `void` and the `iVar2 = func_0x802dcd74()` preamble
/// are both artefacts of `__savegprlr_27`.
/// # Confidence
/// medium - every guard, field write and call argument is literal; 18 of the 34
/// callees are unclaimed `0x8079xxxx` siblings and stay address-only.
pub unsafe fn fn_80797d30(this: usize)
```

**Constants.** `0x36`/`0x37` effect ids; `0x3A9`/`0x395` offsets into the
`0x80429F7C` slot table; `0x40` controller flag; `0x400` name probe mask;
`0x16518` area stride; `0x3394` area offset; `0x35` kind id; `2`; `0.0f`
(`0x807D0160`). Globals `0x8042A18C`, `0x80429F7C`, `0x8042A24D`, `0x8042A27C`;
BSS `0x80832EB0`. Unclaimed callees `0x8079BB60`, `0x8079F5F0`, `0x8079B660`,
`0x8079F900`, `0x8079B720`, `0x807980F0`, `0x80796A10`, `0x80799200`,
`0x80797810`, `0x80796CF0`, `0x80797190`, `0x80799340`, `0x80774FA0`,
`0x8075C010`, `0x800F57A0`, `0x800FC9C0`, `0x800F9080`, `0x80C561E0`.

**Verdict: ready (medium).** Every call in the body (42 `bl` sites over 34
distinct callees) has its argument registers written in the instructions before
the `bl`; the sixteen translated siblings
(`fn_807982a0`, `fn_80798310`, `fn_80798370`, `fn_807984f0`, `fn_80798b10`,
`fn_80798c30`, `fn_807995a0`, `fn_80799f10`, `fn_8079a4b0`, `fn_8079a7d0`,
`fn_8079a870`, `fn_8079af60`, `fn_8079b0c0`, plus the two save helpers and
`wm_effect_record_submit`) fix the calling convention of the whole family.

---

## 3. `0x8077DC80` - the eight-state course-select scene controller

**Evidence.** `work/ghidra/rel-d_basesNP/export/8077dc80_FUN_8077dc80.c`,
936 bytes, 234 words, synthetic `0x8077DC80`.

**What it is.** The per-frame update of a scene controller that dispatches on
`+0x1D0` (0..7) through a table of eight 12-byte thunks, forwards the current
"interface id" to vtable slot `+0x60`, then updates the camera model: an effect
probe over the two strings `"Fs00"`/`"Fs01"`, a sound trigger keyed on
`+0x1F0` (`0x16`/`0x8B`), and a 3D submit of the model matrix.

The `+0x1D0` dispatch is *not* a switch statement in the image: the raw code
computes `r12 = 0x807CF6C8 + 12 * this->+0x1D0` and calls the CodeWarrior
virtual-call helper `0x802DCBFC` (see below), so Ghidra prints it as
`func_0x802dcbfc(iVar5, this->+0x1D0 * 0xc)`.

### The dispatch table at `0x807CF6C8`

Eight entries, 12 bytes each, `{ offset, vtable_index, function }`:

| state | offset | index | function |
|---:|---:|---:|---|
| 0 | 0 | -1 | `0x8077E6D0` |
| 1 | 0 | -1 | `0x8077E8B0` |
| 2 | 0 | -1 | `0x8077EA30` |
| 3 | 0 | -1 | `0x8077EEA0` |
| 4 | 0 | -1 | `0x8077FA10` |
| 5 | 0 | -1 | `0x8077FB60` |
| 6 | 0 | -1 | `0x8077FDD0` |
| 7 | 0 | -1 | `0x80780190` |

`0x802DCBFC` is `in_r12[2]` when `in_r12[1] < 0`, else
`(*(void**)(*(void**)(in_r12[2] + arg + in_r12[0]) + in_r12[1]))()`. Every entry
here uses the `index = -1` form, so the call is `handler(this, state * 12)` with
`handler` taken from the table above; the second argument is the same value the
helper received in `r4` and is ignored by all eight handlers.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x124` | `u8` | "actor alive" byte; 0 = the whole update takes the short path |
| `+0x139` | `u8` | "entered" flag, set to 1 on the transitions |
| `+0x1D0` | `i32` | state index 0..7 for the dispatch table |
| `+0x1C8` | `i32` | playing sound handle; negative = not started |
| `+0x1CC` | `ptr` | render owner; `+4` = the NW4R object (`+0x28` key pair, `+0x58` f32, `+0x151`/`+0x152` bytes) |
| `+0x1F0` | `i32` | effect/sound id (`0x16`, `0x8B` are special) |
| `+0x1F4` | `u32` | copied from `*fn_80782050(+0x214)` |
| `+0x214`..`+0x224` | - | interface block; `+0x220` non-zero enables the `+0x1F4` refresh |
| `+0x2D8` | `ptr` | sound/effect manager handle |
| `+0x294` | `ptr` | model; `+0xB0` f32, per-index vec3 at `+0x24/+0x28/+0x2C` |
| `+0x2EC`,`+0x2F0`,`+0x2F4` | `f32` | three copies of the `0x80100A00` result |
| `+0x268` | `u8` | snapshot of `+0x124` |
| `+0x60` | `ptr` | class vtable; `+0x60` = "set interface id", `+0x64` = "is busy" |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | position |
| `+0xDC`,`+0xE0`,`+0xE4` | `f32` | camera target |
| `+0x100`,`+0x102`,`+0x104` | `s16` | three angles |
| `+0x10C` | `f32` | effect position scalar |

### Exact behaviour

Prologue `0x8077DC80..0x8077DCA8`: frame `0xA0`, LR at `+0xA4`, `f31`/`f30`/
`f29` saved as `stfd` + `psq_st ...,qr0,W=0` pairs (the export's `unaff_GQR0`
blocks are these three saves and nothing else), `r31..r28` saved,
`__savegprlr_27` via `r11 = r1+0x70`; `r29 = 0x807CF608`, `r31 = this`.
Epilogue `0x8077DFF8..0x8077E024`.

1. **Short path** (`0x8077DCAC..0x8077DCF4`). If `this->+0x124 == 0`:
   if `this->+0x220 != 0` then `this->+0x1F4 = *fn_80782050(this + 0x214)`;
   `this->+0x1D0 = 0`; `this->+0x139 = 1`; return **1**.
2. **Dispatch gate** (`0x8077DCF8..0x8077DD44`). `ctrl = *(u32*)0x8042A1AC`.
   `fn_802dcbfc(this, this->+0x1D0 * 12)` is called when
   `fn_80781c80(this) != 0 || fn_801015e0(ctrl) == 0x5E ||
    fn_801015e0(ctrl) == 0x70` (the helper is called twice in the evaluation
   order shown: first `fn_80781c80`, then the id probe twice).
3. **Interface id** (`0x8077DD48..0x8077DD68`).
   `v = fn_801015e0(ctrl)`; `(*(this->+0x60))[0x60](this, v, *(u8*)(ctrl + 0x164))`
   - three arguments: `r3 = this`, `r4 = v`, `r5 = byte`.
4. **State-4 extra** (`0x8077DD6C..0x8077DD7C`). If `this->+0x1D0 == 4` then
   `fn_8077fa10(this)`.
5. **Two-string effect probe** (`0x8077DD80..0x8077DE2C`), only when
   `*(u8*)0x8042A24D == 2`. For `i = 0, 1` over `{ "Fs00", "Fs01" }`
   (pointer array at `0x808184A0`):
   ```
   if (fn_80100700(scene, name[i]) == 0) continue;
   fn_80100270(&v, scene, name[i]);            // v = { x, y, z } at r1+0x34
   q = { v.x, 0.0f, v.z };                    // r1+0x40
   fn_801c1650(&q, &w);                       // w = { this->+0xAC, 0.0f, this->+0xB4 }
   d = fn_802be470();                         // distance-like scalar in f1
   if (d < 20.0f && this->+0x1D0 != 7) this->+0xB0 = 15.0f;
   ```
6. **Sound/effect submit** (`0x8077DE30..0x8077DEC4`).
   ```
   obj = *(u32*)(this->+0x1CC + 4);
   if (fn_800d6ab0(obj) == 0 &&
       fn_80100ca0(this->+0x2D8, this->+0x1F0, obj, *(f32*)(obj+0x58),
                   fn_80168080(obj + 0x28)) == 0) goto step 7;
   if (fn_801015e0(ctrl) != 0x42)
       fn_80105060(*(u32*)0x8042A27C,
                   fn_80100bd0(this->+0x2D8, this->+0x1F0),
                   *(u8*)(obj+0x151), *(u8*)(obj+0x152), this->+0x10C);
   ```
7. **Looping sound** (`0x8077DEC8..0x8077DF24`). If `fn_80100f70(this->+0x2D8)`
   is non-zero and `this->+0x1F0` is `0x16` or `0x8B`:
   if `this->+0x1C8 < 0` then
   `this->+0x1C8 = fn_801051c0(*(u32*)0x8042A27C, 0x0B, 0, obj->+0x151, obj->+0x152)`.
   Otherwise `fn_800fdcf0(this)`.
8. **Model submit** (`0x8077DF2C..0x8077DFF4`).
   ```
   s = fn_80100a00(*(u8*)(obj + 0x152));   this->+0x2EC = s; +0x2F0 = s; +0x2F4 = s;
   fn_800d6f40(this->+0x1CC, &{this->+0xAC,+0xB0,+0xB4},
               &{s16 +0x100, +0x102, +0x104}, &{this->+0xDC,+0xE0,+0xE4});
   fn_800d6e90(this->+0x1CC);
   model = this->+0x294; idx = *(u8*)(obj + 0x152);
   fn_800f6810(model->+0xB0, model[idx*12 + 0x24], +0x28, +0x2C, this);
   this->+0x268 = this->+0x124;
   ```
9. Return **1** in both reachable returns.

**Signature.**

```rust
/// Advances the eight-state course-select scene controller and its model.
///
/// # Original
/// `FUN_8077dc80`, `0x8077DC80`, 936 bytes,
/// `work/ghidra/rel-d_basesNP/export/8077dc80_FUN_8077dc80.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - controller; state `+0x1D0`, render owner `+0x1CC`, model
///   `+0x294`, sound manager `+0x2D8`, vtable `+0x60`.
/// # Returns
/// `1` always (both the short path and the main path load `r3 = 1`).
/// # Confidence
/// medium - the dispatch table, the two probe strings and every field write are
/// literal; `0x802DCBFC`'s register contract is taken from its own export.
pub unsafe fn fn_8077dc80(this: usize) -> u32
```

**Constants.** States `0..7`; ids `0x5E`, `0x70`, `0x42`, `0x16`, `0x8B`, `0x0B`;
`20.0f` (`0x807CF764`), `15.0f` (`0x807CF768`), `0.0f` (`0x807CF698`);
string pointer array `0x808184A0` = { `0x8081848C` `"Fs00"`,
`0x80818494` `"Fs01"` }; dispatch table `0x807CF6C8`; globals `0x8042A18C`,
`0x8042A1AC`, `0x8042A24D`, `0x8042A27C`; vtable slots `+0x60` and `+0x64`.

**Verdict: ready (medium).** The three `unaff_GQR0` warnings are void - the
body's only paired-single words are the `f31`/`f30`/`f29` spill pairs.

---

## 4. `0x8077A4B0` - the `"s0"`/`"s1"` child-actor respawn gate

**Evidence.** `work/ghidra/rel-d_basesNP/export/8077a4b0_FUN_8077a4b0.c`,
924 bytes, 231 words, synthetic `0x8077A4B0`.

**What it is.** A spawn/skip gate for a two-part object (a rider plus its
shadow/vehicle): it selects the resource name `"s1"` or `"s0"` from bit
`(this->+0x4 >> 16) & 0xF`, builds a 16-byte key block through the resource
registry, and then either creates the pair of children (`0x27B`/`0x27C`/`0x27D`
plus `0x27E`) or clears the flags, depending on five registry/mode probes
(`0x800FB350`, `0x800FD920`, `0x800FB330`, `0x800FC9C0`, and in the clear arm
`0x800FB3C0`/`0x800FB460`/`0x800FB410`). It always ends by calling
`fn_8077a850(this)`.

The two arms differ in one place that is easy to miss: the clear arm writes
`*(u8*)(this + 0x124) = 0` (raw `stb r30,292(r31)` at `0x8077A788`), while the
create arm writes `*(u8*)(child + 0x124) = 0` on **both** children. Ghidra
reproduces both correctly; the two forms must not be "unified".

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x4` | `u32` | flags; bits 16..19 select `"s1"` (non-zero) or `"s0"` (zero) |
| `+0x1AC` | - | resource registry handle (passed by address) |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | spawn position handed to the second child |
| `+0x1E4` | `ptr` | first child (`0x27B`/`0x27C`/`0x27D`), 0 = none |
| `+0x1E8` | `ptr` | second child (`0x27E`), 0 = none |
| `+0x1EC` | `u8` | "children live" flag: 1 on the create arm, 0 on the clear arm |
| `+0x124` | `u8` | cleared on the clear arm (no child is written) |

### Exact behaviour

Prologue `0x8077A4B0..0x8077A4CC`: frame `0x30`, LR at `+0x34`, `r31/r30/r29/
r28` saved, `r31 = this`. Epilogue `0x8077A82C..0x8077A848`.

1. **Resource key** (`0x8077A4D0..0x8077A53C`). If `(this->+0x4 >> 16) & 0xF`
   is non-zero: `k = fn_80168400(this + 0x1AC)`, `id = fn_80164f00(&k, "s1")`,
   `fn_80168230(this + 0x1AC, id, &blk16)`. Else the same with `"s0"`.
2. **Create arm** (`0x8077A540..0x8077A608`), when
   `(fn_800fb350() & 0xFF) == 0 && fn_800fd920(this) == 0`:
   ```
   mode  = fn_800fb330();                       // 0, 1, 3.., 9
   this->+0x1EC = 1;
   fn_800f1710(scene + scene->+0x338C * 0xBF8 + 0x1A0, 0);
   f  = *(u32*)0x80832BD4;      // BSS
   f1 = f | 0x10000;
   if (mode == 0) {
       if (fn_800fc9c0() == 0) { this->+0x1E4 = 0; this->+0x1E8 = 0; }
       else { spawn(0x27C, f1, &blk16); spawn(0x27E, f, this + 0xAC);
              child(+0x1E4)->+0x124 = 0; child(+0x1E8)->+0x124 = 0; }
   } else if (mode == 9) { spawn(0x27D, f1, &blk16); spawn(0x27E, f, this+0xAC); /* same two clears */ }
   else if (mode >= 3)   { spawn(0x27B, f1, &blk16); spawn(0x27E, f, this+0xAC); /* same two clears */ }
   else if (mode >= 1)   { spawn(0x27C, f1, &blk16); spawn(0x27E, f, this+0xAC); /* same two clears */ }
   ```
   where `spawn(id, flags, pos)` is
   `0x800F2A00(id, this, flags, pos, 0)`; the first child's result is stored at
   `+0x1E4` and the second's at `+0x1E8`.
3. **Clear arm** (`0x8077A60C..0x8077A820`):
   ```
   this->+0x1EC = 0;
   f = *(u32*)0x80832BD4;                       // BSS, no |0x10000 here
   fn_800f1710(scene + scene->+0x338C * 0xBF8 + 0x1A0, 1);
   if (fn_800fb3c0() != 0)      { spawn(0x27D, f, &blk16); spawn(0x27E, f, this+0xAC); this->+0x124 = 0; }
   else if (fn_800fb460() != 0) { spawn(0x27B, f, &blk16); spawn(0x27E, f, this+0xAC); this->+0x124 = 0; }
   else if (fn_800fb410() != 0) { spawn(0x27C, f, &blk16); spawn(0x27E, f, this+0xAC); this->+0x124 = 0; }
   ```
   Note: the two children are stored but **not** touched afterwards; the only
   `+0x124` write is on `this`.
4. `fn_8077a850(this)` (264 bytes, already translated).
5. Return `void`.

`mode` values 1..2 take the `0x27C` branch, 3..8 take `0x27B`, 9 takes `0x27D`,
and any other non-zero value (`10`, `11`, ...) also takes `0x27B` because the
`mode >= 3` test is a signed compare and runs before the `mode >= 1` one.

**Signature.**

```rust
/// Spawns or clears the `"s0"`/`"s1"` child pair of a ride actor.
///
/// # Original
/// `FUN_8077a4b0`, `0x8077A4B0`, 924 bytes,
/// `work/ghidra/rel-d_basesNP/export/8077a4b0_FUN_8077a4b0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - owner; flags `+0x4`, registry `+0x1AC`, children `+0x1E4`/
///   `+0x1E8`, live flag `+0x1EC`.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - the four mode arms, the two strings and all fourteen `0x800F2A00`
/// calls are literal; `0x800F2A00` itself is unclaimed and is described only by
/// its argument shape.
pub unsafe fn fn_8077a4b0(this: usize)
```

**Constants.** Child ids `0x27B`, `0x27C`, `0x27D`, `0x27E`; flag `0x10000`;
layout args `0`/`1`; strings `"s1"` at `0x80818090`, `"s0"` at `0x80818094`;
BSS flag word `0x80832BD4`; area stride `0xBF8`, area offset `0x1A0`; globals
`0x8042A18C`. Callees `0x80168400`, `0x80164F00`, `0x80168230`, `0x800FB350`,
`0x800FD920`, `0x800FB330`, `0x800F1710`, `0x800FC9C0`, `0x800F2A00` (x14),
`0x800FB3C0`, `0x800FB460`, `0x800FB410`, `0x8077A850`.

**Verdict: ready (medium).** Every branch condition and every argument register
is literal; the only unclaimed helper with a non-obvious contract,
`0x800F2A00`, is used as `(id, owner, flags, position, 0) -> child` and its
result is stored, never dereferenced except at `+0x124`.

---

## 5. `0x8077C670` - the twelve-state cursor/transition machine

**Evidence.** `work/ghidra/rel-d_basesNP/export/8077c670_FUN_8077c670.c`,
924 bytes, 231 words, synthetic `0x8077C670`.

**What it is.** A state machine over `this->+0x274` (0..11) driven by a jump
table at `0x8081845C`. It gates on a countdown `+0x210` first, then moves the
selection between a "source" position (`+0x278..+0x280`, snapshotted from
`+0xAC..`) and a destination (`+0x1D8..+0x1E0`), publishing the travelled
distance into the two halfwords `+0x108`/`+0x102`, and finally calls the class
vtable slot `+0x64`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x210` | `i32` | countdown; `> 0` decrements and skips the whole update |
| `+0x274` | `i32` | state 0..11 (jump-table index) |
| `+0x278`,`+0x27C`,`+0x280` | `f32` | source position |
| `+0x1D8`,`+0x1DC`,`+0x1E0` | `f32` | destination position |
| `+0x108`, `+0x102` | `s16` | published distance (both written) |
| `+0x10C` | `f32` | cleared on a successful transition |
| `+0x139` | `u8` | "entered" flag, set to 1 on a successful transition |
| `+0x18C` | `u8` | cleared after the vtable call returns non-zero |
| `+0x60` | `ptr` | class vtable; `+0x64` = "is finished" |

### The jump table at `0x8081845C`

| state | target | state | target |
|---:|---|---:|---|
| 0 | `0x8077C6C8` | 6 | `0x8077C810` |
| 1 | `0x8077C6E4` | 7 | `0x8077C82C` |
| 2 | `0x8077C718` | 8 | `0x8077C860` |
| 3 | `0x8077C764` | 9 | `0x8077C8B0` |
| 4 | `0x8077C7AC` | 10 | `0x8077C92C` |
| 5 | `0x8077C7C8` | 11 | `0x8077C954` |

States `>= 12` fall through to the common tail (no default arm).

### Exact behaviour

Prologue `0x8077C670..0x8077C684`: frame `0x30`, LR at `+0x34`, `r31` saved,
`r31 = this`. Epilogue `0x8077C9F8..0x8077CA08`.

1. `ctrl = *(u32*)0x8042A1A0` (kept in `r5`; the two snapshot arms read
   `+0x1FC`, `+0x200`, `+0x204` from it).
2. If `this->+0x210 > 0` then `this->+0x210 -= 1` and return.
3. `idx = this->+0x274`; if `idx > 11` go straight to step 5; otherwise jump
   through `0x8081845C[idx]`.
4. State bodies. **States 0, 2, 4 and 6 fall through** into 1, 3, 5 and 7
   respectively - there is no `break` after them, so the same frame runs both
   halves. State 8 does **not** fall through: it ends with
   `b 0x8077C9CC` (the common tail). This is the single most important
   structural fact in the body, and Ghidra's `switch` reproduces it.
   * **0** (`0x8077C6C8`): `this->+0x274 = 1`; `fn_8077e410(this, 1, -1, 0.0f)`;
     fall into 1.
   * **1** (`0x8077C6E4`): `fn_8006cc00(this)`; `fn_8006cd40(this)`;
     `if (fn_800fdc60(this, this+0x278, this+0x1D8) != 0) this->+0x274 = 2`.
   * **2** (`0x8077C718`):
     `this->+0x278..+0x280 = this->+0xAC..+0xB4`;
     `this->+0x1D8..+0x1E0 = ctrl->+0x1FC..+0x204`;
     `h = fn_80160e80(this + 0xAC, this + 0x1D8)`;
     `this->+0x108 = h`, `this->+0x102 = h`; `this->+0x274 = 3`; fall into 3.
   * **3** (`0x8077C764`): `fn_8006cc00(this)`; `fn_8006cd40(this)`;
     `if (fn_800fdc60(this, this+0x278, this+0x1D8) != 0) {
        this->+0x10C = 0.0f; fn_8077e570(this); this->+0x139 = 1; }`.
   * **4** (`0x8077C7AC`): `this->+0x274 = 5`; `fn_8077e410(this, 1, -1, 0.0f)`;
     fall into 5.
   * **5** (`0x8077C7C8`): identical to 3.
   * **6** (`0x8077C810`): `this->+0x274 = 7`; `fn_8077e410(this, 1, -1, 0.0f)`;
     fall into 7.
   * **7** (`0x8077C82C`): identical to 1 (three calls, then `+0x274 = 8`).
   * **8** (`0x8077C860`): identical to 2 except that it sets `this->+0x274 = 9`
     and then **jumps to the tail**; the state-9 code runs on the next frame.
   * **9** (`0x8077C8B0`): `fn_8006cc00(this)`; `fn_8006cd40(this)`;
     `d = fn_800f9cd0(&f, this + 0xAC)` (f32 out at `r1+0x14`);
     `lim = 200.0f + (f32)(*(u16*)(*(u32*)*0x8042A480 + 4))` - the `lis 0x4330`
     / `lfd 2^52` / `fsubs` / `fadds` ladder at `0x8077C8C8..0x8077C904`;
     `if (lim < d) { this->+0x10C = 0.0f; fn_8077e570(this); this->+0x139 = 1; }`.
   * **10** (`0x8077C92C`): `fn_800f6410(this, 0x80377DD4)` then
     `fn_8077e410(this, 1, -1, 0.0f)`; jump to the tail.
   * **11** (`0x8077C954`): the same body as 9 with the out slot at `r1+0x8`.
5. **Common tail** (`0x8077C9CC..0x8077C9F4`):
   ```
   if ((*(this->+0x60))[0x64](this) != 0) { fn_800fdc80(this); this->+0x18C = 0; }
   ```
6. Return `void`.

The two transition helpers have fixed register shapes: `fn_8077e410` is called
as `(this, 1, -1, f1 = 0.0f)` at four sites, and `fn_80160e80` takes
`(this + 0xAC, this + 0x1D8)` and returns the `u16` that is published into both
halfwords. `fn_8077e570` is already translated (`fn_8077e570` in
`src/game/bases.rs`).

**Signature.**

```rust
/// Drives the twelve-state cursor transition of the course-select object.
///
/// # Original
/// `FUN_8077c670`, `0x8077C670`, 924 bytes,
/// `work/ghidra/rel-d_basesNP/export/8077c670_FUN_8077c670.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - cursor object; countdown `+0x210`, state `+0x274`, source
///   `+0x278..+0x280`, destination `+0x1D8..+0x1E0`.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - the twelve-entry jump table, both distance ladders and every field
/// write are literal; `0x800FDC60`/`0x800F6410` stay address-only.
pub unsafe fn fn_8077c670(this: usize)
```

**Constants.** States `0..11`; timers `1`, `2`, `3`, `5`, `7`, `8`, `9`;
`200.0f` at `0x807CF6B4`; `2^52` (`0x4330000000000000`) at `0x807CF6B8`;
`0.0f` at `0x807CF698`; jump table `0x8081845C`; the `0x80377DD4` object
(main.dol BSS) passed to `fn_800f6410` by state 10; globals `0x8042A1A0`,
`0x8042A480`; vtable slot `+0x64`. Callees `0x8077E410`, `0x8006CC00`,
`0x8006CD40`, `0x800FDC60`, `0x80160E80`, `0x8077E570`, `0x800F9CD0`,
`0x800F6410`, `0x800FDC80`.

**Verdict: ready (medium).** Twelve arms, one jump table, four documented
fall-throughs (0→1, 2→3, 4→5, 6→7) and one hard `b` out of state 8; the
double-precision ladder is the standard `u16 -> f32` idiom and the comparison is
a plain `d > 200 + n`.

---

## 6. `0x80669560` - the two-pass rotated shadow-quad builder

**Evidence.** `work/ghidra/rel-d_basesNP/export/80669560_FUN_80669560.c`,
916 bytes, 229 words, synthetic `0x80669560`.

**What it is.** Builds the two shadow/decal records of a rotating actor. A byte
at `+0x768` selects which of two mirrored geometries is emitted; each geometry
converts the actor's 16-bit angle `+0x44E` (1/256-turn units) through the two
`0x80237xxx` table interpolators, scales the results by the 16.0 half-size
(`32.0` overall), submits two 8-argument records through `0x800DB210` into
`this + 0x69C` and `this + 0x6FC`, and finally pushes three extents into the
culling owner at `this + 0x4D4`.

`0x80669560` is the only body in this set that uses `qr3`. Four
`psq_l ...,qr3,W=1` on a just-stored halfword perform the `(f32)(u16)`
conversion; with `GQR3 = 0x00050005` (type 5 = unsigned 16-bit, scale 0, pinned
by `docs/REL_ELF_MAPPING.md`) the arithmetic is exactly
`(f32)(u16)angle * (1/256)`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x768` | `u8` | 1 = mirrored geometry, otherwise the base geometry |
| `+0x44E` | `u16` | actor angle, 1/256 turns; `-0x4000` / `+0x4000` bias per arm |
| `+0x3A0`, `+0x3A4` | `f32` | quad origin (x, z) |
| `+0x69C` | record | first shadow record, 0x60 bytes |
| `+0x6FC` | record | second shadow record, 0x60 bytes |
| `+0x4D4` | owner | culling box owner for `fn_8007fce0` / `fd10` / `fd40` |
| `+0x318`, `+0x31C` | `f32` | not read by this body |

### Exact behaviour

Prologue `0x80669560..0x806695B8`: frame `0xC0`, LR at `+0xC4`,
`f31/f30/f29/f28/f27` saved as `stfd` + `psq_st ...,qr0,W=0` pairs (the export's
`unaff_GQR0`/`ldexpf` blocks are exactly these saves), `r31/r30/r29/r28`
saved, `r30 = 0x807C8DC0`, `r28 = this`, `r31 = (u16)this->+0x44E`.
Epilogue `0x806698AC..0x806698F0`.

**Arm A - `this->+0x768 == 1`** (`0x806695C0..0x80669720`):

```
ang = (s16)((u16)this->+0x44E - 0x4000);
sn  = fn_80237c40((f32)(u16)ang * 0.00390625f);   // 0x802F74A4 table interpolator
cs  = fn_80237bd0((f32)(u16)ang * 0.00390625f);   // 0x802F74A0 table interpolator
hx  = 16.0f * sn;  hz = 16.0f * cs;
p   = { this->+0x3A0 - hx, this->+0x3A4 - hz };   // built at r1+0x30 for call 1
f31 = (this->+0x3A0 + hx) - 32.0f;
f30 = this->+0x3A4 + hz;
f29 = this->+0x3A0 + hx;
fn_800db210(this + 0x69C, this, &p /* r1+0x30 */, 0.0f, 32.0f, 0.0f, ang, 1, 0);
a = -16.0f - p.x;      b = 0.0f - p.y;
at = fn_801611c0(b, a);                            // s16 angle, r31
d2 = a*a + b*b;
d  = (d2 != 0.0f) ? d2 * fn_80237ba0() : 0.0f;     // fn_80237ba0 = rcp-sqrt
fn_800db210(this + 0x6FC, this, &p /* r1+0x28 */, 0.0f, 0.0f, -d,
            (s16)(at + 0x8000), 1, 0);
```

**Arm B - `this->+0x768 != 1`** (`0x80669724..0x8066987C`):

```
ang2 = (s16)((u16)this->+0x44E + 0x4000);
sn   = fn_80237c40((f32)(u16)ang2 * 0.00390625f);
cs   = fn_80237bd0((f32)(u16)ang2 * 0.00390625f);
hx   = 16.0f * sn;  hz = 16.0f * cs;
p    = { this->+0x3A0 - hx, this->+0x3A4 - hz };   // built at r1+0x40, used at r1+0x18
f31  = this->+0x3A0 + hx;
f29  = 32.0f + f31;
f30  = this->+0x3A4 + hz;
fn_800db210(this + 0x69C, this, &p /* r1+0x18 */, 0.0f, 0.0f, -32.0f,
            (s16)((u16)this->+0x44E - 0x4000), 1, 0);
a  = 16.0f - p.x;      b = 0.0f - p.y;
at = fn_801611c0(b, a);
d  = (a*a + b*b != 0.0f) ? (a*a + b*b) * fn_80237ba0() : 0.0f;
fn_800db210(this + 0x6FC, this, &p /* r1+0x10 */, 0.0f, d, 0.0f, (s16)at, 1, 0);
```

**Tail - both arms** (`0x80669880..0x806698A8`):

```
fn_806693f0(this);                 // already translated
fn_8007fce0(f31, this + 0x4D4);    // culling byte +0x20
fn_8007fd10(f30, this + 0x4D4);    // culling byte +0x21
fn_8007fd40(f29, this + 0x4D4);    // culling byte +0x22
return;
```

The three `0x8007FCE0`/`FD10`/`FD40` calls take `(f32, owner)` in that register
order (`f1` first, `r3 = this + 0x4D4`) - the ledger rows for those three
functions describe `param_1 - owner->+0xD0` stored as an 8-bit offset.

**Signature.**

```rust
/// Emits the two rotated shadow quads and their culling extents.
///
/// # Original
/// `FUN_80669560`, `0x80669560`, 916 bytes,
/// `work/ghidra/rel-d_basesNP/export/80669560_FUN_80669560.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - actor; mirror flag `+0x768`, angle `+0x44E`, origin
///   `+0x3A0`/`+0x3A4`, shadow records `+0x69C`/`+0x6FC`, culling owner `+0x4D4`.
/// # Returns
/// Nothing.
/// # Confidence
/// high - both mirror arms are transcribed instruction for instruction; the
/// only external facts are `GQR3 = 0x00050005` (documented) and the meaning of
/// the two 0x80237xxx table interpolators (already translated).
pub unsafe fn fn_80669560(this: usize)
```

**Constants.** `0.00390625f` (`1/256`) at `0x807C8E1C`; `16.0f` at `0x807C8E20`;
`32.0f` at `0x807C8E24`; `0.0f` at `0x807C8E28`; `-16.0f` at `0x807C8E2C`;
`-32.0f` at `0x807C8E30`; `0x80829600` (BSS) supplies the `0.0f` used as the
second arm's zero; angle biases `0x4000`, `0x8000`; record stride `0x60`;
argument tail `1, 0`. Callees `0x80237C40`, `0x80237BD0`, `0x80237BA0`,
`0x801611C0`, `0x800DB210` (x4), `0x8007FCE0`, `0x8007FD10`, `0x8007FD40`,
`0x806693F0`.

**Verdict: ready (high).** Every operand of both arms is a literal or a local;
the export's recorded damage (`unaff_GQR0`, `unaff_GQR3`, `ldexpf`) is entirely
prologue/epilogue paired-single save traffic plus the four legitimate GQR3
conversions.

---

## 7. `0x8070FAB0` - the `Wm_ob_witchcraft` slot sweep

**Evidence.** `work/ghidra/rel-d_basesNP/export/8070fab0_FUN_8070fab0.c`,
912 bytes, 228 words, synthetic `0x8070FAB0`.

**What it is.** The per-frame sweep of the world-map object that owns the
`"Wm_ob_witchcraft"` emitter. It walks `this->+0x9FC` object slots, drops every
slot whose object died, despawned, changed to state 1, matched a name, or is
held by one of the four active players, and otherwise feeds the slot's object
position into the slot's render node. When the slot list is drained it ages a
counter, and when `+0x1610` is positive it emits a burst through the
`this + 0x8D4` effect emitter.

This is the second body in this set whose receiver is hidden by a save helper:
it starts with `bl 0x802DCD50` (`__savegprlr_18`) and restores with
`0x802DCD9C` (`__restgprlr_18`), so `this = entry r3` and the function returns
`1` in `r3`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x394` | iface | interface object; vtable `+0x10` = reset, `+0x28` = acquire list, list vtable `+0x14` = name match |
| `+0x5B0` | sub | 500-byte sub-object refreshed by `fn_8007fda0` |
| `+0x694` | `i32` | idle counter compared with `0x80805EA0` (29) |
| `+0x9FC` | `i32` | slot count |
| `+0xA00 + i*0x114` | node | render node per slot; vtable `+0x10` = release, `+0xB0` = submit |
| `+0x15DC + i*4` | `u32` | object handle per slot (0 = free) |
| `+0x1608` | `u32` | frame counter compared with `0x80806618` (300) |
| `+0x1610` | `f32` | emitter intensity |
| `+0x1614` | `u32` | second frame counter, always incremented |
| `+0x8D4` | emitter | vtable `+0xA0` = emit record, `+0x74` = emit position, `+0x2C` = set intensity |
| `+0xB4`, `+0x318`, `+0x31C` | `f32` | extents used for the effect vectors |

### Exact behaviour

Prologue `0x8070FAB0..0x8070FAE4`: frame `0x70`, LR at `+0x74`,
`__savegprlr_18` via `r11 = r1+0x70`; `this = r3`. Epilogue
`0x8070FE0C..0x8070FE3C`.

1. `(*(void**)(this + 0x394))[0x10](this + 0x394)` - one-argument virtual call.
2. `fn_8007fda0(this + 0x5B0)`.
3. **Slot loop** `for (i = 0; i < this->+0x9FC; i++)`, with
   `slot = this + 0x15DC + i*4` and `node = this + 0xA00 + i*0x114`:
   ```
   if (*(u32*)slot == 0) continue;
   obj = fn_80162d00(*(u32*)slot);
   if (obj == 0 || fn_80070cb0(obj + 0x1EC) != 0 || *(u8*)(obj + 0x36C) == 1)
       goto kill;
   if (*(u8*)(obj + 0x38C) == 3) {
       list = (*(void**)(obj + 0x394))[0x28](obj + 0x394);
       if ((*list)[0x14](list, 0x80358004) != 0) goto kill;
   }
   for (p = 0, found = 0; p < 4; p++) {
       if ((*(u32*)0x80429CA8 & (1u << p)) == 0) continue;
       pl = fn_8005f900(p);
       if (((*pl->vtbl)[0x3C8](pl) != 0 || (*pl->vtbl)[0x3CC](pl) != 0)
           && *(u32*)slot == *(u32*)(pl + 0x2A78)) { found = 1; break; }
   }
   if (found) goto kill;
   if (this->+0x1608 >= *(u32*)0x80806618) goto kill;
   (*node->vtbl)[0xB0](node, obj + 0xAC, 0, 0);
   continue;
   kill:
       *(u32*)slot = 0;
       (*node->vtbl)[0x10](node);
   ```
   The player predicates are the `+0x60` vtable slots `0x3C8` and `0x3CC`; the
   slot handle is compared against the player's `+0x2A78`.
4. `if (this->+0x694 < *(u32*)0x80805EA0) fn_8070fe40(this)` - the `<` is a
   signed compare, and `0x8070FE40` is a 460-byte sibling.
5. **Burst**, only when `*(f32*)0x807CCA7C (0.0f) < this->+0x1610`:
   ```
   list = (*(void**)(this + 0x394))[0x28](this + 0x394);
   if ((*list)[0x14](list, 0x8082F788) != 0) {
       fn_8006ced0(&s1, this);            // vec3
       fn_8006ced0(&s2, this);            // vec3
       vec = { s1.x, s2.y, 0.5f * this->+0xB4 };
       (*(void**)(this + 0x8D4))[0xA0](this + 0x8D4, "Wm_ob_witchcraft", 0, &vec, 0, 0);
       pos = { 1.0f, 0.5f * this->+0x318, 0.5f * this->+0x31C };
       (*(void**)(this + 0x8D4))[0x74](this + 0x8D4, &pos, 0, 3);
       (*(void**)(this + 0x8D4))[0x2C](this + 0x8D4, 3, 0.018f * this->+0x1610);
   }
   ```
6. `this->+0x1614 += 1`; `this->+0x1608 += 1`; return `1`.

**Signature.**

```rust
/// Sweeps the object slots of the world-map witchcraft emitter.
///
/// # Original
/// `FUN_8070fab0`, `0x8070FAB0`, 912 bytes,
/// `work/ghidra/rel-d_basesNP/export/8070fab0_FUN_8070fab0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - owner; interface `+0x394`, slot list `+0x9FC`/`+0x15DC`,
///   nodes `+0xA00`, emitter `+0x8D4`, intensity `+0x1610`.
/// # Returns
/// `1`. The export's `void` is the `__savegprlr_18` artefact.
/// # Confidence
/// medium - the sweep, all six kill tests and both emitted vectors are literal;
/// the two name strings are runtime-built BSS and stay address-only.
pub unsafe fn fn_8070fab0(this: usize) -> u32
```

**Constants.** `0x80805EA0` = 29 (idle limit); `0x80806618` = 300 (frame limit);
`0x80806624` = `0.018f`; `0x807CCA78` = `1.0f`; `0x807CCA7C` = `0.0f`;
`0x807CCA88` = `0.5f` (`0x807CCA80` = -256.0f and `0x807CCA84` = 16.0f share the
pool but are not read here); `"Wm_ob_witchcraft"` at `0x80805DF4`; name strings
`0x80358004` (main.dol BSS) and `0x8082F788` (BSS); player mask
`*(u32*)0x80429CA8`. Callees `0x8007FDA0`, `0x80162D00`, `0x80070CB0`,
`0x8005F900`, `0x8006CED0` (x2), `0x8070FE40`, plus the virtual slots listed in
the field map (`+0x10`, `+0x28`, `+0x14`, `+0xB0`, `+0xA0`, `+0x74`, `+0x2C`,
and the player `+0x3C8`/`+0x3CC`).

**Verdict: ready (medium).** All slot arithmetic, both thresholds and the six
emitter arguments are literal; `0x8070FE40` and `0x8006CED0` are unclaimed but
called with visible arguments.

---

## 8. `0x807BFFA0` - the world-map object bounds/position update

**Evidence.** `work/ghidra/rel-d_basesNP/export/807bffa0_FUN_807bffa0.c`,
904 bytes, 226 words, synthetic `0x807BFFA0`.

**What it is.** Rebuilds the six-float bounds box of a world-map object every
frame: it seeds `+0x658..+0x66C` with the `±99999` sentinels, refreshes one
position per area slot (`area->+0x33A0` entries), pulls the area's own two probe
points when `fn_807c13a0` is true, copies the live box into `+0x670..+0x684`,
hands it to the display object's vtable slot `+0x28` (with the `150`/`300`
clamp arguments), runs a `±30`/`±40` clamp pass against `fn_807c0c10`'s result,
and finally pokes audio bank 6.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x658`..`+0x66C` | `f32` x6 | min/max extents (x min, x max, y min, y max, z min, z max - sentinel seeded) |
| `+0x670`..`+0x684` | `f32` x6 | previous-frame copy of the box |
| `+0x688`,`+0x68C`,`+0x690` | `f32` x3 | vector A (`99999`, `-99999`, `0`) |
| `+0x694`,`+0x698`,`+0x69C` | `f32` x3 | vector B (`-99999`, `99999`, `0`) |
| `+0x6A0` | `u32` | zeroed every frame |
| `+0x6A4`,`+0x6A8`,`+0x6AC` | `f32` x3 | copy of `0x80377DC8..0x80377DD0` (main.dol BSS) |

### Exact behaviour

Prologue `0x807BFFA0..0x807BFFB0`: frame `0x80`, LR at `+0x84`,
`__savegprlr_26` via `r11 = r1+0x80`; `this = r3`, `r31 = 0x807D0A68`.
Epilogue `0x807C030C..0x807C0324`.

1. **Seed** (`0x807BFFD4..0x807C0054`), with `P = 99999.0f` (`0x807D0B30`),
   `N = -99999.0f` (`0x807D0B34`), `Z = 0.0f` (`0x807D0A6C`):
   `+0x658 = P`, `+0x65C = P`, `+0x660 = N`, `+0x664 = N`, `+0x668 = N`,
   `+0x66C = P`, `+0x688 = P`, `+0x68C = N`, `+0x690 = Z`, `+0x694 = N`,
   `+0x698 = P`, `+0x69C = Z`.
2. **Area slot sweep** (`0x807C0058..0x807C00AC`), `area = scene + scene->+0x338C * 0x16518`:
   ```
   for (i = 0; i < *(u32*)(area + 0x33A0); i++)
       if (fn_807c0b20(this, i) != 0) {
           fn_80100200(&v, scene, i);
           fn_807c0330(this, &v);
       }
   ```
   Note the stride here is `0x16518` (the *area* record), unlike row 4 and this
   row's own step 3.
3. **Area probes** (`0x807C00B0..0x807C012C`), when `fn_807c13a0(this) != 0`,
   with `a = scene + scene->+0x338C * 0xBF8 + 0x1A0`:
   `fn_800f1040(a, &o1)`; `fn_800f1160(a, &o2)`; `fn_807c0330(this, &o2)`;
   then `d = *(u32*)(*(u32*)0x8042A1A0 + 0x29C); k = *(u32*)(d + 8);`
   `if (k == 2 || k == 4) { this->+0x668 = o2.y; this->+0x65C = o1.y; }`.
4. **Cut** (`0x807C012C..0x807C0148`): if `fn_807c1410(this) != 0` then
   `this->+0x668 = 0.0f`, `this->+0x65C = 0.0f`.
5. **Publish** (`0x807C0148..0x807C01A0`): `this->+0x670..+0x684 = this->+0x658..+0x66C`;
   then
   ```
   d = *(u32*)(*(u32*)0x8042A1A0 + 0x29C);
   (*d->vtbl)[0x28](d, this + 0x658, this + 0x664, 150.0f, 300.0f);
   ```
6. **Reset tail fields**: `this->+0x6A0 = 0`; `+0x6A4/+0x6A8/+0x6AC =
   *(f32*)0x80377DC8/+0xCC/+0xD0` (main.dol BSS, reads 0 in the shipped image).
7. **Clamp pass** (`0x807C01D4..0x807C02F8`) with `f = fn_807c0c10(this)`:
   ```
   if (f < 0.0f && this->+0x66C > f) this->+0x66C = f - 40.0f;
   if (f > 0.0f && this->+0x660 < f) this->+0x660 = f + 40.0f;
   if (this->+0x658 > 0.0f)  this->+0x658 = 0.0f;
   if (this->+0x664 < 0.0f)  this->+0x664 = 0.0f;
   if (this->+0x65C > 0.0f)  this->+0x65C = 0.0f;
   if (this->+0x668 < 0.0f)  this->+0x668 = 0.0f;
   if (this->+0x664 <= 30.0f) this->+0x664 = 30.0f;
   if (this->+0x658 >= -30.0f) this->+0x658 = -30.0f;
   if (this->+0x66C >= -40.0f) this->+0x66C = -40.0f;
   if (this->+0x660 <= 40.0f) this->+0x660 = 40.0f;
   ```
   The last four are the `cror 2,0,2` / `cror 2,1,2` NaN-safe idioms described in
   the Method section; read them as `<=` and `>=` against the constants.
8. `fn_8019ca00(*(u32*)0x8042A4A8, 6)` (audio bank 6); return `1`.

**Signature.**

```rust
/// Rebuilds the world-map object's six-float bounds box and hands it to the
/// display object.
///
/// # Original
/// `FUN_807bffa0`, `0x807BFFA0`, 904 bytes,
/// `work/ghidra/rel-d_basesNP/export/807bffa0_FUN_807bffa0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - object; box `+0x658..+0x66C`, previous box `+0x670`, vectors
///   `+0x688`/`+0x694`, tail vector `+0x6A4`.
/// # Returns
/// `1`. The export's `void` is the `__savegprlr_26` artefact.
/// # Confidence
/// medium - the seed, the sweep, both probe paths and all ten clamps are
/// literal; the display vtable call and `fn_807c0c10` stay address-only.
pub unsafe fn fn_807bffa0(this: usize) -> u32
```

**Constants.** `99999.0f` at `0x807D0B30`, `-99999.0f` at `0x807D0B34`,
`0.0f` at `0x807D0A6C`; `150.0f` at `0x8082158C`, `300.0f` at `0x80821590` (the
two display-clamp arguments); `30.0f` at `0x808215DC`, `40.0f` at `0x808215E0`;
`0x80377DC8..0x80377DD0` (main.dol BSS); area strides `0x16518` and `0xBF8`,
area offset `0x1A0`, slot count offset `0x33A0`; display state values `2`/`4`;
audio bank `6`. Callees `0x80100200`, `0x800F1040`, `0x800F1160`, `0x807C0B20`,
`0x807C0330` (x2), `0x807C13A0`, `0x807C1410`, `0x807C0C10`, `0x8019CA00`, and
the two virtual slots `+0x28` (display) and `+0x60`.

**Verdict: ready (medium).** The body is a straight-line seed + sweep + clamp
pass; the only unclaimed pieces are the four `0x807Cxxxx` siblings and the
display vtable, all called with explicit register arguments.

---

## What a translation lane should watch

Five things in this tranche are worth carrying into the Rust bodies (three are
Ghidra misreadings, two are raw-code subtleties):

1. **Hidden receivers.** `0x80797D30`, `0x8070FAB0` and `0x807BFFA0` start with
   `bl 0x802DCD74` / `0x802DCD50` / `0x802DCD70` and the export shows
   `void FUN_x(void)` plus a bogus `iVar = func_...()` "return". The receiver is
   entry `r3` in all three; write the bodies as `fn_x(this: usize)`.
2. **`0x8077A4B0`'s two arms write different things.** The create arm clears
   `+0x124` on **both** children; the clear arm clears `+0x124` on `this` only
   (`stb r30,292(r31)` at `0x8077A788`). Do not "unify" them.
3. **`0x8077C670` falls through.** States 0, 2, 4 and 6 run the following
   state's body in the same frame; state 8 does not. A Rust `match` needs
   explicit chaining for the first four and a plain `break`-equivalent for 8.
4. **`0x8077DC80`'s dispatch is a thunk, not a switch.** The eight handlers are
   the third word of the `{0, -1, fn}` triplets at `0x807CF6C8`; the helper
   `0x802DCBFC` picks `in_r12[2]` whenever `in_r12[1] < 0`.
5. **`0x80669560`'s `qr3` is not damage.** The four `psq_l ...,qr3,W=1` on a
   stack halfword are `(f32)(u16)` conversions under `GQR3 = 0x00050005`. The
   `unaff_GQR0` blocks in the export are the `f31`/`f30`/`f29`/`f28`/`f27`
   save/restore pairs and contain no arithmetic.

Two more, from the cross-checks:

* `0x80630820` calls `fn_80630800` with **three** arguments (`this`,
  `this->+0x2AC`, `this->+0x2B0`) at two sites, even though the export prints
  the first site as a one-argument call. The `r4`/`r5` are loaded two
  instructions earlier and survive the `cmpi` chain.
* `0x80797D30` re-reads `*(u32*)(this->+0x1D0 + 0x154)` *after* the
  `fn_80799340` / `fn_8079a870` pair; the kind value used in the `0x35` test is
  the second read.

## Verification

* Every address, byte count and word count in this document comes from
  `work/ghidra/rel-d_basesNP/export/functions.tsv`; `decompiled` is `true` and
  the export file exists for all eight.
* `blr` sits at `address + size - 4` in all eight bodies, and all 1,862 words
  decode with no `.word` fallback.
* No address in this document appears in `ledger/functions.csv`, and none
  appears in any heading of `docs/*.md`, before this file was written.
* All literals were read out of `work/rels/elf/d_basesNP.elf` through the
  `p_offset + (address - p_vaddr)` rule; no value was copied from prose.
* No Rust file, ledger CSV or existing document was modified; no merge or check
  tool was run.

