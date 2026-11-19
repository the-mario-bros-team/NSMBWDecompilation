# Wave 146 - `game::bases` - the next eight (0.83-0.88 KB band)

Read-only specification lane. No Rust file, no `ledger/functions.csv` row and no
existing document was modified. The only file written is this one. No merge or
check tool was run. The throwaway decoder and the ELF/table readers live under
`%TEMP%\opencode\w146` (`ppc.js`, `dump.js`, `info.js`, `verify.js`), never in
the repository.

Earlier waves have already walked this bucket from the top; the last row claimed
by `docs/SPEC_WAVE145_BASES.md` is 848 bytes. This wave continues one rank
lower, at 896 bytes down to 848 bytes. The window is now 0.83-0.88 KB per body
(6,932 bytes / 1,733 instructions), so every row gets a field map, a numbered
behaviour list, a constant table and an exact signature.

## The eight

| # | address | bytes | words | export file | verdict |
|--:|---:|---:|---:|---|:--:|
| 1 | `0x80760700` | 896 | 224 | `80760700_FUN_80760700.c` | ready (high) |
| 2 | `0x806E4F30` | 880 | 220 | `806e4f30_FUN_806e4f30.c` | ready (high) |
| 3 | `0x807318F0` | 876 | 219 | `807318f0_FUN_807318f0.c` | ready (high) |
| 4 | `0x80699380` | 868 | 217 | `80699380_FUN_80699380.c` | ready (medium) |
| 5 | `0x80648CB0` | 860 | 215 | `80648cb0_FUN_80648cb0.c` | ready (high) |
| 6 | `0x8069C2C0` | 852 | 213 | `8069c2c0_FUN_8069c2c0.c` | ready (high) |
| 7 | `0x807B77E0` | 852 | 213 | `807b77e0_FUN_807b77e0.c` | ready (high) |
| 8 | `0x807BEC10` | 848 | 212 | `807bec10_FUN_807bec10.c` | ready (medium) |

**Ready: 8. Conditional: 0.**

Every `bytes` value is the `size` column of
`work/ghidra/rel-d_basesNP/export/functions.tsv` verbatim, `decompiled` is
`true` for all eight, and in all eight cases the size is the true instruction
span: the decoder found the final `blr` at `address + size - 4` in each body
(`0x80760A7C`, `0x806E529C`, `0x80731C58`, `0x806996E0`, `0x80649008`,
`0x8069C610`, `0x807B7B30`, `0x807BEF5C`). There is no padding tail and no
`halt_baddata` cut this wave: **0** of the 1,733 words in the eight bodies is
undecodable, and no word falls outside the documented opcode set.

### Why none is conditional

The verdict rule is wave 144's, unchanged:

* **ready** - every reachable instruction is decoded, every literal is read out
  of the image, every callee is named by address (or by its ledger name), and
  the body can be written as code with no assumed calling convention beyond the
  standard PowerPC EABI. Data that lives in `main.dol` BSS (`0x8035xxxx`,
  `0x8037xxxx`, `0x8042xxxx`) or in this REL's own BSS (`0x8082xxxx`-
  `0x8083xxxx`) is an *input*, not an unknown constant, and is named by address.
  Reads from the REL's own `.data` (`0x807Cxxxx`, `0x807Dxxxx`, `0x8082xxxx`
  below `0x80822900`) are resolved to their byte values below.
* **conditional** - exactly one named input would still be missing.

Four rows were audited hard against that second clause and each passed for a
stated reason:

* `0x806E4F30` has the widest indirect surface in the set: three virtual calls
  per state arm plus `0x803717E0`, which is `main.dol` BSS (zero in the shipped
  image). Every virtual call has its receiver, its slot offset and its argument
  register written immediately before the `bctrl`, so the call *shape* is fixed
  by the bytes; `0x803717E0` is a named input, not an unknown constant. The
  `unaff_GQR0` in the export is the four `psq_st f28..f31,qr0,0` prologue
  spills - `qr0` is type 0 (plain `f32` pair) and the body has no quantisation
  at all.
* `0x807318F0` is the only `qr3` user in this set (two `psq_l ...,qr3,W=1`
  halfword loads). `docs/REL_ELF_MAPPING.md` pins `GQR3 = 0x00050005` (type 5,
  unsigned 16-bit, scale 0), and type 5 is exactly `u16 -> f32`, which is what
  the surrounding `* (1/256)` arithmetic needs.
* `0x80648CB0` and `0x8069C2C0` call `0x80237BD0` (sine), `0x80648470` and
  `0x8015F470`, all of which lie outside this REL or are already in the ledger;
  each call has its argument registers (including the `f1`/`f2` float pair)
  written immediately before the `bl`. `0x80648CB0`'s export says
  `unaff_GQR3`, and the raw stream confirms two `psq_l f1,0(rX),qr3,1` loads of
  the same 16-bit angle halfword.
* `0x807BEC10` reads three zero floats from the REL's own `.data`
  (`0x8082154C/50/54`, verified as `0x00000000` in the image) and calls
  `0x801C1650`/`0x802BE470` whose results feed a 0.1-unit movement test. Both
  are named inputs; the zero cells are read out verbatim below.

### This is the list wave 145 predicted, exactly

Re-deriving the candidate set from the current tree gives **1,941** rows: the
`5,615` TSV rows, filtered against the 16,880 addresses in
`ledger/functions.csv` and the 1,480 distinct `0x`-addresses that already appear
in a `#{1,6} ` heading of `docs/*.md`, leave 1,941, and the first eight are
byte-for-byte the list wave 145 published as its "next eight":

```
0x80760700 896 | 0x806E4F30 880 | 0x807318F0 876 | 0x80699380 868
0x80648CB0 860 | 0x8069C2C0 852 | 0x807B77E0 852 | 0x807BEC10 848
```

The 852-byte tie between `0x8069C2C0` and `0x807B77E0` is broken by address
ascending, exactly as wave 145 said it would be. Each of the eight was re-tested
against `ledger/functions.csv` (exact `address` field) and against every heading
in `docs/*.md` - including the `0x`-prefixed and bare spellings - before being
claimed here; all eight are absent from both, and each has a `decompiled=true`
TSV row with a matching `.c` export on disk.

Applying the same filter after this wave, the next eight are:
`0x8069BE20` (844), `0x807373B0` (844), `0x80705C60` (840), `0x80782F60` (836),
`0x80775190` (820), `0x8079A930` (816), `0x807A7220` (816), `0x807B84C0` (812).

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
Literal pools in `0x807Cxxxx` / `0x807Dxxxx` resolve through segments 1-4 and are
read out verbatim below; `0x8082154C` is inside segment 4 (real data, three zero
floats), while segment 5 (`0x80822900..0x80834D84`, `filesz = 0`) is this REL's
BSS and reads as zero. `0x8035xxxx`, `0x8037xxxx`, `0x8042xxxx` are `main.dol`
BSS and are named the same way.

**Decoder.** A throwaway PowerPC decoder was written under `%TEMP%\opencode\w146`
and validated against the facts `docs/REL_ELF_MAPPING.md` pins. It found and
corrected six defects in its own first pass; they matter for anyone re-deriving
these rows:

* Branch `LK` is **bit 0** of the word and `AA` is bit 1. Testing bit 1 for `LK`
  turns every `bl` into a plain `b`, which is how the first pass "lost" 26 call
  sites in row 1.
* Conditional branches use the 14-bit `BD` field at bits 2-15, not the 24-bit
  `LI` field of `b`. Using `LI` produces plausible-looking but wrong targets.
* The `SPR` field is `((word >> 11) & 0x1F) << 5 | ((word >> 16) & 0x1F)`.
  The other order decodes `mflr r0` (`0x7C0802A6`) as `spr256` and hides every
  `mtspr lr` / `mtspr ctr`.
* Opcode 63 holds **both** the double-precision A-form and the X-form moves.
  `fcmpu`/`fcmpo` are `XO10 = 0/32` (13 words in this set - without them rows 2,
  5, 6 and 8 look truncated); `fctiwz` is `XO10 = 15` with `FRA = 0` and
  `FRC = 0` (three words in row 4, the `(int)double` casts). Gekko aliases
  `ps_mr`/`ps_neg`/`ps_abs`/`ps_nabs` onto `fmr`/`fneg`/`fabs`/`fnabs`; only
  `ps_merge00/01/10/11` (`XO10 = 528/560/592/624`) are distinct.
* `psq_l` = 56, `psq_st` = 60; quantiser index in bits 12-14, `W` in bit 15.
  Opcode 59 is the single-precision float family, **not** paired-single.
* Single-float A-form uses **`FRC` (bits 21-25 = `(word >> 6) & 0x1F`)** for
  `fmuls`/`fdivs` and `FRB` for `fadds`/`fsubs`. Reading `fmuls` through `FRB`
  silently turns `fmuls fD,fA,fC` into `fD = fA * f0`; that defect "lost" the
  operands in rows 1, 3 and 5 on the first pass.

Register recovery: `0x802DCDxx` are the CodeWarrior `_savegpr_N` helpers at
4-byte stride with `N = 14 + (address - 0x802DCD40) / 4`. The formula is checked
against `FUN_802dcd58`'s own export, which lists `r20..r31` (=> `N = 20`), and
against `0x802DCD40` (`r14..r31`). `0x802DCD8C..0x802DCDBC` is the matching
`_restgpr_N` family with the same stride from `0x802DCD8C`. The incoming `r3`
survives both families, so Ghidra's `void FUN_807318f0(void)` /
`void FUN_80699380(void)` have lost their receivers; rows 3 and 4 restore them.

**Unexported callees.** Several direct `bl` targets in this set have no
`functions.tsv` row and no `.c` export: `0x800FC370`, `0x800FD8A0`,
`0x800FD8C0`, `0x800FD900`, `0x800FD920`, `0x800F2A00`, `0x800A3430`,
`0x800A4E90`, `0x8006CD90`, `0x8006CDE0`, `0x800955F0`, `0x80072BC0`,
`0x8006EF40`, `0x8006FA40` and `0x801648B0`. Each was checked against the
exported extent that precedes it: every one starts 4-0xC bytes *after* that
extent ends, so they are real entry points the auto-analysis missed, not
interior labels. They are named by address only, exactly as
`docs/SPEC_WAVE128_BASES.md` and `docs/SPEC_WAVE145_BASES.md` do.

