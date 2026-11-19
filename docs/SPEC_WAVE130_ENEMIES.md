# Wave 130 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 173 of the 384
formerly-skipped `d_enemiesNP` addresses as class **B**: the reachable
instruction stream decodes end-to-end, every path ends in `blr` or a jump to a
local epilogue, and nothing depends on a `GQR1..GQR7` quantiser, a jump table,
an unresolved base or an incoming float argument.

Wave 128 (`docs/SPEC_WAVE128_ENEMIES.md`) specified ranks 1-12 of that pool and
wave 129 (`docs/SPEC_WAVE129_ENEMIES.md`) ranks 13-24. This lane is the
continuation: ranks 25-36 of the class-B pool, in the same ordering.

It is **read-only**. It writes no Rust, no ledger row and no other doc. Each
body below was decoded from `work/rels/elf/d_enemiesNP.elf` with the corrected
`+0x1000` segment shift, then cross-checked against the Ghidra export at the
cited path and against `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.

All twelve were re-checked against the `address` column of
`ledger/functions.csv` (absent, 16,139 rows) and against `functions.tsv`
(present). Nothing here duplicates a wave-128 or wave-129 claim.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk. Ties are broken by TSV size, then
by address. These are ranks 25-36; the next candidate after this set is
`0x80AB2840` at 89 reachable instructions.

| # | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 1 | `0x80AB2460` | 444 | 111 | `Wm_en_burst_water` per-frame step (timer, rate ramp, sound) | **ready** |
| 2 | `0x80AB0C00` | 440 | 110 | 60-byte particle record, three identical state arms | **ready** |
| 3 | `0x80B0F5A0` | 440 | 110 | `pukupuku` model + damage-animation binder | **ready** |
| 4 | `0x80AD4420` | 404 | 101 | state callback: dispatch on `+0x854`, pad tables | **ready** |
| 5 | `0x80A7FB50` | 392 | 98 | icicle model matrix refresh, scale from kind test | **ready** |
| 6 | `0x80ACC740` | 388 | 97 | per-frame step with a temporary `+0xAC` floor | **ready** |
| 7 | `0x80AE63D0` | 388 | 97 | `pukupuku` matrix refresh with `+0xE54` scale | **ready** |
| 8 | `0x80AB1160` | 384 | 96 | two-matrix builder (`z+32`, `z+64`) | **ready** |
| 9 | `0x80A33630` | 380 | 95 | matrix refresh with `(0, +10, -4)` / `(0, -10, +4)` | **ready** |
| 10 | `0x80B1E540` | 380 | 95 | two-state `+0x360` machine with a speed ramp | **ready** |
| 11 | `0x80A7BD30` | 372 | 93 | `walk_*` clip-pair selector (2 sides x 18 pairs) | **ready** |
| 12 | `0x80AC9AA0` | 372 | 93 | per-frame step, angle-flip gate, two effect records | **ready** |

**Split: 12 ready, 0 conditional.**

"Conditional" is used here exactly as in waves 128/129: a named datum in
`main.dol` that is **consumed arithmetically or used as an index** and must be
read before a byte-exact port. None of these twelve does that. Every external
address in this set is either (a) a `main.dol` callee, called by address, or
(b) a pointer, state token or callback word that is only dereferenced, stored
or passed through as an argument - never combined into a value that decides a
branch or an arithmetic result. The four remaining risk classes that made waves
110-126 skip a body are all absent:

* no `psq_l`/`psq_st` other than `f31`/`f30` stack spills with `qr0`
  (verified opcode-by-opcode: 6 spill pairs in 5 of the 12 bodies);
* no `bctr` jump table - every `bc` target is inside the body
  (checked mechanically: zero out-of-body branch targets);
* no incoming float argument - every body's `f1`/`f2`/`f3` is written before
  it is read; the only uninitialised FPR reads in the whole set are the
  `f30`/`f31` prologue saves (5 bodies);
* no unledgered callee landmine - every in-REL `bl` target is a
  `functions.tsv` entry.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`); the same `+0x1000` shift holds
  for the data segments (re-verified against `0x80B32DC0` at file offset
  `0x133DC0`). The image ends at `0x80B6E434`; anything above that is a
  `main.dol` address.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* An address that does not resolve in `d_enemiesNP.elf` is a `main.dol`
  address. It is marked **(ext)** and must be treated as a named external
  datum, not as a missing base.
* Branch mnemonics are the C form after the exact `bc` operand decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 4,0` = `bge`, `bc 12,1` = `bgt`,
  `bc 4,1` = `ble`, `bc 12,0` = `blt`.
* `li rX,imm` is `addi rX,r0,imm`; `lis` is `addis rX,r0,imm`.
* **Record-form instructions matter.** Three sites in this set depend on the
  `Rc` bit, and the Ghidra exports print all three without the dot. They are
  written here with the dot: `rlwinm. r0,r0,0,22,22` (#4),
  `rlwinm. r0,r0,1,31,31` (#5) and `mullw. r0,r30,r0` (#12). A port that drops
  the dot reads a stale `cr0`.
* `cror 2,0,2` / `cror 2,1,2` after an `fcmpo` rewrite `cr0.eq` to
  `lt|eq` / `gt|eq`; the following `beq`/`bne` is written in the C form that
  results, not in the raw `cr0.eq` form.
* Helper roles below are the ledger's, not new claims:
  `0x8006CDE0`/`0x8006CD40` = the two halves of the base actor update;
  `0x800955F0` = second base update; `0x8006A4B0` = position to sound argument;
  `0x80198660` = sound with an actor argument `(system, cue, owner, &pos, 0)`;
  `0x80197F00` = sound with a position argument;
  `0x80064AF0` = position + Euler to vector (`(vec, ang, mode)`);
  `0x801C0BD0` = translation-matrix builder `(out, x, y, z)`;
  `0x801C0540` = concat `(out, a, b)`; `0x8016A170` = copy into a holder;
  `0x8016A150` = offset a holder; `0x8016A1A0` = clear a holder flag;
  `0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0` = set a matrix yaw/pitch/roll from a
  `u16`; `0x801680E0` = set an animation rate (float as `f1`);
  `0x80168FE0` = set an animation frame (float as `f1`);
  `0x80169F80` = apply an animation blend `(f1, holder, holder2)`;
  `0x80169020` = test an animation holder; `0x8015F4F0` = blend an angle
  toward a target `(u16 *, target, mode)`;
  `0x800DF180` = load a model by name `(archive, name, path)`;
  `0x80239E30` = find a model node; `0x8023A0B0` = find an animation by name;
  `0x8023A200` = find a second animation by name;
  `0x800690C0` = reset a model holder; `0x800690E0` = finish it;
  `0x80169D90` = bind a model record; `0x80064BD0` = finish a model record;
  `0x801650D0` = bind an animation pair; `0x801651F0` = attach an animation;
  `0x800B8700` = commit an effect record `(owner, record, 1)`;
  `0x802DCA98` = register a record callback `(record, fn, bytes, count)`.

## Symbolic helpers used below

```text
fn_80ab1ce0(this) -> u32          # cluster 66, translated: cached-player check
fn_80ab1ca0(this) -> u32          # cluster 66, translated: height clamp
fn_80ab1ec0(this) -> u32          # cluster 66, translated: visibility query
fn_80ab1e00(this)                 # cluster 66, translated: make visible
fn_80ab1fa0(this)                 # cluster 66, translated: angle sweep
fn_80ab1df0(this)                 # cluster 66, translated: holder update
fn_80ab1830(this)                 # untranslated: 0x40-frame countdown
fn_80ab12e0(this, a, b, c)        # untranslated, same bucket
fn_80ad42e0(this)                 # untranslated, same bucket
fn_80acae70(this, obj)            # translated
fn_80acb200(this) / fn_80acb330(this)   # untranslated
fn_80acb280(this, out) -> u32     # untranslated
fn_80ac9310(this)                 # translated
fn_80a7a2c0(blend, this)          # translated, blend is f1
fn_80a7a360(blend, this)          # translated, blend is f1
fn_80a7a030(this) / fn_80a7a170(this)   # translated
fn_80a33470(this)                 # translated
fn_80b1d680(this) -> f32          # translated
fn_80b1da40(this)                 # untranslated

vt(obj, slot)   # (*(u32 *)obj)->vt[slot], receiver r3 = obj
kind_test(base, token) -> i32
  # obj = (*(u32 *)(base + 0x394))->vt[0x28](base + 0x394)
  # return (*obj)->vt[0x14](obj, token)
```

---

## 1. `0x80AB2460` - `Wm_en_burst_water` per-frame step

* **TSV size:** 444 bytes
* **Reachable body:** 111 instructions; 10 `bl` + 3 `bctrl`; ends at
  `0x80AB2618 blr` (shared epilogue at `0x80AB2600`)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab2460_FUN_80ab2460.c`
