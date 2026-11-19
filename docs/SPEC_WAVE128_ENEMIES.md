# Wave 128 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 173 of the 384
formerly-skipped `d_enemiesNP` addresses as class **B**: the reachable
instruction stream decodes end-to-end, every path ends in `blr` or a tail call
to a real entry, and nothing depends on a `GQR1..GQR7` quantiser, a jump table,
an unresolved base or an incoming float argument.

This lane is **read-only**. It writes no Rust, no ledger row and no other doc.
It selects the twelve largest class-B bodies that are still absent from
`ledger/functions.csv`, decodes each one from
`work/rels/elf/d_enemiesNP.elf`, and records an implementation-ready
specification: signature, stack frame, every branch, every store, every call
site with its argument registers, every constant with its source address, and
the residual ambiguity.

All twelve were re-checked against the `address` column of
`ledger/functions.csv` (absent) and against `functions.tsv` (present). No row
below is a duplicate of an existing claim.

## Selection

Ordering is the wave-127 "reachable instruction count", which is the only
size metric that is not inflated by the TSV extent quirk (a TSV size is a
symbol bucket, not a body length). These are the twelve largest class-B rows;
the next candidate after this set is `0x80B0F800` at 140 instructions.

| # | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 1 | `0x80A37830` | 1148 | 287 | 6-state per-frame handler, drop/bounce physics | **ready** |
| 2 | `0x80AB5710` | 1064 | 266 | two-actor type test + bounce exchange | **ready** |
| 3 | `0x80AAC870` | 900 | 225 | N-slot matrix emitter (+0x60C array, +0x5CC, +0x58C) | **ready** |
| 4 | `0x80AFFC90` | 788 | 197 | 6-state `+0x8E3` timer machine with trajectory drift | **conditional** |
| 5 | `0x80AD3D50` | 688 | 172 | bubble actor step (collision, animation names, hit record) | **ready** |
| 6 | `0x80AAC360` | 680 | 170 | N-slot matrix emitter, Z-spread variant | **ready** |
| 7 | `0x80A7C7D0` | 628 | 157 | per-part model-matrix builder + 16-entry part table | **ready** |
| 8 | `0x80AB1510` | 624 | 156 | fishbone actor constructor | **ready** |
| 9 | `0x80A36C10` | 608 | 152 | Choropu appear/escape state machine | **conditional** |
| 10 | `0x80AEFCF0` | 600 | 150 | red-ring actor constructor | **ready** |
| 11 | `0x80AAC610` | 596 | 149 | single-slot matrix emitter (+0x5CC/+0x58C/+0x60C) | **ready** |
| 12 | `0x80AA8470` | 592 | 148 | node-list contact dispatch + hard-hit effect | **conditional** |

"Conditional" means the control flow and every operand are exact, but one
named datum lives in `main.dol` and must be read there before a byte-exact
port: `0x80B781C8..0x80B781EC` for #4, three absolute globals plus two
untranslated helpers for #9, and `0x8042A310` for #12. None of them is a
reason to skip.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`; data addresses in
  `0x80B2xxxx` / `0x80B7xxxx` use the same `+0x1000` shift (verified against
  `0x80B32DC0` at file offset `0x133DC0`).
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are
  byte offsets from `this`.
* An address such as `0x8042A310` or `0x80B75390` that does not resolve in
  `d_enemiesNP.elf` is a `main.dol` address. It is marked **(ext)** and must
  be treated as a named external datum, not as a missing base.
* Matrix helpers are the Dolphin-style SDK routines already named in the
  ledger: `0x801C0BD0` = translation-matrix builder
  (`PSMTXTrans(out, x, y, z)`), `0x801C0C60` = scale builder,
  `0x801C0540` = concat (`out = a * b`), `0x801C0500` = matrix copy,
  `0x8016A170` = copy, `0x8016A150` = concat, `0x8016A1A0` = identity,
  `0x8016EDD0` / `0x8016ECB0` = rotation-by-`u16` steps.
* `fN` values entering a block are written as `fN = ...`; integer argument
  registers are listed in order `(r3, r4, ...)` at the call site.

## Symbolic helpers used below

```text
fn_80a37830(this)                                   # this spec, #1
fn_80a36a50(this) -> i32                            # existing entry, returns bool-ish
fn_80a37de0(this) -> *const f32                     # existing entry, [0] is a position component
fn_80a38050(this)                                   # existing entry
kind_test(base, token) -> i32                       # #2:
  obj = (*(u32 *)(base + 0x394))->vt[0x28](base + 0x394)
  return (*(u32 *)obj)->vt[0x14](obj, token)
