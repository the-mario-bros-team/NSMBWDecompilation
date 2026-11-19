# Wave 134 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses: 173 class **B** (reconstructible now), 160 class C and
51 class D.

Waves 128 - 133 (`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE133_ENEMIES.md`)
specified **72** of those B rows in wave-127 rank order: ranks 1-60 by the five
12-row tranches, then ranks 62-74 (rank 65, `0x80A07080`, was already a
`translated` ledger row and was skipped).

This lane is the continuation: **ranks 75-86** of the same ordering. That is the
first twelve B rows below rank 74's 52 reachable instructions.

It is **read-only**. It writes no Rust, no ledger row and no other doc. Each body
was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000` segment shift
of `docs/REL_ELF_MAPPING.md`, cross-checked against the Ghidra exports under
`work/ghidra/rel-d_enemiesNP/export/` and against
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Every in-REL constant below
was read out of the ELF itself, so the quoted float values are the shipped ones.

All twelve were checked against the **address column** of
`ledger/functions.csv` (all 16,350 rows: absent), and against every `##` heading
in every `docs/*.md` (absent). Three of them are *named* inside other waves'
prose but were never specified and are not ledger rows; see
"Cross-references and collisions" at the end.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then by ascending address.

The method was re-validated on both immediately preceding tranches before this
page was written. Walking the twelve wave-132 addresses reproduces all twelve
published counts exactly (`71, 70, 69, 69, 64, 62, 62, 61, 61, 60, 60, 58`) and
walking the twelve wave-133 addresses reproduces (in its table order)
`57, 57, 57, 56, 56, 56, 54, 53, 52, 52, 52, 52`. This page picks up at 47: the
next count below wave 133's 52.

There is a real gap in the pool between 52 and 47 - the 48/49/50/51-count B rows
are all either ledger rows or already specified - so rank 75 starts at 47.

| rank | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 75 | `0x80A79750` | 188 | 47 | 4-frame flag + `EA9` countdown + SE at the actor | **ready** |
| 76 | `0x80AFF0D0` | 184 | 46 | `sanbo` model + `sanbo_body`/`sanbo_head` binder | **ready** |
| 77 | `0x80A3F8F0` | 176 | 44 | `+0xAC += 4.0 * f398`, SE below `0.1` | **ready** |
| 78 | `0x80A7ED20` | 176 | 44 | sub-object reset + `0x2D9` effect at the actor | **ready** |
| 79 | `0x80B19FA0` | 176 | 44 | 4-player probe, two-arm dispatch on the class word | **ready** |
| 80 | `0x80A46BF0` | 172 | 43 | spin step: `+0xAC += fAE0 * f398`, angle ramp | **ready** |
| 81 | `0x80B27980` | 168 | 42 | per-player contact record (three deltas + byte) | **ready** |
| 82 | `0x80B0CFD0` | 160 | 40 | six sub-object vtable resets | **ready** |
| 83 | `0x80B0DB90` | 156 | 39 | 300-frame retry arm reset | **ready** |
| 84 | `0x80A369B0` | 152 | 38 | side-indexed heading step + `+0x102` clamp | **conditional** |
| 85 | `0x80A2F910` | 136 | 34 | node-name match after the last `:` | **ready** |
| 86 | `0x80ADE3A0` | 136 | 34 | `block_pata` step: `+0xAC` delta sets `+0x348` | **ready** |

**Split: 11 ready, 1 conditional.**

"Conditional" is used exactly as in waves 128-133: a named datum in `main.dol`
that is **consumed arithmetically or used as an index** and must be read before
a byte-exact port. Rank 84 is that case - it indexes two `s16` tables at
`0x8042C1A8`/`0x8042C1A4` and adds the result to an angle. Every other external
address in this set is (a) a `main.dol` callee called by address, (b) a pointer
or state token that is only dereferenced, stored or passed as an argument, or
(c) a fixed low-RAM word read through a base register that is literally zero on a
path the game cannot reach (ranks 77, 80; see the notes).

The four risk classes that made waves 110-126 skip a body are all absent:

* **paired-single traffic is spill-only.** Exactly one `psq_st`/`psq_l` pair
  exists in the whole tranche (`f31`, quantiser `qr0`,
  `0x80ADE3B0`/`0x80ADE410`, rank 86) and it is a plain stack save/restore. No
  `psq` op anywhere else, no `GQR1..GQR7`.
* **no `bctr` jump table.** All 19 conditional branches in the tranche target an
  address inside their own body (checked mechanically: zero out-of-body `bc`
  targets). There are 11 `bctrl` in total, all vtable dispatches with a resolved
  `+0xNN` slot.
* **no incoming float argument.** No body reads `f1`/`f2`/`f3` before writing it.
* **no unledgered callee landmine.** Every in-REL `bl` target is a
  `functions.tsv` entry: `0x80A7E630`, `0x80B19CB0`, `0x80B19EE0`,
  `0x80B19F50`, `0x80B0CD80`, `0x80B0D070`, `0x80ADDAC0`.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`). The image ends at `0x80B6E434`;
  anything at or above that is a `main.dol` address.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* An address that does not resolve in `d_enemiesNP.elf` is a `main.dol`
  address. It is marked **(ext)** and must be treated as a named external datum
  or callee, not as a missing base.
* In-REL float constants are quoted with their shipped value; they were read at
  `(addr - 0x80A00000) + 0x1000` and are `f32` unless marked `f64`.
* Branch mnemonics are the C form after the exact `bc` decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 12,0` = `blt`, `bc 4,0` = `bge`.
* `addi rX,r0,imm` is `li rX,imm` (`r0` is not read when `RA == 0`).
* All twelve bodies are exactly `TSV size / 4` instructions with no trailing
  unreachable words, and every one ends in a single `blr` (no tail `b`). The
  "ends at" address below is the address **of** that `blr`.
