# Wave 129 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 173 of the 384
formerly-skipped `d_enemiesNP` addresses as class **B**: the reachable
instruction stream decodes end-to-end, every path ends in `blr` or a tail call
to a real entry, and nothing depends on a `GQR1..GQR7` quantiser, a jump table,
an unresolved base or an incoming float argument.

Wave 128 (`docs/SPEC_WAVE128_ENEMIES.md`) specified the twelve largest of them.
This lane is the continuation of that list. It is **read-only**: it writes no
Rust, no ledger row and no other doc. It decodes each body from
`work/rels/elf/d_enemiesNP.elf`, and records an implementation-ready
specification: signature, stack frame, every branch, every store, every call
site with its argument registers, every constant with its source address, and
the residual ambiguity.

All twelve were re-checked against the `address` column of
`ledger/functions.csv` (absent) and against `functions.tsv` (present).

## Selection

The ordering metric is the wave-127 "reachable instruction count" (the only
size metric that is not inflated by the TSV extent quirk). Ranks 1-12 of the
class-B pool are the wave-128 set; the twelve rows below are ranks 13-24, in
descending order.

| # | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 1 | `0x80A96230` | 572 | 143 | model-matrix refresh + kind-test sub-block | **ready** |
| 2 | `0x80B0F800` | 560 | 140 | water-wave timered state machine (2 states) | **ready** |
| 3 | `0x80A77C30` | 548 | 137 | two-arm button/pad gate, returns 1 or 2 | **ready** |
| 4 | `0x80AC95D0` | 540 | 135 | Huhu breath/haze actor step (5 sub-objects) | **ready** |
| 5 | `0x80B1DFE0` | 540 | 135 | wall-bounce actor step (direction flip on contact) | **ready** |
| 6 | `0x80ADD350` | 536 | 134 | `block_pata` spawn entry (ground query + 3 callbacks) | **conditional** |
| 7 | `0x80A45DB0` | 500 | 125 | "spawn under water surface" entry | **ready** |
| 8 | `0x80AD5760` | 500 | 125 | 9-cell grid spawn (`+0x4C2C/+0x4C2D` counts) | **ready** |
| 9 | `0x80ACC520` | 492 | 123 | Huhu per-frame step with direction sound | **ready** |
| 10 | `0x80A35170` | 464 | 116 | ice/`crow` slide state aid (angle ramp) | **ready** |
| 11 | `0x80A0BD20` | 460 | 115 | Choropu-like appear handler (2 states) | **conditional** |
| 12 | `0x80A2BC50` | 456 | 114 | two-actor push/bounce effect | **ready** |

The next candidate after this set is `0x80AB2460` at 111 instructions.

Note on the wave-128 hand-off note: it said "the next candidate after this set
is `0x80B0F800` at 140 instructions". That skipped `0x80A96230`, which the
wave-127 table itself lists at 143 reachable instructions and class B. This
lane restores the correct order, so `0x80A96230` is #1 here and
`0x80B0F800` is #2.

"Conditional" means the control flow and every operand are exact, but a named
datum lives in `main.dol` and must be read there before a byte-exact port:
the two floats at `0x8042BB08/0x8042BB0C` for #6, and the byte table based at
`0x8042BB18` for #11 (both are consumed arithmetically or used as an index,
not merely passed through). Neither is a reason to skip.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`; data addresses in
  `0x80B2xxxx` / `0x80B5xxxx` use the same `+0x1000` shift (re-verified against
  `0x80B32DC0` at file offset `0x133DC0`). The REL's image ends at
  `0x80B6E434`; anything above that is a `main.dol` address.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* An address that does not resolve in `d_enemiesNP.elf` is a `main.dol`
  address. It is marked **(ext)** and must be treated as a named external
  datum, not as a missing base.
* Helper names follow the ledger: `0x801C0BD0` = translation-matrix builder
  (`out, x, y, z`), `0x801C0C60` = scale builder, `0x801C0540` = concat
  (`out = a * b`), `0x801C0500`/`0x8016A170` = copy, `0x8016A150` = concat,
  `0x8016A1A0` = identity, `0x8016EDD0`/`0x8016ECB0`/`0x8016EEF0` =
  rotation-by-`u16` about Y/X/Z, `0x80064AF0` = position-from-angle helper,
  `0x8015F4F0` = signed-16 write helper, `0x801680C0`/`0x80168FE0` =
  float get/set on a `+0x584`-style sub-object, `0x80168230`/`0x80168180` =
  matrix row readers, `0x80168400` = matrix flush.
* Branch mnemonics are written in C form after the exact `bc` operand decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 4,0` = `bge`, `bc 12,1` = `bgt`,
  `bc 4,1` = `ble`, `bc 12,0` = `blt`.
* `li rX,imm` is the assembler form of `addi rX,r0,imm` (`rA = 0` means the
  literal zero, not GPR `r0`). Where a store value is a `li`, that is stated.

## Symbolic helpers used below

```text
fn_80a96230(this)                                   # this spec, #1
fn_80b0f800(this)                                   # this spec, #2
fn_80a77c30(this) -> i32                            # this spec, #3
fn_80ac95d0(this)                                   # this spec, #4
fn_80b1dfe0(this)                                   # this spec, #5
fn_80add350(this) -> i32                            # this spec, #6
fn_80a45db0(this) -> i32                            # this spec, #7
fn_80ad5760(this) -> i32                            # this spec, #8
fn_80acc520(this)                                   # this spec, #9
fn_80a35170(this)                                   # this spec, #10
fn_80a0bd20(this)                                   # this spec, #11
fn_80a2bc50(this, other)                            # this spec, #12

vt(obj, slot)            # (*(u32 *)obj)->vt[slot], receiver r3 = obj
kind_test(base, token)   # (*(u32 *)(base+0x394))->vt[0x28](base+0x394)
                         # then (*obj)->vt[0x14](obj, token); returns i32
```

`kind_test` is the literal sequence `lwzu r12,0x394(r3)` / `lwz r12,0x28(r12)`
/ `bctrl` / `lwz r12,0(r3)` / `lwz r12,0x14(r12)` / `bctrl` with `r4 = token`.
The receiver of both calls is explicit, so none of these is one of wave 127's
class-C indirect calls.

---

## 1. `0x80A96230` - model-matrix refresh with kind-test sub-block

* **TSV size:** 572 bytes
* **Reachable body:** 143 instructions; 19 `bl` + 3 `bctrl`; ends at
  `0x80A96468 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a96230_FUN_80a96230.c`
* **Proposed signature:** `pub unsafe fn fn_80a96230(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0xD0(r1)`; LR at `+0xD4`; `stfd f31,+0xC0` + `psq_st f31,qr0,+0xC8`;
`r31@+0xBC`, `r30@+0xB8`. `r30 = this`.

Stack scratch: `+0x28..+0x34` (3 floats from `0x8012DE80`), `+0x40..+0x48`
(3 floats), `+0x14..+0x18` (3 `s16`), `+0x50` and `+0x34` matrix/vector
buffers, `+0x80` matrix.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2ED44` | `f32` | `2.0` |
| `0x80B741F8` | type token | **(ext)** `r4` of the `vt[0x14]` kind call |

All other `lfs` in this body read actor fields or its own stack; the only
global float is `0x80B2ED44`.

### Behaviour

```c
f31 = *(f32 *)(this + 0x5FC);          // 0x80A96250
*(f32 *)(this + 0x5FC) = *(f32 *)(this + 0x5F8);   // 0x80A96258 (temporary)

// 0x80A9625C: lwzu r12,0x394(r3) -> r3 = this+0x394, r12 = *(this+0x394)
obj = (*(u32 *)(this + 0x394))->vt[0x28](this + 0x394);
r   = obj->vt[0x14](obj, 0x80B741F8);              // (ext) token
if (r != 0) {                                       // 0x80A96284..0x80A96290
    o = fn_8005F900((s16)*(u8 *)(this + 0x38D));    // lbz + extsh
    if (fn_80056CF0(o, 4) == 0) {                   // 0x80A962B0 bne
        fn_8012DE80(&v3[0], o);                     // r3 = r1+0x28, r4 = o
        fn_801C0BD0(&mtx, v3[0], v3[1], v3[2]);     // r3 = r1+0x80
        p = fn_80056E30(o);
        fn_801C0540(&mtx, p + 0x108, &mtx);         // r4 = p+0x108, r5 = &mtx
        fn_8016F130(&mtx, this + 0xAC);             // r4 = this+0xAC
    }
}

// 0x80A962FC: rebuild this+0x7C from AC/B0/B4 and 0x100/0x102/0x104
vec[0] = *(f32 *)(this + 0xAC);
vec[1] = *(f32 *)(this + 0xB0);
vec[2] = *(f32 *)(this + 0xB4);                     // to r1+0x40/44/48
ang[0] = *(s16 *)(this + 0x100);                    // lha -> r1+0x14
ang[1] = *(s16 *)(this + 0x102);
ang[2] = *(s16 *)(this + 0x104);
fn_80064AF0(&vec[0], &ang[0], 1);                   // fills vec[2]
fn_801C0BD0(this + 0x7C, vec[0], vec[1], vec[2]);
r4 = (u16)ang[1]; fn_8016EDD0(this + 0x7C, &r4);    // Y rotation
r4 = (u16)ang[0]; fn_8016ECB0(this + 0x7C, &r4);    // X rotation
r4 = (u16)ang[2]; fn_8016EEF0(this + 0x7C, &r4);    // Z rotation

fn_8016A170(this + 0x550, this + 0x7C);             // copy
v3[0] = *(f32 *)(this + 0x464);
v3[1] = *(f32 *)(this + 0x468);
v3[2] = *(f32 *)(this + 0x46C);                     // to r1+0x1C/20/24
fn_8016A150(this + 0x550, &v3[0]);                  // concat
fn_8016A1A0(this + 0x550, 0);                       // identity
fn_80168400(this + 0x550);
fn_80168180(this + 0x550, 2, r1 + 0x50);            // row reader, slot 2
*(f32 *)(this + 0x5A0) = *(f32 *)(r1 + 0x5C);
*(f32 *)(this + 0x5A4) = *(f32 *)(r1 + 0x6C);
*(f32 *)(this + 0x5A8) = *(f32 *)(r1 + 0x7C);

