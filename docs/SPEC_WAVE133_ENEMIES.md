# Wave 133 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses: 173 class **B** (reconstructible now), 160 class C and
51 class D. Waves 128, 129, 130, 131 and 132
(`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE132_ENEMIES.md`) specified
ranks 1-61 of that pool.

This lane is the continuation: **ranks 62-74** of the same wave-127 ordering,
minus one rank that has since been consumed. Rank 65, `0x80A07080` (224 bytes,
56 reachable instructions), is a `translated` ledger row now
(`game::enemies`, `fn_80a07080`), so this tranche runs one rank deeper and ends
at rank 74.

It is **read-only**. It writes no Rust, no ledger row and no other doc. Each
body below was decoded from `work/rels/elf/d_enemiesNP.elf` with the corrected
`+0x1000` segment shift, cross-checked against the Ghidra export at the cited
path and against `work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Every
constant that lives inside the REL image was read out of the ELF itself, so the
float values below are the shipped values, not guesses.

All twelve were re-checked against the `address` column of
`ledger/functions.csv` (all 16,296 rows, absent) and against every `0x80Axxxxx`
/ `0x80Bxxxxx` mention in the five earlier enemies specs (absent). Nothing here
duplicates a wave-128 .. wave-132 claim.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then by ascending address. The method was re-validated against this tranche's
predecessors: walking the twelve wave-132 addresses reproduces all twelve
published counts exactly (71, 70, 69, 69, 64, 62, 62, 61, 61, 60, 60, 58), and
this page picks up at 57 - the next count below wave 132's 58.

| rank | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 62 | `0x80A360D0` | 228 | 57 | Choropoo model + `walk` clip binder | **ready** |
| 63 | `0x80A37460` | 228 | 57 | attack puff + player-SE at the actor | **ready** |
| 64 | `0x80B18F20` | 228 | 57 | type-`0x19B` box-overlap list scan | **ready** |
| 66 | `0x80A2EA60` | 224 | 56 | 8-slot countdown tick, vtable `+0x14` | **ready** |
| 67 | `0x80A2F830` | 224 | 56 | child-chain "first busy" query, vtable `+0xC` | **ready** |
| 68 | `0x80AFEDD0` | 224 | 56 | byte-identical twin of rank 67 | **ready** |
| 69 | `0x80AEAFF0` | 216 | 54 | vertical interval emitter, `0x80AEAC20` | **ready** |
| 70 | `0x80A7E640` | 212 | 53 | 4-player `x`-proximity test, mask byte | **ready** |
| 71 | `0x80A442D0` | 208 | 52 | coin effect + four-stage teardown | **ready** |
| 72 | `0x80AD3C10` | 208 | 52 | 3-key value track on `+0x728` | **ready** |
| 73 | `0x80AEDAC0` | 208 | 52 | deleting destructor, two `+0x694` members | **ready** |
| 74 | `0x80B28710` | 208 | 52 | deleting destructor, two registered callbacks | **ready** |

Rank 65 (`0x80A07080`) is skipped: `translated` in `ledger/functions.csv`
(`game::enemies`, 224-byte body). It is the only ledger-consuming address
inside the window.

**Split: 12 ready, 0 conditional.**

"Conditional" is used exactly as in waves 128-132: a named datum in `main.dol`
that is **consumed arithmetically or used as an index** and must be read before
a byte-exact port. No row here does that. Every external address in this set is
either (a) a `main.dol` callee called by address, (b) a pointer or state token
that is only dereferenced, stored or passed through as an argument, or (c) a
mask byte ANDed against a shift value where the literal expression is
reproduced in full (the wave-132 rank-57 precedent, `0x80429CA8`). Rank 70 is
that (c) case and rank 63 is the (b) case (`0x80429D5C` is only passed as `r3`).

The four risk classes that made waves 110-126 skip a body are all absent:

* no `psq_l`/`psq_st` other than `f31`/`f30` stack spills with `qr0`
  (6 spill pairs in 4 bodies, all `qr0`, all prologue/epilogue saves);
* no `bctr` jump table - every conditional branch target is inside its body
  (checked mechanically: zero out-of-body `bc` targets), and every `bctrl` is a
  vtable dispatch with a resolved `+0xNN` slot (12 in total, in 4 bodies);
* no incoming float argument - no body reads `f1`/`f2`/`f3` before writing it;
* no unledgered callee landmine - every in-REL `bl` target is a
  `functions.tsv` entry: the two callees in this set are `0x80AEAC20` (192-byte
  row) and the self-recursions of ranks 64, 67 and 68.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`); the same shift holds for the
  data segments. The image ends at `0x80B6E434`; anything above that is a
  `main.dol` address.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* An address that does not resolve in `d_enemiesNP.elf` is a `main.dol`
  address. It is marked **(ext)** and must be treated as a named external
  datum, not as a missing base.
* In-REL float constants are quoted with their shipped value. They were read
  at file offset `(addr - 0x80A00000) + 0x1000` and are `f32` unless marked
  `f64`.
