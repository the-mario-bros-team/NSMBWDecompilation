# Wave 136 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses: 173 class **B** (reconstructible now), 160 class C and
51 class D.

Waves 128 - 135 (`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE135_ENEMIES.md`)
specified **96** of those B rows. Wave 135's window ran from 34 reachable
instructions down to 28 and ended at `0x80A1E100`.

This lane is the continuation: it starts at 28 reachable instructions - the tie
with wave 135's last row, broken by ascending address - and runs down to 22. It
specifies the twelve highest-ranked class-B rows that are still absent from
`ledger/functions.csv` and from every earlier `SPEC_WAVE*_ENEMIES.md` heading.
It is a **read-only** pass: it writes no Rust, no ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000`
segment shift of `docs/REL_ELF_MAPPING.md` (first `PT_LOAD` is
`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`), cross-checked against the Ghidra
export at the cited path and against
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Every constant that lives
inside the REL image was read out of the ELF itself, so the float values below
are the shipped values. Addresses that do not resolve in this REL are marked
**(ext)**; they are `main.dol` addresses and are described from the call site.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then by ascending address.

The metric was re-validated before this page was written: walking the twelve
wave-135 addresses reproduces `34, 33, 33, 32, 31, 31, 31, 31, 30, 29, 29, 28`
exactly, and walking all 96 addresses specified by waves 128-135 reproduces
every published count (287 down to 28). This page picks up inside the same
28-instruction band, at the first address after wave 135's last row, and runs
down to 22.

| # | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 1 | `0x80AB2620` | 112 | 28 | two pad-field resets, then `fn_80AB1780(1.0f, this, 1)` | **ready** |
| 2 | `0x80A0B8A0` | 108 | 27 | `0x83` class-list walk with a lookup cursor | **ready** |
| 3 | `0x80AB1890` | 104 | 26 | `s16` angle word to `+0x164 = v/4096`, `+0x7DC = -3v/4096` | **ready** |
| 4 | `0x80A6EFB0` | 100 | 25 | three-band altitude state machine, returns 0/1/2 | **conditional** |
| 5 | `0x80AD6CA0` | 100 | 25 | ten-argument `0x800DB180` builder + `0x800DAE80` reset | **ready** |
| 6 | `0x80AED210` | 100 | 25 | deleting destructor, `+0x18` sub-object, one dead guard | **ready** |
| 7 | `0x80B0D070` | 100 | 25 | pad-field initialiser, value `1` | **ready** |
| 8 | `0x80B0D0E0` | 100 | 25 | pad-field initialiser, value `3` | **ready** |
| 9 | `0x80AED1B0` | 96 | 24 | deleting destructor, `+0x18` sub-object, clean guard set | **ready** |
| 10 | `0x80A2F6D0` | 92 | 23 | `0x8015F800` release + `0x802B9280` free, one dead guard | **ready** |
| 11 | `0x80AFEC80` | 92 | 23 | instruction-identical twin of #10 (call displacements only) | **ready** |
| 12 | `0x80A2F670` | 88 | 22 | same teardown as #10 with a single guard | **ready** |

**Split: 11 ready, 1 conditional.** Total 1,192 bytes, 298 reachable
instructions.

The next band - 22 instructions, 88 bytes - has ten members. Three are already
translated ledger rows (`0x80A1F390`, `0x80A1FFB0`, `0x80ADD0E0`), this lane
takes the lowest unclaimed one (`0x80A2F670`), and the remaining seven
(`0x80A43DC0`, `0x80A446B0`, `0x80AECF00`, `0x80AEE6D0`, `0x80AEF910`,
`0x80AFEC20`, `0x80AFF4F0`) become wave 137's candidates.

"Conditional" is used exactly as in waves 128-135: one named datum must be
established before a byte-exact port. Rank 4 is the only case, and it is an
**ABI** datum rather than a memory word: its middle band ends in
`li r3,2; b 0x800B2EE0`, a tail call into a `void` stage setter, so the value
that path returns is the callee's leftover `r3` and is not defined by any
instruction of this body.

The four risk classes that made waves 110-126 skip a body are all absent:

* **no paired-single traffic at all.** Not one `psq_l`/`psq_st` in the tranche;
  no `GQR1..GQR7`, no `ldexpf` noise in any of the twelve exports.
* **no `bctr` jump table.** All 22 conditional branches target an address inside
  their own body (checked mechanically: zero out-of-body `bc` targets). There is
  no `bctrl` anywhere in the tranche.
* **no incoming float argument.** No body reads `f1`/`f2`/`f3`/`f4` before
  writing it; every float operand is loaded from the actor or from an in-REL
  constant, and the only float-carrying call site (#5) fills all five FPR
  arguments explicitly.
* **no unledgered in-REL callee landmine.** Every in-REL `bl` target
  (`0x80AB1780`, `0x80A0B8A0`) is a `functions.tsv` entry; the `main.dol`
  targets are `ledger/functions.csv` rows or the documented `0x800DB180` /
  `0x800DAE80` pair.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`). The image ends at `0x80B6E438`;
  anything at or above that is a `main.dol` address.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* Branch mnemonics are the C form after the exact `bc` decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 12,0` = `blt`, `bc 4,0` = `bge`,
  `bc 12,1` = `bgt`, `bc 4,1` = `ble`, `bc 20,x` = unconditional `b`.
  `bclr 20,0` is printed `blr`.
* `addi rX,r0,imm` is `li rX,imm` (`r0` is not read when `RA == 0`).
* `xoris rD,rS,UI` XORs the **upper half** of `rS` with `UI`, i.e. with
  `UI << 16`. `xoris r6,r4,0x8000` is therefore `r6 = r4 ^ 0x80000000`, not
  `r4 ^ 0x8000`. This single bit decides rank 3's result and is the one place
  where a hand decode of the export can go wrong.
* **Three-operand single-precision ops are not uniform.** `fadds`/`fsubs`/
  `fdivs` take their second source from instruction bits 11-15, while
  `fmuls`/`fmadds`/`fmsubs` take the multiplicand from bits 6-10 and the addend
  from bits 11-15. No row here uses a single-precision arithmetic op at all:
  rank 3's `fmul`/`fneg` are the double-precision forms (`opcode 59`) and only
  its `stfs` rounds to `f32`; rank 5's `fmr` moves are not arithmetic.
* **The `2^52` integer-to-double idiom** (`lis 0x4330` + `xoris` + `stw` pair +
  `lfd` + `fsub`) is read as an exact conversion, the convention used by
  `SPEC_WAVE129`, `SPEC_WAVE131`, `SPEC_WAVE133` and `SPEC_WAVE135`.
* All twelve bodies end in a single `blr` (rank 4 also has one tail `b`) and are
  exactly `TSV size / 4` instructions with no trailing unreachable words; the
  "ends at" address below is the address **of** that last instruction.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv` evidence column) or the helper
