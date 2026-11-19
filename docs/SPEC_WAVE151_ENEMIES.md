# Wave 151 - implementation specification for 12 untranslated `d_enemiesNP` bodies

## What this page is

`docs/ANALYSIS_WAVE127_ENEMIES.md` reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B**, 160 class **C** and 51 class **D**.
Waves 128-150 worked through that block and left the size-ranked tranche at
284 bytes. This page takes the twelve largest bodies that are **still
untranslated**, have a Ghidra export, and are fully walkable from their entry.

A live scan of `work/ghidra/rel-d_enemiesNP/export/functions.tsv` (3,555 rows),
the `address` column of `ledger/functions.csv` (17,042 rows), and every
`0x80Axxxxx`/`0x80Bxxxxx` address in a `^#` heading of any `docs/*.md` leaves
**85** bodies that are absent from the ledger and from every heading and carry
a `decompiled = true` export. One of those 85 (`0x80A10220`, 484 bytes) is
already translated in `src/game/enemies.rs` as `pub unsafe fn fn_80a10220`
without a ledger row, so it is **excluded** below; the remaining **84** are the
real work list. The twelve sections that follow are the twelve largest.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

## Selection

The ranking metric is the **reachable body**, not the raw `functions.tsv` extent,
because the TSV size is a bucket extent that can include a neighbour's padding or
an unreachable arm (see `docs/REL_ELF_MAPPING.md`). Each candidate was walked from
its entry through every conditional branch, every internal `b` and every
**linked** indirect branch (`bctrl`), until all paths ended in `bclr`, a tail
`b` or a tail `bcctr`. Conditional `bclr`/`bcctr` forms were followed on both the
taken and the fall-through edge.

| # | address | TSV size | reachable insns | `qr3` sites | role | verdict |
|---:|---|---:|---:|---:|---|---|
| 1 | `0x80A10880` | 940 | 235 | 0 | segmented-trail actor frame step: side speed, ground probe, sound `0x283`, 29-unit trail emitter | **conditional** |
| 2 | `0x80A969C0` | 844 | 211 | 0 | player/carrier contact test: expanded AABB vs swept segment, per-state counter, block-motion spawn | **conditional** |
| 3 | `0x80A707E0` | 652 | 163 | 0 | chain builder: one `0x278`-byte segment object per slot, linked through `+0x238` | **conditional** |
| 4 | `0x80A8BD20` | 640 | 160 | 0 | `kurage` constructor: `g3d/kurage.brres`, three model holders, three light clips | **ready** |
| 5 | `0x80AAD490` | 484 | 121 | 0 | shot/slide launch: variant-packed id, `0x1BD` sound, `+0xE34` latch | **ready** |
| 6 | `0x80AC0840` | 396 | 99 | 1 | `lift_maruta` per-slot collision response: wrapped phase -> height ramp | **ready** |
| 7 | `0x80A7A8C0` | 320 | 80 | 0 | two-row ground probe, returns the number of hit rows | **ready** |
| 8 | `0x80ADE060` | 320 | 80 | 0 | impact body: single/multi-player effect record at `(x, y-8, z+16)` | **ready** |
| 9 | `0x80A7A460` | 312 | 78 | 0 | segment position writer: 2x3x4 interpolation grid into `+0x6B4 + k*0xA4` | **ready** |
| 10 | `0x80A7AA00` | 292 | 73 | 0 | downward probe fallback for body 7, returns `0/-1/-2` | **ready** |
| 11 | `0x80AEAED0` | 284 | 71 | 0 | signed offset fan: one trail node per table entry, `+0x818` result | **ready** |
| 12 | `0x80AF2400` | 284 | 70 | 0 | `Wm_en_burst_m` cannon placement: colour -> rate, `guide_st` clip | **conditional** |

**Split: 8 ready, 4 conditional.** Total 5,768 bytes of TSV extent and 1,441
reachable instructions. Eleven of the twelve walk to a terminal instruction at
exactly `entry + size - 4`, so `reachable == extent` for them. `0x80AF2400` is
the exception: its extent stops at `0x80AF2518` (`addi r1,r1,48`) and the
`blr` sits at `0x80AF251C`, one word past the extent; the next TSV entry starts
at `0x80AF2520`.

### The exclusion: `0x80A10220`

`0x80A10220` (484 bytes, 121 reachable) is the largest address left by the
wave-150 rule, but `src/game/enemies.rs` already defines
`pub unsafe fn fn_80a10220(this: usize)` (around line 194198) with a full body,
doc comment and "Original: Ghidra `FUN_80a10220`" evidence block. It is
unledgered and unmentioned in any heading, which is exactly why the wave-150
filter would have selected it again. It is **not** a work item and is not
specced here. The next largest after it is `0x80AF2400` (284), which is claimed
as rank 12.

### Rows below the cut

| address | TSV size | reachable | why it is not claimed |
|---|---:|---:|---|
| `0x80A965D0` | 280 | 70 | clean per-slot child activation; `pk`-model holder rebuild; next in size order, left for wave 152 |
| `0x80AC1800` | 276 | 69 | `mecha_koopa` clip binder; already carries a `W145`/`W84` note |
| `0x80B16A80` | 276 | 69 | eye/mouth node offset step; two `strcmp` names |
| `0x80B27DC0` | 272 | 68 | scene-object relay; one callee, three branches |
| `0x80A80F40` | 264 | 66 | one `qr3` site; sway/heading step with 208..768 phase clamp |
| `0x80B1AD30` | 264 | 66 | ground-snap step; final callee `0x800A0A30` uncharacterised |
| `0x80AF5C60` | 260 | 65 | two `qr3` sites; world-position from guide angle |
| `0x80A5E590` | 252 | 63 | `daEnGabon_c` initialiser; second argument of `0x80A5E1A0` unused |

## The `qr3` pool at this wave

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (ST/LD type 5 =
unsigned 16-bit, both scales 0), installed by `main.dol` at
`0x8025EEEC`/`0x8025EEF0`. The RAW stream of this REL contains no `mtspr GQRn`.
Exactly one of the twelve bodies reads `qr3`:

| address | size | `qr3` sites | where |
|---|---|---:|---|
| `0x80AC0840` | 396 | 1 | `psq_l f1,0(r4),3,1` at `0x80AC0940` |

