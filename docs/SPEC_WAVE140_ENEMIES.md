# Wave 140 - implementation specification for 12 class-D `d_enemiesNP` bodies

## What this page is

Wave 127 (`docs/ANALYSIS_WAVE127_ENEMIES.md`) reclassified 384 formerly-skipped
`d_enemiesNP` addresses into 173 class **B** (reconstructible now), 160 class C
and 51 class D. Waves 128-139 (`docs/SPEC_WAVE128_ENEMIES.md` ..
`docs/SPEC_WAVE139_ENEMIES.md`) specified 132 of the B rows, and the Wave 140
implementation lane then drained the last nine B rows
(`docs/COVERAGE_WAVE140_ENEMIES.md`).

That leaves **no class-B row**: a live re-walk of all 173 B addresses against
`ledger/functions.csv`, every `docs/*.md` heading and every `pub unsafe fn
fn_<addr>` body in `src/` returns zero untranslated entries. This lane therefore
takes the second half of the wave-127 selection rule: the **largest class-D
rows whose only blocker was a `psq_l ...,qr3` u16 load, now
`(f32)*(u16 *)`** because `GQR3 = 0x00050005` is pinned
(`docs/REL_ELF_MAPPING.md`).

This page is **read-only**: it writes no Rust, no ledger row and no other doc.

Each body was decoded from `work/rels/elf/d_enemiesNP.elf` with the `+0x1000`
segment shift of `docs/REL_ELF_MAPPING.md` (first `PT_LOAD` is
`p_offset = 0x10F0`, `p_vaddr = 0x80A000F0`), cross-checked against the Ghidra
export at the cited path and against the `size` column of
`work/ghidra/rel-d_enemiesNP/export/functions.tsv`. Constants that live inside
the REL image were read out of the ELF itself, so the integer and float values
below are the shipped values. Addresses outside the file-backed image are
marked **(ext)**.

## Selection

Ordering is the wave-127 qr3 table of `docs/SPEC_WAVE139_ENEMIES.md` (largest
TSV size first), which is also this pass's reachable-instruction ranking - the
two metrics agree on the same twelve rows. Every row below is absent from the
`address` column of `ledger/functions.csv`, absent from every heading of every
`docs/*.md`, and has no `fn_<addr>` body in `src/`.

| # | address | TSV size | reachable insns | role | verdict |
|---:|---|---:|---:|---|---|
| 1 | `0x80A3A4A0` | 2620 | 655 | Fire Snake (`Wm_en_firesnk`) 150-segment body manager | **conditional** |
| 2 | `0x80AEDD40` | 1208 | 301 | 16-byte waypoint chase with a cached offset vector | **ready** |
| 3 | `0x80AEF130` | 1180 | 294 | sibling waypoint chase with `+0x670` state fields | **ready** |
| 4 | `0x80AA93F0` | 496 | 124 | side-flip steering with `+0x348` facing latch | **ready** |
| 5 | `0x80AFDAC0` | 480 | 120 | vertical seek on `+0x608`, vtable event on arrival | **ready** |
| 6 | `0x80A429C0` | 476 | 119 | orbit/offset follow through `+0xAE4` owner | **ready** |
| 7 | `0x80AA4070` | 444 | 111 | four-corner contact probe around a swept box | **ready** |
| 8 | `0x80AA35E0` | 408 | 102 | angle-to-target plus velocity clamp, Q12 facing | **ready** |
| 9 | `0x80AFD910` | 388 | 97 | fall onto a `+0x604` row table and bank the model | **ready** |
| 10 | `0x80B1D520` | 352 | 88 | trail-normal from a 6-entry angle table | **ready** |
| 11 | `0x80A436C0` | 328 | 82 | rotate a 3-vector into screen space and clip | **ready** |
| 12 | `0x80A24AD0` | 312 | 78 | balloon rise driven by the player's height | **ready** |

**Split: 11 ready, 1 conditional.** Total 8,692 bytes of TSV extent and 2,171
reachable instructions.

Rank 1 is conditional for one bounded reason only: the slot-state dispatch at
`0x80A3A74C` reuses a **loop-carried `r0`** whose producer in the previous
iteration is overwritten by the sprite-byte packer at `0x80A3ADB0..0x80ADBC`.
Everything else in the body - the 4-player head loop, the 150-segment loop, the
constants and every callee - decodes end-to-end. See rank 1, `Ambiguity`.

## GQR3 and the qr3 pool (state at this wave)

`docs/REL_ELF_MAPPING.md` pins **`GQR3 = 0x00050005`** (unsigned 16-bit, scale
0), installed by `main.dol` at `0x8025EEEC`/`0x8025EEF0`. The RAW stream in this
REL holds no `mtspr GQRn`, so every `psq_*` in this module depends on a
`main.dol` word.

Every data `psq_l` in the twelve bodies below is the same instruction shape:

```
psq_l fD,0(rA),1,qr3        ; word = E0<D>xB000, DQ = 0, W = 1, qr = 3
```

With `GQR3 = 0x00050005` that is **one unsigned 16-bit load scaled by `2^0`**:

```
fD = (f32)*(u16 *)(rA)
```

and every occurrence in this tranche has the same layout: the halfword is
staged on the stack immediately before the load (`sth r0,10(r1)`,
`addi r3,r1,10`, `psq_l f1,0(r3),1,qr3`). A Rust port reads the `u16` field
directly and widens it; no paired-single traffic is needed. The `psq_st
f31/f30,..,qr0` prologue saves are plain FPR spills and stay as `stfd`.

## Reading conventions

* Synthetic base is `0x80A00000`. ELF file offset is
  `(address - 0x80A00000) + 0x1000`.
* `this` is the incoming `r3` unless stated otherwise; `+0xNNN` are byte
  offsets from `this`.
* `addi rX,r0,imm` is `li rX,imm`, and `addis rX,r0,imm` is `lis rX,imm`
  (`r0` is not read when `RA == 0`). `or rX,rY,rY` is `mr rX,rY`.
* `cmpi` is primary opcode **11**; `cmpli` is **10**. `rlwinm` is **21**,
  `rlwimi` is **20** and `rlwnm` is **23**. `fcmpu` is opcode 63 / XO 0;
  `fcmpo` is XO 32.
* On a `bc`/`bclr`, `bo = 12` means "branch if CR bit true" (`beq`, `bgt`,
  `blt`) and `bo = 4` means "branch if CR bit false" (`bne`, `ble`, `bge`).
  `bclr bo=20 bi=0` is `blr`; `bclr bo=12 bi=2` is `beqlr`.
* The paired-single opcodes in this build are `psq_l = 56` and `psq_st = 60`
  (`docs/REL_ELF_MAPPING.md`); the quantiser is bits 12-14, `W` is bit 15.
* TSV `size` is a bucket extent. All twelve bodies below fill their extent
  exactly (the walk from the entry reaches the last word of the extent and every
  path terminates in `blr`, `b`, `bc` or `bctr`).

## Symbolic helpers used below

| address | name / shape | role |
|---|---|---|
| `0x8015F140` | `step_f32_clamped(r3 dst: *mut f32, f1 target: f32, f2 rate: f32, f3 deadband: f32, f4 limit: f32) -> u32` | ledgered `engine::display`; f32 rate/limit/dead-band step, returns `1` on arrival |
| `0x8015F200` | `step_angle_toward(r3 value: *mut i16, r4 target: i32, r5 step: i32, r6 limit: i32, r7 limit2: i32)` | ledgered `game::task`; used for the two smoothed angle fields |
| `0x8015F470` | `step_float(f1 target: f64, f2 step: f64, r3 value: *mut f32) -> u32` | ledgered `engine::display` thunk; returns `1` on arrival |
| `0x8015F4F0` | `step_short_clamped_wrap(r3 value: *mut i16, r4 target: i32, r5 step: i32) -> u32` | ledgered `game::task`; steps a halfword and wraps |
| `0x80237BD0` | `sin_angle16(f1 angle_units: f32) -> f32` | ledgered `resource::bres`; angle reducer + `0x802F74A4` table (sine) |
| `0x80237C40` | `cos_angle16(f1 angle_units: f32) -> f32` | ledgered `resource::bres`; angle reducer + `0x802F74A0` table (cosine) |
| `0x801611C0` | `angle16_of(f1 y: f64, f2 x: f64) -> i16` | ledgered `resource::registry`; eight-octant `atan2` in 16-bit turn units |
| `0x802BE470` | `sqrtf(f1) -> f32` | ledgered `resource::archive`; square-root wrapper |
| `0x80170980` | `normalize3(r3: *mut [f32;3])` | ledgered `ui::menu`; scales by SDA2-0x5328 when above SDA2-0x532C |
| `0x801C1650` | `dist_sq(r3 a: *const [f32;3], r4 b: *const [f32;3]) -> f32` | ledgered `runtime::dvd`; squared distance of two 3-vectors |
| `0x8005F900` | `player_by_slot(r3 slot: u32) -> usize` | ledgered `game::player`; table-A resolver, returns `0` for an empty slot |
| `0x80056CF0` | `player_has(r3 player, r4 kind: u32) -> u32` | ledgered `game::player`; reads `+0x1014 + 4*(kind>>5)` |
| `0x80162D00` | `lookup_by_id(r3 id: u32) -> usize` | ledgered `resource::registry`; id&7 selects a 0x20-byte bucket |
| `0x80162590` | `current_player() -> usize` | `main.dol` player getter (same family as `0x8005F900`) |
| `0x80AFD170` | `fn_80afd170(this) -> f32` | sibling body in this module; produces a per-frame x delta |
| `0x80A3A0A0` | `fn_80a3a0a0(r3 dst: *mut [f32;3], r4 src: *const [f32;3], r5 out: *mut [f32;3])` | ledgered `game::enemies`; builds a difference vector |
| `0x80A24580` | `fn_80a24580(this)` | sibling body in this module; balloon follow-up |
| `0x80AA3F50` | `fn_80aa3f50(this, r4 out: *mut [f32;4])` | sibling body; fills a swept-box record |
| `0x80AA4020` | `fn_80aa4020(this, r4 point: *const [f32;2]) -> u32` | sibling body; point-in-box test |
| `0x80AA9350` | `fn_80aa9350(this, r4 dir: *mut [f32;3], r5 angle: u32) -> i16` | sibling body; returns an adjusted facing angle |
| `0x80077810` | `(r3 manager, r4 x: u16, r5 y: u16, r6 layer: u8) -> u32` | screen-space clip test used by the effect emitters |

