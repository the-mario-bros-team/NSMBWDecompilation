# Wave 138 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B** (reconstructible now), 160 class C
and 51 class D. Waves 128 - 137 (`docs/SPEC_WAVE128_ENEMIES.md` ..
`docs/SPEC_WAVE137_ENEMIES.md`) specified 120 of those B rows, twelve per wave.

This lane is the continuation of the same ranking. It specifies the twelve
highest-ranked class-B rows that are still genuinely **untranslated**: absent
from the `address` column of `ledger/functions.csv`, absent from every heading
of every `docs/*.md`, and with no `pub unsafe fn fn_<addr>` body in
`src/game/enemies.rs` (or any other Rust file). It is a **read-only** pass: it
writes no Rust, no ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000`
segment shift of `docs/REL_ELF_MAPPING.md` (first `PT_LOAD` is
`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`), cross-checked against the Ghidra
export at the cited path and against the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Constants that live inside
the REL image were read out of the ELF itself, so the float values below are
the shipped values. Addresses outside the file-backed image are marked
**(ext)**.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then by ascending address.

Wave 137's closing note named `0x80A2B490`, `0x80A6F8C0`, `0x80AA39D0` and
`0x80A22B20` as wave 138's candidates. That note is now stale: those four, plus
`0x80A61E10`, have `pub unsafe fn fn_...` bodies in `src/game/enemies.rs` at
the time of writing (they still have no `ledger/functions.csv` row, which is why
a ledger-only check still lists them). Removing them, and re-walking every
remaining class-B row, leaves **33** untranslated B rows. This page takes the
top twelve of those; the true next band is **not** the 12-instruction one.

| # | address | TSV size | reachable insns | owner / table row | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80AB1830` | 92 | 23 | fishbone per-frame chain, five direct callers | **ready** |
| 2 | `0x80A01700` | 80 | 20 | 24-frame effect timer, called by the untracked body at `0x80A005F0` | **ready** |
| 3 | `0x80B250F0` | 56 | 14 | global-gated effect emitter, called by `fn_80b25020` | **ready** |
| 4 | `0x80AEDC90` | 48 | 12 | vtable word `0x80B61BC8`, RailPolyParent block | **conditional** |
| 5 | `0x80B0C0D0` | 40 | 10 | vtable word `0x80B6805C`, `Wm_en_teresatail` block | **ready** |
| 6 | `0x80A0E160` | 36 | 9 | effect-band thunk, called by `fn_80a0e450` | **ready** |
| 7 | `0x80AD42B0` | 36 | 9 | state transition, called by `fn_80ad3d50` | **ready** |
| 8 | `0x80B07C90` | 36 | 9 | effect-band thunk, called by `fn_80b07650` | **ready** |
| 9 | `0x80A281C0` | 32 | 8 | descriptor binder, called by `fn_80a29010` | **ready** |
| 10 | `0x80A337C0` | 32 | 8 | effect emitter, reached through the 4-byte thunk `0x80A337B0` | **ready** |
| 11 | `0x80A36A50` | 32 | 8 | side facing gate, called by `fn_80a37700` / `fn_80a37830` | **ready** |
| 12 | `0x80A3F630` | 32 | 8 | three-range guard, called by the untracked body at `0x80A3EA20` | **ready** |

**Split: 11 ready, 1 conditional.** Total 552 bytes, 138 reachable
instructions.

The four risk classes that made waves 110-126 skip a body are absent or
bounded:

* **no paired-single traffic.** No `psq_l`/`psq_st`, no `psq_*`, no `bctr`
  jump table, no `ldexpf` pseudo-register in any of the twelve bodies.
* **no incoming float argument.** Every float is loaded from an actor field or
  from an in-image constant before it is used; no body reads `f1`-`f4` on
  entry. Ranks 1, 2, 3 and 10 read `f32` actor fields, never the ABI.
* **no unresolved callee landmine.** Every direct `bl`/`b` target is either a
  ledgered `main.dol` function (`0x8008C380`, `0x8015F4F0`) or one of the five
  external targets whose call-site shapes are fixed below (`0x80078470`,
  `0x800D8B00`, `0x80022100`, `0x80027A60`, `0x8006D090`).
* **one indirect call only** (rank 4). Its receiver is known
  (`*(this + 0x394)`) and the slot is known (`+0x10`); the only missing datum
  is that slot's argument list, which is why rank 4 is the sole conditional.