The load address is `r4 = r1 + 8`, where the body has just stored the
halfword phase with `sth r30,8(r1)`. `W = 1` selects the single-element form,
so the instruction returns one float: `f1 = (f32)(u16)phase`, with `GQR3 =
0x00050005` contributing no scale. The very next instruction,
`fmuls f1, f0, f1` at `0x80AC0948`, multiplies that by `0.00390625 = 1/256`
(`0x80B30198`), and the result is the argument of the angle routine
`0x80237C40`. The Ghidra export's `unaff_GQR3` / `ldexpf` block is therefore
cosmetic; the body is fully typable.

No other body in the twelve uses a non-zero quantiser. Every other
`psq_l`/`psq_st` word is a `qr0` `f31`/`f30`/`f29`/`f28` stack spill beside a
matching `stfd`/`lfd`.

## Reading conventions

* Synthetic base is `0x80A00000`. The REL has five file-backed `PT_LOAD`
  segments (`0x80A000F0`, `0x80B2A518`, `0x80B2A7A4`, `0x80B2A7B0`,
  `0x80B32DC0`); `docs/REL_ELF_MAPPING.md`'s `+0x1000` formula applies to the
  first. The file-backed image ends at `0x80B6E438`.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte offsets
  from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z; `+0x100`/`+0x102`/`+0x104`
  are the three `s16` angles; `+0x348` is the side/facing byte; `+0x38C` is the
  actor kind byte, `+0x38D` the effect kind byte and `+0x38F` the ground kind
  byte.
* **`main.dol` data.** A byte table and several manager pointers live below
  `0x80A00000`; how to read them is given per body. `work/dol/main.elf` maps
  them (`0x8042B080..0x8042FBE0` is a file-backed `main.dol` segment).
* **CodeWarrior prologues.** `0x802DCD54/68/70/74` are `_savegpr_27..28`
  helpers and `0x802DCDA0/B4/C0` their `_restgpr` partners; they are not
  argument-destroying stubs and their incoming `r3..r10` survive. A body whose
  only "calls" to those addresses is the prologue/epilogue is call-free.
  Ghidra often prints `iVar = func_0x802dcd68()` and then reads `this` from the
  high half of the 64-bit result; that is the saved `r3`, not a second return
  value. The stream's `r3` is the actor.
* Sizes are the `size` column of `functions.tsv`; "reachable" is the count of
  distinct instruction words on a path from the entry.

---

## 1. `0x80A10880` - segmented-trail frame step

| field | value |
|---|---|
| TSV size | 940 bytes |
| reachable | 235 instructions, ends at `0x80A10C28` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a10880_FUN_80a10880.c` |
| signature | `void fn_80a10880(usize this)` |
| verdict | **conditional** |

### What the stream does

```c
// r31 = 0x80B2AD90 (float pool), r28 = this
v = *(u32*)(this + 0x544);                    // sub-object's vtable
i32 r = ((i32 (*)(u32, u32))*(u32*)(v + 0x2C))(this + 0x544, 0);

side = *(u8*)(this + 0x348);
f = f32_table_80B35230[side];                 // { 1.5, -1.5 }
*(f32*)(this + 0xE8) = f;
if (r == 0)                                   // slot +0x2C's return value
    *(f32*)(this + 0xE8) = f * 0.5f;

FN_8006CDE0(this);
FN_800955F0(this);
if (FN_80072BC0(this + 0x1EC) != 0)
    *(f32*)(this + 0xEC) = 0.0f;
if (FN_8006FA40(this + 0x1EC, 0) != 0)
    *(u8*)(this + 0x348) ^= 1;

*(u32*)(this + 0x5C4) = *(u32*)(this + 0xAC); // current -> previous
*(u32*)(this + 0x5C8) = *(u32*)(this + 0xB0);
*(u32*)(this + 0x5CC) = *(u32*)(this + 0xB4);

FN_80A10410(this);                            // ring shift, already translated

ring = *(u8***)(this + 0x92C);
ang  = (s16)FN_801611C0(*(f32*)(ring + 4)  - *(f32*)(ring + 0x20),   // dy
                        *(f32*)(ring + 0)  - *(f32*)(ring + 0x1C));  // dx, 1/65536 turn
if (*(u8*)(this + 0x348) == 1)
    ang = (s16)(-0x8000 - ang);
FN_800B7510(this + 0x544, ang);

*(u8*)(this + 0x934) = *(u8*)(this + 0x935);
f32 out = 0.0f;
if (FN_80075270(*(f32*)(this + 0xAC), *(f32*)(this + 0xB0) + 20.0f,
                *(u8*)(this + 0x38F), &out) == 0) {
    *(u8*)(this + 0x935) = 0;
    FN_80075560(*(f32*)(this + 0xAC), *(f32*)(this + 0xB0) + 20.0f,
                *(u8*)(this + 0x38F), &out);
} else {
    *(u8*)(this + 0x935) = 1;
}

if (*(u8*)(this + 0x935) != *(u8*)(this + 0x934) && out < 0.0f) {
    u32   pos[3]  = { bits(*(f32*)(this + 0xAC)), bits(out), bits(6500.0f) };
    u32   one[3]  = { bits(1.0f), bits(1.0f), bits(1.0f) };
    FN_800929F0(*(u32*)0x80429EE8, pos, (*(u8*)(this + 0x38F) << 16) | 6, -1, one);
    FN_8006A4B0(local, this + 0xAC);
    FN_80198C30(*(u32*)0x80429D60, 0x283, local, 0);
    FN_80078410(*(f32*)(this + 0xAC), out, *(u32*)0x80429DD0, 6);
}

FN_801C14C0(this + 0xE8);

