# Wave 144 - `game::bases` - the eight largest remaining bodies (1.0 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/table readers live under
`%TEMP%\opencode` (`ppc.js`, `w144_scan.js`, `tables.js`), never in the
repository.

Waves 128-143 claimed 128 rows out of
`work/ghidra/rel-d_basesNP/export/functions.tsv`. This wave continues one rank
lower. The window is now 0.97-1.04 KB per body (8,060 bytes / 2,015
instructions), so every row gets a full field map, a numbered behaviour list and
a constant table.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80609440` | 1,040 | 260 | `80609440_FUN_80609440.c` | ready (medium) |
| 2 | `0x80757320` | 1,032 | 258 | `80757320_FUN_80757320.c` | ready (medium) |
| 3 | `0x80741510` | 1,016 | 254 | `80741510_FUN_80741510.c` | ready (medium) |
| 4 | `0x8077AC30` | 1,012 | 253 | `8077ac30_FUN_8077ac30.c` | ready (medium) |
| 5 | `0x80786EA0` | 1,008 | 252 | `80786ea0_FUN_80786ea0.c` | ready (medium) |
| 6 | `0x807886E0` | 1,004 | 251 | `807886e0_FUN_807886e0.c` | ready (medium) |
| 7 | `0x8065C370` | 976 | 244 | `8065c370_FUN_8065c370.c` | ready (high) |
| 8 | `0x80628A00` | 972 | 243 | `80628a00_FUN_80628a00.c` | ready (high) |

**Ready: 8. Conditional: 0.**

Every `bytes` value is the `size` column of the TSV verbatim, and in all eight
cases the size is the true instruction span: the decoder found the final `blr` at
`address + size - 4` in each of the eight bodies (`0x8060984C`, `0x80757724`,
`0x80741904`, `0x8077B020`, `0x8078728C`, `0x80788AC8`, `0x8065C73C`,
`0x80628DC8`). There is no padding tail and no `halt_baddata` cut this wave.

### Verdict rule used here

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image, every callee is named by address (or by its ledger name), and
  the body can be written as code with no assumed calling convention beyond the
  standard PowerPC EABI. Data that lives in `main.dol` BSS (`0x8035xxxx`,
  `0x8042xxxx`) is an *input*, not an unknown constant, and is named by address.
* **conditional** - exactly one named input would still be missing. No row this
  wave meets that test. The three "contract" assumptions that a reviewer may
  want to see are spelled out per section: the out-vector contract of the
  virtual method at `0x8077AC30`'s `+0x29C` object, the `f32` return of the
  `+0x60` vtable slot `0x88` in `0x807886E0`, and the resource-slot layout read
  by `0x8065C370`. Each is fixed by how the body consumes the result, so each is
  a named contract rather than an unknown.

### This is the list wave 143 predicted, exactly

Wave 143's "next eight" list started at `0x80609440` and ended at
`0x80628A00` / `0x80630820` (972 each). Re-deriving the candidate set from the
current tree gives **1,973** rows, and the first eight are byte-for-byte that
list. The residual difference from wave 143's count is heading bookkeeping -
wave 143 added eight `## N. 0xADDR` headings to its own document, and the
two lanes' heading regexes differ by four addresses. The selection is
independent of that bookkeeping: every row below was re-tested against
`ledger/functions.csv` (exact `address` field) and against every `#{1,6} `
heading in `docs/*.md` before being claimed.

The 972-byte tie at the cut is broken by address ascending, so `0x80628A00` is
claimed and `0x80630820` is left as the ninth. Applying the same filter after
this wave, the next eight are: `0x80630820` (972), `0x80797D30` (960),
`0x8077DC80` (936), `0x8077A4B0` (924), `0x8077C670` (924), `0x80669560` (916),
`0x8070FAB0` (912), `0x807BFFA0` (904).

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
Literal pools in `0x807Cxxxx` / `0x807D0xxx` resolve through the same rule and
are read out verbatim below. `0x8035xxxx`, `0x8042xxxx` and `0x8043xxxx` are
`main.dol` BSS and read as zero in the shipped image; they are named, never
guessed.

**Decoder.** A throwaway PowerPC decoder was written under `%TEMP%\opencode`
and validated against the facts `docs/REL_ELF_MAPPING.md` pins:

* `psq_l` = 56, `psq_st` = 60; quantiser index in bits 12-14, `W` in bit 15.
  Opcodes 58/59 are **not** paired-single: 59 is the single-precision float
  family (`fadds`/`fmuls`/`fsubs`/`fdivs`), which the first decoder pass
  misclassified. After the fix, the eight bodies contain exactly **two** `psq_`
  words - the `f31` spill pair in row 5 - and both are `qr0, W=0`.
* For opcode 4 the extended opcode is five bits (`(word >> 1) & 0x1F`).
* `addi`/`addis` with `RA = 0` are `li`/`lis` (the literal zero, not `r0`).
  Several branch constants in these bodies (`li r0,2`, `li r0,1`, `li r0,0x1E`)
  only read correctly with that rule; Ghidra's export already applies it.
* `xoris` XORs the **upper** 16 bits, so the CodeWarrior signed-to-double ladder
  `lis r0,0x4330; ...; xoris r0,r0,0x8000; lfd` produces `2^52 + 0x80000001`
  and subtracts `2^52 + 0x80000000`, i.e. a clean `1.0`. Reading it as a
  low-half XOR turns the ladder into a nonsense `-0x7FFF7FFF`.

Register recovery: `0x802DCD40..0x802DCD74` are `__savegprlr_N` helpers and
`0x802DCDA0..0x802DCDC0` the matching `__restgprlr_N`; the incoming `r3`
survives all of them. `0x802DCD58` is `__savegprlr_20` (saves `r20..r31`, writes
nothing to `r3`), which is why row 8's Ghidra signature has lost its argument.

---

## 1. `0x80609440` - the pane-visibility/state refresher

**Evidence.** `work/ghidra/rel-d_basesNP/export/80609440_FUN_80609440.c`,
1,040 bytes, 260 words, synthetic `0x80609440`.