The remaining 21 untranslated class-B rows are all 7 instructions or fewer.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000`.
* `this` is the incoming `r3` unless stated otherwise. All `+0xNNN` are byte
  offsets from `this`.
* `addi rX,r0,imm` is `li rX,imm` and `addis rX,r0,imm` is `lis rX,imm`
  (`r0` is not read when `RA == 0`). This is decisive for ranks 1, 2, 6, 8 and
  10 (and for the `li r3,1` return at rank 5): an `addi r5,r0,7` is the literal
  `7`, not `byte + 7`.
* `bc 12,2` = `beq`, `bc 4,2` = `bne`, `bclr 20,0` = `blr`. In this build a
  `bclr 4,2` is a conditional return (`bnelr`): it returns when CR0[EQ] is
  clear, i.e. when the last setting instruction produced a non-zero value.
  `rlwinm.` (Rc = 1) is what sets that bit at ranks 3 and 7.
* `lhax rD,rA,rB` is the repo's name for the indexed signed-halfword load
  (`XO 343`, word `0x7C8402AE` at rank 11); it loads
  `*(i16 *)(rA + rB)` and sign-extends to 32 bits. The table at
  `0x8042C1A4` is a signed-halfword pair, so this is a halfword load, not a
  word load.
* `0x80078470` has no `main.dol` export. Its call shape is fixed by every
  translated user in this module: `(f32 x, f32 y, r3 manager, r4 layer,
  r5 count)`. The `x`/`y` operands are **single-precision** - all call sites
  use `lfs`/`fadds`. Do not reuse the `call5_f64x2` helper, which exists for
  the unrelated `0x800B1100` shape.
* `0x800D8B00` is ledgered (`engine::display`, 164 bytes) with signature
  `(object, player: u32, row: i32, bands: u32, tails: u32)`. The two thunks
  here also set `r8 = 0`; the callee does not read it, but the repo's
  convention is to model it with `call6`.
* The C snippets use `call5_f32x2(addr, f1, f2, a, b, c)` for the
  five-argument effect shape (`f32` in `f1`/`f2`, then `r3`/`r4`/`r5`) and
  `call_indirect(slot, arg)` for the single `bctrl`. Neither helper exists in
  `src/game/enemies.rs` today; both are trivial wrappers over
  `core::mem::transmute`, and the existing `call5_f64x2` must not be reused
  for the f32 shape.

## Symbolic helpers used below

| address | role |
|---|---|
| `0x80078470` | **(ext)** three-layer stage-effect request `(f32 x, f32 y, r3 manager, r4 layer, r5 count)`; no export (see `docs/COVERAGE_WAVE28_ENEMIES.md`) |
| `0x800D8B00` | ledgered `fn_800d8b00(object, player, row, bands, tails)`; writes the two effect-record bands (`src/engine/display.rs`) |
| `0x80022100` | **(ext)** state-table transition `(this, record, signed mode byte)`; named in `src/game/enemies.rs` as `W133_FN_80022100` |
| `0x80027A60` | **(ext)** per-frame teardown step `(this)`; grouped with `0x80036A70`/`0x80027D70` in `docs/SPEC_WAVE133_ENEMIES.md` |
| `0x8006D090` | **(ext)** record binder `(record, owner, d1, d2, d3)`; export `work/ghidra/main-gb/export/8006d090_FUN_8006d090.c` |
| `0x8008C380` | ledgered `collision_area_unregister(area)` (`game::stage`) |
| `0x8015F4F0` | ledgered `step_short_clamped_wrap(*mut i16, target: i32, step: i32) -> u32` (`game::task`); returns 1 on arrival |
| `0x80429DD0` | **(ext, BSS)** pointer to the shared effect manager object (`u32` word). Read in this module as `platform::read_u32(0x8042_9DD0)` |
| `0x8042A008` | **(ext, BSS)** handle of the shared display/effect object (`BURST_DAT_8042A008` in `src/game/enemies.rs`) |
| `0x8042A200` | **(ext, BSS)** global frame/state word; bit 0 gates the effect at rank 3 |

---

## 1. `0x80AB1830` - fishbone 64-frame effect timer

* **Address:** `0x80AB1830`
* **TSV size:** 92 bytes
* **Reachable body:** 23 instructions; 1 `beq`; 1 `bnelr`; ends in a tail `b`
  to `0x80078470`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab1830_FUN_80ab1830.c`
  - **truncated** (`halt_baddata`), so the export is not usable as a plan; the
    decoded body below is authoritative.
* **Proposed signature:** `pub unsafe fn fn_80ab1830(this: usize)`
* **Callers in this REL:** five direct `bl`, all with `r3 = this`:
  `0x80AB2180` (`fn_80ab2050`), `0x80AB2268` (`fn_80ab21d0`),
  `0x80AB23C4` (`fn_80ab22d0`), `0x80AB25FC` (`fn_80ab2460`),
  `0x80AB2818` (`fn_80ab26a0`). No 32-bit word references the entry.
* **Constants:** timer reload `64`; side table `0x80B2FB30`; manager word
  `0x80429DD0`.
* **Verdict:** **ready** (confidence high)

### Frame

None. The body is a leaf with no `stwu`, no LR save and no callee-saved
register: every path either returns through `bnelr` or tail-branches.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2FB30` | `f32[4]` | `{6.0, -6.0, -0.000244140625, 0.000732421875}` - per-side x offset added to `+0x884`. File offset `0x12FB30` |
| `0x80429DD0` | `u32` **(ext, BSS)** | effect-manager pointer passed to `0x80078470` |
| `64` | literal | `li r6,64` - timer reload written to `+0x984` |

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lwz r4,0x984(r3)` |
| `+0x04` | `cmpwi cr0,r4,0` |
| `+0x08` | `beq +0x14` (`0x80AB1844`) |
| `+0x0C` | `addi r0,r4,-1` |
| `+0x10` | `stw r0,0x984(r3)` |
| `+0x14` | `lwz r0,0x984(r3)` |
| `+0x18` | `cmpwi cr0,r0,0` |
| `+0x1C` | `bnelr` |
| `+0x20` | `lbz r0,0x348(r3)` |
| `+0x24` | `lis r4,0x80B3` |
| `+0x28` | `li r6,64` |
| `+0x2C` | `lfs f1,0x884(r3)` |
| `+0x30` | `rlwinm r0,r0,2,0,29` |
| `+0x34` | `addi r4,r4,-0x4D0` |
| `+0x38` | `lfsx f0,r4,r0` |
| `+0x3C` | `lis r5,0x8043` |
| `+0x40` | `stw r6,0x984(r3)` |
| `+0x44` | `li r4,1` |
| `+0x48` | `fadds f1,f1,f0` |
| `+0x4C` | `lfs f2,0x888(r3)` |
| `+0x50` | `lwz r3,-0x6230(r5)` |
| `+0x54` | `li r5,1` |
| `+0x58` | `b 0x80078470` |

### Behaviour

```c
void fn_80ab1830(usize this) {
    u32 t = read_u32(this + 0x984);
    if (t != 0) write_u32(this + 0x984, t - 1);
    if (read_u32(this + 0x984) != 0) return;       // bnelr
    let side = read_u8(this + 0x348) as u32;
    write_u32(this + 0x984, 64);
    let x = read_f32(this + 0x884) + read_f32(0x80B2FB30 + 4 * side);
    let y = read_f32(this + 0x888);
    call5_f32x2(0x80078470, x, y, read_u32(0x80429DD0), 1, 1);
}
```

* The first `lwz`/`cmpwi` pair clamps a non-zero timer down by one; the second
  pair tests the **new** value. The effect therefore fires on the call where
  the stored timer is zero (including the first call, when the field is still
  zero) and reloads 64.