* **Proposed signature:** `pub unsafe fn fn_80ab2460(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`, `r29@+0x24`.
`r29 = this`. Scratch: `r1+0x10/0x14/0x18` (three-float direction vector, dead
once the velocity stores below have consumed it) and `r1+0x08..0x18` (the
12-to-16 byte sound position record that `0x8006A4B0` fills later, overwriting
the vector tail).

### Constants

| address | type | value |
|---|---|---|
| `0x80B2FAE4` | `f32` | `0.0` |
| `0x80B54BB0` | `f32` | `0.025` (rate decrement) |
| `0x80B54BA8` | `f32` | `1.2` (rate floor) |
| `0x80B750D0` | state token | **(ext)** argument of the hidden-state dispatch |
| `0x80429D5C` | pointer | **(ext)** sound system |

### Behaviour

```c
// 0x80AB247C: u16 frame timer at +0x998
if (*(u16 *)(this + 0x998) != 0)
    *(u16 *)(this + 0x998) -= 1;

// 0x80AB2498 bne 0x80AB24CC: the timer gates the hidden path
if (*(u16 *)(this + 0x998) == 0 && fn_80AB1CE0(this) == 0) {
    vt(this, 0xD4) (0x80B750D0);          // r3 = this, r4 = token
    goto epilogue;                        // 0x80AB24C8 b 0x80AB2600
}

// 0x80AB24CC: direction from the two cached displacement words
v[0] = *(f32 *)(this + 0x8B4);
v[1] = *(f32 *)(this + 0x8B8);
v[2] = 0.0f;                              // 0x80B2FAE4
fn_80170A00(&v);                          // r3 = r1+0x10, in-place normalise

rate = *(f32 *)(this + 0x958) - 0.025f;   // 0x80B54BB0
*(f32 *)(this + 0x958) = rate;
if (rate <= 1.2f) {                       // fcmpo + cror 2,0,2 + bne
    rate = 1.2f;                          // 0x80B54BA8
    *(f32 *)(this + 0x958) = rate;
}

// 0x80AB251C: scale the unit vector; z stays zero
*(f32 *)(this + 0xE8) = v[0] * rate;
*(f32 *)(this + 0xEC) = v[1] * rate;
*(f32 *)(this + 0xF0) = 0.0f;             // 0x80B2FAE4, not v[2]*rate

// 0x80AB255C: note the argument order - f1 is the z slot
fn_8006CD40(this, /*f1*/ 0.0f, /*f2*/ v[1] * rate, /*f3*/ v[0] * rate);

if (fn_80AB1EC0(this) != 0) goto epilogue;

if (*(u16 *)(this + 0x998) != 0)
    fn_80AB1CA0(this);                    // height clamp while the timer runs

fn_8006A4B0(&pos, this + 0xAC);           // r3 = r1+0x08, r4 = this+0xAC
fn_80198660(*(u32 *)0x80429D5C, 0x208, *(u32 *)this, &pos, 0);

fn_80AB1FA0(this);                        // angle sweep
vt(this + 0x73C, 0x14);                   // 0x80AB25CC
if (*(u32 *)(this + 0x988) == 1)          // 0x80AB25D0
    vt(this + 0x794, 0x14);               // 0x80AB25EC
fn_80AB1DF0(this);
fn_80AB1830(this);                        // 0x40-frame countdown
```

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AB24A0` | `0x80AB1CE0` | `r3 = this` |
| `0x80AB24C4` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B750D0` |
| `0x80AB24EC` | `0x80170A00` | `r3 = r1+0x10` |
| `0x80AB255C` | `0x8006CD40` | `r3 = this`, `f1 = 0.0`, `f2 = v[1]*rate`, `f3 = v[0]*rate` |
| `0x80AB2564` | `0x80AB1EC0` | `r3 = this` |
| `0x80AB2580` | `0x80AB1CA0` | `r3 = this` |
| `0x80AB2598` | `0x8006A4B0` | `r3 = r1+0x08`, `r4 = this+0xAC` |
| `0x80AB25B0` | `0x80198660` | `r3 = *(0x80429D5C)`, `r4 = 0x208`, `r5 = *(u32 *)this`, `r6 = r1+0x08`, `r7 = 0` |
| `0x80AB25B8` | `0x80AB1FA0` | `r3 = this` |
| `0x80AB25CC` | vtable `+0x14` of `*(this+0x73C)` | `r3 = this+0x73C` |
| `0x80AB25EC` | vtable `+0x14` of `*(this+0x794)` | `r3 = this+0x794` |
| `0x80AB25F4` | `0x80AB1DF0` | `r3 = this` |
| `0x80AB25FC` | `0x80AB1830` | `r3 = this` |

### Ambiguity

None in-REL. Three notes for the port: (a) the `b 0x80AB2600` at `0x80AB24C8`
targets this function's own epilogue, so it is a `return`, not a tail call;
(b) `r5` of the sound call is the actor's first word (`*(u32 *)this`), the
sound subsystem's owner handle - role inferred, value exact; (c) `0x8006CD40`
is untranslated, so the *meaning* of its `f1..f3` operands is inferred from the
register setup at this call site (the three floats are freshly computed here,
which is why this spec lists them). The `f1 = 0.0` is literal and must not be
replaced by `v[2]*rate` (which is also zero, but arrives through a different
register).

---

## 2. `0x80AB0C00` - 60-byte particle record, three identical state arms