**What it is.** A UI-screen state refresher. Eleven pane pointers live at
`+0x244`, `+0x24C`, `+0x250`, `+0x254`, `+0x258`, `+0x25C`, `+0x260`, `+0x264`,
`+0x270`, `+0x274`, `+0x278`; every one of them is a BRK pane whose `+0xBB`
byte bit 0 is the visibility flag (`pane_set_flag0` / `fn_80620c30` in
`src/game/stage.rs`). Two selectors at `+0x27C` and `+0x280` pick a message pair; the
body formats both numbers through `0x800CDDB0` and stamps the strings through
the six-argument message front-end `0x800C9A60`, using the writer root returned
by `0x800CDC60` (`fn_800cdc60` in `src/game/stage.rs`). The same object shape appears in
`fn_80620b60` in `src/game/bases.rs`, so the roles of the text calls are
fixed by an already-translated sibling.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x244` | pane* | primary page pane; shown unless an early-out fires |
| `+0x24C`, `+0x258` | pane* | flag bit 0 pair (state 0) |
| `+0x250`, `+0x25C` | pane* | flag bit 1 pair (state 1) |
| `+0x254`, `+0x260` | pane* | flag bit 2 pair (state 2) |
| `+0x264`, `+0x278` | pane* | "gate passed" pair (second gate only) |
| `+0x268` | pane* | message target pane |
| `+0x270`, `+0x274` | pane* | the two numbered message panes |
| `+0x27C` | `i32` | first selector (3, 5, 7 are special) |
| `+0x280` | `i32` | second selector (0x14, 0x15, 0x16, 0x17, 0x25, 0x2A) |
| `+0x284` | `i32` | signed flag word; negative = hide all six pair panes |
| `+0xBB` (of any pane) | `u8` | bit 0 = visible |

### Exact behaviour

Let `A = this->+0x27C`, `B = this->+0x280`, and `gate(x, y) = 0x800FAE40(x, y)`
(a two-key predicate; address-only). All three `gate` calls re-read `A`/`B`
from memory; none is cached.

1. Prologue: `stwu r1,-0x30`; save LR `+0x34`, `r31` `+0x2C`, `r30` `+0x28`;
   `r30 = this`. Epilogue at `0x80609838..0x8060984C`.
2. **Early-out 1** (`0x80609458..0x8060949C`). If
   `(A == 3 || A == 5) && B == 0x25 && gate(A, 0x17) == 0` then
   `pane(+0x244)->+0xBB &= 0xFE; return;`.
3. **Early-out 2** (`0x806094A0..0x806094D8`). Else if
   `A == 7 && B == 0x17 && gate(7, 0x25) == 0` then
   `pane(+0x244)->+0xBB &= 0xFE; return;`.
4. **Main arm** (`0x806094DC..`).
   `writer = 0x800CDC60()` (the shared `0x80359CB4` message object).
   `pane(+0x244)->+0xBB = (b & 0xFE) | 1`.
   `0x800CDDB0(A + 1, buf, 0x10, 1, 0)` - decimal text into the 16-byte stack
   buffer.
   `0x800C9A60(pane(+0x268), writer, 2, 0x3F, 1, buf)`.
5. If `B < 0x14` (`0x80609534..0x806095A0`):
   `pane(+0x274)->+0xBB |= 1`, `pane(+0x270)->+0xBB &= 0xFE`,
   `0x800CDDB0(B + 1, buf, 0x10, 1, 0)`,
   `0x800C9A60(pane(+0x274), writer, 2, 0x35, 1, buf)`.
   Then go to step 8.
6. Else (`0x806095A4..0x80609660`): `pane(+0x274)->+0xBB &= 0xFE`,
   `pane(+0x270)->+0xBB |= 1`, and pick the message id:
   ```
   if (B - 0x15 <= 1)      id = 0x36;
   else if (B - 0x17 <= 1) id = (A == 7) ? 0x3B : 0x37;
   else if (B == 0x25)     id = 0x39;
   else if (B == 0x14)     id = 0x38;
   else if (B == 0x2A)     { pane(+0x244)->+0xBB &= 0xFE; return; }
   else                    id = 0;
   ```
   `0x800C9A60(pane(+0x270), writer, 2, id, 0)` - five arguments at this site
   (`r8` is stale; the `0` flag makes it unused, which is exactly how the export
   prints it).
7. **Flag block** (`0x80609664..0x806097D4`). If `(i32)this->+0x284 < 0`, clear
   bit 0 on all six panes `+0x24C`, `+0x258`, `+0x250`, `+0x25C`, `+0x254`,
   `+0x260`. Otherwise, for each bit `n = 0..2` with pane pair
   `(lo, hi) = (+0x24C, +0x258)`, `(+0x250, +0x25C)`, `(+0x254, +0x260)`:
   `if (this->+0x284 & (1 << n)) { lo &= 0xFE; hi |= 1; }` else
   `{ lo |= 1; hi &= 0xFE; }`.
8. **Second gate** (`0x806097D8..0x80609834`).
   `g = gate(this->+0x27C, this->+0x280)`. If `g == 0`, set bit 0 on both
   `+0x278` and `+0x264`; else clear bit 0 on both.
9. Return.

The `+0x244` early-outs and the `B == 0x2A` arm all leave **every other pane
untouched**, so callers see the previous frame's flags. That ordering is
load-bearing.

**Signature.**

```rust
/// Refreshes the eleven BRK panes of a message/help screen from the two selectors.
///
/// # Original
/// `FUN_80609440`, `0x80609440`, 1,040 bytes,
/// `work/ghidra/rel-d_basesNP/export/80609440_FUN_80609440.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - screen object; panes `+0x244..+0x278`, selectors `+0x27C`/`+0x280`,
///   flags `+0x284`. Incoming `r4` is dead (the body overwrites it at
///   `0x80609458`), so the one-argument shape is correct.
/// # Returns
/// Nothing; the export's `void` is accurate.
/// # Confidence
/// medium - every pane write, message id and branch is literal; `0x800FAE40`
/// is unclaimed and is described by its two keys only.
pub unsafe fn fn_80609440(this: usize)
```

**Constants.** Selectors `3`, `5`, `7`, `0x14`, `0x15`, `0x16`, `0x17`, `0x25`,
`0x2A`; message ids `0x35`, `0x36`, `0x37`, `0x38`, `0x39`, `0x3B`, `0x3F`;
digit-buffer capacity `0x10`; visibility masks `0xFE` / `0x01`; flag bits
`0x1`/`0x2`/`0x4`; pane byte `+0xBB`. Callees `0x800FAE40`, `0x800CDC60`,
`0x800CDDB0`, `0x800C9A60`.

**Verdict: ready (medium).** All four calls have explicit argument lists; the
only unclaimed callee (`0x800FAE40`) is a pure two-key predicate whose return is
only compared with zero.

---

## 2. `0x80757320` - the `remo_kanaami` rider-seat scroll controller

**Evidence.** `work/ghidra/rel-d_basesNP/export/80757320_FUN_80757320.c`,
1,032 bytes, 258 words, synthetic `0x80757320`.

**What it is.** The per-frame seat-selector of the `remo_kanaami` ride actor
whose helpers `fn_80757060` / `fn_80757140` / `fn_80757220` are already
translated (`fn_80757060` / `fn_80757140` / `fn_80757220` in
`src/game/bases.rs`). It polls
`0x80756FD0` for the set of players touching the seat, switches the active
rider slot `+0x63D`, and drives the seat offset `+0x620` with a
velocity/spring state machine that plays sounds `0x2F4`/`0x2F5` and stops the
rider sound on a reversal.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x104` | `s16` | published seat y = `+0x614 - (s16)+0x620` |
| `+0x614` | `s16` | seat base coordinate |
| `+0x618` | `i32` | upper travel limit (max) |
| `+0x61C` | `i32` | lower travel limit (min) |
| `+0x620` | `i32` | current offset |
| `+0x624` | `s16` | velocity `v` |
| `+0x628` | `s16` | last target from `0x807571E0` |
| `+0x62A` | `s16` | `3 * (s16)+0x620` (published) |
| `+0x62C` | `u8` | settle direction (0/1); indexes the step table |
| `+0x62D` | `u8` | settle countdown, reloaded with 12 |
| `+0x62E` | `u8` | step index, wrapped modulo 4 |
| `+0x630` | `u16` | interaction sound timer (see `fn_80757060`) |
| `+0x63C` | `u8` | "settling" flag |
| `+0x63D` | `s8` | active rider slot, `-1` = none |
| `+0xAC` | vec3 | sound position source |

### Exact behaviour

Let `slot = (s8)this->+0x63D`, `V = (s16)this->+0x624`, `P = this->+0x620`.
`0x80756FD0(this, &out)` returns a 4-bit player mask and writes the chosen
player index (`s8`) to `out`. `0x807571E0(this)` returns the raw input target.

1. Prologue: save `r28..r31`, `r31 = mask` (return of `0x80756FD0`), `r30 = this`.
2. **No input** (`0x80757350..0x80757380`). If `mask == 0`: if `slot >= 0` and
   `0x80757060(this, slot) != 0` then `this->+0x63D = -1`, `+0x624 = 0`,
   `+0x628 = 0`.
3. **Input** (`0x80757384..0x80757400`):
   * Read `maskTable[slot]` **before** testing `slot != -1` (the raw code does
     `lbzx` at `0x80757394` and only then `cmpi r4,-1`), so a `slot` of `-1`
     reads one byte before `0x807CE9D0`. Keep that order.
   * If `(mask & maskTable[slot]) == 0 && slot != -1` and
     `0x80757060(this, slot) != 0` then `this->+0x63D = -1`.
   * If `(s8)this->+0x63D < 0` then `this->+0x63D = out`,
     `0x80757140(this)`, `this->+0x63C = 0`.
   * If `(s8)this->+0x63D == out` then `this->+0x630 = 1`.
4. Re-read `slot = (s8)this->+0x63D`.
5. **No rider** (`slot < 0`, `0x80757658..0x807576E8`):
   `this->+0x63C = 0`. If `P != 0`:
   * If `P < 0`: `V += 0x100`; if `V > 0x3FF` then `V = 0x400`.
   * Else: `V -= 0x100`; if `V < -0x3FF` then `V = -0x400`.
   * `P += V`; if `(u32)(P + 0x80) < 0x101` then `P = 0`.
   * `this->+0x104 = this->+0x614 - (s16)P`.
6. **Normal scroll** (`slot >= 0 && this->+0x63C == 0`,
   `0x8075741C..0x807575D8`):
   ```
   target = (s16)0x807571E0(this);            // clamped to +-0x1A2C there
   this->+0x628 = target;
   step   = (target >> 10) + ((target < 0 && (target & 0x3FF) != 0) ? 1 : 0);
   oldV   = V; V += step; this->+0x624 = V;
   if (target < 0) { if (V < -0x17F) V = -0x180; }
   else            { if (V >  0x17F) V =  0x180; }
   this->+0x624 = V;
   P += V; this->+0x620 = P;
   if ((oldV < 0 && V > 0) || (oldV > 0 && V < 0)) 0x80757220(this);
   quake = false;
   if (P >= this->+0x618) {
       P = this->+0x618; quake = (V >= 0x28);
       this->+0x62C = 0; this->+0x62D = 0xC; this->+0x62E = 0;
       if (quake) this->+0x63C = 1;
       V = 0; this->+0x624 = 0;
   }
   if (P <= this->+0x61C) {
       P = this->+0x61C; this->+0x62C = 1;
       this->+0x62D = 0xC; this->+0x62E = 0;
       if (V <= -0x28) { quake = true; this->+0x63C = 1; }
       V = 0; this->+0x624 = 0;
   }
   if (quake) {
       0x800439B0(this, mask);
       0x8006A4B0(&pos, this + 0xAC);
       0x80198C30(*(u32*)0x80429D60, 0x2F5, &pos, 0);
   }
   this->+0x104 = this->+0x614 - (s16)P;
   amp = (s16)0x802DE514(V);
   0x8006A4B0(&pos, this + 0xAC);
   0x801995A0(*(u32*)0x80429D60, 0x2F4, *(u32*)this, &pos, amp, 0);
   ```
   Note the two clamps use `>=`/`<=` (the `bc 12,0`/`bc 12,1` pair), and the
   velocity clamp is applied **after** the addition but **before** `P += V`.