* `li r6,64` is a literal `64`, not `side + 64`.
* The side byte is not bounds-checked. `rlwinm r0,r0,2,0,29` allows `0..255`;
  indices `>= 4` read past `0x80B2FB30` into the following words. The shipped
  code does exactly that; do not add a clamp.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AB1888` | `0x80078470` (tail `b`) | `f1 = +0x884 + table[side]`, `f2 = +0x888`, `r3 = *(0x80429DD0)`, `r4 = 1`, `r5 = 1` |

### Ambiguity

None. The Ghidra export reproduces the whole countdown and stops only at the
tail branch; the decoded listing above is authoritative for the tail call.
`0x80078470` has no export, but the four other translated callers in this
module fix its shape.

---

## 2. `0x80A01700` - 24-frame effect timer

* **Address:** `0x80A01700`
* **TSV size:** 80 bytes
* **Reachable body:** 20 instructions; 1 `beq`; 1 `bnelr`; ends in a tail `b`
  to `0x80078470`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a01700_FUN_80a01700.c`
  - **truncated** (`halt_baddata`), same cosmetic truncation as rank 1.
* **Proposed signature:** `pub unsafe fn fn_80a01700(this: usize)`
* **Callers in this REL:** one direct `bl`, `0x80A00658` with `r3 = this`, in
  the **untracked** body that starts at `0x80A005F0` (no `functions.tsv`
  row; the preceding function ends at `0x80A005E4`, and the body ends at
  `0x80A0068C`). No 32-bit word references the entry.
* **Constants:** timer reload `24`; float `0x80B2A860 = 24.0f`; manager word
  `0x80429DD0`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf, no `stwu`; both exits are `bnelr` or a tail branch.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2A860` | `f32` | `24.0` - base x offset added to `+0x8EC`. File offset `0x12B860` |
| `0x80429DD0` | `u32` **(ext, BSS)** | effect-manager pointer |
| `24` | literal | `li r0,24` - timer reload written to `+0x918` |

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lwz r4,0x918(r3)` |
| `+0x04` | `cmpwi cr0,r4,0` |
| `+0x08` | `beq +0x14` (`0x80A01714`) |
| `+0x0C` | `addi r0,r4,-1` |
| `+0x10` | `stw r0,0x918(r3)` |
| `+0x14` | `lwz r0,0x918(r3)` |
| `+0x18` | `cmpwi cr0,r0,0` |
| `+0x1C` | `bnelr` |
| `+0x20` | `lis r4,0x80B3` |
| `+0x24` | `li r0,24` |
| `+0x28` | `lfs f1,-0x57A0(r4)` |
| `+0x2C` | `lis r6,0x8043` |
| `+0x30` | `lfs f0,0x8EC(r3)` |
| `+0x34` | `li r4,3` |
| `+0x38` | `stw r0,0x918(r3)` |
| `+0x3C` | `li r5,1` |
| `+0x40` | `lfs f2,0x8F0(r3)` |
| `+0x44` | `fadds f1,f1,f0` |
| `+0x48` | `lwz r3,-0x6230(r6)` |
| `+0x4C` | `b 0x80078470` |

### Behaviour

```c
void fn_80a01700(usize this) {
    u32 t = read_u32(this + 0x918);
    if (t != 0) write_u32(this + 0x918, t - 1);
    if (read_u32(this + 0x918) != 0) return;
    write_u32(this + 0x918, 24);
    let x = read_f32(0x80B2A860) + read_f32(this + 0x8EC);   // 24.0f + field
    let y = read_f32(this + 0x8F0);
    call5_f32x2(0x80078470, x, y, read_u32(0x80429DD0), 3, 1);
}
```

* Same countdown polarity as rank 1.
* `lis r6,0x8043` / `lwz r3,-0x6230(r6)` resolves to `*(u32 *)0x80429DD0`.
  The `li r0,24` does not feed the manager register.
* The layer argument is the literal `3`. Other callers of `0x80078470` use
  `0/1/2`; this body passes `3` and must keep it.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A0174C` | `0x80078470` (tail `b`) | `f1 = 24.0f + *(f32 *)(this+0x8EC)`, `f2 = *(f32 *)(this+0x8F0)`, `r3 = *(0x80429DD0)`, `r4 = 3`, `r5 = 1` |

### Ambiguity

None in the body. The layer value `3` is outside the `0..2` range that
`docs/COVERAGE_WAVE28_ENEMIES.md` observed at other call sites; that is a
property of this call site, not a missing datum.

---

## 3. `0x80B250F0` - global-gated effect emitter

* **Address:** `0x80B250F0`
* **TSV size:** 56 bytes
* **Reachable body:** 14 instructions; `rlwinm.` + `bnelr` gate; ends in a
  tail `b` to `0x80078470`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b250f0_FUN_80b250f0.c`
  - **truncated** (`halt_baddata`) after the gate; the listing below is the
    complete body.
* **Proposed signature:** `pub unsafe fn fn_80b250f0(this: usize)`
* **Callers in this REL:** one direct `bl`, `0x80B250C0` with `r3 = this`,
  inside `fn_80b25020` (TSV 0xCC bytes). No 32-bit word references the entry.
* **Constants:** gate word `0x8042A200` bit 0; manager `0x80429DD0`; actor
  fields `+0x524`, `+0xAC`, `+0xB0`; layer bias `10`; count `2`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lis r4,0x8043` |
| `+0x04` | `mr r6,r3` |
| `+0x08` | `lwz r0,-0x5E00(r4)` |
| `+0x0C` | `rlwinm. r0,r0,0,31,31` |
| `+0x10` | `bnelr` |
| `+0x14` | `lwz r4,0x524(r6)` |
| `+0x18` | `lis r3,0x8043` |
| `+0x1C` | `lwz r3,-0x6230(r3)` |
| `+0x20` | `li r5,2` |
| `+0x24` | `addi r0,r4,10` |
| `+0x28` | `lfs f1,0xAC(r6)` |
| `+0x2C` | `lfs f2,0xB0(r6)` |
| `+0x30` | `rlwinm r4,r0,0,24,31` |
| `+0x34` | `b 0x80078470` |

### Behaviour

```c
void fn_80b250f0(usize this) {
    if ((read_u32(0x8042A200) & 1) != 0) return;      // rlwinm. + bnelr
    let layer = read_u32(this + 0x524).wrapping_add(10) & 0xFF;
    call5_f32x2(0x80078470,
                read_f32(this + 0xAC),
                read_f32(this + 0xB0),
                read_u32(0x80429DD0),
                layer,
                2);
}
```

* `rlwinm. r0,r0,0,31,31` isolates bit 0 and sets CR0[EQ] = (bit == 0); the
  `bnelr` returns when that bit is set. The effect fires only when
  `0x8042A200 & 1 == 0`.
* `mr r6,r3` keeps the actor in `r6`; every field read uses `r6`.
* The layer is the low byte of `*(u32 *)(this+0x524) + 10`; the `+10` is
  literal and is added before the byte mask.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B25124` | `0x80078470` (tail `b`) | `f1 = *(f32 *)(this+0xAC)`, `f2 = *(f32 *)(this+0xB0)`, `r3 = *(0x80429DD0)`, `r4 = (u8)(*(u32 *)(this+0x524)+10)`, `r5 = 2` |

