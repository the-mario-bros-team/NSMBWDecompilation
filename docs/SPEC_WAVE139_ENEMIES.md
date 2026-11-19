# Wave 139 - implementation specification for 12 class-B `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B** (reconstructible now), 160 class C
and 51 class D. Waves 128 - 138 (`docs/SPEC_WAVE128_ENEMIES.md` ..
`docs/SPEC_WAVE138_ENEMIES.md`) specified 132 of those B rows, twelve per wave.

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
the REL image were read out of the ELF itself, so the integer and float values
below are the shipped values. Addresses outside the file-backed image are
marked **(ext)**.

The twelve bodies are the largest of what remains of the B pool, and they are
all 8 instructions or fewer. Wave 138's closing section already recorded that
the remaining B pool was small; this page drains the part of it that fits one
tranche.

## Selection

Ordering is the wave-127 "reachable instruction count" - the only size metric
that is not inflated by the TSV extent quirk - with ties broken by TSV size and
then by ascending address.

Re-walking all 173 B rows against the ledger, the heading scan and the Rust
`fn_<addr>` scan leaves **21** untranslated B rows. Twelve are taken here. Nine
remain (`0x80AAA480`, `0x80A3F6D0`, `0x80A2F730`, `0x80A2F740`, `0x80A4EC90`,
`0x80A12110`, `0x80A7E630`, `0x80AB1A00`, `0x80ADA6E0`); all nine are 5
instructions or fewer and are listed at the end of this page for wave 140.

| # | address | TSV size | reachable insns | owner / role | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80AD0A90` | 32 | 8 | `+0x348` side step of `+0x8A8` toward ±30°; also data word `0x80B5B428` | **ready** |
| 2 | `0x80B1B0E0` | 32 | 8 | `+0x348` side step of `+0x102` toward `0x80B32488[side]` | **ready** |
| 3 | `0x80A18770` | 28 | 7 | row-2 / band-1 rumble on the `0x8042A008` manager | **ready** |
| 4 | `0x80A40FA0` | 28 | 7 | object message `0x191` with the caller's shape byte | **ready** |
| 5 | `0x80A9F750` | 28 | 7 | sound-stop guard on `(s8)other[+0x1080] < 7` | **conditional** |
| 6 | `0x80AA0080` | 28 | 7 | ground-probe dispatch on bit 0 of `+4` | **ready** |
| 7 | `0x80A18730` | 24 | 6 | step `+0x10C` toward `2.0` at `0.1` per frame | **ready** |
| 8 | `0x80A18750` | 24 | 6 | snap `+0x10C` to `1.25` | **ready** |
| 9 | `0x80A443A0` | 24 | 6 | object message `0x192` on a position (player lookup) | **ready** |
| 10 | `0x80A1B2E0` | 20 | 5 | retire the actor when `+0x616 == 0xFFFF` | **ready** |
| 11 | `0x80A2D460` | 20 | 5 | per-frame base select on `+0x666 == 2` | **ready** |
| 12 | `0x80AAA3E0` | 20 | 5 | spawn object kind `0x81` | **ready** |

**Split: 11 ready, 1 conditional.** Total 316 bytes, 77 reachable
instructions.

The class-C pool was **not** needed: 21 B rows were still free, so the GQR3 pin
does not enter the selection. It is still worth recording why, because the pin
is otherwise the most valuable datum added since wave 138 - see the next
section. In short: no class-C enemies row uses quantiser 3. The qr3 bodies are
all class D in the wave-127 table, and they are now unblocked for a later wave.

## GQR3 and the qr3 pool (finding)

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`
(`li r3,5` / `oris r3,r3,5` / `mtspr 915,r3`). The RAW stream in this REL holds
no `mtspr GQRn`, so every `psq_*` in this module depends on a `main.dol` word.

A scan of all 384 reclassified rows for `psq_l`/`psq_st` with a base other than
`r1` gives a clean result:

* **93** such instructions in **47** rows, and **every one of them is
  `psq_l fD,0(rA),1,qr3`**: opcode 56, `W = 1`, `DQ = 0`, quantiser index 3,
  `fD` = `f1` (86 times) or `f0` (7 times), base `r3`/`r4`/`r5`/`r6`/`r29`/`r30`.
* **zero** of them is a store (`psq_st`), and **zero** uses qr0, qr1, qr2 or
  qr4..qr7 on a data base.
* all 47 rows are class **D** in the wave-127 table; no class-B or class-C row
  contains a data `psq` at all.

Two consequences:

1. **The wave-127 "GQR4" prose is wrong.** Every D row that the analysis table
   describes as "`psq` against GQR4, programmed outside this REL"
   (`0x80A0E580`, `0x80A1D530`, `0x80A3A4A0`, `0x80B23480`, ...) actually
   encodes **qr3**. The quantiser index is bits 12-14 of the instruction word,
   and the words are e.g. `0x80A1D588 = E023B000` -> `psq_l f1,0(r3),1,qr3`.
   Nothing in this REL reads qr4 on a data base.
2. **With the pin, the 47 rows become mechanical.** `psq_l fD,DQ(rA),1,qr3`
   with `GQR3 = 0x00050005` is one unsigned 16-bit load scaled by `2^0`, i.e.
   `fD = (f32)*(u16 *)(rA + DQ)`. The common shape is a stack halfword written
   immediately before the load (`sth r0,10(r1)` then `psq_l f1,0(r5),1,qr3`
   with `r5 = r1+10`), which is an integer angle fed to the sine/cosine helper
   at `0x80237BD0`. A Rust body therefore reads the `u16` field directly and
   widens it to `f32`; no paired-single games are required.

This lane does not claim any of those rows, because the selection rule ranks
class-B first and 21 B rows remained. The largest qr3 rows, for whoever takes
the next tranche, are:

| # | address | TSV size | | # | address | TSV size |
|---:|---|---:|---|---:|---|---:|
| 1 | `0x80A3A4A0` | 2620 | | 7 | `0x80AE4A50` | 660 |
| 2 | `0x80AEDD40` | 1208 | | 8 | `0x80A4B740` | 604 |
| 3 | `0x80AEF130` | 1180 | | 9 | `0x80A4B420` | 548 |
| 4 | `0x80A0E580` | 968 | | 10 | `0x80AA93F0` | 496 |
| 5 | `0x80B0C540` | 848 | | 11 | `0x80A635D0` | 488 |
| 6 | `0x80AEB450` | 700 | | 12 | `0x80AFDAC0` | 480 |

They still need the wave-127 classification corrected from D to C (or B) before
they are claimed; the wave-127 D rule ("a `psq_l/psq_st` whose quantiser is
`GQR1..GQR7`") no longer applies once the specific word is pinned.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000`.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte
  offsets from `this`.