for (u32 i = 0; i < *(u32*)(this + 0x540); i++) {
    i32   count = *(i32*)(this + 0x928);
    f32   target = 29.0f * (f32)(i + 1);
    u8   *ring = *(u8**)(this + 0x92C);
    u8   *node = *(u8**)(this + 0x77C);       // chain head; next is +0x238
    f32   acc = 0.0f;
    for (i32 k = 0; k < count; k++) {
        f32 seg_len = acc + *(f32*)(ring + k * 0x1C + 0x10);
        if (target < seg_len) {
            u8 *cur = ring + k * 0x1C, *prev = cur - 0x1C;
            f32 t  = (target - acc) / (seg_len - acc);
            struct { f32 x, y; s16 a0, a1, a2; u16 pad; u32 w; u16 h; } rec;
            rec.x  = *(f32*)(prev + 0) + (*(f32*)(cur + 0) - *(f32*)(prev + 0)) * t;
            rec.y  = *(f32*)(prev + 4) + (*(f32*)(cur + 4) - *(f32*)(prev + 4)) * t;
            rec.a0 = *(s16*)(prev + 8);
            rec.a1 = *(s16*)(prev + 10);
            rec.a2 = *(s16*)(prev + 12);
            rec.w  = *(u32*)(cur - 8);
            rec.h  = *(u16*)(cur - 4);
            (*(void (**)(u8*, void*))(*(u32*)node + 0x2C))(node, &rec);
            break;
        }
        acc = seg_len;
    }
    node = *(u8**)(node + 0x238);
}
```

Notes that matter for a faithful port:

* The chain node walk starts at `+0x77C` and follows `+0x238`; it is advanced
  once per `i`, whether or not the inner loop emitted.
* The `k == 0` arm computes `prev = ring - 0x1C`, i.e. one record *before* the
  block. The ring is a `0x2BD0`-byte allocation made by the translated
  `fn_80a10170`, so the original reads a live word there; a port must either
  reproduce the pointer arithmetic or clamp `k == 0` to `prev = cur`.
* The `29.0f` step is `0x80B2AD9C`, and `+0x10` of a ring record is the
  segment length written by `fn_80a10410` (the `fn_802BE470` result).

### Constants

| address | value | use |
|---|---|---|
| `0x80B2AD90` | `0.0f` | zero; `+0xEC`/`out` seed |
| `0x80B2AD9C` | `29.0f` | trail emitter spacing |
| `0x80B2AE58` | `0.5f` | side-speed scale when the vtable slot returns zero |
| `0x80B2AE5C` | `20.0f` | probe height above `+0xB0` |
| `0x80B2AE60` | `6500.0f` | `pos[2]` of the motion block |
| `0x80B2AE64` | `1.0f` | motion-block scale triple `{1,1,1}` |
| `0x80B2AE68` | `0x4330000080000000` | `2^52` unsigned-int-to-double bias |
| `0x80B35230` | `1.5f`, `-1.5f` | per-side value loaded into `+0xE8` |
| `0x80429EE8` | pointer | scene/motion manager |
| `0x80429D60` | pointer | sound manager |
| `0x80429DD0` | pointer | effect owner for `0x80078410` |

### Verdict: conditional

Two vtable dispatches cannot be named from this module:
`0x80A108C4` calls slot `+0x2C` of the class behind `this+0x544`, and
`0x80A10BDC` calls slot `+0x2C` of each chain node behind `this+0x77C`.
Everything else - the probe, the sound, the motion block, the 29-unit
interpolation - is literal. Step needed: name the `+0x544` sub-object and the
`+0x77C` chain-node classes (their `+0x2C` slot is an "emit trail point"
method).

---

## 2. `0x80A969C0` - carrier contact test

| field | value |
|---|---|
| TSV size | 844 bytes |
| reachable | 211 instructions, ends at `0x80A96D08` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a969c0_FUN_80a969c0.c` |
| signature | `u32 fn_80a969c0(usize this, i32 row, const f32 *extent)` - always returns `0` |
| verdict | **conditional** |

### What the stream does

```c
if (*(u8*)(*(u32*)0x8042A1C8 + 0x120D) - 0x20 > 2) {   // mode not 0x20/0x21/0x22
    f32 ex = extent[0], ey = extent[1], pz = *(f32*)(this + 0xB4);
    f32 a[3] = { *(f32*)(this + 0xAC) - ex, *(f32*)(this + 0xB0) - ey, pz }; // min
    f32 b[3] = { *(f32*)(this + 0xAC) + ex, *(f32*)(this + 0xB0) + ey, pz }; // max

    u32 state = *(u32*)(this + 0x128);                 // 0..3 slot, 0xFFFFFFFF = none
    if (state < 4) {
        u8 *flags = *(u8**)0x80429F7C;                 // per-state 0x16-byte rows
        if (flags[state * 0x16 + row + 0xAFE] != 0)
            *(u32*)(this + 0x128) = (u32)(flags[state * 0x16 + row + 0xAFE] - 1);
    }

    if (*(u32*)(this + 0x128) < 4) {
        p = FN_8005F900(*(u32*)(this + 0x128));        // player by slot
        if (p != 0) {
            ride = FN_80147B60(p);
            f32 v[4];
            FN_80141F70(p, v);
            f32 sy = v[1] + *(f32*)(p + 0xB0);
            f32 sx = v[0] + *(f32*)(p + 0xAC);
            f32 sz = *(f32*)(p + 0xB4);
            f32 sa = FN_80101AA0(sx);                  // angle from x
            f32 t[3] = { sa - v[2] - 2.0f, sy - v[3], sz };
            f32 u[3] = { sa + v[2] + 2.0f, sy + v[3], sz };

            if (*(u8*)(*(u32*)0x8042A320 + state * 0x23C + 0x381) == 0)
                FN_800B3600(state, row);

            if (FN_800B3100(0.0f, a, b, t, u) != 0 && ride != 0) {
                *(u16*)(this + 0x634) = 0;
            } else if (*(u16*)(this + 0x634) < 0x3D) {
                *(u16*)(this + 0x634) += 1;
            } else {
                FN_800B3750(state, row, 0);
                *(u32*)(this + 0x128) = 0xFFFFFFFF;
            }
        }
    } else {
        a2 = FN_80064740(this, local);                 // nearby actor from the box
        if (a2 != 0 && *(u8*)(a2 + 0x38C) == 1 &&
            (pc = (*(u8 *(*)(u32))(*(u32*)(a2 + 0x60) + 0x6C))(a2),
             *(u8*)(*(u8**)0x80429F7C + *pc * 0x16 + row + 0xAFE) == 0)) {
            ride = FN_80147B60(a2);
            FN_80141F70(a2, v);
            /* same expanded box and swept segment as above, from a2 */
            if (ride != 0 && FN_800B3100(0.0f, a, b, t, u) != 0) {
                pc = (*(u8 *(*)(u32))(*(u32*)(a2 + 0x60) + 0x6C))(a2);
                *(u32*)(this + 0x128) = (u32)(s8)*pc;  // adopt the state
            }
        }
    }
}
return 0;
```

The two branches are the same computation from two sources: `+0x128 < 4`
resolves the actor through `0x8005F900(slot)` and drives the `+0x634` counter;
`+0x128 > 3` finds an adjacent actor through `0x80064740` and copies its state
byte into `+0x128` when the swept segment hits.

`FN_800B3100(0.0f, min, max, t, u)` is a segment-versus-AABB test
(`0.0f` is `0x80B2ECD4`); `FN_800B3600`/`FN_800B3750` are the stage-side
enter/leave notifications; `0x800929F0` is `game::stage::stage_block_spawn_motion`
(it does not appear in this body, only in body 1).