### Ambiguity

None. `0x8042A200` is a main.dol BSS word with no file bytes; only its bit-0
role is used, and that is literal in the stream.

---

## 4. `0x80AEDC90` - RailPolyParent vtable word `0x80B61BC8`: forward to the `+0x394` sub-object

* **Address:** `0x80AEDC90`
* **TSV size:** 48 bytes
* **Reachable body:** 12 instructions; no `bc`; one `bctrl`; ends in `blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aedc90_FUN_80aedc90.c`
* **Proposed signature:** `pub unsafe fn fn_80aedc90(this: usize) -> u32`
* **Callers in this REL:** none by direct `bl`/`b`. The only reference is the
  32-bit word at `0x80B61BC8`, between `0x800643E0` and `0x800643F0`, in the
  function-pointer block that begins at `0x80B61BB0` (the surrounding name
  block at `0x80B61D14` is `daEnRailPolyParent_c::StateID_RailMoveWait`; id
  pair `0x01E9_01EF` at `0x80B61B98`). The word is slot `+0x18` of that block
  if the block base is `0x80B61BB0`; the port should bind by **word address**,
  not by slot number, because the base is inferred from the block boundary.
* **Constants:** sub-object offset `+0x394`; vtable slot offset `+0x10`;
  return literal `1`.
* **Verdict:** **conditional** (body confidence high; one ABI datum missing)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `stwu r1,-0x10(r1)` |
| `+0x04` | `mflr r0` |
| `+0x08` | `stw r0,0x14(r1)` |
| `+0x0C` | `lwzu r12,0x394(r3)` |
| `+0x10` | `lwz r12,0x10(r12)` |
| `+0x14` | `mtctr r12` |
| `+0x18` | `bctrl` |
| `+0x1C` | `lwz r0,0x14(r1)` |
| `+0x20` | `li r3,1` |
| `+0x24` | `mtlr r0` |
| `+0x28` | `addi r1,r1,0x10` |
| `+0x2C` | `blr` |

### Behaviour

```c
u32 fn_80aedc90(usize this) {
    u32 obj = read_u32(this + 0x394);        // r12, and r3 becomes this+0x394
    let slot = read_u32(obj + 0x10);
    call_indirect(slot, this + 0x394);       // r4..r10 forwarded unchanged
    return 1;                                // li r3,1 after the call
}
```

* `lwzu r12,0x394(r3)` loads `r12 = *(u32 *)(this + 0x394)` and updates `r3`
  to `this + 0x394`; the virtual call therefore receives the sub-object, not
  the actor.
* The receiver-displacement chain is `*(u32 *)(this + 0x394)` then slot
  `+0x10`. The same `+0x394` sub-object is used by
  `docs/SPEC_WAVE128_ENEMIES.md`'s `kind_test` helper (slot `+0x28` then
  `+0x14`), so the receiver is not an unresolved base.
* `li r3,1` overwrites the callee's return; the wrapper always returns 1.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AEDAC8` | `*(u32 *)(*(u32 *)(this+0x394) + 0x10)` (`bctrl`) | `r3 = this + 0x394`; `r4..r10` are passed through unchanged |

### Ambiguity

**The one missing datum is the argument list of slot `+0x10`.** Ghidra prints
`(**(code **)(*(int *)(param_1 + 0x394) + 0x10))()` with no arguments, which
is consistent with a no-argument method but does not prove it. The port is
byte-exact once that slot is known to take no arguments beyond its receiver;
until then it is **conditional**. If a later wave finds an argument, extend
the Rust signature rather than dropping the pass-through.

---

## 5. `0x80B0C0D0` - Teresa-tail collision-area release

* **Address:** `0x80B0C0D0`
* **TSV size:** 40 bytes
* **Reachable body:** 10 instructions; one `bl`; ends in `blr`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b0c0d0_FUN_80b0c0d0.c`
* **Proposed signature:** `pub unsafe fn fn_80b0c0d0(this: usize) -> u32`
* **Callers in this REL:** none by direct `bl`/`b`. The only reference is the
  32-bit word at `0x80B6805C`, between `0x800951D0` and `0x80064390`, in the
  function-pointer block that begins at `0x80B68050`, immediately after the
  model name `Wm_en_teresatail` at `0x80B68034`.
* **Constants:** collision-area offset `+0x8D4`; return literal `1`.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-0x10(r1)`; LR at `+0x14`.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `stwu r1,-0x10(r1)` |
| `+0x04` | `mflr r0` |
| `+0x08` | `addi r3,r3,0x8D4` |
| `+0x0C` | `stw r0,0x14(r1)` |
| `+0x10` | `bl 0x8008C380` |
| `+0x14` | `lwz r0,0x14(r1)` |
| `+0x18` | `li r3,1` |
| `+0x1C` | `mtlr r0` |
| `+0x20` | `addi r1,r1,0x10` |
| `+0x24` | `blr` |

### Behaviour

```c
u32 fn_80b0c0d0(usize this) {
    collision_area_unregister(this + 0x8D4);   // 0x8008C380
    return 1;
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B0C0E0` | `0x8008C380` (`collision_area_unregister`) | `r3 = this + 0x8D4` |

### Ambiguity

None. The Ghidra export is clean and agrees line for line.

---

## 6. `0x80A0E160` - effect-band thunk with a signed player byte