fn_80168230(this + 0x550, 0, r1 + 0x34);            // row reader, slot 0
*(f32 *)(this + 0x5AC) = *(f32 *)(r1 + 0x38) - 2.0f;   // 0x80B2ED44
*(f32 *)(this + 0x5FC) = f31;                       // 0x80A96418 restore
fn_8016A1A0(this + 0x550, 0);

if (*(u32 *)(this + 0x5F4) == 1)                    // 0x80A96430 cmpli 1
    vt(this + 0x654, 0x10);                         // 0x80A96448
```

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A96268` | vtable `+0x28` of `*(this+0x394)` | `r3 = this+0x394` |
| `0x80A96280` | vtable `+0x14` of the returned object | `r3 = obj`, `r4 = 0x80B741F8` |
| `0x80A9629C` | `0x8005F900` | `r3 = (s16)(u8)*(this+0x38D)` |
| `0x80A962A8` | `0x80056CF0` | `r3 = o`, `r4 = 4` |
| `0x80A962BC` | `0x8012DE80` | `r3 = r1+0x28`, `r4 = o` |
| `0x80A962D0` | `0x801C0BD0` | `r3 = r1+0x80`, `f1 = v3[0]`, `f2 = v3[1]`, `f3 = v3[2]` |
| `0x80A962D8` | `0x80056E30` | `r3 = o` |
| `0x80A962EC` | `0x801C0540` | `r3 = r1+0x80`, `r4 = ret+0x108`, `r5 = r1+0x80` |
| `0x80A962F8` | `0x8016F130` | `r3 = r1+0x80`, `r4 = this+0xAC` |
| `0x80A96338` | `0x80064AF0` | `r3 = r1+0x40`, `r4 = r1+0x14`, `r5 = 1` |
| `0x80A9634C` | `0x801C0BD0` | `r3 = this+0x7C`, `f1..f3 = r1+0x40/44/48` |
| `0x80A96360` | `0x8016EDD0` | `r3 = this+0x7C`, `r4 = r1+0x10` (`u16` copy of `+0x102`) |
| `0x80A96374` | `0x8016ECB0` | `r3 = this+0x7C`, `r4 = r1+0x0C` (`+0x100`) |
| `0x80A96388` | `0x8016EEF0` | `r3 = this+0x7C`, `r4 = r1+0x08` (`+0x104`) |
| `0x80A96394` | `0x8016A170` | `r3 = this+0x550`, `r4 = this+0x7C` |
| `0x80A963B8` | `0x8016A150` | `r3 = this+0x550`, `r4 = r1+0x1C` |
| `0x80A963C4` | `0x8016A1A0` | `r3 = this+0x550`, `r4 = 0` |
| `0x80A963CC` | `0x80168400` | `r3 = this+0x550` |
| `0x80A963DC` | `0x80168180` | `r3 = this+0x550`, `r4 = 2`, `r5 = r1+0x50` |
| `0x80A96404` | `0x80168230` | `r3 = this+0x550`, `r4 = 0`, `r5 = r1+0x34` |
| `0x80A96428` | `0x8016A1A0` | `r3 = this+0x550`, `r4 = 0` |
| `0x80A96448` | vtable `+0x10` of `*(this+0x654)` | `r3 = this+0x654` |

### Ambiguity

None in-REL. `0x80B741F8` is a `main.dol` type token used only as the `r4`
operand. The three `psq_st/psq_l` pairs are `f31` stack spills with `qr0`.
Note the `+0x5FC` store at `0x80A96258` is undone at `0x80A96418`; the visible
effect is only on the nested calls, so it must be reproduced in that order.

---

## 2. `0x80B0F800` - water-wave timered state machine

* **TSV size:** 560 bytes
* **Reachable body:** 140 instructions; 8 `bl` + 2 `bctrl`; ends at
  `0x80B0FA2C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0f800_FUN_80b0f800.c`
* **Proposed signature:** `pub unsafe fn fn_80b0f800(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`, `r29@+0x24`,
`r28@+0x20`. `r30 = this`, `r31 = 0x80B31F48` (constant page; built by
`addis r31,r0,-32589` / `addi r31,r31,0x1F48`).

### Constants

| address | type | value |
|---|---|---|
| `0x80B31F80` | `f32` | `0.0` |
| `0x80B31FB0` | `f32` | `-4.0` |
| `0x80B31FB4` | `f32` | `12.0` |
| `0x80B31FB8` | `f32` | `4.0` |
| `0x80B31FBC` | `f32` | `6016.0` |
| `0x80B687D0` | `char[]` | `"Wm_en_waterwave_in"` |
| `0x80429DD0` | pointer | **(ext)** stage/query object; field `+0x8FEC4` is an `f32` |
| `0x80429D5C` | pointer | **(ext)** sound manager |

The stage field is addressed as `addis rX,rY,9; lfs f,-0x13C(rX)`, i.e.
`+0x90000 - 0x13C = +0x8FEC4`.

### Behaviour

```c
if (*(s32 *)(this + 0x5F0) != 0) {                  // 0x80B0F828
    *(s32 *)(this + 0x5F0) -= 1;
    f1 = *(f32 *)(*(u32 *)0x80429DD0 + 0x8FEC4);     // (ext)
    *(f32 *)(this + 0x5FC) = -4.0f + f1;             // 0x80B31FB0
    vt(this + 0x604, 0xB0);                          // r4 = this+0x5F8, r5=0, r6=0, f1 = f1
}

fn_8006CDE0(this);
fn_800955F0(this);

s32 st = *(s32 *)(this + 0x5EC);                    // 0x80B0F884
s32 ev = 0;
if (st == 0) {                                      // 0x80B0F890 bne
    f0 = 12.0f + *(f32 *)(this + 0xB0);              // 0x80B31FB4
    if (f0 <= *(f32 *)(*(u32 *)0x80429DD0 + 0x8FEC4)) {   // fcmpo + cror + bne
        *(s32 *)(this + 0x5EC) = 1;
        ev = 1;
    }
} else if (st == 1) {                               // 0x80B0F8CC
    if (*(f32 *)(this + 0xEC) < 0.0f &&             // fcmpo vs 0x80B31F80, bge skips
        4.0f + *(f32 *)(this + 0xB0) < *(f32 *)(*(u32 *)0x80429DD0 + 0x8FEC4)) {
        *(s32 *)(this + 0x5EC) = 2;
        ev = 2;
    }
}

if (ev != 0) {                                      // 0x80B0F914
    *(s32 *)(this + 0x5F0) = 80;                    // li r0,0x50
    f3 = *(f32 *)(*(u32 *)0x80429DD0 + 0x8FEC4);
    *(f32 *)(this + 0x5F8) = *(f32 *)(this + 0xAC);
    *(f32 *)(this + 0x5FC) = -4.0f + f3;            // 0x80B31FB0
    *(f32 *)(this + 0x600) = 6016.0f;               // 0x80B31FBC
    vt(this + 0x604, 0xA0);                         // r4 = 0x80B687D0, r5=0,
                                                    // r6 = this+0x5F8, r7=0, r8=0,
                                                    // f1 = -4.0 + f3
    if (ev == 1) {                                  // 0x80B0F97C
        *(s32 *)(this + 0x5F4) = 0;
        fn_8006A4B0(r1 + 0x10, this + 0x5F8);
        fn_80197F00(*(u32 *)0x80429D5C, 0x1DB, r1 + 0x10, 0);
        fn_80078410(*(u32 *)0x80429DD0, 7,
                    *(f32 *)(this + 0x5F8), *(f32 *)(this + 0x5FC));
    } else {
        *(s32 *)(this + 0x5F4) = 1;
        fn_8006A4B0(r1 + 0x08, this + 0x5F8);
        fn_80197F00(*(u32 *)0x80429D5C, 0x1DC, r1 + 0x08, 0);
        fn_80078410(*(u32 *)0x80429DD0, 6,
                    *(f32 *)(this + 0x5F8), *(f32 *)(this + 0x5FC));
    }
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0F870` | vtable `+0xB0` of `*(this+0x604)` | `r3 = this+0x604`, `r4 = this+0x5F8`, `r5 = 0`, `r6 = 0`, `f1 = *(f32 *)(stage+0x8FEC4)` |
| `0x80B0F878` | `0x8006CDE0` | `r3 = this` |
| `0x80B0F880` | `0x800955F0` | `r3 = this` |
| `0x80B0F978` | vtable `+0xA0` of `*(this+0x604)` | `r3 = this+0x604`, `r4 = 0x80B687D0`, `r5 = 0`, `r6 = this+0x5F8`, `r7 = 0`, `r8 = 0`, `f1 = -4.0 + *(f32 *)(stage+0x8FEC4)` |
| `0x80B0F99C` | `0x8006A4B0` | `r3 = r1+0x10`, `r4 = this+0x5F8` |
| `0x80B0F9B0` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 0x1DB`, `r5 = r1+0x10`, `r6 = 0` |
| `0x80B0F9C4` | `0x80078410` | `r3 = *(0x80429DD0)`, `r4 = 7`, `f1 = *(this+0x5F8)`, `f2 = *(this+0x5FC)` |
| `0x80B0F9E4` | `0x8006A4B0` | `r3 = r1+0x08`, `r4 = this+0x5F8` |
| `0x80B0F9F8` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 0x1DC`, `r5 = r1+0x08`, `r6 = 0` |
| `0x80B0FA0C` | `0x80078410` | `r3 = *(0x80429DD0)`, `r4 = 6`, `f1 = *(this+0x5F8)`, `f2 = *(this+0x5FC)` |

### Ambiguity

None in-REL. The two `main.dol` pointers are dereferenced with fixed offsets;
`+0x8FEC4` is a live field of the stage object, not an unresolved base. The
`f1` at the `+0xB0` call is the raw stage field (not the `+0x5FC` sum) — the
`fadds` writes `f0`, and only `f0` is stored. Ready.

---

## 3. `0x80A77C30` - two-arm button gate returning 1 or 2

* **TSV size:** 548 bytes
* **Reachable body:** 137 instructions; 12 `bl` + 4 `bctrl`; ends at
  `0x80A77E50 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a77c30_FUN_80a77c30.c`