* `lwz rX,4(rY)` in rank 75 is a **pointer load**, not a payload word: the
  incoming objects of that callback are holders whose `+4` points at the actor
  state. Nothing in the body touches the holders themselves.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv` evidence column) or the helper
table of waves 128-133, not new claims.

| address | role |
|---|---|
| `0x8005F900` | player object by index (table A) |
| `0x80064BD0` | finaliser run right after `0x8016A3A0` in this family (not a ledger row) |
| `0x80066630` | SE / spatial-audio helper used next to `0x80198C30` (not a ledger row) |
| `0x800690C0` | reset a model holder `(holder, -1, resetWord, 0, 0x20)` |
| `0x800690E0` | finish a model holder |
| `0x8006A4B0` | copy a `Vec3` from `r4` into the caller's buffer `r3` (not a ledger row) |
| `0x80096240` | probe `0x80064790` on the x/y pair; returns the side/direction byte |
| `0x800DF180` | load a model by name `(archive, name, path)` |
| `0x801336E0` | state-3 + sound `0x34` trigger taking a `Vec3`; returns 1 on the firing path |
| `0x80162D00` | class-list lookup; `id & 7` selects a `0x20`-byte bucket |
| `0x80162D50` | `u16` payload key lookup in list `0x80377A10` |
| `0x80169020` | pad/animation state query (`+0x2C` / `+0x28 - 1.0` by mode byte), returns 0/1 |
| `0x8016A3A0` | build a `0x20`-byte model/animation record into `r3` |
| `0x80197F00` | player-SE spawn (twin of `0x80199390` using the `0x52B8`/`0x52B4` level pair) |
| `0x80199390` | `0x14`-row scan / effect-slot spawn, tag `+0x68` |
| `0x80239E30` | find a model node by name inside an archive (`3DModels(NW4R)`) |
| `0x802DE514` | branchless `abs` (here applied to `s16` angles) |
| `0x802E1AB4` | `strcmp` (aligned word path collapsed to bytes) |
| `0x802E1C40` | `strrchr` (last match accumulator) |

---

## 1. rank 75 - `0x80A79750`, 4-frame flag + `EA9` countdown + SE

* **TSV size:** 188 bytes
* **Reachable body:** 47 instructions; 1 `bl` + 0 `bctrl`; ends at
  `0x80A79808 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a79750_FUN_80a79750.c`
* **Proposed signature:** `pub unsafe fn fn_80a79750(this: usize, other: usize)`
* **Callers in this REL:** `0x80A79950`, `0x80A7B72C`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; no GPR saves. Locals: `+0x08` = high word of
the integer-to-double bias (`0x43300000`), `+0x0C` = its low word (`4`), `+0x10`
= `fctiwz` result. `r5 = *(void **)(this + 4)`, `r6 = *(void **)(other + 4)`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2E050` | `f32` | `4.0` - added to the converted word before the `fctiwz` |
| `0x80B2E058` | `f64` | `2^52` (`0x4330000000000000`) - the integer-to-double bias |
| `0x80429D40` | `u32` | **(ext)** sound-manager pointer, passed as `r3` of `0x80066630` |

### Behaviour

```c
u8 *a = *(u8 **)(this  + 4);          // 0x80A7975C
u8 *b = *(u8 **)(other + 4);          // 0x80A79760

if (a->bE9F != 0) return;             // 0x80A79764 lbz / cmpwi / bne 0x80A797FC

a->bE9A  = 1;                         // 0x80A79798
a->bE9E  = 4;                         // 0x80A797A0
a->bE9F  = (u8)(4.0f + (f32)((f64)0x4330000000000004 - (f64)2^52));  // = 4
a->bEAA  = a->bEA9;                   // 0x80A797B8 / 0x80A797BC  (copy first)
if (a->bEA9 == 0) return;             // 0x80A797C0 cmpwi / beq 0x80A797FC
a->bEA9 -= 1;                         // 0x80A797C8 / 0x80A797CC

fn_80066630(*(u32 *)0x80429D40,       // 0x80A797E0
            a + 0x61C,                // 0x80A797D4, r4
            a->f61C < b->fAC,         // fcmpo + mfcr + srwi r5,r0,31, r5
            1, 0);                    // r6 = 1, r7 = 0
```

The `+0xE9F` store is the only subtle part. The stream builds
`(f64)2^52 + 4` on the stack (`lis r0,0x4330` / `stw r0,8(r1)` / `li r4,4` /
`stw r4,12(r1)` / `lfd f1,8(r1)`), subtracts the shipped `f64` `2^52`
(`lfd f2` from `0x80B2E058`) and does it with **`fsubs`** (`0x80A7979C`), not
`fsub`. In single precision `(f32)(2^52 + 4)` rounds back to `2^52`, so the
difference is `0.0`; `4.0 + 0.0` is `4.0` and `fctiwz` stores **4**. A port that
re-derives the expression as `4 + (float)(u32)4` - which is what the bias idiom
would give if the subtraction were done in double - gets 8 and is wrong. Both
Ghidra's fold and the single-precision arithmetic agree on 4, so the shipped byte
is 4.

The `EA9`/`EAA` pair is a two-byte countdown: `EAA` always receives the value
`EA9` had at the top of the frame, and `EA9` is decremented only when it was
non-zero. Both branches at `0x80A7976C` and `0x80A797C4` land on the epilogue.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A797F8` | `0x80066630` | `r3 = *(u32 *)0x80429D40`, `r4 = a + 0x61C`, `r5 = (a->f61C < b->fAC)`, `r6 = 1`, `r7 = 0` |

The predicate is `mfcr` + `srwi r5,r0,31` (`0x80A797F0`), i.e. the **CR0.LT**
bit of the `fcmpo` at `0x80A797E4`, not a general truth value. Reproduce it as
`(*(f32*)(a+0x61C) < *(f32*)(b+0xAC)) as i32`.

### Ambiguity

The incoming objects are only ever dereferenced at `+4`; nothing in the body
touches `this` or `other` themselves beyond that. Nothing external is consumed
arithmetically, so the row stays ready.

---

## 2. rank 76 - `0x80AFF0D0`, `sanbo` model binder

* **TSV size:** 184 bytes
* **Reachable body:** 46 instructions; 6 `bl` + 0 `bctrl`; ends at
  `0x80AFF184 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aff0d0_FUN_80aff0d0.c`
* **Proposed signature:** `pub unsafe fn fn_80aff0d0(this: usize)`
* **Callers in this REL:** `0x80AFF054`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C` = `0x80B65650` (string block
base), `r30@+0x18` = `this`. One 4-byte local at `+0x08` (the node handle).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80377C48` | `u32` | **(ext)** reset word passed as `r5` of `0x800690C0` |
| `0x8042A038` | `u32` | **(ext)** current archive pointer; `+4` is the archive argument |
| `0x80B65650` | - | base of the string block loaded into `r31` |
| `0x80B6565C` | `char[]` | `"sanbo_body"` - node name for side 0 |
| `0x80B65668` | `char[]` | `"sanbo_head"` - node name for side 1 |
| `0x80B65678` | `char *[2]` | `{0x80B6565C, 0x80B65668}` node-name table |
| `0x80B65680` | `char[]` | `"g3d/sanbo.brres"` - model path |
| `0x80B65690` | `char[]` | `"sanbo"` - model name |

The two table entries were read out of the image: `0x80B65678` holds
`0x80B6565C` and `0x80B6567C` holds `0x80B65668`.

### Behaviour

```c
fn_800690C0(this + 0x524,                 // 0x80AFF0FC, r3
            -1,                           // 0x80AFF104, r4
            *(u32 *)0x80377C48,           // 0x80AFF100, r5
            0,                            // 0x80AFF0DC, r6
            0x20);                        // 0x80AFF0E4, r7