* **Address:** `0x80A0E160`
* **TSV size:** 36 bytes
* **Reachable body:** 9 instructions; no `bc`; ends in a tail `b` to
  `0x800D8B00`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a0e160_FUN_80a0e160.c`
  - **truncated** (`halt_baddata`); the nine words below are the whole body.
* **Proposed signature:** `pub unsafe fn fn_80a0e160(this: usize)`
* **Callers in this REL:** one direct `bl`, `0x80A0E54C` with `r3 = this`,
  inside `fn_80a0e450` (TSV 276 bytes). No 32-bit word references the entry.
* **Constants:** player byte `+0x614`; row `7`; bands `2`; tails `0`;
  object handle `0x8042A008`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf, tail branch only.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lis r4,0x8043` |
| `+0x04` | `lbz r0,0x614(r3)` |
| `+0x08` | `lwz r3,-0x5FF8(r4)` |
| `+0x0C` | `li r5,7` |
| `+0x10` | `li r6,2` |
| `+0x14` | `extsb r4,r0` |
| `+0x18` | `li r7,0` |
| `+0x1C` | `li r8,0` |
| `+0x20` | `b 0x800D8B00` |

### Behaviour

```c
void fn_80a0e160(usize this) {
    let b = read_u8(this + 0x614);
    call6(0x800D8B00,
          read_u32(0x8042A008),      // object
          (b as i8) as i32 as u32,   // player, sign-extended byte
          7,                         // row
          2,                         // bands
          0,                         // tails
          0);                        // r8, not read by the callee
}
```

* `lis r4,0x8043` + `lwz r3,-0x5FF8(r4)` loads the shared display object from
  `0x8042A008`. `r4` is then overwritten by `extsb`.
* `li r5,7` is a literal row of `7`; it is not `byte + 7`.
* `extsb r4,r0` sets the player argument to the **signed** byte. The callee
  forwards `player as u8` to the band writers, so the two representations are
  deliberately different in the middle register.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A0E180` | `0x800D8B00` (tail `b`) | `r3 = *(0x8042A008)`, `r4 = (i8)*(u8 *)(this+0x614)`, `r5 = 7`, `r6 = 2`, `r7 = 0`, `r8 = 0` |

### Ambiguity

None. `0x800D8B00` is ledgered and its five-parameter Rust body matches this
call; the `r8 = 0` is the same extra register that the existing
`BURST_FN_800D8B00` call site models with `call6`.

---

## 7. `0x80AD42B0` - state transition when `+0x84C` is `0` or `2`

* **Address:** `0x80AD42B0`
* **TSV size:** 36 bytes
* **Reachable body:** 9 instructions; one `beq`, one `bnelr`; ends in a tail
  `b` to `0x80022100`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad42b0_FUN_80ad42b0.c`
  - **truncated** after the two guards; the listing below is complete.
* **Proposed signature:** `pub unsafe fn fn_80ad42b0(this: usize)`
* **Callers in this REL:** one direct `bl`, `0x80AD3F5C` with `r3 = this`,
  inside `fn_80ad3d50` (TSV 688 bytes). No 32-bit word references the entry.
* **Constants:** state word `+0x84C`; transition record `+0x854`; mode byte
  `+0x694`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lwz r0,0x84C(r3)` |
| `+0x04` | `cmpwi cr0,r0,0` |
| `+0x08` | `beq +0x14` (`0x80AD42C4`) |
| `+0x0C` | `cmpwi cr0,r0,2` |
| `+0x10` | `bnelr` |
| `+0x14` | `lbz r0,0x694(r3)` |
| `+0x18` | `addi r4,r3,0x854` |
| `+0x1C` | `extsb r5,r0` |
| `+0x20` | `b 0x80022100` |

### Behaviour

```c
void fn_80ad42b0(usize this) {
    let m = read_u32(this + 0x84C);
    if (m != 0 && m != 2) return;              // beq to the body, else bnelr
    call3(0x80022100,
          this,                                 // actor
          this + 0x854,                         // transition record
          (read_u8(this + 0x694) as i8) as i32 as u32);
}
```

* The two compares are signed `cmpwi`; the accepted values are exactly `0` and
  `2`.
* `extsb r5,r0` is the mode byte, sign-extended. The callee is the same
  `0x80022100` used elsewhere in this module as `W133_FN_80022100`
  ("state-table transition with a signed mode byte").

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80AD42D0` | `0x80022100` (tail `b`) | `r3 = this`, `r4 = this + 0x854`, `r5 = (i8)*(u8 *)(this+0x694)` |

### Ambiguity

None. The gate values and the three call arguments are literal.

---

## 8. `0x80B07C90` - second effect-band thunk

* **Address:** `0x80B07C90`
* **TSV size:** 36 bytes
* **Reachable body:** 9 instructions; no `bc`; ends in a tail `b` to
  `0x800D8B00`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b07c90_FUN_80b07c90.c`
  - **truncated** (`halt_baddata`); the nine words below are the whole body.
* **Proposed signature:** `pub unsafe fn fn_80b07c90(this: usize)`
* **Callers in this REL:** one direct `bl`, `0x80B077A8` with `r3 = this`,
  inside `fn_80b07650` (TSV 0x22C bytes). No 32-bit word references the entry.
* **Constants:** player byte `+0x6C2`; row `7`; bands `2`; tails `0`;
  object handle `0x8042A008`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf, tail branch only.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lis r4,0x8043` |
| `+0x04` | `lbz r0,0x6C2(r3)` |
| `+0x08` | `lwz r3,-0x5FF8(r4)` |
| `+0x0C` | `li r5,7` |
| `+0x10` | `li r6,2` |
| `+0x14` | `extsb r4,r0` |
| `+0x18` | `li r7,0` |
| `+0x1C` | `li r8,0` |
| `+0x20` | `b 0x800D8B00` |

### Behaviour