### Constants

| address | value | use |
|---|---|---|
| `0x8042A1C8` | pointer | global mode object; `+0x120D` is the game-mode byte |
| `0x80429F7C` | pointer | per-state flag table, `0x16`-byte rows, +0xAFE column |
| `0x8042A320` | pointer | per-state `0x23C`-byte block; `+0x381` gate byte |
| `0x80B2ECD4` | `0.0f` | last argument of `0x800B3100` |
| `0x80B2ED44` | `2.0f` | half-width of the swept segment in x |

### Verdict: conditional

Two indirect calls at `0x80A96BE8` and `0x80A96CC8` dispatch slot `+0x6C` of
the class behind `*(u32*)(actor+0x60)`; the slot returns a state object whose
first byte is the state id. Step needed: name that class/slot (the caller
`0x80A977C0` passes `0x12` as `row`). The rest - the AABB expansion, the
`0x16`-byte flag table, the `0x23C`-byte state block and the `0x3D` counter
limit - is literal.

---

## 3. `0x80A707E0` - chain segment builder

| field | value |
|---|---|
| TSV size | 652 bytes |
| reachable | 163 instructions, ends at `0x80A70A68` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a707e0_FUN_80a707e0.c` |
| signature | `void fn_80a707e0(usize this, u16 value)` - the `value` arrives in `r5` |
| verdict | **conditional** |

### What the stream does

```c
side = *(u8*)(this + 0x348);
*(u16*)(0x80B2DAE4 + side * 2) = value;        // per-side stored halfword (global, REL data)
u16 h = *(u16*)(0x80B2DAE4 + side * 2);        // read back, halved then doubled
s16 a6 = (s16)((s16)(h / 2) * 2);              // round-to-even

f32 step = (f32)(s8)0x8042C1A0[side ^ 1] * 10.4f;   // main.dol byte table: +1 / -1 -> +-10.4

/* two-point ground record passed to the stage */
struct { s16 a8, a6, a4; } ang = { 0, a6, 0 };
FN_800B6830(10.4f, this + 0x544, this + 0xAC, &ang);

f32 x = *(f32*)(this + 0x5C4);                 // x accumulator
f32 y = *(f32*)(this + 0x5C8);
f32 z = *(f32*)(this + 0x5CC);
u32 *link = (u32*)(this + 0x544);              // first link slot

for (i32 i = 0; i < *(i32*)(this + 0x540); i++) {
    x += step;                                 // one segment per slot, +-10.4
    obj = FN_802B9210(0x278);                  // 632-byte allocation
    if (obj != 0) {
        obj->vtable   = 0x803158D8;            // temporary, replaced below
        obj->field4   = 0;
        FN_80169CD0(obj + 0x08);               // node holder (float triple + angles)
        FN_80168D10(obj + 0x48);
        obj->vtbl48   = 0x80B330E0;
        *(f32*)(obj + 0x80) = x;
        *(f32*)(obj + 0x84) = y;
        *(f32*)(obj + 0x88) = z;
        *(u16*)(obj + 0x8C) = 0;
        *(s16*)(obj + 0x8E) = a6;
        *(u16*)(obj + 0x90) = 0;
        FN_8008C200(obj + 0xC4);               // collision area 1
        FN_8008C200(obj + 0x168);              // collision area 2
        *(u32*)(obj + 0x224) = this;           // parent
        *(u32*)(obj + 0x228) = 1;
        *(u16*)(obj + 0x230) = 0;
        *(f32*)(obj + 0x234) = 0.0f;
        *(u32*)(obj + 0x238) = 0;              // next link
        FN_800B6830(10.0f, obj, obj + 0x80, obj + 0x8C);
        obj->vtable = 0x80315840;              // final class vtable
        *(u32*)(obj + 0x23C) = 0;
        *(u32*)(obj + 0x240) = 0x803295D8;     // temporary, replaced below
        *(u32*)(obj + 0x244) = 0;
        *(u32*)(obj + 0x248) = 0;
        FN_8016A5E0(obj + 0x24C);
        *(u32*)(obj + 0x240) = 0x80329548;     // final sub-vtable
        *(u32*)(obj + 0x268) = 0;
        *(f32*)(obj + 0x26C) = -1.0f;
        *(u32*)(obj + 0x270) = 0;
        *(u8*)(obj + 0x274) = 0;
        *(u8*)(obj + 0x275) = 0;
        *(u16*)(obj + 0x276) = 0;
    }
    *link = obj;                               // previous link points at the new object
    link  = obj + 0x238;                       // next link slot
}
```

`this+0x544` is the head link slot, so the chain is `this+0x544 -> obj0 -> obj1
-> ...` through `+0x238`. `0x278` is the object size the wave-14 note records
("allocates one `0x278`-byte object per missing segment").

### Constants

| address | value | use |
|---|---|---|
| `0x80B2DAD8` | `0.0f` | `+0x234` seed |
| `0x80B2DB50` | `10.4f` | ground-record height and per-slot step base |
| `0x80B2DB70` | `-1.0f` | `+0x26C` |
| `0x80B2DB78` | `2^52` double | int-to-double bias |
| `0x80B2DAE4` | `u16[2]` | per-side stored halfword (written then read) |
| `0x8042C1A0` | `01 FF 00 00` (`main.dol`) | side direction: `+1` / `-1` |
| `0x803158D8`, `0x80315840` | vtables | chain-segment class (temporary then final) |
| `0x80B330E0` | vtable | `+0x48` sub-object class |
| `0x803295D8`, `0x80329548` | vtables | `+0x240` sub-object class |

### Verdict: conditional

The body is fully literal and call-free apart from resolved constructors, but
the only caller, `0x80A70770`, is `FUN_80a707e0(param_1)` - it passes **one**
argument, so the `u16` that this body stores into `0x80B2DAE4[side]` is
whatever `0x8016A960` left in `r5`. Step needed: decide the value the port
should pass (the original has no defined third argument on this call path);
everything else can be written today.

---

## 4. `0x80A8BD20` - `kurage` constructor

| field | value |
|---|---|
| TSV size | 640 bytes |
| reachable | 160 instructions, ends at `0x80A8BF9C` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a8bd20_FUN_80a8bd20.c` |
| signature | `void fn_80a8bd20(usize this, u32 parent)` |
| verdict | **ready** |

### What the stream does