tables of waves 128-135, not new claims.

| address | role |
|---|---|
| `0x800DAE80` | reset/stop sweep over `0x60`-byte motion records **(ext, unexported)** |
| `0x800DB180` | ten-argument builder: `f1..f5` + `r3..r7` (`COVERAGE_WAVE133_BASES.md`; host helper `call_f32x5_u32x5` already exists) **(ext, unexported)** |
| `0x800B2EE0` | `fn_800b2ee0` - `void` stage setter, duplicate of `0x800B2ED0`; `FUN_80161460(r13-0x573C, arg)` |
| `0x8015F800` | `state_descriptor_release(this, 0)` - free only when `free > 0` |
| `0x8015F9F0` | `state_object_release(this, 0)` - same shape as `0x8015F800` |
| `0x80162D50` | `fn_80162d50(key, cursor)` - class-list lookup in `0x80377A10`, returns `*(node+0x10)` |
| `0x80166340` | `store_byte(obj, value, idx)` - `*(u8 *)(*(obj+0x28) + idx*0x38 + 0x34) = value` |
| `0x80166F40` | same byte store as `0x80166340` (duplicate symbol) |
| `0x802B9280` | `arc_heap_free_object(this)` - tail call `0x802B8F70(block, 0)` |
| `0x80AB1780` | in-REL: builds the `+0x704` model matrix from `(*(void **)(0x80B54C90))[idx]`, sets `+0x98C = idx`, takes an incoming `f1` |

---

## 1. `0x80AB2620` - pad-field reset then `fn_80AB1780(1.0f, this, 1)`