* Branch mnemonics are the C form after the exact `bc` operand decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 4,0` = `bge`, `bc 12,1` = `bgt`,
  `bc 4,1` = `ble`, `bc 12,0` = `blt`, `bc 16,*` = `bdnz`.
* **`addi rX,r0,imm` is `li rX,imm`.** Several bodies build small integers this
  way; the source register `r0` is not read when `RA == 0`.
* **A negative `addi` on a `lis` base is a page-relative data address.** In
  ranks 62, 63, 70 and 71 the base is built as `lis rX,0x80B3` / `lis
  rX,0x80B4` and then adjusted with a negative `addi`; the quoted data
  addresses below are the computed ones (`0x80B2xxxx` / `0x80B3xxxx`), not the
  `lis` page. Getting this wrong reads a valid-looking neighbouring table.
* **`fmuls`/`fmadds` take `FRT,FRA,FRC`, and `FRC` is encoded in bits 21-25**,
  not bits 16-20; bits 16-20 are the reserved `FRB` field and are zero here. The
  `fadds`/`fsubs`/`fdivs` family is X-form and takes `FRT,FRA,FRB`, with `FRB`
  in bits 16-20. This matters for ranks 63 and 72: a decoder that reads the
  reserved field as a third multiplicand turns `-0.9 * this->E8` into
  `-0.9 * 2.5` and `16 * 0.5` into `16 * 0.5 * 0`. Ghidra reads both correctly;
  the four-operand listing is the trap.
* The `addic. rX,rA,imm` / `beq` pairs in ranks 73 and 74 are CodeWarrior's
  inlined sub-object null guards (`if (&member)`) and are never false at run
  time. They are reproduced as the source's `if` because the branch is in the
  stream, not because the address can be null.
* Helper roles below are the ledger's, not new claims:
  `0x800690C0` = reset a model holder `(holder, -1, resetWord, 0, 0x20)`;
  `0x800690E0` = finish it; `0x800DF180` = load a model by name
  `(archive, name, path)`; `0x80239E30` = find a model node;
  `0x8023A0B0` = find an animation by name; `0x80169D90` = bind a model record
  `(holder, &node, owner, mode, 1, 0)`; `0x80064BD0` = finish a model record;
  `0x801650D0` = attach a node+animation pair to a clip holder;
  `0x801680E0` = set an animation rate; `0x8016C890` = spawn an effect by name
  `(name, 0, &pos, 0, 0)`; `0x8006A4B0` = position to sound argument
  `(&out, pos)`; `0x80197F00` = sound with a position `(system, cue, &pos, 0)`;
  `0x8005F900` = resolve a player-table entry by index (returns a pointer);
  `0x80162D50` = list probe by type `(type, previous)` - the cursor is global,
  so repeated calls walk the list (see `docs/COVERAGE_WAVE17_BOSSES.md`);
  `0x8007F950`/`0x8007FDA0` = the `+0x534` sub-object pair bracketing rank 71's
  body (roles unpublished); `0x8015F9F0` = release an embedded sub-object
  `(ptr, 0)`; `0x8007F810` = release the `+0x584` clip holder `(holder, -1)`;
  `0x80169D20` = release a model record `(holder, -1)`;
  `0x80069060` = release a model holder `(holder, -1)`;
  `0x8006CDE0`, `0x800955F0`, `0x80027A60`, `0x80036A70`, `0x80027D70` = the
  shared actor step/teardown calls, used by address (roles unpublished);
  `0x802DCA98` = register a record callback `(record, fn, bytes, count)`;
  `0x802B9280` = release one object (the `0x8015F800`/`0x802B9280` teardown
  family); `0x80095130` = base-object teardown step;
  `0x80162920` = conditional delete when the deleting flag is positive.

## Symbolic helpers used below

```text
vt(obj, slot)                  # (*(u32 *)obj)->vt[slot], receiver r3 = obj
fn_8015F9F0(ptr, 0)            # release an embedded sub-object
fn_802B9280(ptr)               # release one object
fn_80162920(this)              # operator delete when r4 > 0
fn_80a00200(ptr, flag)         # in-REL deleting thunk: if (ptr && flag>0) release
fn_80aeac20(f1, this, &pos, a, i)  # in-REL per-slot emitter (untranslated)
fn_80a360d0(this)              # this page, rank 62
fn_80b18f20(this)              # this page, rank 64 (self recursion)
fn_80a2f830(this)              # this page, rank 67 (self recursion, 5 levels in)
```

--------------------------------------------------------------------------------

## 1. rank 62 - `0x80A360D0`, Choropoo model + `walk` clip binder

* **TSV size:** 228 bytes
* **Reachable body:** 57 instructions; 8 `bl` + 0 `bctrl`; ends at
  `0x80A361B4 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a360d0_FUN_80a360d0.c`
* **Proposed signature:** `pub unsafe fn fn_80a360d0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`, `r29@+0x24`.
`r29 = this` (`or r29,r3,r3` at `0x80A360FC`), `r30 = 0x80B3DD68` (the shared
Choropoo string block, built as `lis r30,0x80B4` + `addi r30,r30,-0x2298`).
No loop, and no spills beyond the three saved registers.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B3DD80` | `char[]` | `"g3d/choropoo.brres"` (`r30+0x18`) |
| `0x80B3DD94` | `char[]` | `"choropoo"` (`r30+0x2C`) |
| `0x80B3DDA0` | `char[]` | `"walk"` (`r30+0x38`) |
| `0x80377C48` | `u32` | **(ext)** resource word passed as `r5` to `0x800690C0` |
| `0x8042A038` | `u32` ptr | **(ext)** archive handle; `+4` is passed to `0x800DF180` |

### Behaviour

```c
fn_800690C0(this + 0x524, 0xFFFFFFFF, *(u32 *)0x80377C48, 0, 0x20);

*(u32 *)(this + 0x540) =
    fn_800DF180(*(u32 *)0x8042A038 + 4, "choropoo", "g3d/choropoo.brres");
r31 = fn_80239E30(this + 0x540, "choropoo");     // node: also stored at r1+0x10
fn_80169D90(this + 0x544, &node, this + 0x524, 0x20, 1, 0);
fn_80064BD0(this + 0x544);

anim = fn_8023A0B0(this + 0x540, "walk");
fn_801650D0(this + 0x584, &node, &anim, this + 0x524, 0);

fn_800690E0(this + 0x524);
```