* `addi rX,r0,imm` is `li rX,imm`, and `addis rX,r0,imm` is `lis rX,imm`
  (`r0` is not read when `RA == 0`). `or rX,rY,rY` is `mr rX,rY`;
  `or rX,rY,rZ` is a three-operand OR.
* `rlwinm rA,rS,SH,MB,ME` follows the PowerPC rotate-and-mask definition; the
  two idioms used below are `rlwinm r0,r0,1,0,30` (integer `* 2`) and
  `rlwinm. r0,r0,0,31,31` (extract bit 31 **and** set `CR0`).
* `lhax rD,rA,rB` is the XO-343 form: `rD = (s16)*(u16 *)(rA + rB)`,
  sign-extended to 32 bits. It is not `sthx` (XO 407).
* `lha` is a signed halfword load; `lhz` is unsigned. `lbz` is unsigned; the
  `extsb` XO-954 form narrows to `s8`.
* The paired-single opcodes in this build are `psq_l = 56` and `psq_st = 60`
  (`docs/REL_ELF_MAPPING.md`); the quantiser is bits 12-14, `W` is bit 15.
* TSV `size` is a bucket extent. Every one of the twelve bodies below fills its
  extent exactly, with no trailing unreachable words.

## Symbolic helpers used below

| address | name / shape | role |
|---|---|---|
| `0x8015F4F0` | `step_short_clamped_wrap(value: *mut i16, target: i32, step: i32) -> u32` | ledgered `game::task`; steps the halfword toward `target`, returns `1` on arrival |
| `0x8015F470` | `step_float_clamped(f1 target: f64, f2 step: f64, r3 value: *mut f32) -> u32` | ledgered `engine::display` thunk to `0x8015F480`; returns `1` on arrival |
| `0x800D8C60` | `fn_800d8c60(object, row: i32, bands: u32, flags: u32, extra: u32)` | ledgered `engine::display`; per-player rumble/quakes over four slots |
| `0x80064610` | `(kind: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) -> u32` | object/message factory; returns the created handle or `0` |
| `0x80064D10` | `(this, mode: u32)` | retires an actor with a mode flag (void) |
| `0x800CE8C0` | `(handle)` | cancels the sound/effect on the manager word `0x80429FB0` (void) |
| `0x80097170` | `(f1 scale, r3 this, r4 point)` | notice trigger; latches `+0x45A` on a ground code |
| `0x80097250` | `(f1 scale, r3 this, r4 point)` | ground probe plus four vtable groups |
| `0x80021600` | `(this)` | alternate per-frame base of `fn_80a1c7a0` |
| `0x800214F0` | `(this)` | normal per-frame base of `fn_80a1c7a0` |

`0x80064610`'s kind table is only partly established: `0x191` is the effect
message used by the ammo-drop step (`docs/COVERAGE_WAVE77_ENEMIES.md`), `0x192`
is the position lookup whose result the caller treats as a player pointer, and
`0x81`/`0x82` are the two shot kinds documented next to `0x80AAA360`/`0x80AAA400`.

---

## 1. `0x80AD0A90` - side step of the `+0x8A8` angle toward ±30°

* **TSV size:** 32 bytes
* **Reachable body:** 8 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80AD0AAC b 0x8015F4F0`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad0a90_FUN_80ad0a90.c`
* **Proposed signature:** `pub unsafe fn fn_80ad0a90(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame, no LR save, no FPR traffic. Only `r0`, `r3`, `r4` and
`r5` are written.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B30850` | `s16[2]` | `{-5461, +5461}` = ±30° in 16-bit turn units (`0x1555`; `65536/12 = 5461.33`) |
| `+0x348` | `u8` | side index, exactly the byte `0x80B32488` is indexed by in rank 2 |
| `0x400` | immediate | `1024` step (5.625° per frame) |
| `+0x8A8` | `i16` | destination angle |

