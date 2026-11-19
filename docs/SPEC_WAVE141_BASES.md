# Wave 141 - `game::bases` - the next eight multi-KB bodies

Read-only specification lane. No Rust file, ledger row or existing document was
modified. The only file written is this one.

Wave 140 claimed the twelve largest untranslated rows of
`work/ghidra/rel-d_basesNP/export/functions.tsv`. This wave continues the same
population one rank lower and stops at **eight** instead of twelve, because the
remaining bodies are 1.6-2.9 KB each (14,960 bytes / 3,740 instructions total)
and a twelve-row lane would have been a summary rather than a specification.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x807BD110` | 2,908 | 727 | `807bd110_FUN_807bd110.c` | ready (high) |
| 2 | `0x8062C8E0` | 1,804 | 451 | `8062c8e0_FUN_8062c8e0.c` | ready (medium) |
| 3 | `0x8065DD50` | 1,772 | 443 | `8065dd50_FUN_8065dd50.c` | conditional (medium) |
| 4 | `0x8065F090` | 1,772 | 443 | `8065f090_FUN_8065f090.c` | conditional (medium) |
| 5 | `0x8071D500` | 1,748 | 437 | `8071d500_FUN_8071d500.c` | ready (medium) |
| 6 | `0x8067FC80` | 1,676 | 419 | `8067fc80_FUN_8067fc80.c` | ready (medium) |
| 7 | `0x807740D0` | 1,656 | 414 | `807740d0_FUN_807740d0.c` | conditional (medium) |
| 8 | `0x806A24F0` | 1,624 | 406 | `806a24f0_FUN_806a24f0.c` | conditional (medium) |

**Ready: 4** (`0x807BD110`, `0x8062C8E0`, `0x8071D500`, `0x8067FC80`).
**Conditional: 4** (`0x8065DD50`, `0x8065F090`, `0x807740D0`, `0x806A24F0`).

Every `bytes` value is the `size` column of the TSV verbatim. Unlike several
earlier waves, **every one of these eight sizes is the true instruction span**:
the decoder below found the final `blr` at `address + size - 4` in all eight
cases except `0x807BD110`, whose last four bytes are a zero pad after the `blr`
(`0x807BDC68 blr`, `0x807BDC6C .word 0`).

### Selection rule

Candidate set: every `functions.tsv` row of `rel-d_basesNP` whose address is

1. absent from the `address` column of `ledger/functions.csv` (all modules),
2. absent from every `#{1,6} ` heading of every `docs/*.md` **including the
   twelve `docs/SPEC_WAVE140_BASES.md` sections added last wave**, and
3. absent from every `fn_<addr>` identifier in `src/game/bases.rs`.

That leaves **2,011** rows. Ranked by `size` descending the first eight are the
table above, with one deliberate correction to the previous lanes' skip list:

**`0x807BD110` is claimed this wave.** Waves 139 and 140 both listed it as "the
`dWorldSelect_c` 44-entry `StateID` registration table, not a body" and excluded
it "because it needs a data/table generator". That classification is wrong on
both counts and the shipped bytes prove it:

* it is a **function**, entered with `stwu r1,-608(r1)`, that ends in
  `blr` after `_restgpr`; it is not a table in `.data`;
* it registers **13** `StateID` objects, not 44;
* the "table" it writes is the *payload* of those 13 state objects, and the
  constructor supplies each payload from a 13 x 9-word descriptor array at
  `0x80820EB8` (fully decoded in section 1 below).

So the largest remaining untranslated row of `d_basesNP` **is** a body and is
specifiable. Section 1 is the complete specification, including all 13 state
names, all 39 handler pointers and the 17-float/9-halfword global block.

The nine rows immediately below the cut, for a later lane:
`0x806D09B0` (1,508), `0x806B8380` (1,500), `0x807962B0` (1,428),
`0x80664610` (1,292), `0x80792A20` (1,212), `0x80605BA0` (1,164),
`0x8067E640` (1,160), `0x8062B5C0` (1,136), `0x807A84B0` (1,124).

## Method

**Address mapping.** Re-read from the program headers of
`work/rels/elf/d_basesNP.elf` (not copied from the older docs):

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
for every address in this document, as `docs/REL_ELF_MAPPING.md` requires. The
cross-check against `work/rels/d_basesNP.rel` (text `rawOffset 0xF0`,
`size 0x1C6004`) is `rel_offset = address - 0x80600000`, i.e. exactly 0xFFF
lower; the same words appear at both offsets.

**Decoder.** A throwaway PowerPC decoder was written under
`%TEMP%\opencode` (never in the repo) and validated against the three facts
`docs/REL_ELF_MAPPING.md` pins:

* `psq_l` = opcode 56, `psq_st` = 60, with `DQ` in bits 0-11, `I` in 12-14,
  `W` in 15, `rA` in 16-20, `Fs` in 21-25. Verified on
  `0x807BD124 = f3e10258` -> `psq_st f31, 600(r1), W=0, qr0`, which pairs with
  the preceding `stfd f31, 592(r1)` and the `addi r1, r1, 608` epilogue.
* the extend group is opcode 31 / XO 954 (`extsb`) and 922 (`extsh`). Two words
  in these bodies (`0x7C000775`, `0x7C170774`) are `extsb. r0,r0` and
  `extsb r23,r0` - byte-sign-extension pairs for the `lbz`+`cmpwi` idiom - and
  a decoder without XO 954 prints them as `.word`, which is what made earlier
  lanes report "an unclassified compare" at `0x807BD168`. It is
  `lbz r0,832(r28)` / `extsb. r0,r0` / `bne`.
* no `mtspr GQRn` exists in any of the eight bodies. Every `psq` site in this
  wave is `qr0` (`I` = 0): they are `f28..f31` prologue saves and reloads only.
  **GQR3 (`0x00050005`) is not needed by any body here.**

**Jump tables.** Only two of the eight dispatch indirectly; both were expanded
from the file image, not guessed:

* `0x8067FC80` walks a 24 x 3-word table of function-pointer triples at
  `0x807ECB48` (listed in section 6);
* `0x807740D0` has a 17-case `switch` on the third byte of a spawn-name string
  (`'0'..'9'`, `'A'`, `'C'`, `'G'`, `'K'`, `'T'`, `'W'`, `'X'`) that compiles to
  a comparison tree; decoded case by case in section 7.

The other six bodies have no indirect branch.

**Signature convention (`undefined8` returns).** Six of the eight are printed by
Ghidra as `void FUN_x(void)` whose first statement is
`iVar = func_0x802dcd54/58/60/70/74()`. Those five entry points are the
`_savegpr_N` helpers: they spill `rN..r31` and LR, and they **do not touch
`r3`/`r4`**, so the printed `undefined8` is the incoming `r3:r4` pair. This
wave states, for every body, which incoming registers are real arguments and
quotes the instruction that proves it (an `or rX,r3,r3`, an `addi rX,r4,...`,
or the first use of `r4` before it is overwritten). `0x806A24F0` is the one
body that genuinely takes **two** pointer arguments and reads them as
`iVar6 = (int)uVar13` / `iVar5 = (int)(uVar13 >> 32)` - i.e. `r3` and `r4`.

**Caller-visible facts.** `0x80429F7C` is the current course-object pointer and
`0x80429F80` its flag word (already documented in `src/engine/globals.rs` as
`SDA_5704_OBJECT` / `SDA_5700_FLAGS`); `0x8042A040` is the "master"/scene
pointer; `0x80429CA8` is the active-player bitmask (4 bits used);
`0x80429F10` is the flow/game-com manager whose `+0x15C` is a "level cleared"
style flag.

---

## 1. `0x807BD110` - `dWorldSelect_c` constructor (ready, high)

* TSV size **2,908** bytes, **727** words; export
  `work/ghidra/rel-d_basesNP/export/807bd110_FUN_807bd110.c`
* instruction extent `0x807BD110`-`0x807BDC69` (2,904 bytes), then
  `0x807BDC6C .word 0` padding
* frame 608 bytes; saves `f27..f31` as `stfd` **and** `psq_st ... qr0`, and
  `r28..r31` through `_savegpr_28_31` (`bl 0x802DCD74`)

### Signature

```rust
/// `dWorldSelect_c::dWorldSelect_c()`
/// No arguments (r3 is never read: r28 is loaded with the constant
/// 0x808340E8 at 0x807BD14C), no return value.
pub unsafe extern "C" fn fn_807bd110();
```

