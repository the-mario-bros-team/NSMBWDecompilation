# Wave 135 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses: 173 class **B** (reconstructible now), 160 class C and
51 class D.

Waves 128 - 134 (`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE134_ENEMIES.md`)
specified **84** of those B rows, which occupy ranks 1-86 of the wave-127
ordering. Two ranks in that window are `translated` ledger rows and carry no
spec; rank 65 (`0x80A07080`) is the documented one.

This lane is the continuation: **ranks 87-98**, the twelve B rows immediately
below wave 134's window. It is a **read-only** pass: it writes no Rust, no
ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the corrected
`+0x1000` segment shift of `docs/REL_ELF_MAPPING.md`, cross-checked against the
Ghidra export at the cited path and against
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Every constant that lives
inside the REL image was read out of the ELF itself, so the float values below
are the shipped values. Addresses that do not resolve in this REL are marked
**(ext)**; they are `main.dol` addresses and are described from the call site.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then ascending address.

The metric was re-validated on the three preceding tranches before this page
was written. Walking the twelve wave-132 addresses reproduces
`71, 70, 69, 69, 64, 62, 62, 61, 61, 60, 60, 58`; the twelve wave-133
addresses reproduce `57, 57, 57, 56, 56, 56, 54, 53, 52, 52, 52, 52`; and the
twelve wave-134 addresses reproduce `47, 46, 44, 44, 44, 43, 42, 40, 39, 38,
34, 34`. This page picks up at 34 - the next count below wave 134's last row -
and runs down to 28.

| rank | address | TSV size | reachable insns | body character | status |
|---:|---|---:|---:|---|---|
| 87 | `0x80AFEEB0` | 136 | 34 | `strcmp` of the text after the last `:` on both sides | **ready** |
| 88 | `0x80A2D660` | 132 | 33 | `+0x6C0` countdown: retire or `0x80A2D460`, then two resets | **ready** |
| 89 | `0x80A398C0` | 132 | 33 | `+0xDC` ramp to `1.0` by `0.05`, then a `+0xD4` callback | **ready** |
| 90 | `0x80A7EE20` | 128 | 32 | sub-object `+0x548` reset + per-frame step chain | **ready** |
| 91 | `0x80A77BB0` | 124 | 31 | `+0x72C` step by `+0x874`/`+0x88C`, 6-argument emitter call | **ready** |
| 92 | `0x80AEEA30` | 124 | 31 | two nibble-to-float ramps into `+0xDC`/`+0xE0` | **conditional** |
| 93 | `0x80B0D1B0` | 124 | 31 | `Wm_en_spindamage` emitter at `+0x9A0..0x9A8` | **ready** |
| 94 | `0x80B0DD80` | 124 | 31 | side flip on the sign of the `0x80B0D2A0` distance probe | **ready** |
| 95 | `0x80B0C4C0` | 120 | 30 | two-slot `+0x3F0`/`+0x3F4` callback bridge | **ready** |
| 96 | `0x80A441E0` | 116 | 29 | position copy from the `+0x9F8` link, `+0xB0 -= [0x8042B354]` | **conditional** |
| 97 | `0x80A778F0` | 116 | 29 | four-float `+0x728..+0x734` table, same 6-argument call | **ready** |
| 98 | `0x80A1E100` | 112 | 28 | two `0x800146B0`/`0x800146F0` writes into `+0x698` | **ready** |

**Split: 10 ready, 2 conditional.** Total 1,488 bytes, 372 reachable
instructions.

One ledger row sits inside the window and is skipped without consuming a rank:
`0x80A06B00` (124 bytes, 31 reachable instructions) is `translated` in
`ledger/functions.csv` (`game::enemies`, `fn_80a06b00`), and it falls between
rank 90 (`0x80A7EE20`, 32) and rank 91 (`0x80A77BB0`, 31) in the ordering.

"Conditional" is used exactly as in waves 128-134: one named datum must be read
before a byte-exact port. Rank 96 is the conventional case - a `main.dol` `f32`
at `0x8042B354` is subtracted from the copied `y`. Rank 92 is the other kind:
its `+0xE4` word is copied from an **uninitialised stack slot**, so the stored
value is whatever the caller left at `0x10(r1)`.

The four risk classes that made waves 110-126 skip a body are all absent:

* **no paired-single traffic at all.** Not one `psq_l`/`psq_st` in the tranche;
  no `GQR1..GQR7`, no `ldexpf` noise in any of the twelve exports.
* **no `bctr` jump table.** All 18 conditional branches target an address inside
  their own body (checked mechanically: zero out-of-body `bc` targets). The six
  `bctrl` sites are vtable dispatches with resolved slots
  (`+0x20`, `+0xD4`, `+0x1C`, `+0xA0`, `+0x3F0`, `+0x3F4`).
* **no incoming float argument.** No body reads `f1`/`f2`/`f3` before writing
  it; every float operand is loaded from the actor or from an in-REL constant.
* **no unledgered callee landmine.** Every in-REL `bl` target is a
  `functions.tsv` entry: `0x80A2D460`, `0x80A7E630`, `0x80A7E780`,
  `0x80B0D0E0`, `0x80B0D2A0`.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000` for the first `PT_LOAD`
  (`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`). The image ends at `0x80B6E438`;
  anything at or above that is a `main.dol` address.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* Branch mnemonics are the C form after the exact `bc` decode:
  `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bc 12,0` = `blt`, `bc 4,0` = `bge`.
  `bc 4,2` after the `cror 2,1,2` idiom is written `>=` in the pseudocode below
  (see rank 89).
* `addi rX,r0,imm` is `li rX,imm` (`r0` is not read when `RA == 0`).
* **Three-operand single-precision ops are not uniform.** `fadds`/`fsubs`/
  `fdivs` take their second source from instruction bits 11-15, while
  `fmuls`/`fmadds`/`fmsubs` take the multiplicand from bits 6-10 and the
  addend from bits 11-15. Reading `fmuls f1,f2,f3` as `f1,f2,f0` is the one
  mis-read that changes rank 92's result.
* **The `2^52` integer-to-float idiom** (`stw 0x43300000,r1+n` + `stw value` +
  `lfd` + `fsubs`) is read as an exact conversion, the convention used by
  `SPEC_WAVE129` ("`f1 = (f32)cell` exactly"), `SPEC_WAVE131` ("exactly the
  unsigned byte `n`") and `SPEC_WAVE133` ("the conversion is exact for
  `i < 2^24`"). `SPEC_WAVE134` reads the same pair as collapsing to `0.0` on
  its rank 75; see "Cross-references and collisions" - that disagreement does
  not touch any row before rank 92.
* All twelve bodies end in a single `blr` or a single tail `b` and are exactly
  `TSV size / 4` instructions with no trailing unreachable words; the "ends at"
  address below is the address **of** that last instruction.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv` evidence column) or the helper
