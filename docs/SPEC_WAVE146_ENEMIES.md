# Wave 146 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped `d_enemiesNP`
addresses into 173 class **B**, 160 class **C** and 51 class **D**. Waves 128-145
worked through that block; this page is the next plain size-ranked tranche: the
twelve largest bodies that are still untranslated, still have a Ghidra export,
and whose reachable body fills its `functions.tsv` extent.

A live re-walk of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows)
against the `address` column of `ledger/functions.csv` (16,880 addresses), against
every `0x80Axxxxx`/`0x80Bxxxxx` address in a `^#` heading of any `docs/*.md`
(304 distinct addresses) and against every `fn_<addr>`/`FN_<addr>` token in
`src/**/*.rs` leaves **152** untranslated bodies, 151 of them with an export.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the **reachable body**, not the raw `functions.tsv` extent,
because the TSV size is a bucket extent that can include a neighbour's padding or
an unreachable arm (see `docs/REL_ELF_MAPPING.md`). Each candidate was walked from
its entry through every conditional branch and every internal `bl` until all
paths ended in `blr`, a tail `b` or a tail call. Rows whose walk stops early
because of a computed jump, an indirect `lwzu`+vtable call or a zero word are
**not** claimed here; they are listed in "Rows a size sort would place higher".

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A84F10` | 744 | 177 | 0 | `I_kinoko` brres/clip binder of the `daEnItem_c` block | **ready** |
| 2 | `0x80A56620` | 736 | 184 | 0 | multi-slot side classifier over the `+0x2484` segment table | **ready** |
| 3 | `0x80B0D6E0` | 696 | 174 | 0 | turn-order heading/pitch governor | **ready** |
| 4 | `0x80AB0570` | 664 | 158 | 0 | actor initialiser: 1532/16 lane ladder, two anim holders | **ready** |
| 5 | `0x80A0F610` | 660 | 165 | 0 | 648-byte effect-chain spawner | **conditional** |
| 6 | `0x80AB0EE0` | 640 | 160 | 0 | five-slot world-matrix / matrix-clip refresh | **ready** |
| 7 | `0x80A1C8F0` | 636 | 158 | 0 | player-attack collision dispatcher with a 31-entry table | **ready** |
| 8 | `0x80B16060` | 608 | 143 | 0 | `0x11D`-variant initialiser with three state transitions | **ready** |
| 9 | `0x80A1E990` | 560 | 140 | 4 | aim/placement step over a `qr3` angle | **ready** |
| 10 | `0x80B16F40` | 560 | 140 | 2 | angle to travel-vector step with the `150.0` gate | **ready** |
| 11 | `0x80AD98B0` | 544 | 136 | 0 | four-player overlap probe | **ready** |
| 12 | `0x80A85260` | 528 | 132 | 0 | `+0xDFA` fall/land state machine | **conditional** |

**Split: 10 ready, 2 conditional.** Total 7,576 bytes of TSV extent and 1,867
reachable instructions.

The two conditional rows each have exactly one external fact missing, named in
their **Verdict** line: a BSS-seeded angle table (`0x80A0F610`) and a `main.dol`
ground pointer (`0x80A85260`). Everything else in both bodies (strings, fields,
call order, constants) is literal.

### Rows a size sort would place higher

These are skipped deliberately, with the measured reason. Every one of them is
either recorded as deferred by an earlier page or leaves the linear stream:

| address | TSV size | reachable | why it is not claimed |
|---|---:|---:|---|
| `0x80A3DEA0` | 2940 | 231 | 16-case `bctr` table at `0x80B3EE94`; deferred in `docs/COVERAGE_WAVE145_ENEMIES.md` |
| `0x80B04E90` | 1592 | 398 | two-pass polyline analyser; deferred in `docs/COVERAGE_WAVE145_ENEMIES.md` |
| `0x80A7BEC0` | 1464 | 257 | clip-name stride table plus six unverified vtable `bctrl` slots; deferred in wave 145 |
| `0x80A9B3C0` | 1196 | 299 | `slot` argument staged only by the indirect caller; conditional in wave 145 |
| `0x80A92F40` | 1172 | 291 | three unledgered `main.dol` helpers; conditional in wave 145 |
| `0x80A56B10` | 672 | 32 | main path enters `lwzu r12,0x20D0(r3)` + vtable `+0xA0` at `0x80A56B8C`; the walk cannot follow it, so the body length is not established |
| `0x80A6FBC0` | 624 | 16 | body ends in a vtable `bcctr` at `0x80A6FBFC`; the remaining 140 words are padding |
| `0x80ADDE10` | 580 | 13 | body ends in a vtable `bcctr` at `0x80ADDE40`; the remaining 132 words are padding |
| `0x80AD4000` | 532 | 27 | body returns (`blr`) at `0x80AD4210`; the remaining 106 words are padding |
| `0x80A20900` | 520 | 40 | body returns (`blr`) at `0x80A20B04`; the remaining 90 words are padding |

Two notes for the coordinator:

* `docs/SPEC_WAVE145_ENEMIES.md` states that `0x80A84F10`, `0x80A56620` and
  `0x80A56B10` are "already named `fn_<addr>` bodies in `src/game/enemies.rs`".
  A re-check of that file finds **no** `fn_`/`FN_` token for any of the three;
  `0x80A84F10` appears only in the wave-24 block comment (line 32490) and
  `0x80A56620`/`0x80A56B10` only in a comment at line 155849. Ranks 1 and 2 are
  therefore genuinely untranslated and are claimed here; `0x80A56B10` is held
  back for the reason in the table above, not because a body exists.
* `0x80B16060` was named by wave 145 as "the largest remaining row and starts
  wave 146". It is rank 8 here because ranks 1-7 are larger bodies that wave 145
  passed over for four different reasons (see its `## Selection` section); all
  four of those reasons are re-verified above.

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`. The RAW stream of this
REL contains no `mtspr GQRn`, so every `psq_*` here depends on that `main.dol`
word. With `W = 1` and `I = 3` every data `psq` in this module is one unsigned
16-bit load scaled by `2^0`:

```
psq_l fD,0(rA),1,qr3      ; fD = (f32)*(u16 *)rA
```

Two of the twelve rows use it, six sites in total:

| address | site | what the halfword is |
|---|---|---|
| `0x80A1E990` | `0x80A1EA10` | `(s16)(target_yaw + +0x726)` - the sine angle |
| `0x80A1E990` | `0x80A1EA40` | the same halfword - the cosine angle |
| `0x80A1E990` | `0x80A1EAD8` | `(s16)(target_yaw + 0x4000)` - sine angle, mode B |
| `0x80A1E990` | `0x80A1EAF8` | the same halfword - cosine angle, mode B |
| `0x80B16F40` | `0x80B1704C` | the yaw from `0x801611C0` - sine |
| `0x80B16F40` | `0x80B17080` | the same yaw - cosine |

In both bodies the halfword is a stack temporary (`sth` then `psq_l` from
`r1+off`) and is pre-scaled by `DAT_80B2B6F0` / `DAT_80B322F0` = `1/256` before
the `sin`/`cos` call, so the port reads `(f32)(u16)halfword` and multiplies by
`1/256`.

Four `qr3` bodies remain untranslated after this wave, all at or below 520
bytes: `0x80A1EBC0` (520, 6 sites), `0x80A06560` (500, 2), `0x80A56920` (476,
4), `0x80A88BC0` (368, 2), `0x80A58D70` (308, 1), `0x80A80F40` (264, 1),
`0x80A584E0` (232, 2), `0x80A80E50` (220, 1), `0x80B22750` (128, 1). They are
the continuation of the `qr3` tranche.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments; the `+0x1000` formula of `docs/REL_ELF_MAPPING.md` applies to the
  first one (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`); later segments use
  their own `p_offset`/`p_vaddr`. The file-backed image ends at `0x80B6E438`;
  the last `PT_LOAD` is the zero-fill segment `0x80B6E500` (`memsz = 0xB4E0`),
  so anything at or above `0x80B6E500` is BSS and is marked **(bss)**.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z (the module's actor convention);
  `+0x100`/`+0x102`/`+0x104` are the three s16 angles; `+0xDC`/`+0xE0`/`+0xE4`
  are the three "colour/scale" floats; `+0x348` is the side/facing byte;
  `+0x38F` is the player id byte.
* `addi rX,r0,imm` is `li`, `addis rX,r0,imm` is `lis`, `or rX,rY,rY` is `mr`.
  `rlwinm rA,rS,sh,mb,me` is `ROTL32(rS,sh) & mask(mb..me)` with the mask bits
  numbered **from the MSB** (bit 0 = `0x80000000`, bit 31 = `1`). So
  `rlwinm r0,r3,0,28,31` is `r3 & 0xF`, `rlwinm r0,r3,16,31,31` is
  `(r3 >> 16) & 1`, `rlwinm r0,r3,2,22,29` is `(r3 & 0xFF) * 4`, and
  `rlwinm r0,r3,27,24,31` is `(r3 >> 5) & 0xFF`. (`clz` counts leading zeros;
  `cntlzw` is opcode 31/XO 26.)
* On a `bc`, `bo = 12` branches on a true CR bit and `bo = 4` on a false one;
  `bi & 3` is `LT/GT/EQ/SO`. `cror 2,1,2` (`0x4C411382`, XO 449) after an
  `fcmpo` rewrites `cr0.eq` as `gt|eq`, so the following `bne` reads
  `if !(a >= b)`; `cror 2,0,2` (`0x4C401382`) gives `lt|eq`.
* `psq_st`/`psq_l` with `qr0` into `0x30..0x58(r1)` next to a matching
  `stfd`/`lfd` is an **f31..f27 spill**, not quantisation. The `unaff_GQR0`
  noise and the `ldexpf` calls in the Ghidra exports of this module are that
  spill plus the `2^52` integer-to-double idiom.

## Symbolic helpers used below

| address | Rust symbol | contract used here |
|---|---|---|
| `0x80070BA0` | `fn_80070ba0(x: f64, y: f64, layer: u32) -> u32` | cell test; bit `0x400` of the result is set when the point is blocked |
| `0x80070BF0` | `fn_80070bf0(x: f64, y: f64, layer: u32) -> u32` | companion cell test; low byte `< 2` means "clear" |
| `0x801611C0` | `fn_801611c0(f1: f64, f2: f64) -> s16` | `atan2`-style yaw in 1/256 turns |
| `0x8015F4F0` | `step_short_clamped_wrap(dst: *mut s16, delta: s16, wrap: s32)` | wrapping s16 step, `wrap` = `0x100` |
| `0x800690C0` | `fn_800690c0` | fixed-size array/holder constructor (`this, -1, name, 0, count)`) |
| `0x800690E0` | `fn_800690e0` | matching destructor |
| `0x8008C3E0` / `0x8008C330` | `fn_8008c3e0` / `fn_8008c330` | begin/commit a matrix or animation block |
| `0x8008C200` | `fn_8008c200` | clear a matrix block |
| `0x80064BF0` / `0x80064C10` | `fn_80064bf0` / `fn_80064c10` | model visibility on / off (unledgered, both plain setters) |
| `0x80064610` | `fn_80064610` | set one player's attack/collision flags |
| `0x80022490` | `fn_80022490` | register a collision hit against a state |
| `0x80022180` / `0x800221E0` / `0x80022390` | player-attack helpers | unledgered; argument lists are the export's and match the raw `r3..r8` staging |
| `0x80169D90` | `fn_80169d90` | bind an animation clip into a holder |
| `0x801650D0` / `0x801651F0` / `0x80166830` / `0x80166BD0` | anim-holder pair binders | as used in wave 145 |
| `0x80169F80` / `0x80166EF0` / `0x80166F10` | anim rate setters | take `f1` first |
| `0x80168180` | `fn_80168180` | read bone matrix `n` into a stack buffer |
| `0x80237BD0` / `0x80237C40` | `fn_80237bd0` / `fn_80237c40` | `cosf`/`sinf`-style helpers on the `1/256`-scaled angle |
| `0x802B9210` | `fn_802b9210` | allocate (`0x288` bytes here) |
| `0x80239E30` / `0x8023A0B0` / `0x8023A200` | res clip lookups | `(resource, name) -> clip` |
| `0x800DF180` | `fn_800df180` | `(arc, name, path) -> resource` |
| `0x8005F900` | `fn_8005f900(index) -> *mut actor` | player actor lookup |
| `0x800B86F0` | `fn_800b86f0` | set a `0x484`-based 4-slot matrix effect |
| `0x802DCD4C/60/6C/70/74` | `_savegpr_N` | save helpers; incoming `r3..` survive |

---

## 1. `0x80A84F10` - `I_kinoko` brres/clip binder

* **Extent** 744 bytes (186 words), 177 reachable. Export
  `work/ghidra/rel-d_enemiesNP/export/80a84f10_FUN_80a84f10.c`.
* **Signature** `unsafe fn fn_80a84f10(this: usize, index: u32)` - `r3` = actor,
  `r4` = table index (masked to 16 bits). Returns `void`.
* **Role** the model binder of the `0xE68`-byte item actor (`daEnItem_c`), the
  one wave 24 left with the constructor: it loads `g3d/I_kinoko.brres`, binds
  the `wait` clip plus `I_kinoko_switch`, and builds either one animation holder
  or ten, depending on the `+0xDCA` state.

### Behaviour

1. `r5 = *(u32 *)0x80377C48` (`main.dol`), then
   `0x800690C0(this + 0x524, 0xFFFFFFFF, r5, 0, 0x20)`: construct the 32-entry
   base holder block at `this+0x524`.
2. `idx = (index & 0xFFFF) * 4` (`rlwinm r29,r29,2,14,29`).
3. Three parallel pointer tables, all in file-backed `.data`, are indexed by
   `idx`:
   * `*(char **)(0x80B4E218 + idx)` - the character name (`"I_kinoko"` for
     entry 0),
   * `*(char **)(0x80B4E2F4 + idx)` - the model path
     (`"g3d/I_kinoko.brres"` for entry 0),
   * `*(char **)(0x80B4E34C + idx)` - the base clip name (`"I_kinoko"`).
4. `res = 0x800DF180(*(u32 *)0x8042A038 + 4, name, path)`; `*(u32 *)(this+0x540) = res`.
5. `clip = 0x80239E30(this + 0x540, clipName)`.
6. If `*(s16 *)(this+0xDCA) ∈ {0, 0xD, 6}` bind with kind `0x23`, else with
   kind `0x20`:
   `0x80169D90(this + 0x544, &clip, this + 0x524, kind, 1, 0)`.
7. If `*(s16 *)(this+0xDCA) ∈ {3, 0x10}` call `0x80064BF0(this + 0x544)`,
   else `0x80064C10(this + 0x544)` (visible / hidden).
8. State dispatch, `s = *(u16 *)(this+0xDCA)`:
   * **base path** when `s < 3`, `s ∈ {0xD, 0xE}`, or `s ∈ {6, 4, 0xB}`:
     1. `anim = 0x8023A0B0(this + 0x540, "wait")` (`0x80B4E390`).
     2. `0x801650D0(this + 0x804, &{anim, clip}, &anim, this + 0x524, 0)`.
     3. `0x801651F0(this + 0x804, this + 0x544, &anim, 0)`.
     4. `0x80169F80(1.0f, this + 0x544, this + 0x804)`
        (`0x80B2E590 = 1.0`).
     5. If `s ∈ {0, 0xD, 6}`:
        * `clip2 = 0x8023A200(this + 0x540, "I_kinoko_switch")` (`0x80B4E398`);
          `*(u32 *)(this+0x83C) = clip2`;
        * `0x80166830(this + 0x840, &clip, &clip2, this + 0x524, 0, 1)`;
        * `rate = 0.0` (`0x80B2E4B4`), `1.0` when `s == 6` (`0x80B2E590`),
          `2.0` when `s == 0xD` (`0x80B2E5E0`);
        * `0x80166BD0(this + 0x840, this + 0x544, &clip2, 0, 1)`;
        * `(*(this+0x544)->vtable[0x18])(this + 0x544, this + 0x840)` -
          the only `bctr` in the body (`0x80A85168`);
        * `0x80166EF0(rate, this + 0x840, 0)`, then
          `0x80166F10(0.0f, this + 0x840, 0)`.
   * **ten-holder path** when `s == 0x10`: loop `i = 0..9` over
     `handles = this + 0x584 + i * 0x40`:
     `0x80169D90(handles, &clip, this + 0x524, 0x20, 1, 0)`,
     then `0x80064BF0(handles)`.
   * **otherwise** fall through to the teardown.
9. `0x800690E0(this + 0x524)`; return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B4E218` | `{"I_kinoko", ...}` | character-name pointer table |
| `0x80B4E2F4` | `{"g3d/I_kinoko.brres", ...}` | model-path pointer table |
| `0x80B4E34C` | `{"I_kinoko", ...}` | clip-name pointer table |
| `0x80B4E390` | `"wait"` | shared idle clip |
| `0x80B4E398` | `"I_kinoko_switch"` | switch clip |
| `0x80B2E590` | `1.0f` | base animation rate |
| `0x80B2E4B4` | `0.0f` | switch rate for state 0 |
| `0x80B2E5E0` | `2.0f` | switch rate for state `0xD` |
| `0x80B4E168` | `{0x80A83DE0, 0x003C003A, ...}` | class descriptor of the `0xE68` actor (constructor `0x80A83DE0`) |

### Port notes and verdict

* `r4` is `index`, not a byte pointer: the three tables are indexed, not
  offset. The same body binds `I_kinoko` (item 0) and its three siblings.
* The `psq_st f31`/`ldfd f31` pair is the spill of `f31`; the export's
  `in_f31` and `unaff_GQR0` are artefacts. The body has **no** float argument.
* The one `bctr` is the animation holder's own vtable slot `+0x18`; its
  receiver is `this+0x544`, a holder this body just filled, so the call is
  reconstructible as a method on the holder.
* **Verdict: ready.** Every string, table, clip name, state value and call
  order is literal; no quantiser, no unresolved base, no incoming float.

---

## 2. `0x80A56620` - multi-slot side classifier

* **Extent** 736 bytes (184 words), 184 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a56620_FUN_80a56620.c`.
* **Signature** `unsafe fn fn_80a56620(this: usize)` - `r3` only, returns
  `void`.
* **Role** classify every slot of a `+0x2484`-entry segment table into side
  `1/2/3` (left/right/both-clear), using two cell probes `4.0` units to either
  side of the slot's world position. It is the same three-way byte the item/segment
  blocks write at `+0x1E7`; the per-slot copies live at `+0x5EF` with stride
  `0xA4`.

### Behaviour

1. Cache `x = *(f32 *)(this+0xAC)`, `y = *(f32 *)(this+0xB0)`;
   `k = 4.0f` (`0x80B2CEC0`), and `layer = *(u8 *)(this+0x38F)`.
2. `a = 0x80070BA0(x + k, y, layer)`, `ab = 0x80070BF0(x + k, y, layer)`,
   `b = 0x80070BA0(x - k, y, layer)`, `bb = 0x80070BF0(x - k, y, layer)`.
   `ab`/`bb` are the low bytes of the results (`rlwinm ...,0,24,31`).
3. `clearA = (a & 0x400) == 0 || ab < 2`, `clearB = (b & 0x400) == 0 || bb < 2`.
4. `*(u8 *)(this+0x1E7) = 3` when `clearA && clearB`; else `1` when
   `((*(u32 *)(this+4) >> 16) & 1) == 0`; else `2`.
5. `n = *(u16 *)(this+0x2484)`; for `i = 0 .. n-1`:
   * `px = x + *(f32 *)(this + 0x1EF0 + i*0xC)`,
     `py = y + *(f32 *)(this + 0x1EF4 + i*0xC)`;
   * run the same four probes at `(px ± k, py)`;
   * `*(u8 *)(this + 0x5EF + i*0xA4) = 3 / 1 / 2` by the same rule.
6. If `*(u16 *)(this+0x2486) == 1`, repeat the same classification for the
   second half of the table, `j = i + n`:
   * record `(this + 0x1EF0 + j*0xC)`, byte `(this + 0x5EF + j*0xA4)`.
   The two arrays therefore hold `2n` entries when the mirror flag is set.
7. Return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2CEC0` | `4.0f` | probe half-width |
| `0x80B2CEB8` | `0.0f` | unused base word of the same literal pool (the `addis` sets `r4 = 0x80B2CEB8`, `lfs` reads `+8`) |
| `0x80070BA0` | `fn_80070ba0` | cell test, blocked bit `0x400` |
| `0x80070BF0` | `fn_80070bf0` | cell test, low-byte height index |

### Port notes and verdict

* Write strides are `0xC` for the position table (`+0x1EF0`) and `0xA4` for the
  per-slot byte table (`+0x5EF`); the mirror half starts at index `n`, so the
  tables must be sized `2n` whenever `+0x2486 == 1`.
* `+0x2484` is a **u16** count and `+0x2486` a **u16** flag; the loop compare is
  `cmpw` on the zero-extended count.
* The two probe helpers are `game::stage` translators with a recorded medium
  confidence; their bit `0x400`/low-byte encoding is the only non-literal fact
  and is already used the same way by the translated `0x80A87030` and
  `0x80A1E100` bodies.
* `docs/COVERAGE_WAVE127_ENEMIES.md` grouped this address with the firebar
  emitters; the field map (`+0x2484` count, `+0x1EF4` y offset, `+0x5EF`
  stride `0xA4`) is the segment/record pair described above, so the body is
  specified without asserting the class identity.
* **Verdict: ready.** Body reachable == extent, no quantiser, no incoming
  argument, all offsets literal.

---

## 3. `0x80B0D6E0` - turn-order heading/pitch governor

* **Extent** 696 bytes (174 words), 174 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b0d6e0_FUN_80b0d6e0.c`.
* **Signature** `unsafe fn fn_80b0d6e0(this: usize) -> u32` (`r3` in, `r3` out;
  returns `0` when the governor ran, `1` when it deferred).
* **Role** pick a side, latch it, ramp the horizontal tilt `+0xE8` and the
  vertical speed `+0xEC` through `0.625/1.0` and `0.0078125/0.03125` tables,
  then turn `+0x100` toward the target yaw. It is the step that
  `COVERAGE_WAVE142_ENEMIES.md:161` left open for its caller vtable.

### Behaviour

1. `d = fn_80b0d2a0(this, &vec)` - returns `target.x - this.x` (the target's
   world x/y come back through the two stack floats; `0.0` when no target).
2. `if (fn_80b0d4d0(this) == 0) return 1;` - the turn-order test says "not my
   turn".
3. `if (*(u16 *)(this+0x9E2) != 0) return 0;` - latched off.
4. `side = d < 0.0f` (unsigned 0/1). If `side != *(u8 *)(this+0x348)` and
   `*(u16 *)(this+0x9E0) == 0`, then set `*(u16 *)(this+0x9E0) = 1` and
   `*(u8 *)(this+0x348) ^= 1`.
5. Tilt, only when `*(u16 *)(this+0x500) == 0` and `*(u16 *)(this+0x9E0) == 0`:
   * `idx = (*(u32 *)(this+0x9D4) * 2 + side) & 0xFF`;
   * `lim = DAT_80B31E90[idx]`; if `!side` then
     `if (lim < tilt) tilt = lim; else carry = 1`, otherwise
     `if (tilt < lim) tilt = lim; else carry = 1`;
   * if `carry`, `tilt += DAT_80B31EB0[idx]`.
6. Pitch: `dy = vec.y - (8.0f + *(f32 *)(this+0xB0))` (`0x80B31E38 = 8.0`).
   * If `|dy| >= 4.0f` (`0x80B31ED0`):
     `idx2 = (*(u32 *)(this+0x9D4) * 2 + (dy < 0.0f)) & 0xFF`;
     if `*(u16 *)(this+0x500) == 0 && *(u16 *)(this+0x9E0) == 0` apply the
     same clamp against `DAT_80B31EA0[idx2]` with `<=` comparisons and, on
     carry, `+= DAT_80B31EC0[idx2]`;
   * else `*(f32 *)(this+0xEC) = 0.0f`.
7. Yaw: `yaw = 0x801611C0(d)` then `yaw -= 0x4000` when `*(u8 *)(this+0x348) == 0`,
   otherwise `yaw = 0x801611C0(d, -dy) + 0x4000`.
8. Clamp `yaw` to `[-0xE38, 0xE38]`.
9. `s16 t = *(s16 *)(this+0x100); 0x8015F4F0(&t, yaw, 0x100); *(s16 *)(this+0x100) = t;`
10. Return `0`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B31E34` | `0.0f` | zero / `d < 0` threshold |
| `0x80B31E38` | `8.0f` | target y offset |
| `0x80B31ED0` | `4.0f` | pitch dead zone |
| `0x80B31E90` | `{0.625, -0.625, 1.0, -1.0}` | tilt limits |
| `0x80B31EA0` | `{0.625, -0.625, 1.0, -1.0}` | pitch limits |
| `0x80B31EB0` | `{0.0078125, -0.0078125, 0.03125, -0.03125}` | tilt increment |
| `0x80B31EC0` | `{0.0078125, -0.0078125, 0.5, -0.5}` | pitch increment |

### Port notes and verdict

* `docs/ANALYSIS_WAVE127_ENEMIES.md:457` classified this row as C for an
  "incoming `f1`". The `psq_st f31,0x38(r1)` + `stfd f31,0x30(r1)` at
  `0x80B0D6F0`/`0x80B0D6F4` is the f31 **spill**; the first use of `f1` is
  after the `80B0D2A0` call. The body has no float argument.
* `fn_80b0d2a0` returns the signed x distance and writes `{target.x,
  target.y + 0xD4 + f(+0x9E6)}`; `fn_80b0d4d0` returns the 0/1/2 turn-order
  verdict. Both are in the export set and are specified by their use here.
* The `cror 2,1,2`/`cror 2,0,2` pairs at `0x80B0D7C0`/`0x80B0D7EC` and
  `0x80B0D8A4`/`0x80B0D8CC` invert the `fcmpo` result exactly as the export
  shows; keep the `>=`/`<=` direction.
* **Verdict: ready.** Tables, clamps and the `0xE38` bound are literal; the two
  callees are local bodies with a documented contract.

---

## 4. `0x80AB0570` - actor initialiser with the 1532/16 lane ladder

* **Extent** 664 bytes (166 words), 158 reachable. Export
  `work/ghidra/rel-d_enemiesNP/export/80ab0570_FUN_80ab0570.c`.
* **Signature** `unsafe fn fn_80ab0570(this: usize) -> u32` (always `1`).
* **Role** the constructor tail of the `0x80AB0000` actor: it derives side and
  variant from the `+4` descriptor word, places the actor on a 1532-unit lane
  with a 16-unit pitch, initialises two animation holders and enters the first
  state through the class vtable slot `+0xD4`.

### Behaviour

1. `*(u32 *)(this+0x988) = 0`; if `camera_mode_is_mid()` (`0x800CD250`) is
   non-zero, `*(u32 *)(this+0x988) = 1` and `*(u32 *)(this+0x9A0) = 0xFF`.
2. `w = *(u32 *)(this+4)`; `side = (w >> 4) & 0xF`; `lane = (w >> 8) & 0xF`;
   `n = (w & 0xF) + 1`.
3. `*(f32 *)(this+0xB4) = 1532.0f - 16.0f * lane` (`0x80B2FAD8`, `0x80B2FADC`).
4. `*(f32 *)(this+0x97C) = -16.0f * n`; `*(f32 *)(this+0x980) = 16.0f * n`
   (the two `2^52` integer conversions, `0x80B2FAF8`).
5. `*(u8 *)(this+0x348) = side`.
6. `*(f32 *)(this+0x890) = x`, `+0x894 = y`, `+0x898 = z` (copy of
   `+0xAC/+0xB0/+0xB4`); `*(u8 *)(this+0x99C) = 0xFF`.
7. `fn_80ab1510(this)`.
8. `*(s16 *)(this+0x102) = *(s16 *)(0x80B2FA74 + side*2)`, i.e.
   `{0x4000, 0xC000, 0x00FF, 0xFF00}[side]`.
9. `c = 1.0f` (`0x80B54B9C`); `*(f32 *)(this+0xDC/+0xE0/+0xE4) = c`.
10. First holder: `0x8008C3E0(this + 0x148, this, 0x80B2FA90)`; then
    `{-4.0, 0.0, 8.0, 8.0}` from `0x80B2FAE0`, `+0xE4`, `+0xE8` (used twice) into
    `+0x864/+0x868/+0x86C/+0x870` and `+0x164/+0x168/+0x16C/+0x170`;
    `*(u8 *)(this+0x1E6) = 1`; `0x8008C330(this + 0x148)`.
11. Second holder: `0x8008C3E0(this + 0x7C0, this, 0x80B2FA90)`; then
    `{12.0, 0.0, 8.0, 8.0}` (`0x80B2FAEC/E4/E8`) into
    `+0x874/+0x878/+0x87C/+0x880` and `+0x7DC/+0x7E0/+0x7E4/+0x7E8`;
    `*(u8 *)(this+0x85E) = 1`; `*(u8 *)(this+0x860) = *(u8 *)0x80429D28`;
    `0x8008C330(this + 0x7C0)`.
12. `fn_80ab1890(this)`; `*(u8 *)(this+0x36D) = 0`;
    `0x8006D090(this + 0x1EC, this, 0x80B2FAB4, 0x80B2FAC0, 0x80B2FACC)`.
13. `*(f32 *)(this+0x320) = *(f32 *)0x80B2FA80 (0.0)`;
    `+0x324 = 0.0` (`0x80B2FA84`); `+0x318 = 32.0` (`0x80B2FA88`);
    `+0x31C = 16.0` (`0x80B2FA8C`); `fn_80ab1780(1.0f, this, 0)`.
14. Side split: if `side == 0`, `+0xE8 = 0.3f` (`0x80B54BA0`), `+0xEC = +0xF0 = 0.0`
    (`0x80B2FAE4`); else `+0xE8 = -0.3f`, `+0xEC = 0.0`, `+0xF0 = 0.0`.
15. `*(f32 *)(this+0xF4) = *(f32 *)(this+0xE8)`;
    `*(f32 *)(this+0x11C) = 0.03125f` (`0x80B2FAF4`).
16. `(*(this+0x60)->vtable[0xD4])(this, 0x80B74FD0)` - state change **(bss
    descriptor)**; return `1`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2FA74` | u16 `{0x4000, 0xC000, 0x00FF, 0xFF00}` | per-side yaw |
| `0x80B2FA80/84` | `0.0, 0.0` | `+0x320/+0x324` |
| `0x80B2FA88/8C` | `32.0, 16.0` | `+0x318/+0x31C` |
| `0x80B2FA90` | inline `""` name | holder name argument |
| `0x80B2FAB4/C0/CC` | matrix-name table | `0x8006D090` arguments |
| `0x80B2FAD8` | `1532.0f` | lane origin |
| `0x80B2FADC` | `16.0f` | lane spacing |
| `0x80B2FAE0` | `-4.0f` | holder scale x |
| `0x80B2FAE4` | `0.0f` | zero |
| `0x80B2FAE8` | `8.0f` | holder scale |
| `0x80B2FAEC` | `12.0f` | holder scale |
| `0x80B2FAF0` | `1.0f` | `fn_80ab1780` argument |
| `0x80B2FAF4` | `0.03125f` | `+0x11C` |
| `0x80B2FAF8` | `0x43300000` | `2^52` for the two nibble conversions |
| `0x80B54B9C` | `1.0f` | colour/scale triple |
| `0x80B54BA0` | `0.3f` | lateral tilt |
| `0x80429D28` | `main.dol` byte | `+0x860` |

### Port notes and verdict

* The nibble map is `side = (w >> 4) & 0xF`, `lane = (w >> 8) & 0xF`,
  `n = (w & 0xF) + 1`, exactly the three expressions the export prints;
  `+0x348` gets the side nibble and `+0xB4` the lane nibble.
* `0x80B74FD0` is a BSS state descriptor (above the file-backed image end),
  the same pattern wave 145 used for `0x80B746xx`; the descriptor's name is
  outside the body, but the `+0xD4` call is the standard state change.
* The two `2^52` folds are ordinary `(f32)(integer)` conversions; the export's
  `DAT_80b2faf8` is the `0x43300000` half of the double.
* **Verdict: ready.** Every store, table and nibble is literal; the three local
  callees (`0x80AB1510`, `0x80AB1780`, `0x80AB1890`) are in the TSV.

---

## 5. `0x80A0F610` - 648-byte effect-chain spawner (conditional)

* **Extent** 660 bytes (165 words), 165 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a0f610_FUN_80a0f610.c`.
* **Signature** `unsafe fn fn_80a0f610(this: usize)` - `r3` only.
* **Role** spawn `n = *(int *)(this+0x540)` 648-byte (`0x288`) effect objects,
  chain them through `+0x238`, and seed each with a position `10.0` apart along
  z, the owner's three angles, and one of four `s16` yaw seeds taken from a
  **BSS** table.

### Behaviour

1. `side = *(u8 *)(this+0x348)`; `n = *(int *)(this+0x540)`;
   `x = *(f32 *)(this+0x5C4)`, `y = *(f32 *)(this+0x5C8)`, `z = *(f32 *)(this+0x5CC)`;
   `seed = (s16)*(u16 *)(0x80B6EF08 + side*2)` - **bss**, see verdict.
2. `*(u16 *)(this+0x5D0) = 0`; `*(s16 *)(this+0x5D2) = seed`;
   `*(u16 *)(this+0x5D4) = 0`.
3. `head = this + 0x544`; for `i = 0 .. n-1`:
   * `z -= 10.0f` (`0x80B2AE34`);
   * `e = 0x802B9210(0x288)`; if `e == 0` skip the seeding;
   * `*(u32 *)e = 0x803158D8` (vtable), `*(u32 *)(e+4) = side`,
     `0x80169CD0(e + 8)`, `0x80168D10(e + 0x48)`,
     `*(u32 *)(e+0x48) = 0x80B330E0` (sub-vtable);
   * `*(f32 *)(e+0x80) = x`, `+0x84 = y`, `+0x88 = z`;
     `*(u16 *)(e+0x8C) = side`, `+0x8E = side << 1`, `+0x90 = side`;
   * `0x8008C200(e + 0xC4)`, `0x8008C200(e + 0x168)`;
   * `*(u32 *)(e+0x224) = this`; `+0x228 = 1`; `+0x230 = side`;
     `+0x234 = 5.0f` (`0x80B2ADB4`); `+0x238 = 0`;
   * `0x800B6830(*(f32 *)0x8042C5C8, e, e + 0x80, e + 0x8C)`;
   * `*(u32 *)e = 0x80315840`; `+0x23C = 0`; `+0x240 = 0x803295D8`;
     `+0x244 = +0x248 = 0`; `0x8016A5E0(e + 0x24C)`;
   * `+0x240 = 0x80329548`; `+0x268 = 0`; `+0x26C = -1.0f` (`0x80B2AE38`);
     `+0x270 = i`; `+0x274 = 0`; `+0x275 = 0`; `+0x276 = 0`;
   * `*(u32 *)e = 0x80315770`; `+0x278 = 0x803157C0`; `+0x27C = e`;
     `+0x280 = +0x284 = 0`;
   * after the skip label: `*(f32 *)(e+0x234) = 5.0f`;
     `*(u32 *)(head + 0x238) = e`; `head = e`.
4. Return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2AD90` | `0.0f` | transient `+0x234` seed (then overwritten by `5.0`) |
| `0x80B2AE34` | `10.0f` | z spacing |
| `0x80B2AE38` | `-1.0f` | `+0x26C` |
| `0x80B2ADB4` | `5.0f` | `+0x234` |
| `0x80B6EF08` | **(bss)** `s16[4]` | per-side yaw seed |
| `0x80B330E0` | `0x00000000` | element sub-vtable (filled at runtime) |
| `0x803158D8`, `0x80315840`, `0x80315770`, `0x803157C0`, `0x803295D8`, `0x80329548` | `main.dol` | element vtables |
| `0x8042C5C8` | `main.dol` float | `0x800B6830` scale argument |

### Port notes and verdict

* `0x80B6EF08` is above the file-backed image end (`0x80B6E438`) and below the
  zero-fill segment, i.e. **BSS**; nothing in this body writes it. The four
  entries are `s16` yaw seeds (the same role `0x80B2FA74` plays in rank 4), and
  a port needs their writer or a recorded default.
* The `psq_st f31/f30/f29/f28` prologue is four spills; there is no incoming
  float and no quantiser in the body.
* `0x80B330E0` is a static descriptor word that reads `0` on disk and is
  patched at runtime; the element vtable `0x803158D8` is main.dol, so the
  element class is outside the REL (same situation as wave 145's effect rows).
* **Verdict: conditional.** One bounded step: read the four `s16` values out of
  the writer that fills `0x80B6EF08` (or pin them as a table with a comment).
  Everything else is literal.

---

## 6. `0x80AB0EE0` - five-slot world-matrix / matrix-clip refresh

* **Extent** 640 bytes (160 words), 160 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ab0ee0_FUN_80ab0ee0.c`.
* **Signature** `unsafe fn fn_80ab0ee0(this: usize)` - `r3` only.
* **Role** rebuild the actor's world matrix `this+0x7C` from
  `pos + rotation + scale` and publish it into the animation holder
  `this+0x544`; then copy three bone matrices into `+0x884..0x8B0`. Two modes:
  a single static matrix (`+0x99B == 0`) or five slots of 12-byte records.

### Behaviour

**Mode A - `*(u8 *)(this+0x99B) == 0`:**

1. `pos = {*(f32 *)(this+0xAC), *(f32 *)(this+0xB0), *(f32 *)(this+0xB4)}` into
   `r1+0x3C..0x44`; `rot = {*(s16 *)(this+0x100/0x102/0x104)}` into `r1+0x28..0x2C`.
2. `0x80064AF0(&pos, &rot, 1)`; `0x801C0BD0(pos.x, pos.y, pos.z, this+0x7C)`.
3. `0x8016EDD0(this+0x7C, &rot.y)`; `0x8016ECB0(this+0x7C, &rot.x)`;
   `0x8016EEF0(this+0x7C, &rot.z)`.
4. `0x8016A170(this+0x544, this+0x7C)`; `0x8016A150(this+0x544, this+0xDC)`;
   `0x8016A1A0(this+0x544, 0)`.
5. `0x80168180(this+0x544, 4, buf)` -> `*(f32 *)(this+0x89C/+0x8A0/+0x8A4) = buf[0..2]`.
6. `0x80168180(this+0x544, 0, buf)` -> `+0x8A8/0x8AC/0x8B0`.
7. `0x80168180(this+0x544, 5, buf)` -> `+0x884/0x888/0x88C`.

**Mode B - `*(u8 *)(this+0x99B) != 0`:** for `i = 1 .. 4` over
`rec = this + 0x8CC + (i-1)*0xC` (x/y/z floats) and `ang = this + 0x940 + (i-1)*6`
(three s16):

1. `pos = {rec.x, rec.y, rec.z}`, `rot = {ang[0..2]}`;
2. `0x80064AF0`, `0x801C0BD0`, the three `0x8016Exxx` calls as above;
3. `holder = this + 0x584 + (i-1)*0x40`;
   `0x8016A170(holder, this+0x7C)`, `0x8016A150(holder, this+0xDC)`,
   `0x8016A1A0(holder, 0)`;
4. only at `i == 1`: `0x80168180(holder, 1, buf)` ->
   `*(f32 *)(this+0x89C/+0x8A0/+0x8A4) = buf[0..2]`.

Return.

### Constants

The body reads no float literal; all values come from the record at
`this+0x8CC` and the angle array at `this+0x940`. The only literal addresses are
the six engine helpers (`0x80064AF0`, `0x801C0BD0`, `0x8016EDD0`, `0x8016ECB0`,
`0x8016EEF0`, `0x8016A170/150/1A0`, `0x80168180`).

### Port notes and verdict

* Mode B's loop is `i = 1..4` (five slots including the `+0x584` base that mode A
  also uses); the record stride is `0xC` for positions and `6` for the angle
  triples, the holder stride is `0x40`.
* Only `i == 1` publishes bone 1 into `+0x89C`; slots 2-4 keep the previous
  values. This asymmetry is in the stream (`cmpi cr0,0,r26,1` at
  `0x80AB10FC`), not an export artefact.
* `0x80064AF0` and `0x801C0BD0` are unledgered `main.dol` helpers; the argument
  staging (`&pos`, `&rot`, `1`) is visible in the raw registers.
* **Verdict: ready.** No quantiser, no indirect call, no external table.

---

## 7. `0x80A1C8F0` - player-attack collision dispatcher

* **Extent** 636 bytes (159 words), 158 reachable. Export
  `work/ghidra/rel-d_enemiesNP/export/80a1c8f0_FUN_80a1c8f0.c`.
* **Signature** `unsafe fn fn_80a1c8f0(this: usize, arg: u32) -> u32`
  (`r3` = actor, `r4` = the "is real attack" flag staged by the caller).
  Returns `1` when a hit was registered, `0` otherwise.
* **Role** the body between the `_savegpr_24`/`_restgpr_24` mutex pair: it
  snapshots the actor position, consumes the `+0x648` latch, runs the 31-entry
  state table at `0x80B38334`, and emits either one `0x80064610` hit per
  multi-hit arc or a single hit for the non-attack path.

### Behaviour

1. `if (*(u32 *)(this+0x648) != 0) return 0;` - already consumed this frame.
2. `pos = {*(f32 *)(this+0xAC), *(f32 *)(this+0xB0), *(f32 *)(this+0xB4)}`;
   `w = *(u32 *)(this+4)`; `mask = w & 0xF0FFFF7F`.
3. Early-out: if `*(u32 *)(this+0x800) == 0x15 && arg == 1 &&
   0x80022180(this, *(s8 *)(this+0x823)) != 0` then return `0`.
4. `special = table31[*(u32 *)(this+0x800)]` where
   `table31 = (u32 *)0x80B38334`; entries pointing at `0x80A1C9A8` mean
   `special = 1` (states `2,3,4,10,12,13,18,29,30`), entries pointing at
   `0x80A1C9AC` mean `special = 0` (all others). Values `> 30` take the
   `special = 0` path without reading the table.
5. `hits = 0x80022390(this)`.
6. `if (hits == 0 || *(u32 *)(this+0x804) != 0 || special != 0)` -> **single-hit
   path**:
   1. if `*(u32 *)(this+0x800) == 0x15 && arg == 0 &&
      0x80022180(this, *(s8 *)(this+0x823)) != 0` then `pos.y -= 8.0f`
      (`0x80B2B614`);
   2. if `*(u8 *)(this+0x81C) == 0` then
      `0x80064610(0x3C, mask, &pos, 0, 0)` else
      `0x80064610(0x3C, (w & 0x8003001F) | 0x03000000, &pos, 0, 0)`;
   3. `*(u32 *)(this+0x648) = 1`;
   4. `0x80022490(this, &pos, *(u32 *)(this+0x800), *(s8 *)(this+0x823), 0, 0)`;
   5. return `1`.
7. else -> **multi-hit path**:
   1. `0x800221E0(this, w & 0x1F)` (the attack id);
   2. if `*(u8 *)(this+0x824) == 3` then `flags = 0x0C000000` and
      `pos.y -= 8.0f`; else `flags = 0x0B000000` and `pos.y += 16.0f`
      (`0x80B2B618`);
   3. for `i = 0 .. hits` (inclusive):
      `0x80064610(0x3C, *(u32 *)(this + 0x650 + i*4) |
       0x80B2B630[i] | 0x80B2B640[i] | flags, &pos, 0,
       *(u8 *)(this+0x38F))`, then
      `0x80022490(this, &pos, *(u32 *)(this+0x650 + i*4),
       *(s8 *)(this+0x823), 1, 0)`;
   4. `*(u32 *)(this+0x648) = 1`; return `1`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2B614` | `8.0f` | y drop for the "air" attack |
| `0x80B2B618` | `16.0f` | y lift for the "low" attack |
| `0x80B2B630` | u32 `{0x00000000, 0x80000000, 0x00000000, 0x80000000}` | per-arc flag |
| `0x80B2B640` | u32 `{0, 0, 0x00400000, 0x00400000}` | per-arc flag |
| `0x80B38334` | 31-entry code table | state dispatch, verified below |

`0x80B38334` entry -> target (index 0..30):

```
0..30 = C9AC C9AC C9A8 C9A8 C9A8 C9AC C9AC C9AC C9AC C9AC C9A8 C9AC C9A8 C9A8
        C9AC C9AC C9AC C9AC C9A8 C9AC C9AC C9AC C9AC C9AC C9AC C9AC C9AC C9AC
        C9AC C9A8 C9A8
```

`0x80A1C9A8` is `li r27,1`; `0x80A1C9AC` is the `or r3,r31,r31` before the
`0x80022390` call. So `special` is exactly the set
`{2, 3, 4, 10, 12, 13, 18, 29, 30}` the export prints.

### Port notes and verdict

* The jump table is file-backed and readable; `COVERAGE_WAVE116_ENEMIES.md:109`
  left this row open because the export sits between the mutex pair. Both the
  table and the mutex pair resolve, so the body is complete.
* `mask` is `w & 0xF0FFFF7F` on the plain path and
  `(w & 0x8003_001F) | 0x0300_0000` on the `+0x81C` path; the `oris` writes
  `0x0300_0000`, not `0x3000_0000`.
* `0x80064610`'s second argument is the collision flag word built above; the
  first is the constant `0x3C` (60 frames).
* **Verdict: ready.** Jump table read and quoted; no quantiser, no float
  argument.

---

## 8. `0x80B16060` - `0x11D`-variant initialiser

* **Extent** 608 bytes (152 words), 143 reachable. Export
  `work/ghidra/rel-d_enemiesNP/export/80b16060_FUN_80b16060.c`.
* **Signature** `unsafe fn fn_80b16060(this: usize) -> u32` (always `1`, through
  `_restgpr_24` returning `1`).
* **Role** the initialiser of the `0x80B1....` actor's "big" variant: sets the
  `+0x628` flag from `*(u16 *)(this+8) == 0x11D`, maps the `+4` descriptor's
  nibbles to side / variant, halves the collision span, builds the render block
  and enters one of three states.

### Behaviour

1. `*(u32 *)(this+0x628) = 0`; if `*(u16 *)(this+8) == 0x11D` then
   `*(u32 *)(this+0x628) = 1`.
2. `w = *(u32 *)(this+4)`; `*(u8 *)(this+0x348) = (w >> 8) & 0xF`;
   `variant = (w >> 4) & 0xF` (`r27`); `*(u32 *)(this+0x618) = w & 0xF`.
3. `*(f32 *)(this+0x614) = 1.0f` (`0x80B322B4`).
4. `fn_80b16ce0(this)` (the base initialiser).
5. If `+0x628 == 1`: `c = 1.0f` (`0x80B69E40`), `*(f32 *)(this+0xDC/+0xE0/+0xE4) = c`.
   Else: `c = 1.0f` (`0x80B69E30`), same three stores, then
   `0x800B86F0(this + 0x484, 2, 0, 5)`.
6. `span = 16.0f` (`0x80B322B8`); `base = (f32)((w >> 4) & 0xF)` via
   `0x43300000` (`0x80B322C0`):
   * if `*(u32 *)(this+0x618) == 0` (i.e. `w & 0xF == 0`):
     `z = *(f32 *)(this+0xB0)`;
   * else `z = *(f32 *)(this+0xAC)`;
   * `*(f32 *)(this+0x60C) = z`; `+0x604 = z - span*base`; `+0x608 = z + span*base`.
7. `0x8008C3E0(this + 0x148, this, 0x80B32290)`; `*(u8 *)(this+0x1E6) = 1`;
   `0x8008C330(this + 0x148)`.
8. If `+0x628 == 1`, `*(u16 *)(this+0x180) |= 0x100`.
9. Render block: `+0x610 = 14.0f` (`0x80B69E34`);
   `+0x320 = 0.0`, `+0x324 = 0.0` (`0x80B32280/84`);
   `+0x318 = 32.0`, `+0x31C = 16.0` (`0x80B32288/8C`);
   `scale = 80.0f + *(f32 *)0x8042BB08` (`0x80B322BC`); `+0x340 = +0x344 = scale`;
   `*(u8 *)(this+0x36D) = 0`.
10. State:
    * `variant == 0`: `*(u32 *)(this+0x618) = 2`;
      `vtable[0xD4](this, 0x80B68EB8)`;
    * else if `*(u32 *)(this+0x618) == 0`: `vtable[0xD4](this, 0x80B78EF8)`;
    * else `vtable[0xD4](this, 0x80B78F38)`.
11. Return `1`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B322B4` | `1.0f` | `+0x614` |
| `0x80B322B8` | `16.0f` | half-span |
| `0x80B322BC` | `80.0f` | base render scale |
| `0x80B322C0` | `0x43300000` | `2^52` for the nibble conversion |
| `0x80B32280/84` | `0.0, 0.0` | `+0x320/+0x324` |
| `0x80B32288/8C` | `32.0, 16.0` | `+0x318/+0x31C` |
| `0x80B32290` | inline `""` | render-block name argument |
| `0x80B69E30/40` | `1.0f` | colour triple, both branches |
| `0x80B69E34` | `14.0f` | `+0x610` |
| `0x80B68EB8`, `0x80B68EF8`, `0x80B68F38` | data records | state descriptors passed to `+0xD4` |
| `0x8042BB08` | `main.dol` float | render-scale bias |

### Port notes and verdict

* The raw maps the `+4` word as `side = (w>>8)&0xF`, `variant = (w>>4)&0xF`,
  `slot = w&0xF`; the export prints the same three expressions, so keep them.
* The three descriptor arguments are 0x40 bytes apart in file-backed `.data`
  (`0x80B68EB8`, `0x80B68EF8`, `0x80B68F38`); each is the first word of a
  `{s32, ptr, 0}` record block, not a string. They are passed by address only.
* `0x8008C3E0`'s name argument `0x80B32290` is an inline empty string, not a
  descriptor.
* **Verdict: ready.** The `+0xD4` slot is the class state-change method; its
  receiver is `this` and the argument is a literal address in this REL.

---

## 9. `0x80A1E990` - aim/placement step over a `qr3` angle

* **Extent** 560 bytes (140 words), 140 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a1e990_FUN_80a1e990.c`.
* **Signature** `unsafe fn fn_80a1e990(this: usize)` - `r3` only.
* **Role** keep the actor on a circle around a target: read the target's
  position and yaw from `0x80162D00(this+0x71C)`, add the local offsets
  `+0x724/+0x726` (mode A) or `0x4000` (mode B), convert the resulting
  `s16` angle with `qr3` + `1/256`, and write `pos = target + r*(sin,cos) -
  offset + delta`.

### Behaviour

1. `t = 0x80162D00(*(u32 *)(this+0x71C))`; if `t == 0` return.
2. `tx = *(f32 *)(t+0xAC)`, `ty = *(f32 *)(t+0xB0)`, `tyaw = *(s16 *)(t+0x104)`;
   `mode = *(u8 *)(this+0x714)`.
3. **Mode A (`mode == 1`)**
   1. `*(s16 *)(this+0x712) = tyaw`;
      `a = tyaw + *(s16 *)(this+0x726)`;
      `*(s16 *)(this+0x104) = *(s16 *)(this+0x724) + tyaw`.
   2. `h = (s16)a`; `s = sin((f32)(u16)h * (1/256))`,
      `c = cos((f32)(u16)h * (1/256))`.
   3. `*(f32 *)(this+0xAC) = ((tx + *(f32 *)(this+0x720) * s)
      - *(f32 *)(this+0xD0)) + *(f32 *)(this+0x618)`.
   4. `*(f32 *)(this+0xB0) = ((ty + *(f32 *)(this+0x720) * c)
      - *(f32 *)(this+0xD4)) + *(f32 *)(this+0x61C)`.
   5. `*(f32 *)(this+0xB4) = (*(f32 *)(this+0xB4) - *(f32 *)(this+0xD8))
      + *(f32 *)(this+0x620)`.
4. **Mode B (`mode != 1`)**
   1. `*(s16 *)(this+0x104) = tyaw + 0x4000`.
   2. `s = sin((f32)(u16)(tyaw+0x4000) * (1/256))`,
      `c = cos(...)`.
   3. `k1 = *(f32 *)(this+0x6F8)`, `k2 = *(f32 *)(this+0x6FC)`.
   4. `*(f32 *)(this+0xAC) = ((tx + k1*s - k2*c) - *(f32 *)(this+0xD0))
      + *(f32 *)(this+0x618)`.
   5. `*(f32 *)(this+0xB0) = ((ty + k1*c + k2*s) - *(f32 *)(this+0xD4))
      + *(f32 *)(this+0x61C)`.
   6. `*(f32 *)(this+0xB4) = (*(f32 *)(this+0xB4) - *(f32 *)(this+0xD8))
      + *(f32 *)(this+0x620)`.
5. Return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2B6F0` | `0.00390625f` (1/256) | angle to radians |
| `0x80A1EA10/0x80A1EA40/0x80A1EAD8/0x80A1EAF8` | `qr3` sites | `(f32)(u16)halfword` |

### Port notes and verdict

* The four `psq_l f1,0(r4),1,qr3` sites read a **u16** stack copy of the `s16`
  angle; with `GQR3 = 0x00050005` the load is `(f32)*(u16 *)`, exactly as the
  wave-138 pin says. Do not model them as paired floats.
* The `(f32)` conversion then multiplies by `1/256`, so the angle unit is
  `1/256` turn and `0x4000` is a quarter turn.
* `0x80162D00` is `resource::registry::fn_80162d00` (a lookup by id); the target
  is re-read every frame, and `+0x720`/`+0x6F8`/`+0x6FC` are the radii.
* The two branches differ only in the rotation basis (single radius vs a
  two-term `k1/k2` basis) and in adding `0x4000` to the yaw; both write the
  same three position fields.
* **Verdict: ready.** `qr3` is pinned, all constants are literal.

---

## 10. `0x80B16F40` - angle to travel-vector step with the `150.0` gate

* **Extent** 560 bytes (140 words), 140 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80b16f40_FUN_80b16f40.c`.
* **Signature** `unsafe fn fn_80b16f40(this: usize)` - `r3` only.
* **Role** convert the vector to a target into two angle deltas: read the target
  point through `0x80064740(this, &vec)`, gate on distance `> 150.0`, compute
  the yaw with `0x801611C0`, then step `+0x102`/`+0x100` by
  `delta * 1.5 * 0.25 * 4 * (1/256 angle)` divided by `48.0` (or by the
  `0x11D` variant's `910.222 / 48.0`).

### Behaviour

1. `if (0x80064740(this, &vec) == 0) return;` - `vec` is `{x, y, z}` at
   `r1+0x18..0x20`.
2. If `*(u32 *)(this+0x628) == 0`:
   * `0x8006CED0(&conv, target)` (`target` is the returned pointer; `conv` is
     the `{x, y, z}` copy at `r1+0x20..0x28`);
   * `dy = |conv[1] - *(f32 *)(this+0xB0)|`, `dx = |conv[0] - *(f32 *)(this+0xAC)|`;
   * `d = 0x802BE470(dx*dx + dy*dy)` (a length);
   * `if (|d| > 150.0f)` (`0x80B69E60`): step `+0x102` by `0` with wrap
     `0x100`, step `+0x100` by `0`, and `return`. (Both
     `0x8015F4F0(&s16, 0, 0x100)` calls.)
3. `yaw = 0x801611C0(-vec[0], vec[1])`; `s16 a = yaw`.
4. `s = sin((f32)(u16)a * (1/256))`; `c = cos(...)`.
5. `*(f32 *)(this+0x5F0) = 1.5f * 0.25f * (4.0f * s)`.
6. `*(f32 *)(this+0x5F4) = 4.0f * c`.
7. `wrap = 0x80`; `num = 2000.0f`, `den = 48.0f`; if `*(u32 *)(this+0x628) == 1`
   then `wrap = 0x100`, `num = 910.222f`, `den = 48.0f`.
8. `step = (int)((vec[0] * num) / den)`;
   `s16 t = *(s16 *)(this+0x102); 0x8015F4F0(&t, step, wrap); *(s16 *)(this+0x102) = t`.
9. Same for the second axis: `step = (int)((-vec[1] * num) / den)`;
   `s16 u = *(s16 *)(this+0x100); 0x8015F4F0(&u, step, wrap); *(s16 *)(this+0x100) = u`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B69E60` | `150.0f` | distance gate |
| `0x80B322F0` | `0.00390625f` (1/256) | angle to radians |
| `0x80B322F4` | `4.0f` | speed factor |
| `0x80B322E0` | `0.25f` | speed factor |
| `0x80B69E4C` | `1.5f` | speed factor |
| `0x80B69E50` | `2000.0f` | step numerator, normal variant |
| `0x80B69E54` | `48.0f` | step denominator |
| `0x80B69E58` | `2800.0f` | second-axis numerator |
| `0x80B69E5C` | `48.0f` | second-axis denominator |
| `0x80B322F8` | `910.2222f` | `0x11D` variant numerator |
| `0x80B322FC` | `48.0f` | `0x11D` variant denominator |
| `0x80B1704C` / `0x80B17080` | `qr3` sites | `(f32)(u16)` sine / cosine angle |

### Port notes and verdict

* The two `qr3` sites read the `s16` yaw stack copy twice (sine and cosine).
  `GQR3 = 0x00050005` -> `(f32)(u16)`, then `* (1/256)`.
* The `150.0` gate uses `|d|` (`fabs` + `frsp`) and only runs for the
  `+0x628 == 0` variant; the `0x11D` variant skips the distance test entirely.
* `0x80064740` is unledgered but its two arguments (`this`, `&vec`) and its
  0/non-0 return are visible in the raw register staging.
* `0x8006CED0` is the target-position copy (`(out, target)`), also unledgered.
* The two `fdivs`+`fctiw` pairs are the integer truncations; keep the sign
  (`-vec.z` for the second axis).
* **Verdict: ready.** Both `qr3` sites are `(f32)(u16)` and all six ratio
  constants are literal words in this REL.

---

## 11. `0x80AD98B0` - four-player overlap probe

* **Extent** 544 bytes (136 words), 136 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80ad98b0_FUN_80ad98b0.c`.
* **Signature** `unsafe fn fn_80ad98b0(this: usize) -> u32` - returns `0` when a
  player overlaps, `1` when none does.
* **Role** the four-player overlap test of the `0x4C30` container: lazily fill
  two eight-float offset tables from a four-float literal pool, then test each
  of the four players' `x + 0x1030` box against the actor's
  `x + table[side*8 + slot*2]`.

### Behaviour

1. Lazily initialise two tables (guarded by `0x80B769B0` / `0x80B769F8`):
   * `0x80B769B8[16]` from `{-8, 40, -40, 8, -40, 40, -8, 8, -40, 40, -8, 8,
     -40, 8, -8, 40}` (`0x80B30A20..2C` cycled),
   * `0x80B76A00[16]` from the same pool in a different order
     (`{40, -8, 8, -40, 8, -8, 40, -40, 8, -8, 40, -40, 8, -40, 40, -8}`).
2. `side = *(u8 *)(this+0x348)`; `slot = *(u32 *)(this+0x550)`.
3. `x0 = this.x + T1[side*8 + slot*2]`, `x1 = this.x + T2[side*8 + slot*2]`,
   `y0 = this.y + T1[side*8 + slot*2 + 1]`,
   `y1 = this.y + T2[side*8 + slot*2 + 1]`.
4. `for i = 0..3`:
   `p = 0x8005F900(i)`; if `p == 0` continue;
   accept (`return 0`) when
   `x0 <= p.x + 8.0` and `p.x - 8.0 <= x1` and
   `y0 <= p.y + *(f32 *)(p+0x1030)` and `p.y <= y1`.
5. `return 1`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B30A20` | `-8.0f` | offset pool |
| `0x80B30A24` | `40.0f` | offset pool |
| `0x80B30A28` | `-40.0f` | offset pool |
| `0x80B30A2C` | `8.0f` | offset pool and the ±8 box |
| `0x80B769B0` / `0x80B769F8` | **(bss)** init flags | one-shot table guards |
| `0x80B769B8` / `0x80B76A00` | **(bss)** `f32[16]` | the two offset tables |

### Port notes and verdict

* `COVERAGE_WAVE80_ENEMIES.md:140` recorded this row as having an incoming
  `f4` plus ten spill words. The five `stfd`/`psq_st` pairs at
  `0x80AD98CC..0x80AD98F0` are the f31..f27 **spills**; there is no float
  argument and no quantiser.
* The tables are BSS but this body writes them itself before use, so the
  initial values are inside the body (the literals are the eight floats at
  `0x80B30A20..0x80B30A2C`). The write order is verifiable from the two
  sixteen-store blocks at `0x80AD9904..0x80AD995C` and
  `0x80AD996C..0x80AD99C4`.
* The comparison chain is asymmetric: the low edge uses `<=` against
  `p.x + 8.0`, the high edge uses `<=` against `x1` with `p.x - 8.0`; keep the
  `cror`-modified directions exactly as at `0x80AD9A4C..0x80AD9A78`.
* `0x8005F900` is `game::player::fn_8005f900`, already translated.
* **Verdict: ready.** Self-initialising tables, all offsets literal.

---

## 12. `0x80A85260` - `+0xDFA` fall/land state machine (conditional)

* **Extent** 528 bytes (132 words), 132 reachable (exact). Export
  `work/ghidra/rel-d_enemiesNP/export/80a85260_FUN_80a85260.c`.
* **Signature** `unsafe fn fn_80a85260(this: usize, partner: u32)` - `r3` actor,
  `r4` non-zero when a second (partner) actor is present.
* **Role** the item actor's `+0xDFA` wobble/stand machine: state 0 falls until
  the ground threshold, state 1 waits for input/landing, state 2 re-checks,
  state 3 runs `0x80A86070`/`0x80A865E0`/`0x80A85D60`.

### Behaviour

1. `st = *(u8 *)(this+0xDFA)`.
2. **`st == 0`**: if
   `*(f32 *)(this+0xB0) - 8.0f <= -*(f32 *)(*(u32 *)0x80429E00 + 0x3C)`
   then `*(u8 *)(this+0xDFA) = 1`. (The right side is the negated first word of
   the `main.dol` ground object.)
3. **`st == 1`**:
   1. `in = 0x80072BC0(this + 0x1EC)`; `grounded = 0x800976B0(this, this+0xAC)`;
   2. if `!grounded`, `*(f32 *)(this+0xF8) = -8.0f` (`0x80B2E5B0`);
   3. `ok = (in & 0x8000) != 0 ? 0 : ((clz(in) >> 5) & 0xFF)`;
   4. if `partner != 0 && 0x8006EF40(this + 0x1EC, 0) != 0` then `ok = 0`;
   5. if `ok == 0`: `*(u8 *)(this+0xDFA) = 2`;
      else `*(u8 *)(this+0xDFA) = 3`, `*(u32 *)(this+0xBD4) = 0x800`,
      `*(u32 *)(this+0xBE0) = 0x800` when `partner != 0`, and
      `*(u8 *)(this+0xDF4) = 0`.
4. **`st == 2`**:
   1. same `in` and `grounded` reads; same `+0xF8 = -8.0` store;
   2. `ok = in == 0 || (in & 0x8000) != 0`;
   3. if `partner != 0 && 0x8006EF40(this+0x1EC, 0) != 0` then `ok = 0`;
   4. if `ok`: same three stores as 3.5 (`+0xDFA = 3`, `+0xBD4 = 0x800`,
      optional `+0xBE0 = 0x800`, `+0xDF4 = 0`).
5. **`st == 3`** (anything else): if `0x80072BC0(this+0x1EC) != 0`:
   * if `*(u8 *)(this+0xDF4) == 0` then `*(u8 *)(this+0xDF4) = 1` and
     `0x80A86070(this)`;
   * `*(f32 *)(this+0xEC) = 0.0f` (`0x80B2E4B4`);
   * `0x80A865E0(this)`; `0x80A85D60(this)`.
6. Return.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2E58C` | `8.0f` | ground offset subtracted from `+0xB0` |
| `0x80B2E5B0` | `-8.0f` | `+0xF8` when airborne |
| `0x80B2E4B4` | `0.0f` | `+0xEC` in state 3 |
| `0x80429E00` | `main.dol` pointer | ground object; `+0x3C` is the first float |
| `0x800` | literal | `+0xBD4`/`+0xBE0` timer |

### Port notes and verdict

* `0x80072BC0` (`game::stage::fn_80072bc0`) and `0x800976B0`
  (`game::stage::fn_800976b0`) are already translated; the only unresolved
  read is `*(f32 *)(*(u32 *)0x80429E00 + 0x3C)`, a `main.dol` ground object
  whose layout is not in this REL. Wave 24 recorded the same gap ("needs the
  ground-contact block as context").
* The `clz`-based `ok` value is `((cntlzw(in) >> 5) & 0xFF)` only when bit 15
  (`0x8000`) is clear; when it is set `ok = 0`. This is the "six frames of
  grace" count the truncated export shows.
* `0x80A86070`, `0x80A865E0` (20 bytes) and `0x80A85D60` are in the TSV.
* **Verdict: conditional.** One bounded step: the `main.dol` ground object at
  `0x80429E00` (`+0x3C` first float) - name it and the row is ready.

---

## Cross-checks between the twelve

* **Spill vs quantiser.** Nine of the twelve begin with
  `stfd fN` + `psq_st fN,...,0,qr0` pairs. All of them are `f31..f27` saves and
  all are restored with `psq_l ...,0,qr0` + `lfd` in the epilogue. Only ranks 9
  and 10 contain a data `qr3` site (six in total, quoted above).
* **Two bodies share a lane.** Ranks 1 and 2 both belong to the `I_kinoko`
  item block: rank 1 is the model binder (`+0x524/+0x540/+0x544` holder
  triplet) and rank 2 is the per-slot classifier (`+0x1E7`, `+0x5EF`). They do
  not touch the same fields.
* **Two bodies share the `0x80AB` initialiser pattern.** Ranks 4 and 6 both
  write `+0xDC..+0xE4`, both call `0x8008C3E0`/`0x8008C330` on a holder, and
  both end in a class-vtable `+0xD4` state change; rank 4 uses
  `0x80B74FD0`, rank 6 uses `0x80B78Exx`.
* **Two bodies share the `0x80B1` variant flag.** Ranks 8 and 10 both test
  `*(u32 *)(this+0x628)` for the `0x11D` variant; rank 8 sets it from
  `*(u16 *)(this+8) == 0x11D`, rank 10 reads it to flip the wrap/divisor pair.
* **Two `sin`/`cos` pairs, two conventions.** Rank 9 uses one radius
  (`+0x720`) in mode A and a two-term basis (`+0x6F8/+0x6FC`) in mode B; rank 10
  uses `4.0 * sin` and `4.0 * cos` separately. Both scale the halfword by
  `1/256` before the call.
* **Frame timing.** Only rank 7 writes a timer (`+0xBD4/+0xBE0 = 0x800`) and
  rank 12 uses `0x800` through `+0xBD4/+0xBE0`; the two values are independent
  counters, not a shared constant.

## Verification

* `functions.tsv` row count 3,555; ledger addresses 16,880; heading addresses
  304; Rust `fn_`/`FN_` tokens 15,997.
* Every one of the twelve decodes end-to-end from its entry with the
  `file_offset = address - 0x80A00000 + 0x1000` mapping; the walk produces no
  out-of-extent target, no zero word and no unknown opcode.
* Reachable/extent ratios: 1.00 for eight rows, 0.95 (`0x80A84F10`), 0.95
  (`0x80AB0570`), 0.99 (`0x80A1C8F0`), 0.94 (`0x80B16060`). The shortfalls are
  the epilogue words after a `bcctr`/`b` that the walk does not visit (for
  example `0x80A851DC` in rank 1 and `0x80AB07E8` in rank 4), not missing code.
* Every `psq_l`/`psq_st` in the twelve was decoded with opcode 56/60 and its
  quantiser field read: `qr0` everywhere except the six `qr3` data sites.
* No `mtspr GQRn` exists in the run of any of the twelve.
* The jump table of rank 7 was read out of the image at `0x80B38334`
  (31 entries) and each entry disassembles to one of two adjacent instructions,
  which is exactly the export's two-valued `switch`.
* All float and halfword constants quoted above were read from
  `work/rels/elf/d_enemiesNP.elf`; the BSS addresses (`0x80B6EF08`,
  `0x80B769B0..`) return no file bytes, which is why ranks 5 and 12 are
  conditional and rank 11 is not.