Proof there is no receiver:

```
807bd148: 4bb1fc2d  bl 0x802DCD74        ; _savegpr_28_31
807bd14c: 3f808083  addis r28,r0,-32637  ; r28 = 0x80830000
807bd154: 3b9c40e8  addi  r28,r28,16616  ; r28 = 0x808340E8   <- global, not r3
807bd160: 3bde0980  addi  r30,r30,2432   ; r30 = 0x807D0980   const block
807bd164: 3bbd0c18  addi  r29,r29,3096   ; r29 = 0x80820C18   float block
```

### Globals it owns

| address | contents |
|---|---|
| `0x808340E8` | 16-byte header of the world-select state list (BSS) |
| `0x808340F8 + 0x40*i`, `i = 0..12` | 13 state objects, 0x40 bytes each (BSS) |
| `0x80834428` | one-time-init guard byte |
| `0x80820C20 .. 0x80820C75` | 17 `f32` + 9 `u16` layout block |
| `0x808212B0` | 12-byte type descriptor used as each state's `[0]` word |

`0x808212B0` reads `0, 0, 0x807BDC70`; the third word is the state destructor
that is also passed as the functor to `0x80600160`.

### Behaviour

**Step 1 - one-time initialisation** (`if (*(u8*)0x80834428 == 0)`,
`0x807BD15C`-`0x807BD290`). The comparison is
`lbz r0,832(r28)` / `extsb. r0,r0` / `bne -> 0x807BD294`. Values are built with
the classic 2^52 magic-number sequence (`addis r27,r0,17200` = `0x43300000`,
`xoris` the low half, `lfd`, then `fsubs` against the double `0x4330000080000000`
at `0x807D09F8`); the two products use the `f32` at `0x807D0A2C` (= 100.0f)
times the multipliers at `0x807D0984`/`0x807D0988` (1.2f / 0.9f).

| offset | value | source |
|---|---|---|
| `0x80820C20` | `165.0` | `f32 0x807D0980` |
| `0x80820C24` | `3500.0` | `int 3500` -> float |
| `0x80820C28` | `120.0` | `100.0 * 1.2` |
| `0x80820C2C` | `90.0` | `100.0 * 0.9` |
| `0x80820C30` | `0.0` | `f32 0x807D0998` |
| `0x80820C34` | `-73.0` | `f32 0x807D099C` |
| `0x80820C38` | `5300.0` | `int 5300` -> float |
| `0x80820C3C` | `190.0` | `f32 0x807D098C` |
| `0x80820C40` | `0.2` | `f32 0x807D0990` |
| `0x80820C44` | `1.6` | `f32 0x807D0994` |
| `0x80820C48` | `5.0` | `int 5` -> float |
| `0x80820C4C` | `138.0` | `f32 0x807D09A0` |
| `0x80820C50` | `25.0` | `f32 0x807D09A4` |
| `0x80820C54` | `9.0` | `f32 0x807D09A8` |
| `0x80820C58` | `-150.0` | `f32 0x807D09AC` |
| `0x80820C5C` | `0.0` | `f32 0x807D09B0` |
| `0x80820C60` | `16.0` | `f32 0x807D09B4` |
| `0x80820C64` | `4` (u16) | `0x807BD26C` |
| `0x80820C66` | `60` | `0x807BD270` |
| `0x80820C68` | `25` | `0x807BD274` |
| `0x80820C6A` | `25` | `0x807BD278` |
| `0x80820C6C` | `25` | `0x807BD27C` |
| `0x80820C6E` | `80` | `0x807BD280` |
| `0x80820C70` | `3` | `0x807BD284` |
| `0x80820C72` | `30` | `0x807BD288` |
| `0x80820C74` | `180` | `0x807BD28C` |

then `*(u8*)0x80834428 = 1` (`0x807BD290`).

**Step 2 - thirteen state registrations.** For `i = 0..12` the same block
repeats at `0x807BD294 + i*0x4C`-ish (the compiler interleaves, but the effect
is uniform):

```
obj = 0x808340F8 + 0x40*i
load 9 words from 0x80820EB8 + 0x24*i  (three {0, -1, handler} triples)
obj[0x00] = 0x808212B0                 ; type word
8015F7C0(obj, name)                    ; state-object constructor
obj[0x0C] = w0 ; obj[0x10] = w1 ; ... obj[0x2C] = w8
80600160(obj, 0x807BDC70, obj-0x10)    ; register into the list at 0x808340E8
```

`0x8015F7C0` is the state constructor `(self, const char* name)`; `0x80600160`
is the list-registration helper `(self, dtor, list_header)`.

The 13 `(name, handler triple)` tuples - the name pointers are exact and were
read from the strings at those addresses:

| # | name | obj | handlers (`+0x0C`, `+0x10`-pair, `+0x1C`) |
|--:|---|---|---|
| 0 | `dWorldSelect_c::StateID_OnStageWait` | `0x808340F8` | `0x807BC070`, `0x807BBE20`, `0x807BBD80` |
| 1 | `dWorldSelect_c::StateID_OpenAnimeEndWait` | `0x80834138` | `0x807BC1F0`, `0x807BC160`, `0x807BC090` |
| 2 | `dWorldSelect_c::StateID_SelectExpand` | `0x80834178` | `0x807BC2C0`, `0x807BC220`, `0x807BC200` |
| 3 | `dWorldSelect_c::StateID_Select` | `0x808341B8` | `0x807BC5C0`, `0x807BC310`, `0x807BC2E0` |
| 4 | `dWorldSelect_c::StateID_IsLandSizeDown` | `0x808341F8` | `0x807BC740`, `0x807BC660`, `0x807BC5D0` |
| 5 | `dWorldSelect_c::StateID_IslandMove` | `0x80834238` | `0x807BC9D0`, `0x807BC7E0`, `0x807BC7C0` |
| 6 | `dWorldSelect_c::StateID_IslandAgainMoveWait` | `0x80834278` | `0x807BCA80`, `0x807BCA00`, `0x807BC9F0` |
| 7 | `dWorldSelect_c::StateID_ExitAnimeEndWait` | `0x808342B8` | `0x807BCB80`, `0x807BCAF0`, `0x807BCA90` |
| 8 | `dWorldSelect_c::StateID_DecisionAnimeEndWait` | `0x808342F8` | `0x807BCD60`, `0x807BCCE0`, `0x807BCC00` |
| 9 | `dWorldSelect_c::StateID_SceneChangeWait` | `0x80834338` | `0x807BCD90`, `0x807BCD80`, `0x807BCD70` |
| 10 | `dWorldSelect_c::StateID_World9OnStageStartWait` | `0x80834378` | `0x807BCDF0`, `0x807BCDB0`, `0x807BCDA0` |
| 11 | `dWorldSelect_c::StateID_World9OnStageAnimeEndWait` | `0x808343B8` | `0x807BCED0`, `0x807BCE70`, `0x807BCE00` |
| 12 | `dWorldSelect_c::StateID_World9DeMoEndWait` | `0x808343F8` | `0x807BCF30`, `0x807BCEF0`, `0x807BCEE0` |

The payload triples are literally `{ 0, 0xFFFFFFFF, <handler> }` in all 39
slots (`0x80820EB8` = `00000000 ffffffff 807bc070`, and so on). Rows 9 and 12
use handlers 0x10 apart (`0x807BCD70/80/90`, `0x807BCEE0/EF0/F30`), which are
the short `return`-style stubs for states with no work.

**Step 3 - epilogue.** `addi r11,r1,528` + five `psq_st`/`lfd` pairs restoring
`f27..f31`, `bl 0x802DCDC0` (`_restgpr_28_31`), `lwz r0,612(r1)`,
`mtspr lr,r0`, `addi r1,r1,608`, `blr`.

### Why it was mis-classified

`docs/SPEC_WAVE139_BASES.md` and `docs/SPEC_WAVE140_BASES.md` read the
`DAT_808340F8 = &DAT_808212b0` / `DAT_80834104 = uVar8` ... pattern in the Ghidra
output as "a data table of 44 entries". The raw words show
`bl 0x8015F7C0` and `bl 0x80600160` between each group: every one of the 13
records is *constructed* by two calls, and the 9-word payload is copied through
`r1+0x1B8..r1+0x1D8` before being stored at `obj+0x0C..obj+0x2C`. This is
constructor code; the `.data`-table reading is not supported by the bytes.

---