```c
void fn_80b07c90(usize this) {
    let b = read_u8(this + 0x6C2);
    call6(0x800D8B00,
          read_u32(0x8042A008),
          (b as i8) as i32 as u32,
          7, 2, 0, 0);
}
```

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80B07CB0` | `0x800D8B00` (tail `b`) | `r3 = *(0x8042A008)`, `r4 = (i8)*(u8 *)(this+0x6C2)`, `r5 = 7`, `r6 = 2`, `r7 = 0`, `r8 = 0` |

### Ambiguity

None. Instruction-for-instruction identical to rank 6 except the byte offset
(`+0x6C2` instead of `+0x614`).

---

## 9. `0x80A281C0` - bind the `+0x1EC` placement record

* **Address:** `0x80A281C0`
* **TSV size:** 32 bytes
* **Reachable body:** 8 instructions; no `bc`; ends in a tail `b` to
  `0x8006D090`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a281c0_FUN_80a281c0.c`
  - **truncated** (`halt_baddata`); the eight words below are the whole body.
* **Proposed signature:** `pub unsafe fn fn_80a281c0(this: usize)`
* **Callers in this REL:** one direct `bl`, `0x80A2903C` with `r3 = this`,
  inside `fn_80a29010` (TSV 76 bytes). No 32-bit word references the entry.
* **Constants:** record `+0x1EC`; descriptor addresses `0x80B2B87C`,
  `0x80B2B888`, `0x80B2B898`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf, tail branch only.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2B87C` | `u32[3]` | `{0x00000000, 0x00000000, 0x00010000}` (file offset `0x12C87C`) |
| `0x80B2B888` | `u32[3]` | `{0x00000001, 0xFFFFD000, 0x00003000}` (i.e. `{1, -0x3000, 0x3000}`) |
| `0x80B2B898` | `u32[3]` | `{0x00000000, 0x00006000, 0x00006000}` |

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lis r7,0x80B3` |
| `+0x04` | `mr r4,r3` |
| `+0x08` | `addi r7,r7,-0x4798` |
| `+0x0C` | `addi r3,r3,0x1EC` |
| `+0x10` | `addi r5,r7,0x20` |
| `+0x14` | `addi r6,r7,0x14` |
| `+0x18` | `addi r7,r7,0x30` |
| `+0x1C` | `b 0x8006D090` |

### Behaviour

```c
void fn_80a281c0(usize this) {
    let base = 0x80B2B868;             // lis 0x80B3 ; addi -0x4798
    call5(0x8006D090,
          this + 0x1EC,                 // record
          this,                         // owner
          0x80B2B888,                   // base + 0x20
          0x80B2B87C,                   // base + 0x14
          0x80B2B898);                  // base + 0x30
}
```

* The three trailing addresses are static descriptors. `0x8006D090` stores
  them at `+0x08`, `+0x0C` and `+0x10` of the record and links the record back
  to the owner at `+0x14..+0x1C`
  (`work/ghidra/main-gb/export/8006d090_FUN_8006d090.c`).
* `0x8006D090` also calls the owner's vtable (`*(u32 *)(owner + 0x60)`) slot
  `+0x6C` and copies its first byte to `record + 0x98`; the port must call the
  real function, not inline a partial store.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A281DC` | `0x8006D090` (tail `b`) | `r3 = this+0x1EC`, `r4 = this`, `r5 = 0x80B2B888`, `r6 = 0x80B2B87C`, `r7 = 0x80B2B898` |

### Ambiguity

None. The descriptor words are in the file-backed image and were read
directly. `0x8006D090` has a clean main.dol export.

---

## 10. `0x80A337C0` - effect emitter, layer 2 / count 4

* **Address:** `0x80A337C0`
* **TSV size:** 32 bytes
* **Reachable body:** 8 instructions; no `bc`; ends in a tail `b` to
  `0x80078470`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a337c0_FUN_80a337c0.c`
  - **truncated** (`halt_baddata`); the eight words below are the whole body.
* **Proposed signature:** `pub unsafe fn fn_80a337c0(this: usize)`
* **Callers in this REL:** no direct `bl`. Reached through the 4-byte thunk at
  `0x80A337B0` (`b 0x80A337C0`), which is itself called at `0x80A3351C` inside
  `fn_80a334d0`. No 32-bit word references either entry.
* **Constants:** `+0x6CC`, `+0x6D0`; manager `0x80429DD0`; layer `2`;
  count `4`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf, tail branch only.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `mr r6,r3` |
| `+0x04` | `lis r3,0x8043` |
| `+0x08` | `lwz r3,-0x6230(r3)` |
| `+0x0C` | `li r4,2` |
| `+0x10` | `lfs f1,0x6CC(r6)` |
| `+0x14` | `li r5,4` |
| `+0x18` | `lfs f2,0x6D0(r6)` |
| `+0x1C` | `b 0x80078470` |

### Behaviour

```c
void fn_80a337c0(usize this) {
    call5_f32x2(0x80078470,
                read_f32(this + 0x6CC),
                read_f32(this + 0x6D0),
                read_u32(0x80429DD0),
                2,
                4);
}
```

* `mr r6,r3` keeps the actor while `r3` is replaced with the manager pointer.
* The layer and count are literals (`2` and `4`).

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A337DC` | `0x80078470` (tail `b`) | `f1 = *(f32 *)(this+0x6CC)`, `f2 = *(f32 *)(this+0x6D0)`, `r3 = *(0x80429DD0)`, `r4 = 2`, `r5 = 4` |
| `0x80A3351C` | `0x80A337B0` (4-byte `b`) | `r3 = this`; the thunk forwards to this body |

### Ambiguity

None. The only indirection is the 4-byte `b` thunk, which is already recorded
as class D in wave 127 and needs no separate body.

---

## 11. `0x80A36A50` - side facing gate toward ±0x2000

* **Address:** `0x80A36A50`
* **TSV size:** 32 bytes
* **Reachable body:** 8 instructions; no `bc`; ends in a tail `b` to
  `0x8015F4F0`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a36a50_FUN_80a36a50.c`
  - **truncated** (`halt_baddata`); the listing below is complete.
* **Proposed signature:** `pub unsafe fn fn_80a36a50(this: usize) -> u32`
* **Callers in this REL:** three direct `bl`, all with `r3 = this`:
  `0x80A377B8` (`fn_80a37700`), `0x80A37968` and `0x80A37C50`
  (`fn_80a37830`). No 32-bit word references the entry.