---

## 1. `0x80A3A4A0` - Fire Snake 150-segment body manager

* **Address:** `0x80A3A4A0`
* **TSV size:** 2620 bytes
* **Reachable body:** 655 instructions in 63 basic blocks; 30 direct `bl`;
  **no indirect call**; one `blr` at `0x80A3AED8`. Every block terminates in a
  `b`, a `bc`, a `bl`, or the epilogue, so the extent is exact.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a3a4a0_FUN_80a3a4a0.c`
  - usable only after the GQR noise is stripped: the first ~620 lines are
    `ldexpf`/`CONCAT` restores for the `f14..f31` prologue saves. The body below
    is authoritative.
* **Proposed signature:** `pub unsafe fn fn_80a3a4a0(this: usize)`
* **Callers in this REL:** one tail branch, `0x80A3C9E8` (`b`, no `bl`), inside
  the untracked body that starts before `0x80A3C9E8`. No 32-bit word references
  the entry.
* **Constants:** block `0x80B2C2A8` (20 floats + 11 packed byte words); string
  `Wm_en_firesnk_icehit_h` at `0x80B3C2F4`, `Wm_en_firesnk_icehitsmk_h` at
  `0x80B3C30C`, model `g3d/bubble.brres` at `0x80B3C328`.
* **Verdict:** **conditional** (confidence medium) - one loop-carried register
  is unresolved; see `Ambiguity`.

### Frame

`stwu r1,-544(r1)`; `mfspr r0,lr` saved at `+0x224`; `std f31..f14` at
`+0x210` down to `+0x100` interleaved with `psq_st fN,..,qr0` (plain FPR
spills). `bl 0x802DCD40` = `_savegpr_14` (saves `r14..r31`); the epilogue uses
`0x802DCD8C` = `_restgpr_14`. Live ranges: `r14..r31`, `f14..f31`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2C2A8` | `f32` | `0.1` - dead-band for the 4-player head lerp |
| `0x80B2C2AC` | `f32` | `0.1` - second dead-band slot |
| `0x80B2C2B0` | `f32` | `80.0` |
| `0x80B2C2B4` | `f32` | `120.0` |
| `0x80B2C2B8` | `u32` | `0x00FF0000` (packed colour bytes) |
| `0x80B2C2BC` | `f32` | `4.5` |
| `0x80B2C2C0` | `f32` | `1.5` |
| `0x80B2C2C4` | `f32` | `25.0` |
| `0x80B2C2C8` | `u32` | `0x04B0FFFF` |
| `0x80B2C2CC` | `u32` | `0xFFFF0000` |
| `0x80B2C2D0` | `u32` | `0x00FFFAFA` |
| `0x80B2C2D4` | `u32` | `0xFFFF0032` |
| `0x80B2C2D8` | `u32` | `0xFFFF0500` |
| `0x80B2C2DC` | `u32` | `0x00FF9100` |
| `0x80B2C2E0` | `f32` | `0x37FFFFFF` = `3.0517576e-5` |
| `0x80B2C2E4` | `u32` | `0xFFFFE6E6` |
| `0x80B2C2E8` | `u32` | `0xFFFF0096` |
| `0x80B2C2EC` | `u32` | `0x00BE0190` |
| `0x80B2C2F0` | `f32` | `60.0` |
| `0x80B2C2F4` | `f32` | `0.025` |
| `0x80B2C2F8` | `f32` | `1.5` |
| `0x80B2C2FC` | `f32` | `2.0` |
| `0x80B2C300` | `f32` | `25.0` |
| `0x80B2C304` | `u32` | `0x0A280078` |
| `0x80B2C308` | `u32` | `0x00B4010E` |
| `0x80B2C30C` | `f32` | `60.0` |
| `0x80B2C310` | `f32` | `0.018` |
| `0x80B2C314` | `f32` | `0.0` |
| `0x80B2C318` | `f32` | `1.2` |
| `0x80B2C31C` | `f32` | `2.6` |
| `0x80B2C320` | `f32` | `4800.0` |
| `0x80B2C324` | `f32` | `0.0` |
| `0x80B2C328..0x80B2C32C` | `f64` | `2^52` bias used by the `fctiwz`/`xoris` integer-to-float idiom |
| `0x80B2C330` | `f32` | `1.0` |
| `0x80B2C334` | `f32` | `32.0` |
| `0x80B2C338` | `f32` | `0.25` |
| `0x80B2C33C` | `f32` | `1.0e-5` |
| `0x80B2C340` | `f32` | `0.6` |
| `0x80B2C344` | `f32` | `0.1` |
| `0x80B2C348` | `f32` | `1.0e-6` |
| `0x80B2C34C` | `f32` | `0.4` |
| `0x80B2C350` | `f32` | `0.5` |
| `0x80B2C354` | `f32` | `1.5` |
| `0x80B2C358` | `f32` | `0.3` |
| `0x80B2C35C` | `f32` | `8192.0` |
| `0x80B2C360` | `f32` | `26.0` |
| `0x80B2C364` | `f32` | `65536.0` |
| `0x8043A200` | `u32` **(ext, BSS)** | two-bit "enabled" gate for the `+0x7B04` counter |

### Fields touched

| offset | type | role |
|---|---|---|
| `+0x00`, `+0x04` | `f32` | shared per-actor rate/limit pair used by the head lerp |
| `+0x08`, `+0x14`, `+0x18`, `+0x1C` | `f32` | cached at block `0x80A3A6A4` into `f29`, `f21`, `f20`, `f17` |
| `+0x48`, `+0x6C`, `+0x80`, `+0x88` | `f32`/`f64` | cached into `f23`, `f24`, `f27`, `f16` |
| `+0xA4`, `+0xA8`, `+0xB0`, `+0xC4`, `+0xDC`, `+0xE0`, `+0xE4`, `+0xEC` | `f32` | cached into `f18`, `f19`, `f22`, `f26`, `f28`, `f30`, `f31`, `f0` |
| `+0xF8` | `f64` | cached into `f25` |
| `+0x7AF0 + 4*i` | `f32` | four player head-targets (one per slot) |
| `+0x7B04` | `u32` | active-segment counter, capped at `500`, gated by `> 47` |
| `+0x2CC0 + 4*i` | `u32` | segment state (0, 1, 2, 3, 10) |
| `+0x4430 + 12*i` | `[f32;3]` | segment position A (passed to `0x80A3A0A0`) |
| `+0x5BA0 + 12*i` | `[f32;3]` | segment position B |
| `+0x82DC + 12*i` | `[f32;3]` | per-segment player-hit position |
| `+0x5B0 + 8*i` | `[f32;2]` | cached two-float pair |
| `+0x1D20 + 4*i` | `u8[4]` | sprite/colour bytes A |
| `+0x24F0 + 4*i` | `u8[4]` | sprite/colour bytes B |
| `+0x7310 + 2*i`, `+0x7EF2 + 2*i` | `i16` | per-segment angle deltas |
| `+0x1550` | `u32` | state-2/3 dwell timer (decremented, retires at 0) |
| `+0x1524` | `u32` | "spawn armed" word tested by state 0 |
| `+0x9A4C` | `u32` | id passed to `0x80162D00` to resolve the owner actor |
| `+0x3490 + 4*i` (via `r26`) | `f32` | per-segment value written from `f18+f1` |
| `+0x3C60 + 4*i` (via `r26`) | `f32` | per-segment speed scalar |

### Behaviour