## 2. `0x8062C8E0` - world-select cell/slot initialiser (ready, medium)

* TSV size **1,804** bytes, **451** words; export
  `work/ghidra/rel-d_basesNP/export/8062c8e0_FUN_8062c8e0.c`
* extent `0x8062C8E0`-`0x8062CFE9`, final `blr` at `0x8062CFE8`
* frame 128 bytes; `_savegpr` at `0x802DCD60`

### Signature

```rust
/// `void f(Self* self)` - `r3` only.
/// Proof: 0x8062C8E0 `bl 0x802DCD60`, then 0x8062C8F4 `or r29,r3,r3`;
/// r4 is overwritten before any read at 0x8062C904 `addi r4,r0,3`.
pub unsafe extern "C" fn fn_8062c8e0(this: *mut u8);
```

### Behaviour

1. **Sub-object at `this+0x70`.** `800C9500(this+0x70)` (constructor), then
   `800C93D0(this+0x70, k, 0)` for `k = 3,4,5,6,1,0,0x1C` and
   `800C9380(this+0x70, 0x1E)` - i.e. `event`/`slot` registration for ids
   3,4,5,6,1,0 and 28, then `0x1E` = 30.
2. **Scene hand-off.** Saves the four words at `0x80354E60..0x80354E6C`
   (`r25/r27/r30/r28`), then calls, in order:
   `800E0380(master, sext(master[0x26]))`, `800CDED0()`,
   `800E1840(master, sext(master[0x26]))`, and restores the four words.
   `master = *(void**)0x8042A040`; the byte is loaded with
   `lbz r0,38(r22)` + `extsb r23,r0` (`0x8062C990`/`0x8062C9A0`) and passed in
   `r4` (`or r4,r23,r23` at `0x8062C9AC`). `r31 = master+0x20` is kept for the
   grid-query calls below.
3. **Ten slot records at `this+0x440`, stride `0x10`.** Each is
   `{ s32 a = 10, s32 b = 42, u16 c = 0, u16 pad, s32 value }`:
   `0x440/0x450/.../0x4D0` get `10`, `+4` gets `0x2A`, `+8` gets `0` (u16).
   `+0xC` is written later (step 8), so initialise it to 0.
4. **Selection seed from the course object.** `course = *(void**)0x80429F7C`;
   `this+0x500 = *(s32*)(course+996)` and `this+0x4F4 = *(s32*)(course+1000)`.
5. **Grid scan** (helps the "unlocked cell" pass):
   `for row = 8 down to 0 { for col = 37 down to 0 {`
   `let hit = if (*(u32*)0x80429F80 & 0x20) != 0 { 800CE600(master+0x20,row,col) }
              else { 800CE640(master+0x20,row,col) };`
   `if (hit as u16) != 0 { fn_8062C770(this, row, col, hit); } } }`
   The fourth argument is the non-zero query result, which is still live in `r6`
   at the call site (`0x8062CAAC rlwinm r6,r3,0,16,31`, `0x8062CABC`).
   `fn_8062C770` walks the 10-entry list at `this+0x448` (`lhz r0,1096(r3)`,
   `cmpl` against `r6`) and appends through `0x8062C690`; it is an *insert*, not
   a plain store, so translate it as an ordered 10-slot list with a bounded tail
   shift.
6. **Bank selection.** `idx = this[0x500]*10 + this[0x4F4]`.
   `if (*(u32*)0x80429F80 & 0x40) != 0` the live bank is `this+0x400`
   ("A"), else `this+0x770` ("B"). Every subsequent access uses
   `bank + n*8` with the 8-byte record `{ u8 x, u8 y, u16 pad, u32 value }`.
7. **Snapshot + clear.** `sel = *(u8*)(bank + idx*8)`; copy 10 records
   `bank + (100+sel+j)*8`, `j = 0..9`, into a 400-byte stack area
   (`r1+8`, 2 groups of 5 records, 40 bytes per group), and clear each source
   record to `{ 10, 42, 0 }` (`stb r7=10`, `stb r6=42`, `stw r5=0`).
   The copy is `mtctr 2` (`addi r4,r0,2` at `0x8062CAF8`; **`rA = 0` is the
   literal zero, not `r0`** - the preceding `rlwinm r0,r4,3,0,28` supplies the
   bank offset, not the count).
8. **Resolve the selected cell.** If `this[0x500] == 10`, scan the 10 slot
   records for one whose `(a & 0xFF, b & 0xFF)` equals the `(x, y)` read at
   step 7, and set `this[0x4F4]` to that record's index
   (`0x8062CD48`-`0x8062CDC8`, 2 x 5 records with `r17 += 0x50`).
   Then for each of the 10 records set `+0xC = 0` and scan the 15-record
   snapshot for a `(x, y)` match, copying the matched `value` into `+0xC`
   (`0x8062CE38`-`0x8062CF20`).
9. **Write back.** 10 records, `mtctr 5` with two stores per iteration:
   `dest = this+0x320 + (0x400 if flag&0x40 else 0x770) + 8*i`, and each record
   is `dest[0] = x`, `dest[1] = y`, `dest[4..7] = value`
   (`0x8062CF3C`-`0x8062CFC0`). So the live bank is rewritten to
   `this+0x720 + 8*i` (A) or `this+0xA90 + 8*i` (B).
10. `fn_8062C800(this, 0, 0x14)` - pack/re-sort helper
    (`0x8062CFC4`-`0x8062CFD0`; `0x8062C800` reads only `r3`, `r4`, `r5`).
11. Epilogue: `addi r11,r1,128`, `bl 0x802DCDAC`, LR restore, `blr`.

### Constants

`10` (0x0A) and `42` (0x2A) are the "empty cell" pair written into both banks;
`0x14` = 20 (record count for the packer); flag bits `0x20` (query variant) and
`0x40` (bank A/B) of `0x80429F80`.

### Callees

`0x802DCD60`, `0x800C9500`, `0x800C93D0`, `0x800C9380`, `0x800E0380`,
`0x800CDED0`, `0x800E1840`, `0x800CE600`, `0x800CE640`, `0x8062C770`,
`0x8062C800`, `0x802DCDAC`.

`0x800CE600`/`0x800CE640` are the two variants of one grid query
`(grid, row, col) -> u16`; only "non-zero" is used here, so the spec is
insensitive to which one the flag selects.

---

## 3. `0x8065DD50` - world-map "witchcraft" object update (conditional, medium)

* TSV size **1,772** bytes, **443** words; export
  `work/ghidra/rel-d_basesNP/export/8065dd50_FUN_8065dd50.c`
* extent `0x8065DD50`-`0x8065E439`, final `blr` at `0x8065E438`
* frame 160 bytes; `_savegpr` at `0x802DCD58`; saves `f28..f31` (qr0)

### Signature

```rust
/// `void f(Self* self)` - r3 only.
/// Proof: 0x8065DD80 `bl 0x802DCD58`, 0x8065DD90 `or r25,r3,r3`;
/// r4 is overwritten at 0x8065DD94 `lbz r4,1012(r28)`.
pub unsafe extern "C" fn fn_8065dd50(this: *mut u8);
```

`this` is the object that owns `+0x9C20` (an embedded "area/rect" record),
`+0x3D0` (timer, `s32`), `+0x3E0` (state, 0 or 1) and `+0x394`
(`dStateMng_c`). The string it binds on completion
(`&DAT_80828D5C`, BSS) and the ids it plays (`0x8019C510`) are the only parts
this lane could not resolve, hence **conditional**.

### Behaviour

1. `fn_8065C100(this)` - reset the embedded easing/phases (same call as its
   sibling `0x8065F090`, section 4).
2. `r29 = (x | -x) >> 31` where `x = *(u8*)(0x807E6EC0 + 0x3F4)` - i.e. a bool
   "one-shot already fired" latch. All later uses of `r29` select between the
   two variants of a value (see step 4 and 6).
3. `d = 800E7E30(this+0x9C20)` (a per-frame scalar, called 4x; cache it),
   `local_98 = (800.0 + this[0xAC]) - 0.5*d`,
   `local_94 = (-1520.0 + this[0xAC]) + 0.5*d`,
   `local_90 = 800B30C0()` (engine time), then
   `this[0x9CC4] = FUN_8065E4E0(&local_90, &local_94, &local_98)` - a clamped
   interpolation across the three samples; constants `0x807C8AE8` = 0.0,
   `0x807C8AEC` = 0.5, `0x807C8AF4` = 1.0, `0x807C8AF8` = 176.0 (2^52).