---

## 1. `0x80760700` - the `cobCourse` door's enter/attach handler

**Evidence.** `work/ghidra/rel-d_basesNP/export/80760700_FUN_80760700.c`,
896 bytes, 224 words, synthetic `0x80760700`. Body `0x80760700..0x80760A7C`.

**What it is.** The post-load handler of the `cobCourse` / `cobCourseClear`
door object (the class documented in `docs/COVERAGE_WAVE114_BASES.md` as
`0x807605C0..0x80761930`, vtable `0x808150A0` at `+0x60`). It re-mounts the
door parts, writes the transform, snapshots the rest pose, and then decides -
from the world byte `0x8042A24D`, the door's own actor id and four door-state
predicates - which of the two course-exit actors to spawn, whether to mirror the
spawn position onto the world-map marker `[0x8042A1A0]`, and which of five cues
to post. It always returns `1`.

### Signature

```rust
pub unsafe fn fn_80760700(this: usize) -> u32
```

`r3` is the door object; `r3 = 1` on every exit path.

### Field map (offsets from `this`)

| offset | type | meaning |
|---:|---|---|
| `+0x04` | `u32` | actor id; only the low byte is used (`& 0xFF`) |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | current position (x, y, z) |
| `+0x128`,`+0x12C`,`+0x130` | `f32` | rest-pose snapshot of the position |
| `+0x134` | `f32` | rest-pose height, set to `80.0` |
| `+0x1A8` | opaque | sub-object reset by `0x8016A2A0(obj, 0, 0)` |
| `+0x250` | `u8` | cleared to `0` at entry |

Globals used: `0x8042A24D` (world byte, `u8`), `0x8042A1A0` (`u32` pointer to
the world-map attention/marker object), `0x8042A1AC` (`u32` cue-manager
pointer), `0x80429F7C` (`u32` pointer to the save/course-state object),
`0x80377DC8/CC/D0` (three `f32`, zero-fill `main.dol` BSS - the "no position"
vector).

### Exact behaviour

Prologue `0x80760700..0x8076071C`: `stwu r1,-0x50`; save LR, `r31`, `r30`,
`r29`; `r30 = this`. Epilogue `0x80760A60..0x80760A7C`: restore, `r3 = 1`,
`blr`.

```rust
fn_80760b90();                                  // 0x8076071C - mount the door parts
fn_80760f40(this);                              // 0x80760724 - write the transform
write_u8 (this, 0x250, 0);
write_f32(this, 0x128, read_f32(this, 0xAC));
write_f32(this, 0x12C, read_f32(this, 0xB0));
write_f32(this, 0x130, read_f32(this, 0xB4));
write_f32(this, 0x134, 80.0);                   // 0x807CEDC0
```

1. **Global gate** (`0x80760754`): `if call0(0x800FC370) != 0` -> step 11.
2. **Actor gate** (`0x80760760..0x80760770`):
   `if call1(0x800F9840, read_u32(this,0x04) & 0xFF) != 8` -> step 11.