```c
// ---- entry -----------------------------------------------------------
let owner = fn_80162d00(read_u32(this + 0x9A4C));      // may be 0
// four player slots: lerp this+0x7AF0+4*i toward a per-kind target
for i in 0..4 {
    let p = fn_8005f900(i);
    if p != 0 { p_slot = p; }
    let dst = this + 0x7AF0 + 4*i;
    if has(0x53) || has(0x2B) || has(0x0C) {
        step_f32(dst, 1.0, 0.25, 0.1, 1.0e-5);
    } else if has(0x2E) || has(0x29) {
        step_f32(dst, 0.6, 0.25, 0.1, 1.0e-5);
    } else {
        step_f32(dst, 0.0, 0.1, 0.1, 1.0e-6);
    }
}
// active-segment counter
let n = read_u32(this + 0x7B04);
if n > 47 && (read_u32(0x8043A200) & 3) == 0 && owner != 0 && n < 500 {
    write_u32(this + 0x7B04, n + 1);
}
// ---- 150-segment loop ------------------------------------------------
for i in 0..150 {
    let st = read_u32(this + 0x2CC0 + 4*i);
    match st {
        0 => { /* arm the next segment when the counter reaches this slot */ }
        1 => { /* build the segment: sin/cos placement, +0x3C60 speed */ }
        10 => { /* render path: sample the sprite byte, distance to players */ }
        _ => { /* 2, 3 and everything else fall to the common tail */ }
    }
}
// ---- common tail (0x80A3AD0C) ---------------------------------------
// sprite byte patch: b = *(u8*)(this+0x7B09); b2 = (b - 2) & 0xFF;
// if b2 <= 1 { write 0xFA,0xFA,0xFF into the +0x1D20 bytes }
// then pack 4 bytes from (0x1D20) and (0x24F0), advance the per-slot
// cursors by 4/12/2/8/12 and loop.
// ---- epilogue -------------------------------------------------------
fn_801c0bd0(this + 0x7C, *(f32*)(this+0x6C), *(f32*)(this+0x6C), 0.0);
fn_8016a170(this + 0x544, this + 0x7C, *(f32*)(this+0x19A54));
step_f32(this + 0x7B00, *(f32*)(this+0x88), *(f32*)(this+0x90),
         *(f32*)(this+0x9C), *(f32*)(this+0xF0));
return 1;
```

Per-state notes that are directly evidenced:

* **State 1** (`0x80A3A7A0..0x80A3A92C`): `f1 = fn_800b2f10(*(f32*)(this+0x98))`;
  `f16 = f18 + f1`; stores `f19*f16` at `+0x3490` (via `r26`) and
  `f21 + f16*f20` at `+0x3C60` (via `r26`); copies `this+0xAC..+0xB4` to
  `+0x4430+12i`; calls `fn_800b2ee0`. Then, for `owner != 0`, it samples
  `+0x102`/`+0x100`/`+0x102` as a `f64` and writes a small `f32` at
  `+0x82DC+12i`, adds `+0x100`/`+0x102` into the two `i16` accumulators, and
  increments the state to **2**.
* **State 2** (`0x80A3AA68`): decrements `+0x1550`; when it hits zero the
  state becomes **3**; then lerps `+0x5B0+8i` toward `this+0x1456` with
  `(f1, f2, f3, f4) = (this+0x6C, this+0xD4, this+0xA8, this+0xD8)`.
* **State 3** (`0x80A3AAC4`): reads the 4-byte group at `+0x24F0+4i`; byte 3
  is clamped (`c < 8 ? 0 : c - 8`), the group is written back, and when the
  clamped byte is zero and the slot index equals `+0x7B04` it calls
  `fn_80064d10(this, 1)` and decrements `+0x1550`.
* **State 10** (`0x80A3A930..0x80A3AA68`): the heavy update. It builds a
  3-float record from `+0x4430+12i`, samples `asin`-style helpers
  (`0x80237BD0`/`0x80237C40`) at `+0x7310+2i`, `+0x7EF2+2i` and
  `+0x7310+2i ± 1200`, then calls `fn_8015f140` on the segment pair. The
  `+0x3C60` scalar is multiplied by `1/256`-style angle constants and stored
  back. After that, for each of four players whose kind is `0x53`, it computes
  `fn_801c1650` against `+0x82DC+12i`, takes `sqrtf`, and when the distance is
  below `f14 = *(f32*)(this+0x7B00) * f29` it copies the player position into
  `+0x82DC+12i` and sets the segment state to **10**.
* **State 0** (`0x80A3A770`): requires `owner != 0`, `*(u32*)(owner+0x5F4) != 0`,
  `*(u32*)(this+0x1524) == 0` and `i <= *(u32*)(this+0x7B04)`; then writes the
  next state (`0x80A3A798`). The block also sets `f15 = 0.0` on entry.
* **`owner != 0 && *(u32*)(owner+0x5F4) != 0`** forces the state to **3** at
  `0x80A3AB8C` after the state-2/3 blocks.

### Call table (direct)

| site | callee | arguments |
|---|---|---|
| `0x80A3A540` | `0x802DCD40` | `_savegpr_14` (`r14..r31`) |
| `0x80A3A560`, `0x80A3A59C`, `0x80A3ABAC` | `0x8005F900` | slot index `0` then `i` |
| `0x80A3A570` | `0x80162D00` | `*(u32*)(this+0x9A4C)` |
| `0x80A3A5B4/5C8/5DC/60C/620` | `0x80056CF0` | `(player, 0x53/0x2B/0x0C/0x2E/0x29)` |
| `0x80A3A5FC/640/65C` | `0x8015F140` | `this+0x7AF0+4i`, `(1.0|0.6|0.0)`, `0.25|0.1`, `0.1`, `1e-5|1e-6` |
| `0x80A3A7A4` | `0x800B2F10` | `*(f32*)(this+0x98)` |
| `0x80A3A7DC` | `0x800B2EE0` | no args (duplicate of `0x800B2ED0`) |
| `0x80A3A95C/9D4/9F4` | `0x80237BD0` | angle unit staged on the stack |
| `0x80A3A9B8` | `0x80237C40` | angle unit staged on the stack |
| `0x80A3AA44/AAB0` | `0x8015F140` | `+0x5B0+8i` and `+0x1456` pair |
| `0x80A3AB5C` | `0x80064D10` | `(this, 1)` - retires the actor |
| `0x80A3ABC0` | `0x80056CF0` | `(player, 0x53)` |
| `0x80A3ABF0` | `0x801C1650` | player point vs `+0x82DC+12i` |
| `0x80A3ABF4` | `0x802BE470` | squared distance -> sqrt |
| `0x80A3AC48` | `0x80A3A0A0` | `(r3 = &stack[88], r4 = this+0x82DC+12i, r5 = &stack[76])` |
| `0x80A3AE08` | `0x801C0BD0` | `(this+0x7C, f1, f2, f3)` |
| `0x80A3AE14` | `0x8016A170` | `(this+0x544, this+0x7C)` |
| `0x80A3AE2C` | `0x8015F140` | `this+0x7B00`, 4 floats |
| `0x80A3AEC8` | `0x802DCD8C` | `_restgpr_14` |

### Ambiguity

One, and it is bounded:

* `0x80A3A748` executes `addi r0,r0,-2` *before* reloading the state, so the
  `cmpli cr0,0,r0,0x1` / `bgt 0x80A3A930` at `0x80A3A74C` tests a value left in
  `r0` by the **previous** iteration's tail. The tail
  (`0x80A3ADB0..0x80ADBC`) overwrites `r0` twice (`+199`, `+207`) after packing
  the sprite bytes, so the literal reading is "previous packed byte + 406",
  which makes the `(x-2) <= 1` test unreachable. The two coherent readings are
  (a) the test is a dead artefact of the byte packer and only `state == 10`
  reaches `0x80A3A930`, or (b) the intended value is the previous segment's
  sprite byte and the test selects "carry a sprite of type 2/3 into this slot".
  Both readings produce the same code for states 0, 1, 2, 3 and 10; only the
  entry into the render block differs for the first two slots of a frame.
  Confirm by tracing one live frame before porting; do **not** invent a state
  value.

Everything else is exact: there is no `bctr`, no jump table, no `extraout_`
register, and every `psq` is either an `f14..f31` spill (`qr0`) or the
`psq_l fD,0(rA),1,qr3` u16 load.

---

## 2. `0x80AEDD40` - 16-byte waypoint chase

* **Address:** `0x80AEDD40`
* **TSV size:** 1208 bytes
* **Reachable body:** 301 instructions; 15 direct `bl`; 3 indirect vtable calls
  (`bcctr`, see the call table); ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aedd40_FUN_80aedd40.c`
  - usable. The first ~80 lines are the `f31` GQR0 spill restore; the body from
    `iVar10 = func_0x802dcd6c()` onward is faithful.
* **Proposed signature:** `pub unsafe fn fn_80aedd40(this: usize)`
* **Callers in this REL:** none. The body is reached only through the actor's
  vtable, so the entry has no `bl` and no 32-bit word reference.
* **Constants:** block `0x80B31208`; message payloads in `0x80B77640`
  **(ext, BSS)**.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-144(r1)`; LR at `+0x94`; `stfd f31,+0x80` + `psq_st f31,+0x88,0,qr0`