4. `s = FUN_8065E460(this[0x9CC4], &DAT_807E6E42, &DAT_807E6E44)`; the two
   bounds are the `s16` halves of the word at `0x807E6E40` (= `0x7200007F`, so
   `lo = 127`, `hi = 0x7200` = 29184) - a fixed-point clamp. Then
   `8019C510(*(void**)0x8042A4A8, s)` - the shared "play id" hook.
5. `a = 80162D50(0x21, 0)` (actor of profile `0x21`), `t = 800B30C0()`,
   `k = (s32)(t - a[0xAC])`, `(*(*0x80429D64 + 0x2C))(0x80429D64, 0x31B, (s16)k, 0)`.
6. **State `this[0x3E0] == 0`.**
   `beyond = any(active player p (mask 0x80429CA8, i < 4) : this[0xAC] + 800.0 < p[0xAC])`.
   If the latch `r29 == 0` (`(s32)-r29 < 0`, i.e. latched), for each active
   player: let `e = 800E7E30(this+0x9C20)`;
   `p[0xAC] = max(p[0xAC] + 1.0, (this[0xAC] + 800.0) - 0.25*e)` when
   `(this[0xAC]+800.0) - 0.25*e <= p[0xAC]`, else
   `p[0xAC] = (this[0xAC]+800.0) - 0.25*e`; then
   `800E7ED0(p[0xAC] - 0.75*e, this+0x9C20)`.
   If `beyond`: `8006A730(0x19, 0)`, `8005C930(*(void**)0x80429C94)`,
   `*(u8*)(*(void**)0x8042A070 + 0xC) = 1`, `this[0x9CD0] = 0`, and for each
   active player `p`: `80056C70(p, 0x73)` and
   `this[0x9CD4 + 4*n] = p[4] & 0xF` (n = `this[0x9CD0]++`) - the player-order
   list. Then an insertion sort of that list by `p[0xAC]` using
   `FUN_8065E440(&a, &b)` swaps (compared with `8005F900(id)`), and
   `this[0x9C34] = 800E7EE0(this+0x9C20)`,
   `this[0x9C38] = 1088.0 + (this[0xAC] - 0.5*800E7E30(this+0x9C20))`,
   `this[0x9C44] = 800E80D0(this+0x9C20)`,
   `this[0x9C48] = this[0xB0] + 240.0`,
   `*(u32*)(*(void**)0x80429F10 + 0x15C) = 1`,
   `this[0x3D0] = -1`, `this[0x3E0] = 1`.
7. **State `this[0x3E0] == 1`.**
   Iterate `i = 80162DB0(2, 0)`, `i = 80162DB0(2, i)`:
   * `i[8] == 0x69` (u16 spawn code) and `i[0x408] == 0` and `80070CB0() != 0`
     -> `80040EC0(i)`;
   * `i[8] == 0x68` and `i[0x3D4] == 0` and `80070CB0() != 0`
     -> `80123A00(i)`, `i[0x3D4] = 1`.
   Then move the area along both axes:
   `v = 800E7E50(this[0x9C38], 2.0, this+0x9C20)`,
   `h = 800E8030(this[0x9C48], 0.5, this+0x9C20)`.
   `n = this[0x9CD0]`; `step = 96.0 / (f32)(n+1)` (magic-number divide);
   `x0 = 0.5*96.0 + this[0xAC] + 960.0`;
   for each listed player (index `k`): if `p[0x10D4] & 1`:
   `tx = x0 - step*(k+1)`; `spread = (k>0) ? |p[0xAC]-prev[0xAC]|` adjusted by
   `step` (ratio form) else 1.0; `sz = min(spread * 2.5, 5.0)`;
   if `p[0xAC] < tx`: `local_8c = FUN_8065C210(0.1, p+0xE8, &sz)`,
   `80052080(p, &tx, &local_8c)`, `moved = false`.
   Finally if `v != 0 && h != 0 && moved`:
   `800E7ED0(this[0x9C38], this+0x9C20)`, `800E80C0(this[0x9C48], this+0x9C20)`,
   `(*(*(this+0x394) + 0x18))(this+0x394, &DAT_80828D5C)`.
8. Epilogue: restore `f28..f31` (`lfd` + `psq_st`), `bl 0x802DCDA4`,
   LR restore, `blr`.

### Constants (all `f32` unless noted)

`0x807E72A4 = -1520.0`, `0x807E72B0 = 800.0`, `0x807E72B8 = 1088.0`,
`0x807E72C0 = 240.0`, `0x807E72C4 = 0.5`, `0x807E72C8 = 960.0`,
`0x807E72CC = 2.5`, `0x807E72D0 = 5.0`, `0x807E72D4 = 0.1`,
`0x807E72D8 = 96.0`; `0x807C8AEC = 0.5`, `0x807C8AF4 = 1.0`,
`0x807C8AF8 = 176.0` (2^52), `0x807C8B18 = 0.25`, `0x807C8B1C = 0.75`.
Spawn codes `0x68`, `0x69`; profile ids `0x21`, `2`; event ids `0x73`, `0x19`;
`0x31B`; bound word `0x7200007F`.

### Open items (why conditional)

* `DAT_80828D5C` is in BSS (`0x80822900` + `0x6484`): the `StateID` the object
  switches to is built elsewhere; its name/behaviour is not in the file image.
* The `0x8042A4A8` / `0x80429D64` / `0x8042A070` singletons and the meaning of
  `this+0x9C20`'s accessors (`800E7E30/7ED0/7EE0/7E50/8030/80C0`) are external.
* `0x9C34/0x9C38/0x9C44/0x9C48` are the x-out/x/y-out/y of the camera area;
  the four field names are inferred from their use, not from a symbol.

---

## 4. `0x8065F090` - attract/demo state machine update (conditional, medium)

* TSV size **1,772** bytes, **443** words; export
  `work/ghidra/rel-d_basesNP/export/8065f090_FUN_8065f090.c`
* extent `0x8065F090`-`0x8065F779`, final `blr` at `0x8065F778`
* frame 32 bytes; `_savegpr` at `0x802DCD74`

### Signature

```rust
/// `void f(Self* self)` - r3 only.
/// Proof: 0x8065F0A0 `bl 0x802DCD74`, 0x8065F0A4 `or r30,r3,r3`;
/// r4 is never read before the loop (`or r3,r27,r27` at 0x8065F0D4).
pub unsafe extern "C" fn fn_8065f090(this: *mut u8);
```

Same class as section 3: `this+0x394` is the `dStateMng_c`, `this+0x3D0` the
timer and `this+0x3E0` the 3-valued state. This is the *sibling* that selects
which state to enter.

### Behaviour

1. `fn_8065C100()` then `fn_8065C260(this)`.
2. **Per-player "rescue" pass**, `i = 0..3`: if bit `i` of
   `*(u32*)0x80429CA8`: `p = 8005F900(i)`;
   if `(p[0x10D4] & 1) != 0 && (p[0x10D8] & 2) == 0`:
   * `8004DB40(p, 4) == 0` -> `80051E70(p)`;
   * else -> if `80056CF0(p, 0x5F) == 0` then `80056C70(p, 0x5F)`;
     if `80056CF0(p, 0x72) != 0` then
     `if p[0x348] != 1 { 80051F30(p, 1) }` and
     `if 80052220(p, 0x92) == 0 { 80052170(p, 0x92) }`.
3. **All-players-ready flag**: if every active player (same mask, `i < 4`) has
   `8004DB40(p, 4) != 0`, set `*(u32*)(*(void**)0x80429F10 + 0x15C) = 1`.
4. **State 0** (search phase): `best = 0`; iterate
   `a = 80162D50(0x112, prev)` over the actor list;
   `pi = (*(*(a+0x394) + 0x28))(a+0x394)`;
   `hit = (*(*pi + 0x14))(pi, &DAT_8082917C)`; if `hit != 0` and
   (`best == 0` or `best[0xAC] > a[0xAC]`), `best = a`.
   * if `best == 0` and `this[0x3D0] >= DAT_807E72E8` (fn starts the "give up"
     branch): `c = 80162D50(0x101, 0)` and evaluate four name tests on
     `c`'s vtable-`0x28` object against `0x80C6EDCC`, `0x80C6EE0C`,
     `0x80C6EF4C`, `0x80C6EF8C` combined with `c[0x778] < 1/4/3/3` (twice, in
     two nested priority chains). If the last chain fails,
     `(*(*(c+0x60) + 0xD4))(c, 0x80C6EE0C)`. Then
     `this[0x3D0] = -1`, `this[0x3E0] = 1`.
   * else if `best != 0` and `this[0x3D0] >= DAT_807E72E4`:
     `(*(*(best+0x394) + 0x18))(best+0x394, &DAT_8082917C)`, `this[0x3D0] = -1`.