tables of waves 128-134, not new claims.

| address | role |
|---|---|
| `0x800146B0` | write `(f1,f2,f3)` to `r3+0xC / +0x10 / +0x14` (Vec3 setter) |
| `0x800146F0` | `r3+0x0A = 2`, `f32 r3+0x18 = f1`, `u16 r3+0x1C = r4`, `r3+0x1E = 0` |
| `0x8006CDE0` | per-frame body step, `r3 = this` (recurring preamble in this family) |
| `0x8006EF40` | landing/contact probe on a body object, returns a bit mask |
| `0x80072BC0` | "`0x8006DB30` status" query on the shared body object |
| `0x8007FB10` | six-argument emitter: splits `param_3[0..6]` into two points and forwards to `0x8007FAD0` |
| `0x8007FDA0` | tile/path update on a sub-object (eight corner stores, `0x80080170`) |
| `0x800955F0` | orbit/accumulator step (`+0x44C`, `+0x45C`/`+0x45D` guards) |
| `0x80097170` | notice latch over `0x80097250` (`+0x45A`, `+0x390`) |
| `0x800DAE80` | reset/stop sweep over `0x60`-byte motion records **(ext, unexported)** |
| `0x80162510` | recursive `+0x0B` shutdown mark plus virtual `+0x38` on the `+0x60` vtable |
| `0x80162D00` | class-list lookup; `id & 7` selects a `0x20`-byte bucket |
| `0x802E1AB4` | `strcmp` (aligned word path collapsed to bytes) |
| `0x802E1C40` | `strrchr` (last match accumulator) |
| `0x80A2D460` | in-REL: `+0x666 == 2 ? tail 0x80021600 : tail 0x800214F0` |
| `0x80A7E630` | in-REL 8-byte trampoline (rank of wave 134) |
| `0x80A7E780` | in-REL `+0x610` variant effect (ledger `translated`) |
| `0x80B0D0E0` | in-REL: writes `3` to `+0x674/+0x770/+0x6AC/+0x7A8`, two `0x80166F40` |
| `0x80B0D2A0` | in-REL: distance probe, returns `probe_x - *(f32*)(this+0xAC)` |

---

## 1. rank 87 - `0x80AFEEB0`, suffix-of-name equality test

* **TSV size:** 136 bytes
* **Reachable body:** 34 instructions; 2 `bl` + 1 `bctrl`; ends at
  `0x80AFEF34 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80afeeb0_FUN_80afeeb0.c`
* **Proposed signature:** `pub unsafe fn fn_80afeeb0(this: usize, name: *const u8) -> u32`
* **Callers in this REL:** none by direct `bl` or `b`; the body shape
  (`(this->vtbl[8])()` then a `strcmp`) says it is reached through a vtable slot
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `name`, `r30@+0x08` = `this`.
No locals, no FPR traffic.

### Constants

None in the REL image. The only literal is the selector byte `0x3A` (`':'`),
built twice with `li r4,58` at `0x80AFEEC4` and `0x80AFEEF8`.

### Behaviour

```c
char *a = (char *)name;                    // 0x80AFEEC0  mr r31,r4
char *p = strrchr(a, ':');                 // 0x80AFEED4  bl 0x802E1C40
if (p != 0) a = p + 1;                     // 0x80AFEED8 cmpwi / 0x80AFEEDC beq
                                           // 0x80AFEEE0 addi r31,r3,1

char *n = (char *)((u32 *)(*(u32 *)this))[0x20 / 4](this);   // 0x80AFEEE4..0x80AFEEF4
char *q = strrchr(n, ':') + 1;             // 0x80AFEEF8 bl / 0x80AFEF04 addi r3,r3,1
return strcmp(q, a) == 0;                  // 0x80AFEF08 bl / 0x80AFEF0C..0x80AFEF1C
```

The vtable dispatch is `lwz r12,0(r30)` -> `lwz r12,32(r12)` -> `mtctr r12` ->
`bctrl` with `r3 = this` (`0x80AFEEE4..0x80AFEEF4`), i.e. **slot `+0x20` of
`*(u32 *)this`**. That slot returns a `char *` in `r3`.

The suffix is taken on **both** sides. On the first side the `+1` is guarded
(`beq` skips it when `strrchr` returns 0); on the second side the `+1` at
`0x80AFEF04` is **unconditional** - the shipped body adds 1 to a possibly-null
`strrchr` result. Reproduce it as written, not with a second guard.

Return value: `li r3,1` at `0x80AFEF14` when `strcmp` returns 0, else
`li r3,0` at `0x80AFEF1C`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AFEED4` | `0x802E1C40` | `r3 = name`, `r4 = 0x3A` |
| `0x80AFEEF4` | vtable `+0x20` of `*(u32 *)this` | `r3 = this` |
| `0x80AFEEFC` | `0x802E1C40` | `r3` = the string returned by the slot, `r4 = 0x3A` |
| `0x80AFEF08` | `0x802E1AB4` | `r3 = strrchr(name_out) + 1`, `r4 = a` |

### Ambiguity

None for this body: no constants, no external operands, no stack locals. The
only assumption is that slot `+0x20` returns a NUL-terminated `char *`, which
the `strrchr` call makes explicit.

---

## 2. rank 88 - `0x80A2D660`, `+0x6C0` countdown then retire

* **TSV size:** 132 bytes
* **Reachable body:** 33 instructions; 5 `bl` + 0 `bctrl`; ends at
  `0x80A2D6E0 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2d660_FUN_80a2d660.c`
* **Proposed signature:** `pub unsafe fn fn_80a2d660(this: usize) -> u32`
* **Callers in this REL:** none by direct `bl` or `b` (a vtable step)
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `this`. No locals.

### Constants

None in the REL image. Every operand is an actor field or an immediate `0`.

### Behaviour

```c
fn_8006EF40(this + 0x1EC, 0);            // 0x80A2D668 li r4,0 / 0x80A2D67C bl