7. **Settle** (`slot >= 0 && this->+0x63C != 0`, `0x807575DC..0x80757654`):
   ```
   if (this->+0x62D != 0) this->+0x62D -= 1;
   step = stepTable[this->+0x62E + this->+0x62C * 4];
   oldP = P; P += step; this->+0x620 = P;
   this->+0x62E = (this->+0x62E + 1) & 3;
   this->+0x104 = this->+0x614 - (s16)P;
   if (this->+0x62D == 0) { this->+0x63C = 0; 0x80757220(this); }
   ```
8. Common tail (`0x807576EC..0x80757704`):
   `this->+0x62A = (s16)this->+0x620 * 3; 0x80757280(this);` then epilogue.

The settle step table is `{80, 80, -80, -80, -80, -80, 80, 80}`: direction 0
takes the first four, direction 1 the last four, so the direction flips twice
per 4-step cycle.

**Signature.**

```rust
/// Advances the `remo_kanaami` rider-seat selector by one frame.
///
/// # Original
/// `FUN_80757320`, `0x80757320`, 1,032 bytes,
/// `work/ghidra/rel-d_basesNP/export/80757320_FUN_80757320.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - the `0x640`-byte ride actor; slots `+0x618..+0x63D`.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - the state machine, both clamps, the step table and the four sound
/// ids are literal; `0x80757280` and `0x802DE514` stay address-only calls.
pub unsafe fn fn_80757320(this: usize)
```

**Constants.** Mask table `0x807CE9D0 = {01, 02, 04, 08, 10, 20, 40, 80}` (only
the first four are used by `0x80756FD0`; this body indexes it with the slot).
Step table `0x807CE9E0` = `i32[8] = {80, 80, -80, -80, -80, -80, 80, 80}`.
Clamps `0x17F`/`0x180`, `0x3FF`/`0x400`, `0x27`, `0x28`, `0x80`, `0x100`,
`0x101`; target clamp `0x1A2C` (inside `0x807571E0`); settle reload `0xC`;
sound ids `0x2F4`, `0x2F5`, `0x2A6`, `0x2A7`. Globals `0x80429D60` (sound
manager). Callees `0x80756FD0`, `0x80757060`, `0x80757140`, `0x807571E0`,
`0x80757220`, `0x80757280`, `0x800439B0`, `0x8006A4B0`, `0x80198C30`,
`0x801995A0`, `0x802DE514`.

**Verdict: ready (medium).** Every field and clamp is literal; the only
callee whose own export is damaged is `0x80757280`, which is called with
`r3 = this` and no return, so this body's reconstruction does not depend on its
internals.

---

## 3. `0x80741510` - the first-active effect-record driver

**Evidence.** `work/ghidra/rel-d_basesNP/export/80741510_FUN_80741510.c`,
1,016 bytes, 254 words, synthetic `0x80741510`.

**What it is.** The missing member of the multi-player marker actor pipeline
translated in wave 64: `fn_807419c0` calls it between the record collect step
and `fn_80741910` with an `// unknown:` note
(`fn_807419c0` in `src/game/bases.rs`, at its `// unknown:` call site). It
drives **the first active** `0x20`-byte
effect record in `+0x514..+0x714` (the loop returns from inside the first active
record), reads the four per-player input-flag words through the global pointer
`0x80429F50`, and plays the point-token sound when the record crosses zero.

### Field map

Record (`+0x514 + i*0x20`, 16 records; same record as `fn_807412a0` /
`fn_80741380`):

| offset | type | meaning |
|---:|---|---|
| `+0x00` | `s16` | oscillator offset `x` |
| `+0x02` | `s16` | oscillator velocity `y` |
| `+0x04` | `s16` | divisor, written `0x200` |
| `+0x06` | `s16` | band, written 2/1/0 and then 4 by the bump arms |
| `+0x08` | `s16` | limit, written `(s16)(int)this->+0x3D4` |
| `+0x0A` | `s16` | range, written 8 or 0 |
| `+0x0C` | `s16` | wrap, written `0x20` or 0 |
| `+0x10` | `s16` | oscillator hit flag (written by `0x800DF890`) |
| `+0x14` | `f32` | timer (`fn_807412a0`) |
| `+0x18` | `f32` | coverage coordinate (`fn_80741380`) |
| `+0x1C` | `u8` | active |
| `+0x1D` | `u8` | player slot index |

Actor:

| offset | type | meaning |
|---:|---|---|
| `+0x394 + j*0x10` | `i32` | slot `j` record index, `-1` = unused |
| `+0x398 + j*0x10` | `u8` | slot `j` live |
| `+0x39C + j*0x10` | `f32` | slot `j` bump speed |
| `+0x3D4` | `f32` | smoothed limit (stepped by `0x8015F470`) |
| `+0x3D8` | `u8` | alternate-size selector |
| `+0x3E0` | `f32` | band source (2.5 / 1.5 / 0.0) |
| `+0x3E4` | `i32` | previous `x` (crossover detector) |
| `+0x3E8` | `i32` | `+0x3E4` snapshot before the store |
| `+0x4` | `u32` | bit 0 selects sound `0x12E` instead of `0x12A` |

### Exact behaviour

1. Prologue: `stwu r1,-0x40`; save `r28..r31`; `r28 = this`; `r29 = this+0x514`;
   `r3 = this+0x714` (loop bound). `r30 = *(u32*)0x80429F50` (player array
   base), advanced by 4 per slot.
2. `do { if (record->+0x1C == 0) { record += 0x20; continue; } ... ; return; }`
   - **only the first active record is processed**, then the function returns.
3. `target = fn_80740F20(this)` (double); `step = (this->+0x3D4 <= target) ? 1.0
   : 0.3`; `0x8015F470(target, step, &this->+0x3D4)`
   (`step_float_clamped(target, step, value)` in `src/game/task.rs`).
4. `record->+0x4 = 0x200`; `record->+0x8 = (s16)(int)this->+0x3D4`.
5. Band arm:
   ```
   if (this->+0x3D8 == 0) {
       this->+0x3E0 = 2.5; record->+0xA = 8; record->+0xC = 0x20;
   } else if ((u16)record->+0x0 <= 0x10) {
       this->+0x3E0 = 0.0; record->+0xA = 0; record->+0xC = 0;
   } else if ((u16)(record->+0x0 + 0x10) <= 0xF) {
       this->+0x3E0 = 0.0; record->+0xA = 0; record->+0xC = 0;
   } else {
       this->+0x3E0 = 1.5;
   }
   record->+0x6 = (s16)(int)this->+0x3E0;
   ```
   The two `0.0` arms are the unsigned windows `x in [0, 0x10]` and
   `x in [-0x10, -1]`; the `2.5`/`1.5` arms are the `1.0 * constant` forms the
   CodeWarrior ladder emits.
6. For `j = 0..3` (`0x80741690`, `r3 = this`, `r30 += 4`):
   ```
   slot = this + 0x394 + j*0x10;
   if (slot->+0x0 == -1 || slot->+0x4 == 0) continue;
   flags = *(u32*)(*(u32*)(*(u32*)0x80429F50 + 4*(j+1)) + 0x14);
   x = record->+0x0; y = record->+0x2; speed = slot->+0x8;
   if ((flags & 8) != 0) {
       if (x > 0) record->+0x6 = 4;
       else if (y >= 0) record->+0x0 += (s16)((speed / 135.0) * -(f32)y);
   } else if ((flags & 4) != 0) {
       if (x < 0) record->+0x6 = 4;
       else if (y <= 0) record->+0x0 += (s16)((speed / 135.0) * (f32)y);
   }
   ```
   The two `cror` instructions at `0x807416F4` and `0x8074177C` fold
   `GT|EQ` and `LT|EQ`; the conditions above are the folded forms.
7. Band zeroing (`0x807417C4..0x807417FC`): if `y < 0` and `x <= 0` then
   `record->+0x6 = 0`; else if `y >= 0` and `x >= 0` then `record->+0x6 = 0`.
8. `hit = fn_800DF890(record)` (the integer oscillator; it rewrites
   `+0x00/+0x02/+0x10` from `+0x04/+0x06/+0x08/+0x0A/+0x0C`).