(spill); `bl 0x802DCD6C` = `_savegpr_25` (`r25..r31`); epilogue `0x802DCDB8` =
`_restgpr_25`. Only `f31` is live.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B31208` | `f32` | `-0.15` (unused by the reachable body) |
| `0x80B31210` | `f32` | `1.0` |
| `0x80B31214` | `f32` | `0.0` |
| `0x80B31218` | `f32` | `4096.0` |
| `0x80B3121C` | `f32` | `0.01` - cache dead-band |
| `0x80B31220` | `f32` | `-0.01` - cache dead-band |
| `0x80B31224` | `f32` | `0.00390625` = `1/256` - angle-unit scale |
| `0x80B31228` | `f32` | `1.0e-4` - `0x8015F140` limit |
| `0x80B3122C` | `f32` | `0.1` - position clamp margin |
| `0x80B31230..0x80B31234` | `f64` | `2^52 + 2^31` bias for the signed `s16`->float idiom |
| `0x80B77640 + 0x10` | record **(ext)** | message payload passed to vtable `+0x18` |
| `0x80B77640 + 0x90` | record **(ext)** | payload for `(flags >> 16) & 3 == 0` |
| `0x80B77640 + 0xD0` | record **(ext)** | payload for `(flags >> 16) & 3 == 2` |
| `0x8043A1C8` | `u8*` **(ext)** | player table base; `+0x120E` is the active player index |
| `0x80439E98` | `u8*` **(ext)** | 944-byte per-player record array base |

### Fields touched

| offset | type | role |
|---|---|---|
| `+0x04` | `u32` | bitfield: `>>20 & 0xF` actor index, `>>19 & 1` instant-angle mode, `>>16 & 3` arrival variant, `>>8 & 0xF` variant word |
| `+0xAC`, `+0xB0`, `+0xB4` | `f32` | x/y/z position |
| `+0xE8` | `f32` | speed scalar for the position step |
| `+0x394` | vtable | `*(vt+0x18)(this+0x394, payload)` |
| `+0x3D0` | `u32` | per-frame advance added to `+0x3D4` |
| `+0x3D4` | `u32` | waypoint index (16-byte records) |
| `+0x3D8` | `i16` | last waypoint word (arrival marker) |
| `+0x3E0` | `u32` | "arrived once" latch |
| `+0x3E4`, `+0x3E6` | `i16` | two smoothed angle fields |
| `+0x3E8` | `u16` | zeroed on the instant-angle path |
| `+0x3F8`, `+0x3FC`, `+0x400` | `f32` | cached last offset vector |

### Behaviour

```c
let flags = read_u32(this + 4);
let obj   = fn_800d90c0((flags >> 20) & 0xF);
let rec_tbl = 0x80439E98 + 944 * *(u8*)(0x8043A1C8 + 0x120E);
let rec_base = if read_u32(rec_tbl + 0x2C) == 0 { 0 } else { read_u32(rec_tbl + 0x3C) };
let rec = rec_base + 16 * (read_u16(obj + 2) as u32 + read_u32(this + 0x3D4));

// waypoint delta; the record's second halfword is negated
let dx = (read_s16(rec + 0) as f32) - read_f32(this + 0xAC);
let dy = (-read_s16(rec + 2) as f32) - read_f32(this + 0xB0);
let dz = 0.0f32;

// |dx|,|dy| < 0.01 -> keep the cached offset, else cache the new one
if !(dx > -0.01 && dx < 0.01 && dy > -0.01 && dy < 0.01) {
    write_f32(this + 0x3F8, dx);
    write_f32(this + 0x3FC, dy);
    write_f32(this + 0x400, dz);
} else {
    dx = read_f32(this + 0x3F8);
    dy = read_f32(this + 0x3FC);
    dz = read_f32(this + 0x400);
}
let v = [dx, dy, dz];
fn_80170980(&v);                       // normalise in place

let pitch = fn_801611c0(v[1] as f64, sqrtf(v[0]*v[0] + v[2]*v[2]) as f64);
let yaw   = fn_801611c0(v[0] as f64, v[2] as f64);

if (flags >> 19) & 1 == 1 {
    write_s16(this + 0x3E4, pitch);
    write_s16(this + 0x3E6, yaw);
    write_u16(this + 0x3E8, 0);
} else {
    fn_8015f200(this + 0x3E4, pitch, 1, 100, 100);
    fn_8015f200(this + 0x3E6, yaw,   1, 100, 100);
}

// four sin/cos products from the two smoothed angles
let s1 = sin16((read_s16(this+0x3E4) as u16) as f32 * (1.0/256.0));
let c2 = cos16((read_s16(this+0x3E6) as u16) as f32 * (1.0/256.0));
let step_x = s1 * c2;
let s3 = sin16((read_s16(this+0x3E4) as u16) as f32 * (1.0/256.0));
let c4 = cos16((read_s16(this+0x3E6) as u16) as f32 * (1.0/256.0));
let step_z = s3 * c4;

fn_8015f140(this + 0xE8, read_f32(rec + 4), read_f32(rec + 8), 1.0e-4);
let px = read_f32(this + 0xAC) + step_x * read_f32(this + 0xE8);
let py = read_f32(this + 0xB0) + step_z * read_f32(this + 0xE8);
let pz = 0.0;
write_f32(this + 0xAC, px);
write_f32(this + 0xB0, py);
write_f32(this + 0xB4, 0.0);

// clamp the step to +/-0.1 of the raw waypoint coordinate
if px < read_f32(rec + 0) + 0.1 && read_f32(rec + 0) - 0.1 < px {
    write_f32(this + 0xAC, read_f32(rec + 0));
}
if py < read_f32(rec + 2) + 0.1 && read_f32(rec + 2) - 0.1 < py {
    write_f32(this + 0xB0, read_f32(rec + 2));
}

// arrival test: current position bracketed by the cached offset
if (brackets(dx, px) && brackets(dy, py)) {
    if read_u32(this + 0x3E0) != 0 {
        let w = read_s16(rec + 0xC);
        write_s16(this + 0x3D8, w);
        if w != 0 { vcall(this + 0x394, 0x80B77650); }
    }
    write_u32(this + 0x3E0, 1);
    let next = read_u32(this + 0x3D4) + read_u32(this + 0x3D0);
    if next >= read_u16(obj + 4) as u32 {
        match (flags >> 16) & 3 {
            0 => vcall(this + 0x394, 0x80B776D0),
            1 => next = (flags >> 8) & 0xF,
            2 => vcall(this + 0x394, 0x80B77710),
            _ => {}
        }
    }
    write_u32(this + 0x3D4, next);
}
```

Notes that are exact in the stream:

* `0x80AEDD40`'s first *dead* read is `lfs f0,76(r1)`: the compiler kept a
  stack slot for the third component that is overwritten (`0.0` at
  `0x80AEDE58`) and never used. A port should use `dz = 0.0` and ignore
  `0x80AEDE18`.
* The `0x80AEE0AC..0x80AEE120` block is a three-way OR of
  "`dx` is bracketed" and "`dy` is bracketed" built with `fcmpu` + `cror`;
  the two `cror` forms are `cror 2,1,2` and `cror 2,0,2` (i.e. `||`).
* `read_u16(obj + 4)` is the waypoint count. `+0x3D0` is usually `1`.
* The vtable dispatch is `r12 = *(u32*)(this + 0x394); r12 = *(u32*)(r12 + 0x18);
  r4 = &payload; r3 = this + 0x394; bcctr r12`.

### Ambiguity

None for the port. The export's `in_f31`/`unaff_GQR*` names are the spill
artefacts and the pinned `GQR3`; the five `psq_l f1,0(rA),1,qr3` sites all read
a `u16` staged two instructions earlier.

---

## 3. `0x80AEF130` - sibling waypoint chase (`+0x670` state block)

* **Address:** `0x80AEF130`
* **TSV size:** 1180 bytes
* **Reachable body:** 294 instructions; 15 direct `bl`; 2 vtable `bcctr`; ends
  in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aef130_FUN_80aef130.c`
  - usable after the same GQR strip as rank 2.
* **Proposed signature:** `pub unsafe fn fn_80aef130(this: usize)`
* **Callers in this REL:** none (vtable-only entry).
* **Constants:** block `0x80B31240`; payloads `0x80B77790` and `0x80B777D0`
  **(ext, BSS)**.
* **Verdict:** **ready** (confidence high)

### Frame