* **TSV size:** 112 bytes
* **Reachable body:** 28 instructions; 2 `bl`; ends at `0x80AB268C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab2620_FUN_80ab2620.c`
* **Proposed signature:** `pub unsafe fn fn_80ab2620(this: usize)`
* **Callers in this REL:** none by direct `bl`/`b`; the body is reachable only
  through the vtable word at `0x80B55010` (cluster
  `0x80B55000: 0xFFFFFFFF, 0x80AB26A0, 0, 0xFFFFFFFF, 0x80AB2620, 0, ...`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`. `r31 = this`. No float
temporaries; the whole frame exists only for the two saved words.

Exact order: `mflr r0`, `li r4,3`, `li r5,0`, `stw r0,0x14(r1)`,
`stw r31,0x0C(r1)`, `mr r31,r3` (`or r31,r3,r3`), `addi r3,r3,0x73C`,
`bl 0x80166340`.

### Constants

| address | type | value |
|---|---:|---|
| `0x80B2FAF0` | `f32` | `1.0` (loaded `lfs f1,-1296(r4)` with `r4 = 0x80B30000`) |

No other literal: the two class values are `3` and the index is `0` / `1`.

### Behaviour

```c
void fn_80ab2620(usize this) {
    store_byte(this + 0x73C, 3, 0);          // 0x80AB2640, r3 = this+0x73C, r4 = 3, r5 = 0
    if (*(u32 *)(this + 0x988) == 1) {       // 0x80AB2644 lwz r0,0x988(r31); 0x80AB2648 cmpwi r0,1
        store_byte(this + 0x794, 3, 0);      // 0x80AB265C, taken only on == 1 (0x80AB264C bne skips)
    }
    fn_80ab1780(1.0f, this, 1);              // 0x80AB2670, f1 = [0x80B2FAF0], r3 = this, r4 = 1
    *(u16 *)(this + 0x104) = 0;              // 0x80AB2678 sth r0,0x104(r31) with r0 = li 0
}
```

The function writes no return value: `r3` at `blr` is whatever `0x80AB1780`
left behind, which is why Ghidra types it `void`. Keep the `-> ()` signature.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AB2640` | `0x80166340` | `r3 = this+0x73C`, `r4 = 3`, `r5 = 0` |
| `0x80AB265C` | `0x80166340` | `r3 = this+0x794`, `r4 = 3`, `r5 = 0` (guarded by `+0x988 == 1`) |
| `0x80AB2670` | `0x80AB1780` | `f1 = 1.0f`, `r3 = this`, `r4 = 1` |

### Ambiguity

`0x80AB1780` is itself a class-C row of wave 127 (it reads an incoming `f1`).
That classification does not block this caller: the argument is written
explicitly at `0x80AB2668` from the in-REL constant `0x80B2FAF0`, so the callee's
typing problem stays with the callee. Ghidra's `(double)DAT_80b2faf0` is the
same `lfs`, not a double load.

`+0x988` is a 32-bit word compared against `1`, not a byte: `lwz r0,0x988(r31)`
+ `cmpwi r0,1`, so the guard is `== 1` exactly.

---

## 2. `0x80A0B8A0` - `0x83` class-list walk with a lookup cursor

* **TSV size:** 108 bytes
* **Reachable body:** 27 instructions; 2 `bl` (one of them recursive); ends at
  `0x80A0B908 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a0b8a0_FUN_80a0b8a0.c`
* **Proposed signature:** `pub unsafe fn fn_80a0b8a0(this: usize, cursor: usize) -> usize`
* **Callers in this REL:** `0x80A0BDE4` (inside `fn_80a0bd20`, rank 11 of
  `SPEC_WAVE129_ENEMIES.md`, which already writes the call as
  `fn_80A0B8A0(this, 0)`) and `0x80A0B8F4` (its own recursion)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`. `r31 = this`. Nothing is
spilled except over the two calls.

### Constants

None. The lookup key is the literal `131` (`li r3,131`, i.e. `0x83`).

### Behaviour

```c
usize fn_80a0b8a0(usize this, usize cursor) {
    usize found = fn_80162d50(0x83, cursor);       // 0x80A0B8B8; r4 = cursor, untouched by the prologue
    if (found == 0)                                // 0x80A0B8BC cmpwi r3,0 / 0x80A0B8C0 bne
        return 0;                                  // 0x80A0B8C4 li r3,0 / 0x80A0B8C8 b -> epilogue
    if (*(u32 *)(this  + 0x754) == *(u32 *)(found + 0x754)   // 0x80A0B8D8 cmpw, 0x80A0B8DC bne
     && *(u32 *)(found + 0x758) != 0)                        // 0x80A0B8E4 cmpwi, 0x80A0B8E8 beq
        return found;                              // 0x80A0B8EC b -> epilogue
    return fn_80a0b8a0(this, found);               // 0x80A0B8F0 mr r3,this; 0x80A0B8F4 bl (r4 = found)
}
```

The `r4 = found` at the recursive `bl` is not an accident: `or r4,r3,r3` at
`0x80A0B8D0` runs while `r3` still holds the lookup result, and `r4` is not
rewritten afterwards. The recursion therefore advances the lookup cursor, which
is what makes the walk terminate.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A0B8B8` | `0x80162D50` | `r3 = 0x83`, `r4 = cursor` (the incoming second argument) |
| `0x80A0B8F4` | `0x80A0B8A0` | `r3 = this`, `r4 = found` |

### Ambiguity

Wave 127 summarised this body as "tail recursion on `r3` only (`b 0x80A0B8A0`)";
the stream shows a plain `bl` at `0x80A0B8F4` (raw word `0x4BFFFFAD`), followed
by the shared epilogue at `0x80A0B8F8`. It also shows that `r4` **is** consumed:
`0x80162D50` takes `(key, cursor)` and adds `0x10` to a non-null cursor before
the list search, so a one-parameter port would change the first call's
behaviour. That is why the signature above keeps two parameters - the same arity
wave 129's call site already uses.

Callers that pass `cursor = 0` (wave 129's `0x80A0BDE4`) and the recursive
`cursor = found` are the only two known entry states; both are exact.

---

## 3. `0x80AB1890` - `s16` angle word to two speed fields

* **TSV size:** 104 bytes
* **Reachable body:** 26 instructions; no calls; ends at `0x80AB18F4 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab1890_FUN_80ab1890.c`
* **Proposed signature:** `pub unsafe fn fn_80ab1890(this: usize)`
* **Callers in this REL:** `0x80AB0720` (inside `fn_80ab0570`) and `0x80AB2258`
  (inside `fn_80ab21d0`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x20(r1)`, no LR save (leaf), no GPR saves. Two 8-byte stack
temporaries: a double at `+0x08..+0x0F` and a second double at `+0x10..+0x17`.
No float register is live across a call because there is no call.

### Constants

| address | type | value | notes |
|---|---:|---|---|
| `0x80B2FB38` | `f32` | `-0.000244140625` = `-2^-12` | `lfs f2,-1224(r5)` |
| `0x80B2FB3C` | `f32` | `0.000732421875` = `3 * 2^-12` | `lfs f0,-1220(r4)` |
| `0x80B2FB40` | `f64` | `0x4330000080000000` = `2^52 + 2^31` | `lfd f4,-1216(r7)` |
| `0x43300000` | imm | high word of the `2^52` bias | `lis r0,0x4330` |

### Behaviour

```c
void fn_80ab1890(usize this) {
    i32 v = *(i16 *)(this + 0x102);            // 0x80AB18A0 lha r4,0x102(r3)  (sign-extended)
    u32 w = (u32)v ^ 0x80000000;               // 0x80AB18A8 xoris r6,r4,0x8000  (UI is the HIGH half)
    f64 d1 = (f64)bits(0x43300000, w) - 2.0 * 1073741824.0;   // 0x80AB18BC lfd / 0x80AB18C4 fsub
    f64 d2 = (f64)bits(0x43300000, w) - 2.0 * 1073741824.0;   // 0x80AB18D0 lfd / 0x80AB18D8 fsub
    *(f32 *)(this + 0x164) = (f32)( 0.000244140625 * d2);      // 0x80AB18D4 fmul(-2^-12), 0x80AB18DC fneg, 0x80AB18E4 stfs
    *(f32 *)(this + 0x7DC) = (f32)(-0.000732421875 * d1);      // 0x80AB18E0 fmul(3*2^-12), 0x80AB18E8 fneg, 0x80AB18EC stfs
}
```

Both `fsub`s subtract the same loaded double, so `d1 == d2 == (f64)v`, exactly,
for every `s16` value including `-32768`. The two stores are then, exactly:

```
[this+0x164] = (f32)( 0.000244140625 * v)   == (f32)( v / 4096.0)
[this+0x7DC] = (f32)(-0.000732421875 * v)   == (f32)(-3v / 4096.0)
```

The `fmul`/`fneg` are double-precision (`opcode 59`); only the `stfs` rounds to
`f32`. A port must therefore compute in `f64` and cast once at the store; doing
the multiply in `f32` changes the last bit for large `v`.

Building the same `w` twice (two independent `stw` pairs at `0x08` and `0x10`)
is what the compiler emitted; the values are identical. A port can use one
temporary - the stores are the only observable effect.

### Call sites

None.

### Ambiguity

Ghidra prints `uVar2 = (int)*(short *)(param_1 + 0x102) ^ 0x80000000` and then
reuses `uVar2` for both conversions, which matches the stream. Its
`(double)CONCAT44(0x43300000,uVar2)` is the `lfd` of the stack pair, not a
register pair. Field `+0x102` is the only input; `+0x164` and `+0x7DC` the only
outputs. No ambiguity.

---

## 4. `0x80A6EFB0` - three-band altitude state machine, returns 0/1/2

* **TSV size:** 100 bytes
* **Reachable body:** 25 instructions; 1 tail `b`; ends at `0x80A6F010 blr`
  (the other path ends at `0x80A6F000 b 0x800B2EE0`)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a6efb0_FUN_80a6efb0.c`
  (truncated at `halt_baddata`; the decoder `cror` is the missing instruction)
* **Proposed signature:** `pub unsafe fn fn_80a6efb0(this: usize) -> i32`
* **Callers in this REL:** `0x80A6F1B4` (inside the unlisted body at
  `0x80A6F1A0`, which stores the result twice and indexes two 3-entry tables)
* **Verdict:** **conditional** (confidence high on the control flow, one ABI
  datum open)

### Frame

None. Leaf, no stack frame, no saved registers, no LR save. `r4` is the only
scratch register; `f0`, `f1`, `f2` are the only FPRs.

### Constants

| address | type | value | notes |
|---|---:|---|---|
| `0x80B2DA60` | `f32` | `56.0` | `lfs f0,-9632(r4)` with `r4 = 0x80B30000`, low band edge |
| `0x80B2DA64` | `f32` | `120.0` | `lfs f0,-9628(r4)`, high band edge |

Consumer context (one level up, `0x80A6F1A0`, not part of this body): the
returned index multiplies by 4 and indexes `0x80B2DA68 = {2.0f, 16.0f, 32.0f}`
(loaded `lfsx`) and the adjacent word table at `0x80B2DA70 = {0x42000000,
0x00000000, 0x3F19999A}` = `{32.0f, 0.0f, 0.6f}`, stored to `+0xEC` and `+0x84C`.
That confirms the contract: the return value is a 0..2 index, never 3.

### Behaviour

```c
i32 fn_80a6efb0(usize this) {
    f64 d = (f64)*(f32 *)(this + 0xB0) - (f64)*(f32 *)(this + 0x850);  // 0x80A6EFB0/EFB8 lfs, EFC0 fsub
    if (d < 56.0) {                       // 0x80A6EFC4 fcmpo cr0,f1,56.0; 0x80A6EFC8 bge skips this block
        *(u32 *)(this + 0x860) = 0;       // 0x80A6EFCC li r0,0 / 0x80A6EFD0 stw
        return 1;                         // 0x80A6EFD4 li r3,1 / 0x80A6EFD8 blr
    }
    cror 2,1,2;                           // 0x80A6EFDC: CR2[EQ] = CR0[GT] | CR0[EQ]  (i.e. d >= 56)
    if (!CR2[EQ]) goto else_;             // 0x80A6EFE0 bne on the EQ bit - taken only for NaN
    if (!(d < 120.0)) goto else_;         // 0x80A6EFE8 lfs 120.0; 0x80A6EFEC fcmpo; 0x80A6EFF0 bge
    *(u32 *)(this + 0x860) = 1;           // 0x80A6EFF4 li r0,1 / 0x80A6EFF8 stw
    return fn_800b2ee0(2);                // 0x80A6EFFC li r3,2 / 0x80A6F000 b 0x800B2EE0
else_:
    *(u32 *)(this + 0x860) = 2;           // 0x80A6F004 li r0,2 / 0x80A6F008 stw
    return 0;                             // 0x80A6F00C li r3,0 / 0x80A6F010 blr
}
```

Bands, for all non-NaN `d`:

| `d` | `+0x860` | return |
|---|---:|---|
| `d < 56.0` | `0` | `1` |
| `56.0 <= d < 120.0` | `1` | tail call, see below |
| `d >= 120.0` | `2` | `0` |
| NaN | `2` | `0` |

The `fsub` is a **double** subtraction of two exact `f32` values, so the port must
compare in `f64` (`(*p as f64) - (*q as f64)`); subtracting in `f32` can round and
flip a band edge.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A6F000` | `0x800B2EE0` | `r3 = 2`, tail call (`b`), no return value defined |

### Ambiguity

`0x800B2EE0` is `fn_800b2ee0`, `void fn_800b2ee0(undefined4)`
(`ledger/functions.csv`, `game::stage`, `translated`): it forwards its single
argument to `0x80161460(r13-0x573C, arg)`. The middle band therefore writes the
state word and then **tail-calls a void function**. Whatever `r3` holds at that
point becomes this function's return value; no instruction in this body defines
it. Two defensible readings:

1. the source was `return 2;` after the setter call - then the port should be
   `fn_800b2ee0(2); return 2;`;
2. the source called a void setter on the "return" path and the function is
   effectively `void` on that path - then the port should reproduce
   `fn_800b2ee0(2)` and return the callee's leftover `r3`.

Nothing inside `d_enemiesNP` distinguishes them. The consumer at `0x80A6F1A0`
stores the result at `+0x85C` and indexes the two 3-entry tables above with it,
which means a real build *did* receive `0..2` - so reading 1 is the one to
verify. That verification is the single bounded step; the other 24 instructions
are exact.

Wave 127 described this body as "returning 0/1/2" and read the `halt_baddata` as
the `cror`. The `cror` reading is correct; the "returns 2" part is the point that
needs the step above, because `2` is written to `r3` as the **argument** of the
tail call.

---

## 5. `0x80AD6CA0` - ten-argument `0x800DB180` builder plus reset

* **TSV size:** 100 bytes
* **Reachable body:** 25 instructions; 2 `bl`; ends at `0x80AD6D00 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad6ca0_FUN_80ad6ca0.c`
* **Proposed signature:** `pub unsafe fn fn_80ad6ca0(this: usize)`
* **Callers in this REL:** none by direct `bl`/`b`; reached through the vtable
  word at `0x80B5D72C` (cluster
  `0x80B5D720: 0x80AD7820, 0x80AD5980, 0x80AD6C00, 0x80AD6CA0, ...`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`. `r31 = this`. All five
float arguments are loaded from memory and copied through `fmr`; no FPR is live
across the first call except the argument registers themselves.

### Constants

| address | type | value | notes |
|---|---:|---|---|
| `0x80B309C4` | `f32` | `0.0` | `lfs f1,2500(r4)` with `r4 = 0x80B30000` |
| `0x80B309C8` | `f32` | `-22.0` | `lfs f5,2504(r4)`, fifth float argument |

The five integer arguments are `r3 = this+0x4CD4`, `r4 = this`, `r5 = 0`,
`r6 = 1`, `r7 = 0`.

### Behaviour

```c
void fn_80ad6ca0(usize this) {
    f32 zero  = *(f32 *)0x80B309C4;             // 0x80AD6CB0 lfs f1,2500(r4)
    f32 neg22 = *(f32 *)0x80B309C8;             // 0x80AD6CC4 lfs f5,2504(r4)
    fn_800db180(zero, zero, zero, zero, neg22,  // f1..f5; 0x80AD6CC0/CD0/CD8 fmr copy f1 into f2/f3/f4
                this + 0x4CD4,                  // r3  (0x80AD6CE0 addi r3,r3,19668)
                this,                           // r4  (0x80AD6CD4 mr r4,r31)
                0, 1, 0);                       // r5, r6, r7
    fn_800dae80(this + 0x4CD4);                 // 0x80AD6CE8 addi r3,r31,0x4CD4; 0x80AD6CEC bl
}
```

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AD6CE4` | `0x800DB180` | `f1..f4 = 0.0f`, `f5 = -22.0f`, `r3 = this+0x4CD4`, `r4 = this`, `r5 = 0`, `r6 = 1`, `r7 = 0` |
| `0x80AD6CEC` | `0x800DAE80` | `r3 = this+0x4CD4` |

### Ambiguity

Both callees are `main.dol` and unexported, but their contracts are already
documented: `0x800DB180` is the ten-argument builder (`f1..f5` plus `r3..r7`)
used by several `d_basesNP` bodies (`docs/COVERAGE_WAVE130_BASES.md`,
`docs/COVERAGE_WAVE133_BASES.md`), with the host-side helper
`call_f32x5_u32x5` already present in the port; `0x800DAE80` is the `0x60`-byte
motion-record reset. Every register is set by this body, so the emitted call is
determined even though the callee body is not in this REL.

Ghidra's `(double)DAT_80b309c4` / `(double)DAT_80b309c8` are two `lfs` loads
promoted to double for printing; the arguments are `f32`.

---

## 6. `0x80AED210` - deleting destructor, `+0x18` sub-object, one dead guard

* **TSV size:** 100 bytes
* **Reachable body:** 25 instructions; 4 `bc`; 2 `bl`; ends at `0x80AED270 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aed210_FUN_80aed210.c`
* **Proposed signature:** `pub unsafe fn fn_80aed210(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; reached through the vtable
  word at `0x80B61A90` (cluster `0x80B61A84: 0, 0, 0x80AED210, 0x80AED9A0, ...`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C` (= `free_count`); `r30` at
`+0x08` (= `this`). `r3` and `r4` are copied to `r30`/`r31` before any branch.

### Constants

None. The sub-object displacement is the literal `0x18` folded into
`addic. r3,r3,0x18`.

### Behaviour

```c
usize fn_80aed210(usize this, i32 free_count) {
    if (this != 0) {                       // 0x80AED218 cmpwi r3,0; 0x80AED230 beq -> return
        // 0x80AED234 beq 0x80AED248 re-tests the same CR0 with no compare in between: never taken
        u32 sub = (u32)(this + 0x18);      // 0x80AED238 addic. r3,r3,0x18 (computes AND records CR0)
        if (sub != 0)                      // 0x80AED23C beq 0x80AED248
            fn_8015f9f0(this + 0x18, 0);   // 0x80AED240 li r4,0; 0x80AED244 bl
        if (free_count > 0)                // 0x80AED248 cmpwi r31,0 (signed); 0x80AED24C ble
            fn_802b9280(this);             // 0x80AED250 mr r3,r30; 0x80AED254 bl
    }
    return this;                           // 0x80AED258 mr r3,r30, then the epilogue
}
```

`addic.` is not a dead guard: it *computes* `this+0x18` into `r3`, which is the
pointer passed to `0x8015F9F0`, and its recorded CR0 is the null test on that
same pointer. Both effects must be kept.

The second `beq` (`0x80AED234`) re-tests CR0 with no compare in between, so it
is never taken. It is part of the 100-byte body and must be reproduced to stay
byte-identical; see #10/#11 for the same idiom.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AED244` | `0x8015F9F0` | `r3 = this+0x18`, `r4 = 0` |
| `0x80AED254` | `0x802B9280` | `r3 = this` |

### Ambiguity

None in the body. The `r31` compare is signed (`cmpwi` + `ble`), so the second
argument is a signed count, not a flag; the same callee pair is used by
`0x80A2F670`/`0x80A2F6D0`/`0x80AFEC80` below.

---

## 7. `0x80B0D070` - pad-field initialiser, value `1`

* **TSV size:** 100 bytes
* **Reachable body:** 25 instructions; 1 `bl`; ends at `0x80B0D0D0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0d070_FUN_80b0d070.c`
* **Proposed signature:** `pub unsafe fn fn_80b0d070(this: usize)`
* **Callers in this REL:** `0x80B0DBF4` (inside `fn_80b0db90`, rank 83 of
  `SPEC_WAVE134_ENEMIES.md`, which calls it as `fn_80B0D070(this)`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31` at `+0x0C`; `r30` at `+0x08`.
`r31 = 1` (the value, loaded once by `li r31,1`) and `r30 = this`.

### Constants

None. The four field values are the literal `1`; the two sub-object
displacements are `+0x6B4` and `+0x6E0`, the class value `3`, the index `0`.

### Behaviour

```c
void fn_80b0d070(usize this) {
    *(u8 *)(this + 0x674) = 1;              // 0x80B0D094 stb r31,1652(r3)   (r3 still == this)
    store_byte(this + 0x6B4, 1, 0);         // 0x80B0D098 addi r3,r3,1716; 0x80B0D09C bl 0x80166F40
    *(u8 *)(this + 0x770) = 1;              // 0x80B0D0A0 stb r31,1904(r30)
    *(u8 *)(this + 0x6AC) = 1;              // 0x80B0D0B0 stb r31,1708(r30)
    store_byte(this + 0x6E0, 1, 0);         // 0x80B0D0A4 addi r3,r30,1760; 0x80B0D0B4 bl
    *(u8 *)(this + 0x7A8) = 1;              // 0x80B0D0B8 stb r31,1960(r30)
}
```

The exact store/call interleaving is: `stb +0x674`, `addi r3,this+0x6B4`,
`bl`, `stb +0x770`, `addi r3,this+0x6E0`, `li r4,1`, `li r5,0`, `stb +0x6AC`,
`bl`, `stb +0x7A8`. Keeping that order reproduces the machine code; no store
aliases the byte the calls write, so the order is otherwise free.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80B0D09C` | `0x80166F40` | `r3 = this+0x6B4`, `r4 = 1`, `r5 = 0` |
| `0x80B0D0B4` | `0x80166F40` | `r3 = this+0x6E0`, `r4 = 1`, `r5 = 0` |

### Ambiguity

None. `SPEC_WAVE134_ENEMIES.md` already names this entry as the sibling that
sets `+0x674`/`+0x770`/`+0x6AC`/`+0x7A8`; this page gives it a rank and a body.

---

## 8. `0x80B0D0E0` - pad-field initialiser, value `3`

* **TSV size:** 100 bytes
* **Reachable body:** 25 instructions; 1 `bl`; ends at `0x80B0D140 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0d0e0_FUN_80b0d0e0.c`
* **Proposed signature:** `pub unsafe fn fn_80b0d0e0(this: usize)`
* **Callers in this REL:** `0x80B0DDDC` (inside `fn_80b0dd80`, rank 94 of
  `SPEC_WAVE135_ENEMIES.md`, which calls it as `fn_80B0D0E0(this)`)
* **Verdict:** **ready** (confidence high)

### Frame

Identical to #7: `stwu r1,-0x10(r1)`; LR `+0x14`; `r31` `+0x0C`; `r30` `+0x08`.
`r31 = 3`, `r30 = this`.

### Constants

None beyond the literal `3` written to the four fields and passed to both
`0x80166F40` calls.

### Behaviour

```c
void fn_80b0d0e0(usize this) {
    *(u8 *)(this + 0x674) = 3;              // 0x80B0D104 stb r31,1652(r3)
    store_byte(this + 0x6B4, 3, 0);         // 0x80B0D108 addi r3,r3,1716; 0x80B0D10C bl
    *(u8 *)(this + 0x770) = 3;              // 0x80B0D110 stb r31,1904(r30)
    *(u8 *)(this + 0x6AC) = 3;              // 0x80B0D120 stb r31,1708(r30)
    store_byte(this + 0x6E0, 3, 0);         // 0x80B0D114 addi r3,r30,1760; 0x80B0D124 bl
    *(u8 *)(this + 0x7A8) = 3;              // 0x80B0D128 stb r31,1960(r30)
}
```

#7 and #8 are the same body with the literal `1` replaced by `3`; the offsets,
call order and register assignment are identical (`li r31,3` at `0x80B0D0F8`).

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80B0D10C` | `0x80166F40` | `r3 = this+0x6B4`, `r4 = 3`, `r5 = 0` |
| `0x80B0D124` | `0x80166F40` | `r3 = this+0x6E0`, `r4 = 3`, `r5 = 0` |

### Ambiguity

None. `SPEC_WAVE135_ENEMIES.md` lists this entry in its helper table as "writes
`3` to `+0x674/+0x770/+0x6AC/+0x7A8`, two `0x80166F40`"; the stream confirms it
exactly, including the class value `3` passed to the two calls.

---

## 9. `0x80AED1B0` - deleting destructor, `+0x18` sub-object, clean guard set

* **TSV size:** 96 bytes
* **Reachable body:** 24 instructions; 3 `bc`; 2 `bl`; ends at `0x80AED20C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aed1b0_FUN_80aed1b0.c`
* **Proposed signature:** `pub unsafe fn fn_80aed1b0(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; reached through the vtable
  word at `0x80B61AC0` (cluster `0x80B61AB4: 0, 0, 0x80AED1B0, 0x80AED9A0, ...`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31` `+0x0C` (= `free_count`); `r30` `+0x08`
(= `this`).

### Constants

None.

### Behaviour

```c
usize fn_80aed1b0(usize this, i32 free_count) {
    if (this != 0) {                       // 0x80AED1B8 cmpwi r3,0; 0x80AED1D0 beq -> return
        r3 = this + 0x18;                  // 0x80AED1D4 addic. r3,r3,0x18
        if (r3 != 0)                       // 0x80AED1D8 beq 0x80AED1E4
            fn_8015f9f0(this + 0x18, 0);   // 0x80AED1DC li r4,0; 0x80AED1E0 bl
        if (free_count > 0)                // 0x80AED1E4 cmpwi r31,0; 0x80AED1E8 ble
            fn_802b9280(this);             // 0x80AED1EC mr r3,r30; 0x80AED1F0 bl
    }
    return this;                           // 0x80AED1F4 mr r3,r30, epilogue
}
```

This is #6 with the redundant `beq` removed - one instruction shorter, same two
callees, same three live branches. Wave 127 described it as "teardown of the
`+0x18` sub-object via `0x8015F9F0` then `0x802B9280`", which matches.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AED1E0` | `0x8015F9F0` | `r3 = this+0x18`, `r4 = 0` |
| `0x80AED1F0` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. The only difference from `0x80AED210` is the absent redundant branch;
a port of one is a port of the other minus one `if`.

---

## 10. `0x80A2F6D0` - `0x8015F800` release plus `0x802B9280` free

* **TSV size:** 92 bytes
* **Reachable body:** 23 instructions; 3 `bc`; 2 `bl`; ends at `0x80A2F728 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2f6d0_FUN_80a2f6d0.c`
* **Proposed signature:** `pub unsafe fn fn_80a2f6d0(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; reached through the word
  table at `0x80B3C728` (region `0x80B3C718: "Fall", 0, 0, 0, 0x80A2F6D0,
  0x8015F840, 0x8015F850, 0x8015F8C0`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31` `+0x0C` (= `free_count`); `r30` `+0x08`
(= `this`).

### Constants

None.

### Behaviour

```c
usize fn_80a2f6d0(usize this, i32 free_count) {
    if (this != 0) {                       // 0x80A2F6D8 cmpwi r3,0; 0x80A2F6F0 beq -> return
        if (this == 0) goto after_rel;     // 0x80A2F6F4 beq 0x80A2F700 - same CR0, never taken
        fn_8015f800(this, 0);              // 0x80A2F6F8 li r4,0; 0x80A2F6FC bl
after_rel:
        if (free_count > 0)                // 0x80A2F700 cmpwi r31,0; 0x80A2F704 ble
            fn_802b9280(this);             // 0x80A2F708 mr r3,r30; 0x80A2F70C bl
    }
    return this;                           // 0x80A2F710 mr r3,r30, epilogue
}
```

The dead branch at `0x80A2F6F4` is a second `beq` on the **same** CR0 the
`cmpwi r3,0` produced. Because the first `beq` already left the block when
`r3 == 0`, the second one can never be taken. It is the single extra word that
makes this body 92 bytes / 23 instructions rather than 88 / 22 (compare #12,
which is the same body without it).

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A2F6FC` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80A2F70C` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. `0x8015F800` is `state_descriptor_release` in the ledger, whose own note
records the same "free is printed with no argument; `this` is passed in `r3`"
quirk, so the call site above is the authoritative reading.

---

## 11. `0x80AFEC80` - instruction-identical twin of #10

* **TSV size:** 92 bytes
* **Reachable body:** 23 instructions; 3 `bc`; 2 `bl`; ends at `0x80AFECD8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80afec80_FUN_80afec80.c`
* **Proposed signature:** `pub unsafe fn fn_80afec80(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; reached through the word
  table at `0x80B655F0` (region `0x80B655E0: "ID_I", "ce\0\0", 0, 0,
  0x80AFEC80, 0x8015F840, ...` - the class-name literal `"ID_Ice"`)
* **Verdict:** **ready** (confidence high)

### Frame

Identical to #10: `stwu r1,-0x10(r1)`; LR `+0x14`; `r31` `+0x0C`; `r30` `+0x08`.

### Constants

None.

### Behaviour

```c
usize fn_80afec80(usize this, i32 free_count) {
    if (this != 0) {                       // 0x80AFEC88 cmpwi r3,0; 0x80AFECA0 beq -> return
        if (this == 0) goto after_rel;     // 0x80AFECA4 beq 0x80AFECB0 - same CR0, never taken
        fn_8015f800(this, 0);              // 0x80AFECA8 li r4,0; 0x80AFECAC bl
after_rel:
        if (free_count > 0)                // 0x80AFECB0 cmpwi r31,0; 0x80AFECB4 ble
            fn_802b9280(this);             // 0x80AFECB8 mr r3,r30; 0x80AFECBC bl
    }
    return this;                           // 0x80AFECC0 mr r3,r30, epilogue
}
```

Verified mechanically: the 23 words of this body are word-for-word identical
to #10's once the two relative `bl` displacements are normalised (the call
targets `0x8015F800`/`0x802B9280` sit at fixed absolute addresses, so the
encoded displacements differ). Same registers, same order, same callees.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80AFECAC` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80AFECBC` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. `ANALYSIS_WAVE127_ENEMIES.md` lists this address in the w119 skip set
("teardown with the extra `cmpi` guard"); no earlier spec names it.

---

## 12. `0x80A2F670` - the same teardown with a single guard

* **TSV size:** 88 bytes
* **Reachable body:** 22 instructions; 2 `bc`; 2 `bl`; ends at `0x80A2F6C4 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2f670_FUN_80a2f670.c`
* **Proposed signature:** `pub unsafe fn fn_80a2f670(this: usize, free_count: i32) -> usize`
* **Callers in this REL:** none by direct `bl`/`b`; reached through the word
  table at `0x80B3C75C` (region `0x80B3C74C: 0x80A2F9D0, 0x80A2FA00, 0, 0,
  0x80A2F670, 0x8015F840, ...`)
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR `+0x14`; `r31` `+0x0C` (= `free_count`); `r30` `+0x08`
(= `this`).

### Constants

None.

### Behaviour

```c
usize fn_80a2f670(usize this, i32 free_count) {
    if (this != 0) {                       // 0x80A2F678 cmpwi r3,0; 0x80A2F690 beq -> return
        fn_8015f800(this, 0);              // 0x80A2F694 li r4,0; 0x80A2F698 bl
        if (free_count > 0)                // 0x80A2F69C cmpwi r31,0; 0x80A2F6A0 ble
            fn_802b9280(this);             // 0x80A2F6A4 mr r3,r30; 0x80A2F6A8 bl
    }
    return this;                           // 0x80A2F6AC mr r3,r30, epilogue
}
```

This is #10/#11 with the redundant second `beq` absent - 88 bytes, 22
instructions. The Ghidra export and this body agree line for line.

### Call sites

| site | callee | arguments at the call |
|---|---|---|
| `0x80A2F698` | `0x8015F800` | `r3 = this`, `r4 = 0` |
| `0x80A2F6A8` | `0x802B9280` | `r3 = this` |

### Ambiguity

None. Under the tie-break this is the first of the eight 22-instruction,
88-byte teardowns; the other seven remain unclaimed.

---

## Cross-references and collisions

All twelve addresses were checked against the `address` column of
`ledger/functions.csv` (16,453 data rows: absent) and against every
`0x80Axxxxx` / `0x80Bxxxxx` token in every `#`-heading of every `docs/*.md`
(absent). They are *named* in prose in earlier waves, always as a neighbour,
caller or callee, never as a specified body:

| address | where it is mentioned | in what role |
|---|---|---|
| `0x80AB2620` | `SPEC_WAVE132_ENEMIES.md` (rank 52 body `0x80AB22D0`) | prose: "the two pad sub-objects already written by `0x80AB2620`; the `+0x988 == 1` guard selects the second one" |
| `0x80A0B8A0` | `SPEC_WAVE129_ENEMIES.md` (rank 11 body `0x80A0BD20`) | call site `0x80A0BDE4`, `fn_80A0B8A0(this, 0)` |
| `0x80B0D070` | `SPEC_WAVE134_ENEMIES.md` (rank 83 body `0x80B0DB90`) | callee `0x80B0DBF4`; "sibling `0x80B0D070` sets `+0x674`/`+0x770`/`+0x6AC`/`+0x7A8`"; "separate TSV row, not ledger" |
| `0x80B0D0E0` | `SPEC_WAVE135_ENEMIES.md` (rank 94 body `0x80B0DD80`) | helper table and callee `0x80B0DDDC` |
| `0x80AB1890` | analysis only (`ANALYSIS_WAVE127_ENEMIES.md`, w120 skip list) | no earlier spec mention |
| `0x80A6EFB0` | analysis only (w124 skip list) | no earlier spec mention |
| `0x80AD6CA0` | analysis only (w111 skip list) | no earlier spec mention |
| `0x80AED210`, `0x80AED1B0` | analysis only (w122 skip list) | no earlier spec mention |
| `0x80A2F6D0`, `0x80A2F670` | analysis only (w123 skip list) | no earlier spec mention |
| `0x80AFEC80` | analysis only (w119 skip list) | no earlier spec mention |

Two readings need reconciling, neither of them blocking:

1. **Wave 129's call arity for `0x80A0B8A0`.** That spec writes
   `fn_80A0B8A0(this, 0)`; wave 127 describes the body as "tail recursion on `r3`
   only". The stream settles it: `r4` is forwarded into `0x80162D50` and the
   recursion passes `found` in `r4`, so the two-parameter form is correct and
   wave 129's call site should be kept as written.
2. **Wave 134's `2^52` note.** `SPEC_WAVE134_ENEMIES.md` reads the
   `lfd` + `fsub` pair as collapsing to `0.0` for `0x80A79750`, while
   `SPEC_WAVE129/131/133/135` read it as an exact conversion. Rank 3 uses the
   exact-conversion reading, and it is the only row here that depends on it:
   `xoris r6,r4,0x8000` XORs bit 31 of the sign-extended `s16` (the `UI` field
   is the *high* half), so `(double)(2^52 + (v ^ 0x80000000)) -
   (2^52 + 2^31)` is exactly `v`. If the wave-134 reading were applied to this
   body, both stores would become `0.0`, which cannot be right for a speed
   field.

New helper claims: none. The two `main.dol` callees of rank 5 are described in
`COVERAGE_WAVE130_BASES.md` / `COVERAGE_WAVE133_BASES.md`; the rest of the
helper table is reused from waves 128-135.

## Verification

* **Selection.** The twelve addresses are the first twelve non-ledger,
  non-specified B rows at or below wave 135's last count: reachable counts
  `28, 27, 26, 25, 25, 25, 25, 25, 24, 23, 23, 22`, TSV sizes
  `112, 108, 104, 100, 100, 100, 100, 100, 96, 92, 92, 88`. The 28-instruction
  tie with wave 135's `0x80A1E100` is resolved by ascending address; the
  22-instruction band is entered at its lowest unclaimed address and cut after
  `0x80A2F670`. Three ledger rows share that band and are skipped without taking
  a slot (`0x80A1F390`, `0x80A1FFB0`, `0x80ADD0E0`); no ledger row sits *between*
  the twelve chosen ones.
* **Sizes.** Every TSV size above matches the size column of
  `work/ghidra/rel_d_enemiesNP/export/functions.tsv`, and in all twelve the
  reachable span equals `TSV size / 4` instructions with no trailing unreachable
  words.
* **Control flow.** 22 conditional branches, all with in-body targets; 18 `bl`;
  1 tail `b` (rank 4, to `0x800B2EE0`); no `bctrl`; no indirect call; 12
  terminating `blr`s plus the one tail `b`. No body reads `f1`/`f2`/`f3`/`f4`
  before writing it, and there is no `psq` op anywhere in the tranche.
* **Constants.** Every in-REL constant quoted above was read from
  `work/rels/elf/d_enemiesNP.elf` at `(address - 0x80A00000) + 0x1000`
  (data addresses use their own `PT_LOAD`), and the values are the shipped
  values: `0x80B2FAF0 = 1.0f`, `0x80B2FB38 = -2^-12`,
  `0x80B2FB3C = 3*2^-12`, `0x80B2FB40 = 0x4330000080000000`,
  `0x80B2DA60 = 56.0f`, `0x80B2DA64 = 120.0f`, `0x80B309C4 = 0.0f`,
  `0x80B309C8 = -22.0f`. `main.dol` callees are marked **(ext)**.
* **Entry references.** Direct-caller and data-word scans over the whole REL
  agree with the "callers" lines: `0x80A0B8A0` <- `0x80A0BDE4`/self,
  `0x80AB1890` <- `0x80AB0720`/`0x80AB2258`, `0x80A6EFB0` <- `0x80A6F1B4`,
  `0x80B0D070` <- `0x80B0DBF4`, `0x80B0D0E0` <- `0x80B0DDDC`; the other seven
  appear only as 32-bit words in the vtable clusters quoted per section.
* **Ledger/doc state.** No ledger row, Rust file, CSV or other doc was written
  by this pass. `docs/SPEC_WAVE136_ENEMIES.md` is the only artefact.
* **Concurrent-lane overlap.** When this page was written `src/game/enemies.rs`
  already contained `fn_80afec80` (the rank-11 body) with the same two calls and
  the same dead-branch note; it was added by the wave-136 implementation lane
  before `ledger/functions.csv` was updated, so the ledger-based selection above
  is still satisfied. The other eleven have no Rust definition at this pass -
  `0x80A0B8A0`, `0x80B0D070` and `0x80B0D0E0` appear only as callee constants,
  and `0x80A0B8A0` is called with the two-argument form specified here.