5. **State 1**: `c = 80162D50(0x101, 0)`; the same three name/threshold tests
   (without the `<1` case); if all pass, `this[0x3D0] = -1`, `this[0x3E0] = 2`.
6. **State 2**: `8005C9D0(*(void**)0x80429C94)`; if non-zero and
   `this[0x3D0] <= best`, `(*(*(this+0x394) + 0x18))(this+0x394, &DAT_80828E1C)`.
7. `bl 0x802DCDC0`, LR restore, `blr`.

### Constants

`DAT_807E72EC` / `0x807E72E8` / `0x807E72E4` are the per-state timeout
thresholds (read from `0x807E72E4..0x807E72EC`); `p[0x778]` thresholds 1, 3, 4;
event ids `0x5F`, `0x72`, `0x92`; profile ids `0x101`, `0x112`.

### Open items (why conditional)

The four name pointers `0x80C6EDCC`, `0x80C6EE0C`, `0x80C6EF4C`, `0x80C6EF8C`
are **outside** this REL and outside `main.dol`'s file-backed ranges (they
resolve into another module's synthetic space), so their text could not be read
here; and `&DAT_8082917C` / `&DAT_80828E1C` are BSS `StateID`s. The control flow
and thresholds are exact; the *names* are not.

---

## 5. `0x8071D500` - world-9 water-level sequence (ready, medium)

* TSV size **1,748** bytes, **437** words; export
  `work/ghidra/rel-d_basesNP/export/8071d500_FUN_8071d500.c`
* extent `0x8071D500`-`0x8071DBD1`, final `blr` at `0x8071DBD0`
* frame 208 bytes; `_savegpr` at `0x802DCD70`; saves `f30/f31` (qr0)

### Signature

```rust
/// `void f(Self* self)` - r3 only.
/// Proof: 0x8071D520 `bl 0x802DCD70`, 0x8071D538 `or r28,r3,r3`,
/// 0x8071D524 `addis r4,r3,2` reads self+0x1A59C immediately.
pub unsafe extern "C" fn fn_8071d500(this: *mut u8);
```

`this` is a large world-map object; the state machine lives at
`this+0x1A59C` (`(s32)this[0x6967]`), the sub-state/step at `this+0x1A488`
(`this[0x6922]`), and the actor flag byte at `this+0x1A59A`.

### Behaviour

```
state = this[0x6967]                       // 0..5
if state == 0:
    if this[0x1A488] == 0:                 // first frame only
        if *(s32*)0x8082FFEC != 3:
            if *(u8*)(this+0x1ACCC) == 0:  // this[0x6B33]
                v = FUN_806601B0(this+0x1A488*4? -> this+0x1A488,
                                 &DAT_808095C8, &DAT_80809660)
                this[0x6B24] = max(this[0x6B24], (f32)v)
            if this[0x6B24] >= 1.0:        // DAT_807CD020
                this[0x6922] = -1 ; this[0x6967] = 1
        else:
            (*(*(this+0x394) + 0x18))(this+0x394, &DAT_80830000)
```

* **state 0** - scale-in: `FUN_806601B0(this+0x6922*4, &DAT_808095C8,
  &DAT_80809660)` returns the interpolation of `this[0x6B24]` toward
  `DAT_80809660` (= 150.0); it takes the max and waits for 1.0.
* **state 1** - build the water: `c = (*(*(this+0x6968)+0xA0))` and
  `(*(*(this+0x69B2)+0xA0))` create `"Wm_we_watersign"` and `"Wm_we_water"`
  at `(x = 800B30C0(), y = 800B30E0(), z = -7500.0)`; a 3-float vector is
  normalised through `8006A3F0(&vec, &src)` and played with
  `80199390(*(void**)0x80429D60, 0x550, *(u32*)this, &vec, 0)`; then if
  `this[0x6922] >= 150` (`DAT_80809664`) it creates the screen object
  `"obj_waterscreen"` (`8023A270(this+0x692E, "obj_waterscreen")`) and its
  draw pair `800E7A80(0.0, 1.0, this+0x6930, this+0x6950, &obj, 0, 0)`, sets
  `*(u8*)(this+0x1A59A) = 1`, `this[0x2C] = *(u32*)0x8082FFE4 + 15`,
  `this[0x6926] = 0.0`, `this[0x6923] = 800B30C0()`,
  `this[0x6924] = 800B30E0() + 320.0*2.0`, `this[0x6925] = -7500.0`,
  `this[0x6922] = -1`, `this[0x6967] = 2`.
* **state 2** - rise: `(*(*(this+0x6930)+0x1C))()` and
  `(*(*(this+0x6950)+0x14))()` (draw/update the screen pair);
  `this[0x6926] = max(this[0x6926] - 0.3, -20.0)` (0x80809674/0x80809678);
  `this[0x6924] += this[0x6926]`; play `0x551` again; when
  `this[0x6924] <= 800B30E0()`: `this[0x6922] = -1`, `this[0x6967] = 3`,
  `this[0x6B25] = -1.0 / (120.0 - 176.0)` (the magic-number pair at
  `0x80809694` / `0x807CD028`).
* **state 3** - sign pop: draws `"Wm_we_waterupsign"` at
  `(800B30C0(), y = 800B30C0() - v, 5500.0)`; plays `0x551`; when
  `this[0x6922] >= 150` or `*(s32*)0x8082FFEC == 3` -> `this[0x6967] = 4`.
* **state 4** - continued rise with the second sign; same draw/play; when
  `this[0x6924] <= 800B30E0() - 320.0*2.0`:
  `*(u8*)(this+0x1A59A) = 0`, `80198C30(*(void**)0x80429D60, 0x552, &vec, 0)`,
  `this[0x6922] = -1`, `this[0x6967] = 5`.
* **state 5** - teardown: draws `"Wm_we_waterup"`; when
  `this[0x6922] >= 15` (`DAT_80809680`) ->
  `(*(*(this+0x394)+0x18))(this+0x394, &DAT_80830080)`.

Epilogue: `addi r11,r1,176`, `lfd f31`, `psq_st f31`, `lfd f30`, `psq_st f30`,
`bl 0x802DCDBC`, LR restore, `blr`.

### Constants

| symbol | value | use |
|---|---|---|
| `0x80809660` | `150.0` (int 150) | state-0 timeout |
| `0x80809664` | `150` (int) | state-1 threshold |
| `0x80809668` | `2.0` | height factor |
| `0x8080966C` | `320.0` | water height |
| `0x80809674` | `-0.3` | rise acceleration |
| `0x80809678` | `-20.0` | rise clamp |
| `0x8080967C` | `150` (int) | state-3 threshold |
| `0x80809680` | `15` (int) | state-5 threshold |
| `0x80809694` | `120` (int) | duration |
| `0x807CD014` | `0.0` | draw arg |
| `0x807CD018` | `-7500.0` | water z |
| `0x807CD01C` | `-1.0` | slope |
| `0x807CD020` | `1.0` | scale target |
| `0x807CD024` | `5500.0` | sign z |
| `0x807CD028` | `176.0` (2^52) | magic number |

Strings (exact, read from the file image): `"Wm_we_watersign"` (`0x808095D0`),
`"Wm_we_water"` (`0x808095E0`), `"Wm_we_waterupsign"` (`0x808095EC`),
`"Wm_we_waterup"` (`0x80809600`), `"obj_waterscreen"` (`0x80809560`), and the
vector constant at `0x808095C8`.

### Notes

* `0x8082FFEC` is a mode/`mDemo` flag; `3` means "skip the sequence"
  (the same value short-circuits states 0, 1 and 3).
* `this[0x2C]` and `this[0x6B25]` are the only two fields written outside the
  `0x6922`-`0x6B33` window; `this[0x2C]` is a timer, `this[0x6B25]` a scale
  increment.
* The `FUN_806601B0(this+0x6922*4, ...)` argument is the *sub-state id*, so the
  first argument of the easing helper is `this+0x1A488`, not a float.

---

## 6. `0x8067FC80` - water-surface actor create (ready, medium)