if (*(u16 *)(this + 0x6C0) == 0) {       // 0x80A2D680 lhz 1728 / 0x80A2D684 cmpwi
    fn_80162510(this);                   // 0x80A2D6A4  retire
} else {
    *(u16 *)(this + 0x6C0) = *(u16 *)(this + 0x6C0) - 1;   // 0x80A2D68C / 0x80A2D690
    fn_80A2D460(this);                   // 0x80A2D698
}
if (*(u16 *)(this + 0x6C4) != 0)         // 0x80A2D6A8 lhz 1732 / 0x80A2D6B0 beq
    fn_8007FDA0(this + 0x534);           // 0x80A2D6B4 addi r3,r31,1332 / 0x80A2D6B8 bl

fn_800217B0(this);                       // 0x80A2D6C0
*(u8 *)(this + 0x366) = 0;               // 0x80A2D6C4 li r0,0 / 0x80A2D6C8 stb r0,870
return 1;                                // 0x80A2D6CC li r3,1
```

The countdown is a `u16` reload/store pair, not a register decrement: the
`lhz`/`addi r0,r3,-1`/`sth` sequence at `0x80A2D680..0x80A2D690` truncates at
`0xFFFF` and wraps to `0xFFFF` from `0`.

`0x80A2D460` is itself a wave-127 B row: it reads `*(u8 *)(this+0x666)` and
tails to `0x80021600` when the byte is 2, otherwise to `0x800214F0`. That row
is outside this tranche; the call here is by address with all operands explicit.

`0x800217B0` is a `main.dol` entry that sits in a gap of the `main-gb` export
set (between `0x800217A0` and `0x800217F0`), so only the call shape is known:
`r3 = this`, no return used. It is the same "actor post-step" slot the family
uses; nothing in the body depends on its result.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A2D67C` | `0x8006EF40` | `r3 = this+0x1EC`, `r4 = 0` |
| `0x80A2D698` | `0x80A2D460` | `r3 = this` |
| `0x80A2D6A4` | `0x80162510` | `r3 = this` |
| `0x80A2D6B8` | `0x8007FDA0` | `r3 = this+0x534` |
| `0x80A2D6C0` | `0x800217B0` | `r3 = this` |

### Ambiguity

The two `+0x6C0`/`+0x6C4` halves are `lhz`/`sth`, so they are unsigned 16-bit.
Nothing external is consumed arithmetically; the row is ready.

---

## 3. rank 89 - `0x80A398C0`, `+0xDC` ramp with a callback at the ceiling

* **TSV size:** 132 bytes
* **Reachable body:** 33 instructions; 1 `bl` + 1 `bctrl`; ends at
  `0x80A39940 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a398c0_FUN_80a398c0.c`
* **Proposed signature:** `pub unsafe fn fn_80a398c0(this: usize)`
* **Callers in this REL:** none by direct `bl` or `b` (a vtable step)
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `this`. `r5`/`r4` hold the
two constant pages (`lis r5,0x80B3` / `lis r4,0x80B3`), not actor bases.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C2A0` | `f32` | `0.05` - per-frame ramp added to `+0xDC` |
| `0x80B2C268` | `f32` | `1.0` - ceiling, also stored back into `+0xDC` |
| `0x80B70618` | **(ext)** | `main.dol` pointer passed as `r4` to the `+0xD4` slot |

`0x80B70618` lies `0x21E0` bytes above the end of this REL image, so it is a
`main.dol` datum. It is only passed as an argument (never dereferenced here);
its identity does not block the port.

### Behaviour

```c
f32 t = *(f32 *)(this + 0xDC) + 0.05f;        // 0x80A398E4 lfs / 0x80A398E8 fadds
*(f32 *)(this + 0xDC) = t;                    // 0x80A398EC stfs

if (t >= 1.0f) {                              // 0x80A398F0 fcmpo cr0,f1,f0
                                              // 0x80A398F4 cror 2,1,2
                                              // 0x80A398F8 bne 0x80A39924 (skip)
    *(f32 *)(this + 0xDC) = 1.0f;             // 0x80A398FC stfs f0,220(r3)
    fn_800DAE80(this + 0x5B0);                // 0x80A39900 addi / 0x80A39904 bl
    (*(void (**)(void *, void *))(*(u32 *)(this + 0x60) + 0xD4))
        (this, (void *)0x80B70618);           // 0x80A39908..0x80A39920 bctrl
}
*(f32 *)(this + 0xE0) = *(f32 *)(this + 0xDC);   // 0x80A39924 lfs / 0x80A39928 stfs
*(f32 *)(this + 0xE4) = *(f32 *)(this + 0xDC);   // 0x80A3992C stfs
```

The comparison is the `fcmpo` + `cror 2,1,2` idiom: `cror` sets `CR0[EQ]` to
`CR0[GT] | CR0[EQ]`, so the following `bne` skips only when the value is
**below** `1.0` (or unordered). Write it as `if (t >= 1.0f)`, not as
`if (t < 1.0f) ... else ...`.

The two trailing stores reload `+0xDC` at `0x80A39924` (after the branch
merge), so both fields receive the final value - `1.0` on the clamped path,
the incremented value otherwise.

The callback receiver is `this` (vtable at `*(u32 *)(this + 0x60)`), slot
`+0xD4`, with the `main.dol` pointer as its second argument.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A39904` | `0x800DAE80` | `r3 = this+0x5B0` |
| `0x80A39920` | vtable `+0xD4` of `*(u32 *)(this+0x60)` | `r3 = this`, `r4 = 0x80B70618` |

### Ambiguity

None for the body. The `+0xD4` slot is resolved; the `main.dol` pointer is only
forwarded, so the row is ready.

---

## 4. rank 90 - `0x80A7EE20`, sub-object reset plus the per-frame chain

* **TSV size:** 128 bytes
* **Reachable body:** 32 instructions; 6 `bl` + 1 `bctrl`; ends at
  `0x80A7EE9C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7ee20_FUN_80a7ee20.c`
* **Proposed signature:** `pub unsafe fn fn_80a7ee20(this: usize)`
* **Callers in this REL:** none by direct `bl` or `b` (a vtable step)
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `this`. No locals.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2E248` | `f32` | `1.0` - `f1` argument of `0x80097170` |

`0x80B2E248` is loaded as `lis r4,0x80B3` + `lfs f1,-7608(r4)`
(`0x80A7EE70..0x80A7EE78`).

### Behaviour

```c
(*(void (**)(void *))(*(u32 *)(this + 0x548) + 0x1C))(this + 0x548);  // 0x80A7EE34..44
fn_8006CDE0(this);                    // 0x80A7EE4C
fn_800955F0(this);                    // 0x80A7EE54
if (fn_80072BC0(this + 0x1EC) != 0)   // 0x80A7EE58 addi / 0x80A7EE5C bl / 0x80A7EE60 cmpwi
    fn_80A7E780(this);                // 0x80A7EE6C