The bind mode is `0x20` (the fishbone binder in `SPEC_WAVE128_ENEMIES.md` §8
uses `0x128`/`0x120`), so only the record size differs from that family; the
call sequence - reset, load, find node, bind, finish, find clip, attach, finish
holder - is the same.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A3610C` | `0x800690C0` | `r3 = this+0x524`, `r4 = -1`, `r5 = *(0x80377C48)`, `r6 = 0`, `r7 = 0x20` |
| `0x80A36124` | `0x800DF180` | `r3 = *(0x8042A038)+4`, `r4 = "choropoo"`, `r5 = "g3d/choropoo.brres"` |
| `0x80A36134` | `0x80239E30` | `r3 = this+0x540`, `r4 = "choropoo"` |
| `0x80A36158` | `0x80169D90` | `r3 = this+0x544`, `r4 = &stack+0x10`, `r5 = this+0x524`, `r6 = 0x20`, `r7 = 1`, `r8 = 0` |
| `0x80A36160` | `0x80064BD0` | `r3 = this+0x544` |
| `0x80A3616C` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = "walk"` |
| `0x80A3618C` | `0x801650D0` | `r3 = this+0x584`, `r4 = &stack+0x0C` (node), `r5 = &stack+0x08` (anim), `r6 = this+0x524`, `r7 = 0` |
| `0x80A36194` | `0x800690E0` | `r3 = this+0x524` |

### Ambiguity

None. Both external words are passed straight through; all strings and the
record size are in-REL.

---

## 2. rank 63 - `0x80A37460`, attack puff + player SE at the actor

* **TSV size:** 228 bytes
* **Reachable body:** 57 instructions; 4 `bl` + 0 `bctrl`; ends at
  `0x80A37544 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a37460_FUN_80a37460.c`
* **Proposed signature:** `pub unsafe fn fn_80a37460(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`.
`r30 = this`, `r31 = 0x80B2C100` (the shared constant page; built as
`lis r31,0x80B3` + `addi r31,r31,-0x3F00`). Stack record at `+0x10..+0x18`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C1C0` | `f32` | `1.3` (animation rate) |
| `0x80B2C1C8` | `f32` | `-0.9` (decay applied to `+0xE8`) |
| `0x80B2C1CC` | `f32` | `2.5` (stored to `+0xEC`) |
| `0x80B2C1D0` | `f32` | `-3.0` (`x` offset when `+0x348 != 0`) |
| `0x80B2C1D4` | `f32` | `3.0` (`x` offset when `+0x348 == 0`) |
| `0x80B2C1D8` | `f32` | `2.0` (extra `y`) |
| `0x80B2C1AC` | `f32` | `5500.0` (effect `z`) |
| `0x80B3DE10` | `char[]` | `"Wm_en_comattack"` |
| `0x80429D5C` | `u32` ptr | **(ext)** sound-system handle, passed as `r3` |

`0x80B2C1AC` is `0x45ABE000`; it is the same `5500.0` "far plane" `z` that
`SPEC_WAVE128_ENEMIES.md` §9 uses for the same string block, so the two bodies
share the convention.

### Behaviour

```c
*(f32 *)(this + 0xEC) = 2.5f;                                  // 0x80A3748C
*(f32 *)(this + 0xE8) = (-0.9f) * *(f32 *)(this + 0xE8);       // 0x80A37490
fn_801680E0(this + 0x584, /*f1=*/ 1.3f);                       // rate

f32 dx = (*(u8 *)(this + 0x348) != 0) ? -3.0f : 3.0f;           // 0x80A374A4..B8
vec3 pos;
pos.x = *(f32 *)(this + 0xAC) + dx;
pos.y = *(f32 *)(this + 0xB0) + *(f32 *)(this + 0xD4) + 2.0f;
pos.z = 5500.0f;
fn_8016C890("Wm_en_comattack", 0, &pos, 0, 0);                 // 0x80A37500