9. `prev = this->+0x3E4; this->+0x3E8 = prev; this->+0x3E4 = (s16)record->+0x0`.
10. `crossed = (prev > 0 && (s16)record->+0x0 <= 0) ||
    (prev < 0 && (s16)record->+0x0 >= 0)`.
11. If `crossed`:
    ```
    vol = (s16)0x802DE514(-hit / (s16)record->+0x4);
    id  = (this->+0x4 & 1) == 0 ? 0x12A : 0x12E;
    0x8006A4B0(&pos, this + 0xAC);
    0x80198FC0(*(u32*)0x80429D60, id, &pos, vol, 0);
    ```
12. Return (there is no loop back after an active record).

**Signature.**

```rust
/// Advances the first active point-token effect record of the marker actor.
///
/// # Original
/// `FUN_80741510`, `0x80741510`, 1,016 bytes,
/// `work/ghidra/rel-d_basesNP/export/80741510_FUN_80741510.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - marker actor; records `+0x514..+0x714`, slots `+0x394..+0x3DC`,
///   scratch `+0x3D4..+0x3E8`.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - all six record fields, both input arms and both sound ids are
/// literal; `0x800DF890`, `0x80198FC0`, `0x802DE514` and `0x8015F470` are
/// address-named.
pub unsafe fn fn_80741510(this: usize)
```

**Constants.** `0x807CE08C = 0.0f`, `0x807CE090 = 1.0f`,
`0x807CE0C0 = f64 0x4330000080000000` (2^52+2^31),
`0x807CE0CC = 0.3f`, `0x807CE0D0 = 1.5f`, `0x807CE0D4 = 2.5f`,
`0x807CE0D8 = 135.0f`. Words `0x200`, `0x10`, `0x11`, `0xF`, `0x12A`, `0x12E`.
Globals `0x80429F50` (player array), `0x80429D60` (sound manager). Callees
`0x80740F20`, `0x8015F470`, `0x800DF890`, `0x802DE514`, `0x8006A4B0`,
`0x80198FC0`.

**Verdict: ready (medium).** The `+0x3D4` step, the four player-slot walks and
the oscillator are all explicit; `0x8015F470` is the already-translated
`step_float_clamped` thunk.

---

## 4. `0x8077AC30` - the state-command handler with the animation request

**Evidence.** `work/ghidra/rel-d_basesNP/export/8077ac30_FUN_8077ac30.c`,
1,012 bytes, 253 words, synthetic `0x8077AC30`.

**What it is.** A three-argument command handler on the same class as
`0x807886E0` (vtable at `+0x60`, slot `+0x68`; fields `+0x124`, `+0x139`,
`+0x1E4`, `+0x1E8`, `+0x1EC`, `+0x1FC`) and in the same point-result date-file
family as `0x80786EA0`. It services commands `0x3C`, `0x60`, `0x61`, `0x62`,
arms a 30-frame animation request on the manager at `0x8042A284`, and toggles
the current area record at `0x8042A18C + n*0xBF8 + 0x1A0`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x60` | vtable* | class vtable; slot `+0x68` = "advance state" |
| `+0x124` | `u8` | visible flag on `this` and on the `+0x1E8`/`+0x1E4` children |
| `+0x139` | `u8` | "command consumed / finished" flag |
| `+0x1E4` | object* | third child (visibility only) |
| `+0x1E8` | object* | primary child; `+0xAC` is handed to effects |
| `+0x1EC` | `u8` | child-ready gate |
| `+0x1F0..+0x1F8` | `f32` x3 | position vector filled by the `+0x29C` virtual |
| `+0x1FC` | `i32` | command countdown (0x1E = 30 frames) |

Manager `M = *(u32*)0x8042A284`: `M+0x5F0 = 0`, `M+0x5F4 = target` (a `f32*`),
`M+0x604 = 2` or `7`, `M+0x608 = 0`, `M+0x624 = 0`, `M+0x71C = 0x807CF5D8`.
`M+0x71C` is a 12-byte-record keyframe table (the body stores the pointer only).

### Exact behaviour

1. Prologue: `stwu r1,-0x40`; `addi r11,r1,0x40`; `0x802DCD74` (save helper);
   `r29 = this`, `r30 = command (r4)`, `r31 = M`.
2. If `command == -1`, return.
3. If `run_first (r5) == 0`, skip to step 5.
4. First phase, by `command`:
   * `0x60` (`0x8077AC88`): require `this->+0x1E8 != 0 && this->+0x1EC != 0`,
     else `vtable[0x68](this)` and go to step 5.
     ```
     obj  = *(u32*)0x8042A1A0;
     v    = *(u32*)(obj + 0x29C);
     fn   = *(u32*)(*(u32*)(v + 4) + 0x30);
     ((void (*)(f32*, u32))fn)(&vec, v);      // writes vec[0..2]
     this->+0x1F0 = vec[0]; this->+0x1F4 = vec[1]; this->+0x1F8 = vec[2];
     M->+0x604 = 2; M->+0x5F4 = &this->+0x1F0; M->+0x5F0 = 0;
     M->+0x624 = 0; M->+0x608 = 0; M->+0x71C = 0x807CF5D8;
     this->+0x1FC = 0x1E;
     ```
   * `0x61` (`0x8077AD28`): require `+0x1E8 != 0 && +0x1EC != 0`, else fall
     through to step 5 (where the same gate routes to `vtable[0x68]`).
     ```
     this->+0x124 = 0; this->+0x1E8->+0x124 = 1;
     0x80103390(*(u32*)0x8042A26C, 0x21, this->+0x1E8 + 0xAC, 0, 0);
     this->+0x1E4->+0x124 = 1; this->+0x1FC = 0;
     ```
   * `0x3C` (`0x8077AD80`): require `+0x1E8 != 0 && +0x1EC == 0` and
     `0x800FD920(this) != 0`; else fall through to step 5.
     ```
     this->+0x124 = 1; this->+0x1E8->+0x124 = 0; this->+0x1FC = 0;
     ```
   * `0x62` (`0x8077ADC0`): require `+0x1E8 != 0 && +0x1EC != 0`, else
     `vtable[0x68](this)` and go to step 5.
     ```
     M->+0x604 = 7; M->+0x5F4 = &stack[0x14];  // NOTE: a stack address
     M->+0x5F0 = 0; M->+0x624 = 0; M->+0x608 = 0;
     M->+0x71C = 0x807CF5D8; this->+0x1FC = 0x1E;
     ```
   * anything else: nothing.
5. Second phase (always reached), by `command`:
   * `0x60` (`0x8077AE48`): require `M->+0x5F4 == 0` (else return); then
     `if (this->+0x1FC > 0) --this->+0x1FC; else vtable[0x68](this);`.
   * `0x61` (`0x8077AE84`): require `+0x1E8 != 0 && +0x1EC != 0`; otherwise
     `vtable[0x68](this)` and return. Then the countdown test; when it reaches
     zero:
     ```
     area = *(u32*)0x8042A18C;
     n    = *(u32*)(area + 0x338C);
     0x800F1710(area + n*0xBF8 + 0x1A0, 1);
     vtable[0x68](this);
     ```
   * `0x3C` (`0x8077AF04`): require `+0x1E8 != 0 && 0x800FD920(this) != 0`;
     otherwise `vtable[0x68](this)` and return. Then the countdown test; when
     zero: the same area record call with `0` instead of `1`, then
     `vtable[0x68](this)`.
   * `0x62` (`0x8077AF88`): require `M->+0x5F4 == 0` (else return); require
     `M->+0x4EC == M->+0x4E0 && M->+0x4F0 == M->+0x4E4 &&
     M->+0x4F4 == M->+0x4E8` (three `fcmpu`; else return); then the countdown
     test; when zero `vtable[0x68](this)`.
   * anything else: `this->+0x139 = 1`.
6. Epilogue: `0x802DCDC0` (restore), LR, `blr` (`0x8077B020`).

The virtual call is `vtable = *(u32*)(this+0x60); fn = *(u32*)(vtable+0x68);
fn(this)` with no other arguments; the `+0x29C` call is a second, independent
object.

**Signature.**