* **Proposed signature:** `pub unsafe fn fn_80a77c30(this: usize) -> i32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0xC`, `r30@+0x8`. `r31 = this`.
`r30 = 0x80B2DD0C` (table page, built inside each arm). Return value in `r3`:
`1` on the normal path, `2` when `*(u32 *)(this + 0x87C) == 0`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2DD0C` | `f32[5]` | `{1.0, 1.0, 1.0, 1.0, 1.0}` (index = `*(u32 *)(this+0x88C)`, valid 0..4) |
| `0x80B72E28` | type token | **(ext)** "off" state transition |
| `0x80B72F28` | type token | **(ext)** "on" state transition |
| `0x8042A078` | `u32[2]` | **(ext)** live pad/button state (`+0`, `+4`) |
| `0x80429D20` | `u32` | **(ext)** mask applied to `pad[0]` / result term |
| `0x80429D24` | `u32` | **(ext)** mask applied to `pad[1]` / result term |

### Behaviour

```c
if (*(u32 *)(this + 0x880) == 0) {                  // 0x80A77C48/50 bne
    // arm A, 0x80A77C54..0x80A77C80
    r0 = (*(u32 *)0x8042A078      & *(u32 *)0x80429D20) |
         (*(u32 *)(0x8042A078 + 4) & *(u32 *)0x80429D24);
    if (r0 == 0) {                                  // 0x80A77C80 beq
        vt(this, 0xD4, 0x80B72E28);                 // r3 = this, r4 = token
        return 1;
    }
} else {
    // arm B, 0x80A77D40..0x80A77D7C
    u32 both = *(u32 *)0x8042A078 | *(u32 *)(0x8042A078 + 4);
    r6 = (both == 0) ? 1 : 0;                       // cntlzw + rlwinm r6,r6,27,5,31
    r5 = r6 >> 31;                                  // srawi 31 -> always 0
    r0 = (r6 & *(u32 *)0x80429D24) | (r5 & *(u32 *)0x80429D20);
    if (r0 == 0) {                                  // 0x80A77D7C beq
        vt(this, 0xD4, 0x80B72E28);
        return 1;
    }
}

// shared action, duplicated at 0x80A77C84 and 0x80A77D80
if (*(u32 *)(this + 0x87C) == 0)
    return 2;                                       // li r3,2 / b epilogue

u32 idx = *(u32 *)(this + 0x88C);
if (idx == 4) {                                     // 0x80A77C9C cmpli 4
    fn_80168FE0(this + 0x584, *(f32 *)(0x80B2DD0C + 4 * idx));
    fn_80167B20(this + 0x5BC, idx, *(f32 *)(0x80B2DD0C + 4 * idx));
} else {
    f32 c = *(f32 *)(this + 0x5AC);
    fn_80168FE0(this + 0x584, c);
    fn_80167B80(this + 0x5BC, 0);
    fn_80167B20(this + 0x5BC, 0, c);
}
fn_80A77BB0(this);
vt(this, 0xD4, 0x80B72F28);
return 1;
```

The `rlwinm r6,r6,27,5,31` after `cntlzw` is exactly `(clz(x) == 32) ? 1 : 0`,
i.e. `(x == 0)`. `srawi r5,r6,31` is therefore always `0`; it is emitted as
part of the compiler's branchless zero test and must be kept for byte equality.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A77CB8` | `0x80168FE0` | `r3 = this+0x584`, `f1 = table[idx]` |
| `0x80A77CD0` | `0x80167B20` | `r3 = this+0x5BC`, `r4 = idx`, `f1 = table[idx]` |
| `0x80A77CE0` | `0x80168FE0` | `r3 = this+0x584`, `f1 = *(f32 *)(this+0x5AC)` |
| `0x80A77CEC` | `0x80167B80` | `r3 = this+0x5BC`, `r4 = 0` |
| `0x80A77CF8` | `0x80167B20` | `r3 = this+0x5BC`, `r4 = 0`, `f1 = *(f32 *)(this+0x5AC)` |
| `0x80A77D00` | `0x80A77BB0` | `r3 = this` |
| `0x80A77D1C` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B72F28` |
| `0x80A77D38` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B72E28` |
| `0x80A77DB4` | `0x80168FE0` | `r3 = this+0x584`, `f1 = table[idx]` |
| `0x80A77DCC` | `0x80167B20` | `r3 = this+0x5BC`, `r4 = idx`, `f1 = table[idx]` |
| `0x80A77DDC` | `0x80168FE0` | `r3 = this+0x584`, `f1 = *(f32 *)(this+0x5AC)` |
| `0x80A77DE8` | `0x80167B80` | `r3 = this+0x5BC`, `r4 = 0` |
| `0x80A77DF4` | `0x80167B20` | `r3 = this+0x5BC`, `r4 = 0`, `f1 = *(f32 *)(this+0x5AC)` |
| `0x80A77DFC` | `0x80A77BB0` | `r3 = this` |
| `0x80A77E18` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B72F28` |
| `0x80A77E34` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B72E28` |

### Ambiguity

The four `main.dol` words (`0x8042A078[2]`, `0x80429D20`, `0x80429D24`) are
live input state and masks; the two arms are reproduced literally above, which
is sufficient for a byte-exact port. `0x80B2DD0C` is a five-entry `1.0` table
in this REL; only index 4 is reachable from the `cmpli 4` arm, but the table
extent is real. Ready.

---

## 4. `0x80AC95D0` - Huhu breath/haze actor step

* **TSV size:** 540 bytes
* **Reachable body:** 135 instructions; 10 `bl` + 6 `bctrl`; ends at
  `0x80AC97E8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ac95d0_FUN_80ac95d0.c`
* **Proposed signature:** `pub unsafe fn fn_80ac95d0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`, `r29@+0x24`,
`r28@+0x20`. `r28 = this`, `r31 = 0x80B30500`, `r30 = 0x80B59C70`
(string page). Stack: `+0x14/+0x18/+0x1C` and `+0x08/+0x0C/+0x10` are two
`Vector3` temporaries.

### Constants

| address | type | value |
|---|---|---|
| `0x80B305FC` | `f32` | `74.0` |
| `0x80B30544` | `f32` | `0.0` |
| `0x80B305F8` | `f32` | `5500.0` |
| `0x80B30540` | `s16[2]` | `{0x0E40, 0xF1C0}` = `{3648, -3648}` |
| `0x80B59D28` | `char[]` | `"Wm_en_huhubreath"` |
| `0x80B59D3C` | `char[]` | `"Wm_en_huhubreathstart"` |
| `0x80B59D08` | `char[]` | `"Wm_en_huhuhaze"` |
| `0x80B59D18` | `char[]` | `"Wm_en_huhufloat"` |
| `0x80B75EB0` | type token | **(ext)** |
| `0x80429D5C` | pointer | **(ext)** sound manager |

### Behaviour

```c
vt(this + 0x544, 0x1C);                             // 0x80AC9600..0x80AC9610
fn_8006CDE0(this);
fn_800955F0(this);

if (fn_801690E0(this + 0x584, 74.0f) != 0) {        // 0x80AC962C/30 beq
    s32 mode = *(s32 *)(this + 0x624);
    s32 r = fn_80064610(0x148,                          // r3 (li)
                        (mode != 0) ? 0x30 : 0x10,      // r4
                        this + 0xAC, 0, 0);             // r5,r6,r7
    if (r != 0)
        *(u32 *)(r + 0x9A4C) = *(u32 *)this;            // addis r3,r3,1; stw -0x65B4
}

vt(this + 0x628, 0xA0);                             // r4 = 0x80B59D28,
                                                    // r5 = 0, r6 = this+0x5F8,
                                                    // r7 = this+0x100, r8 = 0
vt(this + 0x750, 0xA0);                             // r4 = 0x80B59D3C, same rest
if (fn_80072BC0(this + 0x1EC) != 0)
    *(f32 *)(this + 0xEC) = 0.0f;                   // 0x80B30544

s16 dir0 = *(s16 *)(this + 0x102);                   // lha -> r29
u8  d    = fn_80096240(this, this + 0xAC);           // returns direction index
*(u8  *)(this + 0x348) = d;                          // stb r3,840
u32 i2   = ((u32)d << 1) & 0x1FE;                    // rlwinm r0,r3,1,23,30
fn_8015F4F0(this + 0x102,
            (s32)*(s16 *)(0x80B30540 + i2),          // lhax
            0x400);                                  // li r5,0x400

// two vector3 temporaries built from 0x604..0x614 and 0x878/0x9A0 vtable calls
v1 = { *(f32 *)(this + 0x604), *(f32 *)(this + 0x608), 5500.0f };  // r1+0x08/0C/10
v0 = { *(f32 *)(this + 0x610), *(f32 *)(this + 0x614), 5500.0f };  // r1+0x14/18/1C
vt(this + 0x878, 0xA0);                             // r4 = 0x80B59D08,
                                                    // r5 = 0, r6 = r1+0x14,
                                                    // r7 = 0, r8 = 0
vt(this + 0x9A0, 0xA0);                             // r4 = 0x80B59D18,
                                                    // r5 = 0, r6 = r1+0x08,
                                                    // r7 = 0, r8 = 0

if (*(s16 *)(this + 0x102) <= 0) {                   // 0x80AC9784 lha + bc 12,1 (bgt)
    fn_80168080(this + 0x584);
    fn_80AC9230(this);
}

if (fn_80169020(this + 0x584) != 0)                  // 0x80AC97A4/AC
    vt(this, 0xD4, 0x80B75EB0);
```