fn_80097170(1.0f, this, this + 0xAC); // 0x80A7EE78 lfs f1 / 0x80A7EE74 r3=this
                                      // 0x80A7EE7C addi r4,r31,172 / 0x80A7EE80 bl
fn_80A7E630(this);                    // 0x80A7EE88
```

The opening dispatch loads `r12 = *(u32 *)(this + 0x548)` then
`r12 = *(u32 *)(r12 + 0x1C)`, caches it in `ctr` (`0x80A7EE38..0x80A7EE3C`) and
**only then** computes the receiver `r3 = this + 0x548`
(`0x80A7EE40 addi r3,r3,1352`). The slot is `+0x1C` of the sub-object vtable,
not of the actor vtable.

The order of the six calls is load-bearing for byte comparison:
`+0x548 slot`, `0x8006CDE0`, `0x800955F0`, conditional `0x80A7E780`,
`0x80097170`, `0x80A7E630`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A7EE44` | vtable `+0x1C` of `*(u32 *)(this+0x548)` | `r3 = this+0x548` |
| `0x80A7EE4C` | `0x8006CDE0` | `r3 = this` |
| `0x80A7EE54` | `0x800955F0` | `r3 = this` |
| `0x80A7EE5C` | `0x80072BC0` | `r3 = this+0x1EC` |
| `0x80A7EE6C` | `0x80A7E780` | `r3 = this` |
| `0x80A7EE80` | `0x80097170` | `f1 = 1.0`, `r3 = this`, `r4 = this+0xAC` |
| `0x80A7EE88` | `0x80A7E630` | `r3 = this` |

### Ambiguity

None. `0x80097170`'s Ghidra export declares a single parameter; the other two
registers are restored from the call sites, and here they are `this` and
`this+0xAC` - i.e. a `(f32, void *, void *)` call.

---

## 5. rank 91 - `0x80A77BB0`, `+0x72C` step then the six-argument emitter

* **TSV size:** 124 bytes
* **Reachable body:** 31 instructions; 0 `bl` + 0 `bctrl`; 4 `bc`, 3 internal
  `b`, and a tail `b 0x8007FB10` at `0x80A77C28`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a77bb0_FUN_80a77bb0.c`
* **Proposed signature:** `pub unsafe fn fn_80a77bb0(this: usize)`
* **Callers in this REL:** `0x80A77D00`, `0x80A77DFC`, `0x80A7829C`
* **Confidence:** **high / ready**

### Frame

None. The body never touches `r1`; no register saves, no FPR traffic. `r4` is
used as the constant page (`lis r4,0x80B3` + `addi r4,r4,-8984` =
`0x80B2DCE8`).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2DD74` | `f32` | `8.0` - `+0x72C` for `+0x874 == 1`, variant != 4 |
| `0x80B2DDB8` | `f32` | `12.0` - `+0x72C` for `+0x874 == 0`, variant 4 |
| `0x80B2DDBC` | `f32` | `4.0` - `+0x72C` for `+0x874 == 0`, other variant |
| `0x80B2DDC0` | `f32` | `36.0` - `+0x72C` for `+0x874 == 1`, variant 4 |

All four are reached as `base 0x80B2DCE8` + offset (`+0x8C`, `+0xD0`, `+0xD4`,
`+0xD8`). The block is shared with rank 97, so the offsets are quoted from the
same base there.

### Behaviour

```c
if (*(u32 *)(this + 0x874) == 0) {                    // 0x80A77BB0 lwz 2164 / 0x80A77BBC cmpwi
    *(f32 *)(this + 0x72C) =                          // 0x80A77BC4 lwz 2188 / 0x80A77BC8 cmpli
        (*(u32 *)(this + 0x88C) == 4) ? 12.0f : 4.0f; // 0x80A77BD0 / 0x80A77BDC
} else if (*(u32 *)(this + 0x874) == 1) {             // 0x80A77BE8 cmpli / 0x80A77BEC bne
    *(f32 *)(this + 0x72C) =
        (*(u32 *)(this + 0x88C) == 4) ? 36.0f : 8.0f;  // 0x80A77BFC / 0x80A77C08
}
return fn_8007FB10(this + 0x754,                      // 0x80A77C24 addi r3,r3,1876
                   this,                              // 0x80A77C14 mr r4,r3
                   this + 0x728,                      // 0x80A77C18 addi r5,r3,1832
                   3,                                 // 0x80A77C20 li r6,3
                   *(u8 *)(this + 0x38F),             // 0x80A77C10 lbz r7,911
                   this + 0x838);                     // 0x80A77C1C addi r8,r3,2104
```

Both discriminant compares are **unsigned** (`cmpli`), and both compare against
`4`; `+0x874` compares against `0`/`1`. A value outside `{0,1}` leaves `+0x72C`
untouched and still emits.

The tail call is a plain `b 0x8007FB10` (no LR write), so this body's own frame
is gone by the time the emitter runs. The six arguments are a `Vec`-like
`{ this+0x728, this+0x72C, this+0x730, this+0x734 }` plus the record at
`this+0x838`; `0x8007FB10` reads `param_3[0..6]` and forwards them to
`0x8007FAD0`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A77C28` | `0x8007FB10` (tail) | `r3 = this+0x754`, `r4 = this`, `r5 = this+0x728`, `r6 = 3`, `r7 = *(u8 *)(this+0x38F)`, `r8 = this+0x838` |

### Ambiguity

None in-REL. The Ghidra export stops at `halt_baddata` after the two variant
arms; the `lbz`/`mr`/`addi`/`li` block at `0x80A77C10..0x80A77C24` and the tail
`b` are the missing six instructions, and they are plain code.

---

## 6. rank 92 - `0x80AEEA30`, two nibble ramps into `+0xDC`/`+0xE0`

* **TSV size:** 124 bytes
* **Reachable body:** 31 instructions; 0 `bl` + 0 `bctrl`; no branches; ends at
  `0x80AEEAA8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aeea30_FUN_80aeea30.c`
* **Proposed signature:** `pub unsafe fn fn_80aeea30(this: usize)`
* **Callers in this REL:** `0x80AED540`
* **Confidence:** **medium / conditional**

### Frame

`stwu r1,-0x30(r1)`; **no LR save** (leaf). Locals: `+0x08`, `+0x0C` (dead
stores of the two results), `+0x10` (**never written**), `+0x18`/`+0x1C` and
`+0x20`/`+0x24` (the two `2^52 + n` page words).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B31244` | `f32` | `0.5` - multiplier for both ramps |
| `0x80B31248` | `f32` | `1.0` - added to each nibble before scaling |
| `0x80B31250` | `f64` | `0x4330000000000000` (`2^52`) - bias subtracted by `fsubs` |
| `0x43300000` | `u32` | page word written to `+0x18` and `+0x20` (`lis r6,0x4330`) |