```rust
/// Handles one state command for the point-result / date-file screen object.
///
/// # Original
/// `FUN_8077AC30`, `0x8077AC30`, 1,012 bytes,
/// `work/ghidra/rel-d_basesNP/export/8077ac30_FUN_8077ac30.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - command target; children `+0x1E4`/`+0x1E8`, countdown `+0x1FC`.
/// * `command` - `0x3C`, `0x60`, `0x61`, `0x62`; `-1` returns immediately.
/// * `run_first` - non-zero runs the "arm" phase before the "tick" phase.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - every branch, store and countdown is literal; the `+0x29C`
/// virtual method is assumed to fill the three-float out-vector, which is how
/// the body consumes `r3`.
pub unsafe fn fn_8077ac30(this: usize, command: i32, run_first: i32)
```

**Constants.** Commands `0x3C`, `0x60`, `0x61`, `0x62`, `-1`; countdown `0x1E`;
manager mode `2` / `7`; effect id `0x21`; area stride `0xBF8`, record offset
`0x1A0`, count offset `0x338C`; keyframe table `0x807CF5D8`. Globals
`0x8042A284`, `0x8042A1A0`, `0x8042A26C`, `0x8042A18C`. Callees `0x800FD920`,
`0x800F1710`, `0x80103390`. Save helpers `0x802DCD74` / `0x802DCDC0`.

**Verdict: ready (medium).** The `0x62` arm stores a **stack** pointer into
`M+0x5F4` (`addi r0,r1,0x14; stw r0,0x5F4(r31)`); that is what the shipped
bytes do and must be reproduced verbatim, not "fixed" to a persistent buffer.

---

## 5. `0x80786EA0` - the `dPointResultDateFile_c` model update

**Evidence.** `work/ghidra/rel-d_basesNP/export/80786ea0_FUN_80786ea0.c`,
1,008 bytes, 252 words, synthetic `0x80786EA0`.

**What it is.** The per-frame transform update of the point-result date-file
screen. The class's state-name strings live at `0x807DF960` in segment 4
(`dPointResultDateFile_c::StateID_StarCoinAnimeEndWait`,
`...StateID_RankChangeStartWait`, `...StateID_RankExitAnimeEndWait`,
`...StateID_RankChange`, `...StateID_BigSmallAnimeEndWait`), and the per-index
tables this body reads (target angles `-35/-5/130`, spin speeds `220/260/280`,
two transition thresholds) are the star-coin/rank animations those names
describe.

The export carries `unaff_GQR0` and an `ldexpf` call. **That is a decompiler
artifact, not paired-single code.** The only two `psq_` words in the body are
the `f31` prologue spill `psq_st f31,0x48(r1),0,0` at `0x80786EB4` and its
`psq_l f31,0x48(r1),0,0` restore at `0x8078726C`, both `qr0, W=0`. There is no
GQR read, no quantiser index other than 0 and no `ps_*` arithmetic. The
`ldexpf` is Ghidra's rendering of the pair spill. This is the same correction
waves 142/143 applied to the `qr0` "GQR" families; the address must not be
skipped on that ground.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x4` | `u32` | low byte = model/character index (0..2) |
| `+0xDC..+0xE4` | `f32` x3 | model position x/y/z |
| `+0x1AC` | block | transform target for `0x8016A150` |
| `+0x258` | `f32` | spin/progress angle |
| `+0x2B4` | handle | argument of `0x80102E00` (returns the angular speed) |
| `+0x2B8` | object* | transform object; `+0x0C` offset, `+0x14` finished byte, `+0x34`/`+0x38` angle pair |
| `+0x50C` | `u8` | direction select (0/1) |
| `+0x510` | `u32` | frame counter |
| `+0x514` | `u8` | "transition armed" |
| `+0x515` | `u8` | "spin enabled" |

### Exact behaviour

Let `idx = this->+0x4 & 0xFF`, `T = 0x807CF960`, `obj = this->+0x2B8`.

1. Prologue: `stwu r1,-0x50`; save `f31` with `stfd`+`psq_st`, `r28..r31`;
   `r31 = T`, `r30 = this`.
2. **Spin** (`0x80786ED4..0x80786F2C`). If `this->+0x515 != 0`:
   ```
   target = *(f32*)(T + 0x24 + idx*4);          // -35, -5, 130
   if (this->+0x258 >= target) this->+0x258 = target;
   else this->+0x258 += (f32)(*(f32*)(T + 0x6C + idx*4) / 0x80102E00(this->+0x2B4));
   ```
   (`0x807CF9CC + idx*4` = 220, 260, 280.)
3. `0x801037C0(obj)` (no return used).
4. Build `vec = { this->+0xDC, this->+0xE0 + *(f32*)(obj+0x0C), this->+0xE4 }`
   and call `0x8016A150(this+0x1AC, &vec)`.
5. If `*(u8*)(obj+0x14) != 0` then `this->+0x510 += 1`.
6. **Transition check** (`0x80786F88..0x80787084`). If `this->+0x514 != 0`:
   * `this->+0x50C != 0` (arm A, threshold `90.0`):
     ```
     f = *(f32*)(obj+0x38);
     if (f - *(f32*)(obj+0x34) < 90.0 && 90.0 <= f) {
         0x80103A10(obj, *(f32*)(T + 0x0C + idx*8));   // 0.3
         this->+0x514 = 0;
         0x80103A20(obj, *(f32*)(T + 0x3C + idx*4));   // 2.0, 2.5, 3.25
     }
     ```
   * `this->+0x50C == 0` (arm B, threshold `270.0`):
     ```
     f = *(f32*)(obj+0x38);
     if (f - *(f32*)(obj+0x34) < 270.0 && 270.0 <= f) {
         0x80103A10(obj, *(f32*)(T + 0x0C + idx*8));
         this->+0x514 = 0;
         0x80103A20(obj, *(f32*)(T + 0x30 + idx*4));   // 0.8, 1.0, 2.25
     }
     ```
7. **Next-state arm** (`0x80787088..0x80787268`). If `this->+0x514 == 0`:
   * `this->+0x50C != 0`:
     ```
     if ((i32)this->+0x510 == (i32)*(u32*)(T + 0x60 + idx*4)) {   // 2, 1, 2
         f = *(f32*)(obj+0x38);
         if (f - *(f32*)(obj+0x34) < 270.0 && 270.0 <= f) {
             d = (*(f32*)(T + 0x3C + idx*4) - *(f32*)(T + 0x30 + idx*4)) * 0.5;
             0x80103A10(obj, (f32)(d + *(f32*)(T + 0x0C + idx*8)));
             this->+0x510 = 0; this->+0x50C = 1; this->+0x514 = 1;
             0x80103A20(obj, (f32)(d + *(f32*)(T + 0x30 + idx*4)));
         }
     }
     ```
   * `this->+0x50C == 0`:
     ```
     if ((i32)this->+0x510 == (i32)*(u32*)(T + 0x54 + idx*4)) {   // 2, 1, 2
         f = *(f32*)(obj+0x38);
         if (f - *(f32*)(obj+0x34) < 90.0 && 90.0 <= f) {
             d = (*(f32*)(T + 0x3C + idx*4) - *(f32*)(T + 0x30 + idx*4)) * 0.5;
             0x80103A10(obj, (f32)(d + *(f32*)(T + 0x0C + idx*8)));
             this->+0x510 = 0; this->+0x50C = 0; this->+0x514 = 1;
             0x80103A20(obj, (f32)(d + *(f32*)(T + 0x30 + idx*4)));
         }
     }
     ```
8. Epilogue: restore `f31` (`psq_l`/`lfd`), `r28..r31`, `blr`.

The `(i32)this->+0x510 == (i32)*(u32*)(...)` comparison converts both words with
the CodeWarrior signed-int-to-double ladder (`0x43300000` + `xoris 0x8000`,
subtract `2^52+2^31`) and compares the two doubles; write it as an integer
comparison in Rust.

### Per-index tables (base `0x807CF960`)

| table | offsets | idx 0 | idx 1 | idx 2 |
|---|---|---:|---:|---:|
| target angle | `+0x24 + idx*4` | -35.0 | -5.0 | 130.0 |
| speed | `+0x6C + idx*4` | 220.0 | 260.0 | 280.0 |
| base angle | `+0x0C + idx*8` | 0.3 | 0.3 | 0.3 |
| angle A | `+0x30 + idx*4` | 0.8 | 1.0 | 2.25 |
| angle B | `+0x3C + idx*4` | 2.0 | 2.5 | 3.25 |
| frame A | `+0x54 + idx*4` (u32) | 2 | 1 | 2 |
| frame B | `+0x60 + idx*4` (u32) | 2 | 1 | 2 |

Fixed: `+0x84 = 90.0`, `+0x88 = 270.0`, `+0x8C = 0.5`,
`+0x90 = f64 0x4330000080000000`. (`+0x98 = 2160.0` and `+0x9C = -30.0` are in
the same pool but are not read by this body.)

**Signature.**