fn_8006A4B0(&se_pos, this + 0xAC);                             // 0x80A37514
fn_80197F00(*(u32 *)0x80429D5C, 0x21C, &se_pos, 0);            // 0x80A37528
```

`0x80A37490` is `fmuls f2,f2,f1` with `f2 = -0.9`, `f1 = *(this+0xE8)`; the
Ghidra export prints `fVar2 * *(float*)(param_1 + 0xe8)`, which is the correct
two-operand form: the `2.5` is only *stored* to `+0xEC` and is not
a multiplicand. `se_pos` is a 3-float copy taken by `0x8006A4B0`, not the
effect position; the sound uses the raw actor position.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A374A0` | `0x801680E0` | `r3 = this+0x584`, `f1 = 1.3` |
| `0x80A37500` | `0x8016C890` | `r3 = "Wm_en_comattack"`, `r4 = 0`, `r5 = &pos`, `r6 = 0`, `r7 = 0` |
| `0x80A37514` | `0x8006A4B0` | `r3 = &se_pos`, `r4 = this+0xAC` |
| `0x80A37528` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 0x21C`, `r5 = &se_pos`, `r6 = 0` |

### Ambiguity

None. `+0x348` is a byte flag inside the actor; both of its values are covered
by the two shipped constants.

---

## 3. rank 64 - `0x80B18F20`, type-`0x19B` box-overlap list scan

* **TSV size:** 228 bytes
* **Reachable body:** 57 instructions; 2 `bl` + 0 `bctrl`; ends at
  `0x80B19004 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b18f20_FUN_80b18f20.c`
* **Proposed signature:** `pub unsafe fn fn_80b18f20(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`. `r31 = this`. Six dead
`stfs` words are written to `+0x08..+0x1C` (the compiler spilled the temporary
`vec`/half-extent values); they are never read and must not be given meaning.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B32418` | `f32` | `0.5` - half-scale applied to the probe's `+0x5B4` extent |

### Behaviour

```c
u32 p = fn_80162D50(0x19B);          // list probe; r4 = incoming r4 (see below)
if (p == 0) return 0;

f32 h = 0.5f * *(f32 *)(p + 0x5B4);          // 0x80B18F60, fmuls f1,f0,f8
f32 pmaxx = *(f32 *)(p + 0xAC) + h;
f32 pminx = *(f32 *)(p + 0xAC) - h;
f32 pmaxy = *(f32 *)(p + 0xB0) + *(f32 *)(p + 0x5B8);
f32 pcy   = *(f32 *)(p + 0xB0);

f32 tminx = *(f32 *)(this + 0xAC) + *(f32 *)(this + 0x398);
f32 tmaxx = *(f32 *)(this + 0xAC) + *(f32 *)(this + 0x3A0);
f32 tmaxy = *(f32 *)(this + 0xB0) + *(f32 *)(this + 0x3A4);
f32 tminy = *(f32 *)(this + 0xB0) + *(f32 *)(this + 0x39C);

if (pmaxx < tminx || tmaxx < pminx || pmaxy < tmaxy || tminy < pcy)
    return fn_80b18f20(this);        // 0x80B18FEC, r4 = 0x80B30000 (see below)
return 1;                            // 0x80B18FE0
```

The four comparisons are the ones in the stream, in this order (`fcmpo` +
`blt`/`bgt` at `0x80B18F88`, `0x80B18FC8`, `0x80B18FD0`, `0x80B18FD8`). The
second `y` test compares the probe's **centre** `+0xB0` against
`this->0xB0 + this->0x39C`, not an extent - it is reproduced verbatim.

### Port hazard - the clobbered cursor

`0x80B18F4C` does `lis r4,0x80B3` to reach the `0.5f` constant, which destroys
the incoming `r4`. Nothing reloads it, so:

* the first `0x80162D50` call passes **the caller's `r4` unchanged**;
* the self-recursion at `0x80B18FEC` passes `r4 = 0x80B30000`.

`0x80162D50` is a global-cursor list probe (`(type, previous)`, see
`docs/COVERAGE_WAVE17_BOSSES.md`), so the intent is "test the next object of
type `0x19B`". A byte-exact port must call
`fn_80162d50(0x19B, 0x80B30000)` on the recursive path; if the cursor argument
is proven unused by the helper, `0` is equivalent. This does not block the row:
the register value is known exactly.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B18F38` | `0x80162D50` | `r3 = 0x19B`, `r4` = incoming (unmodified) |
| `0x80B18FEC` | `0x80B18F20` | `r3 = this`, `r4 = 0x80B30000` |

### Ambiguity

The `+0x398..+0x3A4` block is read as four independent offsets, not as a
centre/extent pair: the `x` tests use `+0x398`/`+0x3A0` and the `y` tests use
`+0x3A4`/`+0x39C`, so the block order is `(minX, minY, maxX, maxY)` in the
source's own naming, whatever those names were.

The `y` test is **asymmetric** and must be ported as written: it rejects when
`p->B0 + p->5B8 < this->B0 + this->3A4`, i.e. it requires
`this->B0 + 0x3A4 <= p->B0 + p->5B8`, while the next test requires
`this->B0 + 0x39C >= p->B0`. The two conditions together mean "the `y` span of
`this` lies inside the `y` span of `p`", not the usual symmetric overlap; the
`x` tests *are* symmetric. Symmetrising the `y` test would change behaviour.

---

## 4. rank 66 - `0x80A2EA60`, 8-slot countdown tick

* **TSV size:** 224 bytes
* **Reachable body:** 56 instructions; 0 `bl` + 1 `bctrl`; ends at
  `0x80A2EB40 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2ea60_FUN_80a2ea60.c`
* **Proposed signature:** `pub unsafe fn fn_80a2ea60(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `f31` saved at `+0x30` (`stfd`) and `+0x38`
(`psq_st qr0`); `f30` at `+0x20`/`+0x28`; `r31@+0x1C`, `r30@+0x18`,
`r29@+0x14`, `r28@+0x10`. `r28` is the slot index, `r30` walks `+0x88` per
slot, `r29 = this + 0x744` walks `+0x88` per slot. The two `psq` words are the
only paired-single traffic in the body and are pure `f31`/`f30` saves.

### Slot layout

Slot `i` (`i = 0..7`) is `this + 0x744 + 0x88 * i`:

| offset | type | role |
|---|---|---|
| `+0x00` | `u32` | vtable pointer, slot `+0x14` is called |
| `+0x38` | `f32` | countdown (absolute `+0x77C` from `this`) |
| `+0x3C` | `f32` | mirror (`+0x780`) |
| `+0x40` | `f32` | mirror (`+0x784`) |
| `+0x44` | `u16` | state; only `1` is acted on (`+0x788`) |

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2BC58` | `f32` | `0.0` - lower bound for the countdown |
| `0x80B2BC88` | `f32` | `0.06` - per-tick decrement |

### Behaviour

```c
for (u32 i = 0; i != 8; ++i) {                       // 0x80A2EAAC..0x80A2EB0C
    u8 *slot = (u8 *)this + 0x744 + 0x88 * i;        // r29
    u8 *base = (u8 *)this + 0x88 * i;                // r30
    if (*(u16 *)(base + 0x788) != 1) continue;

    vt(slot, 0x14)(slot);                            // 0x80A2EAC8 bctrl
    f32 t = *(f32 *)(base + 0x77C) - 0.06f;
    *(f32 *)(base + 0x77C) = t;
    if (0.0f < t) {                                  // 0x80A2EAD8..0x80A2EAF8
        *(f32 *)(base + 0x780) = t;
        *(f32 *)(base + 0x784) = t;
    } else {
        *(u16 *)(base + 0x788) = 0;
    }
}
```

The `fcmpo` + `cror 2,0,2` + `bne` at `0x80A2EAD8..0x80A2EAE0` is
`if (0.0f < t)`; `cror 2,0,2` sets `cr0[EQ] = cr0[LT] | cr0[EQ]`, so the
`bne` (branch when `EQ == 0`) is the `GT` case. Writing it as `t > 0` is what
the export shows (`dVar6 < fVar1`).

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A2EAC8` | vtable `+0x14` of `*(slot)` | `r3 = slot = this+0x744+0x88*i` |

### Ambiguity

None. The loop bound is the literal `8` (`cmpi cr0,r28,8` at `0x80A2EB04`), not
a field.

---

## 5. rank 67 - `0x80A2F830`, child-chain "first busy" query

* **TSV size:** 224 bytes
* **Reachable body:** 56 instructions; 1 `bl` + 5 `bctrl`; ends at
  `0x80A2F910 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2f830_FUN_80a2f830.c`
* **Proposed signature:** `pub unsafe fn fn_80a2f830(this: usize) -> usize`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C`. `r31` is the node currently
being returned; it walks down the chain with `lwz r31,0x30(r31)`.

### Behaviour

```c
usize node = this;
if (vt(*(u32 *)(node + 0x30), 0xC)(*(u32 *)(node + 0x30)) != 0)  // level 0
    return node;
node = *(u32 *)(node + 0x30);
if (vt(*(u32 *)(node + 0x30), 0xC)(...) != 0) return node;        // level 1
node = *(u32 *)(node + 0x30);
if (... != 0) return node;                                        // level 2
node = *(u32 *)(node + 0x30);
if (... != 0) return node;                                        // level 3
node = *(u32 *)(node + 0x30);
if (... != 0) return node;                                        // level 4
node = *(u32 *)(node + 0x30);
return fn_80a2f830(*(u32 *)(node + 0x30));                        // level 5 -> recurse
```

Each test is the same three instructions: `lwz r3,0x30(node)`, `lwz r12,0(r3)`,
`lwz r12,0xC(r12)`, `mtspr CTR`, `bcctrl`, `cmpi cr0,r3,0`. The receiver of the
virtual call is the **child**, and the returned `node` is the **parent** whose
child reported non-zero - the helper answers "deepest ancestor whose immediate
child is busy".

The five inlined levels are a straight-line unroll, not a loop: the compiler
inlined the recursive definition five times, then emitted the real recursive
call. A re-implementation must keep the `return` values exactly as above - each
level returns the **parent** whose immediate child tested non-zero, and only the
sixth descent recurses. Collapsing the unroll into a loop that returns the child
would change which object the caller receives.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A2F854` | vtable `+0xC` of `*(*(this+0x30))` | `r3 = *(this+0x30)` |
| `0x80A2F874` | vtable `+0xC` of `*(*(r31+0x30))` | `r3 = *(r31+0x30)` |
| `0x80A2F894` | same, level 2 | |
| `0x80A2F8B4` | same, level 3 | |
| `0x80A2F8D4` | same, level 4 | |
| `0x80A2F8E8` | `0x80A2F830` | `r3 = *(r31+0x30)` |

### Ambiguity

None; the body has no constants and no external operands. The only requirement
is that `+0x30` is the "next in chain" pointer, which all six `lwz` sites
confirm.

---

## 6. rank 68 - `0x80AFEDD0`, byte-identical twin of rank 67

* **TSV size:** 224 bytes
* **Reachable body:** 56 instructions; 1 `bl` + 5 `bctrl`; ends at
  `0x80AFEEB0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80afedd0_FUN_80afedd0.c`
* **Proposed signature:** `pub unsafe fn fn_80afedd0(this: usize) -> usize`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C`.

### Behaviour

Identical to rank 67 instruction for instruction, with only the two branch
displacements changed:

| rank 67 | rank 68 | instruction |
|---|---|---|
| `0x80A2F830` | `0x80AFEDD0` | entry |
| `0x80A2F854` | `0x80AFEDF4` | `bcctrl`, level 0 |
| `0x80A2F8E8` | `0x80AFEE88` | `bl 0x80AFEDD0`, recursive level |
| `0x80A2F90C` | `0x80AFEEAC` | `blr` |

Every other word matches: same `stwu -0x10`, same six `lwz r3,0x30` /
`lwz r12,0` / `lwz r12,0xC` / `mtspr CTR` / `bcctrl` sequences, same
`cmpi cr0,r3,0` tests, same `return node` epilogue. The body can be written once
as a shared helper used by two ledger rows, or duplicated; the rows are separate
functions and the ledger expects both.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AFEDF4` | vtable `+0xC` of `*(*(this+0x30))` | `r3 = *(this+0x30)` |
| `0x80AFEE14` | same, level 1 | |
| `0x80AFEE34` | same, level 2 | |
| `0x80AFEE54` | same, level 3 | |
| `0x80AFEE74` | same, level 4 | |
| `0x80AFEE88` | `0x80AFEDD0` | `r3 = *(r31+0x30)` |

### Ambiguity

None; see rank 67.

---

## 7. rank 69 - `0x80AEAFF0`, vertical interval emitter

* **TSV size:** 216 bytes
* **Reachable body:** 54 instructions; 1 `bl` + 0 `bctrl`; ends at
  `0x80AEB0C8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aeaff0_FUN_80aeaff0.c`
* **Proposed signature:** `pub unsafe fn fn_80aeaff0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x50(r1)`; LR at `+0x54`; `f31` at `+0x40` (`stfd`) and `+0x48`
(`psq_st qr0`); `f30` at `+0x30`/`+0x38`; `r31@+0x2C`, `r30@+0x28`,
`r29@+0x24`. `r29 = this`, `r30` is the loop index, `r31` keeps the
`0x43300000` page word.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B311A4` | `f32` | `32.0` - both the per-slot `y` step and the `f1` argument |
| `0x80B311A8` | `f64` | `0x4330000080000000` = `2^52 + 2^31` (unsigned-int bias) |
| `0x43300000` | `u32` | page word for the `2^52` idiom (immediate, not an address) |

### Behaviour

```c
u32 n = *(u32 *)(this + 0x818) + 1;
for (u32 i = 0; i != n; ++i) {
    // (f32)(u32)i via the 2^52 bias: 0x43300000 is stored over i ^ 0x80000000
    // at stack+0x18/+0x1C, `lfd` reads it as the double 0x43300000_XXXXXXXX,
    // and `fsubs` subtracts 0x80B311A8 = 0x4330000080000000 (2^52 + 2^31).
    f32 d = (f32)i;
    vec3 pos;
    pos.x = *(f32 *)(this + 0xAC);
    pos.y = *(f32 *)(this + 0xB0) - 32.0f * d;
    pos.z = *(f32 *)(this + 0xB4);
    fn_80AEAC20(/*f1=*/ 32.0f, /*f2=*/ *(f32 *)(this + 0xAC),
                /*f3=*/ *(f32 *)(this + 0xB0),
                this, &pos, *(u32 *)(this + 0x814), i);
}
```

The conversion is exact for `i < 2^24` and rounds above it, because `fsubs`
rounds the double difference to single; that is the shipped behaviour and must
not be "fixed" into a double conversion. Ghidra renders the same expression as
`(float)((double)CONCAT44(0x43300000, i ^ 0x80000000) - dVar3)`.

The loop condition is `n = *(u32 *)(this + 0x818) + 1` compared **unsigned**
(`cmpl cr0,r30,r0` at `0x80AEB094`); a negative `+0x818` would therefore run
about 4 billion iterations. The count is a count, not a "≤" bound, so the +1 is
part of the shipped loop.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AEB084` | `0x80AEAC20` | `r3 = this`, `r4 = &pos`, `r5 = *(u32)(this+0x814)`, `r6 = i`, `f1 = 32.0`, `f2 = *(f32)(this+0xAC)`, `f3 = *(f32)(this+0xB0)` |

`0x80AEAC20` is in-REL but not part of this tranche; the row stays ready because
the call is by address with all five operands explicitly set.

### Ambiguity

None for this body. The `2^52 + 2^31` page word is the standard
unsigned-int → `f32` idiom already documented for this REL
(`ANALYSIS_WAVE127_ENEMIES.md`, finding 1); the four `ldexpf` blocks Ghidra
prints are cosmetic.

---

## 8. rank 70 - `0x80A7E640`, 4-player `x`-proximity test

* **TSV size:** 212 bytes
* **Reachable body:** 53 instructions; 1 `bl` + 0 `bctrl`; ends at
  `0x80A7E714 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7e640_FUN_80a7e640.c`
* **Proposed signature:** `pub unsafe fn fn_80a7e640(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `f31` at `+0x30`/`+0x38`; `r31@+0x2C`,
`r30@+0x28`, `r29@+0x24`, `r28@+0x20`. `r28 = this`, `r29` = player index,
`r30` = mask bit (`1, 2, 4, 8`), `r31` = `0x80429CA8`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2E1E0` | `f32` | `64.0` - maximum absolute `dx` |
| `0x80429CA8` | `u8` | **(ext)** per-player presence mask byte |

### Behaviour

```c
u8 mask = *(u8 *)0x80429CA8;
u32 bit = 1;
for (u32 i = 0; i != 4; ++i) {
    if ((mask & bit) != 0) {
        u32 p = fn_8005F900(i);
        if (p != 0) {
            f32 dx = *(f32 *)(p + 0xAC) - *(f32 *)(this + 0xAC);
            // three dead stfs: (p->AC, p->B0, p->B4) and |dx|
            if (fabsf(dx) <= 64.0f) return 1;
        }
    }
    bit <<= 1;
}
return 0;
```

The mask test is `lbz r0,0(r31)` + `and r0,r30,r0` + `beq` (`0x80A7E67C..84`);
the shift is `rlwinm r30,r30,1,24,30` (`0x80A7E6DC`), i.e. `(bit << 1) & 0xFE`,
which is `bit <<= 1` for the four iterations that run.

Only `x` is compared. `+0xB0`/`+0xB4` are copied to the stack frame and never
read - the export keeps them because the source materialised a `Vec3` for the
distance computation.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A7E68C` | `0x8005F900` | `r3 = i` (0..3) |

### Ambiguity

`0x80429CA8` is a main.dol byte consumed by a **bit test**, not arithmetically
and not as an index: the literal expression `mask & (1 << i)` is complete
without reading the byte's value. This is the wave-132 rank-57 precedent, so the
row stays ready; a port only needs the address, not the shipped bit pattern.

---

## 9. rank 71 - `0x80A442D0`, coin effect + four-stage teardown

* **TSV size:** 208 bytes
* **Reachable body:** 52 instructions; 8 `bl` + 1 `bctrl`; ends at
  `0x80A443A0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a442d0_FUN_80a442d0.c`
* **Proposed signature:** `pub unsafe fn fn_80a442d0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C`. `r31 = this`. One 3-float
record at `+0x08..+0x10`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C728` | `f32` | `0.0` (`f1` of `0x80027B00`) |
| `0x80B2C72C` | `f32` | `8.0` (added to the effect `y`) |
| `0x80B406FC` | `char[]` | `"Wm_ob_coin"` |

### Behaviour

```c
fn_8007F950(this + 0x534);                       // enters the sub-object
fn_8006CDE0(this);                               // A
fn_800955F0(this);                               // B

u32 ok = fn_80027B00(this, /*f1=*/ 0.0f, 0, 0);
if ((ok & 1) == 0) {                             // 0x80A44314 rlwinm + bne
    vec3 pos;
    pos.x = *(f32 *)(this + 0xAC);
    pos.y = *(f32 *)(this + 0xB0) + 8.0f;
    pos.z = *(f32 *)(this + 0xB4);
    vt(this + 0x8D0, 0xA0)(this + 0x8D0, "Wm_ob_coin", 0, &pos, 0, 0);
}

fn_80027A60(this);
fn_80036A70(this);
fn_80027D70(this);
fn_8007FDA0(this + 0x534);                       // leaves the sub-object
```

`0x80027B00` returns a word whose **bit 0** is the gate
(`rlwinm r0,r3,0,31,31` + `bne`): the effect is spawned only when bit 0 is
clear. The `0x80027A60`, `0x80036A70` and `0x80027D70` calls and the
`0x8007FDA0` release run on both paths, so the actor is torn down whether or not
the effect fired.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A442E8` | `0x8007F950` | `r3 = this+0x534` |
| `0x80A442F0` | `0x8006CDE0` | `r3 = this` |
| `0x80A442F8` | `0x800955F0` | `r3 = this` |
| `0x80A44310` | `0x80027B00` | `r3 = this`, `f1 = 0.0`, `r4 = 0`, `r5 = 0` |
| `0x80A44368` | vtable `+0xA0` of `*(this+0x8D0)` | `r3 = this+0x8D0`, `r4 = "Wm_ob_coin"`, `r5 = 0`, `r6 = &pos`, `r7 = 0`, `r8 = 0` |
| `0x80A44370` | `0x80027A60` | `r3 = this` |
| `0x80A44378` | `0x80036A70` | `r3 = this` |
| `0x80A44380` | `0x80027D70` | `r3 = this` |
| `0x80A44388` | `0x8007FDA0` | `r3 = this+0x534` |

### Ambiguity

`0x80027B00`'s bit-0 return is read exactly; the meaning of the remaining bits
is not needed. The row has no external data operands at all: `0x80027B00` and
the four main.dol teardown helpers are called by address with explicit
arguments, and the only `main.dol` word involved is inside those callees.

---

## 10. rank 72 - `0x80AD3C10`, 3-key value track on `+0x728`

* **TSV size:** 208 bytes
* **Reachable body:** 52 instructions; 4 `bl` + 0 `bctrl`; ends at
  `0x80AD3CE0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad3c10_FUN_80ad3c10.c`
* **Proposed signature:** `pub unsafe fn fn_80ad3c10(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `f31` at `+0x20`/`+0x28`; `r31@+0x1C`,
`r30@+0x18`. `r30 = this`, `r31 = 0x80B308D0`. Local records: a 2-float pair at
`+0x10` and a 2-float key record at `+0x08`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B3093C` | `f32` | `16.0` (`f31`, the track amplitude) |
| `0x80B3092C` | `f32` | `0.0` (first-call value and key record's second word) |
| `0x80B30940` | `f32` | `0.5` (amplitude scale) |
| `0x80B30944` | `f32` | `4.0` (key bias) |

### Behaviour

```c
f32 amp = 16.0f;                                    // 0x80B3093C
f32 pair[2] = { *(f32 *)(this + 0xAC), *(f32 *)(this + 0xB0) };
fn_800C10F0(this + 0x728, pair, /*f1=*/ 0.0f, 0, 0);

f32 key[2];
key[1] = 0.0f;                                      // 0x80B3092C
key[0] = amp * 0.5f - 4.0f;                         // = 4.0
fn_800C1560(this + 0x728, key, 1);

key[0] = 4.0f + (-amp) * 0.5f;                      // = -4.0
fn_800C1560(this + 0x728, key, 2);

fn_800C1580(this + 0x728);
```

Both `fmuls` sites are A-form with the multiplicand in bits 21-25
(`fmuls f2,f31,f1` and `fmuls f1,f2,f1`), giving `16*0.5 = 8` and `-16*0.5 = -8`.
The Ghidra export reproduces the same two results (`4.0` then `-4.0`), so the
export and the stream agree here; only a decoder that reads the reserved
`FRB` field would see a phantom `* 0.0`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AD3C60` | `0x800C10F0` | `r3 = this+0x728`, `r4 = &pair`, `f1 = 0.0`, `r5 = 0`, `r6 = 0` |
| `0x80AD3C8C` | `0x800C1560` | `r3 = this+0x728`, `r4 = &key`, `r5 = 1` |
| `0x80AD3CB4` | `0x800C1560` | `r3 = this+0x728`, `r4 = &key`, `r5 = 2` |
| `0x80AD3CBC` | `0x800C1580` | `r3 = this+0x728` |

### Ambiguity

The three main.dol helpers `0x800C10F0`, `0x800C1560` and `0x800C1580` are not
in the ledger and are called by address with every operand explicit. Their
collective shape (set the base pair, push key 1 and key 2, commit) is the same
one used by the trail/emitter family in waves 128-132, so the row does not need
their source names.

---

## 11. rank 73 - `0x80AEDAC0`, deleting destructor

* **TSV size:** 208 bytes
* **Reachable body:** 52 instructions; 7 `bl` + 0 `bctrl`; ends at
  `0x80AEDB90 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aedac0_FUN_80aedac0.c`
* **Proposed signature:** `pub unsafe fn fn_80aedac0(this: usize, flag: i32) -> usize`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C`, `r30@+0x18`, `r29@+0x14`.
`r29 = this`, `r30 = r4` (the deleting flag), `r31` is a scratch member address.

### Behaviour

```c
if (this == 0) return this;                     // 0x80AEDAC8 + beq

if (this + 0x694) {                             // addic. r31,r3,0x694
    if (this + 0x6D4) {
        if (this + 0x6EC) fn_8015F9F0(this + 0x6EC, 0);
    }
    if (this + 0x698) {
        if (this + 0x6B0) fn_8015F9F0(this + 0x6B0, 0);
    }
}

fn_8007F810(this + 0x584, 0xFFFFFFFF);
fn_80169D20(this + 0x544, 0xFFFFFFFF);
fn_80069060(this + 0x524, 0xFFFFFFFF);
fn_80095130(this, 0);
if (flag > 0) fn_80162920(this);                // 0x80AEDB64 ble 0x80AEDB70
return this;
```

The chain of `addic.` / `beq` at `0x80AEDAEC..0x80AEDB2C` is the inlined
destructor of two embedded members at `+0x6B0` and `+0x6EC`, each preceded by
its own null guard on the parent at `+0x694`. The addresses are compile-time
constants and the guards are never false; the branch structure is preserved so
that the emitted body matches.

Note the two consecutive `beq` at `0x80AEDAE4`/`0x80AEDAE8` test the **same**
`cr0` from the single `cmpi` at `0x80AEDAC8` - they are the two inlined copies
of the `if (this)` test, and a port must not "optimise" the second one into a
different comparison.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AEDB10` | `0x8015F9F0` | `r3 = this+0x6EC`, `r4 = 0` |
| `0x80AEDB2C` | `0x8015F9F0` | `r3 = this+0x6B0`, `r4 = 0` |
| `0x80AEDB38` | `0x8007F810` | `r3 = this+0x584`, `r4 = -1` |
| `0x80AEDB44` | `0x80169D20` | `r3 = this+0x544`, `r4 = -1` |
| `0x80AEDB50` | `0x80069060` | `r3 = this+0x524`, `r4 = -1` |
| `0x80AEDB5C` | `0x80095130` | `r3 = this`, `r4 = 0` |
| `0x80AEDB6C` | `0x80162920` | `r3 = this` |

### Ambiguity

None. The row is a destructor: the `r4` flag is the only argument and its two
cases (`> 0` deletes, otherwise not) are both visible in the `cmpi`/`ble`.

---

## 12. rank 74 - `0x80B28710`, deleting destructor with two callback registrations

* **TSV size:** 208 bytes
* **Reachable body:** 52 instructions; 8 `bl` + 0 `bctrl`; ends at
  `0x80B287E0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b28710_FUN_80b28710.c`
* **Proposed signature:** `pub unsafe fn fn_80b28710(this: usize, flag: i32) -> usize`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C`, `r30@+0x18`, `r29@+0x14`,
`r28@+0x10`. `r28 = this`, `r29 = r4` (flag), `r30 = this+0x584`,
`r31 = 0x80A00000` (only used to reach the in-REL thunk `0x80A00200`).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80A00200` | `code` | in-REL deleting thunk: `if (p && flag > 0) fn_802B9280(p); return p;` (16 insns, TSV 64) |

The thunk at `0x80A00200` is `stwu -0x10` / `cmpi r3,0` / `beq` /
`cmpi r4,0` / `ble` / `bl 0x802B9280` / `return r3`. It is the pair form used by
`0x802DCA98`: the registration supplies the pointer, the thunk and the payload
size, and the runtime calls the thunk with `(pointer, count)`.

### Behaviour

```c
if (this == 0) return this;                          // 0x80B28718 + beq

if (this + 0x584) {                                  // addic. r30,r3,0x584
    fn_802DCA98(this + 0x658, 0x80A00200, 0x0C, 6);
    fn_802DCA98(this + 0x5F8, 0x80A00200, 0x0C, 8);
    fn_80169D20(this + 0x5AC, 0xFFFFFFFF);
    fn_80069060(this + 0x58C, 0xFFFFFFFF);
}

fn_80169D20(this + 0x544, 0xFFFFFFFF);
fn_80069060(this + 0x524, 0xFFFFFFFF);
fn_80095130(this, 0);
if (flag > 0) fn_80162920(this);
return this;
```

The two registrations target the embedded members at `+0x5F8` and `+0x658`
(`this+0x584+0x74` and `this+0x584+0xD4`), each with a 12-byte payload and
counts `8`/`6` respectively - note the **higher** address gets the **lower**
count in the stream order, and that order is preserved.

This row is the same destructor family as rank 73 but with the `+0x584`
sub-object being a callback-registered member rather than a model holder, and
with no `0x8007F810` call.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B28758` | `0x802DCA98` | `r3 = this+0x658`, `r4 = 0x80A00200`, `r5 = 0x0C`, `r6 = 6` |
| `0x80B2876C` | `0x802DCA98` | `r3 = this+0x5F8`, `r4 = 0x80A00200`, `r5 = 0x0C`, `r6 = 8` |
| `0x80B28778` | `0x80169D20` | `r3 = this+0x5AC`, `r4 = -1` |
| `0x80B28784` | `0x80069060` | `r3 = this+0x58C`, `r4 = -1` |
| `0x80B28790` | `0x80169D20` | `r3 = this+0x544`, `r4 = -1` |
| `0x80B2879C` | `0x80069060` | `r3 = this+0x524`, `r4 = -1` |
| `0x80B287A8` | `0x80095130` | `r3 = this`, `r4 = 0` |
| `0x80B287B8` | `0x80162920` | `r3 = this` |

### Ambiguity

None. `0x80A00200` is inside `d_enemiesNP.elf` and is a `functions.tsv` entry,
so it is not an unresolved base; it is passed as a **function pointer**, not
called directly.

--------------------------------------------------------------------------------

## Verification

* Twelve addresses, all absent from `ledger/functions.csv` (16,296 rows) and
  from every `0x80Axxxxx` / `0x80Bxxxxx` in
  `docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE132_ENEMIES.md`.
* Each is present in `work/ghidra/rel-d_enemiesNP/export/functions.tsv` at the
  quoted size, and each has an export at
  `work/ghidra/rel-d_enemiesNP/export/<addr>_FUN_<addr>.c`.
* Reachable spans were obtained by walking raw words from the entry with the
  corrected mapping `(addr - 0x80A00000) + 0x1000`, following `bc` targets and
  `bl` returns until every path ends in `blr` or `bctr`. The decoder was
  validated first against 22 published counts - the ten ranked rows at the top
  of `ANALYSIS_WAVE127_ENEMIES.md` and the twelve of
  `SPEC_WAVE132_ENEMIES.md` - all reproduced exactly.
* In all twelve rows the reachable span equals the TSV size, so the quoted
  instruction count is `size / 4`; there is no extent-truncation case here.
* Constants were read from the ELF at their image addresses; the four
  `main.dol` operands (`0x80377C48`, `0x8042A038`, `0x80429D5C`, `0x80429CA8`)
  are marked **(ext)** and are passed or bit-tested, never scaled or indexed.
* No Rust file, ledger row or other doc was written by this pass.