```c
res = FN_800DF180(*(u32*)0x8042A038 + 4, "kurage", "g3d/kurage.brres");
*(u32*)(this + 0x1C) = res;                        // resource holder

n0 = FN_80239E30(this + 0x1C, "kurage_light_mask");
FN_80169D90(this + 0x20, &n0, parent, 0x324, 1, 0);   // model holder, 804 bytes
n1 = FN_80239E30(this + 0x1C, "kurage_light_display");
FN_80169D90(this + 0x60, &n1, parent, 0x224, 1, 0);   // 548 bytes
n2 = FN_80239E30(this + 0x1C, "kurage_light_fog");
FN_80169D90(this + 0xA0, &n2, parent, 0x224, 1, 0);   // 548 bytes
FN_8016A2A0(this + 0x20, 0xFFFFFFFF, 0x88);

a0 = FN_8023A270(this + 0x1C, "kurage_light_mask");   // srt animation
*(u32*)(this + 0xE0) = a0;
FN_80167420(this + 0xE4, &n0, &a0, parent, 0, 1);     // blend block 1
FN_80167800(this + 0xE4, this + 0x20, &a0, 0, 1);
FN_80169F80(1.0f, this + 0x20, this + 0xE4);
FN_80167B40(0.0f, this + 0xE4, 0);

a1 = FN_8023A190(this + 0x1C, "kurage_light_fog");    // colour animation
FN_80165C30(this + 0x13C, &n2, &a1, parent, 0, 1);
FN_80165FD0(this + 0x13C, this + 0xA0, &a1, 0, 0);
FN_80166310(1.0f, this + 0x13C, 0);
(*(void (**)(u32, u32))(*(u32*)(this + 0xA0) + 0x18))(this + 0xA0, this + 0x13C);

a2 = FN_8023A270(this + 0x1C, "kurage_light_display_angle");
FN_80167420(this + 0x110, &n1, &a2, parent, 0, 2);
FN_80167800(this + 0x110, this + 0x60, &a2, 0, 1);
FN_80167B40(0.0f, this + 0x110, 0);
FN_8023A270(this + 0x1C, "kurage_light_display_noise");  // result unused
FN_80169F80(1.0f, this + 0x60, this + 0x110);

FN_80A8C1C0(this);                                     // already translated sibling
*(u32*)(this + 0x190) = 0xFF;
*(u8*)(this + 0x194) = 1;
```

The strings are contiguous in REL data at `0x80B4F570`:
`+0x0C` `"g3d/kurage.brres"`, `+0x20` `"kurage"`, `+0x28`
`"kurage_light_mask"`, `+0x3C` `"kurage_light_display"`, `+0x54`
`"kurage_light_fog"`, `+0x68` `"kurage_light_display_angle"`, `+0x84`
`"kurage_light_display_noise"`.

Field map (matches the wave-26 note in `src/game/enemies.rs`):

| offset | meaning |
|---|---|
| `+0x01C` | resource holder for `g3d/kurage.brres` |
| `+0x020` | model holder, `0x324` bytes |
| `+0x060` | model holder, `0x224` bytes |
| `+0x0A0` | model holder, `0x224` bytes |
| `+0x0E0` | srt animation for the mask clip |
| `+0x0E4` | srt blend block (node `+0x020`) |
| `+0x110` | srt blend block (node `+0x060`) |
| `+0x13C` | colour blend block (node `+0x0A0`) |
| `+0x190` | `0xFF` |
| `+0x194` | `1` |

### Constants

| address | value | use |
|---|---|---|
| `0x80B2E768` | `0.0f` | `0x80167B40` rate |
| `0x80B2E780` | `1.0f` | `0x80169F80` / `0x80166310` weight |
| `0x8042A038` | pointer | BRRES archive manager (`+4` is the archive) |
| `0x80B4F570` | string block | clip and node names above |

### Verdict: ready

Every call, name, size and store is literal. The one indirect call at
`0x80A8BEF0` is slot `+0x18` of the model holder this body has just built at
`+0x0A0`; the receiver is created here, so the slot is fixed by the resource
module and does not need new evidence. The body can be written now.

---

## 5. `0x80AAD490` - shot/slide launch

| field | value |
|---|---|
| TSV size | 484 bytes |
| reachable | 121 instructions, ends at `0x80AAD670` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80aad490_FUN_80aad490.c` |
| signature | `u32 fn_80aad490(usize this)` - returns `1` on success, `0` on every early exit |
| verdict | **ready** |

### What the stream does

```c
if (FN_80AACD50(this) == 0) return 0;
if (*(u32*)(this + 0xE30) != 0) return 0;
slot = FN_80AACE90(this);
if (slot == 0xFFFFFFFF) return 0;

f32 pos[3];
pos[0] = *(f32*)(this + 0xAC);
pos[1] = *(f32*)(this + 0xB0) + f32_table_80B2F8E8[*(i32*)(this + 0xE18)]
                      + (*(f32*)(this + 0xE04) - 32.0f);
pos[2] = *(f32*)(this + 0xB4) - 16.0f;

i32 variant; u32 left;
if (*(i32*)(this + 0xE24) == 0) { variant = 0;    left = FN_80096240(this, this + 0xAC) & 0xFF; }
else if (*(i32*)(this + 0xE24) == 1) { variant = 0x12; left = 1; }
else { variant = 2; left = 0; }

u8 zero = 0;
u32 *r = FN_80AAA360(pos, variant | (slot << 8), &zero, 0x14);
if (r == 0) return 0;

*(u32*)(this + 0xB44 + slot * 4) = *r;
u32 snd = *(u32*)0x80429D60;
FN_8006A4B0(local, this + 0xAC);
FN_80198C30(snd, 0x1BD, local, 0);

pos[0] = (left != 0) ? *(f32*)(this + 0xAC) - 8.0f : 8.0f + *(f32*)(this + 0xAC);
pos[2] = 3000.0f;
FN_80094490(pos);
*(u16*)(this + 0xE34) = 1;
FN_80168FE0(0.0f, this + 0xA0C);
return 1;
```

### Constants

| address | value | use |
|---|---|---|
| `0x80B2F89C` | `32.0f` | subtracted from `+0xE04` |
| `0x80B2F8A4` | `16.0f` | z offset |
| `0x80B2F8B0` | `8.0f` | x offset for the follow-up probe |
| `0x80B2F8B4` | `0.0f` | `0x80168FE0` rate |
| `0x80B2F8FC` | `3000.0f` | follow-up probe z |
| `0x80B2F8E8` | `48, 40, 24, 24, ...` | per-`+0xE18` height table |
| `0x80B2F900` | `2^52` double | slot -> float bias |
| `0x80429D60` | pointer | sound manager |

### Verdict: ready

All three sibling calls (`0x80AACD50`, `0x80AACE90`, `0x80AAA360`) and the
three `main.dol` calls are explicit with their argument values; there is no
indirect call, no jump table and no unresolved register. The variant/slot
packing `variant | slot << 8` and the `+0xB44 + slot*4` result array are
literal.

---

## 6. `0x80AC0840` - `lift_maruta` per-slot collision response

| field | value |
|---|---|
| TSV size | 396 bytes |
| reachable | 99 instructions, ends at `0x80AC09C8` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ac0840_FUN_80ac0840.c` |
| signature | `void fn_80ac0840(usize this)` - `this` is a `0xC`-byte slot record |
| verdict | **ready** |