```rust
/// Advances the point-result date-file model's spin and state transition.
///
/// # Original
/// `FUN_80786EA0`, `0x80786EA0`, 1,008 bytes,
/// `work/ghidra/rel-d_basesNP/export/80786ea0_FUN_80786ea0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - the date-file screen object; tables at `0x807CF960`.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - the two spin arms, the two transition arms and all seven per-index
/// tables are literal; `0x801037C0`, `0x80103A10`, `0x80103A20` stay
/// address-only.
pub unsafe fn fn_80786ea0(this: usize)
```

**Constants.** See the table above plus `90.0`, `270.0`, `0.5`, `2^52+2^31`.
Callees `0x80102E00` (returns `f64`), `0x801037C0`, `0x8016A150`,
`0x80103A10`, `0x80103A20`.

**Verdict: ready (medium).** The `unaff_GQR0`/`ldexpf` warning is void: the
body's only paired-single words are the two `qr0` `f31` spill accesses.

---

## 6. `0x807886E0` - the `dPointResultDateFile_c` command handler

**Evidence.** `work/ghidra/rel-d_basesNP/export/807886e0_FUN_807886e0.c`,
1,004 bytes, 251 words, synthetic `0x807886E0`.

**What it is.** The command side of the same date-file screen as row 5: vtable
`+0x60`, methods `+0x68` (advance), `+0x88`/`+0x8C`/`+0x98`/`+0x9C` (the four
animation phases). It dispatches commands `0x2A`, `0x2D`, `0x3A`, `0x3B`,
`0x76`, `0x88`, submits `wm_effect_record_submit` records (`0x4E`, `0x4F`,
`0x50`), spawns the door object (`0x80785600`), and uses `+0x424` as a 10-frame
countdown.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x4` | `u32` | low byte = screen state (2, 3, 4) |
| `+0x60` | vtable* | class vtable |
| `+0xAC` | vec3 | position argument of the effect calls |
| `+0x102` | `s16` | angle written by `0x8015F200` |
| `+0x108` | `s16` | mirror of `+0x102` |
| `+0x124` | `u8` | visible flag |
| `+0x139` | `u8` | "finished" flag |
| `+0x334` | `i32` | state word (`== 1` enables the `0x88` branch) |
| `+0x420` | object* | record whose `+0x20/+0x22/+0x24` are three `s16` |
| `+0x424` | `i32` | countdown, set to `10` by `0x3A` |

### Exact behaviour

Let `state = this->+0x4 & 0xFF`, `vt = *(u32*)(this+0x60)`.

1. Prologue: `stwu r1,-0x70`; save `r29..r31`; `r31 = this`, `r30 = T2`
   (`0x807CFA08`). If `command == -1`, return.
2. If `run_first (r5) != 0`, run the first phase:
   * `0x2A` (`0x80788744`):
     ```
     this->+0x124 = 1;
     0x80788AE0(&vec, this);
     f = ((f32 (*)(u32))vt[0x88])(this);          // f32 offset
     vec[1] += f;                                  // r1+0x50..0x58 are x/y/z
     0x80105240(*(u32*)0x8042A27C, 0x4E, this+0xAC, 1);
     0x80788B50(this, &vec2, 0x2D, 0, 13.0, 0.1, 1.0);
     ```
     `vec2` is the same x/y/z copied to `r1+0x38..0x40`; the three floats come
     from `T2+0x4C` (13.0), `T2+0x50` (0.1), `T2+0x48` (1.0).
   * `0x2D` (`0x807887E4`): `vt[0x98](this)` only.
   * `0x3A` (`0x807887F8`): if `state == 3 || state == 4`, `vt[0x68](this)` and
     continue to step 3. Otherwise:
     ```
     0x80788AE0(&vec, this);
     vec[1] -= 100.0;                              // T2+0x4
     0x80788B50(this, &vec, 0x0F, 1, 5.0, 1.0, 0.1);
     this->+0x424 = 10;
     ```
     (`T2+0x54 = 5.0`, `T2+0x48 = 1.0`, `T2+0x50 = 0.1`.)
   * `0x3B` (`0x8078888C`): if `state != 2`, do nothing. Otherwise:
     ```
     0x80788AE0(&vec, this);
     f = ((f32 (*)(u32))vt[0x88])(this);
     vec[1] += f;
     r = ((u32 (*)(u32))vt[0x8C])(this);
     0x80788B50(this, &vec3, r, 0, 10.0, 0.1, 1.0);   // T2+0x58, 0x50, 0x48
     0x80785600(1);
     0x80105240(*(u32*)0x8042A27C, 0x50, this+0xAC, 1);
     ```
   * `0x76` (`0x8078894C`): if `state != 3 && state != 4`,
     `0x80105240(*(u32*)0x8042A27C, 0x4F, this+0xAC, 1)`.
3. Second phase (always), by `command`:
   * `0x2A` (`0x807889A8`):
     ```
     if (0x80789A60(this) == 0) return;
     if (0x80788CB0(this) == 0) return;
     p = this->+0x420;
     0x8015F200(this+0x102, 0, (s16)p[0x20], (s16)p[0x22], (s16)p[0x24]);
     this->+0x108 = this->+0x102;                 // two s16 loads/stores
     if (this->+0x102 != 0) return;
     this->+0x139 = 1;
     ```
   * `0x2D` (`0x80788A04`): `vt[0x9C](this)`.
   * `0x3A` (`0x80788A1C`):
     ```
     if (0x80788CB0(this) != 0) this->+0x139 = 1;
     if (this->+0x424 == 0 && state != 3) 0x80785600(1);
     this->+0x424 -= 1;
     ```
   * `0x3B` (`0x80788A68`): `if (0x80788CB0(this) != 0) this->+0x139 = 1;`
   * `0x88` (`0x80788A84`): `if (this->+0x334 == 1) vt[0x68](this);`
   * anything else: `this->+0x139 = 1`.
4. Epilogue: restore `r29..r31`, LR, `blr` (`0x80788AC8`).

The `0x3B` first-phase copy at `0x807888E0` adds the `vt[0x88]` return to
`vec[1]` (the same layout as `0x2A`), then `vt[0x8C]` supplies the fourth
argument of `0x80788B50`.

**Signature.**

```rust
/// Handles one command of the point-result date-file screen.
///
/// # Original
/// `FUN_807886E0`, `0x807886E0`, 1,004 bytes,
/// `work/ghidra/rel-d_basesNP/export/807886e0_FUN_807886e0.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - screen object; vtable `+0x60`, countdown `+0x424`.
/// * `command` - `0x2A`, `0x2D`, `0x3A`, `0x3B`, `0x76`, `0x88`; `-1` returns.
/// * `run_first` - non-zero runs the submit phase before the state phase.
/// # Returns
/// Nothing.
/// # Confidence
/// medium - all six command arms, the four vtable slots and the five effect ids
/// are literal; `0x80788AE0`/`0x80788B50`/`0x80788CB0` stay address-only.
pub unsafe fn fn_807886e0(this: usize, command: i32, run_first: i32)
```

**Constants.** Pool base `0x807CFA08`: `+0x4 = 100.0`, `+0x48 = 1.0`,
`+0x4C = 13.0`, `+0x50 = 0.1`, `+0x54 = 5.0`, `+0x58 = 10.0`. Commands `0x2A`,
`0x2D`, `0x3A`, `0x3B`, `0x76`, `0x88`, `-1`; states `2`, `3`, `4`; effect
ids `0x4E`, `0x4F`, `0x50`; countdown `10`; sub-command ids `0x2D`, `0x0F`, `0`;
vtable slots `0x68`, `0x88`, `0x8C`, `0x98`, `0x9C`. Globals `0x8042A27C`.
Callees `0x80105240`, `0x8015F200`, `0x80785600`, `0x80788AE0`, `0x80788B50`,
`0x80788CB0`, `0x80789A60`.

**Verdict: ready (medium).** The `vt[0x88]` return is consumed as an `f32` and
`vt[0x8C]` as a `u32`; both are fixed by their use, and the body never inspects
the vtable itself.

---

## 7. `0x8065C370` - the animation-table copy into the `0x9CE8` manager

**Evidence.** `work/ghidra/rel-d_basesNP/export/8065c370_FUN_8065c370.c`,
976 bytes, 244 words, synthetic `0x8065C370`.

**What it is.** A member of the `0x9CE8`-byte manager whose constructor
`fn_8065be80` is already translated in `src/game/bases.rs`: the vtable is
`0x807E7410` at `+0x60`, the inline arrays are at `+0x99E4`, `+0x9AF8`,
`+0x9C20`, plus `+0x9C54` (four-byte elements, eight of them); the `+0x9C20`
block is built by `0x800E7BA0`. Wave 133 already reconstructs
`fn_8065c100`/`fn_8065c260` against the same object and names `+0x9C54` and
`+0x9CC8` (`docs/SPEC_WAVE133_BASES.md:755`, `:789`). This body copies five
words and five floats out of the slot-0 animation resource into those arrays
and zeroes `+0x9CC8`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x9C54 + i*4` | `u32` | five copied animation words (`i = 0..4`) |
| `+0x9C74 + i*4` | `f32` | five copied animation floats (`i = 0..4`) |
| `+0x9CC8` | `f32` | clamp input, set to `0.0` |

Animation resource `R = 0x80164B70(0)` (slot 0 of the resource context):
`R+0x0C` = track array (12-byte elements, first word copied), and
`R+0x14` → `+0x8` = node pointer array; node `a` = `array[a]`, then
`node+0x44` = `f32` array indexed by `b`.

### Exact behaviour