*(u32 *)(this + 0x540) =                  // 0x80AFF12C
    fn_800DF180(*(u32 *)0x8042A038 + 4,   // 0x80AFF114 / 0x80AFF11C, r3
                "sanbo",                  // 0x80B65690, r4
                "g3d/sanbo.brres");       // 0x80B65680, r5

u32 node = *(u32 *)(0x80B65678 + (((*(u32 *)(this + 4)) << 2) & 4));  // 0x80AFF124..0x80AFF138
u32 h    = fn_80239E30(this + 0x540, node);                            // 0x80AFF13C
u32 local = h;                                                         // stw r3,8(r1)

fn_8016A3A0(this + 0x544, &local, this + 0x524, 0x20, 1, 0);           // 0x80AFF15C
fn_80064BD0(this + 0x544);                                             // 0x80AFF164
fn_800690E0(this + 0x524);                                             // 0x80AFF16C
```

The node selector is `rlwinm r0,r0,2,29,29` (`0x80AFF134`), i.e.
`(*(u32 *)(this+4) & 1) * 4` - the low bit of the word at `this+4` picks
`"sanbo_body"` (bit clear) or `"sanbo_head"` (bit set).

Note that the model handle is stored at `+0x540` **before** the address
`this+0x540` is passed to `0x80239E30` (`0x80AFF12C` then `0x80AFF130`), and
that `0x8016A3A0` receives `this+0x524` (the holder) as its third argument while
its destination is `this+0x544`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AFF108` | `0x800690C0` | `this+0x524`, `-1`, `*(u32*)0x80377C48`, `0`, `0x20` |
| `0x80AFF120` | `0x800DF180` | `*(u32*)0x8042A038 + 4`, `"sanbo"`, `"g3d/sanbo.brres"` |
| `0x80AFF13C` | `0x80239E30` | `this+0x540`, `node` |
| `0x80AFF15C` | `0x8016A3A0` | `this+0x544`, `&local`, `this+0x524`, `0x20`, `1`, `0` |
| `0x80AFF164` | `0x80064BD0` | `this+0x544` |
| `0x80AFF16C` | `0x800690E0` | `this+0x524` |

### Ambiguity

`0x80377C48` and `0x8042A038` are only dereferenced/stored, never consumed
arithmetically, so the row stays ready. The `(this+4) & 1` selector is the
per-side model variant used by this family; a port needs the two node names
(quoted above) and nothing else.

---

## 3. rank 77 - `0x80A3F8F0`, `+0xAC += 4.0 * f398`, SE below `0.1`

* **TSV size:** 176 bytes
* **Reachable body:** 44 instructions; 1 `bl` + 1 `bctrl`; ends at
  `0x80A3F99C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a3f8f0_FUN_80a3f8f0.c`
* **Proposed signature:** `pub unsafe fn fn_80a3f8f0(this: usize)`
* **Callers in this REL:** `0x80A3EC2C`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `this`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C56C` | `f32` | `4.0` - the speed multiplier |
| `0x80B2C570` | `f32` | `0.1` - the SE threshold on `|delta|` |
| `0x398` | `f32` | absolute low-RAM word (below both images), read on the null arm |
| `0x80B70728` | `char[]` | **(ext)** token passed as `r4` of vtable slot `+0xD4` |

### Behaviour

```c
u32 p = fn_80162D50(0x79, 0);          // 0x80A3F908 / 0x80A3F90C

f32 v;
if (p != 0) v = *(f32 *)(p + 0x398);   // 0x80A3F91C
else        v = *(f32 *)0x398;         // 0x80A3F940, base register is r3 == 0
v *= 4.0f;                             // 0x80A3F924 / 0x80A3F948

f32 a = fabsf(v);                      // 0x80A3F94C  (fabs + frsp)
*(f32 *)(this + 0xAC) += v;            // 0x80A3F958 / 0x80A3F964

if (a < 0.1f)                          // 0x80A3F968 fcmpo / 0x80A3F96C bge skip
    (*(void (**)(usize, const char *))(*(u32 *)(this + 0x60) + 0xD4))(this, (const char *)0x80B70728);
```

Two blocks are duplicated source-level: the multiplier is emitted twice
(`0x80A3F918..0x80A3F924` for `p != 0`, `0x80A3F93C..0x80A3F948` for `p == 0`),
and `0x80A3F928..0x80A3F938` is a **dead** `fabs`/`frsp`/`fcmpo`/`bge`/`b` pair
because both arms of the branch land on `0x80A3F94C` (Ghidra reports
"Removing unreachable block (ram,0x80a3f938)"). It does not change any value and
a port may drop it.

The null arm is a genuine dangling read: on that path `r3` is the zero returned
by `0x80162D50`, so `lfs f1,0x398(r3)` addresses `0x398` exactly, matching
Ghidra's `fRam00000398`. Wave 127 called this out as the reason the row is class
B ("`fRam00000398` is absolute, not unresolved"). `0x80162D50(0x79, 0)` resolves
the current base actor and does not return 0 in play, so the arm is dead; keep it
as `p ? p->f398 : *(f32*)0x398` (or fold it to `0.0f` if the port accepts that
the path is unreachable).

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A3F90C` | `0x80162D50` | `r3 = 0x79`, `r4 = 0` |
| `0x80A3F988` | `*(u32*)(this+0x60) + 0xD4` (`bctrl`) | `r3 = this`, `r4 = 0x80B70728` |

### Ambiguity

`0x80B70728` is a `main.dol` token passed straight through as `r4`; the body
never reads it. The only arithmetic datum is the in-REL `4.0`, so the row stays
ready.

---

## 4. rank 78 - `0x80A7ED20`, sub-object reset + `0x2D9` effect

* **TSV size:** 176 bytes
* **Reachable body:** 44 instructions; 4 `bl` + 2 `bctrl`; ends at
  `0x80A7EDCC blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7ed20_FUN_80a7ed20.c`
