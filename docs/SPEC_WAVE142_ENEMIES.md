# Wave 142 - implementation specification for 12 class-D `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B**, 160 class C and 51 class D.
Waves 128-141 (`docs/SPEC_WAVE128_ENEMIES.md` .. `docs/SPEC_WAVE141_ENEMIES.md`)
specified the B tranche and the first two `qr3` tranches. A live re-walk of every
address in those pages against `ledger/functions.csv`, every `docs/*.md` heading
and every `pub unsafe fn fn_<addr>` body in `src/` leaves **no unclaimed B or C
row and no unclaimed qr3 row that a previous spec already named**.

This lane is the continuation of the `qr3` tranche opened by
`docs/SPEC_WAVE140_ENEMIES.md:15-18`: the next twelve largest `d_enemiesNP`
addresses that are class D **only** because of a `psq_l ...,qr3` u16 load. Since
`docs/REL_ELF_MAPPING.md` pinned **`GQR3 = 0x00050005`** (unsigned 16-bit,
scale 0), every one of those loads is a plain `(f32)*(u16 *)` widen and the
wave-127 blocker is gone.

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000`
segment shift of `docs/REL_ELF_MAPPING.md` (first `PT_LOAD` is
`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`), instruction by instruction from the
entry, following every conditional branch and every direct call until each path
ended in `blr`, a tail `b` or a tail call. Each body was cross-checked against
the Ghidra export at the cited path and against the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Constants that live inside
the REL image were read out of the ELF itself, so the integer and float values
below are the shipped values. Addresses outside the file-backed image are marked
**(ext)**; helper addresses below `0x80A00000` are `main.dol` functions.

## Selection

Ordering is by `functions.tsv` size, largest first. Every row below is absent
from the `address` column of `ledger/functions.csv` (checked against all rows,
read-only), absent from every heading of every `docs/*.md`, and has no
`fn_<addr>` body in `src/`.

| # | address | TSV size | reachable insns | role | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80AD8900` | 308 | 77 | orbit offset around a resolved sub-actor, then a three-axis bias add | **ready** |
| 2 | `0x80AFDD50` | 300 | 75 | `daEnSanboEl_c` step: height lerp, table-driven lean, spin-driven pitch | **ready** |
| 3 | `0x80AB1900` | 256 | 64 | mirrored `+-4` / `+-12` direction pair from one base angle | **ready** |
| 4 | `0x80A43810` | 252 | 63 | follow a resolved sub-actor at a fixed radius with a mirrored heading | **ready** |
| 5 | `0x80AA37E0` | 240 | 60 | 32-point trail generator from one mirrored heading | **ready** |
| 6 | `0x80A79630` | 236 | 59 | two-state sine envelope driver (`Ibaramushi` helper) | **ready** |
| 7 | `0x80ABAB50` | 236 | 59 | `daEnLiftRotHalf_c`: orbit a child actor and publish a local direction | **ready** |
| 8 | `0x80A40290` | 232 | 58 | `daEnCoin_c`: bounce direction from a 4-entry angle table plus a probe point | **ready** |
| 9 | `0x80A20810` | 228 | 57 | radius-vs-vector length predicate (`radius > abs(vec2)`) | **ready** |
| 10 | `0x80A7ADA0` | 220 | 55 | `1.5 / 0.5` squash pair from a doubled angle | **ready** |
| 11 | `0x80A6EA20` | 212 | 53 | `72 / 32` unit offset pair around the heading (shared platform helper) | **ready** |
| 12 | `0x80A1D530` | 208 | 52 | mirrored `32`-unit offset around a cached base point | **ready** |

**Split: 12 ready, 0 conditional.** Total 2,928 bytes of TSV extent and 732
reachable instructions; the reachable word count equals `size / 4` for all
twelve, so every extent is filled exactly and no row borrows its neighbour's
body.

Two rows changed places relative to a naive "largest remaining" list, and both
changes are deliberate:

* `0x80B07D70` (252 B, the `daEnStarCoin_c` glide step) is **excluded**: it is
  still absent from `ledger/functions.csv` but `src/game/enemies.rs` already
  contains a complete `pub unsafe fn fn_80b07d70` body written by the concurrent
  Wave 142 translation lane, so it is not untranslated.
* `0x80A40CA0` (208 B, one `qr3` load plus the `f31` spill pair) is the
  equal-size runner-up to rank 12 and is left for the next tranche; the two
  metrics are tied at 208 bytes and rank 12 takes the lower address.

No row needed a caveat. The `qr3` load is the only blocker the wave-127 table
recorded for these twelve, and it is now gone: every `psq_l ...,qr3` in the
bodies below is exactly `fD = (f32)*(u16 *)rA`, and the `psq_st/psq_l ...qr0`
pairs that appear in six of them are ordinary `f31` stack spills
(`docs/REL_ELF_MAPPING.md:54-55`).

## GQR3 and the qr3 pool (state at this wave)

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`
(`li r3,5` / `oris r3,r3,5` / `mtspr 915,r3`). The RAW stream in this REL holds
no `mtspr GQRn`, so every `psq_*` in this module depends on a `main.dol` word.

Every data `psq` in the twelve bodies below has the shape

```
psq_l fD,0(rA),1,qr3      ; word 0xE0<D><A>B000: DQ = 0, W = 1, qr = 3
```

which with `GQR3 = 0x00050005` is one unsigned 16-bit load scaled by `2^0`:

```
fD = (f32)*(u16 *)rA
```

The idiom is always "stage the halfword on the stack, then load it":

```
sth  rX,10(r1)            ; the s16/u16 field
addi r3,r1,10
psq_l f1,0(r3),1,qr3      ; f1 = (f32)(u16)field
lfs  f0,..(rN)            ; 1/256
fmuls f1,f0,f1
bl   0x80237BD0           ; sin_angle16  (0x80237C40 is cos_angle16)
```

A Rust port reads the `u16` field directly and widens it; no paired-single
traffic is needed.

Counts, read off the twelve extents (`psq_l ...,qr3` sites / `qr0` spill pairs):

| address | `qr3` sites | `qr0` spill pairs | FPR save |
|---|---:|---:|---|
| `0x80AD8900` | 2 | 0 | none |
| `0x80AFDD50` | 1 | 0 | none |
| `0x80AB1900` | 4 | 1 | `f31` |
| `0x80A43810` | 2 | 0 | none |
| `0x80AA37E0` | 2 | 1 | `f31` |
| `0x80A79630` | 2 | 0 | none |
| `0x80ABAB50` | 2 | 0 | none |
| `0x80A40290` | 2 | 0 | none |
| `0x80A20810` | 2 | 1 | `f31` (argument save) |
| `0x80A7ADA0` | 2 | 0 | none |
| `0x80A6EA20` | 2 | 1 | `f31` |
| `0x80A1D530` | 2 | 0 | none |

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000`.
* The REL's file-backed image ends at `0x80B6E4B8`
  (`docs/SPEC_WAVE141_ENEMIES.md:120`); anything above that is a `main.dol`
  address and is marked **(ext)**.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte
  offsets from `this`. `+0xAC`/`+0xB0`/`+0xB4` are x/y/z in every body below
  (the module's actor convention).
* `addi rX,r0,imm` is `li rX,imm`, and `addis rX,r0,imm` is `lis rX,imm`
  (`r0` is not read when `RA == 0`). `or rX,rY,rY` is `mr rX,rY`.
* On a `bc`/`bclr`, `bo = 12` branches on a true CR bit and `bo = 4` on a false
  one; `bclr bo=20 bi=0` is `blr`; `bcctr bo=20 bi=0` with `LK` is `bctrl`.
* `cror 2,1,2` after an `fcmpo` rewrites `cr0.eq` as `gt|eq`, so the following
  `bc 4,2` reads `if !(a >= b)`; `cror 2,0,2` gives `lt|eq`. Both idioms appear
  below and are written in the C form that results.
* `mfcr r3` + `rlwinm r3,r3,2,31,31` extracts **CR0.GT** (bit 1 of the CR word
  rotated into bit 31); shift 1 would extract LT and shift 3 EQ. Rank 9 returns
  through this idiom.
* The paired-single opcodes in this build are `psq_l = 56` and `psq_st = 60`
  (`docs/REL_ELF_MAPPING.md`); the quantiser is bits 12-14, `W` is bit 15.
* A-form float ops: `fadds/fsubs` take their second source from the field at
  bits 16-20, `fmuls/fdivs` from the field at bits 21-25, and the four-operand
  `fmadds/fmsubs` family prints `fD,fA,fC,fB`. This is why `fmuls f1,f0,f1`
  follows the `1/256` load in the pre-scale idiom above.
* `0x80237BD0` is `sin_angle16(f1)` and `0x80237C40` is `cos_angle16(f1)`
  (`docs/SPEC_WAVE140_ENEMIES.md:116-117`); the angle is in 16-bit turn units
  and the pre-scale is `*(1/256)`, i.e. the helpers take a `0..255` "degree".
* TSV `size` is a bucket extent. All twelve bodies below fill their extent
  exactly and the next entry starts at `address + size`.

## Symbolic helpers used below

Roles are the ledger's (`ledger/functions.csv`) or, where the helper is
unledgered, the role established by the cited wave.

| address | name / shape | role |
|---|---|---|
| `0x80162D00` | `actor_by_id(id: u32) -> usize` | ledgered `resource::registry`; `id & 7` selects a 0x20-byte hash bucket, `0` when missing |
| `0x80162590` | `child_actor(this: usize) -> usize` | **unledgered** `main.dol`; returns `*(u32 *)(*(u32 *)(this + 0x10) + 0x10)`, `0` when the node has no child. Ghidra prints it with no arguments; the raw `bl` still holds `r3 = this` |
| `0x8015F4F0` | `step_short_clamped_wrap(value: *mut i16, target: i32, step: i32) -> u32` | ledgered `game::task`; `1` on arrival |
| `0x80064D10` | `actor_flag_write(this, mode: i32)` | **unledgered** `main.dol`; `FUN_80064d10` clears bit 0 of `*(u8 *)(this+0x34C)` when `mode == 0`, else sets bit 3 and writes `300` to `*(u16 *)(this+0x350)` |
| `0x80237BD0` | `sin_angle16(f1) -> f32` | ledgered `resource::bres`; sine through `0x802F74A4` |
| `0x80237C40` | `cos_angle16(f1) -> f32` | ledgered `resource::bres`; cosine through `0x802F74A0` |
| `0x802BE470` | `sqrtf_wrap(f1) -> f32` | ledgered `resource::archive`; square-root wrapper with f32 narrowing |
| `0x80B07880` | `starcoin_glide_step(this)` | ledgered `game::enemies` (translated); base step of the StarCoin body |
| `0x80B07250` | `starcoin_pointer_step(this)` | ledgered `game::enemies` (translated); `+0x6D4` selects x/z and sign |
| `0x80ABA950` | `lift_rot_dir_publish(this)` | **unledgered** in this REL (156 B, itself a qr3 row): `f32[this+0x5D0] = f32[this+0x5F8]*cos16(angle16(this[0x104]))`, `f32[this+0x5D4] = f32[this+0x5F8]*sin16(...)` |
| `0x802DCD70` / `0x802DCDBC` | `_savegpr_26` / `_restgpr_26` | ledgered `runtime::cpp`; CodeWarrior register save/restore, **not** argument-destroying stubs (not used by these twelve) |

---

## 1. `0x80AD8900` - orbit offset around a resolved sub-actor

* **Address:** `0x80AD8900`
* **TSV size:** 308 bytes
* **Reachable body:** 77 instructions, 77 words = the whole extent; 3 direct
  `bl`, no conditional branch at all, one `blr` at `0x80AD8A30`. The straight
  line means the export's "unreachable block" warnings are only the two `qr3`
  pre-scale sequences.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ad8900_FUN_80ad8900.c`
  - the export is a `GQR3`/`ldexpf` mess; the RAW stream has no `psq_st` at
    all, only the two `qr3` loads at `0x80AD8964` and `0x80AD8994`.
* **Proposed signature:** `pub unsafe fn fn_80ad8900(this: usize)`
* **Callers in this REL:** `bl` from `0x80AD8338` (inside `0x80AD7EA0`, 280 B)
  and `0x80AD83D8` (inside `0x80AD83A0`, 272 B). No vtable slot references the
  entry.
* **Constants:** one load through `lis r31,0x80b3`:

| address | type | value / role |
|---|---|---|
| `0x80B309FC` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x0D0/0x0D4/0x0D8` f32 pivot bias;
  `+0x104` `s16` heading (written here); `+0x618/0x61C/0x620` f32 per-axis
  output bias; `+0x6F6` `s16` last sub-actor heading; `+0x6FC` u32 sub-actor id;
  `+0x700` f32 orbit radius; `+0x704` `s16` heading delta; `+0x706` `s16` orbit
  phase delta.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-80(r1)`; LR saved at `+84`; `r31/r30/r29` at `+76/+72/+68` (by hand,
no `_savegpr` helper); no FPR save - `f0..f7` are scratch. Epilogue at
`0x80AD89FC..0x80AD8A30`: restore the three GPRs and LR, `addi r1,r1,80`,
`blr`. The seven `stfs` stack stores at `0x80AD8A0C..0x80AD8A24` sit after the
restores and are dead spills of the scratch locals.

### Behaviour

```
sub   = u32[this+0x6FC]
actor = actor_by_id(sub)                        # 0x80162D00
ax, ay, az = f32[actor+0x0AC], f32[actor+0x0B0], f32[actor+0x0B4]
a0    = s16[actor+0x104]

s16[this+0x6F6] = a0
s16[this+0x104] = (s16)(a0 + s16[this+0x704])    # own heading follows the sub-actor
a  = (s16)(a0 + s16[this+0x706])                 # orbit angle
r  = f32[this+0x700]

x = ax + r * cos_angle16((f32)(u16)a * (1.0/256.0))
y = ay + r * sin_angle16((f32)(u16)a * (1.0/256.0))
z = az

f32[this+0x0AC] = (x - f32[this+0x0D0]) + f32[this+0x618]
f32[this+0x0B0] = (y - f32[this+0x0D4]) + f32[this+0x61C]
f32[this+0x0B4] = (z - f32[this+0x0D8]) + f32[this+0x620]
```

### Notes

* The raw stream computes the three axes out of order (z, x, y at
  `0x80AD89F0/F4/F8`) because the compiler interleaved the `fsubs` chains; the
  expressions above are the Ghidra-verified ones and the stores are the same
  three words.
* `cos` is the call at `0x80AD8974` (`0x80237C40`) and feeds x; `sin` is the
  call at `0x80AD89A0` (`0x80237BD0`) and feeds y.
* Both `qr3` loads read the same 16-bit value, staged twice (`sth r0,10(r1)`
  at `0x80AD8954` and `sth r30,8(r1)` at `0x80AD8988`); `r30` is `extsh` of the
  same sum.
* `actor_by_id` may return `0`; the retail code then dereferences `+0xAC` of
  the null page, so the sub-actor is assumed live.

---

## 2. `0x80AFDD50` - `daEnSanboEl_c` lean/pitch step

* **Address:** `0x80AFDD50`
* **TSV size:** 300 bytes
* **Reachable body:** 75 instructions = the whole extent; 3 direct `bl`
  (`0x80162590`, `0x8015F4F0`, `0x80237BD0`), two conditional branches
  (`0x80AFDDC0` and the `b` at `0x80AFDDD4`), one `blr` at `0x80AFDE78`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80afdd50_FUN_80afdd50.c`
  - single `qr3` load at `0x80AFDE18`; the export's `ldexpf` block is the
    `2^52 + 2^31` integer-to-float idiom, not paired-single traffic.
* **Proposed signature:** `pub unsafe fn fn_80afdd50(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B65438` - the state-handler
  slot of the `daEnSanboEl_c` table whose name block starts at `0x80B654D8`
  (`daEnSanboEl_c::StateID_Move`, `::StateID_Vacume`,
  `::StateID_DamageJump`, ...; model files `g3d/sanbo.brres`, `sanbo_body`,
  `sanbo_head` at `0x80B6565C..0x80B65688`). No `bl` targets the entry.
* **Constants:** from `0x80B31750` (`lis r31,0x80b3` +
  `addi r31,r31,5968`):

| address | type | value / role |
|---|---|---|
| `0x80B31750` | `s16[4]` | `{0x1000, 0xF000, 0x0000, 0x0000}` - lean table for `+0x604 == 0` |
| `0x80B31758` | `s16[8]` | `{0xEC00, 0x1400, 0x0800, 0xF800, 0x1C00, 0xE400, 0xF400, 0x0C00}` - four lean pairs, group `(mode-1) & 3` |
| `0x80B317B8` | f64 | `4503601774854144.0` = `2^52 + 2^31`, the s32-to-f32 bias |
| `0x80B317C8` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B317CC` | f32 | `8.0` - height offset above the child |
| `0x80B317F0` | f32 | `16.0` - scale of the `+0x608` height field |
| `0x80B317F4` | f32 | `1536.0` - pitch amplitude |

* **Fields:** `+0x010` child node (read by `0x80162590`); `+0x0B0` y;
  `+0x102` `s16` pitch (written here); `+0x5CA` `s16` spin phase; `+0x5CC`
  `s16` lean; `+0x604` s32 lean mode; `+0x608` s32 height input. Child fields
  read: `+0x0B0` y, `+0x52C` s32 lean index.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+52`; `r31/r30` at `+44/+40`; no FPR save. Epilogue
`addi r1,r1,48`, `blr`.

### Behaviour

```
child = child_actor(this)                        # 0x80162590, r3 = this
f32[this+0x0B0] = 8.0 + f32[child+0x0B0]
                + 16.0 * (f32)(s32)u32[this+0x608]

idx = s32[child+0x52C]
if (s32[this+0x604] == 0)
    target = s16[0x80B31750 + idx*2]
else
    target = s16[0x80B31758 + (((s32[this+0x604] - 1) * 4) & 0xC) + idx*2]
step_short_clamped_wrap(this+0x5CC, target, 0x100)        # 0x8015F4F0

a = (s16)(s16[this+0x5CA] + 0x800)                        # 1/8 turn per frame
s16[this+0x5CA] = a
s16[this+0x102] = (s16)(int)(
    (f32)(s16)this[0x5CC] + 1536.0 * sin_angle16((f32)(u16)a * (1.0/256.0)))
```

### Notes

* The `2^52 + 2^31` subtraction is exactly `(f32)(s32)x`: the word at
  `0x80B317B8` cancels both the `2^52` prefix and the `xoris ...,0x8000` sign
  flip.
* The lean table index is the child's `+0x52C`; the pair selector is
  `(mode - 1) & 3`, so mode 1..4 picks `{0xEC00,0x1400}`, `{0x0800,0xF800}`,
  `{0x1C00,0xE400}`, `{0xF400,0x0C00}`. The `& 0xC` mask is the `rlwinm
  6,4,2,28,29` at `0x80AFDDE8`.
* `step_short_clamped_wrap` is called with `step = 0x100` and returns an
  arrival flag the original ignores.
* The pitch store is `fctiwz` + `stfd`/`lwz` followed by `sth`: the converted
  float is truncated toward zero and only the low halfword reaches
  `s16[this+0x102]`.

---

## 3. `0x80AB1900` - mirrored `+-4` / `+-12` direction pair

* **Address:** `0x80AB1900`
* **TSV size:** 256 bytes
* **Reachable body:** 64 instructions = the whole extent; 4 direct `bl` (two
  `sin` at `0x80237BD0`, two `cos` at `0x80237C40`), no branches, one `blr` at
  `0x80AB19FC`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80ab1900_FUN_80ab1900.c`
  - four `qr3` loads at `0x80AB1954`, `0x80AB1974`, `0x80AB199C`, `0x80AB19BC`;
    the `qr0` pair is the `f31` prologue spill (`stfd f31,32` +
    `psq_st f31,40,...,0,qr0`).
* **Proposed signature:** `pub unsafe fn fn_80ab1900(this: usize)`
* **Callers in this REL:** `bl` from `0x80AB2380` (inside `0x80AB22D0`, 276 B)
  and `0x80AB275C` (inside `0x80AB26A0`, 408 B - the body that resets
  `+0x73C`/`+0x794` through `0x80166340` and calls `0x80AB1780`).
* **Constants:** from `0x80B2FA50` (`lis r31,0x80b3` +
  `addi r31,r31,-1456`):

| address | type | value / role |
|---|---|---|
| `0x80B2FAE0` | f32 | `-4.0` - amplitude of the `+0x164/+0x168` vector |
| `0x80B2FAEC` | f32 | `12.0` - amplitude of the `+0x7DC/+0x7E0` vector |
| `0x80B2FB48` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x100` `s16` base heading; `+0x164/0x168` f32 fast direction
  pair; `+0x348` u8 side/mirror byte; `+0x7DC/0x7E0` f32 slow direction pair.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+52`; `stfd f31,32(r1)` + `psq_st f31,40(r1),0,qr0`
(the standard `f31` spill pair); `r31/r30/r29` at `+28/+24/+20`. Epilogue
mirrors it (`psq_l f31,40(r1),0,qr0`, `lfd f31,32(r1)`), `addi r1,r1,48`,
`blr`.

### Behaviour

```
a = (s16)(s16[this+0x100] - 0x4000)              # base heading - 90 deg
if (u8[this+0x348] != 0) a = (s16)(-a)           # side mirror

f32[this+0x164] = -4.0  * sin_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x168] = -4.0  * cos_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x7DC] = 12.0  * sin_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x7E0] = 12.0  * cos_angle16((f32)(u16)a * (1.0/256.0))
```

### Notes

* Each axis pair re-stages the same `s16` into the stack before its `qr3`
  load (`sth r30,14(r1)`, `sth r30,12(r1)`, `sth r30,10(r1)`, `sth r30,8(r1)`),
  so all four products use one angle.
* This body fills the two vectors the sibling `0x80AB1890` seeds (wave-127
  note: `+0x164` and `+0x7DC`); the amplitudes are negative because the pair is
  published as a backwards direction.
* The `f31` spill pair is cosmetic: `f31` holds the `sin` product across the
  `cos` call.

---

## 4. `0x80A43810` - follow a sub-actor at a fixed radius

* **Address:** `0x80A43810`
* **TSV size:** 252 bytes
* **Reachable body:** 63 instructions = the whole extent; 4 direct `bl`
  (`0x80162D00`, `0x80237C40`, `0x80237BD0`, `0x80064D10`), one conditional
  branch with an `else` tail `b` at `0x80A438E0`, one `blr` at `0x80A43908`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a43810_FUN_80a43810.c`
  - two `qr3` loads at `0x80A43870` and `0x80A438A0`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80a43810(this: usize)`
* **Callers in this REL:** `bl` from `0x80A43578` (inside `0x80A432D0`,
  276 B), `0x80A439E4` (inside `0x80A439A0`, 128 B) and `0x80A43A60` (inside
  `0x80A43A40`, 148 B).
* **Constants:** `lis r31,0x80b3` + `lfs f0,-14568(r31)`:

| address | type | value / role |
|---|---|---|
| `0x80B2C718` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x104` `s16` heading (written
  here); `+0x900` u32 sub-actor id; `+0x904` f32 follow radius; `+0x910`
  `s16` heading offset. Sub-actor fields: `+0x4B8/0x4BC/0x4C0` f32 anchor
  position, `+0x104` `s16` heading.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+52`; `r31/r30/r29` at `+44/+40/+36`; no FPR save.
Epilogue `addi r1,r1,48`, `blr`.

### Behaviour

```
sub = actor_by_id(u32[this+0x900])               # 0x80162D00
if (sub == 0) {
    actor_flag_write(this, 0)                    # 0x80064D10
    return
}

a = (s16)(s16[sub+0x104] + 0x4000 - s16[this+0x910])
r = f32[this+0x904]
f32[this+0x0AC] = f32[sub+0x4B8] + r * cos_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x0B0] = f32[sub+0x4BC] + r * sin_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x0B4] = f32[sub+0x4C0]
s16[this+0x104] = (s16)(a - 0x4000)
```

### Notes

* The angle arithmetic wraps in `s16`: `+0x4000` is added so the offset is
  applied to the sub-actor's *rear* direction, then the same value minus
  `0x4000` becomes the follower's own heading.
* `0x80064D10(this, 0)` is the "detach/hide" path: it clears render bit 0 in
  `*(u8 *)(this+0x34C)` without touching the `300` timeout word (that is the
  `mode == 1` branch).
* x is built from the `cos` call and y from `sin`, both at radius `+0x904`;
  z is copied straight through, so the follower stays on the anchor's plane.

---

## 5. `0x80AA37E0` - 32-point trail generator

* **Address:** `0x80AA37E0`
* **TSV size:** 240 bytes
* **Reachable body:** 60 instructions = the whole extent; 2 direct `bl`
  (`0x80237C40` cos, `0x80237BD0` sin); one loop branch at `0x80AA38A8`
  (`bc 12,0` back to `0x80AA3880`); one `blr` at `0x80AA38CC`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa37e0_FUN_80aa37e0.c`
  - two `qr3` loads at `0x80AA3828` and `0x80AA3848`; the `qr0` pair is the
  `f31` prologue spill.
* **Proposed signature:** `pub unsafe fn fn_80aa37e0(this: usize)`
* **Callers in this REL:** `bl` from `0x80AA50B4` (inside `0x80AA5060`,
  108 B).
* **Constants:** `lis r31,0x80b3` + `lfs f0,-2764(r31)`:

| address | type | value / role |
|---|---|---|
| `0x80B2F534` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x0AC/0x0B0` f32 x/y (read only); `+0x814`..`+0x913` 32 f32
  `(x, y)` pairs (written here); `+0xA4E` `s16` source heading.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-64(r1)`; LR at `+68`; `stfd f31,48` + `psq_st f31,56,0,qr0`;
`r31/r30/r29` at `+44/+40/+36`. Epilogue mirrors it, `addi r1,r1,64`, `blr`.

### Behaviour

```
a = s16[this+0xA4E]
b = (a >= 0) ? (s16)(a - 0x8000) : (s16)(-0x8000 - a)   # == |a| - 0x8000

c = cos_angle16((f32)(u16)b * (1.0/256.0))
s = sin_angle16((f32)(u16)b * (1.0/256.0))

x = f32[this+0x0AC]
y = f32[this+0x0B0]
for (i = 0; i < 32; i++) {
    x = x + c
    y = y + s
    f32[this+0x814 + i*8 + 0] = x
    f32[this+0x814 + i*8 + 4] = y
}
```

### Notes

* 0x914 - 0x814 = 0x100 bytes = 32 records of 8 bytes; the loop compares
  `r3` against `this+0x914` after each `addi r3,r3,8` (`0x80AA3898..0x80AA38A8`).
* The accumulator lives on the stack (`stfs f1,16(r1)` / `stfs f0,20(r1)` at
  the end of each iteration) and is seeded from `this+0xAC/0xB0`, so point *n*
  is `(this.x + (n+1)*c, this.y + (n+1)*s)`.
* The mirror formula is `b = |a| - 0x8000`: for `a >= 0` it is
  `a - 0x8000`; for `a < 0` the `subfic r0,r4,-32768` at `0x80AA3818` is
  `-0x8000 - a`.
* `frsp` (`0x80AA385C`, `0x80AA3864`) only narrows the two helper results; the
  body is scalar.

---

## 6. `0x80A79630` - two-state sine envelope driver

* **Address:** `0x80A79630`
* **TSV size:** 236 bytes
* **Reachable body:** 59 instructions = the whole extent; 2 direct `bl`
  (both `0x80237BD0` sin); a 3-way state dispatch, one phase-threshold test,
  two `b` joins, one `blr` at `0x80A79718`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a79630_FUN_80a79630.c`
  - two `qr3` loads at `0x80A79674` and `0x80A796C8`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80a79630(this: usize)`
* **Callers in this REL:** `bl` from `0x80A7C55C` (inside `0x80A7BEC0`,
  1,464 B) and from `0x80A7C664` / `0x80A7C778` (both inside `0x80A7C600`,
  184 B). Those two are the `daEnIbaramushi_c` state bodies (name block
  `0x80B4CC34`), so this is an `Ibaramushi` helper record, not an actor.
* **Constants:** `lis r3,0x80b3` + `lfs f0,-8668(r3)`:

| address | type | value / role |
|---|---|---|
| `0x80B2DE24` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B2DE28` | f32 | `1.0` - offset of the falling half |

* **Fields:** the record is at least 12 bytes: `+0x00` u32 config pointer
  (`+0x00` f32 amplitude, `+0x04` `s16` phase delta), `+0x04` f32 output,
  `+0x08` `s16` phase, `+0x0A` u8 state.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31/r30/r29` at `+28/+24/+20`; no FPR save.
Epilogue `addi r1,r1,32`, `blr`.

### Behaviour

```
state = u8[this+0x0A]
if (state == 1) {
    cfg = u32[this+0x00]
    a   = s16[this+0x08]
    f32[this+0x04] = sin_angle16((f32)(u16)a * (1.0/256.0)) * f32[cfg+0x00]
    n = (s16)(a + s16[cfg+0x04])
    s16[this+0x08] = n
    if (n >= 0x4000) u8[this+0x0A] = 2
} else if (state == 2) {
    cfg = u32[this+0x00]
    a   = s16[this+0x08]
    f32[this+0x04] = (1.0 + f32[cfg+0x00])
                     - sin_angle16((f32)(u16)a * (1.0/256.0))
    s16[this+0x08] = (s16)(a + s16[cfg+0x04])
}
```

### Notes

* State 1 is the rising half (`A*sin`), state 2 the falling half
  (`1 + A - sin`), so the two branches meet at the same value at the switch.
* The threshold is `0x4000` (90 deg) checked on the **new** phase
  (`0x80A7969C..0x80A796B0`), so a caller that seeds `+0x08` below `0x4000`
  always gets at least one state-1 frame.
* The same phase advances in both states, so the record can be re-entered after
  a state-2 -> state-1 reset without a separate timer.
* State values other than 1 and 2 are no-ops.

---

## 7. `0x80ABAB50` - `daEnLiftRotHalf_c` orbit + direction publish

* **Address:** `0x80ABAB50`
* **TSV size:** 236 bytes
* **Reachable body:** 59 instructions = the whole extent; 4 direct `bl`
  (`0x80162D00`, `0x80237C40`, `0x80237BD0`, `0x80ABA950`), one `bctrl`
  through the sub-object at `+0x584`, one `blr` at `0x80ABAC38`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80abab50_FUN_80abab50.c`
  - two `qr3` loads at `0x80ABABA4` and `0x80ABABD8`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80abab50(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B56ED0` - the state-handler
  slot of the `daEnLiftRotHalf_c` table (name block at `0x80B56EE0`
  `daEnLiftRotHalf_...`; neighbours `0x80ABAAC0`, `0x80ABAAD0`, `0x80ABAA80`,
  `0x80ABAB40`, `0x80ABAB30`). No `bl` targets the entry.
* **Constants:** `lis r31,0x80b3` + `lfs f0,-96(r31)`:

| address | type | value / role |
|---|---|---|
| `0x80B2FFA0` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x104` `s16` heading (written);
  `+0x5C4` `s16` mirrored heading (written); `+0x5E4/0x5E8/0x5EC` f32 cached
  child position; `+0x5F0` u32 child id; `+0x5F8` f32 orbit radius; `+0x5FC`
  `s16` heading delta; `+0x5FE` `s16` last child heading; `+0x584` vtable
  pointer of the rotating half-node.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31/r30` at `+28/+24`; no FPR save. Epilogue
`addi r1,r1,32`, `blr`.

### Behaviour

```
child = actor_by_id(u32[this+0x5F0])             # 0x80162D00
f32[this+0x5E4] = f32[child+0x0AC]
f32[this+0x5E8] = f32[child+0x0B0]
f32[this+0x5EC] = f32[child+0x0B4]

a0 = s16[child+0x104]
s16[this+0x5FE] = a0
a  = (s16)(a0 + s16[this+0x5FC])
s16[this+0x104] = a

r = f32[this+0x5F8]
f32[this+0x0AC] = f32[this+0x5E4] + r * cos_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x0B0] = f32[this+0x5E8] + r * sin_angle16((f32)(u16)a * (1.0/256.0))

lift_rot_dir_publish(this)                        # 0x80ABA950
#   -> f32[this+0x5D0] = r * cos(a), f32[this+0x5D4] = r * sin(a)

s16[this+0x5C4] = (s16)(s16[this+0x104] + 0x4000)
(*(vtable)u32[this+0x584])[0x0C](this + 0x584)     # slot 3, receiver = this+0x584
```

### Notes

* The sub-object call is a plain vtable dispatch: `lwz r12,0x584(r30)`,
  `lwz r12,0xC(r12)`, `mtspr CTR,r12`, `bctrl`, with `r3 = this+0x584`. No
  other GPR argument is staged; `f1` still holds `r * sin(a)` from the
  preceding `0x80ABA950` call (that helper leaves `f1 = f32[this+0x5F8] *
  sin`), so keep the same value live if the slot is ported as a trait call.
* `0x80ABA950` is itself an unclaimed qr3 row (156 B) and is reproduced in the
  helper table above, so a lane can inline it or call it by analysis address.
* z is never recomputed: the body only publishes x/y and the two cached
  direction words, matching the `RotHalf` (top half of a rotating lift) role.
* The `+0x5C4` mirror (`heading + 0x4000`) is the value the receiving half-node
  reads back.
---

## 8. `0x80A40290` - `daEnCoin_c` bounce direction + probe point

* **Address:** `0x80A40290`
* **TSV size:** 232 bytes
* **Reachable body:** 58 instructions = the whole extent; 2 direct `bl`
  (`0x80237C40` cos, `0x80237BD0` sin), one side branch at `0x80A40324`, one
  `blr` at `0x80A40374`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a40290_FUN_80a40290.c`
  - two `qr3` loads at `0x80A402CC` and `0x80A402FC`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80a40290(this: usize)`
* **Callers in this REL:** one 32-bit word at `0x80B3F59C` - a `daEnCoin_c`
  state slot (name block `0x80B3F708` `daEnCoin_c::StateID_Move`,
  `::StateID_WaitDown`, `::StateID_Jump`, `::StateID_Down`, ...; neighbours
  `0x80A40210`, `0x80A40220`, `0x80A401D0`, `0x80A40380`, `0x80A40390`).
* **Constants:** from `0x80B2C4E8` (`lis r30,0x80b3` +
  `addi r30,r30,-15128`; `r31 = r30 + 0x11C`):

| address | type | value / role |
|---|---|---|
| `0x80B2C540` | f32 | `8.0` - x offset of the probe point |
| `0x80B2C568` | f32 | `16.0` - y offset of the probe point |
| `0x80B2C5FC` | f32 | `0.5` - direction amplitude |
| `0x80B2C604` | `s16[4]` | `{0x0000, 0x1000, 0x2000, 0x0000}` - bounce angles (0, 22.5, 45, 0 deg) |
| `0x80B2C60C` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** `+0x0AC/0x0B0/0x0B4` x/y/z; `+0x0E8/0x0EC` f32 direction pair;
  `+0x348` u8 side/mirror byte; `+0x6E8/0x6EC/0x6F0` f32 probe point;
  `+0xA95` u8 angle-table index.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31/r30/r29` at `+28/+24/+20`; no FPR save.
Epilogue `addi r1,r1,32`, `blr`.

### Behaviour

```
i = u8[this+0xA95]
a = s16[0x80B2C604 + i*2]

f32[this+0x0E8] =  0.5 * cos_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x0EC] = -0.5 * sin_angle16((f32)(u16)a * (1.0/256.0))
if (u8[this+0x348] != 0)
    f32[this+0x0E8] = -f32[this+0x0E8]

f32[this+0x6E8] = f32[this+0x0AC] - 8.0
f32[this+0x6EC] = f32[this+0x0B0] + 16.0
f32[this+0x6F0] = f32[this+0x0B4]
```

### Notes

* The table read is `lhax r0,r31,r0` with `r0 = u8 * 2` (`rlwinm 0,0,1,0,30`),
  i.e. `s16` at `0x80B2C604 + index*2`; only indices 0..3 stay inside the
  four-entry table - `i` is expected to be a small enum.
* The `fneg` at `0x80A4031C` precedes the side check, so the second component
  (`+0x0EC`) is always negated while the first (`+0x0E8`) is negated only on
  the mirrored side.
* The probe point is the coin's own position with `-8` on x and `+16` on y;
  it is what the following state body hands to the contact queries.

---

## 9. `0x80A20810` - radius vs 2-vector length predicate

* **Address:** `0x80A20810`
* **TSV size:** 228 bytes
* **Reachable body:** 57 instructions = the whole extent; 4 direct `bl`
  (`0x80237C40` cos, `0x80237BD0` sin, two `0x802BE470` sqrt); no branches,
  one `blr` at `0x80A208F0`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a20810_FUN_80a20810.c`
  - two `qr3` loads at `0x80A20844` and `0x80A20868`; the `qr0` pair is the
    `f31` argument save (`stfd f31,48` + `psq_st f31,56,0,qr0`).
* **Proposed signature:**
  `pub unsafe fn fn_80a20810(radius: f32, vec: *const f32, angle: i16) -> bool`
  - `r3` is dead on entry; `f1 = radius`, `r4 = vec` (two f32),
  `r5 = angle` (`s16`, low half used).
* **Callers in this REL:** `bl` from `0x80A209D8` (inside `0x80A20900`,
  520 B). The call site stages `f1` with `lfs f1,-18584(r23)` - that is
  `f32[0x80B2B768] = 400.0` - passes `r4 = r1+16` (the two f32 `|dx|,|dy|`
  that the loop filled from the player's position minus `puVar2[0x2B/0x2C]`,
  i.e. this actor's x/y) and `r5 = extsh(r0)` of the `bl 0x80A207E0` result,
  then branches on `cmpi cr0,r3,0`. With `400.0` as the radius the predicate
  reads "the player is within 400 units".
* **Constants:** `lis r31,0x80b3` + `lfs f0,-18588(r31)`:

| address | type | value / role |
|---|---|---|
| `0x80B2B764` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |

* **Fields:** none; the body reads `vec[0]`, `vec[1]` through `r4` and writes
  nothing.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-64(r1)`; LR at `+68`; `stfd f31,48` + `psq_st f31,56,0,qr0`
(argument `f1` parked in `f31`); `r31/r30/r29` at `+44/+40/+36`. Epilogue
mirrors it, `addi r1,r1,64`, `blr`.

### Behaviour

```
c = cos_angle16((f32)(u16)angle * (1.0/256.0))
s = sin_angle16((f32)(u16)angle * (1.0/256.0))

u  = radius * c
v  = radius * s
m1 = sqrtf(u*u + v*v)                 # == |radius| up to rounding
m2 = sqrtf(vec[0]*vec[0] + vec[1]*vec[1])

return fabsf(m1) > fabsf(m2)          # r3 = 0/1, taken from CR0.GT
```

### Notes

* The boolean comes from `fcmpo cr0,f31,f0` followed by `mfcr r3` and
  `rlwinm r3,r3,2,31,31` (`0x80A208C4..0x80A208D4`), which isolates **GT**;
  `f31` is `|m1|` and `f0` is `|m2|`.
* The Ghidra prototype (`double param_1, undefined4 param_2, float *param_3,
  short param_4`) mis-assigns the argument registers; the raw stream is
  `f1`/`r4`/`r5` as stated above (the callee does `mr r29,r4`, `mr r30,r5`,
  `fmr f31,f1` before any other use).
* Because `cos^2 + sin^2 == 1`, `m1` is the magnitude of `radius`, so the
  predicate is effectively `|radius| > |vec|`; keeping the trig is required
  only for bit-exactness.

---

## 10. `0x80A7ADA0` - `1.5 / 0.5` squash pair from a doubled angle

* **Address:** `0x80A7ADA0`
* **TSV size:** 220 bytes
* **Reachable body:** 55 instructions = the whole extent; 2 direct `bl`
  (both `0x80237BD0` sin); no branches, one `blr` at `0x80A7AE78`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a7ada0_FUN_80a7ada0.c`
  - two `qr3` loads at `0x80A7ADD4` and `0x80A7AE04`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80a7ada0(this: usize)`
* **Callers in this REL:** `bl` from `0x80A7B490` and `0x80A7B5FC`, both
  inside `0x80A7B370` (116 B).
* **Constants:** from `0x80B2DDF0` (`lis r30,0x80b3` +
  `addi r30,r30,-8720`):

| address | type | value / role |
|---|---|---|
| `0x80B2DDFC` | f32 | `2.0` - gain of the `+0xE84` term |
| `0x80B2DE00` | f32 | `1.5` - upper squash base |
| `0x80B2DE04` | f32 | `0.5` - lower squash base |
| `0x80B2DE24` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B2DE28` | f32 | `1.0` - neutral scale |
| `0x80B2E10C` | f32 | `0.5` - quarter-amplitude factor (f32 path) |
| `0x80B2E110` | f64 | `0.5` - quarter-amplitude factor (double path) |

* **Fields:** `+0xE84/0xE88/0xE8C` f32 scale triple; `+0xE90` `s16` angle.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31/r30/r29/r28` at `+28/+24/+20/+16`; no
FPR save. Epilogue restores the four GPRs, `addi r1,r1,32`, `blr`.

### Behaviour

```
a = s16[this+0xE90]
f32[this+0xE84] = 2.0 * sin_angle16((f32)(u16)a * (1.0/256.0))

b = (s16)(a * 2 - 0x4000)
s = sin_angle16((f32)(u16)b * (1.0/256.0))

f32[this+0xE88] = 1.0 + ( 0.25 + 0.25 * s)
f32[this+0xE8C] = 1.0 + (-0.25 - 0.25 * s)
```

### Notes

* The two quarter terms are `(1.5 - 1.0) * 0.5` (f32) and `(0.5 - 1.0) * 0.5`
  (f64) computed in the stream at `0x80A7AE20..0x80A7AE38`; keep the constants
  `0.25` and `-0.25` in the port and the two writes stay bit-identical.
* `+0xE88` and `+0xE8C` always sum to `2.0` and stay in `[1.0, 1.5]` /
  `[0.5, 1.0]`, i.e. a squash/stretch pair driven by a doubled heading.
* The second angle is the doubled heading minus a quarter turn
  (`(s16)(a*2 - 0x4000)`), so the pair is in antiphase about `1.0`.

---

## 11. `0x80A6EA20` - `72 / 32` unit offset pair around the heading

* **Address:** `0x80A6EA20`
* **TSV size:** 212 bytes
* **Reachable body:** 53 instructions = the whole extent; 2 direct `bl`
  (`0x80237C40` cos, `0x80237BD0` sin); no branches, one `blr` at `0x80A6EAF0`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a6ea20_FUN_80a6ea20.c`
  - two `qr3` loads at `0x80A6EA54` and `0x80A6EA74`; the `qr0` pair is the
    `f31` spill.
* **Proposed signature:** `pub unsafe fn fn_80a6ea20(this: usize)`
* **Callers in this REL:** `bl` from `0x80A6E638` (inside `0x80A6E4D0`,
  132 B, a `daEnGuraYuka_c` state body whose table is `0x80B4A840`, name block
  `0x80B4A7EC`) and from `0x80A6E9FC` (inside `0x80A6E9C0`, 84 B, a
  `daEnGuru_c` state body whose table is `0x80B4AB1C`, name block
  `0x80B4AB2C`). It is the shared platform offset helper for both classes, and
  the current `fn_80a6e9c0` body already calls it by analysis address
  (`const FN_80A6EA20` in `src/game/enemies.rs`).
* **Constants:** from `0x80B2D9D0` (`lis r31,0x80b3` +
  `addi r31,r31,-9776`):

| address | type | value / role |
|---|---|---|
| `0x80B2DA20` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B2DA24` | f32 | `72.0` - outer offset |
| `0x80B2DA28` | f32 | `3.0` - downward bias of the outer point |
| `0x80B2DA2C` | f32 | `32.0` - inner offset |

* **Fields:** `+0x0AC/0x0B0` f32 x/y; `+0x104` `s16` heading; `+0x164/0x168`
  f32 outer direction; `+0x5BC/0x5C0` f32 outer point; `+0x5C8/0x5CC` f32 inner
  point.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+52`; `stfd f31,32` + `psq_st f31,40,0,qr0`;
`r31/r30` at `+28/+24`. Epilogue mirrors it, `addi r1,r1,48`, `blr`.

### Behaviour

```
a = s16[this+0x104]
c = cos_angle16((f32)(u16)a * (1.0/256.0))
s = sin_angle16((f32)(u16)a * (1.0/256.0))

f32[this+0x164] = 72.0 * c
f32[this+0x168] = 72.0 * s - 3.0

f32[this+0x5BC] = f32[this+0x0AC] + 72.0 * c
f32[this+0x5C0] = f32[this+0x0B0] + 72.0 * s
f32[this+0x5C8] = f32[this+0x0AC] + 32.0 * c
f32[this+0x5CC] = f32[this+0x0B0] + 32.0 * s
```

### Notes

* The outer point is the heading direction at radius 72 with a `-3` bias
  stored into `+0x168` only, not into `+0x5C0`; the inner point is the same
  direction at radius 32.
* Nothing writes z; both platforms only need the horizontal footprint.
* `f31` parks the `cos` product across the `sin` call; the pair is cosmetic.

---

## 12. `0x80A1D530` - mirrored 32-unit offset around a cached base

* **Address:** `0x80A1D530`
* **TSV size:** 208 bytes
* **Reachable body:** 52 instructions = the whole extent; 2 direct `bl`
  (`0x80237C40` cos, `0x80237BD0` sin); one side branch at `0x80A1D554` with a
  `b` join at `0x80A1D568`; one `blr` at `0x80A1D5FC`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a1d530_FUN_80a1d530.c`
  - two `qr3` loads at `0x80A1D588` and `0x80A1D5C0`; no `qr0` pair.
* **Proposed signature:** `pub unsafe fn fn_80a1d530(this: usize)`
* **Callers in this REL:** `bl` from `0x80A1D650` (inside `0x80A1D600`,
  116 B) and `0x80A1D74C` (inside `0x80A1D690`, 212 B).
* **Constants:** `lis r30,0x80b3` + `lfs f0,-18780(r30)` and
  `lis r31,0x80b3` + `lfs f2,-18776(r31)`:

| address | type | value / role |
|---|---|---|
| `0x80B2B6A4` | f32 | `0.00390625` = `1/256`, turn-units pre-scale |
| `0x80B2B6A8` | f32 | `32.0` - offset radius |

* **Fields:** `+0x0AC/0x0B0` f32 x/y (written); `+0x348` u8 side/mirror byte;
  `+0x7EC/0x7F0` f32 base point (x/y); `+0x816` `s16` mirrored angle;
  `+0x818` `s16` working angle; `+0x81A` `s16` source angle.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-32(r1)`; LR at `+36`; `r31/r30/r29` at `+28/+24/+20`; no FPR save.
Epilogue `addi r1,r1,32`, `blr`.

### Behaviour

```
src = s16[this+0x81A]
if (u8[this+0x348] == 0) {
    s16[this+0x816] = src
    s16[this+0x818] = (s16)(-0x8000 - src)     # half-turn mirror
} else {
    s16[this+0x818] = src
    s16[this+0x816] = (s16)(-src)              # side mirror
}

a = s16[this+0x818]
f32[this+0x0AC] = f32[this+0x7EC] + 32.0 * cos_angle16((f32)(u16)a * (1.0/256.0))
f32[this+0x0B0] = f32[this+0x7F0] + 32.0 * sin_angle16((f32)(u16)a * (1.0/256.0))
```

### Notes

* Both branches fill the pair `(+0x816, +0x818)`; only `+0x818` feeds the
  trig, while `+0x816` is the value a later body reads back.
* `-0x8000 - src` is the `subfic r0,r0,-32768` at `0x80A1D574`; it is an exact
  `s16` half-turn rotation of the mirrored side, not a negation.
* The base point `+0x7EC/+0x7F0` is written elsewhere; this body only reads it
  and publishes the offset position, so the caller owns the base point.

---

## Verification

* Twelve addresses, all absent from `ledger/functions.csv` (checked against
  every row, read-only), from every `docs/*.md` heading and from every
  `pub unsafe fn` body in `src/` at the time of writing.
* Excluded on purpose: `0x80B07D70`, which already has a `fn_80b07d70` body in
  `src/game/enemies.rs`; `0x80A3A4A0`, `0x80AEDD40`, `0x80AEF130`,
  `0x80AA93F0`, `0x80AFDAC0`, `0x80A429C0`, `0x80AA4070`, `0x80AA35E0`,
  `0x80AFD910`, `0x80B1D520`, `0x80A436C0`, `0x80A24AD0`, `0x80A0E580`,
  `0x80B0C540`, `0x80AEB450`, `0x80AE4A50`, `0x80A4B740`, `0x80A4B420`,
  `0x80A635D0`, `0x80A63FF0`, `0x80A641F0`, `0x80ABA080`, `0x80A4BBD0` and
  `0x80A46200`, which are already spec headings of waves 140-141; and
  `0x80A79810`, `0x80AEC4E0`, `0x80AEEFA0` / `0x80A337B0`, which are class D
  for `extraout_`/alias reasons rather than for a qr3 load.
* Every byte count equals the `size` column of
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`, and for every row
  `size / 4` equals the reachable instruction count, so each body ends on the
  instruction before the next entry.
* Every `psq_l ...,qr3` in the twelve extents is a single `u16` widen; no
  `mtspr GQRn` exists in the REL (whole-image scan), and no `qr1..qr7` value is
  consumed anywhere in these bodies.
* Every constant quoted in a **Constants** table was read from
  `work/rels/elf/d_enemiesNP.elf` at the address shown. None of the twelve
  bodies reads above the file-backed end, so no **(ext)** marker was needed;
  the `400.0` radius that rank 9's caller passes is `f32[0x80B2B768]`, also in
  the image.
* No Rust file, ledger CSV, coverage doc or existing specification was
  modified by this lane.