```

`kind_test` is the exact register sequence at the start of #2: `lwz r12,
0x394(r3)`; `lwz r12, 0x28(r12)`; `addi r3, r3, 0x394`; `bctrl`; then `lwz
r12, 0(r3)`; `lwz r12, 0x14(r12)`; `bctrl` with `r4 = token`. The receiver of
both calls is explicit, so this is not one of wave 127's class-C indirect
calls.

---

## 1. `0x80A37830` - six-state drop/bounce handler

* **TSV size:** 1148 bytes
* **Reachable body:** 287 instructions; 20 direct calls + 2 `bctrl`; ends at
  `0x80A37CA8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a37830_FUN_80a37830.c`
* **Proposed signature:** `pub unsafe fn fn_80a37830(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x30(r1)`; saves LR at `+0x34`, `f31` at `+0x20` (plus `psq_st
f31,qr0,+0x28`), `f30` at `+0x10` (plus `psq_st f30,qr0,+0x18`), `r31` at
`+0xC`, `r30` at `+0x8`. `r31 = this`; `r30 = 0x80B2C100` (constant page).
Epilogue at `0x80A37C84` restores in reverse order.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2C158` | `f32` | `1.0` |
| `0x80B2C160` | `f32` | `0.0` |
| `0x80B2C1A0` | `f32` | `5.0` |
| `0x80B2C1B4` | `f32` | `0.8` |
| `0x80B2C1C0` | `f32` | `1.3` |
| `0x80B2C1D8` | `f32` | `2.0` |
| `0x80B2C1DC` | `f32` | `15.0` |
| `0x80B2C1E0` | `f32` | `64.0` |
| `0x80B2C1E4` | `f32` | `1.9` |
| `0x80B2C1E8` | `f32` | `1.2` |
| `0x80B2C1EC` | `f32` | `0.012` |
| `0x80B2C1F0` | `f32` | `-0.012` |
| `0x80B70498` | type-token | **(ext)** `r4` of the state-6 vtable call |

### Behaviour

```c
// 0x80A37860-0x80A37870: (*(this+0x544))->vt[0x1C](this+0x544)
r3 = this + 0x544; call1(*(u32 *)(*(u32 *)(this + 0x544) + 0x1C));

fn_8006CDE0(this);                                   // A
fn_800955F0(this);                                   // B

if (fn_80072BC0(this + 0x1EC) != 0)                  // C (r3 only)
    *(f32 *)(this + 0xEC) = 0.0f;                    // 0x80B2C160

if (fn_8006FA40(this + 0x1EC, 0) != 0) {             // D
    *(f32 *)(this + 0xE8) = 0.0f;                    // 0x80B2C160
    if (*(u8 *)(this + 0x854) != 0) {                // 0x80A378C0 bne
        fn_801680E0(this + 0x584, 1.3f);             // 0x80B2C1C0
    } else {
        fn_801680E0(this + 0x584, 0.8f);             // 0x80B2C1B4
        *(u32 *)(this + 0x360) = 6;
    }
}

fn_80A38050(this);                                   // 0x80A378EC

if (*(u8 *)(this + 0x864) != 0 && *(u8 *)(this + 0x854) == 0) {
    *(f32 *)(this + 0xE8) = 0.0f;
    fn_801680E0(this + 0x584, 0.8f);                 // 0x80B2C1B4
    *(u8  *)(this + 0x864) = 0;
    *(u32 *)(this + 0x360) = 6;
}
```

State dispatch at `0x80A3792C` on `*(u32 *)(this + 0x360)`; the compares are
sequential `cmpi`/`beq` for exactly 1,2,3,4,5,6 and a `b` to the epilogue for
everything else.

```c
switch (*(u32 *)(this + 0x360)) {
case 1:                                              // 0x80A37964
    if (fn_80A36A50(this) == 0) return;              // 0x80A37970 beq
    f2 = *(f32 *)(this + 0xE8);                      // fall height
    f0 = f2 / 15.0f;                                 // 0x80B2C1DC
    *(u8  *)(this + 0x854) = 0;
    *(u32 *)(this + 0x360) = 2;
    *(f32 *)(this + 0x844) = f2;
    *(f32 *)(this + 0x84C) = 0.0f;                   // 0x80B2C160
    *(f32 *)(this + 0x850) = 0.0f;
    *(f32 *)(this + 0x848) = f0;

    if (f2 == 0.0f) f31 = 1.0f;                      // 0x80A379C8, 0x80B2C158
    else            f31 = 64.0f * (fabsf(f2) / 1.9f); // 0x80B2C1E0, 0x80B2C1E4

    if (*(u8 *)(this + 0x348) == 0) {                // 0x80A379D4 beq
        p  = fn_80A37DE0(this);                      // r3 = *const f32
        f2 = p[0] - f31;
        f1 = *(f32 *)(this + 0xAC);
        f1 = f1 - f2;
        f4 = -f1;
        if (f4 > 0.0f) f4 = -f4;                     // 0x80A379F8 bge skips fneg
    } else {
        p  = fn_80A37DE0(this);
        f2 = f31 + p[0];
        f1 = *(f32 *)(this + 0xAC);
        f4 = f2 - f1;
        if (f4 < 0.0f) f4 = -f4;                     // 0x80A37A28 ble skips fneg
    }

    if (f4 == 0.0f) {                                // 0x80A37A38 beq
        *(f32 *)(this + 0x84C) = 0.0f;
        *(f32 *)(this + 0x850) = 0.0f;
        *(f32 *)(this + 0xE8)  = 0.0f;
    } else {
        f2 = 2.0f;                                   // 0x80B2C1D8
        f3 = *(f32 *)(this + 0x844);
        f1 = *(f32 *)(this + 0x848);
        f0 = f2 * f4;                                // 2 * f4
        f2 = f2 * f3;                                // 2 * old E8
        f3 = f1 - f3;                                // (E8/15) - old E8
        f2 = f2 * f3;
        f1 = f3 * f3;
        f1 = f2 + f1;
        f0 = f1 / f0;
        *(f32 *)(this + 0x84C) = f0;                 // horizontal speed
        f30 = f3 / f0;                               // 0x80A37A6C
        f31 = fn_801680C0(this + 0x584);             // 0x80A37A70
        f1  = fn_801680C0(this + 0x584);             // 0x80A37A7C
        f0 = 1.2f * f1;                              // 0x80B2C1E8
        f0 = f0 - f31;
        f0 = f0 / f30;
        *(f32 *)(this + 0x850) = f0;
    }

    // 0x80A37AA4: snap tiny speeds to a minimum
    f2 = *(f32 *)(this + 0x84C);
    f1 = 0.012f;                                     // 0x80B2C1EC
    if (fabsf(f2) > 0.012f) return;                  // cror eq=lt|eq; bne 0x80A37C84
    if (f2 == 0.0f) return;                          // 0x80A37AC8
    if (f2 > 0.0f) *(f32 *)(this + 0x84C) = 0.012f;  // 0x80A37AD4
    else           *(f32 *)(this + 0x84C) = -0.012f; // 0x80A37ADC, 0x80B2C1F0
    return;

case 2:                                              // 0x80A37AE8
    if (*(f32 *)(this + 0x850) > 0.0f) {
        f1 = fn_801680C0(this + 0x584);
        f1 = *(f32 *)(this + 0x850) + f1;
        if (f1 >= 1.3f) f1 = 1.3f;                   // 0x80B2C1C0
        fn_801680E0(this + 0x584, f1);
    }
    f1 = *(f32 *)(this + 0xE8);
    f0 = *(f32 *)(this + 0x84C);
    f2 = *(f32 *)(this + 0x844);
    *(f32 *)(this + 0xE8) = f1 + f0;
    if (fabsf(*(f32 *)(this + 0xE8)) >= fabsf(f2))   // 0x80A37B48 cror eq=gt|eq
        *(f32 *)(this + 0xE8) = *(f32 *)(this + 0x848);
    f1 = *(f32 *)(this + 0x848);
    if ((f1 > 0.0f && f1 >= *(f32 *)(this + 0xE8)) ||
        (f1 <= 0.0f && f1 <= *(f32 *)(this + 0xE8))) {
        *(u32 *)(this + 0x360) = 3;                  // 0x80A37BA0
        f1 = fn_801680C0(this + 0x584);
        *(f32 *)(this + 0x5C4) = f1;
        *(f32 *)(this + 0xE8)  = 0.0f;
        *(f32 *)(this + 0x5C8) = (frsp(f1) - 0.8f) / 5.0f; // 0x80B2C1B4, 0x80B2C1A0
    }
    return;

case 3:                                              // 0x80A37BD8
    r3 = fn_8015F470(this + 0x5C4, 0.8f, *(f32 *)(this + 0x5C8));
    f1 = *(f32 *)(this + 0x5C4);
    fn_801680E0(this + 0x584, f1);
    if (r3 == 0) return;                             // 0x80A37BFC beq
    *(u32 *)(this + 0x360) = 4;
    return;

case 4:                                              // 0x80A37C0C
    old = *(u8 *)(this + 0x348);
    *(u8 *)(this + 0x86C) = old;
    r = fn_80096240(this, this + 0xAC);              // r4 = &this->AC
    *(u8 *)(this + 0x348) = (u8)r;
    *(u32 *)(this + 0x360) = ((u8)r == old) ? 6 : 5; // 0x80A37C30 beq
    return;

case 5:                                              // 0x80A37C4C
    if (fn_80A36A50(this) == 0) return;
    *(u32 *)(this + 0x360) = 6;
    return;

case 6:                                              // 0x80A37C68
    call1(*(u32 *)(*(u32 *)(this + 0x60) + 0xD4), this, 0x80B70498); // (ext) token
    return;
}
```

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A37870` | vtable `+0x1C` of `*(this+0x544)` | `r3 = this+0x544` |
| `0x80A37878` | `0x8006CDE0` | `r3 = this` |
| `0x80A37880` | `0x800955F0` | `r3 = this` |
| `0x80A37888` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80A378A4` | `0x8006FA40` | `r3 = this+0x1EC`, `r4 = 0` |
| `0x80A378CC` | `0x801680E0` | `r3 = this+0x584`, `f1 = 0.8` |
| `0x80A378E4` | `0x801680E0` | `r3 = this+0x584`, `f1 = 1.3` |
| `0x80A378EC` | `0x80A38050` | `r3 = this` |
| `0x80A37918` | `0x801680E0` | `r3 = this+0x584`, `f1 = 0.8` |
| `0x80A37968` | `0x80A36A50` | `r3 = this` |
| `0x80A379DC` | `0x80A37DE0` | `r3 = this` |
| `0x80A37A08` | `0x80A37DE0` | `r3 = this` |
| `0x80A37A70` | `0x801680C0` | `r3 = this+0x584` |
| `0x80A37A7C` | `0x801680C0` | `r3 = this+0x584` |
| `0x80A37AFC` | `0x801680C0` | `r3 = this+0x584` |
| `0x80A37B20` | `0x801680E0` | `r3 = this+0x584`, `f1 = clamped` |
| `0x80A37BAC` | `0x801680C0` | `r3 = this+0x584` |
| `0x80A37BE4` | `0x8015F470` | `r3 = this+0x5C4`, `f1 = 0.8`, `f2 = *(this+0x5C8)` |
| `0x80A37BF4` | `0x801680E0` | `r3 = this+0x584`, `f1 = *(this+0x5C4)` |
| `0x80A37C1C` | `0x80096240` | `r3 = this`, `r4 = this+0xAC` |
| `0x80A37C50` | `0x80A36A50` | `r3 = this` |
| `0x80A37C80` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B70498` |

### Ambiguity

None in the body. `0x80B70498` is a `main.dol` type token; its address is the
operand and its value is not needed to reproduce the call. The two
`fn_80A37DE0` sites read only `p[0]`, so the pointee type can be
`*const f32`.

---

## 2. `0x80AB5710` - two-actor kind test and bounce exchange

* **TSV size:** 1064 bytes
* **Reachable body:** 266 instructions; 24 `bctrl` type-test calls + 2 direct
  calls; ends at `0x80AB5B34 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab5710_FUN_80ab5710.c`
* **Proposed signature:** `pub unsafe fn fn_80ab5710(this: usize, other: usize)`
* **Confidence:** **high / ready** (only the two type-token names are unknown)

### Frame

`stwu r1,-0x40(r1)`; LR at `+0x44`; `f31`/`f30` saved as `stfd`+`psq_st`
pairs at `+0x30`/`+0x38` and `+0x20`/`+0x28`; `r31@+0x1C`, `r30@+0x18`,
`r29@+0x14`, `r28@+0x10`. `r29 = this`, `r30 = other`,
`r31 = 0x80B2FC70` (constant page).

### Constants

| address | type | value |
|---|---|---|
| `0x80B2FCE4` | `f32` | `0.0` |
| `0x80B2FCF4` | `f32` | `0.75` |
| `0x80B2FCF8` | `f32` | `-1.25` |
| `0x80B2FCFC` | `f32` | `0.5` |
| `0x80B2FD00` | `f32` | `0.35` |
| `0x80B2FC98` | `f32[2]` | `{ 1.0, -1.0 }` (index is the new `+0x348` byte) |
| `0x80B75390` | type token | **(ext)** first kind token |
| `0x80B75350` | type token | **(ext)** second kind token |
| `0x80429D60` | `u32` ptr | **(ext)** sound/effect manager |

### Behaviour

```c
f31 = fabsf(*(f32 *)(this  + 0xEC));
f30 = fabsf(*(f32 *)(other + 0xEC));
// fcmpo: *(this+0xB0) vs *(other+0xB0); ble -> the second arm
if (*(f32 *)(this + 0xB0) > *(f32 *)(other + 0xB0)) {
    a = kind_test(this, 0x80B75390);                 // 0x80AB5774..0x80AB5798
    b = kind_test(this, 0x80B75350);                 // 0x80AB579C..0x80AB57D0
    if (a < 0 || b < 0)                              // neg/or/rlwinm; beq 0x80AB57F4
        *(f32 *)(this + 0xEC) = 0.75f * f31;         // 0x80B2FCF4
    c = kind_test(other, 0x80B75390);                // 0x80AB57F4..0x80AB581C
    d = kind_test(other, 0x80B75350);                // 0x80AB5820..0x80AB5854
    if (c < 0 || d < 0)                              // beq 0x80AB5980
        *(f32 *)(other + 0xEC) = -1.25f * f30;       // 0x80B2FCF8
} else {
    a = kind_test(this, 0x80B75390);                 // 0x80AB587C..0x80AB58A0
    b = kind_test(this, 0x80B75350);                 // 0x80AB58A4..0x80AB58D8
    if (a < 0 || b < 0)                              // beq 0x80AB58FC
        *(f32 *)(this + 0xEC) = -1.25f * f31;
    c = kind_test(other, 0x80B75390);                // 0x80AB58FC..0x80AB5924
    d = kind_test(other, 0x80B75350);                // 0x80AB5928..0x80AB595C
    if (c < 0 || d < 0)                              // beq 0x80AB5980
        *(f32 *)(other + 0xEC) = 0.75f * f30;
}

// common at 0x80AB5980
u8 old = *(u8 *)(this + 0x348);
*(u8 *)(this + 0x348) = old ^ 1;
f32 sign = *(f32 *)(0x80B2FC98 + 4 * (old ^ 1));    // old==0 -> -1, old==1 -> +1
f3 = frsp(fabsf(*(f32 *)(this  + 0xE8)));
f4 = frsp(fabsf(*(f32 *)(other + 0xE8)));
f1 = (0.5f * (f3 + f4)) * sign;                      // 0x80B2FCFC
*(f32 *)(this  + 0xE8) = f1;                         // 0x80AB59C4
*(f32 *)(other + 0xE8) = -f1;                        // 0x80AB59D4

if (fabsf(f1) < 0.35f) {                             // 0x80AB59D8 bge skips
    a = kind_test(this, 0x80B75390);                 // 0x80AB59E0..0x80AB5A08
    b = kind_test(this, 0x80B75350);                 // 0x80AB5A0C..0x80AB5A40
    if (a < 0 || b < 0)                              // beq 0x80AB5A60
        *(f32 *)(this + 0xE8) = 0.0f;                // 0x80B2FCE4
    c = kind_test(other, 0x80B75390);                // 0x80AB5A60..0x80AB5A88
    d = kind_test(other, 0x80B75350);                // 0x80AB5A8C..0x80AB5AC0
    if (c < 0 || d < 0)                              // beq 0x80AB5AE0
        *(f32 *)(other + 0xE8) = 0.0f;
}

// 0x80AB5AE0
r28 = *(u32 *)0x80429D60;                            // (ext)
fn_8006A4B0(&local[8], this + 0xAC);                 // (r3=&local8, r4=this+0xAC)
fn_80198C30(r28, 0x2D0, &local[8], 0);
```

`kind_test` uses the receiver directly: `(*(u32 *)(base+0x394))->vt[0x28]
(base+0x394)` returns an object pointer in `r3`, then `(*(u32 *)r3)->vt[0x14]
(r3, token)`. All twenty-four calls are `bctrl` at the sites listed in the
table above; there is no unresolved receiver.

### Call sites (direct)

| site | callee | arguments |
|---|---|---|
| `0x80AB5AF0` | `0x8006A4B0` | `r3 = &local8`, `r4 = this+0xAC` |
| `0x80AB5B04` | `0x80198C30` | `r3 = *(0x80429D60)`, `r4 = 0x2D0`, `r5 = &local8`, `r6 = 0` |

The 24 `bctrl` sites form twelve identical two-call chains. In every chain,
`base` is `this` or `other`; the first call is
`obj = (*(u32 *)(base+0x394))->vt[0x28](base+0x394)` and the second is
`(*obj)->vt[0x14](obj, token)`:

| chain | base | `vt[0x28]` site | `vt[0x14]` site | token |
|---:|---|---|---|---|
| 1 | `this` | `0x80AB5780` | `0x80AB5798` | `0x80B75390` |
| 2 | `this` | `0x80AB57B8` | `0x80AB57D0` | `0x80B75350` |
| 3 | `other` | `0x80AB5804` | `0x80AB581C` | `0x80B75390` |
| 4 | `other` | `0x80AB583C` | `0x80AB5854` | `0x80B75350` |
| 5 | `this` | `0x80AB5888` | `0x80AB58A0` | `0x80B75390` |
| 6 | `this` | `0x80AB58C0` | `0x80AB58D8` | `0x80B75350` |
| 7 | `other` | `0x80AB590C` | `0x80AB5924` | `0x80B75390` |
| 8 | `other` | `0x80AB5944` | `0x80AB595C` | `0x80B75350` |
| 9 | `this` | `0x80AB59F0` | `0x80AB5A08` | `0x80B75390` |
| 10 | `this` | `0x80AB5A28` | `0x80AB5A40` | `0x80B75350` |
| 11 | `other` | `0x80AB5A70` | `0x80AB5A88` | `0x80B75390` |
| 12 | `other` | `0x80AB5AA8` | `0x80AB5AC0` | `0x80B75350` |

For every chain the `vt[0x28]` argument register is `r3 = base+0x394`; the
`vt[0x14]` call uses `r3 = obj`, `r4 = token`. The return value of the second
call is the `i32` tested for `>= 0` / `< 0`.

### Ambiguity

The bodies of the two token tests are outside this REL; only their addresses
are operands, so behaviour is exact. `0x80429D60` is a pointer global. The
`+0x348` byte is a direction flag: it is consumed only through `^1` and the
two-entry `{+1,-1}` table.

---

## 3. `0x80AAC870` - N-slot matrix emitter (`this->E20` copies)

* **TSV size:** 900 bytes
* **Reachable body:** 225 instructions; ends at `0x80AACBF0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aac870_FUN_80aac870.c`
* **Proposed signature:** `pub unsafe fn fn_80aac870(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x1A0(r1)`; LR `+0x1A4`; `r31@+0x19C`, `r30@+0x198`,
`r29@+0x194`, `r28@+0x190`. `r29 = this`, `r31 = 0x80B2F880`.
Stack scratch: `+0x20..+0x30` (3 halfwords + 3 floats), `+0x34..+0x3C`
(3 floats), matrices at `+0x40/+0x70/+0xA0/+0xD0/+0x100/+0x130/+0x160`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2F8B0` | `f32` | `8.0` |
| `0x80B2F8B4` | `f32` | `0.0` |
| `0x80B2F8D0` | `f32` | `-8.0` |

### Behaviour

```c
// 0x80AAC898: save the actor's orientation triple
v[0] = *(f32 *)(this + 0xDC);
v[1] = *(f32 *)(this + 0xE0);
v[2] = *(f32 *)(this + 0xE4);   // stored at local+0x28/0x2C/0x30

if (*(u32 *)(this + 0xE20) != 0) {                  // 0x80AAC8B8 beq skips loop
  for (u8 i = 0; i < *(u32 *)(this + 0xE20); i++) { // 0x80AAC9DC cmpl/blt
    p  = this + i * 0x0C;
    pos[0] = *(f32 *)(p + 0xBB8);
    pos[1] = *(f32 *)(p + 0xBBC);
    q  = this + i * 0x06;
    s16 rot[3] = { *(s16 *)(q + 0xD5C), *(s16 *)(q + 0xD5E), *(s16 *)(q + 0xD60) };
    fn_80064AF0(&pos[0], &rot[0], 1);               // fills pos[2] as well
    fn_801C0BD0(&mtx, pos[0], pos[1], pos[2]);
    fn_8016EDD0(&mtx, &rot[1]);                     // rotation about the middle s16
    fn_801C0BD0(&tmp, 0.0f, 8.0f, 0.0f);            // f1=0, f2=8, f3=0
    fn_801C0540(&mtx, &tmp, &mtx);
    u16 yaw = *(u16 *)(this + 0x100);
    fn_8016ECB0(&mtx, &yaw);
    fn_801C0BD0(&tmp, 0.0f, -8.0f, 0.0f);           // f1=0, f2=-8, f3=0
    fn_801C0540(&mtx, &tmp, &mtx);
    obj = this + ((u8)i << 6) + 0x60C;              // rlwinm 6,18,25
    fn_8016A170(obj, &mtx);                         // copy
    fn_8016A150(obj, &v[0]);                        // concat
    fn_8016A1A0(obj, 0);                            // identity
    (*(u32 *)obj)->vt[0x14](obj);                   // bctrl, r3 = obj
  }
}

// 0x80AAC9EC: one-shot +0x5CC object, same recipe with fields 0xBAC/0xBB0
pos[0] = *(f32 *)(this + 0xBAC);
pos[1] = *(f32 *)(this + 0xBB0);
s16 rot[3] = { *(s16 *)(this + 0xD56), *(s16 *)(this + 0xD58), *(s16 *)(this + 0xD5A) };
fn_80064AF0(&pos[0], &rot[0], 1);
fn_801C0BD0(&mtx, pos[0], pos[1], pos[2]);
fn_8016EDD0(&mtx, &rot[1]);
fn_801C0BD0(&tmp, 0.0f, 8.0f, 0.0f);  fn_801C0540(&mtx, &tmp, &mtx);
u16 yaw = *(u16 *)(this + 0x100);     fn_8016ECB0(&mtx, &yaw);
fn_801C0BD0(&tmp, 0.0f, -8.0f, 0.0f); fn_801C0540(&mtx, &tmp, &mtx);
obj = this + 0x5CC;
fn_8016A170(obj, &mtx); fn_8016A150(obj, &v[0]); fn_8016A1A0(obj, 0);
(*(u32 *)obj)->vt[0x14](obj);

// 0x80AACAE0: one-shot +0x58C object, fields 0xBA0/0xBA4, angles 0xD50/0xD52/0xD54
pos[0] = *(f32 *)(this + 0xBA0);
pos[1] = *(f32 *)(this + 0xBA4);
s16 rot[3] = { *(s16 *)(this + 0xD50), *(s16 *)(this + 0xD52), *(s16 *)(this + 0xD54) };
fn_80064AF0(&pos[0], &rot[0], 1);
fn_801C0BD0(&mtx, pos[0], pos[1], pos[2]);
fn_8016EDD0(&mtx, &rot[1]);
fn_801C0BD0(&tmp, 0.0f, 8.0f, 0.0f);  fn_801C0540(&mtx, &tmp, &mtx);
u16 yaw = *(u16 *)(this + 0x100);     fn_8016ECB0(&mtx, &yaw);
fn_801C0BD0(&tmp, 0.0f, -8.0f, 0.0f); fn_801C0540(&mtx, &tmp, &mtx);
obj = this + 0x58C;
fn_8016A170(obj, &mtx); fn_8016A150(obj, &v[0]); fn_8016A1A0(obj, 0);
(*(u32 *)obj)->vt[0x14](obj);
```

`fn_80064AF0` receives `(r3 = &pos, r4 = &rot, r5 = 1)`. It writes all three
floats of `pos`; the third slot is read afterwards by `0x801C0BD0`, so the
helper is not optional. `0x8016A150` uses the `+0xDC/+0xE0/+0xE4` triple saved
at entry.

### Ambiguity

None in-REL. The `+0x60C` array stride is 64 bytes, indexed by the low byte
of the loop counter (`(u8)i << 6`), matching the TSV body. `0x80064AF0` is a
`main.dol` helper and is not in the ledger; its argument registers are exact
and its write of `pos[2]` is proven by the following `lfs`.

---

## 4. `0x80AFFC90` - six-state `+0x8E3` timer machine with trajectory drift

* **TSV size:** 788 bytes
* **Reachable body:** 197 instructions; ends at `0x80AFFFA0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80affc90_FUN_80affc90.c`
* **Proposed signature:** `pub unsafe fn fn_80affc90(this: usize, pos: *const f32)`
* **Confidence:** **medium / conditional** (external float block)

### Frame

`stwu r1,-0x20(r1)`; LR `+0x24`; `r31@+0x1C`, `r30@+0x18`, `r29@+0x14`,
`r28@+0x10`. `r29 = this`, `r30 = pos` (the incoming `r4`), `r31 =
0x80B318B8`, `r28 = 0x80B781C8` **(ext)**.

### Constants

| address | type | value |
|---|---|---|
| `0x80B318B8` | `f32` | `1.0` |
| `0x80B318C0` | `f32` | `0.0` |
| `0x80B318C4` | `f32` | `1.25` |
| `0x80B318C8` | `f32` | `-1.25` |
| `0x80B781C8` | `f32` | **(ext)** initial `+0x8CC` value |
| `0x80B781CC` | `f32` | **(ext)** initial `+0x8D0` value |
| `0x80B781D0` | `f32` | **(ext)** initial `+0x8D4` value |
| `0x80B781D4` | `f32` | **(ext)** per-frame `+0x8CC` delta |
| `0x80B781D8` | `f32` | **(ext)** per-frame `+0x8D0` delta |
| `0x80B781DC` | `f32` | **(ext)** per-frame `+0x8D4` delta |
| `0x80B781EC` | `f32` | **(ext)** per-frame `+0x8DC` delta |
| `0x80429D60` | pointer | **(ext)** sound/effect manager |

### Behaviour

```c
*(f32 *)(this + 0x8C0) = pos[0];
*(f32 *)(this + 0x8C4) = pos[1];
*(f32 *)(this + 0x8C8) = pos[2];

u8 st = *(u8 *)(this + 0x8E3);
if (st == 1) {
  if (*(u16 *)(this + 0x8E0) == 0x186) {
    fn_80167B60(this + 0x48, 0, 0);                  // r4=0, r5=0
    fn_80169F80(this + 0x04, this + 0x48);           // f1 = 1.0
    fn_800B8250(this + 0x74, 0);
    fn_800B8250(this + 0x74, 1);
    *(f32 *)(this + 0x8CC) = *(f32 *)0x80B781C8;     // (ext)
    *(f32 *)(this + 0x8D0) = *(f32 *)0x80B781CC;     // (ext)
    *(f32 *)(this + 0x8D4) = *(f32 *)0x80B781D0;     // (ext)
    *(f32 *)(this + 0x8D8) = 0.0f;
    *(f32 *)(this + 0x8DC) = 0.0f;
    *(u8  *)(this + 0x8E2) = 1;
    fn_8006A4B0(&local8, pos);                       // r3=&local8, r4=pos
    fn_80198C30(*(u32 *)0x80429D60, 0x26A, &local8, 0);
  }
  u16 t = *(u16 *)(this + 0x8E0);
  *(u16 *)(this + 0x8E0) = t - 1;
  if ((u16)(t - 1) == 0) {                           // 0x80AFFD90 rlwinm.
    *(u16 *)(this + 0x8E0) = 0x5A;
    *(u8  *)(this + 0x8E3) = 2;
  }
} else if (st == 2) {                                // 0x80AFFDAC
  u16 t = *(u16 *)(this + 0x8E0);
  *(u16 *)(this + 0x8E0) = t - 1;
  if ((u16)(t - 1) == 0) {                           // timer expired
    u32 w = *(u32 *)(this + 0x74);
    *(u32 *)(this + 0x74) = w & 0xFFFFFFFD;          // clear bit 1
    fn_800B8250(this + 0x74, 3);                     // r3=this+0x74, r4=3
    fn_800B82D0(this + 0x74);
    *(f32 *)(this + 0x8D8) = 0.0f;
    *(u16 *)(this + 0x8E0) = 0x14;
    *(u8  *)(this + 0x8E2) = 0;
    *(u8  *)(this + 0x8E3) = 3;
    fn_80AFFA50(pos);                                // r3 = pos
  } else {                                           // 0x80AFFE10
    if ((t - 1) & 1)      *(f32 *)(this + 0x8D8) = 0.0f;
    else if ((t - 1) & 2) *(f32 *)(this + 0x8D8) = 1.25f;   // 0x80B318C4
    else                  *(f32 *)(this + 0x8D8) = -1.25f;  // 0x80B318C8
  }
} else if (st == 3) {                                // 0x80AFFE44
  u16 t = *(u16 *)(this + 0x8E0);
  *(u16 *)(this + 0x8E0) = t - 1;
  if ((u16)(t - 1) == 0)
    *(u8 *)(this + 0x8E3) = 0;
} else if (st == 4) {                                // 0x80AFFE6C
  u32 w = *(u32 *)(this + 0x74);
  *(u32 *)(this + 0x74) = w & 0xFFFFFFFD;
  fn_800B8250(this + 0x74, 2);
  fn_800B82D0(this + 0x74);
  *(f32 *)(this + 0x8D8) = 0.0f;
  *(u8  *)(this + 0x8E2) = 0;
  *(u8  *)(this + 0x8E3) = 0;
  fn_80AFFA50(pos);
} else if (st == 5) {                                // 0x80AFFEB0
  if (*(u16 *)(this + 0x8E0) == 0x64) {
    u32 w = *(u32 *)(this + 0x74);
    *(u32 *)(this + 0x74) = w & 0xFFFFFFFD;
    fn_800B8250(this + 0x74, 4);
    *(f32 *)(this + 0x8D8) = 0.0f;
    *(f32 *)(this + 0x8DC) = 0.0f;
    fn_80AFFAA0(pos);
  }
  // 0x80AFFEEC: drift, always applied in state 5
  *(f32 *)(this + 0x8CC) -= *(f32 *)0x80B781D4;      // (ext)
  *(f32 *)(this + 0x8D0) -= *(f32 *)0x80B781D8;      // (ext)
  *(f32 *)(this + 0x8DC) += *(f32 *)0x80B781EC;      // (ext)
  *(f32 *)(this + 0x8D4) -= *(f32 *)0x80B781DC;      // (ext)
  u16 t = *(u16 *)(this + 0x8E0);
  *(u16 *)(this + 0x8E0) = t - 1;
  if ((u16)(t - 1) == 0) {
    *(f32 *)(this + 0x8D8) = 0.0f;
    *(f32 *)(this + 0x8DC) = 0.0f;
    *(u8  *)(this + 0x8E2) = 0;
    *(u8  *)(this + 0x8E3) = 0;
  }
}

// 0x80AFFF5C: common tail
if (*(u8 *)(this + 0x8E2) != 0)
    (*(u32 *)(this + 0x48))->vt[0x14](this + 0x48);   // bctrl, r3 = this+0x48
fn_800B8110(this + 0x74);
return;
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AFFD00` | `0x80167B60` | `r3 = this+0x48`, `r4 = 0`, `r5 = 0` |
| `0x80AFFD10` | `0x80169F80` | `r3 = this+0x4`, `r4 = this+0x48`, `f1 = 1.0` |
| `0x80AFFD1C` | `0x800B8250` | `r3 = this+0x74`, `r4 = 0` |
| `0x80AFFD28` | `0x800B8250` | `r3 = this+0x74`, `r4 = 1` |
| `0x80AFFD6C` | `0x8006A4B0` | `r3 = &local8`, `r4 = pos` |
| `0x80AFFD80` | `0x80198C30` | `r3 = *(0x80429D60)`, `r4 = 0x26A`, `r5 = &local8`, `r6 = 0` |
| `0x80AFFDD8` | `0x800B8250` | `r3 = this+0x74`, `r4 = 3` |
| `0x80AFFDE0` | `0x800B82D0` | `r3 = this+0x74` |
| `0x80AFFE08` | `0x80AFFA50` | `r3 = pos` |
| `0x80AFFE84` | `0x800B8250` | `r3 = this+0x74`, `r4 = 2` |
| `0x80AFFE8C` | `0x800B82D0` | `r3 = this+0x74` |
| `0x80AFFEA8` | `0x80AFFA50` | `r3 = pos` |
| `0x80AFFED4` | `0x800B8250` | `r3 = this+0x74`, `r4 = 4` |
| `0x80AFFEE8` | `0x80AFFAA0` | `r3 = pos` |
| `0x80AFFF78` | vtable `+0x14` of `*(this+0x48)` | `r3 = this+0x48` |
| `0x80AFFF80` | `0x800B8110` | `r3 = this+0x74` |

### Ambiguity

The seven `0x80B781xx` floats are in `main.dol`, not in this REL. The control
flow and the exact `(ext)` addresses are fixed; only their numeric values are
missing, which is why this row is **conditional**, not ready. Read
`0x80B781C8..0x80B781EC` from the DOL (or from the runtime image) before
claiming byte-equality. `0x80AFFA50` / `0x80AFFAA0` are ordinary entries in
this REL.

---

## 5. `0x80AD3D50` - bubble actor step

* **TSV size:** 688 bytes
* **Reachable body:** 172 instructions; ends at `0x80AD3FFC blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad3d50_FUN_80ad3d50.c`
* **Proposed signature:** `pub unsafe fn fn_80ad3d50(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR `+0x44`; `r31@+0x3C`, `r30@+0x38`, `r29@+0x34`,
`r28@+0x30`. `r31 = this`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B30920` | `f32` | `8.0` |
| `0x80B30930` | `f32` | `1.0` |
| `0x80B76630` | type token | **(ext)** |
| `0x80352AF4` | `u32` | **(ext)** animation token for `u8(+0x38C)==1` |
| `0x80352B34` | `u32` | **(ext)** animation token for `u8(+0x38C)==2` |
| `0x80429E18` | `u32` ptr | **(ext)** effect/arg object |
| `0x8042A1C8` | `u32` ptr | **(ext)**, byte `+0x120E` used |
| `0x80429FC8` | `u32` ptr | **(ext)** |
| `0x8042A1C8 + 0x120E` | `u8` | **(ext)** |

### Behaviour

```c
if (!(kind_test(this, 0x80B76630) < 0)) return;      // 0x80AD3D94..0x80AD3DA0
u8 cls = fn_800212C0(this) & 0xFF;                   // 0x80AD3DA8
if (cls == 0) return;

if (*(u32 *)(this + 0x84C) == 2 && *(u32 *)(this + 0x854) == 0) {
  if ((*(u32 *)(this + 0x60C) & 8) != 0 || cls == 3) {
    a[0] = *(f32 *)(this + 0xAC) - 8.0f;             // 0x80B30920
    a[1] = *(f32 *)(this + 0xB0) + 8.0f;
    a[2] = 0.0f;
    rec[0] = *(u8 *)(this + 0x68D);
    rec[1] = 1; rec[2] = 1;
    rec[3] = *(u8 *)(this + 0x694);
    u16 zero = 0;                                    // stored at rec[4]
    fn_80087F40(*(u32 *)0x80429E18, &a[0], *(u8 *)(this + 0x694), /*f1=*/ 8.0f);
    fn_800D0470(*(u32 *)0x80429FC8, this + 0x81C, 3,
                *(u8 *)(*(u32 *)0x8042A1C8 + 0x120E),
                *(u32 *)(this + 0x850));
    fn_8007F950(this + 0x534);
    fn_80064D10(this, 1);
    return;
  }
  *(u32 *)(this + 0x854) = 0x11;
}

*(f32 *)(this + 0xB4) += 1.0f;                       // 0x80B30930

if (cls == 1) {                                      // 0x80AD3E84 cmpli
  call1(*(u32 *)(*(u32 *)(this + 0x60) + 0xD4), this, 0x80352AF4);
  *(u8 *)(this + 0x68A) = 2;
  *(u8 *)(this + 0x689) = 0;
} else if (cls == 2) {
  call1(*(u32 *)(*(u32 *)(this + 0x60) + 0xD4), this, 0x80352B34);
  *(u8 *)(this + 0x68A) = 1;
  *(u8 *)(this + 0x689) = 1;
}

u32 keep = 1;
if (fn_80AD4220(this) == 0) keep = 0;                // 0x80AD3F00
if (*(u32 *)(this + 0x84C) == 2 && *(u32 *)(this + 0x854) == 0x11) keep = 0;
if (keep != 0)
  fn_800D0470(*(u32 *)0x80429FC8, this + 0x81C, 0,
              *(u8 *)(*(u32 *)0x8042A1C8 + 0x120E),
              *(u32 *)(this + 0x850));

fn_80AD42B0(this);

// 0x80AD3F60: 12-byte record + 2 flag bytes
rec.f[0] = *(f32 *)(this + 0xAC);
rec.f[1] = *(f32 *)(this + 0xB0);
rec.f[2] = *(f32 *)(this + 0xB4);
rec.u8[0x0C] = *(u8 *)(this + 0x694);
rec.u32[0x10] = *(u32 *)(this + 0x854);
rec.u8[0x14] = 0;
rec.u8[0x15] = (*(u32 *)(this + 0x858) != 0 && *(u8 *)(this + 0x689) == 0);
rec.u8[0x16] = 0;
if (*(u32 *)(this + 0x71C) != 0)
    rec.u8[0x16] = (u8)(*(u32 *)(this + 0x718) + 1);
fn_80022600(this, &rec);
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AD3D78` | vtable `+0x28` of `*(this+0x394)` | `r3 = this+0x394` |
| `0x80AD3D90` | vtable `+0x14` of the returned object | `r3 = obj`, `r4 = 0x80B76630` |
| `0x80AD3DA8` | `0x800212C0` | `r3 = this` |
| `0x80AD3E30` | `0x80087F40` | `r3 = *(0x80429E18)`, `r4 = &a[0]`, `r5 = *(u8 *)(this+0x694)`, `f1 = 8.0` |
| `0x80AD3E54` | `0x800D0470` | `r3 = *(0x80429FC8)`, `r4 = this+0x81C`, `r5 = *(u8 *)(*(0x8042A1C8)+0x120E)`, `r6 = 3`, `r7 = *(this+0x850)` |
| `0x80AD3E5C` | `0x8007F950` | `r3 = this+0x534` |
| `0x80AD3E68` | `0x80064D10` | `r3 = this`, `r4 = 1` |
| `0x80AD3EB0` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80352AF4` |
| `0x80AD3EE8` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80352B34` |
| `0x80AD3F00` | `0x80AD4220` | `r3 = this` |
| `0x80AD3F54` | `0x800D0470` | same as above with `r6 = 0` |
| `0x80AD3F5C` | `0x80AD42B0` | `r3 = this` |
| `0x80AD3FE4` | `0x80022600` | `r3 = this`, `r4 = &rec` |

### Ambiguity

All numeric constants are in this REL. The two animation tokens and three
absolute pointers are `main.dol` data; they are operands, not bases. The
`u8` at `0x8042A1C8 + 0x120E` is a global scene flag. Ready.

---

## 6. `0x80AAC360` - N-slot matrix emitter, Z-spread variant

* **TSV size:** 680 bytes
* **Reachable body:** 170 instructions; ends at `0x80AAC604 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aac360_FUN_80aac360.c`
* **Proposed signature:** `pub unsafe fn fn_80aac360(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x110(r1)`; LR `+0x114`; `f31` at `+0x100`/`+0x108`; `r31@+0xFC`,
`r30@+0xF8`, `r29@+0xF4`, `r28@+0xF0`. `r28 = this`, `r31 = 0x80B2F880`,
`f31 = 16.0f`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2F8A4` | `f32` | `16.0` |
| `0x80B2F8B0` | `f32` | `8.0` |
| `0x80B2F8B4` | `f32` | `0.0` |
| `0x80B2F8BC` | `f32` | `1.0` |
| `0x80B2F8D0` | `f32` | `-8.0` |

### Behaviour

```c
v[0] = *(f32 *)(this + 0xDC);
v[1] = *(f32 *)(this + 0xE0);
v[2] = *(f32 *)(this + 0xE4);        // local +0x18/1C/20

f32 spread = *(f32 *)(this + 0xB0) + 16.0f * *(f32 *)(this + 0xE0);
// local +0x28 = spread, +0x24 = this->AC, +0x2C = this->B4

// 0x80AAC674: first matrix -> +0x5CC
fn_801C0BD0(&mtx, *(f32 *)(this + 0xAC), spread, *(f32 *)(this + 0xB4));
u16 yaw = *(u16 *)(this + 0x102);
fn_8016EDD0(&mtx, &yaw);
fn_8016A170(this + 0x5CC, &mtx);
fn_8016A150(this + 0x5CC, &v[0]);
fn_8016A1A0(this + 0x5CC, 0);
(*(u32 *)(this + 0x5CC))->vt[0x14](this + 0x5CC);

// 0x80AAC3DC: loop over this->E20
for (u8 i = 0; i < *(u32 *)(this + 0xE20); i++) {
    fn_801C0BD0(&mtx, *(f32 *)(this + 0xAC), spread, *(f32 *)(this + 0xB4));
    u16 yaw = *(u16 *)(this + 0x102);
    fn_8016EDD0(&mtx, &yaw);
    obj = this + ((u8)i << 6) + 0x60C;
    fn_8016A170(obj, &mtx);
    fn_8016A150(obj, &v[0]);
    fn_8016A1A0(obj, 0);
    (*(u32 *)obj)->vt[0x14](obj);
    spread += 16.0f * *(f32 *)(this + 0xE0);         // 0x80AAC44C..0x80AAC460
}

// 0x80AAC4D4: +0x58C, with a scale matrix and a Z offset of -1
v[0] = *(f32 *)(this + 0xDBC);
v[1] = *(f32 *)(this + 0xDC0);
v[2] = *(f32 *)(this + 0xDC4);
f32 z = *(f32 *)(this + 0xB4) - 1.0f;                // 0x80B2F8BC
spread += 16.0f * *(f32 *)(this + 0xE0);
fn_801C0BD0(&mtx, *(f32 *)(this + 0xAC), spread, z);
fn_801C0C60(&scale, v[0], v[1], v[2]);               // scale matrix
fn_801C0540(&mtx, &scale, &mtx);
u16 yaw = *(u16 *)(this + 0x102); fn_8016EDD0(&mtx, &yaw);
fn_801C0BD0(&tmp, 0.0f, 8.0f, 0.0f);  fn_801C0540(&mtx, &tmp, &mtx);
u16 pitch = *(u16 *)(this + 0x100);   fn_8016ECB0(&mtx, &pitch);
fn_801C0BD0(&tmp, 0.0f, -8.0f, 0.0f); fn_801C0540(&mtx, &tmp, &mtx);
fn_8016A170(this + 0x58C, &mtx);
fn_8016A1A0(this + 0x58C, 0);
(*(u32 *)(this + 0x58C))->vt[0x14](this + 0x58C);
```

Note: the `+0x58C` block does **not** call `0x8016A150`; the concat of `v` is
replaced by the scale matrix built from `0xDBC/0xDC0/0xDC4`. The `+0x5CC` and
`+0x60C` objects do concat `v`.

### Ambiguity

None in-REL. The loop is a do-while whose first condition is tested at
`0x80AAC464` (`cmpl`, `blt`) after a forward `b 0x80AAC464`; the walker counts
170 instructions including that entry block.

---

## 7. `0x80A7C7D0` - per-part model matrix builder

* **TSV size:** 628 bytes
* **Reachable body:** 157 instructions; ends at `0x80A7CA40 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7c7d0_FUN_80a7c7d0.c`
* **Proposed signature:**
  `pub unsafe fn fn_80a7c7d0(this: usize, part: u32, out_mtx_slot: *mut *mut f32)`
* **Confidence:** **high / ready**

The third argument is a **pointer slot**, not the matrix itself: every copy
site loads `r3`/`r4` with `lwz rX, 0(r5)` or `lwz rX, 0(r30)`. In the block
below, `*out_mtx_slot` is the destination matrix address.

### Frame

`stwu r1,-0x240(r1)`; LR `+0x244`; `r31@+0x23C`, `r30@+0x238`,
`r29@+0x234`, `r28@+0x230`. `r28 = this`, `r29 = part`, `r30 = out_mtx_slot`
(incoming `r5`), `r31 = 0x80B2DDF0`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2DE20` | `f32` | `0.0` |
| `0x80B2DE28` | `f32` | `1.0` |
| `0x80B2E168` | `f32` | `-10.5` |
| `0x80B2E16C` | `f32` | `10.5` |
| `0x80B2DFB0` | `u8[16]` | `{2,3,4,5,6,7,8,9,12,13,14,15,16,17,18,19}` |

The table is 16 entries even though the wave-127 note calls it "9-entry"; the
loop bound is `0x80B2DFC0` and the bytes above are the full extent.

### Behaviour

```c
if (part == 0x15) {                                  // 0x80A7C7D8 cmpli
    fn_801C0500(*out_mtx_slot, &local1F8);                 // copy caller matrix in
    sub = *(u32 *)(this + 0x04);
    if (*(u8 *)(sub + 0xE9D) != 0) {
        u16 a = *(u16 *)(sub + 0xE94);               // s16 load
        fn_8016EDD0(&local1F8, &a);
    }
    fn_801C0BD0(&local138, 0.0f, *(f32 *)(sub + 0xE84), 0.0f);
    fn_801C0540(&local1F8, &local138, &local1F8);
    fn_801C0C60(&local108, 1.0f, *(f32 *)(sub + 0xE8C), *(f32 *)(sub + 0xE88));
    fn_801C0540(&local1F8, &local108, &local1F8);
    fn_801C0500(&local1F8, *out_mtx_slot);                 // copy back out
    return;
}

// 0x80A7C89C: scan the 16-entry table for `part`
for (u8 *p = 0x80B2DFB0; p < 0x80B2DFC0; p++) {
    if (*p != (u8)part) continue;
    fn_801C0500(*out_mtx_slot, &local1C8);
    sub = *(u32 *)(this + 0x04);
    fn_801C0BD0(&localD8, *(f32 *)(sub + 0xE68), 0.0f, 0.0f);
    fn_801C0540(&local1C8, &localD8, &local1C8);
    fn_801C0500(&local1C8, *out_mtx_slot);
    break;
}

sub = *(u32 *)(this + 0x04);
if (*(u8 *)(sub + 0xE9D) == 0) return;

if ((u8)part == 1) {                                 // 0x80A7C918 cmpli 1
    fn_801C0500(*out_mtx_slot, &local198);
    fn_801C0BD0(&localA8, -10.5f, 0.0f, 0.0f);        // 0x80B2E168
    fn_801C0540(&local198, &localA8, &local198);
    u16 a = *(u16 *)(sub + 0xE94);
    fn_8016EDD0(&local198, &a);
    fn_801C0BD0(&local78, 10.5f, 0.0f, 0.0f);         // 0x80B2E16C
    fn_801C0540(&local198, &local78, &local198);
    fn_801C0500(&local198, *out_mtx_slot);
} else if ((u8)part == 0x0B) {                       // 0x80A7C9A0 cmpli 0xB
    fn_801C0500(*out_mtx_slot, &local168);
    fn_801C0BD0(&local48, -10.5f, 0.0f, 0.0f);
    fn_801C0540(&local168, &local48, &local168);
    u16 a = *(u16 *)(sub + 0xE94);
    fn_8016EDD0(&local168, &a);
    fn_801C0BD0(&local18, 10.5f, 0.0f, 0.0f);
    fn_801C0540(&local168, &local18, &local168);
    fn_801C0500(&local168, *out_mtx_slot);
}
```

`0x801C0500` is a copy with `(r3 = src, r4 = dst)`: the first call in the
`part == 0x15` branch is `r3 = *out_mtx_slot, r4 = &local1F8`, which only makes
sense as "copy the caller matrix into local scratch". The final call is the
reverse pair. The two calls bracket every transform, so the order is part of
the spec, not an inference.

### Ambiguity

The plane/model field layout (`+4` sub-object, `+0xE68`, `+0xE84`,
`+0xE88/+0xE8C`, `+0xE94`, `+0xE9D`) is used literally. The table is fully
in-REL. Ready.

---

## 8. `0x80AB1510` - fishbone actor constructor

* **TSV size:** 624 bytes
* **Reachable body:** 156 instructions; ends at `0x80AB177C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab1510_FUN_80ab1510.c`
* **Proposed signature:** `pub unsafe fn fn_80ab1510(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x40(r1)`; LR `+0x44`; `r31@+0x3C`, `r30@+0x38`, `r29@+0x34`,
`r28@+0x30`. `r28 = this`, `r29 = 0x80B54B90` (string page),
`r30@...` reused for the second string page.

### Constants / strings

| address | type | value |
|---|---|---|
| `0x80B2FAF0` | `f32` | `1.0` |
| `0x80B2FAE4` | `f32` | `0.0` |
| `0x80B54C1C` | `char[]` | `"g3d/fishbone.brres"` |
| `0x80B54C30` | `char[]` | `"fishbone"` |
| `0x80B54C3C` | `char[]` | `"swim"` |
| `0x80B54C10` | `char[]` | `"attack_st"` |
| `0x80B54C44` | `char[]` | `"fish_bone_eye_light"` |
| `0x80B54C58` | `char[]` | `"fish_bone_eye_light_mask"` |
| `0x80377C48` | `u32` | **(ext)** resource/user word passed as `r5` |
| `0x8042A038` | `u32` ptr | **(ext)** archive handle; `+4` is passed |

### Behaviour

```c
fn_800690C0(this + 0x524, 0xFFFFFFFF, *(u32 *)0x80377C48, 0, 0x20);
*(u32 *)(this + 0x540) =
    fn_800DF180(*(u32 *)0x8042A038 + 4, "fishbone", "g3d/fishbone.brres");
r31 = fn_80239E30(this + 0x540, "fishbone");
fn_80169D90(this + 0x544, &r31, this + 0x524, 0x128, 1, 0);
fn_80064BD0(this + 0x544);
fn_80AB13E0(this);
r30 = fn_8023A0B0(this + 0x540, "swim");
fn_801650D0(this + 0x704, &r31, &r30, this + 0x524, 0);
fn_801651F0(this + 0x704, this + 0x544, &r30, 0);
fn_80169F80(this + 0x544, this + 0x704, 1.0f);
r3 = fn_8023A190(this + 0x540, "attack_st");
fn_80165C30(this + 0x73C, &r31, &r3, this + 0x524, 0, 1);
(*(u32 *)(this + 0x544))->vt[0x18](this + 0x544, this + 0x73C);
fn_80166340(this + 0x73C, 1, 0);
fn_801662F0(this + 0x73C, 0.0f, 0);

if (*(u32 *)(this + 0x988) == 1) {
    r31 = fn_80239E30(this + 0x540, "fish_bone_eye_light");
    fn_80169D90(this + 0x6C4, &r31, this + 0x524, 0x128, 1, 0);
    r3 = fn_8023A190(this + 0x540, "fish_bone_eye_light");
    fn_80165C30(this + 0x794, &r31, &r3, this + 0x524, 0, 1);
    (*(u32 *)(this + 0x6C4))->vt[0x18](this + 0x6C4, this + 0x794);
    fn_80166340(this + 0x794, 1, 0);
    fn_801662F0(this + 0x794, 0.0f, 0);
    r3 = fn_80239E30(this + 0x540, "fish_bone_eye_light_mask");
    fn_80169D90(this + 0x684, &r3, this + 0x524, 0x120, 1, 0);
    fn_8016A2A0(this + 0x684, 0xFFFFFFFF, 0x88);
}

fn_800690E0(this + 0x524);
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AB1550` | `0x800690C0` | `r3 = this+0x524`, `r4 = -1`, `r5 = *(0x80377C48)`, `r6 = 0`, `r7 = 0x20` |
| `0x80AB1568` | `0x800DF180` | `r3 = *(0x8042A038)+4`, `r4 = "fishbone"`, `r5 = "g3d/fishbone.brres"` |
| `0x80AB1578` | `0x80239E30` | `r3 = this+0x540`, `r4 = "fishbone"` |
| `0x80AB159C` | `0x80169D90` | `r3 = this+0x544`, `r4 = &r31`, `r5 = this+0x524`, `r6 = 0x128`, `r7 = 1`, `r8 = 0` |
| `0x80AB15A4` | `0x80064BD0` | `r3 = this+0x544` |
| `0x80AB15AC` | `0x80AB13E0` | `r3 = this` |
| `0x80AB15B8` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = "swim"` |
| `0x80AB15DC` | `0x801650D0` | `r3 = this+0x704`, `r4 = &r31`, `r5 = &r30`, `r6 = this+0x524`, `r7 = 0` |
| `0x80AB15F4` | `0x801651F0` | `r3 = this+0x704`, `r4 = this+0x544`, `r5 = &r30`, `r6 = 0` |
| `0x80AB1608` | `0x80169F80` | `r3 = this+0x544`, `r4 = this+0x704`, `f1 = 1.0` |
| `0x80AB1614` | `0x8023A190` | `r3 = this+0x540`, `r4 = "attack_st"` |
| `0x80AB1638` | `0x80165C30` | `r3 = this+0x73C`, `r4 = &r31`, `r5 = &r3`, `r6 = this+0x524`, `r7 = 0`, `r8 = 1` |
| `0x80AB1650` | vtable `+0x18` of `*(this+0x544)` | `r3 = this+0x544`, `r4 = this+0x73C` |
| `0x80AB1660` | `0x80166340` | `r3 = this+0x73C`, `r4 = 1`, `r5 = 0` |
| `0x80AB1674` | `0x801662F0` | `r3 = this+0x73C`, `f1 = 0.0`, `r4 = 0` |
| `0x80AB168C` | `0x80239E30` | `r3 = this+0x540`, `r4 = "fish_bone_eye_light"` |
| `0x80AB16B0` | `0x80169D90` | `r3 = this+0x6C4`, `r4 = &r31`, `r5 = this+0x524`, `r6 = 0x128`, `r7 = 1`, `r8 = 0` |
| `0x80AB16BC` | `0x8023A190` | `r3 = this+0x540`, `r4 = "fish_bone_eye_light"` |
| `0x80AB16E0` | `0x80165C30` | `r3 = this+0x794`, `r4 = &r31`, `r5 = &r3`, `r6 = this+0x524`, `r7 = 0`, `r8 = 1` |
| `0x80AB16F8` | vtable `+0x18` of `*(this+0x6C4)` | `r3 = this+0x6C4`, `r4 = this+0x794` |
| `0x80AB1708` | `0x80166340` | `r3 = this+0x794`, `r4 = 1`, `r5 = 0` |
| `0x80AB1718` | `0x801662F0` | `r3 = this+0x794`, `f1 = 0.0`, `r4 = 0` |
| `0x80AB1724` | `0x80239E30` | `r3 = this+0x540`, `r4 = "fish_bone_eye_light_mask"` |
| `0x80AB1744` | `0x80169D90` | `r3 = this+0x684`, `r4 = &r3`, `r5 = this+0x524`, `r6 = 0x120`, `r7 = 1`, `r8 = 0` |
| `0x80AB1754` | `0x8016A2A0` | `r3 = this+0x684`, `r4 = -1`, `r5 = 0x88` |
| `0x80AB175C` | `0x800690E0` | `r3 = this+0x524` |

### Ambiguity

None; all names are in-REL strings. `0x80377C48` is the only external operand
and is passed straight through as `r5`.

---

## 9. `0x80A36C10` - Choropu appear/escape state machine

* **TSV size:** 608 bytes
* **Reachable body:** 152 instructions; ends at `0x80A36E6C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a36c10_FUN_80a36c10.c`
* **Proposed signature:** `pub unsafe fn fn_80a36c10(this: usize)`
* **Confidence:** **medium / conditional** (external globals and two
  untranslated helpers)

### Frame

`stwu r1,-0x50(r1)`; LR `+0x54`; `r31@+0x4C`, `r30@+0x48`, `r29@+0x44`.
`r31 = this`, `r30 = 0x80B3DD68` (string page), `r29 = 0x80B2C100`.

### Constants / strings

| address | type | value |
|---|---|---|
| `0x80B2C164` | `f32` | `8.0` |
| `0x80B2C1AC` | `f32` | `5500.0` |
| `0x80B2C160` | `f32` | `0.0` |
| `0x80B3DDA8` | `char[]` | `"Wm_en_choroappear"` |
| `0x80B3DDBC` | `char[]` | `"Wm_en_burst_m"` |
| `0x80B3DDCC` | `char[]` | `"Wm_en_choroescape"` |
| `0x80429D5C` | `u32` ptr | **(ext)** |
| `0x80429D28` | `u8` | **(ext)** |
| `0x80429DD0` | `u32` ptr | **(ext)** |

### Behaviour

```c
u32 st = *(u32 *)(this + 0x360);
if (st == 1) {
  if ((*(u32 *)(this + 0x04) & 1) != 0) {            // 0x80A36C58 rlwinm.
    fn_8008C330(this + 0x148);
    *(u32 *)(this + 0x5BC) = 0;
    *(u32 *)(this + 0x5C0) = 0;
    *(u32 *)(this + 0x360) = 2;
    return;
  }
  f1 = 8.0f + *(f32 *)(this + 0xB0);                 // 0x80B2C164
  v[0] = *(f32 *)(this + 0xAC);
  v[1] = f1;
  v[2] = 5500.0f;                                    // 0x80B2C1AC
  // bctrl: r3 = this+0x5E4, r4 = 0x80B3DDA8, r5 = 0, r6 = &v[0], r7 = 0,
  //        r8 = 0, f1 = 8+B0; callee = *(*(this+0x5E4) + 0xA0)
  call_indirect(*(u32 *)(*(u32 *)(this + 0x5E4) + 0xA0));

  *(u32 *)(this + 0x5C0) -= 1;                       // addic./stw
  if (*(u32 *)(this + 0x5C0) > 0) return;            // bgt 0x80A36E54

  v[0] = *(f32 *)(this + 0xAC);
  v[1] = *(f32 *)(this + 0xB0) + *(f32 *)(this + 0xD4);
  v[2] = 5500.0f;
  fn_8016C890(0x80B3DDBC, 0, &v[0], 0, 0);
  fn_8016C890(0x80B3DDCC, 0, &v[0], 0, 0);
  fn_8006A4B0(&local8, this + 0xAC);
  fn_80197F00(*(u32 *)0x80429D5C, 0x21A, &local8, 0);

  s16 ix = (s16)(s32)*(f32 *)(this + 0x5CC);
  s16 iz = (s16)(s32)-*(f32 *)(this + 0x5D0);
  r = fn_80077810(*(u32 *)0x80429DD0, (u16)ix, (u16)iz, *(u8 *)0x80429D28);
  if (r >= 0) {                                      // bne 0x80A36DEC when r<0
    *(f32 *)(this + 0x858) = *(f32 *)(this + 0xAC);
    *(f32 *)(this + 0x85C) = *(f32 *)(this + 0xB0) + *(f32 *)(this + 0xD4);
    *(f32 *)(this + 0x860) = 0.0f;
    fn_80064610(0x15E, 0, this + 0x858, 0, 0);
    fn_800777B0(*(u32 *)0x80429DD0, (u16)ix, (u16)iz, *(u8 *)(this + 0x38F));
  }
  fn_8008C330(this + 0x148);
  *(u32 *)(this + 0x5BC) = 0;
  *(u32 *)(this + 0x5C0) = 0;
  *(u32 *)(this + 0x360) = 2;
  return;
}
if (st == 2) {
  *(u32 *)(this + 0x5C0) -= 1;
  if (*(u32 *)(this + 0x5C0) > 0) return;
  call1(*(u32 *)(*(u32 *)(this + 0x60) + 0xD4), this, 0x80B70418); // (ext) token
  return;
}
return;                                              // all other states
```

The virtual call in state 1 is `lwzu r12,0x5E4(r3)` (so `r3` becomes
`this+0x5E4`), then `lwz r12,0xA0(r12)`, then `bctrl` with `r4 = 0x80B3DDA8`,
`r5 = 0`, `r6 = &v[0]`, `r7 = 0`, `r8 = 0`, and `f1 = 8 + this->B0`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A36CA0` | vtable `+0xA0` of `*(this+0x5E4)` | `r3 = this+0x5E4`, `r4 = 0x80B3DDA8`, `r5 = 0`, `r6 = &v[0]`, `r7 = 0`, `r8 = 0`, `f1 = 8+B0` |
| `0x80A36CE8` | `0x8016C890` | `r3 = 0x80B3DDBC`, `r4 = 0`, `r5 = &v[0]`, `r6 = 0`, `r7 = 0` |
| `0x80A36D00` | `0x8016C890` | `r3 = 0x80B3DDCC`, `r4 = 0`, `r5 = &v[0]`, `r6 = 0`, `r7 = 0` |
| `0x80A36D14` | `0x8006A4B0` | `r3 = &local8`, `r4 = this+0xAC` |
| `0x80A36D28` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 0x21A`, `r5 = &local8`, `r6 = 0` |
| `0x80A36D68` | `0x80077810` | `r3 = *(0x80429DD0)`, `r4 = (u16)ix`, `r5 = (u16)iz`, `r6 = *(u8)0x80429D28` |
| `0x80A36DB0` | `0x80064610` | `r3 = 0x15E`, `r4 = 0`, `r5 = this+0x858`, `r6 = 0`, `r7 = 0` |
| `0x80A36DE8` | `0x800777B0` | `r3 = *(0x80429DD0)`, `r4 = (u16)ix`, `r5 = (u16)iz`, `r6 = *(u8)(this+0x38F)` |
| `0x80A36DF0` | `0x8008C330` | `r3 = this+0x148` |
| `0x80A36E10` | `0x8008C330` | `r3 = this+0x148` |
| `0x80A36E50` | vtable `+0xD4` of `*(this+0x60)` | `r3 = this`, `r4 = 0x80B70418` |

### Ambiguity

`0x80429D28` is a byte read as an argument, `0x80429DD0` is a pointer, and
`0x80429D5C` is a pointer; all three are in `main.dol`. `0x80077810` and
`0x800777B0` are not in the ledger. Their argument registers are exact. This
is the bounded step that keeps the row **conditional**: read those globals
and confirm the two helpers' parameter meanings before the port.

---

## 10. `0x80AEFCF0` - red-ring actor constructor

* **TSV size:** 600 bytes
* **Reachable body:** 150 instructions; ends at `0x80AEFF44 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aefcf0_FUN_80aefcf0.c`
* **Proposed signature:** `pub unsafe fn fn_80aefcf0(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x50(r1)`; LR `+0x54`; `r31@+0x4C`, `r30@+0x48`, `r29@+0x44`,
`r28@+0x40`. `r28 = this`, `r31 = 0x80B62340` (string page).

### Constants / strings

| address | type | value |
|---|---|---|
| `0x80B3129C` | `f32` | `1.0` |
| `0x80B6234C` | `char[]` | `"g3d/red_ring.brres"` |
| `0x80B62360` | `char[]` | `"red_ring"` |
| `0x80B6236C` | `char[]` | `"red_ring_fog"` |
| `0x80377C48` | `u32` | **(ext)** |
| `0x8042A038` | `u32` ptr | **(ext)** |

### Behaviour

```c
fn_800690C0(this + 0x524, 0xFFFFFFFF, *(u32 *)0x80377C48, 0, 0x20);
*(u32 *)(this + 0x540) =
    fn_800DF180(*(u32 *)0x8042A038 + 4, "red_ring", "g3d/red_ring.brres");
r29 = fn_80239E30(this + 0x540, "red_ring");
fn_80169D90(this + 0x544, &r29, this + 0x524, 0x224, 1, 0);
fn_80064BF0(this + 0x544);
r3 = fn_8023A0B0(this + 0x540, "red_ring");
fn_801650D0(this + 0x584, &r29, &r3, this + 0x524, 0);
*(u32 *)(this + 0x5BC) = fn_8023A270(this + 0x540, "red_ring");
fn_80167420(this + 0x5C0, &r29, &r3, this + 0x524, 0, 1);
fn_80167800(this + 0x5C0, this + 0x544, &*(this + 0x5BC), 0, 0);

r29 = fn_80239E30(this + 0x540, "red_ring_fog");
fn_80169D90(this + 0x5EC, &r29, this + 0x524, 0x32C, 1, 0);
fn_80064BF0(this + 0x5EC);
*(u32 *)(this + 0x62C) = fn_8023A270(this + 0x540, "red_ring_fog");
fn_80167420(this + 0x630, &r29, &r3, this + 0x524, 0, 1);
fn_80167800(this + 0x630, this + 0x5EC, &*(this + 0x62C), 0, 0);
fn_80169F80(this + 0x5EC, this + 0x630, 1.0f);

r31 = fn_8023A190(this + 0x540, "red_ring_fog");
fn_80165C30(this + 0x65C, &r29, &r31, this + 0x524, 0, 1);
fn_80165FD0(this + 0x65C, this + 0x5EC, &r31, 0, 0);
(*(u32 *)(this + 0x5EC))->vt[0x18](this + 0x5EC, this + 0x65C);
fn_80166310(this + 0x65C, 1.0f, 0);
fn_800690E0(this + 0x524);
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AEFD30` | `0x800690C0` | `r3 = this+0x524`, `r4 = -1`, `r5 = *(0x80377C48)`, `r6 = 0`, `r7 = 0x20` |
| `0x80AEFD48` | `0x800DF180` | `r3 = *(0x8042A038)+4`, `r4 = "red_ring"`, `r5 = "g3d/red_ring.brres"` |
| `0x80AEFD58` | `0x80239E30` | `r3 = this+0x540`, `r4 = "red_ring"` |
| `0x80AEFD7C` | `0x80169D90` | `r3 = this+0x544`, `r4 = &r29`, `r5 = this+0x524`, `r6 = 0x224`, `r7 = 1`, `r8 = 0` |
| `0x80AEFD84` | `0x80064BF0` | `r3 = this+0x544` |
| `0x80AEFD90` | `0x8023A0B0` | `r3 = this+0x540`, `r4 = "red_ring"` |
| `0x80AEFDB0` | `0x801650D0` | `r3 = this+0x584`, `r4 = &r29`, `r5 = &r3`, `r6 = this+0x524`, `r7 = 0` |
| `0x80AEFDBC` | `0x8023A270` | `r3 = this+0x540`, `r4 = "red_ring"` |
| `0x80AEFDE4` | `0x80167420` | `r3 = this+0x5C0`, `r4 = &r29`, `r5 = &r3`, `r6 = this+0x524`, `r7 = 0`, `r8 = 1` |
| `0x80AEFE04` | `0x80167800` | `r3 = this+0x5C0`, `r4 = this+0x544`, `r5 = &*(this+0x5BC)`, `r6 = 0`, `r7 = 0` |
| `0x80AEFE10` | `0x80239E30` | `r3 = this+0x540`, `r4 = "red_ring_fog"` |
| `0x80AEFE34` | `0x80169D90` | `r3 = this+0x5EC`, `r4 = &r29`, `r5 = this+0x524`, `r6 = 0x32C`, `r7 = 1`, `r8 = 0` |
| `0x80AEFE3C` | `0x80064BF0` | `r3 = this+0x5EC` |
| `0x80AEFE48` | `0x8023A270` | `r3 = this+0x540`, `r4 = "red_ring_fog"` |
| `0x80AEFE70` | `0x80167420` | `r3 = this+0x630`, `r4 = &r29`, `r5 = &r3`, `r6 = this+0x524`, `r7 = 0`, `r8 = 1` |
| `0x80AEFE90` | `0x80167800` | `r3 = this+0x630`, `r4 = this+0x5EC`, `r5 = &*(this+0x62C)`, `r6 = 0`, `r7 = 0` |
| `0x80AEFEA4` | `0x80169F80` | `r3 = this+0x5EC`, `r4 = this+0x630`, `f1 = 1.0` |
| `0x80AEFEB0` | `0x8023A190` | `r3 = this+0x540`, `r4 = "red_ring_fog"` |
| `0x80AEFED8` | `0x80165C30` | `r3 = this+0x65C`, `r4 = &r29`, `r5 = &r31`, `r6 = this+0x524`, `r7 = 0`, `r8 = 1` |
| `0x80AEFEF4` | `0x80165FD0` | `r3 = this+0x65C`, `r4 = this+0x5EC`, `r5 = &r31`, `r6 = 0`, `r7 = 0` |
| `0x80AEFF0C` | vtable `+0x18` of `*(this+0x5EC)` | `r3 = this+0x5EC`, `r4 = this+0x65C` |
| `0x80AEFF1C` | `0x80166310` | `r3 = this+0x65C`, `f1 = 1.0`, `r4 = 0` |
| `0x80AEFF24` | `0x800690E0` | `r3 = this+0x524` |

### Ambiguity

None; the only external data are the two `main.dol` handles, used as
arguments. Ready.

---

## 11. `0x80AAC610` - single-slot matrix emitter (`+0x5CC`, `+0x58C`, `+0x60C`)

* **TSV size:** 596 bytes
* **Reachable body:** 149 instructions; ends at `0x80AAC860 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aac610_FUN_80aac610.c`
* **Proposed signature:** `pub unsafe fn fn_80aac610(this: usize)`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x100(r1)`; LR `+0x104`; `r31@+0xFC`, `r30@+0xF8`. `r30 = this`,
`r31 = 0x80B2F880`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2F89C` | `f32` | `32.0` |
| `0x80B2F8A4` | `f32` | `16.0` |
| `0x80B2F8B0` | `f32` | `8.0` |
| `0x80B2F8B4` | `f32` | `0.0` |
| `0x80B2F8BC` | `f32` | `1.0` |
| `0x80B2F8D0` | `f32` | `-8.0` |

### Behaviour

```c
v[0] = *(f32 *)(this + 0xDC);
v[1] = *(f32 *)(this + 0xE0);
v[2] = *(f32 *)(this + 0xE4);        // local +0x18/1C/20

f32 spread = *(f32 *)(this + 0xB0) + (*(f32 *)(this + 0xE04) - 32.0f);
// local +0x28 = spread, +0x24 = this->AC, +0x2C = this->B4

// 0x80AAC674: +0x5CC
fn_801C0BD0(&mtx, *(f32 *)(this + 0xAC), spread, *(f32 *)(this + 0xB4));
u16 yaw = *(u16 *)(this + 0x102); fn_8016EDD0(&mtx, &yaw);
fn_8016A170(this + 0x5CC, &mtx);
fn_8016A150(this + 0x5CC, &v[0]);
fn_8016A1A0(this + 0x5CC, 0);
(*(u32 *)(this + 0x5CC))->vt[0x14](this + 0x5CC);

// 0x80AAC6C4: +0x58C with scale + Z offset
v[0] = *(f32 *)(this + 0xDBC);
v[1] = *(f32 *)(this + 0xDC0);
v[2] = *(f32 *)(this + 0xDC4);
f32 z = *(f32 *)(this + 0xB4) - 1.0f;
spread += 16.0f;
fn_801C0BD0(&mtx, *(f32 *)(this + 0xAC), spread, z);
fn_801C0C60(&scale, v[0], v[1], v[2]);
fn_801C0540(&mtx, &scale, &mtx);
u16 yaw = *(u16 *)(this + 0x102); fn_8016EDD0(&mtx, &yaw);
fn_801C0BD0(&tmp, 0.0f, 8.0f, 0.0f);  fn_801C0540(&mtx, &tmp, &mtx);
u16 pitch = *(u16 *)(this + 0x100);   fn_8016ECB0(&mtx, &pitch);
fn_801C0BD0(&tmp, 0.0f, -8.0f, 0.0f); fn_801C0540(&mtx, &tmp, &mtx);
fn_8016A170(this + 0x58C, &mtx);
fn_8016A1A0(this + 0x58C, 0);
(*(u32 *)(this + 0x58C))->vt[0x14](this + 0x58C);

// 0x80AAC7C8: +0x60C from 0xDD4/0xDD8/0xDDC and 0xDC8/0xDCC/0xDD0
v[0] = *(f32 *)(this + 0xDD4);
v[1] = *(f32 *)(this + 0xDD8);
v[2] = *(f32 *)(this + 0xDDC);
fn_801C0BD0(&mtx, *(f32 *)(this + 0xDC8),
                  *(f32 *)(this + 0xDCC),
                  *(f32 *)(this + 0xDD0));
u16 yaw = *(u16 *)(this + 0x102); fn_8016EDD0(&mtx, &yaw);
fn_8016A170(this + 0x60C, &mtx);
fn_8016A150(this + 0x60C, &v[0]);
fn_8016A1A0(this + 0x60C, 0);
(*(u32 *)(this + 0x60C))->vt[0x14](this + 0x60C);
```

### Ambiguity

None in-REL. The `+0x58C` slot omits `0x8016A150`; the `+0x5CC` and `+0x60C`
slots include it. That asymmetry is literal (`0x80AAC7A8` has only the
identity call).

---

## 12. `0x80AA8470` - node-list contact dispatch and hard-hit effect

* **TSV size:** 592 bytes
* **Reachable body:** 148 instructions; ends at `0x80AA86BC blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa8470_FUN_80aa8470.c`
* **Proposed signature:** `pub unsafe fn fn_80aa8470(this: usize)`
* **Confidence:** **medium / conditional** (one external float constant)

### Frame

`stwu r1,-0x60(r1)`; LR `+0x64`; `f31/f30/f29` as `stfd`+`psq_st` pairs at
`+0x50/+0x58`, `+0x40/+0x48`, `+0x30/+0x38`; `r31@+0x2C`, `r30@+0x28`,
`r29@+0x24`, `r28@+0x20`. `r28 = this`, `r29 = *(u32 *)(this + 0x878)`,
`r30` = current node's object, `r31 = 0x80B2F590`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2F6F4` | `f32` | `0.2815` |
| `0x80B2F6F8` | `f32` | `0.8125` |
| `0x80B2F6A0` | `f32` | `0.0` |
| `0x8042A310` | `f32` | **(ext)** added to `f29` before the vtable call |
| `0x80B53974` | `char[]` | `"Wm_mr_hardhit"` |
| `0x80429D5C` | `u32` ptr | **(ext)** |

### Behaviour

```c
f29 = *(f32 *)0x80B2F6F4;               // 0.2815f, lfs 0x164(r31), r31=0x80B2F590
f30 = *(f32 *)0x8042A310;               // (ext), lfs -0x5CF0(r4)
f31 = *(f32 *)0x80B2F6F8;               // 0.8125f, lfs 0x168(r31)
for (node = *(u32 *)(this + 0x878); node != 0; node = *(u32 *)(node + 0x0C)) {
    obj = *(u32 *)(node + 0x04);
    u8 cls = *(u8 *)(obj + 0x38C);
    if (cls == 1 || cls == 2) {
        if (fn_80056CF0(obj, 0x1C) != 0) {           // 0x80AA84E8
            if (*(u8 *)(obj + 0x38C) == 1 && fn_80056BD0(obj) != 0) {
                fn_8008F9F0(0x80B53974, 0, this + 0xAC, 0, 0);
                fn_80096710(this, obj);
                fn_80096700(this, obj);
                fn_8006CED0(&local[0x10], this);
                fn_8006A4B0(&local[8], &local[0x10]);
                fn_80197F00(*(u32 *)0x80429D5C, 0x160, &local[8], 0);
                fn_80AA7C50(this, obj);
            } else {
                fn_80AA82F0(this, obj);
            }
            return;                                  // b 0x80AA8688
        }
        if (fn_80056CF0(obj, 0x1F) != 0) {           // 0x80AA85A0
            fn_80AA8170(this, obj);
            continue;                                // b 0x80AA867C
        }
        if (*(u8 *)(obj + 0x38C) == 1 && fn_80056BD0(obj) != 0) {
            f1 = (f29 + f30) * f31;                  // 0x80AA85D8..0x80AA85F0
            f2 = 0.0f;                               // 0x80B2F6A0
            // vtable +0x3FC of *(obj+0x60):
            //   r3 = obj, r4 = 1, r5 = 0, r6 = 0, f1, f2
            call5_f(*(u32 *)(*(u32 *)(obj + 0x60) + 0x3FC),
                    obj, 1, 0, 0, f1, f2);
            continue;
        }
        fn_80096710(this, obj);
        fn_80096910(this, obj);
        fn_80096700(this, obj);
        fn_80AA7C50(this, obj);
        return;                                      // b 0x80AA8688
    }
    if (cls == 3) {                                  // 0x80AA863C
        if (*(u16 *)(obj + 0x08) != 0x3C) {
            fn_80AA7C50(this, obj);
            return;
        }
        continue;
    }
    if (cls == 0) {                                  // 0x80AA8660
        call1(*(u32 *)(*(u32 *)(this + 0x60) + 0x224), this);
        continue;
    }
}
```

**Note on `f29`/`f31`:** the disassembly is `lfs f29,0x164(r31)` with
`r31 = 0x80B2F590`, so `f29 = *(f32 *)0x80B2F6F4 = 0.2815f`; `f31 =
*(f32 *)0x80B2F6F8 = 0.8125f`; `f30 = *(f32 *)0x8042A310` (ext). The table
above is authoritative. The vtable call site passes `(r3=obj, r4=1, r5=0,
r6=0)` plus `f1 = (f29 + f30) * f31` and `f2 = 0.0`; the callee is slot
`+0x3FC`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AA84E8` | `0x80056CF0` | `r3 = obj`, `r4 = 0x1C` |
| `0x80AA8504` | `0x80056BD0` | `r3 = obj` |
| `0x80AA8528` | `0x8008F9F0` | `r3 = 0x80B53974`, `r4 = 0`, `r5 = this+0xAC`, `r6 = 0`, `r7 = 0` |
| `0x80AA8534` | `0x80096710` | `r3 = this`, `r4 = obj` |
| `0x80AA8540` | `0x80096700` | `r3 = this`, `r4 = obj` |
| `0x80AA854C` | `0x8006CED0` | `r3 = &local[0x10]`, `r4 = this` |
| `0x80AA8560` | `0x8006A4B0` | `r3 = &local[8]`, `r4 = &local[0x10]` |
| `0x80AA8574` | `0x80197F00` | `r3 = *(0x80429D5C)`, `r4 = 0x160`, `r5 = &local[8]`, `r6 = 0` |
| `0x80AA8580` | `0x80AA7C50` | `r3 = this`, `r4 = obj` |
| `0x80AA8590` | `0x80AA82F0` | `r3 = this`, `r4 = obj` |
| `0x80AA85A0` | `0x80056CF0` | `r3 = obj`, `r4 = 0x1F` |
| `0x80AA85B4` | `0x80AA8170` | `r3 = this`, `r4 = obj` |
| `0x80AA85CC` | `0x80056BD0` | `r3 = obj` |
| `0x80AA8600` | vtable `+0x3FC` of `*(obj+0x60)` | `r3 = obj`, `r4 = 1`, `r5 = 0`, `r6 = 0`, `f1 = (0.2815+ext)*0.8125`, `f2 = 0.0` |
| `0x80AA8610` | `0x80096710` | `r3 = this`, `r4 = obj` |
| `0x80AA861C` | `0x80096910` | `r3 = this`, `r4 = obj` |
| `0x80AA8628` | `0x80096700` | `r3 = this`, `r4 = obj` |
| `0x80AA8634` | `0x80AA7C50` | `r3 = this`, `r4 = obj` |
| `0x80AA8658` | `0x80AA7C50` | `r3 = this`, `r4 = obj` |
| `0x80AA8678` | vtable `+0x224` of `*(this+0x60)` | `r3 = this` |

### Ambiguity

`0x8042A310` is a `main.dol` float folded into the vtable call's `f1`; its
value is not in this REL. `0x80B53974` is an in-REL string. `0x80096710` and
`0x80096910` are not in the ledger, but their argument registers are literal.
The row is **conditional** only on reading `0x8042A310` (and the pointer at
`0x80429D5C`).

---

## Cross-row notes

1. **No GQR dependence.** Every `psq_st`/`psq_l` in these twelve bodies is an
   `f31`/`f30`/`f29` stack spill with `qr0`; none reads `GQR1..GQR7`.
2. **No jump tables.** Every branch is a direct `b`/`bc`; the state machines
   are compare/`beq` chains, not `bctr` tables.
3. **`blr`/tail-call termination.** All twelve reach a final `blr` at the
   address listed. The two `b` instructions that look like tails
   (`0x80AAC9DC` in #3 and `0x80AA8680` in #12) are intra-body loop entries,
   not tails; the walker resolves them because their targets are not TSV
   entries.
4. **`bctrl` vs `bctr`.** The vtable sites are `bctrl` (`0x4E800421`, LK=1)
   even where a naive decoder prints `bctr`; the return address is set, so
   execution continues at the next instruction.
5. **`main.dol` operands.** `0x80352AF4`, `0x80352B34`, `0x80429D28`,
   `0x80429D5C`, `0x80429D60`, `0x80429DD0`, `0x80429E18`, `0x80429FC8`,
   `0x8042A038`, `0x8042A1C8`, `0x8042A310`, `0x80B70418`, `0x80B70498`,
   `0x80B75350`, `0x80B75390`, `0x80B76630`, `0x80377C48`,
   `0x80B781C8..0x80B781EC` are outside `d_enemiesNP.elf`. They are
   operands, not unresolved bases; only the four rows marked conditional need
   their contents for a byte-exact port.

## Verification of this lane

* Read-only: no Rust file, no `ledger/functions.csv` row, and no existing doc
  was modified. The only file written is this page.
* All twelve addresses are absent from the `address` column of
  `ledger/functions.csv` (re-checked after the wave-127 table was read).
* All twelve are present in
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and the TSV sizes in the
  selection table match that file exactly.
* Reachable instruction counts were produced by walking the stream from the
  entry, following conditional branches and linked calls, and stopping at
  `blr` or a branch to a TSV entry. The counts match the wave-127 column for
  every row.
* The ELF mapping used for every word is
  `file_offset = (address - 0x80A00000) + 0x1000`, cross-checked against the
  segment table (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`, and
  `0x80B32DC0 -> 0x133DC0`).