Note `dir0` (`r29`) is loaded but only used at `0x80AC9788` by a `mullw` whose
result is dead; the branch at `0x80AC978C` tests CR0 from the *second*
`lha` (`*(s16 *)(this+0x102)`), so the condition is the sign of `+0x102`, not
of the product. (Ghidra reconstructed the product test; the stream does not
set CR0 from `mullw`.)

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AC9610` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80AC9618` | `0x8006CDE0` | `r3 = this` |
| `0x80AC9620` | `0x800955F0` | `r3 = this` |
| `0x80AC962C` | `0x801690E0` | `r3 = this+0x584`, `f1 = 74.0` |
| `0x80AC965C` | `0x80064610` | `r3 = 0x148` (li), `r4 = 0x30` or `0x10`, `r5 = this+0xAC`, `r6 = 0`, `r7 = 0` |
| `0x80AC9698` | vtable `+0xA0` of `*(this+0x628)` | `r3 = this+0x628`, `r4 = 0x80B59D28`, `r5 = 0`, `r6 = this+0x5F8`, `r7 = this+0x100`, `r8 = 0` |
| `0x80AC96C0` | vtable `+0xA0` of `*(this+0x750)` | `r3 = this+0x750`, `r4 = 0x80B59D3C`, `r5 = 0`, `r6 = this+0x5F8`, `r7 = this+0x100`, `r8 = 0` |
| `0x80AC96C8` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80AC96E8` | `0x80096240` | `r3 = this`, `r4 = this+0xAC` |
| `0x80AC9704` | `0x8015F4F0` | `r3 = this+0x102`, `r4 = *(s16 *)(0x80B30540+2*idx)`, `r5 = 0x400` |
| `0x80AC9758` | vtable `+0xA0` of `*(this+0x878)` | `r3 = this+0x878`, `r4 = 0x80B59D08`, `r5 = 0`, `r6 = r1+0x14`, `r7 = 0`, `r8 = 0` |
| `0x80AC9780` | vtable `+0xA0` of `*(this+0x9A0)` | `r3 = this+0x9A0`, `r4 = 0x80B59D18`, `r5 = 0`, `r6 = r1+0x08`, `r7 = 0`, `r8 = 0` |
| `0x80AC9794` | `0x80168080` | `r3 = this+0x584` |
| `0x80AC979C` | `0x80AC9230` | `r3 = this` |
| `0x80AC97A4` | `0x80169020` | `r3 = this+0x584` |
| `0x80AC97C8` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B75EB0` |

### Ambiguity

None in-REL; all six strings are in this REL. `0x80B75EB0` and `0x80429D5C`
are `main.dol` operands. Both `0x8015F4F0` calls take a literal limit
(`0x400`, then `0x180`); only the `lhax` table index is data-dependent. Ready.

---

## 5. `0x80B1DFE0` - wall-bounce actor step

* **TSV size:** 540 bytes
* **Reachable body:** 135 instructions; 8 `bl` + 3 `bctrl`; ends at
  `0x80B1E1F8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b1dfe0_FUN_80b1dfe0.c`
* **Proposed signature:** `pub unsafe fn fn_80b1dfe0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `stfd f31,+0x30` + `psq_st f31,qr0,+0x38`;
`r31@+0x2C`, `r30@+0x28`, `r29@+0x24`. `r29 = this`, `r30 = 0x80B32558`.
Stack: `+0x08/+0x0C/+0x10` holds `{0x628, 0x62C, 0x630}`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B32558` | `f32` | `0.0` |
| `0x80B3268C` | `f32` | `1.75` |
| `0x80B326A8` | `f32` | `0.4000000059604645` |
| `0x80B326AC` | `f32` | `-0.4000000059604645` |
| `0x80B32588` | `s16[2]` | `{0x1800, 0xE800}` = `{6144, -6144}` |
| `0x80B792F8` | type token | **(ext)** |

### Behaviour

```c
vt(this + 0x544, 0x1C);
vt(this + 0x5C0, 0x14);
fn_8006CD90(this);
fn_8006CDE0(this);
fn_8006CD40(this);

u32 idx = *(u32 *)(this + 0x5F4);
fn_8015F4F0(this + 0x104, (s32)*(s16 *)(0x80B32588 + 2*idx), 0x300);
fn_8015F4F0(this + 0x100, 0, 0x180);

if (*(f32 *)(this + 0xEC) < 0.0f &&                // 0x80B1E080 fcmpo + bc 4,0 (bge)
    fn_80B1D440(this) != 0) {                      // 0x80B1E094 beq
    s32 old = *(s32 *)(this + 0x5EC);
    s32 ev  = 1;                                   // li r31,1
    u32 dir = *(u32 *)(this + 0x5F4) ^ 1;
    *(u32 *)(this + 0x5F4) = dir;
    *(f32 *)(this + 0xEC)  = 1.75f;                // 0x80B3268C
    *(s32 *)(this + 0x610) = ev;
    if (old == 0) {                                // 0x80B1E0BC bne
        f32 dy = *(f32 *)(this + 0x628) - *(f32 *)(this + 0x61C);   // f31
        v[0] = *(f32 *)(this + 0x628);             // r1+0x08
        v[1] = *(f32 *)(this + 0x62C);             // r1+0x0C
        v[2] = *(f32 *)(this + 0x630);             // r1+0x10
        if (*(u8 *)(this + 0x348) == 0) {          // 0x80B1E0E8 bne
            f = fn_800E9020(this + 0x668);
            if (dy <= 0.4f * f) {                  // 0x80B1E100 cror eq=lt|eq, bne
                *(s32 *)(this + 0x5EC) = 1;
                *(u8  *)(this + 0x348) ^= 1;
                *(f32 *)(this + 0xE8)  = -*(f32 *)(this + 0xE8);
                *(f32 *)(this + 0xF4)  = -*(f32 *)(this + 0xF4);
            }
        } else {                                   // 0x80B1E134
            f = fn_800E9020(this + 0x668);
            if (dy <= -0.4f * f) {                 // 0x80B1E144 cror eq=gt|eq?, bne
                *(s32 *)(this + 0x5EC) = 1;
                *(u8  *)(this + 0x348) ^= 1;
                *(f32 *)(this + 0xE8)  = -*(f32 *)(this + 0xE8);
                *(f32 *)(this + 0xF4)  = -*(f32 *)(this + 0xF4);
            }
        }
    }
}

if (*(s32 *)(this + 0x60C) > 0) {                  // 0x80B1E178..0x80B1E180 bc 4,1 (ble)
    *(s32 *)(this + 0x60C) -= 1;
} else {
    f2 = *(f32 *)(this + 0xB8) - *(f32 *)(this + 0x61C);
    f1 = *(f32 *)(this + 0xAC) - *(f32 *)(this + 0x61C);
    if (f2 * f1 <= 0.0f) {                         // 0x80B1E1AC cror eq=lt|eq, bne
        *(f32 *)(this + 0xAC) = *(f32 *)(this + 0x61C);
        vt(this, 0xD4, 0x80B792F8);
    }
}
```

The two bounce arms differ only in the comparison constant: `0x80B326A8`
(`+0.4`) when `+0x348 == 0`, `0x80B326AC` (`-0.4`) otherwise. Both branches
are `cror cr0.eq,cr0.lt,cr0.eq` followed by `bne`, i.e. "take when
`dy <= c * f`".

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B1E01C` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80B1E030` | vtable `+0x14` of `*(this+0x5C0)` | `r3 = this+0x5C0` |
| `0x80B1E038` | `0x8006CD90` | `r3 = this` |
| `0x80B1E040` | `0x8006CDE0` | `r3 = this` |
| `0x80B1E048` | `0x8006CD40` | `r3 = this` |
| `0x80B1E064` | `0x8015F4F0` | `r3 = this+0x104`, `r4 = *(s16 *)(0x80B32588+2*idx)`, `r5 = 0x300` |
| `0x80B1E074` | `0x8015F4F0` | `r3 = this+0x100`, `r4 = 0`, `r5 = 0x180` |
| `0x80B1E08C` | `0x80B1D440` | `r3 = this` |
| `0x80B1E0F0` | `0x800E9020` | `r3 = this+0x668` |
| `0x80B1E138` | `0x800E9020` | `r3 = this+0x668` |
| `0x80B1E1D4` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B792F8` |

### Ambiguity

None in-REL. All five floats and the `s16` table resolve in this REL. The
three `psq_st`/`psq_l` pairs are `f31` spills with `qr0`, and the `2^52`
`ldexpf` noise in the export is the `fctiwz`/`stfd` integer-to-float idiom at
`0x80B1E0D8`. Ready.

---

## 6. `0x80ADD350` - `block_pata` spawn entry

* **TSV size:** 536 bytes
* **Reachable body:** 134 instructions; 13 `bl`, no `bctrl`; ends at
  `0x80ADD564 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80add350_FUN_80add350.c`
* **Proposed signature:** `pub unsafe fn fn_80add350(this: usize) -> i32`
* **Confidence:** **medium / conditional** (two external floats are copied
  into actor fields)

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C`, `r30@+0x18`. `r30 = this`,
`r31 = 0x80B30AE0`. Returns `1` on the spawn path, `0` when the ground query
reports the cell is already occupied.

### Constants

| address | type | value |
|---|---|---|
| `0x80B30B30` | `f32` | `500.0` |
| `0x80B30B34` | `f32` | `1.0` |
| `0x80B30B38` | `f32` | `0.0` |
| `0x80B30B3C` | `f32` | `16.0` |
| `0x80B30B40` | `f32` | `-8.0` |
| `0x80B30B44` | `f32` | `8.0` |
| `0x80B30B48` | `f64` | `0x4330000080000000` = `2^52 + 2^31` |
| `0x80B5F1D8` | `char[]` | `"fly_sea"` |
| `0x80B5F1E0` | `char[]` | `"fly_normal"` |
| `0x8042BB08` | `f32` | **(ext)** written to `+0x328` |
| `0x8042BB0C` | `f32` | **(ext)** written to `+0x32C` |
| `0x80429DD0` | pointer | **(ext)** stage/query object |
| `0x80429D28` | `u8` | **(ext)** scene flag, `r6` of the ground query |

### Behaviour