* **Proposed signature:** `pub unsafe fn fn_80a7ed20(this: usize)`
* **Callers in this REL:** none - reached through a callback/vtable slot
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C`, `r30@+0x18`, `r29@+0x14`
(`r29 = this`). One 12-byte `Vec3` local at `+0x08`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80429D60` | `u32` | **(ext)** message/effect singleton, passed as `r3` of `0x80199390` |
| `0x80B73378` | `char[]` | **(ext)** token passed as `r4` of vtable slot `+0xD4` |
| `0x2D9` | `u32` | effect id (literal `li r4,0x2D9` at `0x80A7EDA0`) |

### Behaviour

```c
(*(void (**)(usize))(*(u32 *)(this + 0x548) + 0x1C))(this + 0x548);   // 0x80A7ED3C..0x80A7ED4C
if (fn_80169020(this + 0x588) != 0)                                     // 0x80A7ED54 / 0x80A7ED5C
    (*(void (**)(usize, const char *))(*(u32 *)(this + 0x60) + 0xD4))(this, (const char *)0x80B73378);

u32 owner = *(u32 *)this;              // 0x80A7ED80  lwz r31,0(r29)
Vec3 pos;
fn_8006A4B0(&pos, this + 0xAC);        // 0x80A7ED90
fn_80199390(*(u32 *)0x80429D60, 0x2D9, owner, &pos, 0);   // 0x80A7EDA8
fn_80A7E630(this);                     // 0x80A7EDB0, in-REL, TSV row
```

The first dispatch reads the vtable word stored **inside** the sub-object at
`+0x548` (offset 0 of that sub-object) and calls slot `+0x1C` with
`r3 = this + 0x548`. The second dispatch uses the actor vtable at `+0x60`
(slot `+0xD4`), the same slot/token shape as rank 77.

The effect call's third argument is the word at `this + 0`, loaded at
`0x80A7ED80` into `r31` - it is the actor's own id/handle, not an address of
`this`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A7ED4C` | `*(u32*)(this+0x548) + 0x1C` (`bctrl`) | `r3 = this + 0x548` |
| `0x80A7ED54` | `0x80169020` | `r3 = this + 0x588` |
| `0x80A7ED78` | `*(u32*)(this+0x60) + 0xD4` (`bctrl`) | `r3 = this`, `r4 = 0x80B73378` |
| `0x80A7ED90` | `0x8006A4B0` | `r3 = &pos`, `r4 = this + 0xAC` |
| `0x80A7EDA8` | `0x80199390` | `*(u32*)0x80429D60`, `0x2D9`, `*(u32*)this`, `&pos`, `0` |
| `0x80A7EDB0` | `0x80A7E630` | `r3 = this` |

### Ambiguity

`0x80A7E630` is an 8-byte TSV row (class B, `stream runs 2 insns to b` - it is
a one-instruction tail) that is **not** part of this tranche and is not a ledger
row; it must be ported alongside or stubbed with its tail target. The exported
`0x80B73378` and `0x80429D60` are pass-throughs only, so the row stays ready.

---

## 5. rank 79 - `0x80B19FA0`, 4-player probe + two-arm dispatch

* **TSV size:** 176 bytes
* **Reachable body:** 44 instructions; 5 `bl` + 0 `bctrl`; ends at
  `0x80B1A04C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b19fa0_FUN_80b19fa0.c`
* **Proposed signature:** `pub unsafe fn fn_80b19fa0(this: usize)`
* **Callers in this REL:** none - reached through a callback/vtable slot
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C` (found flag), `r30@+0x18`
(class word), `r29@+0x14` (loop index), `r28@+0x10` (`this`).

### Constants

None. The only immediates are the loop bound `4` (`0x80B1A004`) and the field
offset `+0x524` (`0x80B19FC4`).

### Behaviour

```c
u32 cls = fn_80162D00(*(u32 *)(this + 0x524));   // 0x80B19FC4, note: the *word*, not the address
u32 found = 0;
for (s32 i = 0; i != 4; ++i) {                   // 0x80B19FD0 / 0x80B1A000..0x80B1A008
    u32 p = fn_8005F900(i);                      // 0x80B19FD8
    if (p != 0 && fn_80B19CB0(this, p) != 0) {   // 0x80B19FE4 / 0x80B19FF4
        found = 1;                               // 0x80B19FF8
        break;                                   // 0x80B19FFC b 0x80B1A00C
    }
}
if (found) fn_80B19EE0(this, cls);               // 0x80B1A01C
else       fn_80B19F50(this, cls);               // 0x80B1A02C
```

Details that matter for a byte-exact port:

* the loop counter is a **signed int** and the guard is `blt` after
  `addi r29,r29,1 / cmpwi r29,4` (`0x80B1A000..0x80B1A008`), i.e. `i != 4`;
* `0x8005F900` is called with only `r3` set - `r4` holds whatever the previous
  callee left there and is not an argument;
* the callee result is moved to `r4` at `0x80B19FE0` **before** the `beq`, so the
  second `bl` always sees `r4 = player`;
* both "give up" branches (`0x80B19FE4`, `0x80B19FF4`) jump to the same
  `0x80B1A000` increment, and the found path leaves the loop from
  `0x80B19FFC` with `found = 1`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B19FC8` | `0x80162D00` | `r3 = *(u32*)(this+0x524)` |
| `0x80B19FD8` | `0x8005F900` | `r3 = i` |
| `0x80B19FEC` | `0x80B19CB0` | `r3 = this`, `r4 = player` (in-REL TSV row) |
| `0x80B1A01C` | `0x80B19EE0` | `r3 = this`, `r4 = cls` (in-REL TSV row) |
| `0x80B1A02C` | `0x80B19F50` | `r3 = this`, `r4 = cls` (in-REL TSV row) |

### Ambiguity

The three in-REL callees are TSV rows in their own right and are not part of this
tranche; all three are already `translated` ledger rows (`fn_80b19cb0`,
`fn_80b19ee0`, `fn_80b19f50`), so they only need to be called. Nothing external
is consumed arithmetically, so the row stays ready.

---

## 6. rank 80 - `0x80A46BF0`, spin step with an angle ramp

* **TSV size:** 172 bytes
* **Reachable body:** 43 instructions; 1 `bl` + 0 `bctrl`; ends at
  `0x80A46C98 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a46bf0_FUN_80a46bf0.c`
* **Proposed signature:** `pub unsafe fn fn_80a46bf0(this: usize)`
* **Callers in this REL:** `0x80A46CE8`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C` = `this`; `fctiwz` result at
`+0x08..+0x0C`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C7E8` | `f32` | `1024.0` - the angle-ramp scale |
| `0x80B2C7EC` | `f32` | `2.0` - the ramp bias added to `f394` |
| `0x398` / `0x394` | `f32` | absolute low-RAM words (below both images), read on the null arm |

### Behaviour

```c
u32 p = fn_80162D50(0x79, 0);                       // 0x80A46C08 / 0x80A46C0C