* TSV size **1,676** bytes, **419** words; export
  `work/ghidra/rel-d_basesNP/export/8067fc80_FUN_8067fc80.c`
* extent `0x8067FC80`-`0x80680309`, final `blr` at `0x80680308`
* frame 304 bytes; `_savegpr` at `0x802DCD54`; saves `f29..f31` (qr0)

### Signature

```rust
/// `void f(Self* self)` - r3 only.
/// Proof: 0x8067FCA8 `bl 0x802DCD54`, then 0x8067FCB0 `lwz r9,4(r3)`
/// (reads the packed parameter word at self+4); r4 is overwritten at
/// 0x8067FCB8 `addis r4,r0,17200`.
pub unsafe extern "C" fn fn_8067fc80(this: *mut u8);
```

This is the `create`/`init` of an actor whose state is fully derived from the
`u32` parameter word at `self+4`.

### Behaviour

Let `w = *(u32*)(self+4)`.

```
self[0x348]  = w & 1
self[0x374C] = (w >> 8) & 0xF
self[0x3730] = -(s16)(((u16)(w >> 12) & 0xF) << 12)      // see evidence
self[0x104]  = self[0x3730]
self[0xB4]   = -10.0                                    // DAT_807C9658
if self[0x38F] == 1: self[0xB4] = -3490.0               // DAT_807C965C
self[0xB4]  -= 10.0 * (f32)((w >> 16) & 0xF)            // DAT_807C9650
self[0xDC] = self[0xE0] = self[0xE4] = 1.0              // DAT_807C9648
self[0xD0] = self[0xD8] = 0.0                           // DAT_807C9664
```

`d = (f32)((w >> 4) & 0xF)` is the vertical offset in 1/25-of-400 units
(`local_80 - 2^52`), and `f4 = 10.0 * (1.0 / 25.0)`
(`DAT_807C9650 * DAT_807C9648 / DAT_807C964C`) gives
`self[0x3732] = (s16)(f4 * 32.0 * (1.0 + d))`
(`fVar4 * fVar6 * (fVar5 + (float)dVar23)`, `0x807C9654` = 32.0).

Then:

1. `FUN_80680560(self)` and `FUN_80680610(self)` - the two sibling sub-inits.
2. **8 wave segments** at `self+0x3D4`, stride `0x74`:
   `800DB740(400.0, 0.0, -400.0, 0.0, -400.0, &seg, self,
             (s16)(16768 + self[0x3730]), 0, 0, 1, 1)`;
   then `seg[0x4A] (=+0x41E) = DAT_80429D28`, `seg[0x47] (=+0x41B) = 0x0D`,
   `seg[0x3C] (=+0x410) = 0x10000`, `800DAE80(seg)`.
   (`16768 = 0x4180`; `iVar16 += 0x2000` per segment, i.e. the 8 segments wrap
   the map 8 x 45 deg.)
3. `self[0x3724] = d - 48.0` (`DAT_807C9668`).
4. For every 8-byte record `r` in `[self+0x7F4, self+0xC34)` with stride `0x88`
   (`(self+0xCBB - p)/0x88` records):
   `r[0x80] = self[0x100]`, `r[0x82] = self[0x102]`,
   `r[0x84] = self[0x104] - n*0x2000` with `n` incrementing.
   Immediately afterwards `self[0xD4] = -d` (`0x8068004C stfs f0,212(r19)`, the
   only store to that field) - so `d` is used as a *negative* vertical bias
   before the node loop. (`self[0x3724] = d - 48.0` is `0x8067FE64`.)
5. **24 render nodes**, two per iteration: node `A` at `self+0xC34 + 0xE4*i`,
   node `B` at `self+0x2194 + 0xE4*i`, with the shared transform record
   `self+0x36F4`; each iteration `i = 0..23`:
   `8007FAD0(A, self, &uv0, &uv1, T[3i], T[3i+1], T[3i+2], 1, self[0x38F], &f)`
   and the same for `B`; `self[0x36F4] = (s16)(0x2000 * phase)` through
   `8007FD80`; then
   * `i % 3 == 0` -> `A[0xCC] = 10` and `B[0xCC] = 10` (`+0xD00` / `+0x2260`);
   * otherwise `A[0xD0] |= 0x800000`, `B[0xD0] |= 0x800000`
     (`+0xD04` / `+0x2264`);
   * always `A[0xD0] |= 0x380000`, `B[0xD0] |= 0x4380000`;
   * `8007F900(A)`, `8007F900(B)`.
   The table `T` at `0x807ECB48` is 24 triples of code pointers, in order:
   `8067FC70/8067FC60/8067FC40`, `8067FC30/8067FC20/8067FC00`,
   `8067FBF0/8067FBE0/8067FBC0`, `8067FBB0/8067FBA0/8067FB80`,
   `8067FB70/8067FB60/8067FB40`, `8067FB30/8067FB20/8067FB00`,
   `8067FAF0/8067FAE0/8067FAC0`, `8067FAB0/8067FAA0/8067FA80`,
   `8067FA70/8067FA60/8067FA40`, `8067FA30/8067FA20/8067FA00`,
   `8067F9F0/8067F9E0/8067F9C0`, `8067F9B0/8067F9A0/8067F980`,
   `8067F970/8067F960/8067F940`, `8067F930/8067F920/8067F900`,
   `8067F8F0/8067F8E0/8067F8C0`, `8067F8B0/8067F8A0/8067F880`,
   `8067F870/8067F860/8067F840`, `8067F830/8067F820/8067F800`,
   `8067F7F0/8067F7E0/8067F7C0`, `8067F7B0/8067F7A0/8067F780`,
   `8067F770/8067F760/8067F740`, `8067F730/8067F720/8067F700`,
   `8067F6F0/8067F6E0/8067F6C0`, `8067F6B0/8067F6A0/8067F680`.
   Each triple is 12 bytes apart and each pointed-to object 16 bytes apart, so
   the whole 24-triple run spans `0x807ECB48..0x807ED168`.
6. `self[0x374D] = 0xFF`.
7. `self[0x3728] = 80066890(*(void**)0x80429D40, &vec4(x=self[0xAC],
   y=self[0xB0]-d, z=d, w=0))` - the looping sound handle.
8. `(*(*(self+0x394)+0x18))(self+0x394, &DAT_8082A728)` - set the actor state.
9. Epilogue: `addi r11,r1,256`, three `lfd`/`psq_st` pairs, `bl 0x802DCDA0`,
   LR restore, `blr`.

### Constants

`0x807C9648 = 1.0`, `0x807C964C = 25.0`, `0x807C9650 = 10.0`,
`0x807C9654 = 32.0`, `0x807C9658 = -10.0`, `0x807C965C = -3490.0`,
`0x807C9660 = 400.0`, `0x807C9664 = 0.0`, `0x807C9668 = 48.0`,
`0x807C966C = -80.0`, `0x807C9670 = -18.0`, `0x807C9674 = -48.0`,
`0x807C9678 = 18.0`, `0x807C967C = -30.0`, `0x807C9680 = -2.0`,
`0x807C9684 = -16.0`, `0x807C9688 = 16.0`, `0x807C968C = 30.0`,
`0x807C9690 = -32.0`, `0x807C9698 = 176.0` (2^52).
Segment flags `0x10000`, node flags `0x380000` / `0x4380000` / `0x800000`,
draw id `10`, spawn `0x0D` into `+0x41B`.

### Notes

The `psq_st`/`lfd` prologue shows this body was compiled with `f29..f31` live
across calls; the three "GQR reconstruction" blocks in the export are the qr0
spills, not quantisation. All three `I` fields are 0.

---

## 7. `0x807740D0` - stage object spawner (conditional, medium)

* TSV size **1,656** bytes, **414** words; export
  `work/ghidra/rel-d_basesNP/export/807740d0_FUN_807740d0.c`
* extent `0x807740D0`-`0x80774745`, final `blr` at `0x80774744`
* frame 96 bytes; `_savegpr` at `0x802DCD58`

### Signature

```rust
/// `void f(Course* self)` - r3 only.
/// Proof: 0x807740E0 `bl 0x802DCD58`, 0x807740E8 `or r21,r3,r3`;
/// the next instruction uses 0x807740E4 `lwz r0,13196(r3)` (self+0x338C).
pub unsafe extern "C" fn fn_807740d0(this: *mut u8);
```