3. **Inputs** (`0x80760774..0x8076077C`): `w = read_u8(0x8042A24D)`;
   `if call0(0x800FBFB0) == 0` -> step 4, else -> step 5.
   (`0x800FC370` sits in an unexported `main.dol` gap; `0x800F9840` returns the
   character *group* code - its export shows `0xB` for negative ids, `0`/`1`/`2`
   for the first groups - and `0x800FBFB0` is the "player is in the special
   group" gate whose export reads `r13-0x5433` against `0x800FBF30()`.)
4. **No-special arm** (`0x807609EC..0x80760A34`):
   ```rust
   let _p = spawn(0x29F, this, 0, ZERO, 0);       // 0x800F2A00 @0x80760A20
   if call1(0x800FD920, this) != 0 { cue(0x25); } // 0x801016B0 @0x80760A4C
   ```
   then step 11.
5. **Arm A** (`0x80760788..0x807607A4`): `a = call1(0x800FD8A0, this)`;
   `b = call1(0x800FD8C0, this)` **only if** `a != 0`.
   If `a == 0 || b != 0` -> step 6; otherwise -> step 9.
6. **Spawn 0x284** (`0x807607A8..0x80760804`):
   ```rust
   let n = if call1(0x800FD8C0, this) != 0 && w != 7 { 4 } else { 3 };
   let p = spawn(0x284, this, n, ZERO, 0);        // 0x800F2A00 @0x80760804
   ```
   (`0x800FD8C0` is re-read at `0x807607B0` for this test; `w != 7` compares the
   `0x8042A24D` byte with `7`.)
7. **World-7 arm** (`0x80760808..0x80760860`): if `w == 7` go to step 8; else
   ```rust
   if call1(0x800FD920, this) != 0 {
       copy_pos(MARKER, p);                       // [0x8042A1A0]+0xAC/B0/B4 = p+0xAC/B0/B4
       cue(0x24);                                 // 0x801016B0 @0x8076085C
   }
   ```
   then step 10.
8. **World-7 mirror** (`0x80760864..0x807608C0`):
   ```rust
   if call1(0x800FD8C0, this) != 0 {
       let m = call1(0x80789B10, w);              // 0x80789B10 @0x80760878, table 0x807CFA78
       call3(0x80789A80, m, 0, 4);                // 0x80789A80 @0x80760884
       copy_pos(MARKER, p);
       cue(0x2B);
   }
   ```
   then step 10. (`0x80789B10` reads the nine-entry `u32` table at `0x807CFA78`
   = `[0,1,4,2,3,5,6,7,7]`; `w == 7` selects `7`. `0x80789A80(mode, a, b)` is
   the per-mode actor factory.)
9. **Arm B** (`0x807608C8..0x807609A0`): `if call1(0x800FD8A0, this) == 0`
   -> step 10. Otherwise:
   * if `w == 7` (`0x807608EC..0x80760968`):
     ```rust
     let _p = spawn(0x284, this, 3, ZERO, 0);   // 0x800F2A00 @0x80760914
     let s = read_u32(0x80429F7C);
     if read_u32(this,0x04) & 0xFF == 0x25
        && read_u32(s, 0x54) == 0x25
        && (read_u32(s, 0x60).wrapping_sub(2) < 4) { cue(0x30); } // @0x80760964
     ```
   * else (`0x8076096C..0x807609A0`):
     `let _p = spawn(0x29F, this, 0, ZERO, 0);` (`0x800F2A00` @`0x807609A0`).
10. **Final cue** (`0x807609A4..0x807609E8`): if `call1(0x800FD900, this) != 0`
    and (`call1(0x800FD8A0, this) == 0` or `w == 7`) -> `cue(0x22)`.
11. **Tail** (`0x80760A50..0x80760A5C`): `0x8016A2A0(this + 0x1A8, 0, 0)`;
    `return 1`.

Helper shapes fixed by the bytes:
`spawn(id, parent, variant, pos, zero)` = `0x800F2A00(id, parent, variant, &pos, 0)`.
Each of the four call sites assembles its own 12-byte stack vector from the three
zero floats - `r1+44` (`0x807607EC`), `r1+32` (`0x807608F4`), `r1+20`
(`0x80760988`) and `r1+8` (`0x80760A00`) - and passes its address as the fourth
argument. `cue(id)` = `0x801016B0(read_u32(0x8042A1AC), id, 0, 0, 200)`.

### Constants read out of the image

| address | value | use |
|---|---|---|
| `0x807CEDC0` | `f32 80.0` | `+0x134` rest height |
| `0x80377DC8`,`0x80377DCC`,`0x80377DD0` | `f32 0.0` (BSS) | zero spawn vector |
| `0x8042A24D` | `u8` world byte (BSS) | compared with `7` |
| `0x8042A1A0` | `u32` pointer (BSS) | marker object |
| `0x8042A1AC` | `u32` pointer (BSS) | cue manager |
| `0x80429F7C` | `u32` pointer (BSS) | `+0x54` player id, `+0x60` course id |
| `0x807CFA78` | `u32[9] = 0,1,4,2,3,5,6,7,7` | `0x80789B10` table |

Event ids posted: `0x22`, `0x24`, `0x25`, `0x2B`, `0x30`. Spawn ids: `0x284`
(with variants `3`/`4`) and `0x29F` (variant `0`).

### Callees

| address | name | shape |
|---|---|---|
| `0x80760B90` | `fn_80760b90` | `()` |
| `0x80760F40` | `fn_80760f40` | `(this)` |
| `0x800FC370` | unclaimed, `main.dol` gap | `() -> u32` |
| `0x800F9840` | unclaimed | `(u32) -> u32` |
| `0x800FBFB0` | unclaimed | `() -> u32` |
| `0x800FD8A0`,`0x800FD8C0`,`0x800FD900`,`0x800FD920` | unclaimed, `main.dol` gap | `(this) -> u32` |
| `0x800F2A00` | unclaimed spawn helper | `(id, parent, variant, *f32, u32)` |
| `0x80789B10` | `fn_80789b10` | `(u32) -> u32` |
| `0x80789A80` | `FUN_80789a80` | `(u32, u32, u32) -> u32` |
| `0x801016B0` | `fn_801016b0` | `(mgr, id, a, b, c)` |
| `0x8016A2A0` | `fn_8016a2a0` | `(obj, a, b)` |

**Confidence: high.** All 224 words decode, all five cue ids and both spawn ids
are immediates, and the only indirect surface is `0x8042A1A0`/`0x8042A1AC`
(BSS pointers that are dereferenced, never branched on).

---

## 2. `0x806E4F30` - `lift_zen_kaihei` clip state machine

**Evidence.** `work/ghidra/rel-d_basesNP/export/806e4f30_FUN_806e4f30.c`,
880 bytes, 220 words, synthetic `0x806E4F30`. Body `0x806E4F30..0x806E529C`.

**What it is.** The four-state clip driver of the `lift_zen_kaihei` lift actor
(`docs/COVERAGE_WAVE80_BASES.md` records the class: loader `fn_806e4da0`, clip
bind/play `fn_806e4e90`, per-frame composite `fn_806e5680`, which is the only
caller of this body). It watches the actor's "target present" predicate and a
`u32` approach counter, and cross-fades between the idle clip (`mode 1`) and the
open clip (`mode 3`) by re-binding the clip and re-seating its time. The export's
`ldexpf`/`unaff_GQR0` noise is the four `psq_st f28..f31,qr0,0` prologue spills.

### Signature

```rust
pub unsafe fn fn_806e4f30(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x3F0` | `void*` | model holder (`0x80165FD0` receiver, `+0x18` publish) |
| `+0x3FC` | clip | clip/animation object; `r4 = 0` = track 0 |
| `+0x428` | object | animator sub-object; slot `+0x28` returns the motion set |
| `+0x48C` | `u32` | approach counter, converted to `f32` as an *unsigned* value |
| `+0x49C` | `u8` | direction flag; `0` = closing, `1` = opening |
| `+0x588` | `u8` | state: `0`, `1`, `2`, `3` |

### Exact behaviour

Prologue `0x806E4F30..0x806E4F70`: `stwu r1,-0x60`; save LR, `r31`, `r30`;
`stfd f31/f30/f29/f28` each followed by `psq_st fN,qr0,0` (paired-slot spill);
`r31 = this`; `r30 = read_u8(this,0x588)`. Epilogue `0x806E5268..0x806E529C`.

```rust
let t0   = call_f32(0x80166380, this + 0x3FC, 0);   // @0x806E4F74 -> f31
let t1   = call_f32(0x80166360, this + 0x3FC, 0);   // @0x806E4F84 -> f30
let span = t1 - t0;                                 // f29
let counter = read_u32(this, 0x48C) as f32;         // 0x43300000/2^52 ladder
let state = read_u8(this, 0x588);
```

`has` is the target predicate, recomputed once per state arm exactly as below:

```rust
let obj  = vcall(this + 0x428, 0x28);                    // lwz r12,0x28(vt)
let has  = vcall1(obj, 0x14, 0x803717E0) != 0;           // slot +0x14 with a type tag
```

1. **`state == 1`** (`0x806E4F94..0x806E502C`):
   `if has && read_u8(this,0x49C) == 0 && counter < span`:
   `0x806E4E90(this, 3)`; `0x801662F0(t0 + counter, this+0x3FC, 0)`;
   `write_u8(this, 0x588, 2)`.
2. **`state == 2`** (`0x806E5030..0x806E50E0`):
   `if has && read_u8(this,0x49C) == 1 && counter <= 35.0`:
   `0x806E4E90(this, 1)`; `0x801662F0((35.0 - counter) + t0, this+0x3FC, 0)`;
   `write_u8(this, 0x588, 1)`.
   (The `<=` is the CodeWarrior `fcmpo` + `cror 2,0,2` idiom at `0x806E50A8`:
   word `0x4C401382` turns `EQ` into `LT|EQ` before the `bne`.)
3. **`state == 0` or `3`** (`0x806E50E4..0x806E5264`), `has` computed at
   `0x806E50E4`:
   * `has == 0` (`0x806E5210`):
     `if read_u8(this,0x49C) == 1 { 0x806E4E90(this,3); 0x801662F0(t0,...); state=2 }`
     `else { 0x806E4E90(this,1); 0x801662F0(t1,...); state=1 }`.
   * `has != 0` and `read_u8(this,0x49C) == 1` (`0x806E5148..0x806E51B4`):
     `if counter <= 35.0` -> `mode 1`, time `(35.0 - counter) + t0`, `state = 1`;
     else -> `mode 3`, time `t0`, `state = 2`.
   * `has != 0` and `read_u8(this,0x49C) != 1` (`0x806E51B8..0x806E5208`):
     `if counter < span` -> `mode 3`, time `counter + t0`, `state = 2`;
     else -> `mode 1`, time `t1`, `state = 1`.

Every arm ends by falling to the epilogue at `0x806E5268`; no arm returns a
value (`r3` is whatever the last call left).

The `u32 -> f32` ladder is at `0x806E4FE0..0x806E4FFC` (state 1) and
`0x806E5080..0x806E50A4` (state 2) / `0x806E5120..0x806E5144` (state 0/3):
`stfs`/`lwz` of `this+0x48C`, then `lfd [0x807CBCB0]` (a `f64` whose bit
pattern is `0x4330000000000000`, i.e. `2^52`) and `fsubs` - the standard
unsigned-int-to-float idiom, so the counter is read as `u32`, not `i32`.

### Constants read out of the image

| address | raw | value | use |
|---|---|---|---|
| `0x807CBCA8` | `0x420C0000` | `f32 35.0` | open/close threshold |
| `0x807CBCB0` | `0x43300000` | `f64 4503599627370496.0` (`2^52`) | `u32 -> f32` ladder |
| `0x803717E0` | BSS | zero at load | class/type tag passed to slot `+0x14` |

### Callees and slots

| target | shape | note |
|---|---|---|
| `0x80166380` | `(clip, 0) -> f32` | clip start / first key |
| `0x80166360` | `(clip, 0) -> f32` | clip end / last key (`fn_80166360`, 20 B) |
| `0x801662F0` | `(f32, clip, 0)` | f32 clip-time setter |
| `0x806E4E90` | `(this, mode)` | `fn_806e4e90` - binds `lift_zen_kaihei` into `+0x3FC` and plays it with the mode |
| `*(this+0x428)+0x28` | `(this+0x428) -> obj` | animator accessor |
| `obj+0x14` | `(obj, 0x803717E0) -> u32` | type/motion lookup |

**Confidence: high.** All 220 words decode; the only two "unknowns" in the
export (`unaff_GQR0`, the `ldexpf` quadruplets) are the `qr0` prologue spills,
and `0x803717E0` is a BSS tag, not a constant that changes the code shape.

---

## 3. `0x807318F0` - the world-map enemy burst emitter

**Evidence.** `work/ghidra/rel-d_basesNP/export/807318f0_FUN_807318f0.c`,
876 bytes, 219 words, synthetic `0x807318F0`. Body `0x807318F0..0x80731C58`.

**What it is.** The per-frame burst driver of a world-map enemy. Its only caller
is `0x80732010` (`FUN_80732010`), which runs it when the `+0x9D0` countdown
reaches zero and `0x80731F30(this)` agrees. The body probes four compass
directions at radius `24.0` around the actor's feet, skips the actor's own
facing index, spawns `Wm_en_burst_s` particle bursts at up to three of the four
points, records a direction bitmask at `+0xA05`, and then spawns up to three
more bursts at three fixed offsets when that bitmask says so. The export's
`unaff_GQR0`/`unaff_GQR3`/`ldexpf`/`CONCAT` noise is two `psq_st` prologue
spills plus two real `psq_l ...,qr3,W=1` halfword loads.

### Signature

```rust
pub unsafe fn fn_807318f0(this: usize)
```

The export prints `void FUN_807318f0(void)` because `0x802DCD58` is the
`_savegpr_20` helper; the incoming `r3` survives into `r22` at `0x80731924`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x04` | `u32` | actor id (low 22 bits feed the effect attribute; full word feeds `+0x15B`) |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | position; only used to seed the stack vector |
| `+0x978` | `f32` | probe-ring centre x |
| `+0x97C` | `f32` | probe-ring centre y |
| `+0x984`,`+0x988` | `f32` | extra burst 0 (x, y) |
| `+0x990`,`+0x994` | `f32` | extra burst 1 (x, y) |
| `+0x99C`,`+0x9A0` | `f32` | extra burst 2 (x, y) |
| `+0x9CC` | `u32` | mode selector (mirror flag); `0` and non-`0` swap the two tables |
| `+0x9D4` | `i16` | facing angle (16-bit fixed point) |
| `+0xA05` | `u8` | written: `0x807CD770[angle >> 13]` |

### Exact behaviour

Prologue `0x807318F0..0x80731918`: `stwu r1,-0x90`; save LR; `r11 = r1+0x60`;
`stfd f31/f30/f29` + `psq_st fN,qr0,0`; `bl 0x802DCD58` (`_savegpr_20`, saves
`r20..r31`). Epilogue `0x80731C2C..0x80731C58` (`bl 0x802DCDA4` =
`_restgpr_20`). `r22 = this`.

1. **Facing classification** (`0x80731918..0x80731958`):
   ```rust
   let angle = read_i16(this, 0x9D4);
   let mut bump = 0u32;          // r26
   let mut diagonal = 0u32;      // r25
   if matches!(angle, 0x2000 | 0x6000 | -0x6000 | -0x2000) { bump = 0x2000; diagonal = 1; }
   ```
2. **Skip index** (`0x8073195C..0x807319F4`):
   ```rust
   let mode = read_u32(this, 0x9CC);
   let skip = if mode == 0 {
       match angle { -0x8000 | 0x6000 => 3, -0x4000 | -0x6000 => 2, 0 | -0x2000 => 1, _ => 0 }
   } else {
       match angle { -0x8000 | 0x6000 => 1, 0x2000 | 0x4000 => 2, 0 | -0x2000 => 3, _ => 0 }
   };
   ```
3. **Four-direction loop** (`0x807319F8..0x80731B58`), `i = 0..3`,
   `bump` advanced by `0x4000` after every iteration (including skipped ones):
   ```rust
   const IDS_DIAGONAL: [u32;4] = [5, 4, 6, 7];  // 0x807CD750
   const IDS_PLAIN:    [u32;4] = [3, 1, 2, 0];  // 0x807CD760
   if skip != i {
       let theta = f32::from(read_u16_of(bump)) * (1.0 / 256.0);   // psq_l qr3,W=1
       let mut p = [ read_f32(this,0xAC), read_f32(this,0xB0), read_f32(this,0xB4) ];
       p[0] = read_f32(this, 0x978) + 24.0 * cos(theta);           // 0x80237C40
       p[1] = read_f32(this, 0x97C) + 24.0 * sin(theta);           // 0x80237BD0
       p[2] = -50.0;
       let suppress = if diagonal != 0 {
           if mode == 0 { i >= 2 } else { i < 2 }
       } else {
           if mode == 0 { i == 3 } else { i == 1 }
       };
       let id = if diagonal != 0 { IDS_DIAGONAL[i] } else { IDS_PLAIN[i] };
       if !suppress {
           let base = read_u32(this, 0x04);
           let attr = (id << 4) | 0x301 | (base & 0x300000);
           0x80064610(base + 0x15B, attr, &p, 0, 0);               // @0x80731B1C
           let mut m = [0u32; 3];
           0x8006A4B0(&mut m, &p);                                 // @0x80731B2C
           0x80198C30(read_u32(0x80429D60), 0x2A5, &m, 0);        // @0x80731B40
       }
   }
   bump = bump.wrapping_add(0x4000);
   ```
   Note the two `qr3` loads at `0x80731A40` and `0x80731A60`: `sth r26,10(r1)` /
   `sth r26,8(r1)` followed by `psq_l f0,0(r30),qr3,1`. With
   `GQR3 = 0x00050005` (type 5, unsigned 16-bit) these are plain
   `u16 -> f32` conversions of the low half of `bump`.
4. **Bitmask publish** (`0x80731B5C..0x80731B6C`):
   ```rust
   let idx = (read_u16(this, 0x9D4) >> 13) as usize;              // 0..7
   let mask = BYTES_807CD770[idx];
   write_u8(this, 0xA05, mask);
   ```
5. **Three extra bursts** (`0x80731B70..0x80731C28`), each
   `0x8016C890(0x8080D0E8, 0, &[x, y, 5500.0], 0, 0)`:
   * `mask & 8` -> `x = read_f32(this,0x984)`, `y = read_f32(this,0x988)`;
   * `mask & 2` -> `x = read_f32(this,0x990)`, `y = read_f32(this,0x994)`;
   * `mask & 1` -> `x = read_f32(this,0x99C)`, `y = read_f32(this,0x9A0)`.
   `0x8080D0E8` is the C string `"Wm_en_burst_s"` (read out of the image).

### Constants read out of the image

| address | raw | value | use |
|---|---|---|---|
| `0x807CD73C` | `0x41C00000` | `f32 24.0` | probe radius |
| `0x807CD740` | `0x3B800000` | `f32 0.00390625` (`1/256`) | angle scale |
| `0x807CD750` | 4 x `u32` | `5, 4, 6, 7` | effect ids, diagonal table |
| `0x807CD760` | 4 x `u32` | `3, 1, 2, 0` | effect ids, plain table |
| `0x807CD770` | 8 x `u8` | `07 01 0D 09 0F 0A 0E 02` | `+0xA05` bitmask table |
| `0x807CD778` | `0xC2480000` | `f32 -50.0` | probe z |
| `0x807CD77C` | `0x45ABE000` | `f32 5500.0` | extra-burst z |
| `0x8080D0E8` | ASCII | `"Wm_en_burst_s"` | effect name |
| `0x80429D60` | BSS | `u32` pointer | effect-manager argument |

### Callees

| address | name | shape |
|---|---|---|
| `0x802DCD58` / `0x802DCDA4` | save/restore `r20..r31` | - |
| `0x80237C40` | cosine (`f64`) | `(f64) -> f64` |
| `0x80237BD0` | sine (`f64`) | `(f64) -> f64` |
| `0x80064610` | `FUN_80064610` | `(id, attr, *f32, 0, 0)`; forwards to `0x8006CA80` |
| `0x8006A4B0` | `FUN_8006a4b0` | `(out2, in3)` world -> screen projection |
| `0x80198C30` | `fn_80198c30` | `(mgr, 0x2A5, *screen2, 0)` effect spawn |
| `0x8016C890` | `fn_8016c890` | `(name, 0, *f32, 0, 0)` named effect spawn |

**Confidence: high.** `qr3` is pinned, the five tables are read out verbatim,
and every call argument register is written immediately before its `bl`.

---

## 4. `0x80699380` - the moving-collision actor's spawn/configure body

**Evidence.** `work/ghidra/rel-d_basesNP/export/80699380_FUN_80699380.c`,
868 bytes, 217 words, synthetic `0x80699380`. Body `0x80699380..0x806996E0`.
`0x80699380` is the only word in the whole REL equal to the function address; it
sits at `0x807F177C` in the class vtable.

**What it is.** The "become active" body of an actor whose collision is three
12.20 fixed-point motion records (a moving platform/obstacle). It selects a mode
from two character predicates plus the `0x80429F80` flag, writes the two mode
setters, offsets the transform, publishes two global timing floats, installs the
collision descriptor, registers the frame-callback set, hands the three records
to the collision engine, and fills a large block of hitbox/state fields. It
returns `1`.

### Signature

```rust
pub unsafe fn fn_80699380(this: usize) -> u32
```

The export prints `void FUN_80699380(void)` because `0x802DCD74` is
`_savegpr_27`; the receiver survives into `r27` at `0x80699398`. `r3 = 1` at
`0x80699644` survives to the epilogue, so the body returns `1`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x04`..`+0x10` | - | inherited actor header (not touched) |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | position (moved by `(+176, -72, 0)`) |
| `+0xDC`,`+0xE0`,`+0xE4` | `f32` | scale, all `1.25` |
| `+0x148` | object | collision area object (`0x8008C3E0` / `0x8008C330`) |
| `+0x1E6` | `u8` | set to `1` |
| `+0x1EC` | object | collision/motion record container (`0x8006D090`) |
| `+0x348` | `u8` | read: selects `0x8042C1A0[0|1]` |
| `+0x38F` | `u8` | read: variant passed to `0x8007FB80` |
| `+0x500` | `u16` | `30` (from `0x807F13C0`) |
| `+0x654` | object | frame-callback set (`0x8007FB80`, `0x8007F900`) |
| `+0x748` | `u32` | `0` |
| `+0x750`,`+0x754`,`+0x758` | `f32` | saved pose before the offset |
| `+0xD4C` | `u32` | `0` |
| `+0xD50` | `f32` | `old_y - 112.0` |
| `+0xD54`..`+0xD5C` | 3 x `u32` | record 1 = `(0, 0, +v12)` |
| `+0xD60`..`+0xD68` | 3 x `u32` | record 2 = `(0, 0, -v12)` |
| `+0xD6C`..`+0xD74` | 3 x `u32` | record 3 = `(0, +v12, 0)` |
| `+0xD78` | `i16` | `(s8)0x8042C1A0[+0x348] * 4096` |
| `+0xD7C` | `u32` | `120` |
| `+0xD80` | `u32` | `40` |
| `+0xD84` | `u32` | `0` |
| `+0xD88`,`+0xD8C` | `u32` | `0` |
| `+0xD90` | `f32` | `saved_y - 112.0` |
| `+0xD94`,`+0xD98` | `f32` | `x + 128`, `x - 128` |
| `+0xD9C`,`+0xDA0` | `f32` | `x + 64`, `x - 64` |
| `+0xDA4`,`+0xDA8` | `f32` | `x + 96`, `x - 96` |
| `+0xDAC`..`+0xDB8` | `u32` | `0` |
| `+0xDBC` | `u32` | `0` |
| `+0xDC0`,`+0xDC4`,`+0xDC8` | `f32` | `0.0` |
| `+0xDCC` | `u32` | `1` |
| `+0xDD0` | `u32` | `-1` |
| `+0xDD4` | `u32` | `0` |
| `+0xDD8` | `u8` | `1` |
| `+0xDDC` | `f32` | `1.0` |

### Exact behaviour

Prologue `0x80699380..0x80699398`: `stwu r1,-0x40`; save LR; `r11 = r1+0x40`;
`bl 0x802DCD74` (`_savegpr_27`, saves `r27..r31`); `r27 = this`.
Epilogue `0x806996D4..0x806996E0` (`bl 0x802DCDC0` = `_restgpr_27`, then LR and
stack restored in-line).

1. **Mode select** (`0x806993A0..0x806993E4`):
   ```rust
   call0(0x800A3430);
   let mut flag = 0u32;
   if call2(0x800FAE40, 3, 0x25) != 0 { flag = 1; }
   else if call2(0x800FAE40, 5, 0x25) != 0 { flag = 1; }
   if read_u32(0x80429F80) & 0x10 != 0 { flag = 1; }
   vcall2(this, 0x360, flag);          // @0x806993E8..0x806993FC
   vcall2(this, 0x364, 0);             // @0x80699400..0x80699414
   ```
2. **Transform offset** (`0x80699418..0x80699474`):
   ```rust
   let x = read_f32(this,0xAC); let y = read_f32(this,0xB0); let z = read_f32(this,0xB4);
   write_f32(this, 0x750, x); write_f32(this, 0x754, y); write_f32(this, 0x758, z);
   write_f32(this, 0xD50, y - 112.0);
   write_f32(this, 0xAC, x + 176.0);
   write_f32(this, 0xB0, y - 72.0);
   write_f32(this, 0xDC, 1.25); write_f32(this, 0xE0, 1.25); write_f32(this, 0xE4, 1.25);
   ```
3. **Two global timing floats** (`0x80699478..0x806994A8`): two calls to slot
   `+0x314` on `*(this+0x60)`; the `f32` results go to `0x807F1410` and
   `0x807F1414` (`stfs f1,8(r30)` / `stfs f1,12(r30)`).
4. **Collision descriptor** (`0x806994AC..0x806994C8`):
   ```rust
   collision_area_set_descriptor(this + 0x148, this, 0x807F1408);  // 0x8008C3E0
   write_u8(this, 0x1E6, 1);
   collision_area_register(this + 0x148);                          // 0x8008C330
   ```
   `0x807F1408` is four zero words in the REL image (a zeroed descriptor
   template); `0x8008C3E0` stores the owner at `obj+4`, copies the nine
   descriptor fields into `obj+0x1C..0x3C` and clears `obj+0xA1`.
5. **Frame-callback set** (`0x806994CC..0x80699514`):
   ```rust
   let v = vcall_f64(this, 0x314) as f32;
   0x8007FB80(0.0, 0.0, 0.85 * v, this + 0x654, this, 0, 0, 0, 1, read_u8(this, 0x38F));
   0x8007F900(this + 0x654);
   ```
6. **Three motion records + collision hand-off** (`0x80699518..0x806995C4`).
   Three further slot `+0x314` calls, each converted with `fctiwz`
   (`stfd f0,8(r1)` + `lwz r0,12(r1)`) and masked:
   ```rust
   write_u32(this, 0xD54, 0); write_u32(this, 0xD58, 0);
   write_u32(this, 0xD5C, (v1 << 12) & 0xFFFFF000);
   write_u32(this, 0xD60, 0); write_u32(this, 0xD64, 0);
   write_u32(this, 0xD68, ((-v2) << 12) & 0xFFFFF000);
   write_u32(this, 0xD6C, 0);
   write_u32(this, 0xD70, (v3 << 12) & 0xFFFFF000);
   write_u32(this, 0xD74, 0);
   0x8006D090(this + 0x1EC, this, this + 0xD60, this + 0xD54, this + 0xD6C);
   ```
7. **Hitbox extents** (`0x806995C8..0x80699678`). `f10 = 16.0`,
   `f4 = +0x750`:
   `wide = 8*16 = 128`, `mid = 4*16 = 64`, `narrow = 6*16 = 96`,
   `down = -7*16 = -112`:
   ```rust
   write_f32(this, 0xD94, f4 + 128.0); write_f32(this, 0xD98, f4 - 128.0);
   write_f32(this, 0xD9C, f4 +  64.0); write_f32(this, 0xDA0, f4 -  64.0);
   write_f32(this, 0xDA4, f4 +  96.0); write_f32(this, 0xDA8, f4 -  96.0);
   write_f32(this, 0xD90, read_f32(this, 0x754) - 112.0);
   ```
8. **Flag/geometry block** (`0x8069967C..0x806996CC`):
   ```rust
   write_u32(this, 0xD7C, 120);
   write_u32(this, 0xD80, 40);
   write_u32(this, 0xD84, 0);
   write_u32(this, 0x748, 0);
   write_i16(this, 0xD78, (read_i8(0x8042C1A0 + read_u8(this,0x348)) as i32 * 4096) as i16);
   write_u32(this, 0xD8C, 0); write_u32(this, 0xD88, 0);
   write_u32(this, 0xDAC, 0); write_u32(this, 0xDB0, 0);
   write_u32(this, 0xDB4, 0); write_u32(this, 0xDB8, 0);
   write_u32(this, 0xD4C, 0); write_u32(this, 0xDBC, 0);
   write_f32(this, 0xDC8, 0.0); write_f32(this, 0xDC4, 0.0); write_f32(this, 0xDC0, 0.0);
   write_f32(this, 0xDDC, 1.0);
   write_u32(this, 0xDCC, 1); write_u32(this, 0xDD0, -1);
   write_u32(this, 0xDD4, 0); write_u8(this, 0xDD8, 1);
   ```
9. `return 1`.

### Constants read out of the image

| address | raw | value | use |
|---|---|---|---|
| `0x807CA080` | `0x42E00000` | `f32 112.0` | `y - 112` (bottom) |
| `0x807CA084` | `0x43300000` | `f32 176.0` | `x + 176` |
| `0x807CA088` | `0xC2900000` | `f32 -72.0` | `y - 72` |
| `0x807CA08C` | `0x00000000` | `f32 0.0` | `f2 = 0` for `0x8007FB80` |
| `0x807CA090` | `0x3F59999A` | `f32 0.85` | callback rate scale |
| `0x807CA094` | `0x41800000` | `f32 16.0` | extent scale |
| `0x807CA098` | `0x3F800000` | `f32 1.0` | `+0xDDC` |
| `0x807F1344` | `0x3FA00000` | `f32 1.25` | scale x3 |
| `0x807F1354` | `0x1000` (`s16`) | `4096` | `+0xD78` multiplier |
| `0x807F1356` | `0x0078` (`s16`) | `120` | `+0xD7C` |
| `0x807F135E` | `0x0028` (`s16`) | `40` | `+0xD80` |
| `0x807F1364` | `0x41000000` | `f32 8.0` | `128` after `*16` |
| `0x807F1368` | `0x40800000` | `f32 4.0` | `64` after `*16` |
| `0x807F136C` | `0x40C00000` | `f32 6.0` | `96` after `*16` |
| `0x807F1370` | `0xC0E00000` | `f32 -7.0` | `-112` after `*16` |
| `0x807F13C0` | `0x0000001E` | `30` (low half) | `+0x500` |
| `0x807F1408`..`0x807F1414` | 4 x `0x00000000` | zeros | descriptor template, then two published floats |
| `0x8042C1A0` | BSS | `u8` table | indexed by `+0x348` |
| `0x80429F80` | BSS | `u32` flags | bit `0x10` = "all/special" |
| `0x80429D60` | BSS | `u32` pointer | effect manager |

### Callees

| address | name | shape |
|---|---|---|
| `0x802DCD74` / `0x802DCDC0` | save/restore `r27..r31` | - |
| `0x800A3430` | unexported `main.dol` gap | `()` |
| `0x800FAE40` | unclaimed predicate | `(u32, u32) -> u32` |
| `0x8008C3E0` | `collision_area_set_descriptor` | `(obj, owner, *desc)` |
| `0x8008C330` | `collision_area_register` | `(obj)` |
| `0x8007FB80` | unclaimed | `(f1, f2, f3, dest, owner, 0, 0, 0, 1, u8)` |
| `0x8007F900` | `fn_8007f900` | `(callback_set)` |
| `0x8006D090` | unclaimed | `(container, owner, *rec1, *rec2, *rec3)` |

**Confidence: medium.** Two of the calls (`0x800A3430`, `0x800FAE40`) and one
table (`0x8042C1A0`) live outside this REL, so their bodies cannot be checked
here; every argument, every store offset and every fixed-point mask is an
immediate in the stream, and no control flow depends on any of them.

---

## 5. `0x80648CB0` - the bouncing-actor height driver

**Evidence.** `work/ghidra/rel-d_basesNP/export/80648cb0_FUN_80648cb0.c`,
860 bytes, 215 words, synthetic `0x80648CB0`. Body `0x80648CB0..0x80649008`.

**What it is.** The four-state vertical driver of the unnamed `0x1A500`-byte UI
actor recorded in `docs/COVERAGE_WAVE112_BASES.md` (`0x80647B50..0x806485C0`,
class name unknown, height publisher `fn_80648440`, pitch ramp `fn_806485c0`).
It runs a table-driven ease profile, then a sine body with a decaying
amplitude, then a hold, and publishes the resulting height through
`0x80648470(this)`. The export's `unaff_GQR3`/`ldexpf`/`CONCAT` noise is two
`psq_l f1,0(rX),qr3,1` angle loads.

### Signature

```rust
pub unsafe fn fn_80648cb0(this: usize)
```

### Field map (the `+0x1A4xx` block is addressed as `this + 0x20000 - k`)

| offset | type | meaning |
|---:|---|---|
| `+0x1A49C` | `f32` | published height (written every frame) |
| `+0x1A4B0` | `f32` | upper travel limit |
| `+0x1A4B4` | `f32` | step per frame (state 0 seed and state 3 restart) |
| `+0x1A4B8` | `f32` | accumulated offset |
| `+0x1A4CC` | `f32` | velocity integrator (state 0 only) |
| `+0x1A4C8` | `f32` | sine amplitude (state 2) |
| `+0x1A4E8` | `i16` | state: `0`, `1`, `2`, `3` |
| `+0x1A4EC` | `u16` | frame counter **and** phase index (`>> 5`) |
| `+0x1A4EE` | `u16` | hold counter (state 3) |
| `+0x1A4F0`,`+0x1A4F2` | `u16` | hold presets copied into `+0x1A4EE` |
| `+0x1A4F4` | `u16` | selects the fast `0.125` amplitude decay |
| `+0x1A4F6` | `u8` | direction flag (negates the delta) |
| `+0x1A4F9` | `u8` | read by `fn_806485c0`, not by this body |

### Exact behaviour

Prologue `0x80648CB0..0x80648CD0`: `stwu r1,-0x30`; save LR, `r31`, `r30`;
`r31 = 0x807C8448` (constant-pool base); `r30 = this`. Epilogue
`0x80648FF4..0x80649008`.

```rust
let r8 = this + 0x20000;                    // addis r8,r3,2 @0x80648CB8
match read_i16(this, 0x1A4E8) {
```

1. **`state == 0`** (`0x80648CE0..0x80648DB8`):
   ```rust
   let counter = read_u16(this, 0x1A4EC);
   let i = (counter >> 5) as usize;                    // 0..3
   let a = read_i8(0x807C8518 + 2*i) as f32;           // -1 or +1
   let b = read_i8(0x807C8519 + 2*i) as f32;           // -1 or +1
   let f = read_f32(this, 0x1A4B4) * (1.0/64.0) * a;
   write_f32(this, 0x1A4CC, read_f32(this, 0x1A4CC) + f);
   let mut d = read_f32(this, 0x1A4CC) * b;
   if read_u8(this, 0x1A4F6) != 0 { d = -d; }
   if i != 0 { d *= 0.5; }
   write_f32(this, 0x1A4B8, read_f32(this, 0x1A4B8) + d);
   write_f32(this, 0x1A49C, read_f32(this, 0xB0) + read_f32(this, 0x1A4B8));
   fn_80648470(this);
   let c = read_u16(this, 0x1A4EC).wrapping_sub(1);
   write_u16(this, 0x1A4EC, c);
   if c == 0 { write_i16(this, 0x1A4E8, 1); }
   ```
   The two signed-byte loads at `0x80648D00` / `0x80648D08` go through the
   `xoris 0x8000` / `0x43300000` / `2^52+2^31` ladder, i.e. `(f32)(s8)value`.
2. **`state == 1`** (`0x80648DC4..0x80648E40`):
   ```rust
   let r = fn_806485c0(this, 1);
   if r == 2 { write_u8(this, 0x1A4F6, 0); write_u16(this, 0x1A4EE, read_u16(this, 0x1A4F2)); }
   else if r == 1 { write_u8(this, 0x1A4F6, 1); write_u16(this, 0x1A4EE, read_u16(this, 0x1A4F0)); }
   if r != 0 { write_f32(this, 0x1A4C8, 8.0); write_i16(this, 0x1A4E8, 2); }
   write_f32(this, 0x1A49C, read_f32(this, 0xB0) + read_f32(this, 0x1A4B8));
   fn_80648470(this);
   ```
3. **`state == 2`** (`0x80648E4C..0x80648F18`):
   ```rust
   let raw = (read_u16(this, 0x1A4EC) & 0xFF) << 9;              // rlwinm 9,16,22
   let theta = raw as f32 * (1.0/256.0);                         // psq_l qr3,W=1
   let s = sin_80237bd0(theta as f64) as f32;
   write_f32(this, 0x1A49C, read_f32(this, 0x1A4C8) * s
                          + read_f32(this, 0xB0) + read_f32(this, 0x1A4B8));
   fn_80648470(this);
   if read_f32(this, 0x1A4B8) == 0.0 {
       write_u16(this, 0x1A4EC, read_u16(this, 0x1A4EC).wrapping_sub(1) & 0x7F);
   } else if read_f32(this, 0x1A4B8) == read_f32(this, 0x1A4B0) {
       write_u16(this, 0x1A4EC, read_u16(this, 0x1A4EC).wrapping_add(1) & 0x7F);
   }
   let decay = if read_u16(this, 0x1A4F4) != 0 { 0.125 } else { 0.03125 };
   let amp = read_f32(this, 0x1A4C8) - decay;
   write_f32(this, 0x1A4C8, amp);
   if amp <= 3.0 { write_i16(this, 0x1A4E8, 3); }
   ```
   The angle is `((counter & 0xFF) << 9)` stored as a halfword
   (`rlwinm r0,r0,9,16,22` at `0x80648E54`) and read back by
   `psq_l f1,0(r3),qr3,1`; `1/256` then reduces it to `2 * (counter & 0xFF)`.
4. **`state == 3`** (`0x80648F24..0x80648FF0`):
   ```rust
   let hold = read_u16(this, 0x1A4EE);
   if hold != 0 {
       write_u16(this, 0x1A4EE, hold - 1);
   } else if (read_u16(this, 0x1A4EC) & 0x7F) == 0 {
       write_f32(this, 0x1A4CC, read_f32(this, 0x1A4B4));
       write_u16(this, 0x1A4EC, 0x3F);
       write_i16(this, 0x1A4E8, 0);
       return;
   }
   let raw = (read_u16(this, 0x1A4EC) & 0xFF) << 9;
   let theta = raw as f32 * (1.0/256.0);
   let s = sin_80237bd0(theta as f64) as f32;
   write_f32(this, 0x1A49C, 3.0 * s + read_f32(this, 0xB0) + read_f32(this, 0x1A4B8));
   fn_80648470(this);
   if read_f32(this, 0x1A4B8) == 0.0 {
       write_u16(this, 0x1A4EC, read_u16(this, 0x1A4EC).wrapping_sub(1) & 0x7F);
   } else if read_f32(this, 0x1A4B8) == read_f32(this, 0x1A4B0) {
       write_u16(this, 0x1A4EC, read_u16(this, 0x1A4EC).wrapping_add(1) & 0x7F);
   }
   ```

The two float comparisons at `0x80648EA0` / `0x80648EC0` are `fcmpu` (`XO10 = 0`)
- the four `f63_unk xo=0` words a naive opcode-63 table mis-reads.

### Constants read out of the image

| address | raw | value | use |
|---|---|---|---|
| `0x807C847C` | `0x41000000` | `f32 8.0` | amplitude seed (state 1) |
| `0x807C8484` | `0x3F000000` | `f32 0.5` | phase-0 halving |
| `0x807C8508` | `0x40400000` | `f32 3.0` | state-2 -> 3 threshold, state 3 sine scale |
| `0x807C850C` | `0x00000000` | `f32 0.0` | lower travel limit / offset-zero test |
| `0x807C8510` | `0x3B800000` | `f32 0.00390625` | angle scale (`1/256`) |
| `0x807C8518` | `u8[8]` | `01 01 FF FF 01 FF FF 01` | phase table (4 s8 pairs) |
| `0x807C8524` | `0x3C800000` | `f32 0.015625` | velocity scale (`1/64`) |
| `0x807C8528` | `0x3D000000` | `f32 0.03125` | slow amplitude decay |
| `0x807C852C` | `0x3E000000` | `f32 0.125` | fast amplitude decay |
| `0x807C8530` | `0x43300000` | `f64 4503601774854144` (`2^52+2^31`) | `s8 -> f32` ladder |
| `0x1A4xx` block | - | `this + 0x20000` | see field map |

### Callees

| address | name | shape |
|---|---|---|
| `0x80648470` | `FUN_80648470` | `(this)` - publishes `+0x1A49C` into `+0x4CC`, `+0x504`, the two-frame history at `0x80429DD0+0x8FEC4` and (mode) `0x800EBBA0` |
| `0x806485c0` | `fn_806485c0` | `(this, 1) -> u32` - `1` = upper limit hit, `2` = lower limit hit, `0` = still moving; posts cues `0x305`/`0x306` only when `param2 == 0` |
| `0x80237BD0` | sine (`f64`) | `(f64) -> f64` |

**Confidence: high.** All 215 words decode, the 8-byte phase table and every
float are read out of the image, and both `qr3` loads are 16-bit unsigned
(`GQR3 = 0x00050005`).

---

## 6. `0x8069C2C0` - the carried-object per-frame body (sibling of `0x8069B750`)

**Evidence.** `work/ghidra/rel-d_basesNP/export/8069c2c0_FUN_8069c2c0.c`,
852 bytes, 213 words, synthetic `0x8069C2C0`. Body `0x8069C2C0..0x8069C610`.

**What it is.** A per-frame update of the same "carried object" family that
`docs/SPEC_WAVE128_BASES.md` specified at `0x8069B750`. It re-uses that
document's `LAB_8069BB5C` tail *call set* - slot `+0x33C`/`+0x340`,
`0x8006CD90`/`0x8006CDE0`/`0x800955F0`, the two orientation pairs into `+0x114`
and `+0x11C`, the two `+0xEC` clamps, `0x80699E30` and `0x8069A230` - but the
order differs (here `0x800A4480(this, slot(+0x374))` runs *before* the clamps,
and the body has no `+0x314`/`+0x3A0`/`+0xD4` calls at all). This variant is the
shorter, collision-facing half: it drives the `+0x544` sub-object, reads the
model's two clip-idle predicates, publishes the four sector/direction flags
`+0xD30..+0xD34`, runs the two-state `+0x360` launcher timer, and then the tail.

### Signature

```rust
pub unsafe fn fn_8069c2c0(this: usize)
```

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x60` | `void*` | class vtable |
| `+0xAC`,`+0xB0`,`+0xB4` | `f32` | position |
| `+0xEC` | `f32` | vertical speed, clamped to `0.0` by the tail |
| `+0x114`,`+0x11C` | object | two orientation value holders (`0x8015F470`) |
| `+0x1EC` | object | collision/motion container |
| `+0x348` | `u8` | `(x_saved - x) < 0` |
| `+0x360` | `u32` | launch state: `0`, `1`, `2` |
| `+0x500` | `u16` | launch timer, seeded with `30` |
| `+0x544` | object | rider sub-object (slot `+0x1C` is its per-frame tick) |
| `+0x5F8` | object | model object reached through slot `+0x2DC` |
| `+0x768` | `f32` | target x used by the sector test |
| `+0xD30`..`+0xD34` | `u8` | written flags (see below) |

### Exact behaviour

Prologue `0x8069C2C0..0x8069C2E4`: `stwu r1,-0x40`; save LR, `f31` (plus
`psq_st f31,qr0,0`), `r31`, `r30`, `r29`; `r31 = this`. Epilogue
`0x8069C5F0..0x8069C610`.

1. **Rider tick** (`0x8069C2E4..0x8069C2F4`): `vcall1(this + 0x544, 0x1C)`.
2. **Clip-idle gate** (`0x8069C2F8..0x8069C34C`):
   ```rust
   let model = vcall1(this, 0x2DC);                 // *(this+0x60) + 0x2DC
   let clip  = read_u32(model, 0x5F8);
   let mut both = 1u32;
   if vcall0(clip, 0x0C) == 0 && vcall0(clip, 0x10) == 0 { both = 0; }
   if both != 0 { vcall1(this, 0x380); }
   ```
   (`r29` is the `both` flag; both virtual calls have `r3` reloaded from
   `+0x5F8` immediately before the `bctrl`.)
3. **Signed-x flag** (`0x8069C36C..0x8069C3DC`):
   ```rust
   let dx = read_f32(this, 0x768) - read_f32(this, 0xAC);
   let neg = if dx < 0.0 { 1u8 } else { 0u8 };
   write_u8(this, 0x348, neg);
   if neg == 1 { write_u8(this, 0xD31, 1); } else { write_u8(this, 0xD32, 1); }
   ```
   (`fcmpo` + `bge` at `0x8069C394`, word `0xFC040040`; `+0x348` is stored
   before the branch so it always lands.)
4. **Vertical-sector flag** (`0x8069C3E0..0x8069C40C`): `let s = fn_8069a170(this)`;
   `if s == 0 { write_u8(this, 0xD33, 1); } else if s == 2 { write_u8(this, 0xD34, 1); }`
   (`fn_8069a170` is the +/-pi/8 sector test that returns `1` inside the band,
   `0` above and `2` below.)
5. **Launch timer** (`0x8069C40C..0x8069C490`):
   ```rust
   match read_u32(this, 0x360) {
       1 => {
           let model = vcall1(this, 0x2DC);
           let clip  = read_u32(model, 0x5F8);
           if vcall0(clip, 0x0C) == 0 { write_u8(this, 0xD30, 1); }
           write_u16(this, 0x500, 30);              // 0x807F13C0 low half
           write_u32(this, 0x360, 2);
       }
       2 => { if read_u16(this, 0x500) == 0 { fn_8069aca0(this); } }
       _ => {}
   }
   ```
   Any other value falls straight through to step 6 (the `cmpwi 1` /
   `cmpwi 2` / `b` chain at `0x8069C410..0x8069C420`).
6. **Shared tail** (`0x8069C490..0x8069C5FC`):
   ```rust
   call1(0x800A4E90, this);
   vcall1(this, 0x33C); vcall1(this, 0x340);
   call1(0x8006CD90, this); call1(0x8006CDE0, this); call1(0x800955F0, this);
   let a = vcall_f64(this, 0x350) as f32;            // f31
   let b = vcall_f64(this, 0x34C) as f32;
   0x8015F470(b, a, this + 0x114);                   // r3 = dest loaded last
   let c = vcall_f64(this, 0x358) as f32;
   let d = vcall_f64(this, 0x354) as f32;
   0x8015F470(d, c, this + 0x11C);
   vcall1(this, 0x344);
   let s = vcall_s16(this, 0x374);
   call2(0x800A4480, this, s as i32);
   if call1(0x80072BC0, this + 0x1EC) != 0 && read_f32(this, 0xEC) < 0.0 {
       write_f32(this, 0xEC, 0.0);
   }
   if call1(0x8006EF40, this + 0x1EC, 0) != 0 && 0.0 < read_f32(this, 0xEC) {
       write_f32(this, 0xEC, 0.0);
   }
   call1(0x8006FA40, this + 0x1EC, 0);
   fn_80699e30(this);
   fn_8069a230(this);
   ```
   **Argument order.** `0x8015F470`'s first argument is `r3`, and `r3` is
   loaded *last* (`addi r3,r31,0x114` at `0x8069C508`). The Ghidra export prints
   the destination as the third argument; the stream wins. `0x8015F470(dest,
   slot+0x34C, slot+0x350)`.

### Constants read out of the image

| address | raw | value | use |
|---|---|---|---|
| `0x807CA08C` | `0x00000000` | `f32 0.0` | signed-x compare, `+0xEC` clamp |
| `0x807F13C0` | `0x0000001E` | `30` (low half) | `+0x500` seed |

### Callees and slots

| target | shape | note |
|---|---|---|
| `+0x544` -> `+0x1C` | `(obj)` | rider tick |
| `this` -> `+0x2DC` | `(this) -> obj` | model accessor |
| `obj+0x5F8` -> `+0x0C`,`+0x10` | `() -> u32` | two clip-idle predicates |
| `this` -> `+0x380` | `(this)` | "both clips idle" reaction |
| `this` -> `+0x33C`,`+0x340`,`+0x344` | `(this)` | per-frame phases |
| `this` -> `+0x34C`,`+0x350`,`+0x354`,`+0x358` | `(this) -> f64` | two orientation pairs |
| `this` -> `+0x374` | `(this) -> s16` | animation/index selector |
| `0x8069A170` | `fn_8069a170` | `(this) -> u32` sector test (`1` in band, `0` above, `2` below) |
| `0x8069ACA0` | `fn_8069aca0` | `(this)`; writes `+0xD84 = 10` and tail-calls slot `+0xD4(this, 0x8082B5AC)` |
| `0x800A4E90` | unexported `main.dol` gap | `(this)` |
| `0x8006CD90`,`0x8006CDE0`,`0x800955F0` | unclaimed | `(this)` |
| `0x8015F470` | `thunk_FUN_8015f480` | `(dest, f1, f2)` |
| `0x800A4480` | unclaimed | `(this, s16)` |
| `0x80072BC0`,`0x8006EF40`,`0x8006FA40` | unclaimed | `(this+0x1EC[, 0]) -> u32` |
| `0x80699E30` | `FUN_80699e30` | `(this)` |
| `0x8069A230` | `fn_8069a230` | `(this)` |

**Confidence: high.** The tail call set and every slot offset match the
already-transcribed `docs/SPEC_WAVE128_BASES.md` sequence (the order differs, and
this body omits that sibling's `+0x314`/`+0x3A0`/`+0xD4` work), and the whole
body decodes with no `unaff_GQR0` beyond the one `psq_st f31,qr0,0` spill.

---

## 7. `0x807B77E0` - the stage/world-map scene teardown

**Evidence.** `work/ghidra/rel-d_basesNP/export/807b77e0_FUN_807b77e0.c`,
852 bytes, 213 words, synthetic `0x807B77E0`. Body `0x807B77E0..0x807B7B30`.

**What it is.** The "leave the stage" teardown that pairs with the stage archive
mount `fn_807b1c20` (`docs/COVERAGE_WAVE76_BASES.md`). It unmounts the archive,
publishes the four world-map player-pane visibility bytes, clears the scene
flags, and releases eight global owner slots one by one with their per-slot
side-effects. It returns `0` when the mount gate is closed and `1` after a real
teardown.

### Signature

```rust
pub unsafe fn fn_807b77e0(self_: usize) -> u32
```

`r3` is the scene/context object. The export's `undefined4 param_1` is accurate;
the value is only used for `0x807B8EC0(this, 0)` and `0x807B7B40(this)`. This is
the one row in the set with **no `this` field offsets**: everything else it
touches is a `main.dol` BSS cell, listed under "Globals touched".

### Exact behaviour

Prologue `0x807B77E0..0x807B77F8`: `stwu r1,-0x20`; save LR, `r31`, `r30`,
`r29`; `r29 = this`. Epilogue `0x807B7B18..0x807B7B30`.

1. **Gate** (`0x807B77FC..0x807B780C`):
   `if call0(0x807B1C20) == 0 { return 0; }`.
2. **Subsystem stop** (`0x807B7810..0x807B782C`):
   `call0(0x800E38E0); call0(0x800CFC60); 0x807B8EC0(this, 0); call0(0x8005ECA0); 0x807B7B40(this);`
   `0x807B8EC0` is the 8-byte forwarder `or r3,r4,r4; b 0x801648B0`, so this is
   `0x801648B0(0)` - the second argument is what reaches the callee.
3. **Player panes** (`0x807B7830..0x807B78F0`):
   ```rust
   if read_u8(0x8042A21F) != 0 {
       let table = 0x80354E50;                       // lwzu r3,20048(r6)
       let p = read_u32(0x8042A028);
       for i in 0..4 {
           let on = read_u32(table + 4*i) != 0;
           write_u8(read_u32(p, 4 + 4*i), 0x54, on as u8);
       }
   }
   ```
   (`lwzu r3,20048(r6)` loads slot 0 and leaves `r6 = 0x80354E50`; the next
   three slots are `4(r6)`, `8(r6)`, `0xC(r6)`.)
4. **Scene flag** (`0x807B78F0..0x807B7918`):
   ```rust
   let v = read_u32(0x80315B98);
   write_u8(0x8042A21E, (v.wrapping_sub(4) as i32 < 0) as u8);   // cntlzw ladder
   ```
5. **Four pane releases** (`0x807B791C..0x807B7938`):
   `for i in 0..4 { call1(0x800DC9A0, read_u32(read_u32(0x8042A028) + 4 + 4*i)); }`
6. **Mode teardown** (`0x807B793C..0x807B7968`):
   `call0(0x80069510);` then
   `if read_u32(0x8042A204) == 2 { call0(0x80102580); }`
   `else if read_u32(0x8042A204) == 3 { call0(0x801025A0); }`.
7. **Flag clear** (`0x807B7968..0x807B798C`): `0x8042A204 = 0; 0x8042A220 = 0;
   0x8042A21F = 0;`.
8. **Eight owner releases** (`0x807B7990..0x807B7B10`), each guarded by
   `if (ptr != 0)`; the `beq`/`beq` pairs at e.g. `0x807B7990`+`0x807B7994` are
   the optimiser's duplicated guard, not a second condition:

   | slot | extra before release | release call |
   |---|---|---|
   | `0x8042A1D0` | `0x80429F30 = 0` | `0x802B9280(ptr)` |
   | `0x8042A1D4` | - | `vcall(ptr, +0x08)(ptr, 1)` |
   | `0x8042A1D8` | `0x8042A078 = 0` | `0x802B9280(ptr)` |
   | `0x8042A1DC` | `0x800660C0(ptr)`, `0x80429D30 = 0` | `0x802B9280(ptr)` |
   | `0x8042A1E0` | `0x80429D48 = 0` | `0x802B9280(ptr)` |
   | `0x8042A1E4` | - | `0x800D02E0(ptr, 1)` |
   | `0x8042A1E8` | `0x80429D88 = 0` | `0x802B9280(ptr)` |
   | `0x8042A1EC` | - | `0x8008DC90(ptr, 1)` |

   plus three unconditional receiver calls before/after the guarded ones:
   `0x800660C0(0x80429D30)` (`0x807B7A1C`), `0x800D0340(0x80429FC8)`
   (`0x807B7A90`), `0x8008DCF0(0x80429E70)` (`0x807B7AEC`). Every released slot
   is then written back as `0`; the `0x8042A1DC` arm also restores
   `0x80429D30 = 0` **after** its own `0x800660C0`.
9. **Global tail** (`0x807B7B10`): `call0(0x8006A010); return 1;`.

### Globals touched

`0x8042A21F`, `0x8042A21E`, `0x8042A204`, `0x8042A220`, `0x8042A028`
(`u32` pointer to the four-entry player pane table), `0x80354E50..0x80354E5C`
(the four world-map player-slot words), `0x80315B98` (scene word),
`0x80429F30`, `0x8042A078`, `0x80429D30`, `0x80429D48`, `0x80429D88`,
`0x8042A1D0`, `0x8042A1D4`, `0x8042A1D8`, `0x8042A1DC`, `0x8042A1E0`,
`0x8042A1E4`, `0x8042A1E8`, `0x8042A1EC`, `0x80429FC8`, `0x80429E70`.
All are `main.dol` BSS and read as zero in the shipped image.

### Callees

| address | name | shape |
|---|---|---|
| `0x807B1C20` | `fn_807b1c20` | `() -> u32` stage archive mount |
| `0x800E38E0`,`0x800CFC60`,`0x8005ECA0`,`0x80069510`,`0x80102580`,`0x801025A0`,`0x8006A010` | unclaimed | `()` |
| `0x807B8EC0` | 8-byte forwarder | `(a, b) -> 0x801648B0(b)` |
| `0x807B7B40` | `FUN_807b7b40` | `(this)` |
| `0x800DC9A0` | `fn_800dc9a0` | `(pane)` |
| `0x800660C0`,`0x800D0340`,`0x8008DCF0` | unclaimed / `fn_800d0340` | `(ptr)` |
| `0x802B9280` | `arc_heap_free_object` | `(ptr)` |
| `0x800D02E0`,`0x8008DC90` | unclaimed / `fn_8008dc90` | `(ptr, 1)` |

**Confidence: high.** All 213 words decode; the only indirection is the one
`vcall(ptr, +0x08)` on `0x8042A1D4`, whose receiver is a BSS pointer and whose
slot is an immediate.

---

## 8. `0x807BEC10` - the two-point trail/ribbon updater

**Evidence.** `work/ghidra/rel_d_basesNP/export/807bec10_FUN_807bec10.c`,
848 bytes, 212 words, synthetic `0x807BEC10`. Body `0x807BEC10..0x807BEF5C`.

**What it is.** The per-frame trail updater of a world-map object. It lerps two
trail endpoints towards a three-term position sum with a caller-supplied weight,
snaps back and releases the trail effect when the endpoint moved less than
`0.1` units, and allocates/keeps the trail effect (id `0x6D`, preset-B pool)
otherwise. The only caller is `0x807BEAF0`, which passes one of four weights:
`1.0` (`0x80821870`, `0x80821874`, `0x80821878`) or `0.3` (`0x80821558`) when
`0x807C1380(this)` is non-zero.

### Signature

```rust
pub unsafe fn fn_807bec10(this: usize, weight: *const f32)
```

The export's `float *param_2` is accurate: the only use is `f9 = *param_2` at
`0x807BEC28`.

### Field map

| offset | type | meaning |
|---:|---|---|
| `+0x4B0`,`+0x4B4`,`+0x4B8` | `f32` | trail source point A |
| `+0x4BC`,`+0x4C0`,`+0x4C4` | `f32` | trail source point B |
| `+0x4D4`,`+0x4D8`,`+0x4DC` | `f32` | trail endpoint A (lerped) |
| `+0x4E0`,`+0x4E4`,`+0x4E8` | `f32` | trail endpoint B (lerped) |
| `+0x4EC`,`+0x4F0`,`+0x4F4` | `f32` | previous endpoint B |
| `+0x6A0` | `u32` | state; `< 3` means "trail may exist" (`0x807C1470`) |
| `+0x6A4`,`+0x6A8`,`+0x6AC` | `f32` | first position offset |
| `+0x730`,`+0x734`,`+0x738` | `f32` | second position offset |
| `+0x740` | `i32` | effect handle; `< 0` = none |

### Exact behaviour

Prologue `0x807BEC10..0x807BEC90`: `stwu r1,-0x160`; save LR; `stfd`+`psq_st`
for `f20..f31` (12 pairs); `r31 = this`; `f9 = *weight`. Epilogue
`0x807BEEEC..0x807BEF5C`.

```rust
// 1. remember the previous endpoint B
write_f32(this, 0x4EC, read_f32(this, 0x4E0));
write_f32(this, 0x4F0, read_f32(this, 0x4E4));
write_f32(this, 0x4F4, read_f32(this, 0x4E8));

let t  = *weight;                       // f9
let u  = 1.0 - t;                       // f13

// 2. target A = (4B0..4B8) + (8082154C..54) + (6A4..6AC) + (730..738)
//    target B = (4BC..4C4) + the same two offsets
let a = add3(this, 0x4B0, 0x6A4, 0x730);
let b = add3(this, 0x4BC, 0x6A4, 0x730);

// 3. lerp both endpoints (single precision, order is (old * u) + (target * t))
write_f32(this, 0x4D4, read_f32(this,0x4D4)*u + a[0]*t);
write_f32(this, 0x4D8, read_f32(this,0x4D8)*u + a[1]*t);
write_f32(this, 0x4DC, read_f32(this,0x4DC)*u + a[2]*t);
write_f32(this, 0x4E0, read_f32(this,0x4E0)*u + b[0]*t);
write_f32(this, 0x4E4, read_f32(this,0x4E4)*u + b[1]*t);
write_f32(this, 0x4E8, read_f32(this,0x4E8)*u + b[2]*t);

// 4. distance moved by endpoint B  (squared distance, then sqrt)
let d2 = 0x801C1650(this + 0x4EC, this + 0x4E0);   // f64, result in f1
let d  = 0x802BE470(d2);                            // (float)sqrt wrapper

if d < 0.1 {
    // 5a. snap back and drop the trail
    write_f32(this, 0x4E0, read_f32(this, 0x4EC));
    write_f32(this, 0x4E4, read_f32(this, 0x4F0));
    write_f32(this, 0x4E8, read_f32(this, 0x4F4));
    if fn_807c1470(this) != 0 && read_i32(this, 0x740) >= 0 {
        0x801055D0(read_u32(0x8042A27C));
        write_i32(this, 0x740, -1);
    }
} else {
    // 5b. keep/replace the trail
    if fn_807c1470(this) != 0 && read_i32(this, 0x740) < 0 {
        write_i32(this, 0x740,
            0x80105440(read_u32(0x8042A27C), 0x6D, 0, this + 0x4B0, 1, 1));
    }
}
```

where `fn_807c1470(this)` is `(read_u32(this, 0x6A0) < 3)` (its own export is
`countLeadingZeros(*(int*)(p+0x6a0) + -3) >> 5`).

Note the order: `0x801C1650` runs **before** `0x802BE470`, and its `f64` return
is the argument to the square-root wrapper. The export folds the pair into one
expression; the stream is two separate `bl`s at `0x807BEE48` and `0x807BEE4C`.

### Constants read out of the image

| address | raw | value | use |
|---|---|---|---|
| `0x807D0AC8` | `0x3F800000` | `f32 1.0` | `u = 1 - t` |
| `0x807D0AF0` | `0x3DCCCCCD` | `f32 0.1` | movement threshold |
| `0x8082154C`,`0x80821550`,`0x80821554` | `0x00000000` | `f32 0.0` | three-term position offset (REL `.data`, read as zero) |
| `0x80821558` | `0x3E99999A` | `f32 0.3` | caller-supplied lag weight |
| `0x80821870`,`0x80821874`,`0x80821878` | `0x3F800000` | `f32 1.0` | caller-supplied snap weights |
| `0x8042A27C` | BSS | `u32` pointer | world-map effect pool |

### Callees

| address | name | shape |
|---|---|---|
| `0x801C1650` | `fn_801c1650` | `(*f32, *f32) -> f64` squared distance |
| `0x802BE470` | `fn_802be470` | `(f64) -> f32` square-root wrapper |
| `0x807C1470` | `FUN_807c1470` | `(this) -> u32` = `(+0x6A0 < 3)` |
| `0x80105440` | `wm_effect_pool_alloc_preset_b` | `(pool, 0x6D, 0, *f32, 1, 1) -> i32` |
| `0x801055D0` | `fn_801055d0` | `(pool)` clears one pool entry's active byte |

**Confidence: medium.** The arithmetic, the six stores and the two branch
conditions are all literal, and the body's 36 `stfs` temporaries are dead
(`r1+8..r1+0x94`, never reloaded - the only stack float loads in the whole body
are the twelve `lfd` restores of `f20..f31`), so they exist only because the
source used three-component vector temporaries. The one inference is that
`0x802BE470` is the square-root wrapper: its export is
`(float)thunk_FUN_802e8620()` with the argument in `f1`, and the only value in
`f1` at `0x807BEE4C` is `0x801C1650`'s squared distance, which makes the
following `0.1` test a distance (not an area) test. `0x801C1650`'s own export
confirms it returns `(dx*dx + dy*dy + dz*dz)`.

---

## Next eight

Re-applying the filter with these eight headings registered:

```
0x8069BE20 844 | 0x807373B0 844 | 0x80705C60 840 | 0x80782F60 836
0x80775190 820 | 0x8079A930 816 | 0x807A7220 816 | 0x807B84C0 812
```

The 844-byte tie is broken by address ascending.

## Verification

* Every one of the eight is absent from `ledger/functions.csv` (exact `address`
  match) and from every `#{1,6} ` heading in `docs/*.md`, in both the
  `0x`-prefixed and the bare eight-hex spellings.
* Every `bytes` value equals the `size` column of
  `work/ghidra/rel-d_basesNP/export/functions.tsv`, and every named export file
  exists under `work/ghidra/rel-d_basesNP/export/`.
* The final word of each body is `blr` at `address + size - 4`; 0 of 1,733
  instructions is undecodable.
* Every literal address quoted above was read out of
  `work/rels/elf/d_basesNP.elf` through the documented
  `file_offset = (address - 0x80600000) + 0x1000` mapping, and every
  `main.dol`/REL BSS cell is marked `BSS` rather than given a value.
* No file in the repository was written except this report, and no merge or
  check tool was run.