f32 speed = *(f32 *)(this + 0xAE0) *
            (p ? *(f32 *)(p + 0x398) : *(f32 *)0x398);   // 0x80A46C1C..0x80A46C40

*(f32 *)(this + 0xAC) += speed;                     // 0x80A46C54 / 0x80A46C60
*(f32 *)(this + 0xE8)  = speed;                     // 0x80A46C6C

f32 ramp = 2.0f + (p ? *(f32 *)(p + 0x394) : *(f32 *)0x394);   // 0x80A46C4C / 0x80A46C64 / 0x80A46C68
*(s16 *)(this + 0x102) += (s16)(int)(1024.0f * ramp);          // 0x80A46C70..0x80A46C84
```

Reading order in the stream is: `fAC` delta first (`0x80A46C44..0x80A46C60`),
then `fE8 = speed` (`0x80A46C6C`), then the `+0x102` update
(`0x80A46C70..0x80A46C84`). The `f394`/`f398` reads are not in the same place:
the `f398` read is duplicated inside each arm (`0x80A46C18`, `0x80A46C38`), while
the `f394` read happens once after the join (`0x80A46C64`). On the null arm `r3`
is still the zero the lookup returned, so that load addresses `0x394` - the same
dangling-read shape as rank 77.

`0x80A46C24..0x80A46C34` is a dead `fabs`/`frsp`/`fcmpo`/`bge`/`b` pair (both
arms land on `0x80A46C44`); it can be dropped.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A46C0C` | `0x80162D50` | `r3 = 0x79`, `r4 = 0` |

### Ambiguity

`0x80162D50(0x79, 0)` is the same "current base actor" lookup as rank 77, and
the two null arms are equally unreachable. Wave 127 rated this row B on exactly
that reading ("the stream really does `lfs f1,0x398(r0)`"). Keep the loads as
written.

Note: the ledger evidence of row `80a46cc0` says "spin step `0x80A46BF0` is not
translated", but `0x80A46BF0` is **not** an entry in the address column. Do not
mistake that prose for a claim.

---

## 7. rank 81 - `0x80B27980`, per-player contact record

* **TSV size:** 168 bytes
* **Reachable body:** 42 instructions; 2 `bl` + 0 `bctrl`; ends at
  `0x80B27A24 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b27980_FUN_80b27980.c`
* **Proposed signature:** `pub unsafe fn fn_80b27980(this: usize, player: u8)`
* **Callers in this REL:** `0x80B27A74`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C` (player object),
`r30@+0x18` (the raw parameter), `r29@+0x14` (`this`).

### Constants

None in the image. The offsets used are
`+0x744 / +0x754 / +0x764` (three `f32` arrays, stride 4),
`+0x8C0 / +0x8C4` (two `u8` arrays, stride 1), `other+0x1035`, `this+0xAC`.

### Behaviour

```c
s8  idx = (s8)player;                       // 0x80B279A0 extsb r3,r4
u32 o   = fn_8005F900(idx);                 // 0x80B279A4
if (fn_801336E0(o, this + 0xAC) == 0) return;   // 0x80B279B0 / 0x80B279B8

s32 i = (s8)player;                         // 0x80B279C0 extsb r0,r30
*(f32 *)(this + 0x744 + 4 * i) = *(f32 *)(o + 0xAC) - *(f32 *)(this + 0xAC);  // 0x80B279BC..DC
*(f32 *)(this + 0x754 + 4 * i) = *(f32 *)(o + 0xB0) - *(f32 *)(this + 0xB0);  // 0x80B279E0..EC
*(f32 *)(this + 0x764 + 4 * i) = *(f32 *)(o + 0xB4) - *(f32 *)(this + 0xB4);  // 0x80B279F0..FC
*(u8  *)(this + 0x8C0 + i)     = *(u8  *)(o + 0x1035);                        // 0x80B27A00..04
*(u8  *)(this + 0x8C4 + i)     = 1;                                           // 0x80B27A08
```

The parameter is sign-extended twice (`0x80B279A0` for the player lookup,
`0x80B279C0` for the array index) and is used **signed** for the index too, so a
negative value indexes below the array. `slwi r3,r0,2` at `0x80B279C8` is the
`*4` stride, and the byte stores use `add r3,r29,r0` (`0x80B279D4`) without a
scale.

`0x801336E0` is called with `r3 = o` (the player object) and `r4 = this + 0xAC`,
i.e. the *player* is asked to enter its state-3 mode at **my** position; the
per-player record is only written when it fires.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B279A4` | `0x8005F900` | `r3 = (s8)player` |
| `0x80B279B0` | `0x801336E0` | `r3 = player object`, `r4 = this + 0xAC` |

### Ambiguity

Nothing external. The three `f32` reads at `o+0xAC..+0xB4` assume the player
object layout used by every other body in this module; the deltas are stored, not
compared, so the exact consumer does not affect this body.

---

## 8. rank 82 - `0x80B0CFD0`, six sub-object vtable resets

* **TSV size:** 160 bytes
* **Reachable body:** 40 instructions; 0 `bl` + 6 `bctrl`; ends at
  `0x80B0D06C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0cfd0_FUN_80b0cfd0.c`
* **Proposed signature:** `pub unsafe fn fn_80b0cfd0(this: usize)`
* **Callers in this REL:** `0x80B0DD64` (inside `0x80B0DC40`, the wave-131 row),
  `0x80B0DEAC`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `this`. No locals, no
constants, no conditional branches - six straight-line dispatches.

### Constants

| address | type | value / role |
|---|---|---|
| `+0x588` | slot | sub-object 1, vtable word at `+0x588`, slot `+0x1C` |
| `+0x6B4` | slot | sub-object 2, slot `+0x14` |
| `+0x73C` | slot | sub-object 3, slot `+0x14` |
| `+0x5C8` | slot | sub-object 4, slot `+0x1C` |
| `+0x6E0` | slot | sub-object 5, slot `+0x14` |
| `+0x774` | slot | sub-object 6, slot `+0x14` |

### Behaviour