### What the stream does

```c
o1 = FN_80162D00(*(u32*)(this + 4));     // resolve slot's stored word
o2 = FN_80162D00(*(u32*)(this + 0));     // resolve its partner

u16 a = o1->0x100;                       // u16 counter on the first object
u16 b = *(u16*)(this + 8);
u16 phase = (o1->0x100 >= b) ? (u16)(a - b) : (u16)(a - 1 - b);   // wrapped delta

if (o2 == 0) {
    *(u32*)(this + 0) = 0;
} else if (fabs(o2->x - o1->x) > 0.5f * 304.0f) {
    if (o2->0x38C != 1 && o2->0x38C != 2) o2->0xEC = 0.0f;
    *(u32*)(this + 0) = 0;
} else if (o2->0xEC > 0.0f) {
    *(u32*)(this + 0) = 0;
} else {
    f32 angle = (f32)phase * 0.00390625f;              // qr3 load, single lane
    f32 h = 32.0f * FN_80237C40(angle);
    if (o2->0x38C == 1 || o2->0x38C == 2)
        FN_80057D30(o2);                               // snap/stand
    else
        o2->y = o1->y + h;
    if (phase > 0x4000) {
        if (o2->0x38C != 1 && o2->0x38C != 2) o2->0xEC = 0.0f;
        *(u32*)(this + 0) = 0;
    }
}
```

The `psq_l` is `0x80AC0940`, `psq_l f1,0(r4),3,1` with `r4 = r1 + 8` and the
halfword stored there at `0x80AC0930`. With `GQR3 = 0x00050005`, `W = 1`, the
result is exactly `(f32)(u16)phase`; the following `fmuls f1, f0, f1` applies
`1/256`.

`0x80237C40` takes the 1/256-turn angle and returns the unit height
(the same routine is used as "angle -> offset" by the trail actor); `0x32`
scales it to the actor's height range.

### Constants

| address | value | use |
|---|---|---|
| `0x80B30190` | `0.0f` | `+0xEC` clear |
| `0x80B30194` | `0.5f` | half of the width limit |
| `0x80B30198` | `0.00390625f` | `1/256` turn scale |
| `0x80B3019C` | `32.0f` | height scale |
| `0x80B301A0` | `304.0f` | x-distance limit |

### Verdict: ready

No indirect call, no jump table, one `qr3` site whose quantiser is pinned by
`docs/REL_ELF_MAPPING.md`. The earlier note that `0x80AC0840` "still carries
`unaff_GQR3`" is a Ghidra artefact only; the stream types cleanly.

---

## 7. `0x80A7A8C0` - upward row probe

| field | value |
|---|---|
| TSV size | 320 bytes |
| reachable | 80 instructions, ends at `0x80A7A9FC` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a7a8c0_FUN_80a7a8c0.c` |
| signature | `i32 fn_80a7a8c0(usize this, const f32 *pair)` - `pair = {offset, y}` |
| verdict | **ready** |

### What the stream does

```c
f32 y0 = 8.0f + pair[1];
i32 rows = 0;

for (i32 row = 0; row < 2; row++) {
    f32 x0 = pair[0] - 4.0f;                   // reloaded at 0x80A7A92C
    f32 probe = 0.0f;
    i32 clamped = 0, hit = 0;                  // both reset per row
    for (;;) {
        if (clamped) break;                    // `fcmpo probe,8; cror 2,1,2; bne`
        if (probe < 8.0f) { probe = 8.0f; clamped = 1; }
        u32 r = FN_80070BA0(*(u8*)(this + 0x38F), x0 + probe, y0);
        if (r != 0 && r != 2 && r != 0x2000) { hit = 1; break; }
        probe += 16.0f;
    }
    if (!hit) return rows;                     // rows = 0 or 1
    rows += 1;                                 // a hit continues with the next row
    y0 += 16.0f;
    if (rows >= 2) break;
}
return rows;
```

In practice the inner `for` runs once per row: `probe` starts at `0`, is
clamped to `8.0f` on that first pass and `clamped` then terminates the loop.
So the body probes `x0 + 8.0f = pair[0] + 4.0f` at `y0` and, on a hit,
`pair[0] + 4.0f` at `y0 + 16.0f`. The return value is the number of
consecutive rows that reported ground (`0`, `1` or `2`), which is what the
translated caller `fn_80a7ab90` tests with `< 1`.

The `cror 2,1,2` at `0x80A7A948` is the CodeWarrior `!(a < b)` idiom
(`gt | eq` into bit 2); it is a plain instruction and does not need special
handling.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2DE20` | `0.0f` | probe seed |
| `0x80B2E050` | `4.0f` | x offset |
| `0x80B2E0B8` | `8.0f` | first probe offset / clamp |
| `0x80B2E0BC` | `16.0f` | row and step spacing |

### Verdict: ready

No indirect calls; the single callee `0x80070BA0` is `(kind, x, y) -> flags`
and every argument is explicit. The accepted flag set (`!= 0`, `!= 2`,
`!= 0x2000`) is literal.

---

## 8. `0x80ADE060` - impact effect record

| field | value |
|---|---|
| TSV size | 320 bytes |
| reachable | 80 instructions, ends at `0x80ADE19C` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80ade060_FUN_80ade060.c` |
| signature | `void fn_80ade060(usize this, i8 mode)` |
| verdict | **ready** |

### What the stream does

```c
f32 pos[3] = { *(f32*)(this + 0xAC),
               *(f32*)(this + 0xB0) - 8.0f,
               *(f32*)(this + 0xB4) + 16.0f };
u32 n   = FN_8005FEF0();                        // player count / state
u32 idx = *(u32*)(this + 0x878);                // effect selector, 0..7
u32 id  = ((u32*)0x80B30AF0)[idx];              // { 0x11, 1, 2, 3, 4, 5, 6, 7 }
*(u8*)(this + 0x68D) = (u8)(mode + 2);