```c
i32 hit = fn_80077810(*(u32 *)0x80429DD0,
                      (int)*(f32 *)(this + 0xAC) & 0xFFFF,   // r4
                      (int)-*(f32 *)(this + 0xB0) & 0xFFFF,  // r5
                      *(u8  *)0x80429D28);                   // r6
if (hit != 0) {                                     // 0x80ADD3B4 beq
    fn_80162510(this);
    return 0;
}

*(f32 *)(this + 0xB4) = 500.0f;                     // 0x80B30B30
fn_80ADD570(this);

f32 m8  = -8.0f, p8 = 8.0f;                          // 0x80B30B40 / 0x80B30B44
f32 one = 1.0f;                                      // 0x80B30B34
f32 zero = 0.0f;                                     // 0x80B30B38
f32 x = *(f32 *)(this + 0xAC);
f32 y = *(f32 *)(this + 0xB0);
f32 z = *(f32 *)(this + 0xB4);
u32 task = *(u32 *)(this + 4);

*(f32 *)(this + 0xDC) = one;  *(f32 *)(this + 0xE0) = one;  *(f32 *)(this + 0xE4) = one;
*(f32 *)(this + 0x320) = zero; *(f32 *)(this + 0x324) = 16.0f;
*(f32 *)(this + 0x318) = zero; *(f32 *)(this + 0x31C) = zero;
*(f32 *)(this + 0x328) = *(f32 *)0x8042BB08;         // (ext)
*(f32 *)(this + 0x32C) = *(f32 *)0x8042BB0C;         // (ext)
*(u8  *)(this + 0x348) = (u8)((task >> 8) & 1);      // rlwinm r11,r0,24,31,31
*(u32 *)(this + 0x878) = task & 0xF;                 // rlwinm r0,r0,0,28,31
*(f32 *)(this + 0x888) = x; *(f32 *)(this + 0x88C) = y; *(f32 *)(this + 0x890) = z;
*(f32 *)(this + 0x87C) = x; *(f32 *)(this + 0x880) = y; *(f32 *)(this + 0x884) = z;

fn_8007FA40(m8, p8, p8, m8,                           // f1..f4
            this + 0x790, this,                       // r3, r4
            0x80ADDB40, 0x80ADDBD0, 0x80ADDD10,       // r5, r6, r7
            1, 0, 0);                                 // r8, r9, r10
*(u32 *)(this + 0x860) = 0x260;                       // li r0,608
fn_8007F900(this + 0x790);

i32 sea = fn_80075270(*(f32 *)(this + 0xAC), *(f32 *)(this + 0xB0),
                      *(u8  *)(this + 0x38F), 0);      // r3, r4
if (sea == 0)
    fn_80ADE240(this, 0x80B5F1E0, 0.0f);               // "fly_normal"
else
    fn_80ADE240(this, 0x80B5F1D8, 0.0f);               // "fly_sea"

fn_80166F40(this + 0x734, 1, 0);
fn_80169F80(this + 0x6B8, this + 0x734, 0.0f);

u32 cell = (task >> 12) & 3;                          // rlwinm r0,r4,20,30,31
fn_80166EF0(this + 0x734,
            (f32)(double)(0x43300000, cell) - (f32)0x80B30B48,
            cell);                                     // r4 = cell
fn_80166F10(this + 0x734, 0, 0.0f);
fn_80ADDA10(this);
return 1;
```

Note: `0x80ADD4E8..0x80ADD530` uses the classic unsigned-int-to-single idiom
(`stw 0x43300000` + `stw cell`, `lfd f0`, `lfs f1,0x80B30B48`, `fsubs`), so
`f1 = (f32)cell` exactly.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80ADD3AC` | `0x80077810` | `r3 = *(0x80429DD0)`, `r4 = (int)x & 0xFFFF`, `r5 = (-(int)y) & 0xFFFF`, `r6 = *(u8 *)0x80429D28` |
| `0x80ADD3BC` | `0x80162510` | `r3 = this` |
| `0x80ADD3D4` | `0x80ADD570` | `r3 = this` |
| `0x80ADD48C` | `0x8007FA40` | `f1 = -8`, `f2 = 8`, `f3 = 8`, `f4 = -8`, `r3 = this+0x790`, `r4 = this`, `r5 = 0x80ADDB40`, `r6 = 0x80ADDBD0`, `r7 = 0x80ADDD10`, `r8 = 1`, `r9 = 0`, `r10 = 0` |
| `0x80ADD49C` | `0x8007F900` | `r3 = this+0x790` |
| `0x80ADD4B0` | `0x80075270` | `f1 = x`, `f2 = y`, `r3 = *(u8 *)(this+0x38F)`, `r4 = 0` |
| `0x80ADD4CC` | `0x80ADE240` | `r3 = this`, `r4 = 0x80B5F1D8`, `f1 = 0.0` (sea arm) |
| `0x80ADD4E4` | `0x80ADE240` | `r3 = this`, `r4 = 0x80B5F1E0`, `f1 = 0.0` (land arm) |
| `0x80ADD4F4` | `0x80166F40` | `r3 = this+0x734`, `r4 = 1`, `r5 = 0` |
| `0x80ADD504` | `0x80169F80` | `r3 = this+0x6B8`, `r4 = this+0x734`, `f1 = 0.0` |
| `0x80ADD530` | `0x80166EF0` | `r3 = this+0x734`, `r4 = (task>>12)&3`, `f1 = (f32)((task>>12)&3)` |
| `0x80ADD540` | `0x80166F10` | `r3 = this+0x734`, `r4 = 0`, `f1 = 0.0` |
| `0x80ADD548` | `0x80ADDA10` | `r3 = this` |

### Ambiguity

The two `main.dol` floats `0x8042BB08` / `0x8042BB0C` are copied verbatim into
`+0x328` / `+0x32C`; their numeric values are not in this REL, so the row is
**conditional** on reading them (or on confirming they are the
`0x80ADD350`-family spawn tilt pair). Everything else, including the
`0x80ADDB40 / 0x80ADDBD0 / 0x80ADDD10` callback trio, is in-REL. Both offsets
are literal: the `addis` is `0x80AE`, so `-0x24C0` gives `0x80ADDB40` and
`-0x2430` gives `0x80ADDBD0`.

---

## 7. `0x80A45DB0` - "spawn under water surface" entry

* **TSV size:** 500 bytes
* **Reachable body:** 125 instructions; 8 `bl` + 1 `bctrl`; ends at
  `0x80A45FA0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a45db0_FUN_80a45db0.c`
* **Proposed signature:** `pub unsafe fn fn_80a45db0(this: usize) -> i32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`, `r29@+0x24`.
`r29 = this`, `r30 = 0x80B2C7A0`. Returns `1` on the spawn path, `2` when the
ground query reports the cell is occupied.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2C7A8` | `f32` | `-3000.0` |
| `0x80B2C7AC` | `f32` | `500.0` |
| `0x80B2C7B0` | `f32` | `8.0` |
| `0x80B2C7B4` | `f32` | `1.0` |
| `0x80B2C7B8` | `f32` | `0.0` |
| `0x80B70D18` | type token | **(ext)** |
| `0x80B70CE8` | type token | **(ext)** |
| `0x80429DD0` | pointer | **(ext)** stage/query object |
| `0x80429D28` | `u8` | **(ext)** scene flag |
| `0x80429D14` | `u8` | **(ext)** flag byte, bit 0 set on failure |

### Behaviour

```c
if (*(u8 *)0x80429D28 == 0)                         // 0x80A45DDC/DE0 beq
    *(f32 *)(this + 0xB4) = 500.0f;                 // 0x80B2C7AC
else
    *(f32 *)(this + 0xB4) = -3000.0f;               // 0x80B2C7A8

f32 x = *(f32 *)(this + 0xAC);
f32 y = *(f32 *)(this + 0xB0);
f32 z = *(f32 *)(this + 0xB4);
*(f32 *)(this + 0x6E8) = x;
*(f32 *)(this + 0x6EC) = y;
*(f32 *)(this + 0x6F0) = z;

i32 hit = fn_80077810(*(u32 *)0x80429DD0,
                      (int)x & 0xFFFF, (int)-y & 0xFFFF,
                      *(u8 *)0x80429D28);            // r6 still holds the byte
if (hit != 0) {                                     // 0x80A45E44 beq
    u8 *p = *(u8 **)0x80429D14;
    *p |= 1;                                        // ori r0,r0,0x1 / stb
    return 2;                                       // li r3,2
}

f32 nx = x + 8.0f;                                  // fadds f3,f0,f4
f32 ny = y - 8.0f;                                  // fsubs f2,f2,f4
*(f32 *)(this + 0xAC) = nx;
*(f32 *)(this + 0xB0) = ny;
*(f32 *)(this + 0xDC) = 1.0f;
*(f32 *)(this + 0xE0) = 1.0f;
*(f32 *)(this + 0xE4) = 1.0f;
*(f32 *)(this + 0xD0) = 0.0f;
*(f32 *)(this + 0xD4) = 0.0f;
fn_8007FB10(this + 0x534, this, 0x80B70D18, 1, 0, 0);   // f1=1, f2=ny, f3=nx, f4=8
*(u32 *)(this + 0x600) = 5;                         // li r0,5

u32 cur = *(u32 *)(this + 0xAC4);
*(u8  *)(this + 0x612) = *(u8 *)0x80429D28;
if (*(u32 *)0x80429DD0 != cur) {                    // 0x80A45ED8 beq
    if (cur != 0) {                                 // 0x80A45EE4 beq
        u32 arg = (cur == 0xFFFFFF9C) ? 0 : cur;    // addic. / addi -100
        fn_80161800(cur + 100, this + 0xAC4, arg);
    }
    if (*(u32 *)0x80429DD0 != 0) {
        u32 g = *(u32 *)0x80429DD0;
        u32 arg = (g == 0xFFFFFF9C) ? 0 : g;
        fn_80161750(g + 100, this + 0xAC4, arg);
    }
}

u32 h = fn_80078520(*(u32 *)(this + 0xAC4), 0);
fn_80014820(h, this + 0x698);
*(u16 *)(this + 0x6A0) = 0x1E;                      // li r0,30 / sth
fn_800146B0(*(f32 *)(this + 0xAC) - 8.0f,            // f1
            -(8.0f + *(f32 *)(this + 0xB0)),        // f2
            *(f32 *)(this + 0xB4),                  // f3
            this + 0x698);                          // r3
fn_80A46090(this);
vt(this, 0xD4, 0x80B70CE8);
return 1;
```