`self+0x3388` is the stage/area mode (`1, 3, 4, 5, 6` are special-cased) and
`self+0x338C` the current zone index, which scales three separate per-zone
tables: `+0x1A8 + 0xBF8*i` (3064 B), `+0x3394 + 0x16518*i` (91,384 B) and
`+0x5C7F4 + 0x3F08*i` (16,136 B).

### Behaviour

1. `a = 80168400(self + 0x1A8 + self[0x338C]*0xBF8)` -> `local_54` (the zone's
   object descriptor block).
2. `800E0380(*(void**)0x8042A040, 0xFFFFFFFF)`.
3. `in_special = (*(u8*)0x8042A24E == self[0x338C])`; if so
   `skip = 800FCBB0()` else `skip = 0`.
4. `FUN_80774750(self)` - builds the 4-byte spawn name `{'W', self[0x3388]+'1',
   'S', 0}` (`0x80774764`..`0x80774794`) and spawns it through the generic
   path; call it unconditionally first.
5. **Main loop** `for (n = 0; ; n++)`:
   * `cnt = 8023B360(&local_54)`; if `cnt <= n` -> **finish**:
     * `actor = 80162D50(0x2A7, 0)`; if 0, create it:
       `800F29F0(0x2A7, 0x30000, &vec3(*0x80377DC8, *0x80377DCC,
       *0x80377DD0), 0)` and set `actor[0x184] = 0`;
     * `mode = self[0x3388]`:
       - `1` or `3` -> `800F2A00(0x29C, self, 0, 0, 0)`;
       - `6` -> `800F2A00(0x2A0, self, k, 0, 0)` for `k = 0..2`,
         `800F2A00(0x2A1, self, 0, 0, 0)`, then
         `800F2A00(0x2A2, self, k, 0, 0)` for `k = 0..1`;
       - `4` -> `800F2A00(0x2AD, self, 0, 0, 0)`; if
         `800FC970(self[0x3388]) == 0` then
         `800F2A00(0x270, self, 0, self+0xAC, self+0x100)`, else
         `800BBC50(course, self[0x3388], 0)` and
         `800BBC60(course, self[0x3388], 0)`;
       - `5` -> `800F2A00(0x2A2, self, 3, 0, 0)`;
     * `FUN_80761510(0)`, `802DCDA4`, `blr`.
   * `e = 8023B2A0(&local_54, n)` -> the n-th spawn entry.
     `name = e[8] ? e + e[8] : NULL` (REL-relative string offset),
     `sub  = e[0x5C] ? e + e[0x5C] : NULL`.
   * `id = 800F53A0(self + 0x3394 + self[0x338C]*0x16518, name)`; the low byte
     of the result is the object id; `uVar7 = (id & 0xFF) << 8`.
   * **`name[0] == 'K'`**: clear bit 8 of `e[0x14]`; nothing is spawned.
   * **`name[0] == 'F'`**: `801006C0(self, n, &stackvec)`; then
     - if `800F57A0(table, id, 2) != 0` -> `uVar7 |= 0xFF`;
     - if `800F57A0(table, id, 0x400) != 0` -> spawn
       `800F29F0(0x282, uVar7 | ((name[1]-'a') << 16) | 0xFF, &vec, 0)`;
     - else if `800F57A0(table, id, 0x100000) != 0` -> `800F29F0(0x2A5,
       name[3]-'1', &vec, 0)`;
     - else if `800F57A0(table, id, 0x800) != 0` -> `800F29F0(0x278,
       uVar7 | (name[3] != '0' ? 0x10000 : 0) | 0xFF, &vec, 0)`;
     - else nothing spawns;
     then clear bit 8 of `e[0x14]`. This is a strict priority chain: the
     `0x400` test wins, `0x100000` is tried only if `0x400` is clear, and
     `0x800` only if both are clear.
   * **`name[0] == 'W' && name[2] == 'S'`**: fall through to the generic
     "no spawn" tail.
   * **`name[0] == 'W'`** with `name[2]` in
     `{'0'..'9','A','C','G','K','T','W','X'}`: `uVar3 = 800F31E0(self +
     0x5C7F4 + self[0x338C]*0x3F08, name)` (a 1-byte "world object state"),
     `801006C0(self, n, &vec)`, `uVar7 |= uVar3 & 0xFF`, then
     `spawn = 800F29F0(0x27E, uVar7, &vec, 0)`;
     - if `800FCA50() != 0`: if `800FC9C0() != 0 &&
       uVar3 == *(u8*)(actor + (self[0x3388] & 0xFF) + 0x395) &&
       800FCA20() != 0`: `a = 80162D50(0x279,0)`;
       if `a[0x200] != 0`: `800F2A00(0x2A7, spawn, uVar7 | 0x20000,
       spawn+0xAC, 0)` and `a2[0x184] = n`;
     - else (`800FCA50() == 0`): if `uVar3 == *(u8*)(actor +
       (self[0x3388] & 0xFF) + 0x395)`: `flag = (skip != 0 ||
       800FCAE0() == 0) ? 0 : 1`, `uVar7 |= flag << 16`,
       `800F2A00(0x2A7, spawn, uVar7, spawn+0xAC, 0)`, `[0x184] = n`;
       if `skip != 0 && uVar3 == *(u8*)(actor + (self[0x3388] & 0xFF) + 0x39F)`:
       `800F2A00(0x2A7, spawn, uVar7 | 0x20000, spawn+0xAC, 0)`, `[0x184] = n`;
     - clear bit 8 of `e[0x14]`; `goto end_of_iteration`.
   * **generic tail** (all other names): `s = 800F53A0(table, name)`; if
     `s < 0` -> `FUN_80774A60(self, &e)` (report an unknown object), else
     `spawn = FUN_80774880(self, &e, s)`.
   * `end_of_iteration`: `if (spawn != 0) spawn[0x184] = n`.

### Constants / ids

Spawner entry points: `0x270, 0x278, 0x279, 0x27E, 0x282, 0x29C, 0x2A0,
0x2A1, 0x2A2, 0x2A5, 0x2A7, 0x2AD`; profile `0x2A7`; spawn flags `0x30000`,
`0x20000`, `0x10000`, `0xFF`; area modes `1, 3, 4, 5, 6`; object-code prefix
bytes `'K'(0x4B)`, `'F'(0x46)`, `'W'(0x57)`, `'S'(0x53)`.

### Open items (why conditional)

* `self+0x3388` semantics ("area mode" vs "world id") and the three per-zone
  strides are inferred from use; the spec keeps the raw arithmetic so the Rust
  translation does not depend on the naming.
* `0x801006C0`, `0x800F53A0`, `0x800F57A0`, `0x800F31E0`, `0x800FCA50/C9C0/
  CA20/CAE0`, `0x800FC970`, `0x800BBC50/60`, `80162D50/DB0` are all outside this
  REL; the spawn-name grammar (`K`, `F`, `W`+digit, `WS`) is exact.
* `FUN_80774750`, `FUN_80774880`, `FUN_80774A60`, `FUN_80761510` are
  same-module neighbours and are themselves untranslated rows; only their
  argument shapes are fixed here.

---

## 8. `0x806A24F0` - world-map Jr. "miss-hit" response (conditional, medium)

* TSV size **1,624** bytes, **406** words; export
  `work/ghidra/rel-d_basesNP/export/806a24f0_FUN_806a24f0.c`
* extent `0x806A24F0`-`0x806A2B45`, final `blr` at `0x806A2B44`
* frame 176 bytes; `_savegpr` at `0x802DCD74`; saves `f30/f31` (qr0)

### Signature

```rust
/// `void f(Actor* self, Actor* other)` - **two** pointers.
/// Proof: 0x806A2510 `bl 0x802DCD74` (preserves r3/r4), then
/// 0x806A2514 `lfs f2,172(r4)` and 0x806A251C `lfs f0,172(r3)`.
/// Ghidra prints the incoming pair as one `undefined8` return:
/// `iVar5 = uVar13 >> 32` (r4) and `iVar6 = (int)uVar13` (r3).
pub unsafe extern "C" fn fn_806a24f0(a: *mut u8, b: *mut u8);
```

`a` (`r3`, printed `iVar6`) and `b` (`r4`, printed `iVar5`) are two actors;
the three world-map Jr. effect names at `0x807F2780` / `0x807F2798` /
`0x807F27A8` - `"Wm_jr_misshit_ss"`, `"Wm_jr_misshit_s"`, `"Wm_jr_misshit"` -
point at this routine.