* **Constants:** side byte `+0x348`; facing pointer `+0x102`; table
  `0x8042C1A4`; step `0x600`.
* **Verdict:** **ready** (confidence high)

### Frame

None. Leaf, tail branch only.

### Constants

| address | type | value / role |
|---|---|---|
| `0x8042C1A4` | `i16[2]` **(ext)** | `{+0x2000, -0x2000}` - the two ±45-degree facings. The words after the pair are `{+0x0400, -0x0400}`, but the shipped body only indexes the first pair |

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lbz r0,0x348(r3)` |
| `+0x04` | `lis r4,0x8043` |
| `+0x08` | `addi r4,r4,-0x3E5C` |
| `+0x0C` | `li r5,0x600` |
| `+0x10` | `rlwinm r0,r0,1,0,30` |
| `+0x14` | `addi r3,r3,0x102` |
| `+0x18` | `lhax r4,r4,r0` |
| `+0x1C` | `b 0x8015F4F0` |

### Behaviour

```c
u32 fn_80a36a50(usize this) {
    let side = read_u8(this + 0x348) as u32;      // actor side 0/1
    let target = read_i16(0x8042C1A4 + 2 * side) as i32;
    return step_short_clamped_wrap((this + 0x102) as *mut i16, target, 0x600);
}
```

* `lhax` (word `0x7C8402AE`, XO 343 in this build) sign-extends the selected
  halfword; the repo's decoder and `docs/SPEC_WAVE129_BASES.md` /
  `docs/SPEC_WAVE132_ENEMIES.md` call this form `lhax`.
* `step_short_clamped_wrap` is ledgered (`game::task`). It compares the
  incoming `i16` with `target as i16`, steps by ±`0x600` toward it and returns
  `1` when it snaps, `0` otherwise. The return value is the function's return
  value (tail branch, no masking).
* `addi r3,r3,0x102` is the address of the `i16` facing field; the step is
  the literal `0x600` (1536), not a field.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A36A6C` | `0x8015F4F0` (tail `b`) | `r3 = this+0x102`, `r4 = (s32)*(s16 *)(0x8042C1A4 + 2*side)`, `r5 = 0x600` |

### Ambiguity

None. The table pair and the step are literal; the callee is ledgered and its
exact semantics are already in `src/game/task.rs`.

---

## 12. `0x80A3F630` - three-range guard on `+0xA7C`

* **Address:** `0x80A3F630`
* **TSV size:** 32 bytes
* **Reachable body:** 8 instructions; three `beqlr`; ends in a tail `b` to
  `0x80027A60`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a3f630_FUN_80a3f630.c`
  - the export keeps the three guards and stops at `halt_baddata`; the missing
    instruction is the tail branch shown below.
* **Proposed signature:** `pub unsafe fn fn_80a3f630(this: usize)`
* **Callers in this REL:** one direct `bl`, `0x80A3F21C` with `r3 = this`, in
  the **untracked** body that starts at `0x80A3EA20` (immediately after
  `fn_80a3dea0`'s `0xB7C`-byte extent; no `functions.tsv` row). No 32-bit
  word references the entry.
* **Constants:** state word `+0xA7C`; accepted-skip values `0`, `0xF`, `0xD`.
* **Verdict:** **ready** (confidence medium)

### Frame

None. Leaf, three conditional returns and one tail branch.

### Instruction map

| off | instruction |
|---:|---|
| `+0x00` | `lwz r0,0xA7C(r3)` |
| `+0x04` | `cmpwi cr0,r0,0` |
| `+0x08` | `beqlr` |
| `+0x0C` | `cmplwi cr0,r0,0xF` |
| `+0x10` | `beqlr` |
| `+0x14` | `cmplwi cr0,r0,0xD` |
| `+0x18` | `beqlr` |
| `+0x1C` | `b 0x80027A60` |

### Behaviour

```c
void fn_80a3f630(usize this) {
    let m = read_u32(this + 0xA7C);
    if (m == 0 || m == 0xF || m == 0xD) return;
    call1(0x80027A60, this);
}
```

* The first compare is signed `cmpwi`; the next two are unsigned `cmplwi`.
  `0`, `0xF` and `0xD` are the only values that return early.
* Every other value tail-calls the main.dol teardown step `0x80027A60` with
  the actor in `r3`. Wave 133 groups that callee with `0x80036A70` /
  `0x80027D70` as the retire path, which is why the role is inferred rather
  than exported.

### Call sites

| site | callee | arguments |
|---|---|---|
| `0x80A3F64C` | `0x80027A60` (tail `b`) | `r3 = this` |

### Ambiguity

The callee's role is inferred (main.dol has no export for `0x80027A60`), but
the call site itself is exact and the guard values are literal. No datum is
missing for a byte-exact port, so the row is ready; confidence is `medium`
because of the callee role.

---

## Cross-references and collisions

All twelve addresses were checked against the `address` column of
`ledger/functions.csv` (absent), against the `pub unsafe fn fn_<addr>` scan of
`src/**/*.rs` (absent), and against every `0x80Axxxxx` / `0x80Bxxxxx` token in
every `#`-heading of every `docs/*.md` at selection time (absent; the only
headings that contain them now are the twelve sections below). They are named
in prose in earlier waves, always as a neighbour, skip-list member or callee,
never as a specified body:

| address | where it is mentioned | in what role |
|---|---|---|
| `0x80AB1830` | `ANALYSIS_WAVE127_ENEMIES.md:332`, `COVERAGE_WAVE56/72/86/120_ENEMIES.md`, `SPEC_WAVE130_ENEMIES.md:125`, `SPEC_WAVE132_ENEMIES.md:142` | "untranslated 0x40-frame countdown"; call target of the fishbone chain |
| `0x80A01700` | `ANALYSIS_WAVE127_ENEMIES.md:107`, `COVERAGE_WAVE71/83/112_ENEMIES.md`, `COVERAGE_WAVE132_ENEMIES.md:138` | truncated-export skip list |
| `0x80B250F0` | `ANALYSIS_WAVE127_ENEMIES.md:480`, `COVERAGE_WAVE79/110/129/137_ENEMIES.md` | "short trampoline to 0x80078470" |
| `0x80AEDC90` | `ANALYSIS_WAVE127_ENEMIES.md:409`, `COVERAGE_WAVE122_ENEMIES.md:123` | teardown-group skip list |
| `0x80B0C0D0` | `ANALYSIS_WAVE127_ENEMIES.md:443`, `COVERAGE_WAVE115_ENEMIES.md:52/127` | "0x8008C380(+0x8D4) then return 1" |
| `0x80A0E160` | `ANALYSIS_WAVE127_ENEMIES.md:116`, `COVERAGE_WAVE112/132_ENEMIES.md`, `SPEC_WAVE132_ENEMIES.md:141` | 9-instruction trampoline, cited as a callee |
| `0x80AD42B0` | `ANALYSIS_WAVE127_ENEMIES.md:370`, `COVERAGE_WAVE65/111/130_ENEMIES.md`, `SPEC_WAVE128_ENEMIES.md:768` | neighbour of the `0x80AD3D50` step |
| `0x80B07C90` | `ANALYSIS_WAVE127_ENEMIES.md:440`, `COVERAGE_WAVE82/97/115_ENEMIES.md` | `halt_baddata` skip list |
| `0x80A281C0` | `ANALYSIS_WAVE127_ENEMIES.md:143`, `COVERAGE_WAVE76/105/123_ENEMIES.md` | truncated export; called by `fn_80a29010` |
| `0x80A337C0` | `ANALYSIS_WAVE127_ENEMIES.md:166`, `COVERAGE_WAVE70/99/117_ENEMIES.md` | neighbour of the 4-byte alias `0x80A337B0` |
| `0x80A36A50` | `ANALYSIS_WAVE127_ENEMIES.md:177`, `COVERAGE_WAVE70/117_ENEMIES.md`, `SPEC_WAVE128/132_ENEMIES.md` | "state-2 gate, returns bool-ish" |
| `0x80A3F630` | `ANALYSIS_WAVE127_ENEMIES.md:194`, `COVERAGE_WAVE70/117_ENEMIES.md` | three-range guard, cited as a truncated export |

Two corrections to earlier prose are worth recording:

1. **`li` vs `add`.** Earlier prose makes the timer reloads and the effect
   row look field-derived. The bytes are `li r6,64`, `li r0,24` and (at ranks
   6/8) `li r5,7`: the reload and row values are literals, while the **table
   index** at rank 1 really is `*(u8 *)(this+0x348)`.
2. **`lhax`.** The indexed halfword form at `0x80A36A68` is the same
   `XO 343` encoding that `docs/SPEC_WAVE129_BASES.md` and
   `docs/SPEC_WAVE132_ENEMIES.md` call `lhax`; it must be typed as an `i16`
   load, not a `u32` load, or the ±0x2000 facing is lost.

New helper claims: none. `0x80078470`, `0x800D8B00`, `0x80022100`,
`0x80027A60`, `0x8006D090` and `0x8008C380` are declared/used by address
exactly as the module already does; `0x8015F4F0` is ledgered.

## Verification

* **Selection.** The twelve addresses are the highest-ranked untranslated
  class-B rows under the wave-127 metric after removing the five rows that
  already have Rust bodies (`0x80A2B490`, `0x80A6F8C0`, `0x80AA39D0`,
  `0x80A22B20`, `0x80A61E10`). Reachable counts are
  `23, 20, 14, 12, 10, 9, 9, 9, 8, 8, 8, 8`; TSV sizes are
  `92, 80, 56, 48, 40, 36, 36, 36, 32, 32, 32, 32`. At selection time the
  twelve were absent from the ledger, from every `#`-heading of every
  `docs/*.md`, and from the Rust function scan.
* **Sizes.** Every TSV size matches the `size` column of
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and the reachable span
  is exactly `TSV size / 4` instructions in all twelve bodies; there are no
  trailing unreachable words inside the extents.
* **Control flow.** 10 conditional branches (`beq`/`bnelr`/`beqlr`), all with
  in-body or return targets; one `bl` (`0x8008C380` at rank 5); one `bctrl`
  (rank 4); ten tail `b` (ranks 1, 2, 3, 6, 7, 8, 9, 10, 11, 12); two
  terminating `blr` (ranks 4, 5); no jump table, no `bctr`, no `psq`.
* **Constants.** `0x80B2FB30` (four f32: `6.0, -6.0, -0.000244140625,
  0.000732421875`), `0x80B2A860 = 24.0f`, and the three descriptors at
  `0x80B2B87C`/`0x80B2B888`/`0x80B2B898` were read from
  `work/rels/elf/d_enemiesNP.elf` at file offsets `0x12FB30`, `0x12B860`,
  `0x12C87C`, `0x12C888`, `0x12C898`. `0x8042C1A4` is a main.dol i16 pair
  `{+0x2000, -0x2000}`, read from `work/dol/main.elf`. `0x80429DD0`,
  `0x8042A008` and `0x8042A200` are main.dol BSS words (no file bytes) and
  are used only as pointers/bits.
* **Entry references.** Direct-branch and data-word scans over the whole REL
  agree with the caller lines: rank 1 has five `bl` sites, rank 2 one, rank 3
  one, rank 6 one, rank 7 one, rank 8 one, rank 9 one, rank 11 three, rank 12
  one; ranks 4 and 5 appear only as 32-bit words at `0x80B61BC8` and
  `0x80B6805C`; rank 10 appears only through the 4-byte `b` at `0x80A337B0`.
* **Indirect calls.** Only rank 4 has one. Its receiver chain is
  `*(u32 *)(this + 0x394)` and the slot is `+0x10`; the Ghidra export prints
  the same chain. The missing argument list is the sole conditional datum.
* **Read-only state.** No ledger row, Rust file, CSV or other doc was written
  by this pass. `docs/SPEC_WAVE138_ENEMIES.md` is the only artefact. A
  concurrent implementation lane is editing `src/game/enemies.rs`; at the
  time of writing none of the twelve has a `fn_80...` body there, while
  `0x80AB1830`, `0x80A0E160`, `0x80A281C0` and `0x80A36A50` are *named* as
  neighbours/call targets in comments or call tables. The binding selection
  criterion is the ledger plus the spec headings plus the Rust function scan,
  exactly as in waves 128-137.