### Behaviour

```c
u32 w  = *(u32 *)(this + 4);              // 0x80AEEA40 lwz r0,4(r3)
u32 lo = w & 0xF;                          // 0x80AEEA48 rlwinm r4,r0,0,28,31
u32 hi = (w >> 4) & 0xF;                   // 0x80AEEA50 rlwinm r0,r0,28,28,31

f32 a = 1.0f + (f32)lo;                    // 0x80AEEA6C fsubs / 0x80AEEA7C fadds
f32 b = 1.0f + (f32)hi;                    // 0x80AEEA84 fsubs / 0x80AEEA8C fadds

*(f32 *)(this + 0xE4) = *(f32 *)(r1 + 0x10);   // 0x80AEEA80 stfs f0,228(r3)  <-- uninitialised
*(f32 *)(this + 0xDC) = 0.5f * a;              // 0x80AEEA88 fmuls / 0x80AEEA94 stfs 220
*(f32 *)(this + 0xE0) = 0.5f * b;              // 0x80AEEA98 fmuls / 0x80AEEAA0 stfs 224
```

The `2^52` conversions are `lfd f1,0x18(r1)` + `fsubs f3,f1,f5` and
`lfd f1,0x20(r1)` + `fsubs f0,f1,f5`; the page word is `0x43300000`, so the
loaded doubles are `2^52 + lo` and `2^52 + hi`. Read as exact, `lo` and `hi`
are the two nibbles of `*(u32 *)(this+4)`.

Two things in this body are not ordinary:

1. **The `+0xE4` store reads `0x10(r1)` before anything writes it.** The frame
   writes `0x08`, `0x0C`, `0x18`, `0x1C`, `0x20`, `0x24` only. Ghidra declares
   it `undefined4 local_20`. The shipped value is whatever the caller left at
   `this`'s `0x10(r1)`, so a byte-exact port needs the caller
   (`0x80AED540`) to be pinned first.
2. **The two dead stack stores** at `0x80AEEA90` (`stfs f1,8(r1)`) and
   `0x80AEEA9C` (`stfs f0,12(r1)`) duplicate the two actor stores. Keep them or
   drop them, but they are the reason the frame is `0x30` rather than `0x10`.

The order of the three actor stores is `+0xE4` (before either conversion is
consumed), then `+0xDC`, then `+0xE0`. A port that writes `+0xDC`/`+0xE0` first
and `+0xE4` last still ends in the same memory only if the uninitialised word
is modelled - which is the blocker.

The multiplier/operand reading was checked against the encoding: `fmuls`
takes its multiplicand from bits 6-10, so `0x80AEEA88` is `fmuls f1,f2,f3`
(`0.5 * (1.0 + lo)`) and `0x80AEEA98` is `fmuls f0,f2,f0`
(`0.5 * (1.0 + hi)`). Reading both as `*f0` would make the low nibble dead
code, which the `fadds f3,f4,f3` at `0x80AEEA7C` contradicts.

### Call sites

None - the body makes no calls.

### Ambiguity

The instruction stream is complete, but the `+0xE4` operand is not defined by
this function. Additionally, if the `fsubs` conversion is read as rounding its
operands (the `SPEC_WAVE134` reading) rather than its result, both ramps
collapse to `+0xDC = 0.0`, `+0xE0 = 0.5`. The two readings differ, so the row is
conditional on both counts.

---

## 7. rank 93 - `0x80B0D1B0`, `Wm_en_spindamage` emitter

* **TSV size:** 124 bytes
* **Reachable body:** 31 instructions; 0 `bl` + 1 `bctrl`; ends at
  `0x80B0D228 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0d1b0_FUN_80b0d1b0.c`
* **Proposed signature:** `pub unsafe fn fn_80b0d1b0(this: usize)`
* **Callers in this REL:** `0x80B0BF84`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; a three-word local `Vec3` at `+0x08`.
No saved GPRs.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B31E58` | `f32` | `24.0` - added to `+0x9A4` before the emit |
| `0x80B68008` | `char[]` | `"Wm_en_spindamage"` - name argument of the emit |

`0x80B68008` is inside this REL image (0x6430 bytes below the end) and was read
directly: the 15 bytes are `Wm_en_spindamage` followed by a NUL. It is reached
as `lis r4,0x80B7` + `addi r4,r4,-32760` (`0x80B0D1D4` / `0x80B0D1DC`).

### Behaviour

```c
if (*(u32 *)(this + 0x9B0) == 0) {                    // lwz 2480 / cmpwi 1 / beq
                                                      // cmpwi 0 / bne 0x80B0D21C
    f32 x = *(f32 *)(this + 0x9A0);                   // 0x80B0D1F0 lfs 2464
    f32 y = *(f32 *)(this + 0x9A4);                   // 0x80B0D1E0 lfs 2468
    f32 z = *(f32 *)(this + 0x9A8);                   // 0x80B0D1E8 lfs 2472
    Vec3 v;
    v.x = x;                                          // 0x80B0D1F8 stfs f3,8(r1)
    v.z = z;                                          // 0x80B0D204 stfs f1,16(r1)
    v.y = y + 24.0f;                                  // 0x80B0D1F4 fadds / 0x80B0D208 stfs f0,12(r1)

    (*(void (**)(void *, const char *, int, Vec3 *, int, int))
        (*(u32 *)(this + 0x7AC) + 0xA0))
        (this + 0x7AC, "Wm_en_spindamage", 0, &v, 0, 0);   // 0x80B0D20C..0x80B0D218
}
```

The guard is two compares on `+0x9B0` (`!= 1` then `== 0`), so the emit runs
only for the value `0`. `lwzu r12,1964(r3)` at `0x80B0D20C` **updates `r3` to
`this+0x7AC`** and loads the vptr from there; that updated `r3` is the receiver
the `bctrl` at `0x80B0D218` sees. The `f3`/`f1` load order is
`+0x9A0`, `+0x9A4`, `+0x9A8` but the store order into the local is `x`, `z`,
`y` (8, 16, 12) - all three land at `+0x08/+0x0C/+0x10`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0D218` | vtable `+0xA0` of `*(u32 *)(this+0x7AC)` | `r3 = this+0x7AC`, `r4 = 0x80B68008`, `r5 = 0`, `r6 = &v`, `r7 = 0`, `r8 = 0` |