### Behaviour

1. `dx = a[0xAC] - b[0xAC]`, `dy = a[0xB0] - b[0xB0]`
   (`0xAC`/`0xB0` are the x/y of the actor's position).
2. `len = sqrt(dx*dx + dy*dy)` (`802BE470`); `nx = dx * (1.0/len)`,
   `ny = dy * (1.0/len)` - normalise.
3. For each actor `p` in `{b, a}`:
   * `dot = nx*p[0xE8] + ny*p[0xEC]`; if `dot <= 0.0`, negate it.
   * if `(*(*(p+0x60) + 0x31C))(p) != 0`, `dot *= 1.8` (`0x807CA3A8`).
   * `kind = if (*(*(p+0x60) + 0x2D4))(p) != 0 { 2 }
            else { FUN_806A22C0(dot) }`.
   * `v = vec2(p[0xE8], p[0xEC])`; `802BE640(&v)` (normalise);
      += (nx, ny) with the sign taken per actor: - for , + for ;
     `802BE640(&v)` again.
   * if `(*(*(p+0x60) + 0x2D4))(p) != 0 && v.x > 0.0`:
     `v.x = 1.0` (or `-1.0`) - clamp to the unit square.
   * `out_p = v * scale[kind]` where `scale` is the table at `0x807CA390`
     flattened: `{ (1.5, 1.0), (3.0, 2.0), (5.0, 3.0) }`
     (`0x807CA3A0`, `0x807CA398`, `0x807CA3A4`, `0x807CA37C`, `0x807CA2DC`,
     `0x807CA2E0`).
4. Apply the push, unless the actor is AI-controlled (`vtable+0x2D4 != 0`).
   For `a` (`r28` in this block): if `out.x * a[0xE8] < 0.0` then
   `a[0x11C] = 0.0`, and if `out.y * a[0xEC] < 0.0` then `a[0x114] = 0.0`;
   then `(*(*(a+0x60) + 0x2F0))(a, &out_a)`. Then the same for `b`, **but the
   two clamp stores keep writing `a`'s fields**: the shipped words are
   `0x806A28C0 stfs f0,284(r28)` and `0x806A28DC stfs f0,276(r28)` while the
   object being pushed is `r27`. This is not a decompiler artifact - keep the
   asymmetry (`b` never receives the `0x11C`/`0x114` clear).
5. `ta = (*(*(a+0x60)+0x314))(a)`, `tb = (*(*(b+0x60)+0x314))(b)`,
   `t = (f32)((f32)(ta + tb) - len) + 2.0` (`0x807CA37C`).
   Split it by AI state (`a_ai`, `b_ai` from vtable `+0x2D4`):

   | case | shift applied to `b` (`r4` actor) | shift applied to `a` (`r3` actor) |
   |---|---|---|
   | `a_ai` | `2.0 * t` | `0.0` |
   | `b_ai` (and `!a_ai`) | `0.0` | `2.0 * t` |
   | neither | `t` | `t` |

   The exported C names these `dVar10`/`dVar12`; `dVar10` is the `b` shift and
   `dVar12` the `a` shift. The three cases are the three paths at
   `0x806A2948`-`0x806A2990` (`fmuls f4,f0,f30` = `2.0*t`, `lfs f7,184(r29)`
   = 0.0, `fmr f4,f30`/`fmr f7,f30` = `t`,`t`).
6. `b[0xAC] -= shift_b * nx`, `b[0xB0] -= shift_b * ny`;
   `a[0xAC] += shift_a * nx`, `a[0xB0] += shift_a * ny`
   (`a[0xB4]` is rewritten with its own value - a no-op the compiler kept).
7. `kindA = (s32)(DOM * (a? ))`; the two scaled kinds are compared and the
   smaller is kept (`(f32)(x - 176.0) <= (f32)(y - 176.0)`), giving
   `iVar4 = (s32)(uVar3 ^ 0x80000000) - 176` - the "bigger of the two
   effect ids", then:
   `if (*(u8*)(b+0xE8C) == 0) FUN_806A2300(&pos, iVar4)` where
   `pos = (b[0xAC] + nx*(*(*(b+0x60)+0x314))(b),
           b[0xB0] + ny*(*(*(b+0x60)+0x314))(b), b[0xB4])`.
8. If `(*(*(a+0x60)+0x320))(a) == 1 && iVar4 == 2`:
   `s = (*(*(a+0x60)+0x3B4))(a)`; `o = (*(*(*(s+0x5F8))+0x14))()`;
   if `o != 0`, `FUN_8069A910(a)`.
9. Epilogue: `addi r11,r1,144`, `lfd f31`, `psq_st f31`, `lfd f30`,
   `psq_st f30`, `bl 0x802DCDC0`, LR restore, `blr`.

### Constants

`0x807CA3A0 = 1.5`, `0x807CA398 = 1.0`, `0x807CA3A4 = 3.0`,
`0x807CA37C = 2.0`, `0x807CA2DC = 5.0`, `0x807CA2E0 = 3.0`,
`0x807CA390 = 0.0`, `0x807CA39C = -1.0`, `0x807CA3A8 = 1.8`,
`0x807CA3B0 = 176.0` (2^52 magic). Actor offsets: `+0xAC`/`+0xB0`/`+0xB4`
position, `+0xE8`/`+0xEC` direction, `+0x114`/`+0x11C` velocity clamps,
`+0xE8C` a "dead/no-effect" flag.

### Open items (why conditional)

* The vtable slots `+0x2D4`, `+0x2F0`, `+0x314`, `+0x31C`, `+0x320`, `+0x3B4`
  belong to the actor's other module and were not resolved here; the spec names
  them by slot only.
* `FUN_806A22C0` (effect-id lookup) and `FUN_806A2300` (effect spawn) are
  same-module rows that are still untranslated; their argument shapes are
  fixed (`f32 -> u32` and `(&vec3, s32)`), their bodies are not.
* Step 5's split of `t` between the two actors has three branches
  (`a` AI / `b` AI / neither). The literal code is
  `dVar12 = t; if (a_is_ai) { shift_a = 2.0*t; shift_b = 0; }
   else if (b_is_ai) { shift_a = 0; shift_b = 2.0*t; } else { both = t; }`.
  The exported C folds `t` into `dVar10`/`dVar12`; both are `2.0`-scaled, so the
  pre-scale value in the "both" case is `t` and in the single-AI cases `2.0*t`.

---

## Verification performed

1. **Candidate set** re-derived from `functions.tsv` (5,615 rows), the 16,725
   ledger addresses, and the `#{1,6}` headings of all 130 `docs/*.md`:
   2,011 rows remain; the eight above are ranks 1-8 by `size`.
2. **No overlap** with `ledger/functions.csv`, with any spec heading
   (including wave 140's twelve), or with `fn_<addr>` in `src/game/bases.rs`.
3. **Sizes** match the TSV exactly; each `blr` was located and the one 4-byte
   pad case was documented.
4. **Instruction spot-checks** against the raw REL at
   `address - 0x80600000` (0xFFF below the ELF offset) agreed word for word at
   `0x807BD110`, `0x807BD148`, `0x807BD158`, `0x807BD1C0`, `0x807BD124`.
5. **Data reads** (constants, strings, tables) were taken from the ELF's
   file-backed segments; every address quoted in this document is inside
   `0x806000F0..0x808228B8` except the BSS labels explicitly marked as such
   (`0x80828D5C`, `0x8082917C`, `0x8082A728`, `0x8082FFE4`, `0x80830000`,
   `0x80830080`, `0x808340E8`, `0x80834428`).
6. No GQR3 use; all `psq` sites are `qr0` prologue spills.

## Hand-off

The four **ready** bodies (`0x807BD110`, `0x8062C8E0`, `0x8071D500`,
`0x8067FC80`) can be translated as-is: every branch, offset, constant and
callee argument is pinned above. The four **conditional** bodies need one
external fact each before translation:

| address | missing fact |
|---|---|
| `0x8065DD50` | the `StateID` at `0x80828D5C` and the `0x9C20` accessor semantics |
| `0x8065F090` | the four name strings at `0x80C6EDCC/EE0C/EF4C/EF8C` |
| `0x807740D0` | the `+0x3388` area-mode meaning and `0x800F53A0`'s id space |
| `0x806A24F0` | the actor vtable slots `+0x2D4/+0x2F0/+0x314/+0x31C/+0x320/+0x3B4` |