The table word after the pair is `0x80B30854 = 0x00000000`, the first word of an
unrelated float block, so `0x80B30850` is a two-entry table, not an eight-entry
one. For the live `+0x348` (0 or 1) the read is exact; for a synthetic actor
that stored a larger byte the load still lands deterministically in that float
block, so the body needs no guard.

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80AD0A90` | `88030348` | `lbz r0,0x348(r3)` | side byte |
| `0x80AD0A94` | `3C8080B3` | `lis r4,0x80B3` | table page |
| `0x80AD0A98` | `38840850` | `addi r4,r4,0x850` | `r4 = 0x80B30850` |
| `0x80AD0A9C` | `38A00400` | `li r5,0x400` | step argument |
| `0x80AD0AA0` | `5400083C` | `rlwinm r0,r0,1,0,30` | `r0 = 2 * side` |
| `0x80AD0AA4` | `386308A8` | `addi r3,r3,0x8A8` | destination pointer |
| `0x80AD0AA8` | `7C8402AE` | `lhax r4,r4,r0` | `r4 = (s16)table[side]` |
| `0x80AD0AAC` | `4B68EA44` | `b 0x8015F4F0` | tail call |

### Behaviour

```c
u8  side   = *(u8 *)(this + 0x348);
s16 target = *(s16 *)(0x80B30850 + 2u * side);          // lhax
return step_short_clamped_wrap((i16 *)(this + 0x8A8), target, 0x400);
```

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80AE2554` | `bl` from the untracked body at `0x80AE24F0` | `r3 = this`; taken when `+0xAA0 == 0` or `+0x77C < 2`, i.e. the fallback beside the twin ±90° / `0x1000` step at `0x80AE2538` |
| `0x80B5B428` | 32-bit data word | inside the code-pointer run `0x80B5B3E0..0x80B5B43C`, terminated by zeros at `0x80B5B440` |

### Ambiguity

None. `+0x348` is a two-valued side byte here and in rank 2; the two targets are
the shipped ±30°. The sibling computation at `0x80AE2530` uses the same shape
with `0x80B30DB0 = {-16384, +16384}` (±90°) and a `0x1000` step, which is what
makes the `±30° / 0x400` reading of this body unambiguous.

---

## 2. `0x80B1B0E0` - side step of the `+0x102` yaw toward the pivot table

* **TSV size:** 32 bytes
* **Reachable body:** 8 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80B1B0FC b 0x8015F4F0`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b1b0e0_FUN_80b1b0e0.c`
* **Proposed signature:** `pub unsafe fn fn_80b1b0e0(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame, no LR save, no FPR traffic.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B32488` | `s16[8]` | `{12743, -12743, 13653, -13653, 120, 180, 13653, 0}` |
| `+0x348` | `u8` | side index (0..7) |
| `0x800` | immediate | `2048` step |
| `+0x102` | `i16` | destination yaw |

`0x80B32488` is the eight-entry pivot table already quoted by
`docs/SPEC_WAVE132_ENEMIES.md` §7; the values above were re-read from file
offset `0x133488`.

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80B1B0E0` | `88030348` | `lbz r0,0x348(r3)` | side byte |
| `0x80B1B0E4` | `3C8080B3` | `lis r4,0x80B3` | table page |
| `0x80B1B0E8` | `38842488` | `addi r4,r4,0x2488` | `r4 = 0x80B32488` |
| `0x80B1B0EC` | `38A00800` | `li r5,0x800` | step argument |
| `0x80B1B0F0` | `5400083C` | `rlwinm r0,r0,1,0,30` | `r0 = 2 * side` |
| `0x80B1B0F4` | `38630102` | `addi r3,r3,0x102` | destination pointer |
| `0x80B1B0F8` | `7C8402AE` | `lhax r4,r4,r0` | `r4 = (s16)table[side]` |
| `0x80B1B0FC` | `4B6443F4` | `b 0x8015F4F0` | tail call |

### Behaviour

```c
u8  side   = *(u8 *)(this + 0x348);
s16 target = *(s16 *)(0x80B32488 + 2u * side);          // lhax
return step_short_clamped_wrap((i16 *)(this + 0x102), target, 0x800);
```

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80B1B460` | `bl` from `fn_80b1b3f0` (translated) | `r3 = this`; the returned `u32` becomes `r31` and gates the `+0xD4` state switch with `g2` |

`docs/SPEC_WAVE132_ENEMIES.md` §7 currently models this entry as an opaque gate
(`u32 g1 = fn_80B1B0E0(this)`). The body above can replace that call by address
with no other change.

### Ambiguity

None. The table is in the REL, the index byte is the same one the caller uses
for `0x8042C1A0`, and the step is the literal `0x800`.

---

## 3. `0x80A18770` - row-2 / band-1 rumble on the `0x8042A008` manager

* **TSV size:** 28 bytes
* **Reachable body:** 7 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80A18788 b 0x800D8C60`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a18770_FUN_80a18770.c`
* **Proposed signature:** `pub unsafe fn fn_80a18770()`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame. `r3` is overwritten by the global load, so there is no
incoming register this body reads: the signature has **no parameters**.

### Constants

| address | type | value / role |
|---|---|---|
| `0x8042A008` | `u32` | **(ext)** main.dol BSS handle of the camera/rumble manager (`DAT_8042A008` / `BURST_DAT_8042A008`); no file bytes |
| `2` | immediate | `row` |
| `1` | immediate | `bands` |
| `0`, `0` | immediates | `flags`, `extra` |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A18770` | `3C608043` | `lis r3,0x8043` | SDA page |
| `0x80A18774` | `38800002` | `li r4,2` | row |
| `0x80A18778` | `8063A008` | `lwz r3,-0x5FF8(r3)` | `r3 = *(u32 *)0x8042A008` |
| `0x80A1877C` | `38A00001` | `li r5,1` | bands |
| `0x80A18780` | `38C00000` | `li r6,0` | flags |
| `0x80A18784` | `38E00000` | `li r7,0` | extra |
| `0x80A18788` | `4B6C04D8` | `b 0x800D8C60` | tail call |

`0x80430000 - 0x5FF8 = 0x8042A008`.

### Behaviour

```c
fn_800d8c60(*(u32 *)0x8042A008, 2, 1, 0, 0);
```

`bands = 1` selects the `bands & 1` arm, which is
`fn_800d8e40(object, 0xFF, 2, 0)` - the all-slots (`0xFF`) form of row 2; the
`extra` word is not consumed by that arm.

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A18C44` | `bl` from the untracked state-entry body at `0x80A18C20` | `r3 = this` (ignored); that body also copies `+0x698` -> `+0x500` and calls `0x80A18790` / `0x80A18800` |