### Ambiguity

None. The name string is in this REL (quoted above), and the `Vec3` is fully
defined by the three actor fields. The `+0x9B0` guard is a plain integer
compare.

---

## 8. rank 94 - `0x80B0DD80`, side flip on the sign of the distance probe

* **TSV size:** 124 bytes
* **Reachable body:** 31 instructions; 2 `bl` + 0 `bctrl`; ends at
  `0x80B0DDF8 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0dd80_FUN_80b0dd80.c`
* **Proposed signature:** `pub unsafe fn fn_80b0dd80(this: usize)`
* **Callers in this REL:** none by direct `bl` or `b` (a vtable step)
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x20(r1)`; LR at `+0x24`; `r31@+0x1C` = `this`; a two-`f32` local at
`+0x08` (the probe output buffer).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B31E34` | `f32` | `0.0` - the comparison value for the probe's `f1` |

### Behaviour

```c
f32 d = fn_80B0D2A0(this, (f32 *)(r1 + 8));      // 0x80B0DD8C addi r4,r1,8 / 0x80B0DD98 bl
u32 lt = CR0_LT(fcmpo d, 0.0f);                  // 0x80B0DDA4 lfs / 0x80B0DDA8 fcmpo
                                                 // 0x80B0DDAC mfcr / 0x80B0DDB0 rlwinm r0,r0,1,31,31
if (*(u8 *)(this + 0x348) != lt &&               // 0x80B0DDB4 cmplw cr0,r4,r0
    *(u16 *)(this + 0x9E0) == 0) {               // 0x80B0DDBC lhz 2528 / 0x80B0DDC4 bne
    *(u16 *)(this + 0x9E0) = 1;                  // 0x80B0DDD0 sth r3,2528
    *(u8 *)(this + 0x348) ^= 1;                  // 0x80B0DDC8 xori r0,r4,0x1 / 0x80B0DDD4 stb
}
fn_80B0D0E0(this);                               // 0x80B0DDDC
*(u16 *)(this + 0x9E2) = 16;                     // 0x80B0DDE0 li r0,16 / 0x80B0DDE4 sth 2530
```