```c
(*(void (**)(usize))(*(u32 *)(this + 0x588) + 0x1C))(this + 0x588);   // 0x80B0CFE4..0x80B0CFF4
(*(void (**)(usize))(*(u32 *)(this + 0x6B4) + 0x14))(this + 0x6B4);   // 0x80B0CFF8..0x80B0D008
(*(void (**)(usize))(*(u32 *)(this + 0x73C) + 0x14))(this + 0x73C);   // 0x80B0D00C..0x80B0D01C
(*(void (**)(usize))(*(u32 *)(this + 0x5C8) + 0x1C))(this + 0x5C8);   // 0x80B0D020..0x80B0D030
(*(void (**)(usize))(*(u32 *)(this + 0x6E0) + 0x14))(this + 0x6E0);   // 0x80B0D034..0x80B0D044
(*(void (**)(usize))(*(u32 *)(this + 0x774) + 0x14))(this + 0x774);   // 0x80B0D048..0x80B0D058
```

Each dispatch follows the identical four-instruction shape
`lwz r12,off(r31)` / `lwz r12,slot(r12)` / `mtspr ctr,r12` / `addi r3,r31,off`
+ `bctrl`, so the vtable word lives at offset 0 of each sub-object and the
receiver is the sub-object's own address. No arguments beyond `r3` are set: the
callees see whatever `r4..r7` the caller of this body left behind.

### Call sites

| site | slot |
|---|---|
| `0x80B0CFF4` | `+0x588`, `+0x1C` |
| `0x80B0D008` | `+0x6B4`, `+0x14` |
| `0x80B0D01C` | `+0x73C`, `+0x14` |
| `0x80B0D030` | `+0x5C8`, `+0x1C` |
| `0x80B0D044` | `+0x6E0`, `+0x14` |
| `0x80B0D058` | `+0x774`, `+0x14` |

### Ambiguity

None beyond the vtable slots, all of which are literal. Every dispatch sets only
`r3`, so the `+0x1C` and `+0x14` slots are both argument-free here and this body
alone does not distinguish their roles. It is the reset helper that wave 131's
`0x80B0DC40` body calls at the end of its state change (`0x80B0DD64`), and its
sibling `0x80B0D070` sets `+0x674`/`+0x770`/`+0x6AC`/`+0x7A8` and calls
`0x80166F40` on `+0x6B4`/`+0x6E0`, so the six sub-objects are the pad/state
holders of this actor.

---

## 9. rank 83 - `0x80B0DB90`, 300-frame retry arm reset

* **TSV size:** 156 bytes
* **Reachable body:** 39 instructions; 4 `bl` + 0 `bctrl`; ends at
  `0x80B0DC28 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0db90_FUN_80b0db90.c`
* **Proposed signature:** `pub unsafe fn fn_80b0db90(this: usize)`
* **Callers in this REL:** none - reached through a callback/vtable slot
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C` (sound manager),
`r30@+0x18` = `this`. One 12-byte `Vec3` local at `+0x08`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80429D5C` | `u32` | **(ext)** sound context, `r3` of `0x80197F00` |
| `0x1E4` | `u32` | SE id (`li r4,0x1E4`) |
| `300` | `u32` | stored into `+0x9AC` (`li r0,300`, `0x80B0DC00`) |

### Behaviour

```c
if (*(u8 *)(this + 0x9E7) == 0) {                    // 0x80B0DBA8 / 0x80B0DBB0
    fn_80B0CD80(this, 1);                            // 0x80B0DBB8
    if (*(u32 *)(this + 0x9C4) == 0) {               // 0x80B0DBBC / 0x80B0DBC4
        Vec3 pos;
        fn_8006A4B0(&pos, this + 0xAC);              // 0x80B0DBD8
        fn_80197F00(*(u32 *)0x80429D5C, 0x1E4, &pos, 0);   // 0x80B0DBEC
    }
}
fn_80B0D070(this);                                   // 0x80B0DBF4
*(u8  *)(this + 0x9E7) = 0;                          // 0x80B0DC04
*(u32 *)(this + 0x9C8) = 1;                          // 0x80B0DC08
*(u32 *)(this + 0x9AC) = 300;                        // 0x80B0DC0C
*(u16 *)(this + 0x9DE) = 0;                          // 0x80B0DC10
```

Both guards are short-circuited against the *same* join point (`0x80B0DBF0`), so
the SE can only fire on the first entry of a retry window. The four tail stores
and the `fn_80B0D070` call run unconditionally.

This is a **sibling, not a callee**, of wave 131's `0x80B0DC40` ("timered two-state
machine, 300-frame retry"). That body has no `bl` to this entry - it builds its
own 300 at `0x80B0DCAC`/`0x80B0DCDC` - but both reach the same `+0x9AC` timer
field through the same `0x80B0D070` initialiser. This entry has no direct `bl`
caller anywhere in the REL; it is reached through a callback slot.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0DBB8` | `0x80B0CD80` | `r3 = this`, `r4 = 1` (in-REL TSV row) |
| `0x80B0DBD8` | `0x8006A4B0` | `r3 = &pos`, `r4 = this + 0xAC` |
| `0x80B0DBEC` | `0x80197F00` | `*(u32*)0x80429D5C`, `0x1E4`, `&pos`, `0` |
| `0x80B0DBF4` | `0x80B0D070` | `r3 = this` (in-REL TSV row) |

### Ambiguity

`0x80429D5C` is only passed as `r3`; the row stays ready. `0x80B0CD80` and
`0x80B0D070` are separate TSV rows outside this tranche and are **not** ledger
rows, so they must be ported or stubbed before this body links.

---

## 10. rank 84 - `0x80A369B0`, side-indexed heading step (conditional)

* **TSV size:** 152 bytes
* **Reachable body:** 38 instructions; 3 `bl` + 0 `bctrl`; ends at
  `0x80A36A44 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a369b0_FUN_80a369b0.c`
* **Proposed signature:** `pub unsafe fn fn_80a369b0(this: usize) -> u32`
* **Callers in this REL:** `0x80A3716C`
* **Confidence:** **ready apart from one `main.dol` table -> conditional**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C` (normalised limit),
`r30@+0x18` (raw limit), `r29@+0x14` = `this`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x8042C1A8` | `s16[]` | **(ext)** per-side angle **step**, indexed by the side byte |
| `0x8042C1A4` | `s16[]` | **(ext)** per-side angle **limit**; indices 0-1 published as `{+0x2000, -0x2000}` |

### Behaviour

```c
u8  side = (u8)fn_80096240(this, this + 0xAC);   // 0x80A369D0, r4 = this + 0xAC
s16 step  = *(s16 *)(0x8042C1A8 + 2 * side);     // 0x80A369EC
s16 limit = *(s16 *)(0x8042C1A4 + 2 * side);     // 0x80A369F0

*(u8  *)(this + 0x348) = side;                   // 0x80A369F4
*(s16 *)(this + 0x102) = (s16)(*(s16 *)(this + 0x102) + step);   // 0x80A369E0 / 0x80A369F8 / 0x80A36A00