Identical to rank 2: 144-byte frame, `f31` spill, `_savegpr_26`
(`0x802DCD70`), `_restgpr_26` (`0x802DCDBC`).

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B31240` | `f32` | `-0.15` (unused by the reachable body) |
| `0x80B31244` | `f32` | `0.5` |
| `0x80B31248` | `f32` | `1.0` |
| `0x80B3124C` | `f32` | `0.0` |
| `0x80B3125C` | `f32` | `-16.0` |
| `0x80B31260` | `f32` | `16.0` |
| `0x80B31264` | `f32` | `32.0` |
| `0x80B31268` | `f32` | `-32.0` |
| `0x80B3126C` | `f32` | `4096.0` |
| `0x80B31270` | `f32` | `0.01` |
| `0x80B31274` | `f32` | `-0.01` |
| `0x80B31278` | `f32` | `0.00390625` = `1/256` |
| `0x80B3127C` | `f32` | `1.0e-4` |
| `0x80B31280` | `f32` | `0.1` - position clamp margin |
| `0x80B31288..0x80B3128C` | `f64` | `2^52 + 2^31` bias |
| `0x80B77790` | record **(ext)** | payload for `(flags >> 16) & 3 == 1` |
| `0x80B777D0` | record **(ext)** | payload for `(flags >> 16) & 3 == 2` |

### Fields touched

| offset | type | role |
|---|---|---|
| `+0x04` | `u32` | same bitfield layout as rank 2 |
| `+0x38F` | `u8` | "keep y" gate: when zero, `+0xB4` is forced to `0.0` |
| `+0x60` | vtable | `*(vt+0xD4)(this, payload)` |
| `+0xAC`, `+0xB0`, `+0xB4` | `f32` | position |
| `+0xE8` | `f32` | speed |
| `+0x66C` | `u32` | per-frame advance |
| `+0x670` | `u32` | waypoint index |
| `+0x674`, `+0x676` | `i16` | smoothed angles |
| `+0x678` | `u16` | zeroed on the instant-angle path |
| `+0x688`, `+0x68C`, `+0x690` | `f32` | cached offset vector |

### Behaviour

Same algorithm as rank 2 with these substitutions and one extra gate:

```c
let flags = read_u32(this + 4);
let obj   = fn_800d90c0((flags >> 20) & 0xF);
// ... identical 944-byte record lookup ...
let rec = rec_base + 16 * (read_u16(obj + 2) as u32 + read_u32(this + 0x670));
let dx = (read_s16(rec + 0) as f32) - read_f32(this + 0xAC);
let dy = (-read_s16(rec + 2) as f32) - read_f32(this + 0xB0);
// cache branch identical, but compare with fcmpo (not fcmpu)
// fn_80170980, sqrtf, two fn_801611c0 calls identical
if (flags >> 19) & 1 == 1 {
    write_s16(this + 0x674, pitch);
    write_s16(this + 0x676, yaw);
    write_u16(this + 0x678, 0);
} else {
    fn_8015f200(this + 0x674, pitch, 1, 100, 100);
    fn_8015f200(this + 0x676, yaw,   1, 100, 100);
}
// sin/cos step identical
let px = read_f32(this + 0xAC) + step_x * read_f32(this + 0xE8);
let py = read_f32(this + 0xB0) + step_z * read_f32(this + 0xE8);
write_f32(this + 0xAC, px);
write_f32(this + 0xB0, py);
if read_u8(this + 0x38F) == 0 { write_f32(this + 0xB4, 0.0); }
// clamp using +/-0.1 (0x80B31280)
// arrival test identical; on success:
//   read_s16(rec+0xC) != 0 -> vcall(this+0x60, 0x80B77790)
//   next >= read_u16(obj+4):
//     (flags>>16)&3 == 0 -> advance only
//     (flags>>16)&3 == 1 -> next = (flags >> 8) & 0xF
//     (flags>>16)&3 == 2 -> vcall(this+0x60, 0x80B777D0)
//   write_u32(this + 0x670, next)
```

Exact deltas from rank 2, all confirmed in the words:

* The vtable is `this + 0x60` and the entry point is `+0xD4` (rank 2 uses
  `this + 0x394` and `+0x18`), and `r3` is `this` (not `this + 0x394`).
* The arrival branch uses `fcmpo` throughout, so an unordered compare behaves
  differently from rank 2's `fcmpu`; keep `fcmpo`.
* `+0x38F` is read **before** the position stores; the `+0xB4 = 0` write is
  unconditional when the byte is zero (no orientation branch).
* The two other constant blocks at `0x80B3125C..0x80B31268` (`±16`, `±32`) are
  not consumed by the reachable stream; they belong to the shared actor family.

### Ambiguity

None for the port.

---

## 4. `0x80AA93F0` - side-flip steering with a `+0x348` facing latch

* **Address:** `0x80AA93F0`
* **TSV size:** 496 bytes
* **Reachable body:** 124 instructions; 7 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa93f0_FUN_80aa93f0.c`
  - usable; the `f30`/`f31` prologue is a plain spill pair.
* **Proposed signature:** `pub unsafe fn fn_80aa93f0(this: usize) -> u32`
* **Callers in this REL:** one `bl`, `0x80AA9B58`. No 32-bit word reference.
* **Constants:** block `0x80B2F590 + 0x2B8` (`0x80B2F848` = `1/256`),
  `0x80B2F6A0` = `0.0`, `0x80B2F6A4` = `1.0`.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-96(r1)`; LR at `+0x64`; `f31` at `+0x50`/`+0x58`, `f30` at
`+0x40`/`+0x48`; `r28..r31` at `+0x30..+0x3C`. `r30 = 0x80B2F590` (constant
base), `r29 = this`.

### Behaviour

```c
let hit = fn_80064740(this, &out8);            // ground/actor probe
if hit == 0 {
    fn_8006cd40(this);
    return 0;
}
let a = fn_8006ced0(&target, hit);             // copied 3-vector, hit side
let b = fn_8006ced0(&self,  this);             // copied 3-vector, self side
let angle = fn_801611c0(target.z - self.z, target.x - self.x); // atan2(dy, dx)
write_s16(this + 0xA4E, angle);

let s = sin16(angle as u16 as f32 * (1.0/256.0));
let c = cos16(angle as u16 as f32 * (1.0/256.0));
if read_u8(this + 0x348) != 0 { let sc = -c; } else { let sc = c; }

// state gate: countdown then facing comparison
if read_u32(this + 0x9C0) != 0 { write_u32(this + 0x9C0, read_u32(this + 0x9C0) - 1); }
if read_u32(this + 0x9C0) == 0 {
    let facing = fn_80096240(this, this + 0xAC);
    if read_u8(this + 0x348) == facing {
        let a2 = fn_80aa9350(this, &dir, 0xE0);
        let s2 = sin16(a2 as u16 as f32 * (1.0/256.0));
        let c2 = cos16(a2 as u16 as f32 * (1.0/256.0));
        // note: the 0x80B2F848 scale is 1/256, applied before both calls
        if read_u8(this + 0x348) != 0 { write_f32(this+0xE8, -c2); }
        else                           { write_f32(this+0xE8,  c2); }
        write_f32(this + 0xEC, -s2);
        write_f32(this + 0xF0, 0.0);
        write_f32(this + 0x9A0, read_f32(this+0xE8));
        write_f32(this + 0x9A4, read_f32(this+0xEC));
        write_f32(this + 0x9A8, 0.0);
    } else {
        write_u8(this + 0x348, facing);
        write_u32(this + 0x9BC, 1);
        return 1;
    }
} else {
    fn_80aa9350(this, this + 0x9A0, 0xE0);
}
fn_8006cd40(this);
if distance < 1.0 && read_u32(this + 0x9C0) == 0 {
    write_u32(this + 0x9C0, 0x28);            // 40-frame cooldown
}
return 0;
```

Exact points:

* `0x801C14C0(&dir)` returns the squared length; the compare is
  `dVar10 < 1.0` (`0x80B2F6A4`), i.e. the retarget cooldown is re-armed inside a
  unit-radius neighbourhood.
* The `+0x348` facing byte is compared against the result of `0x80096240`,
  which returns an octant; equality means "already facing the target".
* `+0x9C0` is a plain frame countdown (`0x28` = 40).
* `+0x9BC = 1` is the "facing changed this frame" marker.

### Ambiguity

None. The two `psq_l f1,0(rA),1,qr3` sites read the `u16` angle staged by the
preceding `sth`.

---

## 5. `0x80AFDAC0` - vertical seek on `+0x608`, vtable event on arrival

* **Address:** `0x80AFDAC0`
* **TSV size:** 480 bytes
* **Reachable body:** 120 instructions; 6 direct `bl`; 1 vtable `bcctr`; ends
  in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80afdac0_FUN_80afdac0.c`
  - usable.
* **Proposed signature:** `pub unsafe fn fn_80afdac0(this: usize)`
* **Callers in this REL:** none (vtable-only entry).
* **Constants:** block `0x80B31750`; message payload `0x80B78048` **(ext, BSS)**;
  `0x80429D5C` **(ext)** manager.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-80(r1)`; LR at `+0x54`; `stfd f31,+0x40` + `psq_st f31,+0x48,0,qr0`;
`r28..r31` at `+0x30..+0x3C`. `r29 = this`, `r30 = 0x80B31750`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B317B8..0x80B317BC` | `f64` | `2^52 + 2^31` bias |
| `0x80B317C8` | `f32` | `0.00390625` = `1/256` |
| `0x80B317CC` | `f32` | `8.0` - base y offset |
| `0x80B317F0` | `f32` | `16.0` - per-unit y step |
| `0x80B317F4` | `f32` | `1536.0` |
| `0x80B317F8` | `f32` | `0.5` - `0x8015F470` step |
| `0x80B78048` | record **(ext)** | arrival payload |
| `0x80429D5C` | `u32` **(ext)** | effect-manager pointer for `0x80198290` |

### Behaviour

```c
let player = fn_80162590();
let arrived = fn_8015f470(
    read_f32(player + 0xAC) as f64,             // target x
    0.5,                                        // step
    this + 0xAC);                               // value
fn_8006cde0(this);
fn_800955f0(this);

// the seek target is (player.y + 8) + 16 * (f32)(s32)*(this+0x608)
let off = (read_u32(this + 0x608) as i32) as f32;
let y_target = 8.0 + read_f32(player + 0xB0) + 16.0 * off;

if read_f32(this + 0xB0) < y_target {
    // still below the target: bank the model by +/-0xE00 per frame
    if (read_u32(this + 0x608) & 1) == 0 {
        write_s16(this + 0x5CA, read_s16(this + 0x5CA) - 0xE00);
    } else {
        write_s16(this + 0x5CA, read_s16(this + 0x5CA) + 0xE00);
    }
    let s = sin16((read_s16(this + 0x5CA) as u16) as f32 * (1.0/256.0));
    // horizontal sway stored as a signed halfword
    let px = (read_s16(this + 0x5CC) as f32) + 3072.0 * s;
    write_s16(this + 0x102, px as i16);
} else {
    // reached the player: latch and fire the arrival event once
    if read_u32(this + 0x5D8) == 0 && read_u32(this + 0x5D4) >= 0 {
        fn_8006a4b0(&pos, this + 0xAC);
        fn_80198290(read_u32(0x80429D5C), 0x1B5, &pos,
                    read_s32(this + 0x5D4) as i32, 0);
    }
    write_f32(this + 0xB0, y_target);
    write_u32(this + 0x5D8, 1);
}
if arrived != 0 && read_u32(this + 0x5D8) != 0 {
    vcall(this + 0x60, 0x80B78048);             // *(vt+0xD4)
}
```