1. Prologue: `stwu r1,-0x10`; save LR, `r31`; `r31 = this`;
   `R = 0x80164B70(0)`.
2. **First table** (`0x8065C38C..0x8065C4F4`). Count `n1 = *(i32*)0x807E6EF8`
   (5) and index array at `0x807E6EFC + i*8` = `{2, 3, 4, 5, 6}`.
   ```
   for (i = 0; i < n1; i++)
       this->+0x9C54 + i*4 = *(u32*)(*(u32*)(R+0x0C) + index1[i]*12);
   ```
   The 8-way unrolled block is the same operation, eight at a time; the
   shipped count is 5, so only the tail loop runs. Reproduce the loop as
   written, not the unrolled bounds arithmetic.
3. **Second table** (`0x8065C4F8..0x8065C71C`). Count `n2 = *(i32*)0x807E6F3C`
   (5) and 12-byte records at `0x807E6F40 + i*12`:
   `{u16 node, u16 pad, u16 frame, u16 pad, f32 unused}` =
   `{(3,15), (7,20), (9,25), (15,30), (19,35)}`.
   ```
   for (i = 0; i < n2; i++) {
       a = rec[i].node; b = rec[i].frame;
       node = *(u32*)(*(u32*)(*(u32*)(R+0x14) + 8) + a*4);
       arr  = *(u32*)(node + 0x44);
       this->+0x9C74 + i*4 = *(f32*)(arr + b*4);
   }
   ```
4. `this->+0x9CC8 = *(f32*)0x807C8AE8` = `0.0f`.
5. Epilogue: restore `r31`, LR, `blr` (`0x8065C73C`).

The 12-byte record layout is forced by the `lhz` at `+0`, the `lwz`/`rlwinm`
pair at `+4` (`rlwinm r0,r0,2,14,29` is `(u16)+4 * 4`) and the `mulli 0xC`
stride. The `+8` float is present in the image but not read here.

### Tables read

```
0x807E6EF8  i32 n1 = 5
0x807E6EFC  u32[5] stride 8 = {2, 3, 4, 5, 6}
0x807E6F3C  i32 n2 = 5
0x807E6F40  record[5] stride 12 = {node, frame}:
            (3,15) (7,20) (9,25) (15,30) (19,35)
0x807C8AE8  f32 = 0.0
```

**Signature.**

```rust
/// Copies the slot-0 animation values into the manager's `+0x9C54` / `+0x9C74`
/// arrays and resets the `+0x9CC8` clamp input.
///
/// # Original
/// `FUN_8065C370`, `0x8065C370`, 976 bytes,
/// `work/ghidra/rel-d_basesNP/export/8065c370_FUN_8065c370.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - the `0x9CE8` manager built by `fn_8065be80`.
/// # Returns
/// Nothing.
/// # Confidence
/// high - two counted copies over literal tables; the only callee is
/// `0x80164B70(0)`, already translated as `resource::registry::fn_80164b70`.
pub unsafe fn fn_8065c370(this: usize)
```

**Constants.** Counts `5`/`5`; track stride `12`; node-array chain
`R+0x14 → +0x8 → [a] → +0x44 → [b]`; destinations `+0x9C54`, `+0x9C74`,
`+0x9CC8`; `0.0f`. Tables `0x807E6EF8`, `0x807E6EFC`, `0x807E6F3C`,
`0x807E6F40`, `0x807C8AE8`. Callee `0x80164B70`.

**Verdict: ready (high).** Both loops are pure table walks; the tables are in
the REL and are read out above, and the destination offsets are the ones the
translated constructor already allocates.

---

## 8. `0x80628A00` - the 65-entry course/date-file table builder

**Evidence.** `work/ghidra/rel-d_basesNP/export/80628a00_FUN_80628a00.c`,
972 bytes, 243 words, synthetic `0x80628A00`.

**What it is.** A save/course table initialiser. It clears the state word of 70
`0x20`-byte records, resolves the current save image with
`0x800E0380(*(u32*)0x8042A040, -1)` (`save_image_record_a`, already translated
in `src/engine/display.rs`), walks the 65 static entries at `0x807C6FF0`,
and classifies each as collected (`state = 3`), available (`state = 2`) or
locked (`state = 1`). The per-entry gate uses `0x800CE3C0(record, area, stage,
0x30)` (the `fn_800ce3c0` stage-flag mask test in `src/game/stage.rs`), the message-kind
gate uses `0x800B4E80(kind, -1)`
(`fn_800b4e80` in `src/game/stage.rs`), and the tail computes the page count
`ceil(count/5)` and debits the total weight from a counter.

The export's `void FUN_80628A00(void)` is wrong: Ghidra lost the argument
through `0x802DCD58` (`__savegprlr_20`, which saves `r20..r31` and leaves `r3`
intact). `iVar1 = func_0x802dcd58()` in the C is `this`.

### Field map

Record `+0x1B8 + i*0x20` (state words of 70 records are cleared; 65 entries are
written):

| offset | type | meaning |
|---:|---|---|
| `+0x00` | `i32` | entry key A = area/course id |
| `+0x04` | `i32` | entry key B = stage id |
| `+0x08` | `i32` | table column 2 |
| `+0x0C` | `i32` | table column 3 |
| `+0x10` | `i32` | table column 4 |
| `+0x14` | `i32` | table column 5 |
| `+0x18` | `i32` | weight (summed for collected entries) |
| `+0x1C` | `i32` | state: 1 = locked, 2 = available, 3 = collected |

Actor:

| offset | type | meaning |
|---:|---|---|
| `+0xAB0` | `i32` | page count = `ceil(processed/5)` |
| `+0xABC` | `u8` | initialised-once flag |
| `+0xA83`, `+0xA84` | `u8` | the two global unlock bytes for gate kinds 5/6 |
| `+0xAAC` | `i32` | progress value for gate kind 1 |
| `+0xB0` | object* | `+0x280 = 1`, `+0x288 -= total weight` |

### Exact behaviour

1. Prologue: `stwu r1,-0x40`; save `r20..r31` via `0x802DCD58`
   (`r23 = this`); epilogue `0x802DCDA4` (`__restgprlr_20`).
2. **State-word zero pass** (`0x80628A18..0x80628AB4`). Two blocks of 32
   words, one word per record:
   `for (i = 0; i < 2; i++) for (j = 0; j < 32; j++)
   *(u32*)(this + 0x1D4 + i*0x400 + j*0x20) = 0;`
   Then six more state words at `this + 0x9D4 + j*0x20` for `j = 0..5`
   (`0x1D4, 0x1F4, 0x214, 0x234, 0x254, 0x274` after the `+0x800` advance).
   In record terms this clears `+0x1C` of records 0..69.
3. `record = 0x800E0380(*(u32*)0x8042A040, -1)`.
4. **Entry loop** (`0x80628B14..0x80628D3C`). `entry = 0x807C6FF0`,
   `dst = this + 0x1B8`, `index = 0`:
   ```
   while (*(i32*)entry != 10) {
       dst->+0x00 = entry[0]; dst->+0x04 = entry[1];
       dst->+0x08 = entry[2]; dst->+0x0C = entry[3];
       dst->+0x10 = entry[4]; dst->+0x14 = entry[5];
       dst->+0x18 = entry[6]; dst->+0x1C = 1;
       if (0x800CE160(record, index) != 0) {
           dst->+0x1C = 3; total += entry[6];
       } else if (0x800CE3C0(record, entry[0], entry[1], 0x30) != 0) {
           switch (entry[7]) {
             case 1: if (entry[8] <= this->+0xAAC) dst->+0x1C = 2; break;
             case 2: if (*(u8*)(record+2) & 0x02) dst->+0x1C = 2; break;
             case 3: if (*(u8*)(record+2) & 0x04) dst->+0x1C = 2; break;
             case 4: if (*(u8*)(record+2) & 0x08) dst->+0x1C = 2; break;
             case 5: if (this->+0xA84 != 0) dst->+0x1C = 2; break;
             case 6: if (this->+0xA83 != 0) dst->+0x1C = 2; break;
             case 7: if (*(u8*)(record+2) & 0x10) dst->+0x1C = 2; break;
             case 8: if (*(u8*)(record+2) & 0x20) dst->+0x1C = 2; break;
             case 9..15: if (0x800B4E80(kind-9, -1) != 0) dst->+0x1C = 2; break;
             default (0): dst->+0x1C = 2; break;
           }
       }
       index++; dst += 0x20; entry += 0x24;
       if (index > 0x45) break;
   }
   ```
   The shipped table has 65 entries before the `10` sentinel (entry 65 starts
   with `10`), so `index` ends at 65 and `dst` at `this + 0x1B8 + 65*0x20`.
5. `if (this->+0x1D4 <= 2) this->+0x1D4 = 2;` (the first record's state).
6. `this->+0xAB0 = ceil(index / 5)` computed with the
   `0x66666667` magic; `if (index != pages*5) pages += 1`.
7. If `this->+0xABC == 0`:
   `obj = this->+0xB0; obj->+0x280 = 1; obj->+0x288 -= total;
   this->+0xABC = 1;`
8. Return.

The gate `case 0` sets state 2 unconditionally; `case 9..15` call
`0x800B4E80(0..6, -1)` (the `kind` argument is `entry[7] - 9`). The `entry[8]`
column is only read by `case 1`.

### The 65 entries (`0x807C6FF0`, 9 `i32` each)

Columns: `keyA, keyB, c2, c3, c4, c5, weight, gate, threshold`.

```
 0: 0 0 0 0 14400 0 3 0 0
 1: 0 0 1 0 4178 0 5 6 0
 2: 0 1 1 7 1087 1 3 1 5
 3: 0 2 3 32 14400 0 3 3 0
 4: 0 2 0 0 14400 0 5 9 0
 5: 0 21 0 0 2837 0 5 1 10
 6: 0 4 0 0 14400 0 5 1 30
 7: 1 0 2 32 14400 0 0 1 15
 8: 1 0 0 0 14400 0 3 3 0
 9: 1 1 2 32 14400 0 0 1 95