s32 a = fn_802DE514((s32)limit);                 // 0x80A36A04
s32 b = fn_802DE514((s32)*(s16 *)(this + 0x102));// 0x80A36A10
if (b < a) return 0;                             // 0x80A36A18 blt -> 0x80A36A28
*(s16 *)(this + 0x102) = limit;                  // 0x80A36A1C
return 1;                                        // 0x80A36A20
```

The index is scaled by `rlwinm r6,r3,1,23,30` (`0x80A369DC`), i.e.
`(side & 0xFF) * 2`. The side byte is stored to `+0x348` before the clamp, so it
reflects the probe result even on the "not yet at the limit" path.

`0x802DE514` is the branchless `|x|`; applied to two `s16` sign-extended angles,
the comparison is therefore `|cur| >= |limit|`, not a signed compare of the raw
values. The return is `1` exactly when the clamp fired.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A369D0` | `0x80096240` | `r3 = this`, `r4 = this + 0xAC` |
| `0x80A36A04` | `0x802DE514` | `r3 = (s32)limit` |
| `0x80A36A10` | `0x802DE514` | `r3 = (s32)this->s102` |

### Ambiguity - why this row is conditional

The two tables live in `main.dol`, not in the ELF, and both are **consumed
arithmetically**: the `0x8042C1A8` entry is added into `+0x102` and the
`0x8042C1A4` entry is compared against it. `0x8042C1A4`'s pair is already
published (`docs/COVERAGE_WAVE102_ENEMIES.md`: `0x8042C1A4 = {+0x2000, -0x2000}`,
i.e. `+-45` degrees), but **`0x8042C1A8` is not published anywhere in this tree**
and is the per-frame step. Until its two `s16` words are read out of `main.dol`,
the body cannot be byte-exact.

Note also that the two tables are four bytes apart, so `0x8042C1A8[i]` is the same
word as `0x8042C1A4[i + 2]`: if the shipped data is one flat `s16` array, the
"step" for side 0 is whatever follows the `+-0x2000` pair. The spec records the
accesses; it does not assume the array is two entries long.

`0x80096240` returns a `u8` side byte, and this body's two-entry tables imply the
in-play value is 0 or 1 (the same convention as the wave-14/15 boss bodies that
also index `0x8042C1A4`).

---

## 11. rank 85 - `0x80A2F910`, node-name match after the last `:`

* **TSV size:** 136 bytes
* **Reachable body:** 34 instructions; 3 `bl` + 1 `bctrl`; ends at
  `0x80A2F994 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2f910_FUN_80a2f910.c`
* **Proposed signature:** `pub unsafe fn fn_80a2f910(this: usize, name: *const u8) -> u32`
* **Callers in this REL:** none - reached through a callback/vtable slot
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` (candidate name),
`r30@+0x08` = `this`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x3A` | `char` | `':'` - the separator both `strrchr` calls search for |

### Behaviour

```c
const char *n0 = name;                              // r31 = r4
const char *t  = (const char *)fn_802E1C40(n0, ':'); // 0x80A2F934
if (t != 0) n0 = t + 1;                             // 0x80A2F938 beq skips, 0x80A2F93C adds

const char *mine = (const char *)(*(u32 *(**)(usize))(*(u32 *)this + 0x20))(this);  // 0x80A2F944..0x80A2F954

const char *u = (const char *)fn_802E1C40(mine, ':');   // 0x80A2F95C
u32 eq = fn_802E1AB4(u + 1, n0);                        // 0x80A2F968
return eq == 0;                                         // 0x80A2F970..0x80A2F97C
```

This is a "same model node name" predicate: it strips everything up to and
including the last `:` from both names (BRRES names are
`"g3d/sanbo.brres:sanbo_body"`-shaped) and `strcmp`s the remainders. It returns
`1` when the names are equal (`li r3,1` on the `beq`), `0` otherwise.

The asymmetry is real and must be preserved: the **first** `strrchr` result is
null-checked (`0x80A2F938 beq 0x80A2F944`), the **second** is not - `addi
r3,r3,1` at `0x80A2F964` runs unconditionally, so a name without `:` on either
side would address `0x1`. In play the vtable accessor always returns a qualified
name.

The vtable dispatch reads slot `+0x20` from the object's own vtable at offset 0
and requires no arguments beyond `r3 = this`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A2F934` | `0x802E1C40` | `r3 = name`, `r4 = 0x3A` |
| `0x80A2F954` | `*(u32*)this + 0x20` (`bctrl`) | `r3 = this` |
| `0x80A2F95C` | `0x802E1C40` | `r3 = accessor result`, `r4 = 0x3A` |
| `0x80A2F968` | `0x802E1AB4` | `r3 = second_strrchr + 1`, `r4 = first_strrchr + 1` |

### Ambiguity

No data constants at all; the only literal is `':'`. The row is ready as soon as
`0x802E1C40`/`0x802E1AB4` are available, which they are (both `translated` ledger
rows). `fn_80a2f830`'s body ends at `0x80A2F90C`, so this entry is a clean
neighbour, not an alias.

---

## 12. rank 86 - `0x80ADE3A0`, `block_pata` step (`+0xAC` delta sets `+0x348`)

* **TSV size:** 136 bytes
* **Reachable body:** 34 instructions; 1 `bl` + 1 `bctrl`; ends at
  `0x80ADE424 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ade3a0_FUN_80ade3a0.c`
* **Proposed signature:** `pub unsafe fn fn_80ade3a0(this: usize)`
* **Callers in this REL:** none - reached through a callback/vtable slot
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x0C` = `this`; `f31` saved as
`stfd f31,0x10(r1)` + `psq_st f31,0x18(r1),0,0` and restored by
`psq_l f31,0x18(r1),0,0` + `lfd f31,0x10(r1)`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B30B38` | `f32` | `0.0` - the threshold the frame delta is compared against |
| `+0x6B8` | slot | sub-object, vtable word at `+0x6B8`, slot `+0x1C` |

### Behaviour

```c
(*(void (**)(usize))(*(u32 *)(this + 0x6B8) + 0x1C))(this + 0x6B8);  // 0x80ADE3BC..0x80ADE3CC
f32 before = *(f32 *)(this + 0xAC);                                   // 0x80ADE3D0
fn_80ADDAC0(this);                                                    // 0x80ADE3D8
f32 d = *(f32 *)(this + 0xAC) - before;                               // 0x80ADE3DC..0x80ADE3E8
*(u8 *)(this + 0x348) = (d < 0.0f) ? 1 : 0;                           // 0x80ADE3EC..0x80ADE408
```

The comparison is the CodeWarrior `fcmpo` + `cror` idiom:

```
0x80ADE3EC  fcmpo cr0,f1,f0        ; f1 = d, f0 = 0.0f
0x80ADE3F0  cror  2,1,2            ; CR0.EQ := CR0.GT | CR0.EQ   (i.e. EQ := !LT)
0x80ADE3F4  bne   0x80ADE404       ; taken when CR0.EQ == 0, i.e. when d < 0
0x80ADE3F8  li    r0,0 ; stb 0x348 ; fall-through arm
0x80ADE404  li    r0,1 ; stb 0x348 ; taken arm
```

so `+0x348 = 1` exactly when the sub-object step **decreased** `+0xAC`. An
unordered result (NaN) also clears `CR0.EQ` and therefore stores 1; keep the
`!(d >= 0.0f)` form rather than `d < 0.0f` if the port must match NaN behaviour.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80ADE3CC` | `*(u32*)(this+0x6B8) + 0x1C` (`bctrl`) | `r3 = this + 0x6B8` |
| `0x80ADE3D8` | `0x80ADDAC0` | `r3 = this` (in-REL TSV row) |