The `addic.` / `bc 12,2` / `addi r5,r5,-100` triple at `0x80A45EE8` and
`0x80A45F08` is the null-sentinel idiom: the argument is `0` when the pointer
is `-100` (`0xFFFFFF9C`), otherwise the pointer itself.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A45E3C` | `0x80077810` | `r3 = *(0x80429DD0)`, `r4 = (int)x & 0xFFFF`, `r5 = (-(int)y) & 0xFFFF`, `r6 = *(u8 *)0x80429D28` |
| `0x80A45EB8` | `0x8007FB10` | `f1 = 1.0`, `f2 = y-8`, `f3 = x+8`, `f4 = 8.0`, `r3 = this+0x534`, `r4 = this`, `r5 = 0x80B70D18`, `r6 = 1`, `r7 = 0`, `r8 = 0` |
| `0x80A45EFC` | `0x80161800` | `r3 = cur+100`, `r4 = this+0xAC4`, `r5 = (cur==-100?0:cur)` |
| `0x80A45F1C` | `0x80161750` | `r3 = *(0x80429DD0)+100`, `r4 = this+0xAC4`, `r5 = (g==-100?0:g)` |
| `0x80A45F28` | `0x80078520` | `r3 = *(u32 *)(this+0xAC4)`, `r4 = 0` |
| `0x80A45F30` | `0x80014820` | `r3 = ret`, `r4 = this+0x698` |
| `0x80A45F5C` | `0x800146B0` | `f1 = x-8`, `f2 = -(y+8)`, `f3 = z`, `r3 = this+0x698` |
| `0x80A45F64` | `0x80A46090` | `r3 = this` |
| `0x80A45F80` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B70CE8` |

### Ambiguity

None in-REL. The two tokens and the three `main.dol` globals are operands; the
`+0x9A4C`-style `0x80161800`/`0x80161750` helpers take the null-adjusted
argument shown above. Ready.

---

## 8. `0x80AD5760` - 9-cell grid spawn

* **TSV size:** 500 bytes
* **Reachable body:** 125 instructions; 5 `bl` + 3 `bctrl`; ends at
  `0x80AD5950 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad5760_FUN_80ad5760.c`
* **Proposed signature:** `pub unsafe fn fn_80ad5760(this: usize) -> i32`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x50(r1)`; LR at `+0x54`; `stfd f31,+0x40` + `psq_st f31,qr0,+0x48`;
`stfd f30,+0x30` + `psq_st f30,qr0,+0x38`; `r31@+0x2C`, `r30@+0x28`,
`r29@+0x24`. `r30 = this`, `r31 = 0x80B30958`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B30958` | `f32` | `0.0` |
| `0x80B30960` | `f32` | `-256.0` |
| `0x80B30964` | `f32` | `16.0` |
| `0x80B30968` | `f32` | `128.0` |
| `0x80B3096C` | `f32` | `-128.0` |
| `0x80B30970` | `f32` | `736.0` |
| `0x80B30978` | `f64` | `0x4330000080000000` = `2^52 + 2^31` |
| `0x80B5C22C` | `u32` | `0` |
| `0x80B5C230` | `u32` | `0xFFFFFFFF` |
| `0x80B5C234` | `u32` | `0x80AD5AD0` (in-REL, data word; not a TSV entry) |

`0x80B5C22C..0x80B5C237` is a 12-byte block copied into `+0x6D0..+0x6D8`
(`0x6D0 = 0`, `0x6D4 = -1`, `0x6D8 = 0x80AD5AD0`).

### Behaviour

```c
u32 task = *(u32 *)(this + 4);
*(f32 *)(this + 0xB4) = -256.0f;                    // 0x80B30960

fn_800D77D0(this + 0x564,                            // r3
            (task >> 12) & 0xF,                      // r4  rlwinm 20,28,31
            (task >> 24) & 3,                        // r5  rlwinm 8,30,31
            (task >> 16) & 0xF,                      // r6  rlwinm 16,28,31
            (task >> 20) & 0xF,                      // r7  rlwinm 12,28,31
            (task >>  8) & 3);                       // r8  rlwinm 24,30,31

u32 a = ((task & 0xF) + 1) & 0xFF;                   // 0x80AD57C0..0x80AD57C8
*(u8 *)(this + 0x4C2C) = (u8)a;
if ((s16)(u8)a < 2)                                   // extsh + cmpi 2 + bc 4,0 (bge)
    *(u8 *)(this + 0x4C2C) = 2;                       // li r0,2

u32 b = (((task >> 4) & 0xF) + 1) & 0xFF;             // 0x80AD57E4..0x80AD57F0
*(u8 *)(this + 0x4C2D) = (u8)b;
if ((s16)(u8)b < 2)
    *(u8 *)(this + 0x4C2D) = 2;                       // li r0,2

f32 gx = 16.0f * (f32)(s16)*(u8 *)(this + 0x4C2C);    // f31
f32 gz = 16.0f * (f32)(s16)*(u8 *)(this + 0x4C2D);    // f30
fn_80AD5480(this, gx, gz);
fn_80AD54A0(this);
vt(this, 0x280, gx, gz);
fn_80AD5570(this, gx, gz);

*(f32 *)(this + 0x320) = 128.0f;
*(f32 *)(this + 0x324) = -128.0f;
*(f32 *)(this + 0x318) = 736.0f;
*(f32 *)(this + 0x31C) = 736.0f;

// 0x80AD58C0: lwzu r4,-15828(r5) with r5 = 0x80B60000
*(u32 *)(this + 0x6D0) = *(u32 *)0x80B5C22C;
*(u32 *)(this + 0x6D4) = *(u32 *)0x80B5C230;
*(u32 *)(this + 0x6D8) = *(u32 *)0x80B5C234;

u32 r29 = vt(this, 0x290);                            // r3 result
u32 r6  = vt(this, 0x28C) & 0xFFFF;                   // rlwinm r6,r3,0,16,31
fn_800BFC60(this + 0x6DC, 0.0f,                        // r3, f1
            (s16)*(u8 *)(this + 0x4C2C),              // r4
            (s16)*(u8 *)(this + 0x4C2D),              // r5
            r6, r29);                                 // r6, r7
return 1;
```