### Ambiguity

The handle's role is inferred from the module-wide use of `0x8042A008` as the
camera/rumble manager (`fn_80a5c4b0` reads the same word and calls the same
helper). Every operand of the call itself is a literal.

---

## 4. `0x80A40FA0` - object message `0x191` with the caller's shape byte

* **TSV size:** 28 bytes
* **Reachable body:** 7 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80A40FB8 b 0x80064610`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a40fa0_FUN_80a40fa0.c`
* **Proposed signature:** `pub unsafe fn fn_80a40fa0(position: usize, slot: i32) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame. `r3` is the position record; `r4` is the shape/slot byte
from the caller; `r5` is rebuilt from `r3`.

### Constants

| value | role |
|---|---|
| `0x191` | object/effect message kind (the "ammo-drop effect" id) |
| `9` | low nibble/byte of the message flags |
| rotate 12 / mask 12..19 | moves `slot & 0xFF` into bits 12..19 |
| `0` | `arg4`, `arg5` |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A40FA0` | `54806326` | `rlwinm r0,r4,12,12,19` | `r0 = (slot & 0xFF) << 12` |
| `0x80A40FA4` | `7C651B78` | `or r5,r3,r3` | `r5 = position` |
| `0x80A40FA8` | `60040009` | `ori r4,r0,9` | `r4 = flags` |
| `0x80A40FAC` | `38600191` | `li r3,0x191` | kind |
| `0x80A40FB0` | `38C00000` | `li r6,0` | arg4 |
| `0x80A40FB4` | `38E00000` | `li r7,0` | arg5 |
| `0x80A40FB8` | `4B623658` | `b 0x80064610` | tail call |

### Behaviour

```c
u32 flags = (((u32)slot & 0xFFu) << 12) | 9u;
return fn_80064610(0x191, flags, position, 0, 0);
```

The shape field is eight bits wide (bits 12..19), not four. The stage analogue
`0x80088B30` builds the same `0x191|9` word from a shape byte
(`docs/COVERAGE_WAVE31_STAGE.md`), where `-1` is documented as shape `3`;
that mapping happens in the receiver, not here.

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A3F424` | `bl` from `fn_80a3f360` (translated) | `r3 = &stack point {x - 0x80B2C540, y + 0x80B2C568, z}`; `r4 = (s8)` byte returned at `0x80A3F3D8` from the `+0x6C` vtable call, i.e. the slot index |

`fn_80a3f360` currently calls this entry by address with `r3` only
(`W117_FN_80A40FA0`). The spec adds the missing `r4`: it is the sign-extended
slot byte, and `-1` is filtered out at `0x80A3F3DC` before the call.

### Ambiguity

The message kind is named from `docs/COVERAGE_WAVE77_ENEMIES.md`; the flag word
and the argument order are literal.

---

## 5. `0x80A9F750` - sound-stop guard on `(s8)other[+0x1080] < 7`

* **TSV size:** 28 bytes
* **Reachable body:** 7 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80A9F768 b 0x800CE8C0`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a9f750_FUN_80a9f750.c`
* **Proposed signature:** `pub unsafe fn fn_80a9f750(_self: usize, other: usize)`
* **Confidence:** **medium / conditional**

### Frame

Leaf. No stack frame. Crucially, the body reads **`r4` only**; `r3` is never
touched before it is overwritten by the global load. The Ghidra prototype
(`FUN_80a9f750(undefined4 param_1, int param_2)`) agrees.

### Constants

| address | type | value / role |
|---|---|---|
| `+0x1080` | `s8` | state byte of the *other* object |
| `7` | immediate | threshold |
| `0x80429FB0` | `u32` | **(ext)** main.dol effect/sound manager word (`DAT_80429FB0`) |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A9F750` | `88041080` | `lbz r0,0x1080(r4)` | raw byte |
| `0x80A9F754` | `7C000774` | `extsb r0,r0` | `s8` |
| `0x80A9F758` | `2C000007` | `cmpi cr0,0,7` | signed compare |
| `0x80A9F75C` | `4D800020` | `bltlr` | return while the byte is `< 7` |
| `0x80A9F760` | `3C608043` | `lis r3,0x8043` | SDA page |
| `0x80A9F764` | `80639FB0` | `lwz r3,-0x6050(r3)` | `r3 = *(u32 *)0x80429FB0` |
| `0x80A9F768` | `4B62F158` | `b 0x800CE8C0` | tail call |

`0x80430000 - 0x6050 = 0x80429FB0`.

### Behaviour

```c
if ((i8)*(u8 *)(other + 0x1080) < 7) {
    return;
}
fn_800ce8c0(*(u32 *)0x80429FB0);
```

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A9F794` | `bl` from `fn_80a9f770` (translated) | `r3 = this`, `r4` untouched |
| `0x80A9F804` | `bl` from the duplicate body at `0x80A9F7E0` | `r3 = this`, `r4` untouched |
| `0x80A9F874` | `bl` from the duplicate body at `0x80A9F850` | `r3 = this`, `r4` untouched |

All three callers are 0x70-byte near-duplicates (`0x80A9F770`,
`0x80A9F7E0`, `0x80A9F850`) and call at `start + 0x24`, immediately after
`lhz/cmpi/bne` on `this+0x872`. None of them writes `r4` between its own entry
and the `bl`.

### Ambiguity

**Conditional.** The load, the signed compare against `7`, and the tail call to
the sound cancel are exact. The only missing datum is the identity of the
register: `r4` is the *second incoming argument* of the enclosing state
callback (the first is `this`). Wave 135 skipped this row for exactly this
reason (`docs/COVERAGE_WAVE135_ENEMIES.md`, "Deliberate skips"). The
implementation contract is therefore two registers: keep `_self: usize` for
`r3` (unused) and thread the caller's `r4` as `other`. Any caller that can name
that second argument - even as "the parent/payload object" - promotes this row
to ready without changing a byte.

---

## 6. `0x80AA0080` - ground-probe dispatch on bit 0 of `+4`

* **TSV size:** 28 bytes
* **Reachable body:** 7 instructions; 0 `bl` + 0 `bctrl`; two tail branches
  (`0x80AA008C b 0x80097170`, `0x80AA0098 b 0x80097250`)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa0080_FUN_80aa0080.c`