Exact points:

* `fn_8015F470` is `step_float_clamped(f1 target, f2 step, r3 *mut f32) -> u32`;
  it returns `1` on arrival. The x position is stepped toward the player's x by
  `0.5` per frame, independent of the y seek.
* The y branch is a **one-sided** test: while `this+0xB0 < y_target` the actor
  banks and sways; as soon as `this+0xB0 >= y_target` it snaps `+0xB0` to the
  target and latches `+0x5D8 = 1`.
* `0x80B317F4 = 1536.0` is not consumed by this body (it belongs to rank 9).
* The event manager call is `0x80198290(manager, 0x1B5, &pos, (i32)this+0x5D4, 0)`.

### Ambiguity

None. The single `psq_l f1,0(rA),1,qr3` reads the `u16` bank angle staged one
instruction earlier.

---

## 6. `0x80A429C0` - orbit/offset follow through a `+0xAE4` owner

* **Address:** `0x80A429C0`
* **TSV size:** 476 bytes
* **Reachable body:** 119 instructions; 4 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a429c0_FUN_80a429c0.c`
  - usable.
* **Proposed signature:** `pub unsafe fn fn_80a429c0(this: usize)`
* **Callers in this REL:** two `bl`, `0x80A424EC` and `0x80A425A8`.
* **Constants:** `0x80B2C6E0` = `0.00390625` (`1/256`).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-96(r1)`; LR at `+0x64`; `f31` spill at `+0x50`/`+0x58`; `r28..r31`
at `+0x40..+0x4C`. `r28 = this`.

### Fields touched

| offset | type | role |
|---|---|---|
| `+0xAE4` | `u32` | id passed to `0x80162D00` to resolve the owner |
| `+0xAE0` | `u8` | mode: `1` = attached, anything else = orbiting |
| `+0xADE` | `i16` | saved owner angle (attached mode) |
| `+0xAF0`, `+0xAF2` | `i16` | angle offsets written into `+0x104` |
| `+0xAE8` | `f32` | radius |
| `+0xACC`, `+0xAD0` | `f32` | ellipse radii for the orbit mode |
| `+0x104` | `i16` | model angle |
| `+0xAC`, `+0xB0`, `+0xB4` | `f32` | position |
| `+0xD0`, `+0xD4`, `+0xD8` | `f32` | per-frame position bias |

### Behaviour

```c
let owner = fn_80162d00(read_u32(this + 0xAE4));
let ox = read_f32(owner + 0xAC);
let oy = read_f32(owner + 0xB0);
let oa = read_s16(owner + 0x104);

if read_u8(this + 0xAE0) == 1 {
    write_s16(this + 0xADE, oa);
    let a = oa + read_s16(this + 0xAF2);
    write_s16(this + 0x104, read_s16(this + 0xAF0) + oa);
    let c = cos16(a as u16 as f32 * (1.0/256.0));
    let s = sin16(a as u16 as f32 * (1.0/256.0));
    let r = read_f32(this + 0xAE8);
    write_f32(this + 0xAC, ox + r * c - read_f32(this + 0xD0));
    write_f32(this + 0xB0, oy + r * s - read_f32(this + 0xD4));
    write_f32(this + 0xB4, read_f32(this + 0xB4) - read_f32(this + 0xD8));
    fn_80707400(owner);                         // owner->something() callback
} else {
    let a2 = oa + 0x4000;                       // stored; the 0x80A42A6C
    write_s16(this + 0x104, a2);                // addi is this same value
    let c = cos16(a2 as u16 as f32 * (1.0/256.0));
    let s = sin16(a2 as u16 as f32 * (1.0/256.0));
    write_f32(this + 0xB4, read_f32(this + 0xB4) - read_f32(this + 0xD8));
    write_f32(this + 0xAC,
              ox + (read_f32(this + 0xACC) * c - read_f32(this + 0xAD0) * s)
                 - read_f32(this + 0xD0));
    write_f32(this + 0xB0,
              oy + (read_f32(this + 0xACC) * s + read_f32(this + 0xAD0) * c)
                 - read_f32(this + 0xD4));
}
```

Exact points:

* The `else` branch computes `sVar10 = sVar10 + 0x4000` at `0x80A42A70`; the
  `0x80A42A6C` `addi` in the listing is the same statement and there is **no**
  `-0x4000`; a port should write `angle = owner_angle + 0x4000` and store it.
* The attached branch writes both `+0xADE` (the owner angle) and
  `+0x104 = +0xAF0 + owner_angle`, while the *cos/sin* pair uses
  `owner_angle + 0xAF2`. The three offsets are independent.
* `fn_80707400(owner)` is a `main.dol` callback on the owner; it has no export
  in this module, and the port keeps it as a call.
* The two `psq_l` sites are the pinned `u16` angle loads.

### Ambiguity

None for the loop semantics. `0x80707400` is outside this module (marked as
call-through), but it takes no visible argument other than the owner in `r3`.

---

## 7. `0x80AA4070` - four-corner contact probe around a swept box

* **Address:** `0x80AA4070`
* **TSV size:** 444 bytes
* **Reachable body:** 111 instructions; 6 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa4070_FUN_80aa4070.c`
  - usable.
* **Proposed signature:** `pub unsafe fn fn_80aa4070(this: usize) -> u32`
* **Callers in this REL:** one `bl`, `0x80AA4FEC`.
* **Constants:** `0x80B2F530` = `1.0` (margin), `0x80B2F534` = `1/256`,
  `0x80B2F55C` = `12.0` (probe offset).
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-96(r1)`; LR at `+0x64`; `r29..r31` at `+0x54..+0x5C`. `r29 = this`,
`r30 = 0x80B2F4A8`.

### Behaviour

```c
fn_80aa3f50(this, &box);                  // fills [f32;4] at stack+56
let angle = read_s16(this + 0xA4E);
let c = cos16(angle as u16 as f32 * (1.0/256.0));
let s = sin16(angle as u16 as f32 * (1.0/256.0));

// sweep the box outward by 12.0 in the facing direction
let a = [box[0] + 12.0 * c, box[1] + 12.0 * s];
let b = [box[0], box[1]];

// four corners of the axis-aligned box expanded by 1.0
let minx = min(a.x, b.x) - 1.0, maxx = max(a.x, b.x) + 1.0;
let miny = min(a.y, b.y) - 1.0, maxy = max(a.y, b.y) + 1.0;

if fn_80aa4020(this, &[minx, miny]) != 0 { return 1; }
if fn_80aa4020(this, &[maxx, maxy]) != 0 { return 1; }
if fn_80aa4020(this, &[maxx, miny]) != 0 { return 1; }
let hit = fn_80aa4020(this, &[minx, maxy]);
return (-hit | hit) >> 31;                // 1 when hit != 0
```

Exact points:

* `0x80AA4020` is the point-in-box test used four times (min/min, max/max,
  max/min, min/max in stream order). The short-circuit order does not change
  the result because all four are OR-ed.
* The final boolean is the `neg`/`or`/`rlwinm 1,31,31` idiom, so the function
  returns `0` only when all four probes miss.
* The box is expanded by `1.0` on all four sides (the `0x80B2F530` constant),
  and the sweep is `12.0 * (cos, sin)`.
* `+0xA4E` is the facing angle written by rank 4 (`fn_80aa93f0`).

### Ambiguity

None. Both `psq_l f1,0(rA),1,qr3` sites read the `u16` staged by the preceding
`sth`.

---

## 8. `0x80AA35E0` - angle to target plus velocity clamp, Q12 facing

* **Address:** `0x80AA35E0`
* **TSV size:** 408 bytes
* **Reachable body:** 102 instructions; 6 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80aa35e0_FUN_80aa35e0.c`
  - the GQR3 lines are noise; the `(f32)CONCAT22` arms are the real values.
* **Proposed signature:** `pub unsafe fn fn_80aa35e0(this: usize)`
* **Callers in this REL:** two `bl`, `0x80AA4F54` and `0x80AA5550`.
* **Constants:** `0x80B2F500` = `0.03125`, `0x80B2F534` = `1/256`,
  `0x80B2F538` = `1.25`, `0x80B2F53C` = `-1.25`, `0x80B2F540` = `2^52` bias.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-112(r1)`; LR at `+0x74`; `f31` at `+0x60`/`+0x68`; `r29..r31` at
`+0x54..+0x5C`. `r29 = this`, `r30 = 0x80B2F4A8`, `r31` reused.

### Behaviour