10: 1 1 0 0 1731 2 3 1 150
11: 1 2 1 0 14400 0 5 5 0
12: 1 3 3 14 14400 0 5 1 20
13: 1 4 1 3 14400 0 5 10 0
14: 1 4 0 0 14400 0 5 1 215
15: 1 5 3 1 14400 0 10 1 25
16: 2 0 2 32 910 1 0 1 65
17: 2 0 0 0 14400 0 3 1 35
18: 2 1 0 0 14400 0 5 1 165
19: 2 1 1 0 9910 0 5 1 190
20: 2 2 2 2 898 2 0 1 40
21: 2 2 1 5 1241 2 3 11 0
22: 2 2 0 0 14400 0 5 1 195
23: 2 20 3 0 14400 1 5 1 140
24: 2 4 0 0 14400 0 5 1 130
25: 3 0 1 0 3290 0 3 1 45
26: 3 1 1 0 2550 0 5 1 175
27: 3 1 0 0 14400 0 3 8 0
28: 3 2 2 0 14400 0 0 1 125
29: 3 2 0 0 14400 0 5 12 0
30: 3 21 3 4 14400 0 10 1 70
31: 3 3 2 5 2340 1 0 1 50
32: 3 20 3 0 14400 0 5 1 60
33: 3 23 1 0 3170 0 3 1 145
34: 4 0 1 7 4096 0 5 1 105
35: 4 2 1 0 1760 2 3 1 55
36: 4 20 2 0 1299 0 0 1 75
37: 4 20 3 0 14400 0 5 2 0
38: 4 23 0 0 3231 0 3 13 0
39: 5 0 1 6 2626 0 3 1 80
40: 5 1 2 32 4006 3 0 1 135
41: 5 2 2 2 1096 1 0 1 85
42: 5 2 0 0 14400 0 5 1 205
43: 5 4 3 32 14400 0 5 1 90
44: 5 5 3 3 14400 1 10 1 100
45: 5 23 0 2 2157 0 5 14 0
46: 6 0 0 0 14400 0 5 15 0
47: 6 2 2 6 1878 0 0 1 170
48: 6 21 3 0 14400 0 0 1 160
49: 6 20 3 0 14400 0 3 1 120
50: 6 3 1 32 5151 0 3 7 0
51: 6 23 2 32 2282 0 0 1 115
52: 7 1 3 32 14400 0 3 2 0
53: 7 2 0 0 14400 0 5 1 180
54: 7 21 2 32 1547 0 0 1 110
55: 7 37 1 0 4280 0 5 1 155
56: 7 23 0 0 25200 0 5 5 0
57: 8 0 0 0 14400 0 5 1 225
58: 8 1 0 0 14400 0 5 1 220
59: 8 2 1 0 2170 0 5 1 185
60: 8 2 0 0 14400 0 5 1 210
61: 8 3 2 1 14400 0 0 6 0
62: 8 4 0 0 14400 0 5 1 230
63: 8 5 2 1 1145 0 0 1 200
64: 8 6 0 0 14400 0 3 8 0
-- sentinel at 65: 10 42 4 -1 -1 4 -1 16 -1   (first word 10 stops the loop)
```

Gate `entry[7]` values used: 0, 1..15 (all of them). `entry[8]` (threshold) is
nonzero only for gate 1 in this table.

**Signature.**

```rust
/// Builds the 65-entry course/date-file table and its page/weight counters.
///
/// # Original
/// `FUN_80628A00`, `0x80628A00`, 972 bytes,
/// `work/ghidra/rel-d_basesNP/export/80628a00_FUN_80628a00.c`.
/// Synthetic base `0x80600000`; not a runtime address.
/// # Arguments
/// * `this` - the table owner; records `+0x1B8 + i*0x20`, counters `+0xAB0`,
///   `+0xABC`, save-image base `+0xB0`.
/// # Returns
/// Nothing (`0x802DCDA4` restores `r20..r31`).
/// # Confidence
/// high - the table, the 16-way gate and the weight/page arithmetic are all
/// literal; the export's `void(void)` signature is a lost-argument artifact of
/// the `0x802DCD58` save stub.
pub unsafe fn fn_80628a00(this: usize)
```

**Constants.** Table `0x807C6FF0` (65 x `0x24`), sentinel first word `10`;
record stride `0x20`; count bound `0x45`; gate mask `0x30`; gate kinds `0..15`;
save-record byte `+2` masks `0x02`, `0x04`, `0x08`, `0x10`, `0x20`; kind sweep
`0x800B4E80(0..6, -1)`; page divisor `5`; `0x66666667` magic; `+0xAB0`,
`+0xABC`, `+0xA83`, `+0xA84`, `+0xAAC`, `+0xB0`; `+0x280 = 1`, `+0x288 -= total`.
Globals `0x8042A040`. Callees `0x800E0380`, `0x800CE160`, `0x800CE3C0`,
`0x800B4E80`. Save helpers `0x802DCD58` / `0x802DCDA4`.

**Verdict: ready (high).** Every branch, table row and constant is read out of
the shipped image; the only remaining roles are those of the two stage-flag
queries, which are named by address and by their translated signatures.

---

## Summary

| # | address | bytes | export | verdict |
|--:|---:|---:|---|:--:|
| 1 | `0x80609440` | 1,040 | `80609440_FUN_80609440.c` | ready (medium) |
| 2 | `0x80757320` | 1,032 | `80757320_FUN_80757320.c` | ready (medium) |
| 3 | `0x80741510` | 1,016 | `80741510_FUN_80741510.c` | ready (medium) |
| 4 | `0x8077AC30` | 1,012 | `8077ac30_FUN_8077ac30.c` | ready (medium) |
| 5 | `0x80786EA0` | 1,008 | `80786ea0_FUN_80786ea0.c` | ready (medium) |
| 6 | `0x807886E0` | 1,004 | `807886e0_FUN_807886e0.c` | ready (medium) |
| 7 | `0x8065C370` | 976 | `8065c370_FUN_8065c370.c` | ready (high) |
| 8 | `0x80628A00` | 972 | `80628a00_FUN_80628a00.c` | ready (high) |

**8,060 bytes, 2,015 instructions. Ready 8, conditional 0.**

Three clusters account for five of the eight rows, which is why the split is
clean:

* the multi-player marker actor (`0x80741510`, wave-64 pipeline);
* the point-result date-file cluster (`0x8077AC30`, `0x80786EA0`,
  `0x807886E0`), whose state-name strings sit at `0x807DF960`;
* the `0x9CE8` manager (`0x8065C370`, whose constructor and wave-133
  siblings are already in Rust);

and three singletons: the `remo_kanaami` seat (`0x80757320`), the
message-screen refresher (`0x80609440`) and the course-table builder
(`0x80628A00`).

### Corrections carried into this wave

1. **`0x80786EA0` is not a GQR body.** Its export's `unaff_GQR0`/`ldexpf` is
   the `f31` `stfd`+`psq_st` spill; both `psq_` words are `qr0, W=0`.
2. **`0x80628A00` takes `this`.** The `void(void)` signature is an artifact of
   the `0x802DCD58` = `__savegprlr_20` save stub, which leaves `r3` intact.
3. **`xoris` is an upper-half XOR.** The three "1.0 × constant" ladders in
   `0x80741510` and the signed-int compares in `0x80786EA0` only resolve to
   sane floats/doubles with that rule; the low-half reading produces
   `-0x7FFF7FFF` and would have wrongly classified row 3 as conditional.
4. **`addi rD,0,imm` is `li`.** Several constants (`li r0,2` at `0x80628D48`,
   `li r0,1` at the two `0x80741604`/`0x8074162C` ladders) are literals, not
   copies of `r0`.