Both nibble-to-float conversions at `0x80AD580C..0x80AD5860` use the
`0x43300000 | value^0x8000` + `lfd` + `fsubs` idiom against `0x80B30978`, then
multiply by `0x80B30964` (`16.0`).

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AD57B8` | `0x800D77D0` | `r3 = this+0x564`, `r4 = (task>>12)&0xF`, `r5 = (task>>24)&3`, `r6 = (task>>16)&0xF`, `r7 = (task>>20)&0xF`, `r8 = (task>>8)&3` |
| `0x80AD5864` | `0x80AD5480` | `r3 = this`, `f1 = 16.0*countX`, `f2 = 16.0*countZ` |
| `0x80AD586C` | `0x80AD54A0` | `r3 = this` |
| `0x80AD5888` | vtable `+0x280` of `*(this+0x60)` | `r3 = this`, `f1 = 16.0*countX`, `f2 = 16.0*countZ` |
| `0x80AD5898` | `0x80AD5570` | `r3 = this`, `f1 = 16.0*countX`, `f2 = 16.0*countZ` |
| `0x80AD58E4` | vtable `+0x290` of `*(this+0x60)` | `r3 = this` |
| `0x80AD58FC` | vtable `+0x28C` of `*(this+0x60)` | `r3 = this` |
| `0x80AD5920` | `0x800BFC60` | `r3 = this+0x6DC`, `f1 = 0.0`, `r4 = (s16)*(u8 *)(this+0x4C2C)`, `r5 = (s16)*(u8 *)(this+0x4C2D)`, `r6 = (u16)vt[0x28C]`, `r7 = vt[0x290]` |

### Ambiguity

None in-REL. `0x80AD5AD0` is an in-REL data word (it is not in `functions.tsv`,
so it is not a function entry). The two `psq_st` pairs are `f31`/`f30` spills
with `qr0`. Ready.

---

## 9. `0x80ACC520` - Huhu per-frame step with direction sound

* **TSV size:** 492 bytes
* **Reachable body:** 123 instructions; 13 `bl` + 4 `bctrl`; ends at
  `0x80ACC708 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80acc520_FUN_80acc520.c`
* **Proposed signature:** `pub unsafe fn fn_80acc520(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `stfd f31,+0x30` + `psq_st f31,qr0,+0x38`;
`r31@+0x2C`, `r30@+0x28`, `r29@+0x24`, `r28@+0x20`. `r28 = this`,
`r31 = 0x80B75FE0`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B30728` | `f32` | `264.0` |
| `0x80B306B0` | `f32` | `0.0` |
| `0x80B760B8` | type token | **(ext)** |
| `0x80B76138` | type token | **(ext)** |
| `0x80B76178` | type token | **(ext)** |
| `0x80429D5C` | pointer | **(ext)** sound manager |

### Behaviour

```c
u32 id = *(u32 *)this;
fn_8006A4B0(r1 + 0x10, this + 0xAC);
fn_80198870(*(u32 *)0x80429D5C, 0x1E7, id, r1 + 0x10, 1, 0);

vt(this + 0x548, 0x1C);
fn_8006CDE0(this);
fn_800955F0(this);

f31 = *(f32 *)(this + 0xAC);
*(u16 *)(this + 0x102) = *(s16 *)(this + 0x5D0 + 2 * *(u8 *)(this + 0x348));
                                                        // lha r0,1488(r3)
s32 clamped = 0;
if (f31 <= 264.0f) {                                    // 0x80ACC5C4 cror + bne
    *(f32 *)(this + 0xAC) = 264.0f;                     // 0x80B30728
    clamped = 1;                                        // li r29,1
}

fn_80096F60(this);
s32 r30 = fn_80070CB0(this + 0x1EC);
fn_80ACAE70(this, r30);
if (r30 != 0) {                                         // 0x80ACC608 beq
    u16 c = *(u16 *)(this + 0x5D4);
    *(f32 *)(this + 0xEC) = 0.0f;                       // 0x80B306B0
    if (c != 0xFFFF)                                    // 0x80ACC618 cmpli + bc 4,0 (bge)
        *(u16 *)(this + 0x5D4) = c + 1;
}
fn_8006EF40(this + 0x1EC, *(u32 *)(this + 0x274));
fn_80097050(this);
if (clamped != 0)                                       // 0x80ACC644 beq
    *(f32 *)(this + 0xAC) = f31;                        // restore original x

fn_80ACB200(this);
fn_80ACB330(this);

if (fn_80ACB280(this, r1 + 0x08) != 0) {                // 0x80ACC66C beq
    if (*(u8 *)(this + 0x348) == *(u32 *)(r1 + 0x08))   // 0x80ACC678 cmpl + bne
        vt(this, 0xD4, 0x80B76178);
    else
        vt(this, 0xD4, 0x80B76138);
} else {
    if (fn_80ACB430(this, *(u8 *)(this + 0x348)) != 0)  // 0x80ACC6C8 beq
        vt(this, 0xD4, 0x80B760B8);
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80ACC564` | `0x8006A4B0` | `r3 = r1+0x10`, `r4 = this+0xAC` |
| `0x80ACC580` | `0x80198870` | `r3 = *(0x80429D5C)`, `r4 = 0x1E7`, `r5 = *(u32 *)this`, `r6 = r1+0x10`, `r7 = 1`, `r8 = 0` |
| `0x80ACC594` | vtable `+0x1C` of `*(this+0x548)` | `r3 = this+0x548` |
| `0x80ACC59C` | `0x8006CDE0` | `r3 = this` |
| `0x80ACC5A4` | `0x800955F0` | `r3 = this` |
| `0x80ACC5E8` | `0x80096F60` | `r3 = this` |
| `0x80ACC5F0` | `0x80070CB0` | `r3 = this+0x1EC` |
| `0x80ACC600` | `0x80ACAE70` | `r3 = this`, `r4 = ret` |
| `0x80ACC634` | `0x8006EF40` | `r3 = this+0x1EC`, `r4 = *(u32 *)(this+0x274)` |
| `0x80ACC63C` | `0x80097050` | `r3 = this` |
| `0x80ACC650` | `0x80ACB200` | `r3 = this` |
| `0x80ACC658` | `0x80ACB330` | `r3 = this` |
| `0x80ACC664` | `0x80ACB280` | `r3 = this`, `r4 = r1+0x08` |
| `0x80ACC694` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B76178` |
| `0x80ACC6B0` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B76138` |
| `0x80ACC6C0` | `0x80ACB430` | `r3 = this`, `r4 = *(u8 *)(this+0x348)` |
| `0x80ACC6E0` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B760B8` |

### Ambiguity

None in-REL. The three tokens are `main.dol` operands. Ready.

---

## 10. `0x80A35170` - ice slide state aid (angle ramp)

* **TSV size:** 464 bytes
* **Reachable body:** 116 instructions; 7 `bl` + 2 `bctrl`; ends at
  `0x80A3533C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a35170_FUN_80a35170.c`
* **Proposed signature:** `pub unsafe fn fn_80a35170(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; LR at `+0x34`; `r31@+0x2C`, `r30@+0x28`. `r31 = this`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2C094` | `f32` | `0.0` |
| `0x80358E50` | token | **(ext)** `r4` of the `vt[0x10]` query |
| `0x80429D5C` | pointer | **(ext)** sound manager |

### Behaviour

```c
fn_800C1230(this + 0x5BC);
fn_800C12B0(r1 + 0x18, this + 0x5BC);               // 2 floats out
f32 px = *(f32 *)(r1 + 0x18);
f32 py = *(f32 *)(r1 + 0x1C);
*(f32 *)(r1 + 0x20) = px;
*(f32 *)(r1 + 0x24) = py;
*(f32 *)(this + 0xAC) = px;
*(f32 *)(this + 0xB0) = py;

if (*(s32 *)(this + 0x488) > 0)                      // 0x80A351BC bc 12,1 (bgt)
    return;

*(u32 *)(this + 0x6B4) = *(u32 *)(this + 0x6AC);     // 0x80A351C0/C8
u32 now = vt(this + 0x628, 0x28)->vt[0x10](0x80358E50);   // 0x80A351D8/0x80A351F0
u32 prev = *(u32 *)(this + 0x6B4);
*(u32 *)(this + 0x6AC) = now;
if (prev == 0 && now == 1)                           // 0x80A351FC bne, 0x80A35208 bne
    *(u32 *)(this + 0x6B8) += 1;

if (*(s32 *)(this + 0x6B8) > 0 &&                     // 0x80A35220 bc 4,1 (ble)
    *(u8  *)(this + 0x6A8) == 0) {                    // 0x80A3522C bne
    *(u8 *)(this + 0x6A8) = 1;                        // li r3,1
    if ((*(u32 *)(this + 4) >> 8) & 1)                // 0x80A3523C rlwinm. 24,31,31
        fn_80197F00(*(u32 *)0x80429D5C, 0x21E, r1 + 0x08, 0);
    else
        fn_80197F00(*(u32 *)0x80429D5C, 0x21D, r1 + 0x10, 0);

    f32 v = *(f32 *)(this + 0x5F4);
    if (v > 0.0f)                                      // 0x80A352A8 bc 4,1 (ble)
        *(u32 *)(this + 0x6B4) = 0;                    // li r0,0
    else if (v < 0.0f)                                 // 0x80A352B8 bc 4,0 (bge)
        *(u32 *)(this + 0x6B4) = 1;                    // li r0,1
    else
        *(u32 *)(this + 0x6B4) = fn_800B2ED0(2);
}

if (*(u8 *)(this + 0x6A8) == 0)                        // 0x80A352DC beq epilogue
    return;

if (*(u32 *)(this + 0x6B4) == 0)                       // 0x80A352E8 bne
    *(s16 *)(this + 0x104) -= 0x800;
else
    *(s16 *)(this + 0x104) += 0x800;

if (*(s16 *)(this + 0x104) != 0)                       // 0x80A3530C bne epilogue
    return;
*(u8  *)(this + 0x6A8) = 0;                            // li r0,0
*(s32 *)(this + 0x6B8) -= 1;
```

`0x80A3523C` is `rlwinm. r0,r0,24,31,31`, i.e. `(task >> 8) & 1` with CR0 set;
`bc 4,2` then selects `0x21E` when the bit is set and `0x21D` when it is clear.
(Ghidra names `0x21D` first; the stream's fall-through arm is `0x21D`, and the
bit-set arm at `0x80A35270` is `0x21E`.)

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A3518C` | `0x800C1230` | `r3 = this+0x5BC` |
| `0x80A35198` | `0x800C12B0` | `r3 = r1+0x18`, `r4 = this+0x5BC` |
| `0x80A351D8` | vtable `+0x28` of `*(this+0x628)` | `r3 = this+0x628` |
| `0x80A351F0` | vtable `+0x10` of the returned object | `r3 = obj`, `r4 = 0x80358E50` |
| `0x80A35254` | `0x8006A4B0` | `r3 = r1+0x10`, `r4 = this+0xAC` |
| `0x80A35268` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 0x21D`, `r5 = r1+0x10`, `r6 = 0` |
| `0x80A35280` | `0x8006A4B0` | `r3 = r1+0x08`, `r4 = this+0xAC` |
| `0x80A35294` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 0x21E`, `r5 = r1+0x08`, `r6 = 0` |
| `0x80A352CC` | `0x800B2ED0` | `r3 = 2` |

### Ambiguity

None in-REL. `0x80358E50` is a `main.dol` query token; the `vt[0x10]` return is
compared against `0`/`1` literally. The three `addi rX,r0,imm` forms at
`0x80A352B0/E0`, `0x80A352BC` and `0x80A35318` are `li` (`rA = 0`), so the
stored values are exactly `0`, `1` and `0`. Ready.

---

## 11. `0x80A0BD20` - Choropu-like appear handler

* **TSV size:** 460 bytes
* **Reachable body:** 115 instructions; 7 `bl` + 6 `bctrl`; ends at
  `0x80A0BEE8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a0bd20_FUN_80a0bd20.c`
* **Proposed signature:** `pub unsafe fn fn_80a0bd20(this: usize)`
* **Confidence:** **medium / conditional** (an external 256-byte direction
  table is indexed and its byte is passed to the probe)

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C`, `r30@+0x18`. `r30 = this`.
Stack: `+0x08` receives the `0x80064740` result.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2AC30` | `f32` | `24.0` |
| `0x80B2AC2C` | `f32` | `32.0` |
| `0x8042BB18` | `u8[]` | **(ext)** direction table, indexed by `*(u8 *)(this+0x391)` |
| `0x80B6EC58` | type token | **(ext)** |
| `0x80B6EC98` | type token | **(ext)** |

### Behaviour

```c
i32 probe = fn_800709C0(*(f32 *)(this + 0xAC),                     // f1
                        *(f32 *)(this + 0xB0) + 24.0f,             // f2, 0x80B2AC30
                        *(u8  *)(this + 0x38F),                    // r3
                        *(u8  *)(0x8042BB18 + *(u8 *)(this + 0x391)), // r4 (ext)
                        0x859);                                    // r5
if (probe == 0) {                                  // 0x80A0BD6C bne
    fn_80A0B690(this);
    return;
}

vt(this + 0x54C, 0x1C);
if (*(u32 *)(this + 0x750) != 0) {                 // 0x80A0BD98 beq
    vt(this + 0x69C, 0x1C);
    vt(this + 0x718, 0x14);
}

u32 st = *(u32 *)(this + 0x360);
if (st == 1) {                                     // 0x80A0BDCC beq
    u32 *found = fn_80A0B8A0(this, 0);              // 0x80A0BDE4
    if (found != 0) {                               // 0x80A0BDEC beq
        *(u32 *)(this + 0x75C) = found[0];
        *(u32 *)(this + 0x360) = 2;                 // li r0,2
        return;
    }
    if (fn_80064740(this, r1 + 0x08) == 0)          // 0x80A0BE10 beq
        return;
    f32 ax = fabsf(*(f32 *)(r1 + 0x08));            // fabs + frsp
    if (ax < 32.0f)                                 // 0x80A0BE30 bc 4,0 (bge)
        vt(this, 0xD4, 0x80B6EC58);
    return;
}
if (st == 2) {                                      // 0x80A0BDD4 beq
    void *o = fn_80162D00(*(u32 *)(this + 0x75C));
    r31 = o;
    if (o == 0) {                                   // 0x80A0BE64 bne
        vt(this, 0xD4, 0x80B6EC58);
        return;
    }
    if (fn_80A0B7F0(this) == 0)                     // 0x80A0BE98 beq
        return;
    if (o != 0)                                     // 0x80A0BEA0 beq
        *(u8 *)(this + 0x74C) = *(u8 *)((u8 *)o + 0x74C);
    *(u8 *)(this + 0x348) = (u8)fn_80A0B770(this);  // stb r3,840
    vt(this, 0xD4, 0x80B6EC98);
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A0BD64` | `0x800709C0` | `f1 = x`, `f2 = y + 24.0`, `r3 = *(u8 *)(this+0x38F)`, `r4 = *(u8 *)(0x8042BB18 + *(u8 *)(this+0x391))`, `r5 = 0x859` |
| `0x80A0BD74` | `0x80A0B690` | `r3 = this` |
| `0x80A0BD8C` | vtable `+0x1C` of `*(this+0x54C)` | `r3 = this+0x54C` |
| `0x80A0BDAC` | vtable `+0x1C` of `*(this+0x69C)` | `r3 = this+0x69C` |
| `0x80A0BDC0` | vtable `+0x14` of `*(this+0x718)` | `r3 = this+0x718` |
| `0x80A0BDE4` | `0x80A0B8A0` | `r3 = this`, `r4 = 0` |
| `0x80A0BE0C` | `0x80064740` | `r3 = this`, `r4 = r1+0x08` |
| `0x80A0BE50` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B6EC58` |
| `0x80A0BE5C` | `0x80162D00` | `r3 = *(u32 *)(this+0x75C)` |
| `0x80A0BE84` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B6EC58` |
| `0x80A0BE90` | `0x80A0B7F0` | `r3 = this` |
| `0x80A0BEB0` | `0x80A0B770` | `r3 = this` |
| `0x80A0BED0` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B6EC98` |

### Ambiguity

`0x8042BB18` is a `main.dol` byte table; the selected byte is an argument to
the probe, so its content shapes behaviour. Read it (or confirm it is the
`sin/cos`-direction table) before a byte-exact port; that is the bounded step
that keeps this row **conditional**. The two tokens are ordinary operands.

---

## 12. `0x80A2BC50` - two-actor push/bounce effect

* **TSV size:** 456 bytes
* **Reachable body:** 114 instructions; 3 `bl` + 6 `bctrl`; ends at
  `0x80A2BE14 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2bc50_FUN_80a2bc50.c`
* **Proposed signature:** `pub unsafe fn fn_80a2bc50(this: usize, other: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x50(r1)`; LR at `+0x54`; `stfd f31,+0x40` + `psq_st f31,qr0,+0x48`;
`r31@+0x3C`, `r30@+0x38`. `r30 = this` (incoming `r3`), `r31 = other`
(incoming `r4`). Stack: `+0x20/+0x24/+0x28` is the working `Vector3`;
`+0x08..+0x1C` are two dead `Vector3` copies.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2BB78` | `f32` | `0.0` |
| `0x80B2BB7C` | `f32` | `1.0` |

### Behaviour

```c
if (*(u8 *)(other + 0x38C) != 1)                    // 0x80A2BC78 cmpli 1 + bne
    return;

fn_806B4ED0(this, 0);                               // r4 = 0 (li)

f32 dx = *(f32 *)(other + 0xAC) - *(f32 *)(this + 0xAC);   // f0
f32 dy = *(f32 *)(other + 0xB0) - *(f32 *)(this + 0xB0);   // f2
*(f32 *)(r1 + 0x20) = dx;
*(f32 *)(r1 + 0x28) = 0.0f;                         // 0x80B2BB78
*(f32 *)(r1 + 0x24) = (dy > 0.0f) ? dy : 0.0f;      // fcmpo f3,f2 + bc 4,0 (bge)
fn_80170980(r1 + 0x20);                             // normalise the vector

f31 = *(f32 *)(this + 0x6C4);
f1  = vt(this, 0x2A0);                              // 0x80A2BCE0
*(f32 *)(r1 + 0x24) += f1 * (*(f32 *)(other + 0xB0) - f31);

f1 = vt(this, 0x294);                               // 0x80A2BD0C
*(f32 *)(r1 + 0x20) *= f1;
*(f32 *)(r1 + 0x24) *= f1;
*(f32 *)(r1 + 0x28) *= f1;

// 0x80A2BD3C..0x80A2BD70: two dead local copies
*(f32 *)(r1 + 0x14) = *(f32 *)(other + 0xAC);
*(f32 *)(r1 + 0x1C) = *(f32 *)(other + 0xB4);
*(f32 *)(r1 + 0x18) = *(f32 *)(other + 0xB0) + 1.0f;   // 0x80B2BB7C
*(f32 *)(r1 + 0x0C) = *(f32 *)(other + 0xB0) + 1.0f;
*(f32 *)(r1 + 0x10) = *(f32 *)(other + 0xB4);
*(f32 *)(r1 + 0x08) = *(f32 *)(other + 0xAC) + *(f32 *)(r1 + 0x20);

f1 = (*(u32 *)(other + 0x1090) == 3) ? vt(this, 0x29C)   // 0x80A2BD70 bne
                                     : vt(this, 0x298);
f32 c = *(f32 *)(r1 + 0x24);
if (c <= f1) { } else c = f1;                        // 0x80A2BDA4 bc 4,1 (ble)
*(f32 *)(r1 + 0x24) = c;

vt(this, 0x2B8);
u32 v = fn_806B4F40(this);
vt(other, 0x3FC, 1, v, 0, c /* f1 */, *(f32 *)(r1 + 0x20) /* f2 */);
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A2BC84` | `0x806B4ED0` | `r3 = this`, `r4 = 0` |
| `0x80A2BCC8` | `0x80170980` | `r3 = r1+0x20` |
| `0x80A2BCE0` | vtable `+0x2A0` of `*(this+0x60)` | `r3 = this` |
| `0x80A2BD0C` | vtable `+0x294` of `*(this+0x60)` | `r3 = this` |
| `0x80A2BD84` | vtable `+0x29C` of `*(this+0x60)` | `r3 = this` |
| `0x80A2BD9C` | vtable `+0x298` of `*(this+0x60)` | `r3 = this` |
| `0x80A2BDC4` | vtable `+0x2B8` of `*(this+0x60)` | `r3 = this` |
| `0x80A2BDCC` | `0x806B4F40` | `r3 = this` |
| `0x80A2BDF4` | vtable `+0x3FC` of `*(other+0x60)` | `r3 = other`, `r4 = 1`, `r5 = v`, `r6 = 0`, `f1 = min(c, lim)`, `f2 = *(r1+0x20)` |

### Ambiguity

None in-REL for the constants. The three non-vtable callees (`0x806B4ED0`,
`0x806B4F40`, `0x80170980`) live outside this REL, but every argument register
at those sites is literal. The two constants are the only global floats, both
in this REL.
The three `addi rX,r0,imm` forms at `0x80A2BDDC`, `0x80A2BDE4` and `0x80A2BC80`
are `li` (`r4 = 1`, `r6 = 0`, `r4 = 0`). The stores at `+0x08..+0x1C` are
never read again; they are literal dead copies of the other actor's position
and must still be emitted for byte equality. Ready.

---

## Cross-row notes

1. **No GQR dependence.** Every `psq_st`/`psq_l` in these twelve bodies is an
   `f31`/`f30` stack spill with `qr0`; none reads `GQR1..GQR7`. The `ldexpf`
   text in the Ghidra exports is the `0x43300000` integer-to-float idiom.
2. **No jump tables.** Every branch is a direct `b`/`bc`. There is no `bctr`
   except the six `bctrl` vtable calls per body at most.
3. **Termination.** All twelve reach a final `blr` at the address listed in
   each section. The only `b` to a TSV entry is the tail of the shared
   epilogue paths; none of the twelve tail-calls out.
4. **`bctrl` vs `bctr`.** All vtable sites are `0x4E800421` (LK=1); execution
   resumes at the next instruction.
5. **`main.dol` operands.** `0x80358E50`, `0x8042A078`, `0x80429D14`,
   `0x80429D20`, `0x80429D24`, `0x80429D5C`, `0x80429DD0`, `0x8042BB08`,
   `0x8042BB0C`, `0x8042BB18`, `0x80B6EC58`, `0x80B6EC98`, `0x80B70CE8`,
   `0x80B70D18`, `0x80B72E28`, `0x80B72F28`, `0x80B741F8`, `0x80B75EB0`,
   `0x80B760B8`, `0x80B76138`, `0x80B76178`, `0x80B792F8` are outside
   `d_enemiesNP.elf`. They are operands, not unresolved bases; only rows #6
   and #11 need their contents for a byte-exact port.
6. **`li` decoding.** Several stores in these bodies are `addi rX,r0,imm`.
   Because `rA = 0` in `addi` means the literal zero, these are `li` and the
   stored values are the immediates (`5`, `30`, `1`, `0`, `2`, `0x260`,
   `0x50`, `0x1E`, `0x180`, `0x400`, `0x300`). A decoder that reads `r0` as a
   GPR will mis-type them.

## Verification of this lane

* Read-only: no Rust file, no `ledger/functions.csv` row, and no existing doc
  was modified. The only file written is this page.
* All twelve addresses are absent from the `address` column of
  `ledger/functions.csv` (checked read-only against all 16,082 rows).
* All twelve are present in
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and the TSV sizes in the
  selection table match that file exactly.
* Reachable instruction counts were produced by walking the stream from the
  entry, following conditional branches and linked calls, and stopping at
  `blr` or a branch to a TSV entry. The counts match the wave-127 column for
  every row (143, 140, 137, 135, 135, 134, 125, 125, 123, 116, 115, 114), and
  the same walker reproduces all twelve wave-128 counts (287, 266, 225, 197,
  172, 170, 157, 156, 152, 150, 149, 148) as a control.
* The ELF mapping used for every word is
  `file_offset = (address - 0x80A00000) + 0x1000`, cross-checked against the
  segment table (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`, and
  `0x80B32DC0 -> 0x133DC0`).
* All float/`s16`/string values in the constants tables were read from the
  ELF image at the listed addresses, not inferred from the export.