```c
let target = fn_80064740(this, &out8);
if target != 0 {
    fn_8006ced0(&a, target);              // target position
    fn_8006ced0(&b, this);                // self position
    let angle = fn_801611c0(a.z - b.z, a.x - b.x);
    write_s16(this + 0xA4E, angle);

    let s = sin16(angle as u16 as f32 * (1.0/256.0));
    let c = cos16(angle as u16 as f32 * (1.0/256.0));

    // velocity, clamped to +/-1.25 per axis
    let vx = read_f32(this + 0xE8) + 0.03125 * c;
    write_f32(this + 0xE8, clamp(vx, -1.25, 1.25));
    let vz = read_f32(this + 0xEC) + 0.03125 * s;
    write_f32(this + 0xEC, clamp(vz, -1.25, 1.25));

    // Q12 facing pair, computed from the same angle
    write_s16(this + 0x100, -(4096.0 * c) as i32 as i16);
    write_s16(this + 0x102,  (4096.0 * s) as i32 as i16);
}
```

Exact points:

* The `+0x100`/`+0x102` pair is a **fixed-point direction** scaled by `4096`
  (the `0x1000` literal built at `0x80AA36F8` with the `2^52` idiom), not an
  angle. `+0x100` is negated, `+0x102` is not.
* Velocity is integrated by `0.03125` per frame and clamped with the usual
  `fcmpo` + `stfs` pair to `[-1.25, +1.25]` on both axes.
* `fn_801611C0`'s first argument is the **z** difference and the second the
  **x** difference (`local_48 - local_54`, `local_4c - local_58`), which is the
  same convention as ranks 3/4/6.
* `0x80064740(this, &out)` is the ground/actor probe; a zero result skips the
  whole body.

### Ambiguity

None. The three `psq_l f1,0(rA),1,qr3` sites are the pinned u16 angle loads.

---

## 9. `0x80AFD910` - fall onto a `+0x604` row table and bank the model

* **Address:** `0x80AFD910`
* **TSV size:** 388 bytes
* **Reachable body:** 97 instructions; 6 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80afd910_FUN_80afd910.c`
  - usable.
* **Proposed signature:** `pub unsafe fn fn_80afd910(this: usize)`
* **Callers in this REL:** none (reached through the actor table / vtable).
* **Constants:** block `0x80B31750` (same block as rank 5);
  `0x80B2F... ` is not used.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+0x34`; `r29..r31` at `+0x24..+0x2C`. `r29 = player`,
`r30 = this`, `r31 = 0x80B31750`.

### Constants and tables

| address | type | value / role |
|---|---|---|
| `0x80B31750` | `i16[2]` | `{+0x1000, -0x1000}` = `{4096, -4096}` - row used when `+0x604 == 0` |
| `0x80B31758` | `i16[4]` | `{-0x1400, +0x1400, +0x0800, -0x0800}` = `{-5120, 5120, 2048, -2048}` - two rows for `+0x604 != 0` |
| `0x80B317B8..0x80B317BC` | `f64` | `2^52 + 2^31` bias |
| `0x80B317C8` | `f32` | `1/256` |
| `0x80B317CC` | `f32` | `8.0` |
| `0x80B317F0` | `f32` | `16.0` |
| `0x80B317F4` | `f32` | `1536.0` |

### Behaviour

```c
let player = fn_80162590();
write_f32(this + 0xAC, read_f32(this + 0xAC) + fn_80afd170(this));
fn_8006cde0(this);
fn_800955f0(this);

// target y = player.y + 8 + 16 * (f32)(s32)*(this+0x608)
let y = 8.0 + read_f32(player + 0xB0) + 16.0 * (read_u32(this + 0x608) as i32) as f32;
if read_f32(this + 0xB0) < y {
    write_f32(this + 0xB0, y);
}

// model angle target from the +0x604 row table
let idx = read_u32(player + 0x52C) as i32;      // 0..N side/index
let target = if read_u32(this + 0x604) == 0 {
    read_s16(0x80B31750 + 2 * idx)
} else {
    let row = (read_u32(this + 0x604) - 1) & 3;
    read_s16(0x80B31758 + 4 * row + 2 * idx)
};
fn_8015f4f0(this + 0x5CC, target, 256);

// bank: +0x5CA steps by +/-0x800 per frame, selected by bit 0
if (read_u32(this + 0x608) & 1) != 0 {
    write_s16(this + 0x5CA, read_s16(this + 0x5CA) + 0x800);
} else {
    write_s16(this + 0x5CA, read_s16(this + 0x5CA) - 0x800);
}
let s = sin16((read_s16(this + 0x5CA) as u16) as f32 * (1.0/256.0));
write_s16(this + 0x102, ((read_s16(this + 0x5CC) as f32) + 1536.0 * s) as i16);
```

Exact points:

* `0x80AFD170(this)` returns the x delta in `f1`; the addition is
  `this+0xAC += f1` (the export's `lfs f0,172(r29)` then `fadds f0,f1,f0`).
* The y test is one-sided again (`if this.y < target`), and the snap is
  unconditional on that branch.
* `fn_8015F4F0(this+0x5CC, target, 256)` is the halfword stepper; `256` is the
  per-frame step.
* The final store adds `1536.0 * sin` to the stepped `+0x5CC` and truncates to
  `i16` at `+0x102`.
* The `+0x604` index is clipped to two rows by `& 3`, and the row stride is
  `4` bytes (two `i16`), with the `player+0x52C` index added inside the row.

### Ambiguity

None. The single data `psq_l` is the pinned u16 bank angle.

---

## 10. `0x80B1D520` - trail normal from a 6-entry angle table

* **Address:** `0x80B1D520`
* **TSV size:** 352 bytes
* **Reachable body:** 88 instructions; 4 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80b1d520_FUN_80b1d520.c`
  - usable, but the export writes the whole `s16` table as
    `DAT_80b32670 + index * 2`; the raw words below are authoritative.
* **Proposed signature:** `pub unsafe fn fn_80b1d520(this: usize)`
* **Callers in this REL:** one `bl`, `0x80B1E28C`.
* **Constants:** block `0x80B32558`, table `0x80B32670`, floats `0x80B32668`.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-80(r1)`; LR at `+0x54`; `f31` at `+0x40`/`+0x48`, `f30` at
`+0x30`/`+0x38`; `r28..r31` at `+0x20..+0x2C`. `r30 = this`,
`r31 = 0x80B32558`, `r29 = 0x80B32670`.

### Constants and tables

| address | type | value / role |
|---|---|---|
| `0x80B32670` | `i16[6]` | `{0x2000, 0x0D00, 0x3400, 0x3400, 0x0D00, 0x2000}` = `{8192, 3328, 13312, 13312, 3328, 8192}` - angle table |
| `0x80B32668` | `f32[2]` | `{1.0, -1.0}` - x sign selected by the row |
| `0x80B3267C` | `f32` | `14.0` - normal length |
| `0x80B32680` | `f32` | `1/256` |
| `0x80B32684` | `f32` | `8.0` (unused here) |
| `0x80B32688` | `f32` | `16.5` (unused here) |
| `0x80B3268C` | `f32` | `1.75` (unused here) |
| `0x80B32690` | `f32` | `4.0` (unused here) |
| `0x80B32694` | `f32` | `3.0` (unused here) |

### Behaviour

```c
let i = read_u32(this + 0x658) as i32;          // 0..5
let a = read_s16(0x80B32670 + 2 * i);
write_s16(this + 0x608, -a);

let len = 14.0 + fn_800e9020(this + 0x668);     // returns f32 in f1
let c = cos16(a as u16 as f32 * (1.0/256.0));
let s = sin16(a as u16 as f32 * (1.0/256.0));

// row select: 0 for i in 0..2, 1 for i >= 3, -1 for i < 0
let row = (i >> 31) + (if i > 2 { 1 } else { 0 });
let sx = *((0x80B32668 + 4 * row) as *const f32);

let dz = (read_f32(this + 0x620) + len * s) - read_f32(this + 0xB0);
let dx = (read_f32(this + 0x61C) + len * (c * sx)) - read_f32(this + 0xAC);
let d = sqrtf(dx * dx + dz * dz);
write_f32(this + 0x614, dx / d);
write_f32(this + 0x618, dz / d);
```

Exact points:

* `0x800E9020(this + 0x668)` returns `f1`; the call has no other visible side
  effect in the port.
* The `row` expression is the classic signed `/2`-style selector emitted for
  `(i >= 3)` with a negative guard; a plain `if i > 2 { 1 } else { 0 }` is
  equivalent for all non-negative `i`, and `i` is a `u32` at `+0x658`.
* The `+0x614`/`+0x618` pair is the normalised horizontal trail direction.
* `+0x608` gets the **negated** table angle.
* The normal is only 2-D; `+0x620`/`+0x61C` are the y/x anchors and `+0xB0`/
  `+0xAC` the current position.

### Ambiguity

None. There are three `psq_l f1,0(rA),1,qr3` sites; each reads the `u16` angle
copied from `+0x658`.

---

## 11. `0x80A436C0` - rotate a 3-vector into screen space and clip

* **Address:** `0x80A436C0`
* **TSV size:** 328 bytes
* **Reachable body:** 82 instructions; 3 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a436c0_FUN_80a436c0.c`
  - usable.
* **Proposed signature:** `pub unsafe fn fn_80a436c0(this: usize, other: usize) -> u32`
* **Callers in this REL:** two `bl`, `0x80A43560` and `0x80A439C8`.
* **Constants:** `0x80B2C70C` = `8.0`, `0x80B2C710..0x80B2C714` = `2^52` bias,
  `0x80B2C718` = `1/256`.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-64(r1)`; LR at `+0x44`; `r29..r31` at `+0x34..+0x3C`. `r29 = this`,
`r31 = 0x80B2C718`, `other` is the incoming `r4` (kept in `r4`).

### Behaviour

```c
let angle = 0x4000 - read_s16(this + 0x910);     // mirror about +90 deg
write_u32(this + 0x900, read_u32(other + 0x000));
write_f32(this + 0x904,
          8.0 + read_f32(this + 0x8FC) + read_f32(other + 0x4D4));

let c = cos16(angle as u16 as f32 * (1.0/256.0));
let s = sin16(angle as u16 as f32 * (1.0/256.0));

let out = [
    read_f32(other + 0x4B8) + read_f32(this + 0x904) * c,
    read_f32(other + 0x4BC) + read_f32(this + 0x904) * s,
    read_f32(other + 0x4C0),
];
write_f32(this + 0x908, out[0]);
write_f32(this + 0x90C, out[1]);

let r = fn_80077810(read_u32(0x80429DD0),
                    (out[0] as i32) as u16,
                    (-out[1] as i32) as u16,
                    read_u8(0x80429D28));
if r != 0 {
    let p = 0x80429D14 as *mut u8;
    *p |= 1;
}
return if r == 0 { 1 } else { 0 };
```

Exact points:

* The `angle` is `0x4000 - this+0x910` (a `subfic`), so the rotation is mirrored
  about the +90° axis.
* `this+0x904` is used as the **radius** for both the x and the y term, so the
  rotation is a 2-D rotation of the anchor `(other+0x4B8, other+0x4BC)`; the
  third component `other+0x4C0` is copied untouched into the stack record but
  only `+0x908`/`+0x90C` are written back.
* The clip test takes `(int)x & 0xFFFF` and `(int)-y & 0xFFFF` (the `fneg` +
  `fctiwz` + `rlwinm 0,16,31` sequence).
* `0x80429DD0` and `0x80429D28`/`0x80429D14` are `main.dol` globals (ext); the
  bit-0 set on `0x80429D14` is the "effect rejected" flag.
* The return value is **inverted**: `1` means the clip test missed.

### Ambiguity

None. The two data `psq_l` sites read the u16 angle.

---

## 12. `0x80A24AD0` - balloon rise driven by the player's height

* **Address:** `0x80A24AD0`
* **TSV size:** 312 bytes
* **Reachable body:** 78 instructions; 2 direct `bl`; ends in `blr`.
* **Export:** `work/ghidra/rel-d_enemiesNP/export/80a24ad0_FUN_80a24ad0.c`
  - usable.
* **Proposed signature:** `pub unsafe fn fn_80a24ad0(this: usize)`
* **Callers in this REL:** one `bl`, `0x80A243A4`.
* **Constants:** block `0x80B2B808`.
* **Verdict:** **ready** (confidence high)

### Frame

`stwu r1,-48(r1)`; LR at `+0x34`; `f31` at `+0x20`/`+0x28`; `r30`/`r31` at
`+0x18`/`+0x1C`. `r30 = this`, `r31 = 0x80B2B808`.

### Constants

| address | type | value / role |
|---|---|---|
| `0x80B2B808` | `f32` | `4.0` |
| `0x80B2B80C` | `f32` | `8.0` |
| `0x80B2B818` | `f32` | `480.0` |
| `0x80B2B81C` | `f32` | `1.5` |
| `0x80B2B820` | `f32` | `0.0` - idle bob amplitude |
| `0x80B2B824` | `f32` | `8.0` - active bob amplitude |
| `0x80B2B828` | `f32` | `1.7` |
| `0x80B2B82C` | `f32` | `-8.0` |
| `0x80B2B830` | `f32` | `14.0` |
| `0x80B2B834` | `f32` | `-2.0` |
| `0x80B2B838` | `f32` | `16.0` |
| `0x80B2B83C` | `f32` | `1.0` |
| `0x80B2B840` | `f32` | `4.0` |
| `0x80B2B844` | `f32` | `1/256` |
| `0x80B2B848` | `u32` | `0xFFFFFFFF` |
| `0x80B2B84C` | `f32` | `8.0` |
| `0x80B2B850` | `f32` | `-3000.0` |
| `0x80B2B854` | `f32` | `500.0` |

### Behaviour

```c
// the player's y minus a 4.0 margin, then minus a second 4.0 margin
let f31 = (read_f32(read_u32(0x80429DD0) + 0x8FEC4) - 4.0) - 4.0;

let g = read_u32(0x80439D40);
if read_u32(g + 0x2C) == 0 && read_u32(this + 0x718) != 0 {
    write_u32(this + 0x71C, read_u32(this + 0x718));
    write_s16(this + 0x712, 0);
    write_u16(this + 0x710, 0);
}

let bob = 0.0f32;
if read_u32(this + 0x71C) != 0 {
    if (read_u32(this + 0x71C) as i32) < 0 {
        write_s16(this + 0x712, read_s16(this + 0x712) - 0x400);
    } else {
        write_s16(this + 0x712, read_s16(this + 0x712) + 0x400);
    }
    let ph = read_u16(this + 0x710) + 0x400;
    write_u16(this + 0x710, ph);
    if ph >= 0x8000 {
        write_u16(this + 0x710, 0);
        write_u32(this + 0x71C, 0);
        write_s16(this + 0x712, 0);
    }
    bob = 8.0 * sin16((read_u16(this + 0x712)) as f32 * (1.0/256.0));
}
write_f32(this + 0xB0, bob + (f31 + read_f32(this + 0x61C)));

fn_80a24580(this);
write_u32(this + 0x718, read_u32(g + 0x2C));
```

Exact points:

* `f31` is `player_y - 8.0` in two steps (`-4.0` twice), i.e. the rise target
  plus a `-8` margin.
* `+0x71C` is a **signed** direction word: negative subtracts `0x400` from the
  angle, non-negative adds. The angle is `+0x712` (`s16`); the phase `+0x710`
  is `u16` and wraps at `0x8000`.
* The bob term is exactly `8.0 * sin(+0x712 / 256)`, and it is replaced by
  `0.0` when `+0x71C` is zero.
* `+0x61C` is the per-actor y offset and is added after `f31`.
* `+0x718` is a one-frame shadow of `*(u32*)(g + 0x2C)`; the next frame compares
  the new value against it through the `*(u32*)(g+0x2C) == 0` guard.
* `0x80429DD0` and `0x80439D40` are `main.dol` globals (ext).

### Ambiguity

None. The single data `psq_l` reads the `u16` angle staged from `+0x712`.

---

## Verification

* Every one of the twelve bodies was decoded word-by-word from
  `work/rels/elf/d_enemiesNP.elf` at
  `file_offset = address - 0x80A00000 + 0x1000` and walked as a control-flow
  graph until every path ended in `blr`, `b`, or a `bctr` tail. Every reachable
  span fills the TSV extent, so the extents in the table are real bodies.
* The `psq_l` quantiser index was extracted from bits 12-14 of each word. Every
  data `psq_l` in the twelve rows is `W = 1`, `DQ = 0`, `qr = 3`; every other
  `psq_st/psq_l` is a `qr0` spill on `r1`. No row depends on `qr4..qr7`.
* The constants were read from the ELF at their file offsets:
  `0x80B2C2A8`, `0x80B31208`, `0x80B31240`, `0x80B2B808`, `0x80B2C70C`,
  `0x80B32668`/`0x80B32670`, `0x80B31750`, `0x80B2F500`, `0x80B2F530`,
  `0x80B2F6A0`, `0x80B2C6E0`. Addresses at `0x80B77xxx` and `0x80B78xxx` are
  outside the file-backed image and are marked **(ext)**.
* Direct callers were located by scanning every 4-byte-aligned word in the
  image for `bl` displacements; the three vtable-only entries
  (`0x80AEDD40`, `0x80AEF130`, `0x80AFDAC0`, plus `0x80AFD910`) have no `bl`
  and no 32-bit word reference, which matches their use through the actor
  vtable.
* No row in this tranche contains a `bctr` jump table, a `halt_baddata`-only
  body, or an `extraout_` register. The two indirect sites in ranks 2 and 3 are
  plain vtable calls whose receiver is `this+0x394` / `this+0x60`.

### Cross-references and collisions

* Ranks 1 and 4 name the Fire Snake string table (`Wm_en_firesnk_icehit_h`,
  `Wm_en_firesnk_icehitsmk_h`, `g3d/bubble.brres`) only as *evidence* of the
  actor family; those strings are not operands of the bodies.
* The two `i16` tables at `0x80B31750` and `0x80B32670` are shared with ranks 5
  and 9; a port should keep them as single constants, not duplicate them.
* `0x80B2C6E0`, `0x80B2F534`, `0x80B32680`, `0x80B2B844`, `0x80B2F848`,
  `0x80B317C8` all hold the same `1/256` angle scale. They are distinct storage
  locations with identical values; keep the per-actor base.
* No address in this page is present in `ledger/functions.csv` or in any `##`
  heading of `docs/*.md` as of this pass.