* **TSV size:** 440 bytes
* **Reachable body:** 110 instructions; 2 `bl` + 0 `bctrl`; ends at
  `0x80AB0DB4 blr`; the three arms join at `0x80AB0CD0`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab0c00_FUN_80ab0c00.c`
* **Proposed signature:** `pub unsafe fn fn_80ab0c00(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x70(r1)`; LR at `+0x74`; `r31@+0x6C`. Scratch:
`r1+0x08..0x10` (a written-but-never-read scale triplet),
`r1+0x14..0x1C` (the scale triplet that is read),
`r1+0x20..0x28` (the sample position),
`r1+0x2C..0x67` (the 60-byte effect record).

### Constants

| address | type | value |
|---|---|---|
| `0x80B2FAF0` | `f32` | `1.0` |
| `0x80B2FB00` | `f32` | `10.5` (offset subtracted from `+0xB0`) |
| `0x80B2FB04` | `f32` | `0.9` (middle scale, `8.9999998e-1`) |
| `0x80B2FAE4` | `f32` | `0.0` |
| `0x80A0B5A0` | `u8[]` | **(in-REL)** record callback byte block passed to `0x802DCA98` |

`r7 = 0x80B2FA50` (constant page, `addis r7,r0,-32589` / `addi r7,r7,-0x5B0`).

### Behaviour

The state word at `+0x98C` selects one of three arms; **all three arms are
byte-identical** (verified word-for-word: `0x80AB0C24..0x80AB0C58`,
`0x80AB0C64..0x80AB0C98` and `0x80AB0C9C..0x80AB0CCC`), and each falls into
the shared join at `0x80AB0CD0` with the same stack contents. Port it as one
body with an empty guard, or as three copies - behaviour is identical.

```c
switch (*(u32 *)(this + 0x98C)) {         // 0x80AB0C18
case 0: case 1: default:
    s[0] = 1.0f;                          // r1+0x14, 0x80B2FAF0
    s[1] = 0.9f;                          // r1+0x18, 0x80B2FB04
    s[2] = 1.0f;                          // r1+0x1C
    p[0] = *(f32 *)(this + 0xAC);         // r1+0x20
    p[1] = *(f32 *)(this + 0xB0) - 10.5f; // 0x80B2FB00
    p[2] = *(f32 *)(this + 0xB4);         // r1+0x28
    break;
}

// 0x80AB0CD0: build the 60-byte record at r1+0x2C
*(u32 *)(rec + 0x00) = 2;                 // 0x80AB0D38, written last
*(f32 *)(rec + 0x04) = p[0];
*(f32 *)(rec + 0x08) = p[1];
*(f32 *)(rec + 0x0C) = p[2];
*(f32 *)(rec + 0x10) = 1.0f;
*(f32 *)(rec + 0x14) = 0.9f;
*(f32 *)(rec + 0x18) = 1.0f;
for (i = 0; i != 8; ++i)                  // 0x80AB0D60..0x80AB0D7C
    *(f32 *)(rec + 0x1C + i * 4) = 0.0f;

// dead stores kept for byte order: the same scale triplet into r1+0x08..0x10
*(f32 *)(r1 + 0x08) = 1.0f; *(f32 *)(r1 + 0x0C) = 0.9f; *(f32 *)(r1 + 0x10) = 1.0f;

u32 result = fn_800B8700(this + 0x484, rec, 1);
fn_802DCA98(rec, 0x80A0B5A0, 0x3C, 1);
return result;
```

Store order is observable in the original: the eight zero words are written
**before** the type word and the three position words, and `rec+0x00` is
pre-zeroed with `stw r6,48(r1)` before the type store overwrites it. Keep the
sequence if the port is byte-compared.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AB0D80` | `0x800B8700` | `r3 = this+0x484`, `r4 = r1+0x2C`, `r5 = 1` |
| `0x80AB0D9C` | `0x802DCA98` | `r3 = r1+0x2C`, `r4 = 0x80A0B5A0`, `r5 = 0x3C`, `r6 = 1` |

### Ambiguity

None in-REL. The record field roles (`+0x00` type/count, `+0x04..0x0C` position,
`+0x10..0x18` scale, `+0x1C..` rotation/colour words) are inferred from the
neighbouring emitter `fn_80ac53e0` (`0x800B8700` + `0x802DCA98` with the same
15-word layout). The `+0x08..0x10` triplet is dead but present in the original.

---

## 3. `0x80B0F5A0` - `pukupuku` model + damage-animation binder

* **TSV size:** 440 bytes
* **Reachable body:** 110 instructions; 16 `bl` + 1 `bctrl`; ends at
  `0x80B0F754 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0f5a0_FUN_80b0f5a0.c`
* **Proposed signature:** `pub unsafe fn fn_80b0f5a0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `r31@+0x3C`, `r30@+0x38`, `r29@+0x34`,
`r28@+0x30`. `r28 = this`, `r29 = 0x80B687A0` (asset page).
Locals: `r1+0x08` and `r1+0x1C` (the `+0x5BC` animation handle),
`r1+0x0C`, `r1+0x14` and `r1+0x18` (the model node), `r1+0x10` and `r1+0x20`
(the `fly` animation handle).

### Constants

| address | type | value |
|---|---|---|
| `0x80B687AC` | `char[]` | `"g3d/pukupuku.brres"` (archive path, `r5`) |
| `0x80B687C0` | `char[]` | `"pukupuku"` (model/node name, `r4`) |
| `0x80B687CC` | `char[]` | `"fly"` (animation name) |
| `0x80B31F94` | `f32` | `1.0` (blend rate and frame) |
| `0x80B31F80` | `f32` | `0.0` (final rates) |
| `0x8042A038` | pointer | **(ext)** archive manager; the loader takes `+4` |
| `0x80377C48` | word | **(ext)** animator reset argument (`r5` of `0x800690C0`) |
| `0x80B687A0` | block | in-REL class descriptor: `{0x80B0EEF0, 0x01840193, 0x12, "g3d/pukupuku.brres", ...}` |

### Behaviour

```c
fn_800690C0(this + 0x524, 0xFFFFFFFF, *(u32 *)0x80377C48, 0, 0x20);

u32 model_rec = fn_800DF180(*(u32 *)0x8042A038 + 4,
                            0x80B687C0 /*"pukupuku"*/,
                            0x80B687AC /*"g3d/pukupuku.brres"*/);
*(u32 *)(this + 0x540) = model_rec;

u32 model = fn_80239E30(this + 0x540, 0x80B687C0);   // node handle
fn_80169D90(this + 0x544, &model, this + 0x524, 0x23, 1, 0);
fn_80064BD0(this + 0x544);

u32 anim = fn_8023A0B0(this + 0x540, 0x80B687CC /*"fly"*/);
fn_801650D0(this + 0x584, &model, &anim, this + 0x524, 0);
fn_801651F0(this + 0x584, this + 0x544, &anim, 0);
fn_80169F80(1.0f, this + 0x544, this + 0x584);       // 0x80B31F94
fn_801680E0(1.0f, this + 0x584);

u32 dmg = fn_8023A200(this + 0x540, 0x80B687C0);
*(u32 *)(this + 0x5BC) = dmg;

fn_80166830(this + 0x5C0, &model, &dmg, this + 0x524, 0, 1);
fn_80166BD0(this + 0x5C0, this + 0x544, &dmg, 0, 1);

vt(this + 0x544, 0x18) (this + 0x5C0);               // r3 = +0x544, r4 = +0x5C0

fn_80166EF0(0.0f, this + 0x5C0, 0);                  // 0x80B31F80
fn_80166F10(0.0f, this + 0x5C0, 0);
fn_800690E0(this + 0x524);
```

The `&model` / `&anim` / `&dmg` locals are the exact stack slots the original
passes. `r1+0x18` duplicates the model word at `r1+0x0C`, `r1+0x20` duplicates
the `fly` handle at `r1+0x10`, and `r1+0x1C` duplicates the `+0x5BC` handle at
`r1+0x08`; the four calls that take a pointer read those duplicates.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80B0F5E0` | `0x800690C0` | `r3 = this+0x524`, `r4 = -1`, `r5 = *(0x80377C48)`, `r6 = 0`, `r7 = 0x20` |
| `0x80B0F5F8` | `0x800DF180` | `r3 = *(0x8042A038)+4`, `r4 = 0x80B687C0`, `r5 = 0x80B687AC` |
| `0x80B0F608` | `0x80239E30` | `r3 = this+0x540`, `r4 = 0x80B687C0` |
| `0x80B0F62C` | `0x80169D90` | `r3 = this+0x544`, `r4 = r1+0x18`, `r5 = this+0x524`, `r6 = 0x23`, `r7 = 1`, `r8 = 0` |
| `0x80B0F634` | `0x80064BD0` | `r3 = this+0x544` |
| `0x80B0F640` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = 0x80B687CC` |
| `0x80B0F664` | `0x801650D0` | `r3 = this+0x584`, `r4 = r1+0x14`, `r5 = r1+0x10`, `r6 = this+0x524`, `r7 = 0` |
| `0x80B0F67C` | `0x801651F0` | `r3 = this+0x584`, `r4 = this+0x544`, `r5 = r1+0x20`, `r6 = 0` |
| `0x80B0F690` | `0x80169F80` | `f1 = 1.0`, `r3 = this+0x544`, `r4 = this+0x584` |
| `0x80B0F69C` | `0x801680E0` | `f1 = 1.0`, `r3 = this+0x584` |
| `0x80B0F6A8` | `0x8023A200` | `r3 = this+0x540`, `r4 = 0x80B687C0` |
| `0x80B0F6D0` | `0x80166830` | `r3 = this+0x5C0`, `r4 = r1+0x0C`, `r5 = r1+0x08`, `r6 = this+0x524`, `r7 = 0`, `r8 = 1` |
| `0x80B0F6F0` | `0x80166BD0` | `r3 = this+0x5C0`, `r4 = this+0x544`, `r5 = r1+0x1C`, `r6 = 0`, `r7 = 1` |
| `0x80B0F708` | vtable `+0x18` of `*(this+0x544)` | `r3 = this+0x544`, `r4 = this+0x5C0` |
| `0x80B0F71C` | `0x80166EF0` | `f1 = 0.0`, `r3 = this+0x5C0`, `r4 = 0` |
| `0x80B0F72C` | `0x80166F10` | `f1 = 0.0`, `r3 = this+0x5C0`, `r4 = 0` |
| `0x80B0F734` | `0x800690E0` | `r3 = this+0x524` |

### Ambiguity

No blocker. The owning class is the `0x718`-byte object whose constructor is
`0x80B0EEF0` (vtable `0x80B687E8`, holder chain
`+0x524/+0x540/+0x544/+0x584/+0x5BC/+0x5C0/+0x604`); that constructor is the
first word of the in-REL class descriptor at `0x80B687A0`, and the caller is
the unledgered state entry that begins just after `0x80B0EFC4` and calls this
binder at `0x80B0F000`. The wave-115 coverage prose groups this bucket as
`met_toge`; the model strings here say `pukupuku`, so the class *name* is
infrastructure prose, not evidence. Behaviour and every operand are literal.

---

## 4. `0x80AD4420` - state callback: dispatch on `+0x854`

* **TSV size:** 404 bytes
* **Reachable body:** 101 instructions; 1 `bl` + 5 `bctrl`; ends at
  `0x80AD45B0 blr` (shared epilogue at `0x80AD459C`)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad4420_FUN_80ad4420.c`
* **Proposed signature:** `pub unsafe fn fn_80ad4420(this: usize, mode: i32)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C`, `r30@+0x08`.
`r31 = this`, `r30 = mode` (the incoming `r4`).

### Constants

| address | type | value |
|---|---|---|
| `0x80353970` | state token | **(ext)** entry-state dispatch (`+0x84C == 2`, `+0x854 == 0`) |
| `0x80B76630` | state token | **(ext)** "leave state 17" / "state 10, mode != 1" |
| `0x80352B74` | state token | **(ext)** "state 10, mode == 1" |
| `0x80B76670` | state token | **(ext)** default-state dispatch |
| `0x80021140` | function | **(ext)** pad callback A |
| `0x80021150` | function | **(ext)** pad callback B |
| `0x80021160` | function | **(ext)** pad callback C |

### Behaviour

Store order at the head of the body is fixed: `+0xB4` first, then the four
bytes, then the state read.

```c
*(f32 *)(this + 0xB4) = *(f32 *)(this + 0x834);   // 0x80AD4440..0x80AD444C
*(u8  *)(this + 0x675) = 0;                       // 0x80AD4450
*(u8  *)(this + 0x676) = 0;
*(u8  *)(this + 0x677) = 0;
*(u8  *)(this + 0x678) = 0;

if (*(u32 *)(this + 0x84C) == 2 && *(u32 *)(this + 0x854) == 0) {
    vt(this, 0xD4) (0x80353970);
    return;
}

fn_80AD42E0(this);                                // 0x80AD4490

switch (*(u32 *)(this + 0x854)) {                 // 0x80AD4494
case 17:
    vt(this, 0xD4) (0x80B76630);
    *(u32 *)(this + 0x854) = 0;
    return;
case 10:
    vt(this, 0xD4) (mode == 1 ? 0x80352B74 : 0x80B76630);
    return;
default:
    break;
}

vt(this, 0xD4) (0x80B76670);

u32 w = *(u32 *)(this + 0x604);
*(u32 *)(this + 0x84C) = 3;                       // 0x80AD455C
*(u16 *)(this + 0x6A0) = 50;                      // 0x80AD4560
*(u32 *)(this + 0x6D0) = 0x80021140;              // 0x80AD4564
*(u32 *)(this + 0x6D4) = 0x80021150;
*(u32 *)(this + 0x6D8) = 0x80021160;
*(u32 *)(this + 0x574) = 0x80021140;              // 0x80AD4570
*(u32 *)(this + 0x578) = 0x80021150;
*(u32 *)(this + 0x57C) = 0x80021160;

// 0x80AD4544 rlwinm. r0,r0,0,22,22 - bit 22 (0x200) of the word read above
if (w & 0x200)
    *(u32 *)(this + 0x604) = w & ~0x200;          // 0x80AD4584, rlwinm 0,23,21
else
    *(u32 *)(this + 0x604) = w & ~0x80;           // 0x80AD4594, rlwinm 0,25,23
```

### Call sites

Every dispatch uses the same shape: `lwz r12,0x60(rX)`, `lwz r12,0xD4(r12)`,
`mtspr CTR`, `bctrl`, with `r3 = this` and `r4` = the token below.

| site | callee | arguments at the call |
|---|---|---|
| `0x80AD4484` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80353970` |
| `0x80AD4490` | `0x80AD42E0` | `r3 = this` |
| `0x80AD44B8` | vtable `+0xD4` | `r3 = this`, `r4 = 0x80B76630` |
| `0x80AD44F0` | vtable `+0xD4` | `r3 = this`, `r4 = 0x80352B74` |
| `0x80AD4510` | vtable `+0xD4` | `r3 = this`, `r4 = 0x80B76630` |
| `0x80AD4530` | vtable `+0xD4` | `r3 = this`, `r4 = 0x80B76670` |

### Ambiguity

The structure, all five branches and every store are literal. Two role
inferences: `+0x84C` is a state-phase word (`== 2` gates the entry dispatch,
written `3` on the default path) and `+0x854` is the sub-state index
(`0`, `10`, `17` are the only compared values). The six function-pointer
stores are two three-entry pad/event tables. `0x80AD42E0` is untranslated and
is cited by address.

---

## 5. `0x80A7FB50` - icicle model matrix refresh, scale from the kind test

* **TSV size:** 392 bytes
* **Reachable body:** 98 instructions; 12 `bl` + 2 `bctrl`; ends at
  `0x80A7FCD4 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7fb50_FUN_80a7fb50.c`
* **Proposed signature:** `pub unsafe fn fn_80a7fb50(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0xA0(r1)`; LR at `+0xA4`; `f31` at `+0x90` (`stfd` + `psq_st qr0`);
`r31@+0x8C`, `r30@+0x88`. `r30 = this`, `r31 = 0x80B2E298` (constant page).
Scratch: `r1+0x14..0x1C` (three `u16` angles), `r1+0x1C..0x24` (three `f32`
position), `r1+0x28` / `r1+0x58` (48-byte matrices), `r1+0x10`/`0x0C`/`0x08`
(single `u16` rotation arguments).

### Constants

| address | type | value |
|---|---|---|
| `0x80B2E300` | `f32` | `10.0` (scale when the kind test fails) |
| `0x80B2E304` | `f32` | `2.0` (scale when it succeeds) |
| `0x80B2E2F8` | `f32` | `0.0` |
| `0x80B73438` | type token | **(ext)** kind-test token |

### Behaviour

```c
p[0] = *(f32 *)(this + 0xAC);
p[1] = *(f32 *)(this + 0xB0);
p[2] = *(f32 *)(this + 0xB4);                 // to r1+0x1C/20/24
a[0] = *(s16 *)(this + 0x100);
a[1] = *(s16 *)(this + 0x102);
a[2] = *(s16 *)(this + 0x104);                // to r1+0x14/16/18
fn_80064AF0(&p, &a, 1);                       // r3 = r1+0x1C, r4 = r1+0x14

f32 scale = 10.0f;                            // 0x80B2E300
u32 kind = kind_test(this, 0x80B73438);       // 0x80A7FBCC / 0x80A7FBE4
if (kind != 0)                                // neg/or/rlwinm. r0,r0,1,31,31
    scale = 2.0f;                             // 0x80B2E304

fn_801C0BD0(this + 0x7C, p[0], p[1], p[2]);

u16 yaw = (u16)a[1]; fn_8016EDD0(this + 0x7C, &yaw);

fn_801C0BD0(&m1, 0.0f, scale, 0.0f);          // 0x80B2E2F8, r1+0x58
fn_801C0540(this + 0x7C, &m1, this + 0x7C);

u16 pitch = (u16)a[0]; fn_8016ECB0(this + 0x7C, &pitch);

fn_801C0BD0(&m2, 0.0f, -scale, 0.0f);         // r1+0x28
fn_801C0540(this + 0x7C, &m2, this + 0x7C);

u16 roll = (u16)a[2]; fn_8016EEF0(this + 0x7C, &roll);

fn_8016A170(this + 0x544, this + 0x7C);       // copy into the render holder
fn_8016A150(this + 0x544, this + 0xDC);       // offset from the +0xDC vector
fn_8016A1A0(this + 0x544, 0);
```

Note the rotation order: **Y, then the `+scale` translate, then X, then the
`-scale` translate, then Z** - the translates sit *between* the rotations, so
a port that applies all three rotations first produces a different matrix.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A7FBB4` | `0x80064AF0` | `r3 = r1+0x1C`, `r4 = r1+0x14`, `r5 = 1` |
| `0x80A7FBCC` | vtable `+0x28` of `*(this+0x394)` | `r3 = this+0x394` |
| `0x80A7FBE4` | vtable `+0x14` of the returned object | `r3 = obj`, `r4 = 0x80B73438` |
| `0x80A7FC0C` | `0x801C0BD0` | `r3 = this+0x7C`, `f1..f3 = p[0..2]` |
| `0x80A7FC20` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = r1+0x10` (`u16` of `+0x102`) |
| `0x80A7FC34` | `0x801C0BD0` | `r3 = r1+0x58`, `f1 = 0.0`, `f2 = scale`, `f3 = 0.0` |
| `0x80A7FC44` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x58`, `r5 = this+0x7C` |
| `0x80A7FC58` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = r1+0x0C` (`+0x100`) |
| `0x80A7FC6C` | `0x801C0BD0` | `r3 = r1+0x28`, `f1 = 0.0`, `f2 = -scale`, `f3 = 0.0` |
| `0x80A7FC7C` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x28`, `r5 = this+0x7C` |
| `0x80A7FC90` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = r1+0x08` (`+0x104`) |
| `0x80A7FC9C` | `0x8016A170` | `r3 = this+0x544`, `r4 = this+0x7C` |
| `0x80A7FCA8` | `0x8016A150` | `r3 = this+0x544`, `r4 = this+0xDC` |
| `0x80A7FCB4` | `0x8016A1A0` | `r3 = this+0x544`, `r4 = 0` |

### Ambiguity

No in-REL blocker. The kind test consumes only the token as `r4`, never
arithmetically. `+0x394` is the owner object, `+0x544` the render holder and
`+0xDC` the offset vector - roles inferred from the `daEnIcicle_c` neighbours
(`fn_80a7f930`) and from the identical shape in spec 129 #1. `0x8016ECB0` is
`ui::lyt` in the ledger and `0x801C0BD0`/`0x801C0540` are `runtime::dvd`; only
the module tags differ, the bodies are the matrix helpers.

---

## 6. `0x80ACC740` - per-frame step with a temporary `+0xAC` floor

* **TSV size:** 388 bytes
* **Reachable body:** 97 instructions; 11 `bl` + 3 `bctrl`; ends at
  `0x80ACC8C0 blr` (shared epilogue at `0x80ACC8A0`)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80acc740_FUN_80acc740.c`
* **Proposed signature:** `pub unsafe fn fn_80acc740(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `f31` at `+0x20` (`stfd` + `psq_st qr0`);
`r31@+0x1C`, `r30@+0x18`, `r29@+0x14`. `r29 = this`, `r30 = the body object
returned by `0x80070CB0`, `r31` = clamp flag.

### Constants

| address | type | value |
|---|---|---|
| `0x80B30728` | `f32` | `264.0` (temporary `+0xAC` floor) |
| `0x80B306B0` | `f32` | `0.0` (zeroed `+0xEC` on contact) |
| `0x80B76178` | state token | **(ext)** "flip side" dispatch |
| `0x80B76078` | state token | **(ext)** "turn" dispatch |

### Behaviour

```c
vt(this + 0x548, 0x1C);                     // 0x80ACC764, r3 = this+0x548
fn_8006CDE0(this);
fn_800955F0(this);

f32 saved = *(f32 *)(this + 0xAC);          // 0x80ACC78C
s32 clamped = 0;
if (saved <= 264.0f) {                      // fcmpo + cror 2,0,2 + bne
    *(f32 *)(this + 0xAC) = 264.0f;         // 0x80B30728
    clamped = 1;
}

fn_80096F60(this);
usize body = fn_80070CB0(this + 0x1EC);     // 0x80ACC7B4
fn_80ACAE70(this, body);
if (body != 0)
    *(f32 *)(this + 0xEC) = 0.0f;           // 0x80B306B0

fn_8006EF40(this + 0x1EC, *(u32 *)(this + 0x274));
fn_80097050(this);
if (clamped)
    *(f32 *)(this + 0xAC) = saved;          // 0x80ACC800: restore the height

fn_80ACB200(this);
fn_80ACB330(this);

u32 out;
if (fn_80ACB280(this, &out) != 0) {         // 0x80ACC81C
    u8 side = *(u8 *)(this + 0x348);
    if ((u32)side != out)
        *(u8 *)(this + 0x348) = side ^ 1;   // 0x80ACC838 `xoris r0,r3,1`
    vt(this, 0xD4) (0x80B76178);
} else {
    s16 target = *(s16 *)(this + 0x5D0 + side_of(this) * 2);   // lha 0x5D0
    if (fn_8015F4F0(this + 0x102, target, 0x400) != 0)
        vt(this, 0xD4) (0x80B76078);
}
```

The `side_of(this)` index is `*(u8 *)(this + 0x348)`, loaded at `0x80ACC860`,
shifted left by one (`slwi r0,r0,1`) and added to `this`; the halfword comes
from `lha r4,0x5D0(r4)`.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80ACC774` | vtable `+0x1C` of `*(this+0x548)` | `r3 = this+0x548` |
| `0x80ACC77C` | `0x8006CDE0` | `r3 = this` |
| `0x80ACC784` | `0x800955F0` | `r3 = this` |
| `0x80ACC7B4` | `0x80096F60` | `r3 = this` |
| `0x80ACC7BC` | `0x80070CB0` | `r3 = this+0x1EC` |
| `0x80ACC7CC` | `0x80ACAE70` | `r3 = this`, `r4 = body` |
| `0x80ACC7EC` | `0x8006EF40` | `r3 = this+0x1EC`, `r4 = *(u32 *)(this+0x274)` |
| `0x80ACC7F4` | `0x80097050` | `r3 = this` |
| `0x80ACC808` | `0x80ACB200` | `r3 = this` |
| `0x80ACC810` | `0x80ACB330` | `r3 = this` |
| `0x80ACC81C` | `0x80ACB280` | `r3 = this`, `r4 = r1+0x08` |
| `0x80ACC858` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B76178` |
| `0x80ACC878` | `0x8015F4F0` | `r3 = this+0x102`, `r4 = (s16)` table entry, `r5 = 0x400` |
| `0x80ACC89C` | vtable `+0xD4` | `r3 = this`, `r4 = 0x80B76078` |

### Ambiguity

The height clamp is restored on the same frame, so the visible effect is only
on the nested calls - reproduce the save/restore exactly. `+0x548` is a
sub-object, `+0x1EC` a body/physics object, `+0x5D0` a two-entry `s16` target
table indexed by the `+0x348` side byte, and `+0x274` a node handle - all
inferred from the call shapes. `0x80ACB200`, `0x80ACB330` and `0x80ACB280` are
untranslated and are cited by address.

---

## 7. `0x80AE63D0` - `pukupuku` matrix refresh with `+0xE54` scale

* **TSV size:** 388 bytes
* **Reachable body:** 97 instructions; 14 `bl` + 0 `bctrl`; ends at
  `0x80AE6550 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ae63d0_FUN_80ae63d0.c`
* **Proposed signature:** `pub unsafe fn fn_80ae63d0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0xD0(r1)`; LR at `+0xD4`; `r31@+0xCC`, `r30@+0xC8`. `r30 = this`.
Scratch: `r1+0x14/0x16/0x18` (the three `u16` angles),
`r1+0x08/0x0C/0x10` (the single-`u16` rotation copies),
`r1+0x1C..0x24` (three `f32` position), `r1+0x28..0x30` (three `f32` offset),
`r1+0x38` and `r1+0x68` (48-byte matrices), `r1+0x98..0xC8` (row read-back).

### Constants

| address | type | value |
|---|---|---|
| `0x80B30FD8` | `f32` | `0.0` |

Everything else in this body is an actor field: the scale is
`*(f32 *)(this + 0xE54)` and the offset vector is `+0x464/+0x468/+0x46C`.

### Behaviour

```c
p[0] = *(f32 *)(this + 0xAC);
p[1] = *(f32 *)(this + 0xB0);
p[2] = *(f32 *)(this + 0xB4);
a[0] = *(s16 *)(this + 0x100);
a[1] = *(s16 *)(this + 0x102);
a[2] = *(s16 *)(this + 0x104);
fn_80064AF0(&p, &a, 1);

fn_801C0BD0(this + 0x7C, p[0], p[1], p[2]);

u16 yaw = (u16)a[1]; fn_8016EDD0(this + 0x7C, &yaw);

f32 scale = *(f32 *)(this + 0xE54);
fn_801C0BD0(&m1, 0.0f, scale, 0.0f);          // f1 = 0, f2 = scale, f3 = 0
fn_801C0540(this + 0x7C, &m1, this + 0x7C);

u16 pitch = (u16)a[0]; fn_8016ECB0(this + 0x7C, &pitch);

fn_801C0BD0(&m2, 0.0f, -scale, 0.0f);
fn_801C0540(this + 0x7C, &m2, this + 0x7C);

u16 roll = (u16)a[2]; fn_8016EEF0(this + 0x7C, &roll);

fn_8016A170(this + 0x544, this + 0x7C);

o[0] = *(f32 *)(this + 0x464);
o[1] = *(f32 *)(this + 0x468);
o[2] = *(f32 *)(this + 0x46C);
fn_8016A150(this + 0x544, &o);
fn_8016A1A0(this + 0x544, 0);
fn_8016A0F0(this + 0x544, 2, 1);              // 0x80AE6510
fn_80168180(this + 0x544, 3, &rows);          // 0x80AE6520, r5 = r1+0x98

*(f32 *)(this + 0x60C) = *(f32 *)(r1 + 0xA4);
*(f32 *)(this + 0x610) = *(f32 *)(r1 + 0xB4);
*(f32 *)(this + 0x614) = *(f32 *)(r1 + 0xC4);
```

The three read-back words are the translation column of the 48-byte block at
`r1+0x98`: offsets `+0x0C`, `+0x1C` and `+0x2C` inside that block (rows 0, 1
and 2, element 3), and they land in `+0x60C`, `+0x610` and `+0x614` in that
order. The `f2`/`f1`/`f0` load order in the export is `0xC4`, `0xB4`, `0xA4`,
i.e. the stores run last-row-first; keep the three stores in address order.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AE6424` | `0x80064AF0` | `r3 = r1+0x1C`, `r4 = r1+0x14`, `r5 = 1` |
| `0x80AE6438` | `0x801C0BD0` | `r3 = this+0x7C`, `f1..f3 = p[0..2]` |
| `0x80AE644C` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = r1+0x10` |
| `0x80AE6464` | `0x801C0BD0` | `r3 = r1+0x68`, `f1 = 0.0`, `f2 = *(this+0xE54)`, `f3 = 0.0` |
| `0x80AE6474` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x68`, `r5 = this+0x7C` |
| `0x80AE6488` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = r1+0x0C` |
| `0x80AE64A0` | `0x801C0BD0` | `r3 = r1+0x38`, `f1 = 0.0`, `f2 = -*(this+0xE54)`, `f3 = 0.0` |
| `0x80AE64B0` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x38`, `r5 = this+0x7C` |
| `0x80AE64C4` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = r1+0x08` |
| `0x80AE64D0` | `0x8016A170` | `r3 = this+0x544`, `r4 = this+0x7C` |
| `0x80AE64F4` | `0x8016A150` | `r3 = this+0x544`, `r4 = r1+0x28` (`+0x464..0x46C`) |
| `0x80AE6500` | `0x8016A1A0` | `r3 = this+0x544`, `r4 = 0` |
| `0x80AE6510` | `0x8016A0F0` | `r3 = this+0x544`, `r4 = 2`, `r5 = 1` |
| `0x80AE6520` | `0x80168180` | `r3 = this+0x544`, `r4 = 3`, `r5 = r1+0x98` |

### Ambiguity

None in-REL. `0x8016A0F0(holder, 2, 1)` is the only call in this set that has
no counterpart in the wave-128/129 matrix builders; the export shows the three
literal operands and it is safe to call by address. `+0xE54` is the actor's
vertical scale (used with both signs), `+0x60C..0x614` the row-3 read-back.

---

## 8. `0x80AB1160` - two-matrix builder (`z+32`, `z+64`)

* **TSV size:** 384 bytes
* **Reachable body:** 96 instructions; 16 `bl` + 0 `bctrl`; ends at
  `0x80AB12DC blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab1160_FUN_80ab1160.c`
* **Proposed signature:** `pub unsafe fn fn_80ab1160(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `r31@+0x3C`, `r30@+0x38`. `r30 = this`.
Scratch: `r1+0x20/0x22/0x24` (the three zero `u16` angles passed to
`0x80064AF0`), `r1+0x28..0x30` (the three `f32` position), and
`r1+0x08/0x0C/0x10` plus `r1+0x14/0x18/0x1C` (the single-`u16` rotation
arguments - one copy per call, all zero).

### Constants

| address | type | value |
|---|---|---|
| `0x80B2FB0C` | `f32` | `32.0` (the two matrix separations) |

### Behaviour

```c
p[0] = *(f32 *)(this + 0x89C);
p[1] = *(f32 *)(this + 0x8A0);
p[2] = *(f32 *)(this + 0x8A4) + 32.0f;        // 0x80B2FB0C
*(u16 *)(r1 + 0x20) = 0;                      // angle triplet: 0, 0, 0
*(u16 *)(r1 + 0x22) = 0;
*(u16 *)(r1 + 0x24) = 0;
fn_80064AF0(&p, (u16 *)(r1 + 0x20), 1);

// --- first holder: +0x6C4, at z + 32
fn_801C0BD0(this + 0x7C, p[0], p[1], p[2]);
fn_8016EDD0(this + 0x7C, (u16 *)(r1 + 0x1C)); // Y, copy of the a[1] word
fn_8016ECB0(this + 0x7C, (u16 *)(r1 + 0x18)); // X, copy of the a[0] word
fn_8016EEF0(this + 0x7C, (u16 *)(r1 + 0x14)); // Z, copy of the a[2] word
fn_8016A170(this + 0x6C4, this + 0x7C);
fn_8016A150(this + 0x6C4, this + 0xDC);
fn_8016A1A0(this + 0x6C4, 0);

// --- second holder: +0x684, at z + 64
p[2] = p[2] + 32.0f;                          // 0x80AB1230..
fn_801C0BD0(this + 0x7C, p[0], p[1], p[2]);
fn_8016EDD0(this + 0x7C, (u16 *)(r1 + 0x10)); // Y, second copy
fn_8016ECB0(this + 0x7C, (u16 *)(r1 + 0x0C)); // X, second copy
fn_8016EEF0(this + 0x7C, (u16 *)(r1 + 0x08)); // Z, second copy
fn_8016A170(this + 0x684, this + 0x7C);
fn_8016A150(this + 0x684, this + 0xDC);
fn_8016A1A0(this + 0x684, 0);

fn_80AB12E0(this, 1, 0, (u8)*(u32 *)(this + 0x9A0));
```

Two details that a naive port gets wrong: the `+32.0` is added to the **third**
component only (`+0x8A4`, the z slot of the position triple), and the second
pass re-reads the angle words from fresh stack slots
(`r1+0x10/0x0C/0x08`, not the `r1+0x20` triplet)
- they are zero because nothing wrote them, but the register dependency is on
those slots.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AB11B8` | `0x80064AF0` | `r3 = r1+0x28`, `r4 = r1+0x20`, `r5 = 1` |
| `0x80AB11CC` | `0x801C0BD0` | `r3 = this+0x7C`, `f1..f3 = p[0..2]` |
| `0x80AB11E0` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = r1+0x1C` |
| `0x80AB11F4` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = r1+0x18` |
| `0x80AB1208` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = r1+0x14` |
| `0x80AB1214` | `0x8016A170` | `r3 = this+0x6C4`, `r4 = this+0x7C` |
| `0x80AB1220` | `0x8016A150` | `r3 = this+0x6C4`, `r4 = this+0xDC` |
| `0x80AB122C` | `0x8016A1A0` | `r3 = this+0x6C4`, `r4 = 0` |
| `0x80AB124C` | `0x801C0BD0` | `r3 = this+0x7C`, `f1..f3 = p[0..2]` (z + 64) |
| `0x80AB1260` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = r1+0x10` |
| `0x80AB1274` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = r1+0x0C` |
| `0x80AB1288` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = r1+0x08` |
| `0x80AB1294` | `0x8016A170` | `r3 = this+0x684`, `r4 = this+0x7C` |
| `0x80AB12A0` | `0x8016A150` | `r3 = this+0x684`, `r4 = this+0xDC` |
| `0x80AB12AC` | `0x8016A1A0` | `r3 = this+0x684`, `r4 = 0` |
| `0x80AB12C4` | `0x80AB12E0` | `r3 = this`, `r4 = 1`, `r5 = 0`, `r6 = (u8)*(u32 *)(this+0x9A0)` |

### Ambiguity

No external datum. `+0x6C4` and `+0x684` are the two mirrored render holders,
`+0x89C..0x8A4` the source position, `+0x9A0` a variant byte narrowed with
`clrlwi r6,r0,24`. `0x80AB12E0` is untranslated (adjacent descriptor entry).

---

## 9. `0x80A33630` - matrix refresh with `(0, +10, -4)` / `(0, -10, +4)`

* **TSV size:** 380 bytes
* **Reachable body:** 95 instructions; 13 `bl` + 0 `bctrl`; ends at
  `0x80A337A8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a33630_FUN_80a33630.c`
* **Proposed signature:** `pub unsafe fn fn_80a33630(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0xC0(r1)`; LR at `+0xC4`; `f31` at `+0xB0` and `f30` at `+0xA0`
(each `stfd` + `psq_st qr0`); `r31@+0x9C`, `r30@+0x98`. `r30 = this`,
`r31 = 0x80B2BF48` (constant page).
Scratch: `r1+0x14/0x16/0x18` (the three `u16` angles),
`r1+0x08/0x0C/0x10` (the single-`u16` rotation copies),
`r1+0x1C..0x24` (three `f32` position), `r1+0x28..0x30` (three `f32` offset),
`r1+0x38` and `r1+0x68` (48-byte matrices).

### Constants

| address | type | value |
|---|---|---|
| `0x80B2BF50` | `f32` | `0.0` |
| `0x80B2BFD8` | `f32` | `10.0` (held in `f31`) |
| `0x80B2BFDC` | `f32` | `-4.0` (held in `f30`) |

### Behaviour

```c
p[0..2] = *(f32 *)(this + 0xAC/0xB0/0xB4);
a[0..2] = *(s16 *)(this + 0x100/0x102/0x104);
fn_80064AF0(&p, &a, 1);

f31 = 10.0f;                                   // 0x80B2BFD8
f30 = -4.0f;                                   // 0x80B2BFDC

fn_801C0BD0(this + 0x7C, p[0], p[1], p[2]);
fn_8016EDD0(this + 0x7C, &(u16)a[1]);          // Y first

fn_801C0BD0(&m1, 0.0f, f31, f30);              // (0, +10, -4)
fn_801C0540(this + 0x7C, &m1, this + 0x7C);

fn_8016ECB0(this + 0x7C, &(u16)a[0]);          // X
fn_8016EEF0(this + 0x7C, &(u16)a[2]);          // Z

fn_801C0BD0(&m2, 0.0f, -f31, -f30);            // (0, -10, +4)
fn_801C0540(this + 0x7C, &m2, this + 0x7C);

fn_8016A170(this + 0x544, this + 0x7C);
o[0..2] = *(f32 *)(this + 0x464/0x468/0x46C);
fn_8016A150(this + 0x544, &o);
fn_8016A1A0(this + 0x544, 0);
fn_80A33470(this);
```

Rotation order here differs from #5/#7: **Y, translate, X, Z, translate** (the
second translate comes after Z). Keep it.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A3369C` | `0x80064AF0` | `r3 = r1+0x1C`, `r4 = r1+0x14`, `r5 = 1` |
| `0x80A336B8` | `0x801C0BD0` | `r3 = this+0x7C`, `f1..f3 = p[0..2]` |
| `0x80A336CC` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = r1+0x10` |
| `0x80A336E0` | `0x801C0BD0` | `r3 = r1+0x68`, `f1 = 0.0`, `f2 = 10.0`, `f3 = -4.0` |
| `0x80A336F0` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x68`, `r5 = this+0x7C` |
| `0x80A33704` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = r1+0x0C` |
| `0x80A33718` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = r1+0x08` |
| `0x80A3372C` | `0x801C0BD0` | `r3 = r1+0x38`, `f1 = 0.0`, `f2 = -10.0`, `f3 = 4.0` |
| `0x80A3373C` | `0x801C0540` | `r3 = this+0x7C`, `r4 = r1+0x38`, `r5 = this+0x7C` |
| `0x80A33748` | `0x8016A170` | `r3 = this+0x544`, `r4 = this+0x7C` |
| `0x80A3376C` | `0x8016A150` | `r3 = this+0x544`, `r4 = r1+0x28` (`+0x464..0x46C`) |
| `0x80A33778` | `0x8016A1A0` | `r3 = this+0x544`, `r4 = 0` |
| `0x80A33780` | `0x80A33470` | `r3 = this` |

### Ambiguity

No external datum. `0x80A33470` is translated; the call passes only
`this`. Field roles (`+0x7C` matrix, `+0x544` holder, `+0x464..0x46C` offset
vector) are inferred from the sibling builders but appear identically in
wave-129 #1 and in #7 above.

---

## 10. `0x80B1E540` - two-state `+0x360` machine with a speed ramp

* **TSV size:** 380 bytes
* **Reachable body:** 95 instructions; 5 `bl` + 5 `bctrl`; ends at
  `0x80B1E6B8 blr` (shared epilogue at `0x80B1E69C`)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b1e540_FUN_80b1e540.c`
* **Proposed signature:** `pub unsafe fn fn_80b1e540(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `f31` at `+0x10` (`stfd` + `psq_st qr0`);
`r31@+0x0C`, `r30@+0x08`. `r30 = this`, `r31 = 0x80B32558` (constant page).

### Constants

| address | type | value |
|---|---|---|
| `0x80B32640` | `f32` | `1.0` (per-frame ramp step) |
| `0x80B32684` | `f32` | `8.0` (ramp limit) |
| `0x80B32690` | `f32` | `4.0` (added to the queried time) |
| `0x80371B88` | state token | **(ext)** "leave" dispatch (state 1 exit) |
| `0x80371B48` | state token | **(ext)** "enter" dispatch (state 2 exit) |
| `0x80B79378` | state token | **(ext)** actor-vtable dispatch on the state-2 exit |

### Behaviour

```c
vt(this + 0x544, 0x1C);                       // 0x80B1E578
vt(this + 0x5C0, 0x14);                       // 0x80B1E58C

u32 state = *(u32 *)(this + 0x360);           // 0x80B1E590
if (state == 1) {
    *(f32 *)(this + 0x10C) += 1.0f;           // 0x80B32640
    if (*(f32 *)(this + 0x10C) > 8.0f)        // fcmpo + ble
        *(f32 *)(this + 0x10C) = 8.0f;        // 0x80B32684

    f32 ramp = *(f32 *)(this + 0x10C);
    *(f32 *)(this + 0xE8) = ramp * *(f32 *)(this + 0x614);
    *(f32 *)(this + 0xEC) = ramp * *(f32 *)(this + 0x618);

    fn_8006CD40(this);                        // no float arguments here
    fn_8015F4F0(this + 0x104, 0, 0x480);       // 1152

    f32 time = fn_800E9020(this + 0x668);     // returns f1
    f31 = 4.0f + time;                        // 0x80B32690

    if (fn_80B1D680(this) >= f31) {           // fcmpo + cror 2,1,2 + bne
        fn_80B1DA40(this);
        vt(this + 0x694, 0x18) (0x80371B88);
        *(u16 *)(this + 0x500) = 120;         // li r3,0x78
        *(u32 *)(this + 0x360) = 2;
    }
} else if (state == 2) {
    if (*(u16 *)(this + 0x500) == 0) {        // 0x80B1E658
        vt(this + 0x694, 0x18) (0x80371B48);
        vt(this, 0xD4) (0x80B79378);
    }
}
```

`fn_8006CD40` is called with `r3 = this` and **no** float argument prepared in
this body (unlike #1 above, which sets `f1..f3` immediately before the same
helper); the port must call it with the actor only.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80B1E578` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80B1E58C` | vtable `+0x14` of `*(this+0x5C0)` | `r3 = this+0x5C0` |
| `0x80B1E5E8` | `0x8006CD40` | `r3 = this` |
| `0x80B1E5F8` | `0x8015F4F0` | `r3 = this+0x104`, `r4 = 0`, `r5 = 0x480` |
| `0x80B1E600` | `0x800E9020` | `r3 = this+0x668` |
| `0x80B1E610` | `0x80B1D680` | `r3 = this` |
| `0x80B1E624` | `0x80B1DA40` | `r3 = this` |
| `0x80B1E640` | vtable `+0x18` of `*(this+0x694)` | `r3 = this+0x694`, `r4 = 0x80371B88` |
| `0x80B1E67C` | vtable `+0x18` of `*(this+0x694)` | `r3 = this+0x694`, `r4 = 0x80371B48` |
| `0x80B1E698` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B79378` |

### Ambiguity

The `fn_800E9020` return is a time value in `f1` (`f31 = 4.0 + f1`); its unit
is not established here, only the arithmetic. `+0x360` is the state word,
`+0x500` a `u16` countdown, `+0x694` a state-controller sub-object, `+0x544`
and `+0x5C0` render holders, `+0x614`/`+0x618` the two speed factors. The
`>=` comparison is exact (`cror 2,1,2` after `fcmpo`). `0x80B1DA40` is
untranslated.

---

## 11. `0x80A7BD30` - `walk_*` clip-pair selector

* **TSV size:** 372 bytes
* **Reachable body:** 93 instructions; 10 `bl` + 1 `bctrl`; ends at
  `0x80A7BEA0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7bd30_FUN_80a7bd30.c`
* **Proposed signature:** `pub unsafe fn fn_80a7bd30(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `f31` at `+0x20` (`stfd` + `psq_st qr0`);
`r31@+0x1C`, `r30@+0x18`. `r31 = this`, `r30 = 0x80B2DDF0` (table page).

### Constants

| address | type | value |
|---|---|---|
| `0x80B2DE58` | `u32[72]` | in-REL clip table: 2 sides x 18 eight-byte `{walk_st_*, walk_ed_*}` name pairs (288 bytes; the unrelated `wait_A_*` table starts at `0x80B2DF78`). Side 0 covers `walk_st_AR_*`/`walk_ed_BR_*`, side 1 `walk_st_AL_*`/`walk_ed_BL_*`, each in `m`/`u`/`d` variants |
| `0x80B2DE38` | `u8[4]` | in-REL per-side index table, bytes `{0, 1, 1, 0}` (indexed `base + side*2 + sub-state`) |
| `0x80B2DE40` | `f32` | `45.0` |
| `0x80B2DE44` | `f32[]` | `{40.0, 0.25, 0.125, ...}` - stored as a pointer into `+0x618` |
| `0x80B2DE28` | `f32` | `1.0` |
| `0x80B2DE20` | `f32` | `0.0` |

### Behaviour

```c
u32 off = 0x80B2DE58
        + (u32)*(u8 *)(this + 0x348) * 0x90      // side
        + (u32)*(u8 *)(this + 0xEA1) * 0x48      // sub-state 1
        + (u32)*(u8 *)(this + 0xEA2) * 0x18      // sub-state 2
        + (u32)*(u8 *)(this + 0xEA3) * 0x08;     // sub-state 3
u32 name = *(u32 *)off;                          // lwzx r4,r4,r0
*(u32 *)(this + 0x614) = off + 4;                 // the pair's walk_ed_* word

u32 anim = fn_8023A0B0(this + 0x540, name);      // 0x80A7BD9C
fn_801651F0(this + 0x584, this + 0x544, &anim, 1);
fn_80169F80(1.0f, this + 0x544, this + 0x584);   // 0x80B2DE28
fn_801680E0(1.0f, this + 0x584);
vt(this + 0x544, 0x1C);
fn_80168FE0(0.0f, this + 0x584);                 // 0x80B2DE20

f31 = *(f32 *)0x80B2DE40;                         // "45.0" animation rate
*(u8 *)(this + 0xE9B) = 0;                       // 0x80A7BE10
*(u8 *)(this + 0xE9C) = 0;                       // 0x80A7BE14
*(u8 *)(this + 0xEA6) = *(u8 *)(this + 0xEA5);   // 0x80A7BE18
*(u32 *)(this + 0x618) = 0x80B2DE44;             // 0x80A7BE1C

fn_80A7A2C0(45.0f, this);                        // f1 first
fn_80A7A360(45.0f, this);

u8 side = *(u8 *)(this + 0x348);
u8 idx  = *(u8 *)(0x80B2DE38 + side * 2 + *(u8 *)(this + 0xEA1));  // lbzx
*(u8 *)(this + 0xEA8) &= (u8)~(1u << idx);       // 0x80A7BE54..0x80A7BE64
fn_8008C380(this + 0xD1C + idx * 0xA4);          // 0x80A7BE70
fn_80A7A030(this);
fn_80A7A170(this);
```

The index arithmetic is `mulli 0x90`/`mulli 0x48`/`mulli 0x18`/`slwi 3` on the
four bytes (`+0x348`, `+0xEA1`, `+0xEA2`, `+0xEA3`) - note 0x90 (144), not 0x48,
for the side stride, which is why the table is 2 x 0x90 bytes and not 2 x 0x48.
`+0x614` receives the address of the selected entry's *second* word (the
`walk_ed_*` clip), while the first word (the `walk_st_*` clip) is only used for
the look-up in this body. The index-to-clip mapping is in-REL data, not a
jump table.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A7BD9C` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = clip name from the table` |
| `0x80A7BDB4` | `0x801651F0` | `r3 = this+0x584`, `r4 = this+0x544`, `r5 = r1+0x08`, `r6 = 1` |
| `0x80A7BDC4` | `0x80169F80` | `f1 = 1.0`, `r3 = this+0x544`, `r4 = this+0x584` |
| `0x80A7BDD0` | `0x801680E0` | `f1 = 1.0`, `r3 = this+0x584` |
| `0x80A7BDE4` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80A7BDF0` | `0x80168FE0` | `f1 = 0.0`, `r3 = this+0x584` |
| `0x80A7BE20` | `0x80A7A2C0` | `f1 = 45.0`, `r3 = this` |
| `0x80A7BE2C` | `0x80A7A360` | `f1 = 45.0`, `r3 = this` |
| `0x80A7BE70` | `0x8008C380` | `r3 = this+0xD1C+idx*0xA4` |
| `0x80A7BE78` | `0x80A7A030` | `r3 = this` |
| `0x80A7BE80` | `0x80A7A170` | `r3 = this` |

### Ambiguity

The table is in-REL and every index is a `u8` actor byte, so this is not one
of wave 127's "lost jump table" rows. The `(u8)~(1u << idx)` write is derived
from `slw r0,r0,r4` + `nor` + `clrlwi 24` + `and`, exactly as written. Field
roles (`+0x540` model, `+0x544` node holder, `+0x584` animation holder,
`+0x614`/`+0x618` sub-state data, `+0xEA1..0xEA8` clip-selection bytes) are
inferred from the sibling `pukupuku` block.

---

## 12. `0x80AC9AA0` - per-frame step, angle-flip gate, two effect records

* **TSV size:** 372 bytes
* **Reachable body:** 93 instructions; 8 `bl` + 4 `bctrl`; ends at
  `0x80AC9C10 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ac9aa0_FUN_80ac9aa0.c`
* **Proposed signature:** `pub unsafe fn fn_80ac9aa0(this: usize)`
* **Confidence:** **medium / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`, `r29@+0x24`.
`r29 = this`, `r31 = 0x80B30500` (constant page), `r30` = the `+0x102` angle
captured before the turn.
Scratch: `r1+0x08..0x10` and `r1+0x14..0x1C` (two three-`f32` effect records).

### Constants

| address | type | value |
|---|---|---|
| `0x80B30540` | `s16[2]` | `{+0x0E40, -0x0E40}` (turn target per side; `0x0E40` = 3648) |
| `0x80B305F8` | `f32` | `5500.0` (z of both effect records) |
| `0x80B30544` | `f32` | `0.0` |
| `0x80B59D08` | state token | **(ext)** first effect dispatch |
| `0x80B59D18` | state token | **(ext)** second effect dispatch |
| `0x80B75E30` | state token | **(ext)** final dispatch when the holder test fails |

### Behaviour

```c
vt(this + 0x544, 0x1C);                        // 0x80AC9AD4
fn_8006CDE0(this);
fn_800955F0(this);

if (fn_80072BC0(this + 0x1EC) != 0)            // 0x80AC9AEC
    *(f32 *)(this + 0xEC) = 0.0f;              // 0x80B30544

s16 prev = *(s16 *)(this + 0x102);             // 0x80AC9B00
u8 side = (u8)fn_80096240(this, this + 0xAC);  // 0x80AC9B0C
*(u8 *)(this + 0x348) = side;
s16 target = *(s16 *)(0x80B30540 + side * 2);  // lhax, 0x80AC9B20
fn_8015F4F0(this + 0x102, target, 0x400);      // 1024

// first record: (+0x610, +0x614, 5500.0)
r1f[0] = *(f32 *)(this + 0x610);
r1f[1] = *(f32 *)(this + 0x614);
r1f[2] = 5500.0f;                              // 0x80B305F8
vt(this + 0x878, 0xA0) (0x80B59D08, 0, &r1f, 0, 0);

// second record: (+0x604, +0x608, 5500.0)
r2f[0] = *(f32 *)(this + 0x604);
r2f[1] = *(f32 *)(this + 0x608);
r2f[2] = 5500.0f;
vt(this + 0x9A0, 0xA0) (0x80B59D18, 0, &r2f, 0, 0);

// 0x80AC9BB4 mullw. r0,r30,r0 - the turn crossed zero
if ((s32)prev * (s32)*(s16 *)(this + 0x102) <= 0) {
    fn_80168080(this + 0x584);
    fn_80AC9310(this);
}

if (fn_80169020(this + 0x584) != 0)            // 0x80AC9BD0
    vt(this, 0xD4) (0x80B75E30);
```

The sign test is `mullw.` (record form) of the old and new `+0x102`
halfwords, `bgt` skipping the two helpers: the helpers run only when the
product is `<= 0`, i.e. when the angle did not move strictly in the same
direction.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AC9AD4` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80AC9ADC` | `0x8006CDE0` | `r3 = this` |
| `0x80AC9AE4` | `0x800955F0` | `r3 = this` |
| `0x80AC9AEC` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80AC9B0C` | `0x80096240` | `r3 = this`, `r4 = this+0xAC` |
| `0x80AC9B28` | `0x8015F4F0` | `r3 = this+0x102`, `r4 = table target`, `r5 = 0x400` |
| `0x80AC9B80` | vtable `+0xA0` of `*(this+0x878)` | `r3 = this+0x878`, `r4 = 0x80B59D08`, `r5 = 0`, `r6 = r1+0x14`, `r7 = 0`, `r8 = 0` |
| `0x80AC9BAC` | vtable `+0xA0` of `*(this+0x9A0)` | `r3 = this+0x9A0`, `r4 = 0x80B59D18`, `r5 = 0`, `r6 = r1+0x08`, `r7 = 0`, `r8 = 0` |
| `0x80AC9BC0` | `0x80168080` | `r3 = this+0x584` |
| `0x80AC9BC8` | `0x80AC9310` | `r3 = this` |
| `0x80AC9BD0` | `0x80169020` | `r3 = this+0x584` |
| `0x80AC9BF4` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B75E30` |

### Ambiguity

`80096240` returns a player-side index in `r3`; only its low byte is stored
(`stb`) and reused (`rlwinm r0,r3,1,23,30` == `(r3 & 0xFF) * 2`), so the byte
store and the table index must both come from that low byte. `+0x878` and
`+0x9A0` are effect emitters (both slot `+0xA0`), `+0x604..0x618` two
three-float positions, `+0x1EC` a body object, `+0x584` an animation holder.
All roles inferred from the call shapes; the arithmetic is literal.

---

## Verification

* 12 addresses, all absent from `ledger/functions.csv` (checked against the
  `address` column of all 16,139 rows) and all present in
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv` with the sizes quoted in
  the tables above.
* Every address is absent from `docs/SPEC_WAVE128_ENEMIES.md` and
  `docs/SPEC_WAVE129_ENEMIES.md`. `0x80AB2460` appears once in wave 129, and
  only in the hand-off sentence that names it as the next candidate.
* Every body was decoded from `work/rels/elf/d_enemiesNP.elf` with the
  `(address - 0x80A00000) + 0x1000` mapping and walked from the entry through
  every `bc` arm. No path lands on data, no branch leaves the body, and every
  body terminates in `blr` or a jump to its own shared epilogue.
* Reachable instruction counts were cross-checked against the 83 class-B rows
  whose wave-127 note states a count: all 83 match exactly with the decoder
  used here (0 mismatches). That is the calibration for the 64 rows whose
  notes do not state a count, including ranks 25-36 above.
* The record form was honoured: `rlwinm.` at `0x80AD4544` and `0x80A7FBF0`,
  `mullw.` at `0x80AC9BB4` all set `cr0` and are required for the following
  branch.
* Only `qr0` paired-single traffic exists in this set: 6 `psq_st`/`psq_l`
  pairs across 5 bodies, all `f30`/`f31` prologue spills. No `mtspr GQRn`.
* No Rust file, ledger CSV or existing doc was modified by this pass.