if (idx == 1 || n < 2) {
    /* single-player record (24 bytes at r1+48) */
    u8 rec[24] = { pos, *(u8*)(this + 0x38D), 0, 0, 0, id, 0, 0, 0 };
    FN_80022810(this, rec, 0, id, 0);
} else {
    /* multi-player record (24 bytes at r1+24) */
    u8 rec[24] = { pos, *(u8*)(this + 0x38D), 0, 0, 0, id, 1, 1, 0 };
    FN_80022B10(this, rec, n - 1, 0, 1);
}
```

The record layout is: `f32 x, y, z` at `+0`, `u8 kind` at `+12` (from
`this+0x38D`), three zero bytes, `u32 id` at `+16`, then three flag bytes.
`id` is read from the REL table at `0x80B30AF0` (`r31 + 0x10`, `r31 =
0x80B30AE0`) indexed by `this+0x878`; the table is
`{0x11, 1, 2, 3, 4, 5, 6, 7}`. The multi-player branch passes
`r5 = n - 1`, `r6 = 0`, `r7 = 1`; the single-player branch passes `r5 = 0`,
`r6 = id`, `r7 = 0`.

The `0x8005FEF0()` call takes no visible argument; Ghidra prints it bare too,
and the result is only compared with `2`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B30B3C` | `16.0f` | z offset |
| `0x80B30B44` | `8.0f` | y offset |
| `0x80B30AF0` | `{0x11, 1, 2, 3, 4, 5, 6, 7}` | effect id table indexed by `+0x878` |

### Verdict: ready

No indirect call and no unknown register. `0x80022810` and `0x80022B10` are
outside this REL and uncharacterised beyond their argument shape, but both call
sites are literal (`this`, record pointer, plus the three scalar flags), which
is the same evidence bar used for `0x800A0A30` in
`docs/SPEC_WAVE150_ENEMIES.md`.

---

## 9. `0x80A7A460` - segment position writer

| field | value |
|---|---|
| TSV size | 312 bytes |
| reachable | 78 instructions, ends at `0x80A7A594` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a7a460_FUN_80a7a460.c` |
| signature | `void fn_80a7a460(usize this)` |
| verdict | **ready** |

### What the stream does

```c
u32 *counts = (u32*)0x80B2E09C;         // { 4, 2, 2 }, end 0x80B2E0A8
u8  *band   = (u8*)(this + 0x628);      // two 0x30-byte bands
u8  *cur    = (u8*)(this + 0x6B4);      // record cursor

do {
    u32 *p   = counts;
    f32 *row = (f32*)band;
    u8  *end = cur + 0x334;             // this + 0x9E8 on the first pass
    do {
        u32 n = *p;                     // 4, 2, 2
        u32 k = 1;
        u8  *at = cur;
        do {
            cur = at + 0xA4;
            f32 a = (f32)(n - k) / (f32)n;
            f32 b = (f32)k / (f32)n;
            *(f32*)(at + 0x1C) = a * row[0] + b * row[3] - *(f32*)(this + 0xAC);
            *(f32*)(at + 0x20) = a * row[1] + b * row[4] - *(f32*)(this + 0xB0);
            if (cur == end) break;
            k++;
            at = cur;
        } while (k <= n);
    } while (cur != end && (++p, row += 3, p < counts + 3));
    band += 0x30;
    cur   = end;                        // the next band starts where this one ended
    if (band >= (u8*)(this + 0x688)) return;
} while (1);
```

Read from the raw stream, the loop bounds are:

* outer band loop: `band` starts at `this+0x628`, advances by `0x30`, exits
  once `band >= this+0x688` (two band starts: `+0x628`, `+0x658`);
* middle loop: the count table at `0x80B2E09C` has three entries
  (`4, 2, 2`) and ends at `0x80B2E0A8`, so `p` runs over three entries, with
  `row` advancing 12 bytes per entry (three floats);
* inner loop: `k` from `1` to `n` inclusive, one `0xA4` stride each time.

The grid point written is the linear blend
`((n-k)/n) * row[0..1] + (k/n) * row[3..4]` minus the actor's current
`(x, y)`; it is a *relative* offset, which is why the actor's position is
subtracted. The record cursor walks upward in `0xA4` steps to the current
`end`, is reloaded to that `end` for the second band, and the function returns
as soon as the second band start reaches `this+0x688`.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2E09C` | `4, 2, 2` | per-table sample counts |
| `0x80B2E0A8` | `0x4330000080000000` | `2^52` bias for the `(f32)n` conversion |

### Verdict: ready

The body is call-free (only `_savegpr`/`_restgpr`), and every constant, stride
and bound is explicit. The loop shape is unusual but mechanical; the Ghidra
export prints the same nested `do/while` structure.

---

## 10. `0x80A7AA00` - downward row probe

| field | value |
|---|---|
| TSV size | 292 bytes |
| reachable | 73 instructions, ends at `0x80A7AB20` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80a7aa00_FUN_80a7aa00.c` |
| signature | `i32 fn_80a7aa00(usize this, const f32 *pair)` |
| verdict | **ready** |

### What the stream does

```c
f32 y0 = pair[1] - 8.0f;
f32 x0 = pair[0] - 4.0f;
i32 row = 0;