### Ambiguity

The single `psq_st`/`psq_l` pair is `f31`/`qr0` and only spills the register, so
the export's `unaff_GQR0` + `ldexpf` noise is cosmetic (wave-127 finding 1). The
only constant is the in-REL `0.0`, so the row is ready. `0x80ADDAC0` is a TSV
row outside this tranche (the `block_pata` step family that wave 131 already
partially published through `0x80ADE240`).

---

## Verification

Performed read-only, in this order, before writing a line of this page:

1. **Pool.** Re-parsed the wave-127 B table (173 rows) and recomputed the
   reachable instruction count of every row by walking `bc` targets inside the
   body from the entry (conditional branches followed, `bl`/`bctrl` treated as
   fall-through, `b` followed only when it targets the same body, stop at `blr`
   / out-of-body `b`). The walker reproduces the twelve published wave-132
   counts (`71..58`) and the twelve published wave-133 counts (`57..52`)
   exactly, so it is calibrated against both predecessors.
2. **Ranking.** Of the 173 B rows, 83 are already consumed - 72 are spec headings
   (60 of those are ledger rows as well) plus 11 ledger-only rows - which leaves
   **90 free rows**. Sorted by (count desc, TSV size asc, address asc) the first
   twelve are ranks 75-86 above. The counts 48-51 do not occur in the free pool,
   which is why the tranche starts at 47.
3. **Ledger.** Each of the twelve addresses was checked against the **address
   column** of all 16,350 rows of `ledger/functions.csv`: absent. One of them
   (`0x80A46BF0`) is named in the evidence prose of row `80a46cc0`; that is not
   an address row and does not consume the address.
4. **Headings.** Each of the twelve was checked against every `##`/`#` heading in
   every `docs/*.md`: absent. Three are mentioned in earlier waves' prose only
   (`0x80B0CFD0` and `0x80A369B0` in `SPEC_WAVE131_ENEMIES.md` as untranslated
   callees, `0x80A2F910` in `SPEC_WAVE133_ENEMIES.md` as the end marker of
   `0x80A2F830`).
5. **Instruction stream.** All twelve bodies were decoded from
   `work/rels/elf/d_enemiesNP.elf` at `(addr - 0x80A00000) + 0x1000`; every
   decoded body is exactly `TSV size / 4` instructions and re-disassembles to the
   same instruction sequence as its Ghidra `.c` export.
6. **Control flow.** Zero out-of-body `bc` targets; 19 conditional branches, all
   internal. All 11 `bctrl` sites have a resolved `+0x14`/`+0x1C`/`+0x20`/`+0xD4`
   slot.
7. **Callees.** Seven distinct in-REL `bl` targets
   (`0x80A7E630`, `0x80B19CB0`, `0x80B19EE0`, `0x80B19F50`, `0x80B0CD80`,
   `0x80B0D070`, `0x80ADDAC0`); each is a `functions.tsv` entry. Three of them
   (`0x80B19CB0`, `0x80B19EE0`, `0x80B19F50`) are already `translated` ledger
   rows; the other four are still untranslated. Every other `bl` target is a
   `main.dol` address.
8. **Constants.** Every in-REL load was resolved through the
   `lis`/`addi`/`or` chain that builds its base and then read out of the image:
   `0x80B2E050 = 4.0f`, `0x80B2E058 = (f64)2^52`, `0x80B2C56C = 4.0f`,
   `0x80B2C570 = 0.1f`, `0x80B2C7E8 = 1024.0f`, `0x80B2C7EC = 2.0f`,
   `0x80B30B38 = 0.0f`, plus the `sanbo` string block
   (`0x80B6565C` `"sanbo_body"`, `0x80B65668` `"sanbo_head"`,
   `0x80B65678` `{0x80B6565C, 0x80B65668}`, `0x80B65680`
   `"g3d/sanbo.brres"`, `0x80B65690` `"sanbo"`). Addresses above `0x80B6E434`
   are `main.dol` and are marked **(ext)**.

### Cross-references and collisions

* `docs/SPEC_WAVE133_ENEMIES.md` records `0x80A2F830` as "ends at `0x80A2F910`
  blr". The `blr` is at `0x80A2F90C` (`0x80A2F830 + 55*4`); `0x80A2F910` is the
  separate 136-byte TSV entry specified here as rank 85. This page did not edit
  that doc.
* `0x80B0CFD0` (lines 122, 834, 854, 859) and `0x80A369B0` (lines 119, 1072,
  1100, 1109) are listed as *untranslated callees* in
  `docs/SPEC_WAVE131_ENEMIES.md`. Both are specified here for the first time.
* `0x80B0DB90` is the 300-frame-retry **sibling** of
  `docs/SPEC_WAVE131_ENEMIES.md`'s `0x80B0DC40` body (same `+0x9AC` timer, same
  `0x80B0D070` initialiser, no direct call between them - checked by scanning
  every `bl` in the REL). `0x80ADE3A0` is the `block_pata` step sibling of the
  `0x80ADE240` effect calls in the same doc.
* Five of the twelve (`0x80A7ED20`, `0x80B19FA0`, `0x80B0DB90`, `0x80A2F910`,
  `0x80ADE3A0`) have **no** `bl` caller in the REL: they are only reachable
  through a vtable slot or a callback table installed by another module. Each
  section states its callers, found by scanning every opcode-18/LK word in the
  image.
* Rank 84 (`0x80A369B0`) is the only conditional row of the tranche. That is the
  same single-conditional shape as waves 132 and 133.