* **Proposed signature:** `pub unsafe fn fn_80aa0080(this: usize, point: usize, scale: f32) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame. `r3`, `r4` and `f1` are forwarded to the selected tail
callee unchanged, so the wrapper must keep all three.

### Constants

| value | role |
|---|---|
| `+0x004` bit 31 (`& 1`) | selector: `0` -> `0x80097170`, `1` -> `0x80097250` |
| `1` | `cmpli` operand |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80AA0080` | `80030004` | `lwz r0,4(r3)` | actor flags |
| `0x80AA0084` | `540007FF` | `rlwinm. r0,r0,0,31,31` | `r0 = flags & 1`, sets `CR0` |
| `0x80AA0088` | `40820008` | `bne 0x80AA0090` | taken when bit 0 is set |
| `0x80AA008C` | `4B5F70E4` | `b 0x80097170` | bit clear -> notice trigger |
| `0x80AA0090` | `28000001` | `cmpli cr0,0,1` | defensive re-test |
| `0x80AA0094` | `4C820020` | `bnelr` | return unless `r0 == 1` (dead for a 1-bit value) |
| `0x80AA0098` | `4B5F71B8` | `b 0x80097250` | bit set -> ground probe |

### Behaviour

```c
u32 bit = *(u32 *)(this + 4) & 1u;
if (bit == 0) {
    return fn_80097170(scale, this, point);
}
if (bit != 1) {          // bnelr; unreachable because bit is one bit
    return 0;
}
return fn_80097250(scale, this, point);
```