for (;;) {
    f32 probe = 0.0f;
    i32 clamped = 0;
    for (;;) {
        if (clamped) break;
        if (probe < 8.0f) { probe = 8.0f; clamped = 1; }
        u32 r = FN_80070BA0(*(u8*)(this + 0x38F), x0 + probe, y0);
        if ((r & 0x2000) == 0 && (r & 0x801D) != 0)
            return row;                   // 0 or -1
        probe += 16.0f;
    }
    row -= 1;
    y0  -= 16.0f;
    if (row <= -2) return row;            // -2 when both rows are empty
}
```

This is the fallback for body 7 (`fn_80a7ab90` calls it when
`fn_80a7a8c0` returns less than one). It probes the same
`pair[0] + 4.0f` column downward from `pair[1] - 8.0f` in `16.0f` steps and
returns the row index of the first hit: `0`, `-1`, or `-2` for no hit.

### Constants

| address | value | use |
|---|---|---|
| `0x80B2DE20` | `0.0f` | probe seed |
| `0x80B2E050` | `4.0f` | x offset |
| `0x80B2E0B8` | `8.0f` | first probe offset / clamp |
| `0x80B2E0BC` | `16.0f` | row and step spacing |

### Verdict: ready

Identical evidence to body 7: one explicit callee, no indirect branch, no
unknown register. The accepted flag mask is `(r & 0x801D) != 0` with
`(r & 0x2000) == 0`.

---

## 11. `0x80AEAED0` - offset fan emitter

| field | value |
|---|---|
| TSV size | 284 bytes |
| reachable | 71 instructions, ends at `0x80AEAFE8` (`blr`) |
| export | `work/ghidra/rel-d_enemiesNP/export/80aeaed0_FUN_80aeaed0.c` |
| signature | `void fn_80aeaed0(usize this)` |
| verdict | **ready** |

### What the stream does

```c
i32 id = *(i32*)(this + 0x814);
f32 *off = (f32*)0x80B31158;      // 0, +32, -32, +64, -64, ... +256, -256
u32  i = 0;
while (id >= 0) {
    if (i & 1) id -= 1;
    f32 pos[3];
    pos[2] = *(f32*)(this + 0xB4);
    pos[0] = (*(i32*)(this + 0x81C) == 0)
               ? *(f32*)(this + 0xAC) - fabsf(*off)
               : *(f32*)(this + 0xAC) + fabsf(*off);
    pos[1] = *(f32*)(this + 0xB0) +
             ((*(i32*)(this + 0x820) == 0) ? *off * 0.5f : *off);
    FN_80AEACE0(32.0f, this, pos, id, i);
    off += 1;
    i   += 1;
}
*(u32*)(this + 0x818) = i - 1;
```

The offset table is 17 floats: `0, +32, -32, +64, -64, ..., +256, -256`.
`id` starts at `+0x814` and is decremented on every odd `i`, so the loop
terminates once `id` goes negative — the number of emitted nodes is
`2 * (+0x814) + 1`. `f31 = 0.5f` (`0x80B3119C`) is loaded in the prologue and
never used; it is a dead load.

Field map: `+0x814` start id, `+0x818` final count minus one, `+0x81C` x
direction flag, `+0x820` y scale flag, `+0xB0` height.

### Constants

| address | value | use |
|---|---|---|
| `0x80B31158` | 17 floats `0, +-32 .. +-256` | fan offsets |
| `0x80B3119C` | `0.5f` | dead load |
| `0x80B311A4` | `32.0f` | first argument of `0x80AEACE0` |

### Verdict: ready

No indirect call, no jump table, no unresolved register. `0x80AEACE0` is a
sibling in this REL and receives `(f1 = 32.0f, this, pos, id, i)`.

---

## 12. `0x80AF2400` - `Wm_en_burst_m` cannon placement

| field | value |
|---|---|
| TSV size | 284 bytes (body really ends at `0x80AF2520`) |
| reachable | 70 instructions, ends at `0x80AF2518`; `blr` at `0x80AF251C` is outside the extent |
| export | `work/ghidra/rel-d_enemiesNP/export/80af2400_FUN_80af2400.c` |
| signature | `void fn_80af2400(usize this, i32 player)` |
| verdict | **conditional** |

### What the stream does

```c
p = FN_8005F900(player);
f32 rate = 0.0f;
if (p != 0) {
    switch (*(i32*)(p + 0x14D4)) {          // player colour
    case 1: rate = 1.0f; break;
    case 2: rate = 2.0f; break;
    case 3: rate = 3.0f; break;
    default: break;                          // 0 and anything else -> 0.0f
    }

    FN_80166F40(this + 0x80, 1, 0);          // reset hold/animation at +0x80
    FN_80169F80(0.0f, this + 4, this + 0x80);
    FN_80166EF0(rate, this + 0x80, 0);
    FN_80166F10(0.0f, this + 0x80, 0);
}

a = FN_8023A0B0(this, "guide_st");           // 0x80B62BC4
FN_801651F0(this + 0x44, this + 4, &a, 1);
FN_801680E0(1.0f, this + 0x44);
FN_80169F80(0.0f, this + 4, this + 0x44);
/* epilogue: addi r1,r1,48 ; blr  (blr is at 0x80AF251C) */
```

The colour-to-rate table is `{1 -> 1.0, 2 -> 2.0, 3 -> 3.0, else 0.0}`,
which matches the wave-120 note "picks a `1.0`/`2.0`/`3.0`/`4.0` rate from the
player's colour field (`+0x14D4`)" except that the stream's default is `0.0`
and there is no `4.0` arm.

### Constants

| address | value | use |
|---|---|---|
| `0x80B3139C` | `1.0f` | colour 1 rate |
| `0x80B313A0` | `0.0f` | default rate / `0x80169F80` weight |
| `0x80B313A4` | `2.0f` | colour 2 rate |
| `0x80B313A8` | `3.0f` | colour 3 rate |
| `0x80B62BC4` | `"guide_st"` | animation clip name |

`r31` is built as `addis r31,r0,-32589` (`0x80B30000`) `addi r31,r31,4984`
(`+0x1378`) = **`0x80B31378`**; the four rate words are at `+0x24`, `+0x28`,
`+0x2C` and `+0x30` from that base, matching the table above.

### Verdict: conditional

The body is complete and call-free of indirect branches, but the TSV extent
stops 4 bytes early: the last reachable instruction inside the extent is
`addi r1,r1,48` at `0x80AF2518`, and the `blr` at `0x80AF251C` belongs to this
function (the next entry starts at `0x80AF2520`). Step needed: treat the body
as 288 bytes, or splice the `blr`. With that one word the spec is
implementation-ready.

---

## Verification

* 85 untranslated addresses with a `decompiled = true` export are absent from
  the `address` column of `ledger/functions.csv` and from every `^#` heading in
  `docs/*.md`; one (`0x80A10220`) is already translated in
  `src/game/enemies.rs` and is excluded, leaving 84.
* The twelve claimed addresses are 5,768 bytes of `functions.tsv` extent and
  1,441 reachable instruction words.
* Every address was decoded from `work/rels/elf/d_enemiesNP.elf` with the
  `file offset = vaddr - 0x80A00000 + 0x1000` mapping, walking every branch and
  following both edges of conditional `bclr`/`bcctr` forms.
* Eleven of the twelve reach a terminal instruction at exactly
  `entry + size - 4`. `0x80AF2400` reaches `entry + size - 8`; its `blr` is the
  word at `0x80AF251C`, which is outside the TSV extent.
* No `ledger/functions.csv` row, Rust file, or other doc was written by this
  pass; only `docs/SPEC_WAVE151_ENEMIES.md` was created.