The predicate is the `CR0.LT` bit, not a boolean: `mfcr r0` puts `CR0` in the
top nibble, and `rlwinm r0,r0,1,31,31` leaves exactly the `LT` bit as 0/1.
`cmplw cr0,r4,r0` then compares the stored side byte against that 0/1 value, so
the flip happens when the side byte disagrees with the sign of the returned
distance. `0x80B0D2A0` returns `probe_x - *(f32 *)(this + 0xAC)` (the probe
writes `x`/`y` into the caller's two-float buffer).

`+0x9E2 = 16` is a `u16` store of the decimal 16 (`li r0,16`), not `0x10`
bytes of anything.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0DD98` | `0x80B0D2A0` | `r3 = this`, `r4 = r1+8` (two-`f32` output) |
| `0x80B0DDDC` | `0x80B0D0E0` | `r3 = this` |

### Ambiguity

None. `0x80B0D2A0` is in-REL and its own `GQR`-shaped prologue in the export is
an `f30`/`f31` spill only; the return value is a single `f32` in `f1`
(the export spells it `double`).

---

## 9. rank 95 - `0x80B0C4C0`, two-slot callback bridge

* **TSV size:** 120 bytes
* **Reachable body:** 30 instructions; 0 `bl` + 2 `bctrl`; ends at
  `0x80B0C534 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0c4c0_FUN_80b0c4c0.c`
* **Proposed signature:** `pub unsafe fn fn_80b0c4c0(a: usize, b: usize, c: usize) -> u32`
* **Callers in this REL:** none by direct `bl` or `b` (registered callback)
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = the inner object,
`r30@+0x08` = `a` (the first incoming argument).

### Constants

None in the REL image. Two vtable slots: `+0x3F0` (query) and `+0x3F4`
(action).

### Behaviour

```c
void *obj = *(void **)(c + 4);                     // 0x80B0C4D8 lwz r31,4(r5)

if ((*(u32 (**)(void *))(*(u32 *)(obj + 0x60) + 0x3F0))(obj) == 0) {
                                                    // 0x80B0C4DC..0x80B0C4EC bctrl
                                                    // 0x80B0C4F0 cmpwi / 0x80B0C4F4 beq
    (*(void (**)(void *, usize, int))(*(u32 *)(obj + 0x60) + 0x3F4))(obj, a, 0);
                                                    // 0x80B0C500..0x80B0C518 bctrl
}
return 1;                                           // 0x80B0C4F8 / 0x80B0C51C li r3,1
```

`0x80B0C4D8` reads `param_3 + 4`, i.e. the third argument is a holder whose `+4`
points at the real object; the holder itself is never touched. The first call
sets only `r3 = obj`; `r4`/`r5` still hold `b`/`c`, which the shipped callee
ignores (Ghidra declares it with one parameter). The second call sets
`r3 = obj`, `r4 = a`, `r5 = 0`.

Both slots are on the object's **`+0x60` vtable**, not on `+0x00`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0C4EC` | vtable `+0x3F0` of `*(u32 *)(obj+0x60)` | `r3 = obj` (r4/r5 are the incoming `b`/`c`) |
| `0x80B0C518` | vtable `+0x3F4` of `*(u32 *)(obj+0x60)` | `r3 = obj`, `r4 = a`, `r5 = 0` |

### Ambiguity

The carrier at `c + 4` is only dereferenced; no field of it beyond `+4` is read.
The row is ready.

---

## 10. rank 96 - `0x80A441E0`, position copy from the `+0x9F8` link

* **TSV size:** 116 bytes
* **Reachable body:** 29 instructions; 1 `bl` + 0 `bctrl`; ends at
  `0x80A44250 blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a441e0_FUN_80a441e0.c`
* **Proposed signature:** `pub unsafe fn fn_80a441e0(this: usize)`
* **Callers in this REL:** `0x80A44174`
* **Confidence:** **medium / conditional**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `this`. No locals.

### Constants

| address | type | value / role |
|---|---|---|
| `0x8042B354` | **(ext)** | `main.dol` `f32`, subtracted from the copied `y` |

`0x8042B354` is built as `lis r4,0x8043` + `lfs f0,-19628(r4)`
(`0x80A44214` / `0x80A4421C`). It is below this image's base, so it lives in
`main.dol`; the value is not in the REL and `main.dol` is not part of this
checkout.

### Behaviour

```c
if (*(u8 *)(this + 0x9FC) != 0) {                 // 0x80A441F4 lbz 2556 / 0x80A441FC beq
    u32 src = fn_80162D00(*(u32 *)(this + 0x9F8)); // 0x80A44200 lwz 2552 / 0x80A44204 bl
    if (src != 0) {                                // 0x80A44208 cmpwi / 0x80A4420C beq
        *(f32 *)(this + 0xAC) = *(f32 *)(src + 0x7F8);   // 0x80A44210 lfs / 0x80A44218 stfs
        *(f32 *)(this + 0xB0) = *(f32 *)(src + 0x7FC);   // 0x80A44220 lfs f1 / 0x80A44224 stfs
        *(f32 *)(this + 0xB4) = *(f32 *)(src + 0x800);   // 0x80A4422C lfs / 0x80A44230 stfs
        *(f32 *)(this + 0xB0) = *(f32 *)(src + 0x7FC)
                              - *(f32 *)0x8042B354;      // 0x80A44228 fsubs / 0x80A44234 stfs
        *(u16 *)(this + 0x102) = *(u16 *)(src + 0x102);  // 0x80A44238 lha / 0x80A4423C sth
    }
}
```

`+0xB0` is written **twice**: first with the raw source `y`
(`0x80A44224`), then with `y - [0x8042B354]` (`0x80A44234`). The second store
wins; the first still exists in the shipped stream and should be kept if the
port is instruction-compared.

The `+0x102` copy uses `lha`/`sth` (sign-extended load, 16-bit store), i.e. the
low half is copied verbatim.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A44204` | `0x80162D00` | `r3 = *(u32 *)(this+0x9F8)` |

### Ambiguity

The `main.dol` `f32` at `0x8042B354` is consumed arithmetically and cannot be
read from this checkout. That is the one bounded step: read the constant from
`main.dol` (or from the corresponding `main-gb` data export) and the row becomes
ready.

---

## 11. rank 97 - `0x80A778F0`, four-float `+0x728..+0x734` table

* **TSV size:** 116 bytes
* **Reachable body:** 29 instructions; 0 `bl` + 0 `bctrl`; 1 `bc`; a tail
  `b 0x8007FB10` at `0x80A77960`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a778f0_FUN_80a778f0.c`
* **Proposed signature:** `pub unsafe fn fn_80a778f0(this: usize)`
* **Callers in this REL:** `0x80A77B78`, `0x80A78A40`
* **Confidence:** **high / ready**

### Frame

None. The body uses `r4` as the constant page (`lis r4,0x80B3` +
`addi r4,r4,-8984` = `0x80B2DCE8`) and never touches `r1`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2DDAC` | `f32` | `-24.0` - `+0x728` on the variant-4 path |
| `0x80B2DDB0` | `f32` | `24.0` - `+0x730` on the variant-4 path |
| `0x80B2DDB4` | `f32` | `34.0` - `+0x72C` on the variant-4 path |
| `0x80B2DD70` | `f32` | `0.0` - `+0x734` on both paths |
| `0x80B2DD84` | `f32` | `-8.0` - `+0x728` on the other path |
| `0x80B2DD74` | `f32` | `8.0` - `+0x730` on the other path |
| `0x80B2DD78` | `f32` | `16.0` - `+0x72C` on the other path |

Offsets from `0x80B2DCE8`: `+0x88`, `+0x8C`, `+0x90`, `+0x9C`, `+0xC4`,
`+0xC8`, `+0xCC`.

### Behaviour

```c
if (*(u32 *)(this + 0x88C) == 4) {           // 0x80A778F0 lwz 2188 / 0x80A778FC cmpli / bne
    *(f32 *)(this + 0x728) = -24.0f;         // 0x80A77914 stfs f3,1832
    *(f32 *)(this + 0x730) =  24.0f;         // 0x80A77918 stfs f2,1840
    *(f32 *)(this + 0x72C) =  34.0f;         // 0x80A7791C stfs f1,1836
    *(f32 *)(this + 0x734) =   0.0f;         // 0x80A77920 stfs f0,1844
} else {
    *(f32 *)(this + 0x728) = -8.0f;          // 0x80A77938
    *(f32 *)(this + 0x730) =  8.0f;          // 0x80A7793C
    *(f32 *)(this + 0x72C) = 16.0f;          // 0x80A77940
    *(f32 *)(this + 0x734) =  0.0f;          // 0x80A77944
}
return fn_8007FB10(this + 0x754, this, this + 0x728, 3,
                   *(u8 *)(this + 0x38F), this + 0x838);   // 0x80A77960 b 0x8007FB10
```

The `cmpli` is an **unsigned** compare against `4`. The store order is
`+0x728`, `+0x730`, `+0x72C`, `+0x734` (note `+0x72C` is written third), which
is the same order as the `lfs` loads at `0x80A77904..0x80A77910`
(`f3`, `f2`, `f1`, `f0`).

This is the same tail call as rank 91 with a different constant setup; the two
bodies are a matched pair (rank 91 writes one float, rank 97 writes four).

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A77960` | `0x8007FB10` (tail) | `r3 = this+0x754`, `r4 = this`, `r5 = this+0x728`, `r6 = 3`, `r7 = *(u8 *)(this+0x38F)`, `r8 = this+0x838` |

### Ambiguity

None in-REL; all seven constants are inside the image and quoted above. The
Ghidra export stops at `halt_baddata`; the `lbz`/`mr`/`addi`/`li`/`b` block at
`0x80A77948..0x80A77960` is the missing tail.

---

## 12. rank 98 - `0x80A1E100`, two writes into the `+0x698` record

* **TSV size:** 112 bytes
* **Reachable body:** 28 instructions; 2 `bl` + 0 `bctrl`; ends at
  `0x80A1E16C blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a1e100_FUN_80a1e100.c`
* **Proposed signature:** `pub unsafe fn fn_80a1e100(this: usize)`
* **Callers in this REL:** `0x80A1DEE0`, `0x80A1DF48`, `0x80A1DFF0`
* **Confidence:** **high / ready**

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`; `r31@+0x0C` = `this`. No locals, no FPR
saves.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2B6DC` | `f32` | `8.0` - `x` inset |
| `0x80B2B6E4` | `f32` | `16.0` - `y` lift |

Both are `lis r4,0x80B3` / `lis r5,0x80B3` plus negative offsets
(`-18716` -> `0x80B2B6E4`, `-18724` -> `0x80B2B6DC`).

### Behaviour

```c
fn_800146B0(*(f32 *)(this + 0xAC) - 8.0f,          // f1, 0x80A1E124 lfs f2 / 0x80A1E138 fsubs
            -(*(f32 *)(this + 0xB0) + 16.0f),      // f2, 0x80A1E120 lfs f0 / 0x80A1E128 fadds
                                                   //     0x80A1E13C fneg
            *(f32 *)(this + 0xB4),                 // f3, 0x80A1E130 lfs
            (void *)(this + 0x698));               // r3, 0x80A1E134 addi r3,r3,1688

fn_800146F0(*(f32 *)(this + 0xDC),                 // f1, 0x80A1E14C lfs 220
            (void *)(this + 0x698),                // r3, 0x80A1E148 addi
            (u16)(-(s16)*(s16 *)(this + 0x104))); // r4, 0x80A1E144 lha 260
                                                   //     0x80A1E150 neg / 0x80A1E154 extsh
```

The register/argument mapping is the important part, because Ghidra prints the
first call as `(double, double, double, int)` with the object as `param_4`.
The stream puts the **object pointer in `r3`** (`0x80A1E134 addi r3,r3,1688`)
and the three floats in `f1`/`f2`/`f3`; the callee stores them at
`r3+0xC / +0x10 / +0x14`, i.e. `this+0x6A4 / +0x6A8 / +0x6AC`.

The second call's integer arguments are `r3 = this+0x698` and
`r4 = -(s16)*(s16 *)(this+0x104)`, truncated to `u16`; the callee writes
`+0x0A = 2`, `f32 +0x18 = f1`, `u16 +0x1C = r4`, `+0x1E = 0`, i.e. into
`this+0x6A2`, `this+0x6B0`, `this+0x6B4`, `this+0x6B6`.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A1E140` | `0x800146B0` | `f1 = *(f32 *)(this+0xAC) - 8.0`, `f2 = -(*(f32 *)(this+0xB0) + 16.0)`, `f3 = *(f32 *)(this+0xB4)`, `r3 = this+0x698` |
| `0x80A1E158` | `0x800146F0` | `f1 = *(f32 *)(this+0xDC)`, `r3 = this+0x698`, `r4 = (u16)(-(s16)*(s16 *)(this+0x104))` |

### Ambiguity

None. The two `main.dol` callees are only written through; every operand is an
actor field or an in-REL constant.

---

## Cross-references and collisions

All twelve addresses were checked against the `address` column of
`ledger/functions.csv` (16,399 rows: absent) and against every `0x80Axxxxx` /
`0x80Bxxxxx` token in every `#`-heading of every `docs/*.md` (absent). They are
*named* in prose in earlier waves, always as a neighbour, caller or callee, never
as a specified body:

| address | where it is mentioned | in what role |
|---|---|---|
| `0x80AFEEB0` | `SPEC_WAVE133_ENEMIES.md` (rank 68 body) | "ends at `0x80AFEEB0 blr`" - the *end* of `0x80AFEDD0`, which is exactly this entry's first byte |
| `0x80A77BB0` | `SPEC_WAVE129_ENEMIES.md` (rank 19 body) | callee, `fn_80A77BB0(this)` |
| `0x80A778F0` | `SPEC_WAVE131_ENEMIES.md` (helper list) | "untranslated (B row, 116 bytes)" |
| `0x80A1E100` | `COVERAGE_WAVE133_ENEMIES.md` | coverage prose only |

`0x80AEEA30` is the only row in this tranche whose *reading* collides with an
earlier spec. `SPEC_WAVE134_ENEMIES.md` says of `0x80A79750` that the
`lfd` + `fsubs` pair collapses to `0.0` because `(f32)(2^52 + 4)` rounds back to
`2^52`. `SPEC_WAVE129`, `SPEC_WAVE131` and `SPEC_WAVE133` all read the same pair
as an exact conversion, and that is the reading used here (rank 92 above). The
two readings differ only in whether the bias subtraction survives; a port of
`0x80A79750` should settle it before either body is committed.

Three addresses named in wave 134's helper table are reused here unchanged:
`0x80066630`, `0x80064BD0`, `0x800690C0`. No new helper claim is made.

## Verification

* **Selection.** The twelve addresses are the first twelve non-ledger,
  non-specified B rows below wave 134's window under the wave-127 ordering:
  counts `34, 33, 33, 32, 31, 31, 31, 31, 30, 29, 29, 28`, sizes
  `136, 132, 132, 128, 124, 124, 124, 124, 120, 116, 116, 112`. The one
  intervening ledger row (`0x80A06B00`, 31) is skipped.
* **Sizes.** Every TSV size below matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and in every one of the
  twelve the reachable span equals `TSV size / 4` instructions with no trailing
  unreachable words.
* **Control flow.** 18 conditional branches, all with in-body targets; 6
  `bctrl`, all with a resolved vtable slot; 2 tail `b`s, both to `0x8007FB10`
  which is a `functions.tsv`/ledger entry; 12 terminating instructions (10
  `blr`, 2 `b`). No body reads `f1`/`f2`/`f3` before writing it, and there is
  no `psq` op anywhere in the tranche.
* **Constants.** Every in-REL constant quoted above was read from
  `work/rels/elf/d_enemiesNP.elf` at `(address - 0x80A00000) + 0x1000`; the
  `main.dol` ones are marked **(ext)**.
* **Ledger/doc state.** No ledger row, Rust file, CSV or other doc was written
  by this pass. `docs/SPEC_WAVE135_ENEMIES.md` is the only artefact.