The `cmpli/bnelr` pair is kept in the map because it is in the stream; a Rust
body may collapse it to the two-way select without changing observable
behaviour for a value that is always 0 or 1.

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80AA0054` | `bl` from `fn_80a9ff20` (translated) | `r3 = actor`, `r4 = actor+0xAC`, `f1 = 1.0f` (`0x80B2F2E0`) |
| `0x80AA1060` | `bl` from the untranslated body at `0x80AA1030` | same three operands |

### Ambiguity

None. The two tail callees are the notice-trigger/ground-probe pair whose
`(f1 scale, r3 this, r4 point)` shape is documented in
`docs/COVERAGE_WAVE113_STAGE.md`; this wrapper is the bit-0 dispatcher between
them.

---

## 7. `0x80A18730` - step `+0x10C` toward `2.0` at `0.1` per frame

* **TSV size:** 24 bytes
* **Reachable body:** 6 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80A18744 b 0x8015F470`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a18730_FUN_80a18730.c`
* **Proposed signature:** `pub unsafe fn fn_80a18730(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame; two `lfs` loads feed `f1`/`f2`, and `r3` becomes
`this + 0x10C`.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2B438` | `f32` | `0.1f` (`0x3DCCCCCD`) - step (`f2`) |
| `0x80B2B43C` | `f32` | `2.0f` (`0x40000000`) - target (`f1`) |
| `+0x10C` | `f32` | value being stepped |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A18730` | `3CA080B3` | `lis r5,0x80B3` | constant page |
| `0x80A18734` | `3863010C` | `addi r3,r3,0x10C` | value pointer |
| `0x80A18738` | `3885B438` | `addi r4,r5,-0x4BC8` | `r4 = 0x80B2B438` |
| `0x80A1873C` | `C045B438` | `lfs f2,-0x4BC8(r5)` | `f2 = 0.1f` |
| `0x80A18740` | `C0240004` | `lfs f1,4(r4)` | `f1 = 2.0f` |
| `0x80A18744` | `4B746D2C` | `b 0x8015F470` | tail call |

### Behaviour

```c
return step_float_clamped(2.0f64, 0.1f64, (f32 *)(this + 0x10C));
```

`lfs` loads extend to `f64` in the ABI, so the callee's two float arguments are
the exact single-precision bit patterns `0x40000000` and `0x3DCCCCCD` widened.

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A18B50` | `bl` from the untracked per-frame body at `0x80A188A0` (state 1) | `r3 = this` |
| `0x80A18BB4` | `bl` from the same body (state 3) | `r3 = this` |

### Ambiguity

None. Both constants are in the REL at the cited addresses.

---

## 8. `0x80A18750` - snap `+0x10C` to `1.25`

* **TSV size:** 24 bytes
* **Reachable body:** 6 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80A18764 b 0x8015F470`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a18750_FUN_80a18750.c`
* **Proposed signature:** `pub unsafe fn fn_80a18750(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame; `r3` becomes `this + 0x10C`, `r4` points at the constant
block.

### Constants

| address | type | value |
|---|---|---|
| `0x80B2B440` | `f32` | `1.25f` (`0x3FA00000`) - step (`f2`) |
| `0x80B2B444` | `f32` | `1.25f` (`0x3FA00000`) - target (`f1`) |
| `+0x10C` | `f32` | value being stepped |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A18750` | `3C8080B3` | `lis r4,0x80B3` | constant page |
| `0x80A18754` | `3863010C` | `addi r3,r3,0x10C` | value pointer |
| `0x80A18758` | `3884B438` | `addi r4,r4,-0x4BC8` | `r4 = 0x80B2B438` |
| `0x80A1875C` | `C024000C` | `lfs f1,0xC(r4)` | `f1 = 0x80B2B444 = 1.25f` |
| `0x80A18760` | `C0440008` | `lfs f2,8(r4)` | `f2 = 0x80B2B440 = 1.25f` |
| `0x80A18764` | `4B746D0C` | `b 0x8015F470` | tail call |

### Behaviour

```c
return step_float_clamped(1.25f64, 1.25f64, (f32 *)(this + 0x10C));
```

Because `step == target`, the helper snaps to `1.25` and returns `1` while the
current value is in `[0, 2.5]` (the overshoot test `0 <= delta * (next-target)`);
a larger or negative value is first stepped by `1.25` and settles on a later
frame.

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A18D64` | `bl` from the untracked per-frame body at `0x80A188A0` | `r3 = this` |

### Ambiguity

None.

---

## 9. `0x80A443A0` - object message `0x192` on a position (player lookup)

* **TSV size:** 24 bytes
* **Reachable body:** 6 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80A443B4 b 0x80064610`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a443a0_FUN_80a443a0.c`
* **Proposed signature:** `pub unsafe fn fn_80a443a0(point: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame. `r3` is moved to `r5` before `r3` is reloaded with the
message kind.

### Constants

| value | role |
|---|---|
| `0x192` | object/player lookup message kind |
| `0` | `arg2`, `arg4`, `arg5` |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A443A0` | `7C651B78` | `or r5,r3,r3` | `r5 = point` |
| `0x80A443A4` | `38600192` | `li r3,0x192` | kind |
| `0x80A443A8` | `38800000` | `li r4,0` | arg2 |
| `0x80A443AC` | `38C00000` | `li r6,0` | arg4 |
| `0x80A443B0` | `38E00000` | `li r7,0` | arg5 |
| `0x80A443B4` | `4B62025C` | `b 0x80064610` | tail call |

### Behaviour

```c
return fn_80064610(0x192, 0, point, 0, 0);
```

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A94F08` | `bl` from `fn_80a94eb0` (translated) | `r3 = actor+0xAC`; the caller treats the result as a player pointer and writes `player+0x9F8` / `player+0x9FC` |

### Ambiguity

The op id is named from the caller (`fn_80a94eb0` does the same lookup for the
`+0x81C == 1` branch and then writes the player slots), not from the factory.
The call itself is literal.

**Correction to wave 133.** `docs/SPEC_WAVE133_ENEMIES.md` §9 says the
`0x80A442D0` body "ends at `0x80A443A0 blr`". The `blr` is at `0x80A4439C`;
`0x80A443A0` is this separate entry, as the TSV row and the bytes show.

---

## 10. `0x80A1B2E0` - retire the actor when `+0x616 == 0xFFFF`

* **TSV size:** 20 bytes
* **Reachable body:** 5 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80A1B2F0 b 0x80064D10`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a1b2e0_FUN_80a1b2e0.c`
* **Proposed signature:** `pub unsafe fn fn_80a1b2e0(this: usize)`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame; only `r0`, `r3` and `r4` are written.

### Constants

| value | role |
|---|---|
| `+0x616` | `u16` marker |
| `0xFFFF` | "no owner" marker |
| `0x80064D10` | actor retire `(this, mode)` |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A1B2E0` | `A0030616` | `lhz r0,0x616(r3)` | marker |
| `0x80A1B2E4` | `6800FFFF` | `xori r0,r0,0xFFFF` | zero iff the marker is `0xFFFF` |
| `0x80A1B2E8` | `7C000034` | `cntlzw r0,r0` | `32` iff the marker matched |
| `0x80A1B2EC` | `5404DE3E` | `rlwinm r4,r0,27,24,31` | `r4 = (cntlzw == 32) ? 1 : 0` |
| `0x80A1B2F0` | `4B649A20` | `b 0x80064D10` | retire |

### Behaviour

```c
u16 marker = *(u16 *)(this + 0x616);
fn_80064d10(this, marker == 0xFFFF ? 1u : 0u);
```

The `xori`/`cntlzw`/`rlwinm` triple is Metrowerks' boolean materialisation for
`marker == 0xFFFF`; Ghidra prints only the `countLeadingZeros` and loses the
shift, which is why the export looks like a bare expression.

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A1AFF0` | `bl` from the untracked body at `0x80A1AFD0` | `r3 = this`; reached only while `*(u16 *)(this+0x614) == 0xFFFF` |

### Ambiguity

None. The two halfword offsets (`+0x614` guard in the caller, `+0x616` marker in
the callee) are distinct and both literal.

---

## 11. `0x80A2D460` - per-frame base select on `+0x666 == 2`

* **TSV size:** 20 bytes
* **Reachable body:** 5 instructions; 0 `bl` + 0 `bctrl`; two tail branches
  (`0x80A2D46C b 0x80021600`, `0x80A2D470 b 0x800214F0`)
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a2d460_FUN_80a2d460.c`
* **Proposed signature:** `pub unsafe fn fn_80a2d460(this: usize) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame; `r3` is forwarded unchanged.

### Constants

| value | role |
|---|---|
| `+0x666` | `u8` category byte |
| `2` | the "alternate base" category |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80A2D460` | `88030666` | `lbz r0,0x666(r3)` | category (unsigned) |
| `0x80A2D464` | `28000002` | `cmpli cr0,0,2` | unsigned compare |
| `0x80A2D468` | `40820008` | `bne 0x80A2D470` | not 2 -> normal base |
| `0x80A2D46C` | `4B5F4194` | `b 0x80021600` | alternate base |
| `0x80A2D470` | `4B5F4080` | `b 0x800214F0` | normal base |

### Behaviour

```c
if (*(u8 *)(this + 0x666) == 2) {
    return fn_80021600(this);
}
return fn_800214f0(this);
```

The load is `lbz` (unsigned) and the compare is `cmpli`; the Ghidra export's
`char` cast does not change the equality against `2`.

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80A2D698` | `bl` from `fn_80a2d660` (translated) | `r3 = this`; only in the `*(u16 *)(this+0x6C0) != 0` arm after the countdown decrement |

### Ambiguity

None. The two callees are the normal/alternate per-frame bases already named in
`fn_80a1c7a0` (`EN116_FN_80021600` / `EN116_FN_800214F0`).

---

## 12. `0x80AAA3E0` - spawn object kind `0x81`

* **TSV size:** 20 bytes
* **Reachable body:** 5 instructions; 0 `bl` + 0 `bctrl`; ends in the tail
  branch `0x80AAA3F0 b 0x80064610`
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aaa3e0_FUN_80aaa3e0.c`
* **Proposed signature:** `pub unsafe fn fn_80aaa3e0(owner: u32, actor: u32) -> u32`
* **Confidence:** **high / ready**

### Frame

Leaf. No stack frame. `r3` is moved to `r5` (the factory's third word) before
`r3` becomes the object kind; `r4` is forwarded as the factory's second word.

### Constants

| value | role |
|---|---|
| `0x81` | object kind |
| `0` | `arg4`, `arg5` |

### Instruction map

| address | word | instruction | effect |
|---|---|---|---|
| `0x80AAA3E0` | `7C651B78` | `or r5,r3,r3` | `r5 = owner` |
| `0x80AAA3E4` | `38600081` | `li r3,0x81` | kind |
| `0x80AAA3E8` | `38C00000` | `li r6,0` | arg4 |
| `0x80AAA3EC` | `38E00000` | `li r7,0` | arg5 |
| `0x80AAA3F0` | `4B5BA220` | `b 0x80064610` | tail call |

### Behaviour

```c
return fn_80064610(0x81, actor, owner, 0, 0);
```

### Call sites

| site | kind | arguments |
|---|---|---|
| `0x80AAD27C` | `bl` from `fn_80aad010` (translated) | `r3 = &stack 16-byte vector`, `r4 = spawn word`; the caller stores the returned record into `actor + 0xB44 + 4*slot` |

### Ambiguity

The kind's identity ("shot spawn") is inferred from the caller's slot/record
handling; the operand order is the same one `fn_80aaa360` (`0x7F`) and
`fn_80aaa400` (`0x80`) use, so it is fixed.

**Sibling.** `0x80AAA480` is the same five instructions with kind `0x82`. It
ties this row on reachable count and TSV size and loses only the ascending
address tie-break; it is the first row for wave 140.

---

## Cross-references and collisions

Every address was checked against the `address` column of
`ledger/functions.csv` (absent), against the `pub unsafe fn fn_<addr>` scan of
`src/**/*.rs` (absent), and against every `0x80Axxxxx` / `0x80Bxxxxx` token in
every `#`-heading of every `docs/*.md` at selection time (absent; the only
headings that contain them now are the twelve sections above). They are named
in prose in earlier waves, always as a neighbour, skip-list member or callee,
never as a specified body:

| address | where it is mentioned | in what role |
|---|---|---|
| `0x80AD0A90` | `ANALYSIS_WAVE127_ENEMIES.md:366`, `COVERAGE_WAVE51/80/95/111/130_ENEMIES.md` | B table ("GQR/paired-single", 8 insns); skip-list neighbour |
| `0x80B1B0E0` | `ANALYSIS_WAVE127_ENEMIES.md:471`, `COVERAGE_WAVE81/96/113_ENEMIES.md`, `SPEC_WAVE132_ENEMIES.md:150/703/732/758/765` | "short trampoline to `0x8015F4F0`"; gate `g1` inside the wave-132 rank 7 body |
| `0x80A18770` | `ANALYSIS_WAVE127_ENEMIES.md:124`, `COVERAGE_WAVE85/100/116/133_ENEMIES.md` | B table, "stream runs 7 insns to `b`" |
| `0x80A40FA0` | `ANALYSIS_WAVE127_ENEMIES.md:199`, `COVERAGE_WAVE77/93/117/126_ENEMIES.md`, `FUNCTION_INDEX.md:6645` | B table; called by address from `fn_80a3f360` |
| `0x80A9F750` | `ANALYSIS_WAVE127_ENEMIES.md:295`, `COVERAGE_WAVE118_ENEMIES.md:147`, `COVERAGE_WAVE135_ENEMIES.md:139/144`, `FUNCTION_INDEX.md:7704` | guard + tail; "`r4` is never set by its only in-REL caller" |
| `0x80AA0080` | `ANALYSIS_WAVE127_ENEMIES.md:296`, `COVERAGE_WAVE31/33/75/90/107/125_ENEMIES.md` | B table; bit-0 dispatcher called by address |
| `0x80A18730` | `ANALYSIS_WAVE127_ENEMIES.md:122`, `COVERAGE_WAVE85/100/116/133_ENEMIES.md` | B table, "stream runs 6 insns to `b`" |
| `0x80A18750` | `ANALYSIS_WAVE127_ENEMIES.md:123`, `COVERAGE_WAVE85/100/116/133_ENEMIES.md` | B table, "stream runs 6 insns to `b`" |
| `0x80A443A0` | `ANALYSIS_WAVE127_ENEMIES.md:91/208`, `COVERAGE_WAVE29/77/126_ENEMIES.md`, `SPEC_WAVE133_ENEMIES.md:709` | B table; wave 133 uses it as the end marker of `0x80A442D0` (one word late) |
| `0x80A1B2E0` | `ANALYSIS_WAVE127_ENEMIES.md:128`, `COVERAGE_WAVE100/116/133_ENEMIES.md` | B table, "stream runs 5 insns to `b`" |
| `0x80A2D460` | `ANALYSIS_WAVE127_ENEMIES.md:147/148`, `COVERAGE_WAVE76/92/123/136_ENEMIES.md`, `SPEC_WAVE135_ENEMIES.md:43/80/133/231/245/259` | B table; wave 135 names it as the `+0x666` retire tail and calls it by address |
| `0x80AAA3E0` | `ANALYSIS_WAVE127_ENEMIES.md:309`, `COVERAGE_WAVE107_ENEMIES.md:118`, `COVERAGE_WAVE125_ENEMIES.md:129` | B table; earlier "almost certainly a copy of `0x80AAA360`" |

Two corrections to earlier prose are worth recording:

1. **Wave 133's end marker.** `0x80A442D0` ends with `blr` at `0x80A4439C`,
   not `0x80A443A0`; the latter is a separate TSV row and a separate body.
   Nothing in the earlier translation depends on the slip, because the call was
   by address.
2. **Wave 135's "no honest argument".** For `0x80A9F750` the blocker is still
   the unset `r4`, but the body contract is now exact: `r3` is not read at all,
   and the whole body is the signed-byte compare plus the sound cancel. Only the
   caller-side name of `r4` is missing.

### Remaining class-B rows (wave 140)

| address | TSV size | reachable insns | note |
|---|---:|---:|---|
| `0x80AAA480` | 20 | 5 | kind `0x82` sibling of rank 12 |
| `0x80A3F6D0` | 16 | 4 | `+0xA90 == 0 ? return : tail 0x800279F0` |
| `0x80A2F730` | 12 | 3 | returns `0x80357C34` |
| `0x80A2F740` | 12 | 3 | returns `0x80357BF4` |
| `0x80A4EC90` | 12 | 3 | returns `0x80358084` |
| `0x80A12110` | 8 | 2 | `this+0x98C` -> tail `0x800944C0` |
| `0x80A7E630` | 8 | 2 | `this+0x5FC` -> tail `0x800944C0` |
| `0x80AB1A00` | 8 | 2 | `this+0x7C0` -> tail `0x8008C380` |
| `0x80ADA6E0` | 8 | 2 | `this+0x548` -> tail `0x8007FDA0` |

After those nine, the B pool is empty and the qr3 rows in the GQR3 section are
the next band.

## Verification

* **Selection.** The twelve addresses are the highest-ranked untranslated
  class-B rows under the wave-127 metric after removing every row that is in
  the ledger, in a spec heading, or already has a Rust body. Reachable counts
  are `8, 8, 7, 7, 7, 7, 6, 6, 6, 5, 5, 5`; TSV sizes are
  `32, 32, 28, 28, 28, 28, 24, 24, 24, 20, 20, 20`. The first excluded row is
  `0x80AAA480`, which ties rank 12 on both metrics and loses the ascending
  address tie-break.
* **Sizes.** Every TSV size matches the `size` column of
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and the reachable span is
  exactly `TSV size / 4` instructions in all twelve bodies. There are no
  trailing unreachable words inside the extents.
* **Control flow.** Two `bne`/`bc` (`0x80AA0088`, `0x80A2D468`), one `bltlr`
  (`0x80A9F75C`), one `bnelr` (`0x80AA0094`); ten tail `b` (ranks 1, 2, 3, 4, 5,
  7, 8, 9, 10, 12) and two tail-pair bodies (ranks 6 and 11, one `b` each);
  no `blr` terminator inside any extent, no jump table, no `bctr`, no `psq`.
* **Constants.** `0x80B30850 = {-5461, +5461}` and
  `0x80B32488 = {12743, -12743, 13653, -13653, 120, 180, 13653, 0}` were read
  from file offsets `0x131850` and `0x133488`; the float block
  `0x80B2B438..0x80B2B444 = {0.1, 2.0, 1.25, 1.25}` from `0x12C438`;
  `0x80B2F2E0 = 1.0f` from `0x12F2E0`. `0x8042A008` and `0x80429FB0` are
  `main.dol` zero-fill words (`PT_LOAD` with `filesz = 0`, inside
  `0x80429BC0..0x8042B080`), so they have no file bytes and are used only as
  pointers.
* **Entry references.** Direct-branch scans over the whole REL find exactly one
  `bl` to ranks 1, 2, 3, 4, 8, 9, 10, 11 and 12, two to ranks 6 and 7, three to
  rank 5, and one data word to rank 1; the rank-1 data word is `0x80B5B428`
  inside the code-pointer run.
* **GQR3.** `psq_l`/`psq_st` scan over all 384 rows: 93 non-`r1` instructions,
  all `psq_l ...,qr3, W=1, DQ=0`; zero `qr4`. No class-B or class-C row has a
  data `psq`. The pin `0x00050005` therefore does not enter this tranche.
* **Read-only state.** No ledger row, Rust file, CSV or other doc was written
  by this pass. `docs/SPEC_WAVE139_ENEMIES.md` is the only artefact. A
  concurrent implementation lane is editing `src/game/enemies.rs`; at the time
  of writing none of the twelve has a `fn_80...` body there, while
  `0x80B1B0E0`, `0x80A40FA0`, `0x80A9F750`, `0x80AA0080`, `0x80A443A0`,
  `0x80AAA3E0` and `0x80A2D460` are *named* as call targets or constants.
  The binding selection criterion is the ledger plus the spec headings plus the
  Rust function scan, never the coverage prose.
